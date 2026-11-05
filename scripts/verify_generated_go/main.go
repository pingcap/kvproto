// verify_generated_go compares committed Go bindings with freshly generated ones.
package main

import (
	"bytes"
	"compress/gzip"
	"crypto/sha256"
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"io"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
)

const maxDescriptorSize = 64 << 20

type descriptorSpan struct {
	start, end int
	digest     [sha256.Size]byte
}

func normalizeDescriptors(source []byte) ([]byte, error) {
	fset := token.NewFileSet()
	file, err := parser.ParseFile(fset, "generated.go", source, 0)
	if err != nil {
		return nil, err
	}

	var spans []descriptorSpan
	for _, decl := range file.Decls {
		group, ok := decl.(*ast.GenDecl)
		if !ok || group.Tok != token.VAR {
			continue
		}
		for _, spec := range group.Specs {
			value := spec.(*ast.ValueSpec)
			if len(value.Names) == 0 || !strings.HasPrefix(value.Names[0].Name, "fileDescriptor_") {
				continue
			}
			if len(value.Names) != 1 || len(value.Values) != 1 {
				return nil, fmt.Errorf("%s: unexpected declaration", value.Names[0].Name)
			}
			literal, ok := value.Values[0].(*ast.CompositeLit)
			if !ok {
				return nil, fmt.Errorf("%s: expected byte literal", value.Names[0].Name)
			}
			array, ok := literal.Type.(*ast.ArrayType)
			if !ok || array.Len != nil {
				return nil, fmt.Errorf("%s: expected byte slice", value.Names[0].Name)
			}
			element, ok := array.Elt.(*ast.Ident)
			if !ok || element.Name != "byte" {
				return nil, fmt.Errorf("%s: expected byte slice", value.Names[0].Name)
			}

			compressed := make([]byte, 0, len(literal.Elts))
			for _, element := range literal.Elts {
				number, ok := element.(*ast.BasicLit)
				if !ok || number.Kind != token.INT {
					return nil, fmt.Errorf("%s: non-literal descriptor byte", value.Names[0].Name)
				}
				n, err := strconv.ParseUint(number.Value, 0, 8)
				if err != nil {
					return nil, fmt.Errorf("%s: invalid descriptor byte: %w", value.Names[0].Name, err)
				}
				compressed = append(compressed, byte(n))
			}
			reader, err := gzip.NewReader(bytes.NewReader(compressed))
			if err != nil {
				return nil, fmt.Errorf("%s: invalid gzip descriptor: %w", value.Names[0].Name, err)
			}
			descriptor, err := io.ReadAll(io.LimitReader(reader, maxDescriptorSize+1))
			if err != nil {
				return nil, fmt.Errorf("%s: read descriptor: %w", value.Names[0].Name, err)
			}
			if err := reader.Close(); err != nil {
				return nil, fmt.Errorf("%s: close descriptor: %w", value.Names[0].Name, err)
			}
			if len(descriptor) > maxDescriptorSize {
				return nil, fmt.Errorf("%s: descriptor exceeds size limit", value.Names[0].Name)
			}
			spans = append(spans, descriptorSpan{
				start:  fset.Position(literal.Pos()).Offset,
				end:    fset.Position(literal.End()).Offset,
				digest: sha256.Sum256(descriptor),
			})
		}
	}

	sort.Slice(spans, func(i, j int) bool { return spans[i].start < spans[j].start })
	var normalized bytes.Buffer
	offset := 0
	for _, span := range spans {
		normalized.Write(source[offset:span.start])
		fmt.Fprintf(&normalized, "[]byte{descriptor-sha256:%x}", span.digest)
		offset = span.end
	}
	normalized.Write(source[offset:])
	return normalized.Bytes(), nil
}

func generatedGoFile(path string) bool {
	return strings.HasSuffix(path, ".pb.go") || strings.HasSuffix(path, ".pb.gw.go")
}

func gitOutput(root string, args ...string) ([]byte, error) {
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	output, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("git %s: %w", strings.Join(args, " "), err)
	}
	return output, nil
}

func committedFiles(root string) (map[string]bool, error) {
	output, err := gitOutput(root, "ls-tree", "-r", "-z", "HEAD", "--", "pkg")
	if err != nil {
		return nil, err
	}
	files := make(map[string]bool)
	for _, entry := range bytes.Split(output, []byte{0}) {
		if len(entry) == 0 {
			continue
		}
		header, path, ok := bytes.Cut(entry, []byte{'\t'})
		if !ok {
			return nil, fmt.Errorf("invalid git tree entry")
		}
		name := string(path)
		if !generatedGoFile(name) {
			continue
		}
		fields := strings.Fields(string(header))
		if len(fields) != 3 || fields[1] != "blob" || (fields[0] != "100644" && fields[0] != "100755") {
			return nil, fmt.Errorf("%s: generated file is not a regular file", name)
		}
		files[name] = true
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("no committed generated Go files found")
	}
	return files, nil
}

func regeneratedFiles(root string) (map[string]bool, error) {
	files := make(map[string]bool)
	err := filepath.WalkDir(filepath.Join(root, "pkg"), func(path string, entry fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.IsDir() {
			return nil
		}
		relative, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		name := filepath.ToSlash(relative)
		if !generatedGoFile(name) {
			return nil
		}
		info, err := entry.Info()
		if err != nil {
			return err
		}
		if !info.Mode().IsRegular() {
			return fmt.Errorf("%s: generated file is not a regular file", name)
		}
		files[name] = true
		return nil
	})
	return files, err
}

func verify(root string) error {
	committed, err := committedFiles(root)
	if err != nil {
		return err
	}
	regenerated, err := regeneratedFiles(root)
	if err != nil {
		return err
	}
	var paths []string
	for path := range committed {
		paths = append(paths, path)
	}
	for path := range regenerated {
		if !committed[path] {
			paths = append(paths, path)
		}
	}
	sort.Strings(paths)

	var differences []string
	for _, path := range paths {
		if !committed[path] {
			differences = append(differences, path+": uncommitted generated file")
			continue
		}
		if !regenerated[path] {
			differences = append(differences, path+": not generated")
			continue
		}
		before, err := gitOutput(root, "show", "HEAD:"+path)
		if err != nil {
			return err
		}
		after, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(path)))
		if err != nil {
			return err
		}
		before, err = normalizeDescriptors(before)
		if err != nil {
			return fmt.Errorf("%s: committed output: %w", path, err)
		}
		after, err = normalizeDescriptors(after)
		if err != nil {
			return fmt.Errorf("%s: regenerated output: %w", path, err)
		}
		if !bytes.Equal(before, after) {
			differences = append(differences, path+": content differs")
		}
	}
	if len(differences) != 0 {
		return fmt.Errorf("generated Go bindings do not match:\n%s", strings.Join(differences, "\n"))
	}
	return nil
}

func main() {
	if err := verify("."); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
