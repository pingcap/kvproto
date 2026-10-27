// Copyright 2026 PingCAP, Inc.

package pdpb_test

import (
	"testing"

	"github.com/gogo/protobuf/proto"
	"github.com/pingcap/kvproto/pkg/metapb"
	"github.com/pingcap/kvproto/pkg/pdpb"
	"github.com/pingcap/kvproto/pkg/schedulingpb"
)

func TestRecoveryStatusPreservesPresenceAndIdentity(t *testing.T) {
	for _, tc := range []struct {
		name   string
		status *pdpb.RegionRecoveryStatus
	}{
		{"legacy-absent", nil},
		{"explicit-not-ready", &pdpb.RegionRecoveryStatus{}},
		{"ready-full", &pdpb.RegionRecoveryStatus{DataServiceReady: true, SizeSource: pdpb.RegionSizeSource_SIZE_CURRENT}},
		{"recovering-witness", &pdpb.RegionRecoveryStatus{
			State:        pdpb.RegionRecoveryState_RECOVERING_LEADER,
			SizeSource:   pdpb.RegionSizeSource_SIZE_HISTORICAL,
			TargetPeerId: 42, SessionId: []byte("recovery-session"),
		}},
	} {
		t.Run(tc.name, func(t *testing.T) {
			heartbeat := &pdpb.RegionHeartbeatRequest{Region: &metapb.Region{Id: 7}, RecoveryStatus: tc.status}
			encoded, err := heartbeat.Marshal()
			if err != nil {
				t.Fatal(err)
			}
			var decoded pdpb.RegionHeartbeatRequest
			if err := decoded.Unmarshal(encoded); err != nil {
				t.Fatal(err)
			}
			if (decoded.RecoveryStatus == nil) != (tc.status == nil) || !proto.Equal(decoded.RecoveryStatus, tc.status) {
				t.Fatalf("lost status presence or contents: got %v, want %v", decoded.RecoveryStatus, tc.status)
			}
			forwarded := &schedulingpb.RegionHeartbeatRequest{Region: decoded.Region, RecoveryStatus: decoded.RecoveryStatus}
			encoded, err = forwarded.Marshal()
			if err != nil {
				t.Fatal(err)
			}
			var scheduling schedulingpb.RegionHeartbeatRequest
			if err := scheduling.Unmarshal(encoded); err != nil {
				t.Fatal(err)
			}
			if scheduling.Region.GetId() != 7 || (scheduling.RecoveryStatus == nil) != (tc.status == nil) || !proto.Equal(scheduling.RecoveryStatus, tc.status) {
				t.Fatalf("forwarding lost identity/status: %v", &scheduling)
			}
		})
	}
}

func TestRecoverySyncIsSparseAndRegionBound(t *testing.T) {
	response := &pdpb.SyncRegionResponse{
		Regions: []*metapb.Region{{Id: 7}, {Id: 11}},
		RegionRecoveryInfos: []*pdpb.RegionRecoveryInfo{{
			RegionId: 11,
			Status:   &pdpb.RegionRecoveryStatus{State: pdpb.RegionRecoveryState_RECOVERING_LEADER},
		}},
	}
	encoded, err := response.Marshal()
	if err != nil {
		t.Fatal(err)
	}
	var decoded pdpb.SyncRegionResponse
	if err := decoded.Unmarshal(encoded); err != nil {
		t.Fatal(err)
	}
	if len(decoded.RegionRecoveryInfos) != 1 || decoded.RegionRecoveryInfos[0].RegionId != 11 || decoded.RegionRecoveryInfos[0].Status == nil {
		t.Fatalf("sparse recovery identity was not preserved: %v", &decoded)
	}
}

func TestLogOnlyProtocolDeclarationRoundTrips(t *testing.T) {
	for _, version := range []uint32{0, 2} {
		store := &metapb.Store{Id: 7, LogOnlyProtocolVersion: version}
		encoded, err := store.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var decoded metapb.Store
		if err := decoded.Unmarshal(encoded); err != nil {
			t.Fatal(err)
		}
		if decoded.GetLogOnlyProtocolVersion() != version {
			t.Fatalf("store version: %v", &decoded)
		}
		participant := &schedulingpb.Participant{Id: 3, LogOnlyProtocolVersion: version}
		encoded, err = participant.Marshal()
		if err != nil {
			t.Fatal(err)
		}
		var primary schedulingpb.Participant
		if err := primary.Unmarshal(encoded); err != nil {
			t.Fatal(err)
		}
		if primary.GetLogOnlyProtocolVersion() != version {
			t.Fatalf("participant version: %v", &primary)
		}
	}
}
