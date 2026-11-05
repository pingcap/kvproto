package main

import (
	"bytes"
	"compress/gzip"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func generatedSource(t *testing.T, descriptor []byte, level int, extra string) []byte {
	t.Helper()
	var compressed bytes.Buffer
	writer, err := gzip.NewWriterLevel(&compressed, level)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := writer.Write(descriptor); err != nil {
		t.Fatal(err)
	}
	if err := writer.Close(); err != nil {
		t.Fatal(err)
	}
	var source strings.Builder
	source.WriteString("package example\nvar fileDescriptor_1234 = []byte{\n")
	for _, b := range compressed.Bytes() {
		fmt.Fprintf(&source, "0x%02x,", b)
	}
	source.WriteString("\n}\n")
	source.WriteString(extra)
	return []byte(source.String())
}

func runGit(t *testing.T, root string, args ...string) {
	t.Helper()
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	if output, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("git %v: %v\n%s", args, err, output)
	}
}

func TestVerifyGeneratedGo(t *testing.T) {
	root := t.TempDir()
	dir := filepath.Join(root, "pkg", "example")
	if err := os.MkdirAll(dir, 0755); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(dir, "example.pb.go")
	descriptor := bytes.Repeat([]byte("descriptor"), 100)
	committed := generatedSource(t, descriptor, gzip.BestSpeed, "")
	regenerated := generatedSource(t, descriptor, gzip.BestCompression, "")
	if bytes.Equal(committed, regenerated) {
		t.Fatal("expected different compressed representations")
	}
	if err := os.WriteFile(path, committed, 0644); err != nil {
		t.Fatal(err)
	}
	runGit(t, root, "init", "-q")
	runGit(t, root, "add", ".")
	runGit(t, root, "-c", "user.name=Test", "-c", "user.email=test@example.invalid", "commit", "-qm", "baseline")

	if err := os.WriteFile(path, regenerated, 0644); err != nil {
		t.Fatal(err)
	}
	if err := verify(root); err != nil {
		t.Fatalf("equivalent descriptors rejected: %v", err)
	}

	if err := os.WriteFile(path, generatedSource(t, descriptor, gzip.BestCompression, "func injected() {}\n"), 0644); err != nil {
		t.Fatal(err)
	}
	if err := verify(root); err == nil {
		t.Fatal("injected code was accepted")
	}

	if err := os.WriteFile(path, generatedSource(t, []byte("different"), gzip.BestCompression, ""), 0644); err != nil {
		t.Fatal(err)
	}
	if err := verify(root); err == nil {
		t.Fatal("changed descriptor was accepted")
	}

	if err := os.Remove(path); err != nil {
		t.Fatal(err)
	}
	if err := verify(root); err == nil {
		t.Fatal("missing generated file was accepted")
	}

	if err := os.WriteFile(path, regenerated, 0644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "extra.pb.go"), []byte("package example\n"), 0644); err != nil {
		t.Fatal(err)
	}
	if err := verify(root); err == nil {
		t.Fatal("unexpected generated file was accepted")
	}
}

func TestNormalizeRejectsDescriptorExpressions(t *testing.T) {
	source := []byte("package example\nvar fileDescriptor_1234 = []byte{1 + 2}\n")
	if _, err := normalizeDescriptors(source); err == nil {
		t.Fatal("non-literal descriptor byte was accepted")
	}
}
