// Copyright 2026 TiKV Project Authors. Licensed under Apache-2.0.

package coprocessor

import (
	"bytes"
	"testing"
)

func TestAnalyzeBatchTaskDataMergeCapabilityWireCompatibility(t *testing.T) {
	data, err := (&Request{AllowAnalyzeBatchTaskDataMerge: true}).Marshal()
	if err != nil {
		t.Fatal(err)
	}
	if want := []byte{0x90, 0x01, 0x01}; !bytes.Equal(data, want) {
		t.Fatalf("unexpected wire encoding: got %x, want %x", data, want)
	}

	data, err = (&Request{}).Marshal()
	if err != nil {
		t.Fatal(err)
	}
	if len(data) != 0 {
		t.Fatalf("disabled capability must be omitted, got %x", data)
	}
}
