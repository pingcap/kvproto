# DB9 Coprocessor Protocol Design

Status: Draft

Related PRs:
- `pingcap/kvproto#1439`
- `tidbcloud/cloud-storage-engine#4821`

## Summary

This document describes the wire contract introduced by
`proto/db9_coprocessor.proto`.

The goal is to let a DB9 client reuse TiKV's existing coprocessor request
lifecycle while carrying DB9-native scan metadata, expression trees, type
information, rows, and execution stats. The protocol is transported inside the
existing `coprocessor.Request` envelope instead of introducing a new RPC
service.

## Goals

- Reuse the existing TiKV coprocessor routing, region clipping, admission, and
  response lifecycle.
- Keep DB9 semantics separate from TiDB/MySQL-specific `tipb::DagRequest`
  semantics.
- Carry enough schema and type metadata for the storage-side DB9 executor to
  decode rows, evaluate pushed expressions, and encode results.
- Make downstream `kvproto` patching temporary by upstreaming the DB9 protobuf
  contract itself.

## Non-Goals

- Define the DB9 planner, optimizer, or storage-side executor implementation.
- Introduce a new gRPC service for DB9 requests.
- Make DB9 use `tipb` semantics or fit DB9 types into TiDB/MySQL expression
  contracts.
- Cover the full DB9 type/function surface in the MVP protocol.
- Define a streaming DB9 result protocol in this change.

## Why A Separate Proto

`tipb::DagRequest` is coupled to TiDB/MySQL planner and expression semantics.
DB9 needs a different type system, function namespace, row codec, and scan
metadata. Reusing `tipb` would either:

- force lossy mapping from DB9 concepts into TiDB/MySQL concepts, or
- require DB9-specific extensions inside `tipb` that would make an existing
  shared protocol harder to reason about.

Adding a dedicated `db9_coprocessor` package keeps the new wire contract
isolated and explicit, while still reusing the outer TiKV coprocessor transport
path.

## Transport Envelope

The DB9 payload is carried inside the existing `coprocessor.Request`:

- `coprocessor.Request.tp = REQ_TYPE_DB9_DAG`
- `coprocessor.Request.ranges = concrete region-clipped key ranges`
- `coprocessor.Request.data = serialized Db9DagRequest`

This split is intentional:

- the outer envelope continues to own routing and physical key ranges
- the inner DB9 payload owns logical scan shape and DB9 execution metadata

That means DB9 does not add a new RPC or region-routing path in `kvproto`.

## Request Contract

`Db9DagRequest` is the unary request payload for the MVP path.

### Core request identity

- `codec_version`: DB9-local wire contract version, starting from `1`
- `db_id`: tenant/database identity used by the downstream DB9 row and index
  key prefixes

### Schema metadata

`Db9TableInfo` carries the minimum schema snapshot the storage-side executor
needs:

- `table_name`
- `table_id`
- `schema_version`
- `columns`
- `pk_indices`
- `indexes`

This avoids tying storage-side evaluation to a separate schema fetch at request
execution time.

### Scan shape

`Db9ScanSpec` describes the logical scan:

- `kind`
- `index_id`
- `index_name`
- `desc`
- `require_row_fetch`

Physical range clipping still comes from `coprocessor.Request.ranges`.

`require_row_fetch` exists because the MVP index path is not yet a covering
index protocol. The current expectation is:

1. scan index keys
2. resolve base-row identity
3. fetch table rows
4. evaluate selection and projection on the row

### Expressions and projection

DB9 expressions are represented by `Db9Expr` and its `oneof` payload:

- constant
- column reference
- binary expression
- unary expression
- cast
- function call
- `IS NULL` / `IS NOT NULL`

`Db9NamedExpr` is used for projection output naming.

The protocol carries DB9-native expression trees instead of TiDB/MySQL scalar
function signatures.

### Limit

`Db9LimitSpec` carries a pushed limit when present.

Absent means "no pushed limit" rather than `0`.

### Aggregate

`Db9AggregatePlan` carries optional storage-side partial aggregation metadata:

- `mode`
- `group_keys`
- `aggregates`

The initial contract is intentionally narrow:

- `mode = DB9_AGGREGATE_MODE_PARTIAL_ONLY`
- `group_keys` may be empty for plain aggregates
- each `Db9AggregateSpec` carries:
  - `function_name`
  - optional `arg` (`COUNT(*)` leaves this absent)
  - `partial_state_types`

`partial_state_types` makes the row contract explicit for aggregates like `AVG`
that need more than one storage-side state slot.

This request field is additive and optional so existing scalar-only DB9 cop
requests remain unchanged.

## Response Contract

`Db9SelectResponse` is the unary response payload for the MVP path:

- `rows`
- `stats`
- `warnings`

`Db9ExecStats` currently reports:

- `scanned_keys`
- `scanned_rows`
- `decoded_rows`
- `returned_rows`

The protocol intentionally starts with a unary response shape. Streaming DB9
responses are out of scope for this change and are expected to be rejected by
the downstream executor until a dedicated streaming contract exists.

For aggregate pushdown, the protocol continues to reuse `Db9SelectResponse.rows`.
Grouped partial rows are expected to be laid out as:

- `[group_keys..., partial_states...]`

Final merge semantics remain outside `kvproto`; this schema only describes how
partial aggregate work is requested and returned.

## Type System And Wire Values

`Db9TypeKind` models the DB9 type namespace. The initial enum includes the core
types needed by the MVP path plus several future-facing kinds so downstream
messages can refer to them explicitly without overloading TiDB/MySQL types.

`Db9Value` intentionally carries only the subset of wire values needed by the
current downstream executor:

- `null_value`
- `bool_value`
- `int32_value`
- `int64_value`
- `float64_value`
- `text_value`
- `bytes_value`
- `timestamp_value`
- `timestamptz_value`

The timestamp additions use new oneof field numbers:

- `timestamp_value = 8`
- `timestamptz_value = 9`

Both are encoded as milliseconds since Unix epoch. Their semantic distinction is
carried by the specific oneof arm and the surrounding DB9 type metadata.

## Compatibility

This change is intentionally low-risk for existing `kvproto` consumers:

- it adds a new proto package instead of modifying an existing widely-shared
  request contract
- it does not change any existing RPC service definition
- it adds fields only inside the new DB9 message set
- later additive growth keeps using new field numbers rather than rewriting
  existing ones, including the timestamp and aggregate extensions

Expected evolution rules:

- additive protocol growth should use new field numbers
- incompatible DB9-local interpretation changes should be gated by
  `codec_version`
- old consumers that do not use `db9_coprocessor` remain unaffected

## Alternatives Considered

### Extend `tipb::DagRequest`

Rejected because DB9 semantics are not a natural fit for the TiDB/MySQL planner
and execution model.

### Use opaque bytes or JSON inside an existing request type

Rejected because it would hide protocol structure from generated bindings and
remove normal protobuf compatibility discipline.

### Add a dedicated DB9 gRPC service

Rejected for the MVP because the existing TiKV coprocessor envelope already
solves request routing and physical range ownership.

## Follow-Ups

- extend `Db9Value` for more DB9 types as downstream support expands
- define a streaming response contract if DB9 needs streaming coprocessor
  execution
- add covering-index protocol support when the downstream executor no longer
  needs unconditional row fetch on index scans
- revisit shared abstractions only if more DB9-specific request families need to
  be upstreamed into `kvproto`
