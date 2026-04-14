# Agent Instructions for kvproto

## Backup Metadata Compatibility Rule (brpb)

When editing backup metadata schema in `proto/brpb.proto`, handle
`BackupSchemaVersion` explicitly.

### What counts as backup metadata schema

Any wire-format change in `backup.BackupMeta` and messages reachable from it,
for example:

- `BackupMeta`
- `BackupRange`
- `TableMeta`
- `File`
- `MetaFile`
- `PlacementPolicy`
- `StatsFileIndex`
- `Schema`
- `IDMap`
- `PitrTableMap`
- `PitrDBMap`
- `RawRange`

Typical wire-format changes include adding/removing fields, changing field
types, changing field numbers, and changing nested message/enum definitions
used by `BackupMeta`.

### Required actions

1. Bump `BackupSchemaVersion` in `pkg/brpb/backup_schema_version.go`.
2. Update or add the schema fingerprint mapping in
   `pkg/brpb/backup_schema_version_guard_test.go`.
3. Run:
   `GOCACHE=/tmp/go-build-cache go test ./pkg/brpb -run TestBackupMetaSchemaVersionGuard -count=1`

### Notes

- Do not rewrite historical fingerprint entries for old versions.
- Pure comment/formatting changes do not require a version bump.
