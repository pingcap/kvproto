# Agent Instructions for kvproto

## Backup Metadata Compatibility Rule (brpb)

When editing backup metadata schema in `proto/brpb.proto`, use:
`.agents/skills/backup-meta-schema-version/SKILL.md`.

related structures:
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

Any wire-format/semantics change to the messages above requires
`BackupSchemaVersion` to be incremented in the same PR
(`pkg/brpb/backup_schema_version.go`).
