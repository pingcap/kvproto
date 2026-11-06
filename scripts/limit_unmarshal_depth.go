// limit_unmarshal_depth adds a depth budget to recursive generated Go decoders.
package main

import (
	"fmt"
	"go/format"
	"os"
	"path/filepath"
	"strings"
)

type replacement struct {
	old string
	new string
}

type message struct {
	name         string
	replacements []replacement
}

const input = "dAtA[iNdEx:postIndex]"

func recursiveCall(receiver string) replacement {
	return replacement{
		old: receiver + ".Unmarshal(" + input + ")",
		new: receiver + ".unmarshalWithDepth(" + input + ", depth+1)",
	}
}

func oneofCall(messageType string) replacement {
	old := "v := &" + messageType + "{}\n\t\t\tif err := v.Unmarshal(" + input + ")"
	return replacement{
		old: old,
		new: strings.Replace(old, "v.Unmarshal("+input+")", "v.unmarshalWithDepth("+input+", depth+1)", 1),
	}
}

func rewrite(filename string, messages []message) error {
	data, err := os.ReadFile(filename)
	if err != nil {
		return err
	}
	source := string(data)
	for _, msg := range messages {
		signature := "func (m *" + msg.name + ") Unmarshal(dAtA []byte) error {\n"
		if strings.Count(source, signature) != 1 {
			return fmt.Errorf("%s: expected one Unmarshal method for %s", filename, msg.name)
		}
		start := strings.Index(source, signature)
		end := strings.Index(source[start+len(signature):], "\nfunc ")
		if end < 0 {
			return fmt.Errorf("%s: could not find end of %s.Unmarshal", filename, msg.name)
		}
		end += start + len(signature)
		body := source[start:end]
		for _, call := range msg.replacements {
			if strings.Count(body, call.old) != 1 {
				return fmt.Errorf("%s: expected one recursive call %q in %s.Unmarshal", filename, call.old, msg.name)
			}
			body = strings.Replace(body, call.old, call.new, 1)
		}
		wrapper := "func (m *" + msg.name + ") Unmarshal(dAtA []byte) error {\n" +
			"\treturn m.unmarshalWithDepth(dAtA, 0)\n" +
			"}\n\n" +
			"func (m *" + msg.name + ") unmarshalWithDepth(dAtA []byte, depth int) error {\n" +
			"\tif depth >= 1000 {\n" +
			"\t\treturn fmt.Errorf(\"proto: maximum message nesting depth exceeded\")\n" +
			"\t}\n"
		body = strings.Replace(body, signature, wrapper, 1)
		source = source[:start] + body + source[end:]
	}
	formatted, err := format.Source([]byte(source))
	if err != nil {
		return fmt.Errorf("%s: format generated source: %w", filename, err)
	}
	return os.WriteFile(filename, formatted, 0644)
}

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: limit_unmarshal_depth <repository root>")
		os.Exit(2)
	}
	root := os.Args[1]
	files := []struct {
		path     string
		messages []message
	}{
		{
			path: "pkg/kvrpcpb/kvrpcpb.pb.go",
			messages: []message{
				{"LockInfo", []replacement{recursiveCall("m.SharedLockInfos[len(m.SharedLockInfos)-1]")}},
			},
		},
		{
			path: "pkg/db9_coprocessor/db9_coprocessor.pb.go",
			messages: []message{
				{"Db9Expr", []replacement{
					recursiveCall("m.ReturnType"),
					oneofCall("Db9BinaryExpr"),
					oneofCall("Db9UnaryExpr"),
					oneofCall("Db9CastExpr"),
					oneofCall("Db9FuncCallExpr"),
					oneofCall("Db9IsNullExpr"),
				}},
				{"Db9BinaryExpr", []replacement{recursiveCall("m.Left"), recursiveCall("m.Right")}},
				{"Db9UnaryExpr", []replacement{recursiveCall("m.Operand")}},
				{"Db9CastExpr", []replacement{recursiveCall("m.Expr"), recursiveCall("m.TargetType")}},
				{"Db9FuncCallExpr", []replacement{recursiveCall("m.Args[len(m.Args)-1]")}},
				{"Db9IsNullExpr", []replacement{recursiveCall("m.Expr")}},
				{"Db9Type", []replacement{recursiveCall("m.ElemType")}},
			},
		},
	}
	for _, file := range files {
		if err := rewrite(filepath.Join(root, file.path), file.messages); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
	}
}
