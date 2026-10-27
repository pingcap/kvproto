// Copyright 2026 PingCAP, Inc.

package raft_serverpb_test

import (
	"github.com/gogo/protobuf/proto"
	"github.com/pingcap/kvproto/pkg/raft_serverpb"
	"testing"
)

func TestWitnessPressurePreservesPresenceAndProgress(t *testing.T) {
	for _, pressure := range []*raft_serverpb.WitnessLogPressure{
		nil,
		{SessionId: []byte("leader-session"), RequestId: 9},
		{SessionId: []byte("leader-session"), RequestId: 9, Response: true,
			Level:            raft_serverpb.WitnessLogPressureLevel_PRESSURE_CRITICAL,
			RetainedLogBytes: 4096, TruncatedIndex: 80, PersistedIndex: 120,
			DataPersistedIndex: 100, ControlAppliedIndex: 119},
		{SessionId: []byte("leader-session"), RequestId: 10, Response: true,
			Level:          raft_serverpb.WitnessLogPressureLevel_PRESSURE_NORMAL,
			TruncatedIndex: 100, PersistedIndex: 120, DataPersistedIndex: 100,
			ControlAppliedIndex: 120, RaftDiskSpaceKnown: true, RaftDiskAvailableBytes: 8192},
	} {
		message := &raft_serverpb.RaftMessage{RegionId: 7, WitnessLogPressure: pressure}
		encoded, err := message.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var decoded raft_serverpb.RaftMessage
		if err := decoded.Unmarshal(encoded); err != nil {
			t.Fatal(err)
		}
		if decoded.RegionId != 7 || (decoded.WitnessLogPressure == nil) != (pressure == nil) || !proto.Equal(decoded.WitnessLogPressure, pressure) {
			t.Fatalf("lost identity, presence or pressure progress: %v", &decoded)
		}
	}
}
