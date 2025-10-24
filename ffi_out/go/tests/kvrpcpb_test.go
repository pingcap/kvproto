package tests

import (
	"reflect"
	"testing"

	kvrpcpbffi "github.com/pingcap/kvproto/ffi_out/go/kvrpcpb"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	kvrpcpb "github.com/pingcap/kvproto/pkg/kvrpcpb"
	resource_manager "github.com/pingcap/kvproto/pkg/resource_manager"
	tracepb "github.com/pingcap/kvproto/pkg/tracepb"
)

func buildContext() *kvrpcpb.Context {
	ctx := &kvrpcpb.Context{
		RegionId:               42,
		Term:                   11,
		Priority:               kvrpcpb.CommandPri_High,
		IsolationLevel:         kvrpcpb.IsolationLevel_SI,
		NotFillCache:           true,
		SyncLog:                true,
		RecordTimeStat:         true,
		RecordScanStat:         true,
		ReplicaRead:            true,
		ResolvedLocks:          []uint64{1, 2, 3},
		MaxExecutionDurationMs: 12,
		AppliedIndex:           13,
		TaskId:                 14,
		StaleRead:              true,
		ResourceGroupTag:       []byte("tag"),
		DiskFullOpt:            kvrpcpb.DiskFullOpt_AllowedOnAlmostFull,
		IsRetryRequest:         true,
		ApiVersion:             kvrpcpb.APIVersion_V2,
		CommittedLocks:         []uint64{4, 5},
		RequestSource:          "tidb",
		TxnSource:              15,
		BusyThresholdMs:        16,
		KeyspaceId:             17,
		BucketsVersion:         18,
		ClusterId:              19,
	}

	ctx.TraceContext = &tracepb.TraceContext{
		RemoteParentSpans: []*tracepb.RemoteParentSpan{
			{TraceId: 1001, SpanId: 2002},
			{TraceId: 3003, SpanId: 4004},
		},
		DurationThresholdMs: 250,
	}

	ctx.ResourceControlContext = &kvrpcpb.ResourceControlContext{
		ResourceGroupName: "rg",
		Penalty: &resource_manager.Consumption{
			RRU:               1.5,
			WRU:               2.5,
			ReadBytes:         3.5,
			WriteBytes:        4.5,
			TotalCpuTimeMs:    5.5,
			SqlLayerCpuTimeMs: 6.5,
			KvReadRpcCount:    7.5,
			KvWriteRpcCount:   8.5,
		},
		OverridePriority: 99,
	}

	ctx.SourceStmt = &kvrpcpb.SourceStmt{
		StartTs:      20,
		ConnectionId: 21,
		StmtId:       22,
		SessionAlias: "alias",
	}
	return ctx
}

func buildExecDetails() *kvrpcpb.ExecDetailsV2 {
	return &kvrpcpb.ExecDetailsV2{
		TimeDetail: &kvrpcpb.TimeDetail{
			WaitWallTimeMs:     1,
			ProcessWallTimeMs:  2,
			KvReadWallTimeMs:   3,
			TotalRpcWallTimeNs: 4,
		},
		ScanDetailV2: &kvrpcpb.ScanDetailV2{
			ProcessedVersions:         10,
			TotalVersions:             11,
			RocksdbDeleteSkippedCount: 12,
			RocksdbKeySkippedCount:    13,
			RocksdbBlockCacheHitCount: 14,
			RocksdbBlockReadCount:     15,
			RocksdbBlockReadByte:      16,
			ProcessedVersionsSize:     17,
			RocksdbBlockReadNanos:     18,
			GetSnapshotNanos:          19,
			ReadIndexProposeWaitNanos: 20,
			ReadIndexConfirmWaitNanos: 21,
			ReadPoolScheduleWaitNanos: 22,
		},
		WriteDetail: &kvrpcpb.WriteDetail{
			StoreBatchWaitNanos:        30,
			ProposeSendWaitNanos:       31,
			PersistLogNanos:            32,
			RaftDbWriteLeaderWaitNanos: 33,
			RaftDbSyncLogNanos:         34,
			RaftDbWriteMemtableNanos:   35,
			CommitLogNanos:             36,
			ApplyBatchWaitNanos:        37,
			ApplyLogNanos:              38,
			ApplyMutexLockNanos:        39,
			ApplyWriteLeaderWaitNanos:  40,
			ApplyWriteWalNanos:         41,
			ApplyWriteMemtableNanos:    42,
			LatchWaitNanos:             43,
			ProcessNanos:               44,
			ThrottleNanos:              45,
			PessimisticLockWaitNanos:   46,
		},
		TimeDetailV2: &kvrpcpb.TimeDetailV2{
			WaitWallTimeNs:           50,
			ProcessWallTimeNs:        51,
			ProcessSuspendWallTimeNs: 52,
			KvReadWallTimeNs:         53,
			TotalRpcWallTimeNs:       54,
			KvGrpcProcessTimeNs:      55,
			KvGrpcWaitTimeNs:         56,
		},
	}
}

func buildKeyError() *kvrpcpb.KeyError {
	return &kvrpcpb.KeyError{
		Locked: &kvrpcpb.LockInfo{
			PrimaryLock:            []byte("primary"),
			LockVersion:            101,
			Key:                    []byte("key"),
			LockTtl:                102,
			TxnSize:                103,
			LockType:               kvrpcpb.Op_Put,
			LockForUpdateTs:        104,
			UseAsyncCommit:         true,
			MinCommitTs:            105,
			Secondaries:            [][]byte{[]byte("s1"), []byte("s2")},
			DurationToLastUpdateMs: 106,
			IsTxnFile:              true,
		},
		Retryable: "retry",
		Abort:     "abort",
		Conflict: &kvrpcpb.WriteConflict{
			StartTs:          200,
			ConflictTs:       201,
			Key:              []byte("ckey"),
			Primary:          []byte("cprimary"),
			ConflictCommitTs: 202,
			Reason:           kvrpcpb.WriteConflict_PessimisticRetry,
		},
		AlreadyExist: &kvrpcpb.AlreadyExist{Key: []byte("dup")},
		CommitTsExpired: &kvrpcpb.CommitTsExpired{
			StartTs:           300,
			AttemptedCommitTs: 301,
			Key:               []byte("ekey"),
			MinCommitTs:       302,
		},
		TxnNotFound: &kvrpcpb.TxnNotFound{
			StartTs:    400,
			PrimaryKey: []byte("pkey"),
		},
		CommitTsTooLarge: &kvrpcpb.CommitTsTooLarge{CommitTs: 401},
		AssertionFailed: &kvrpcpb.AssertionFailed{
			StartTs:          500,
			Key:              []byte("akey"),
			Assertion:        kvrpcpb.Assertion_NotExist,
			ExistingStartTs:  501,
			ExistingCommitTs: 502,
		},
		PrimaryMismatch: &kvrpcpb.PrimaryMismatch{
			LockInfo: &kvrpcpb.LockInfo{
				PrimaryLock: []byte("pm"),
				Key:         []byte("pmk"),
			},
		},
		TxnLockNotFound: &kvrpcpb.TxnLockNotFound{Key: []byte("missing")},
	}
}

func TestGetRequestResponseRoundTrip(t *testing.T) {
	ctx := buildContext()
	req := &kvrpcpb.GetRequest{
		Context: ctx,
		Key:     []byte("hello"),
		Version: 123,
	}

	resp := &kvrpcpb.GetResponse{
		Error:         buildKeyError(),
		Value:         []byte("world"),
		NotFound:      false,
		ExecDetailsV2: buildExecDetails(),
	}

	arena := runtime.NewArena()
	defer arena.Free()

	reqRepr := kvrpcpbffi.NewReprGetRequestGenerated(arena, req)
	roundReq := kvrpcpbffi.FromReprGetRequestGenerated(reqRepr)
	if !reflect.DeepEqual(req, roundReq) {
		t.Fatalf("request mismatch: expected %#v, got %#v", req, roundReq)
	}

	respRepr := kvrpcpbffi.NewReprGetResponseGenerated(arena, resp)
	roundResp := kvrpcpbffi.FromReprGetResponseGenerated(respRepr)
	if !reflect.DeepEqual(resp, roundResp) {
		t.Fatalf("response mismatch: expected %#v, got %#v", resp, roundResp)
	}
}
