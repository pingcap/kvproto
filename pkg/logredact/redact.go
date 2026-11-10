// Package logredact protects selected protobuf fields in generated String methods.
package logredact

import (
	"fmt"
	"reflect"
	"strings"

	"github.com/golang/protobuf/proto"
)

const replacement = "[REDACTED]"

func Format(state fmt.State, verb rune, message proto.Message) {
	format := fmt.FormatString(state, verb)
	if verb == 'v' {
		format = strings.ReplaceAll(format, "#", "")
	}
	fmt.Fprintf(state, format, CompactTextString(message))
}

// CompactTextString renders a copy with known credentials and keys redacted.
// It does not modify the message or change protobuf and JSON serialization.
func CompactTextString(message proto.Message) string {
	value, changed, ok := redact(reflect.ValueOf(message), make(map[uintptr]bool), 0)
	if !ok {
		return "[REDACTED: message is too deeply nested or cyclic]"
	}
	if changed {
		message = value.Interface().(proto.Message)
	}
	return proto.CompactTextString(message)
}

func redact(value reflect.Value, active map[uintptr]bool, depth int) (reflect.Value, bool, bool) {
	if !value.IsValid() {
		return value, false, true
	}
	if depth > 100 {
		return value, false, false
	}
	switch value.Kind() {
	case reflect.Interface:
		if value.IsNil() {
			return value, false, true
		}
		child, changed, ok := redact(value.Elem(), active, depth+1)
		if !ok || !changed {
			return value, false, ok
		}
		copy := reflect.New(value.Type()).Elem()
		copy.Set(child)
		return copy, true, true
	case reflect.Ptr:
		if value.IsNil() {
			return value, false, true
		}
		address := value.Pointer()
		if active[address] {
			return value, false, false
		}
		active[address] = true
		child, changed, ok := redact(value.Elem(), active, depth+1)
		delete(active, address)
		if !ok || !changed {
			return value, false, ok
		}
		copy := reflect.New(value.Type().Elem())
		copy.Elem().Set(child)
		return copy, true, true
	case reflect.Struct:
		typeInfo := value.Type()
		name := typeInfo.PkgPath() + "." + typeInfo.Name()
		if !protectedMessageTypes[name] && !isOneofWrapper(typeInfo) {
			return value, false, true
		}
		var copy reflect.Value
		for i := 0; i < value.NumField(); i++ {
			field := typeInfo.Field(i)
			if !field.IsExported() {
				continue
			}
			original := value.Field(i)
			var child reflect.Value
			var changed, ok bool
			fieldName := protobufFieldName(field.Tag.Get("protobuf"))
			if secretFields[name][fieldName] {
				child, changed, ok = redactSecret(original)
			} else if secretMapFields[name][fieldName] {
				child, changed, ok = redactSecretMap(original)
			} else {
				child, changed, ok = redact(original, active, depth+1)
			}
			if !ok {
				return value, false, false
			}
			if changed {
				if !copy.IsValid() {
					copy = reflect.New(typeInfo).Elem()
					copy.Set(value)
				}
				copy.Field(i).Set(child)
			}
		}
		if copy.IsValid() {
			return copy, true, true
		}
	case reflect.Slice:
		if value.IsNil() || value.Type().Elem().Kind() == reflect.Uint8 {
			return value, false, true
		}
		var copy reflect.Value
		for i := 0; i < value.Len(); i++ {
			child, changed, ok := redact(value.Index(i), active, depth+1)
			if !ok {
				return value, false, false
			}
			if changed {
				if !copy.IsValid() {
					copy = reflect.MakeSlice(value.Type(), value.Len(), value.Len())
					reflect.Copy(copy, value)
				}
				copy.Index(i).Set(child)
			}
		}
		if copy.IsValid() {
			return copy, true, true
		}
	case reflect.Map:
		if value.IsNil() {
			return value, false, true
		}
		var copy reflect.Value
		iter := value.MapRange()
		for iter.Next() {
			child, changed, ok := redact(iter.Value(), active, depth+1)
			if !ok {
				return value, false, false
			}
			if changed {
				if !copy.IsValid() {
					copy = reflect.MakeMapWithSize(value.Type(), value.Len())
					original := value.MapRange()
					for original.Next() {
						copy.SetMapIndex(original.Key(), original.Value())
					}
				}
				copy.SetMapIndex(iter.Key(), child)
			}
		}
		if copy.IsValid() {
			return copy, true, true
		}
	}
	return value, false, true
}

func isOneofWrapper(t reflect.Type) bool {
	name := t.Name()
	for i := strings.IndexByte(name, '_'); i >= 0; {
		if protectedMessageTypes[t.PkgPath()+"."+name[:i]] {
			return true
		}
		next := strings.IndexByte(name[i+1:], '_')
		if next < 0 {
			break
		}
		i += next + 1
	}
	return false
}

func protobufFieldName(tag string) string {
	for _, part := range strings.Split(tag, ",") {
		if strings.HasPrefix(part, "name=") {
			return strings.TrimPrefix(part, "name=")
		}
	}
	return ""
}

func redactSecret(value reflect.Value) (reflect.Value, bool, bool) {
	switch value.Kind() {
	case reflect.String:
		if value.Len() != 0 {
			return reflect.ValueOf(replacement).Convert(value.Type()), true, true
		}
		return value, false, true
	case reflect.Slice:
		if value.Type().Elem().Kind() == reflect.Uint8 {
			if value.Len() != 0 {
				return reflect.ValueOf([]byte(replacement)).Convert(value.Type()), true, true
			}
			return value, false, true
		}
	}
	return value, false, false
}

func redactSecretMap(value reflect.Value) (reflect.Value, bool, bool) {
	if value.Kind() != reflect.Map || value.Type().Key().Kind() != reflect.String ||
		value.Type().Elem().Kind() != reflect.String {
		return value, false, false
	}
	if value.IsNil() {
		return value, false, true
	}
	var copy reflect.Value
	iter := value.MapRange()
	for iter.Next() {
		if iter.Value().Len() == 0 {
			continue
		}
		if !copy.IsValid() {
			copy = reflect.MakeMapWithSize(value.Type(), value.Len())
			original := value.MapRange()
			for original.Next() {
				copy.SetMapIndex(original.Key(), original.Value())
			}
		}
		copy.SetMapIndex(iter.Key(), reflect.ValueOf(replacement).Convert(value.Type().Elem()))
	}
	if copy.IsValid() {
		return copy, true, true
	}
	return value, false, true
}
