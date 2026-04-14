---
name: backup-meta-schema-version
description: Use when editing backup metadata schema in proto/brpb.proto to ensure BackupSchemaVersion is updated when required.
---

# Backup Metadata Schema Version

## Overview

Use this skill whenever a change may affect backup metadata wire format or
restore semantics in `proto/brpb.proto` for `backup.BackupMeta` and messages
reachable from it.

Covered metadata messages:

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

## Required workflow

1. Apply schema changes in `proto/brpb.proto`.
2. If the change affects backup metadata format/semantics, increment
   `BackupSchemaVersion` in `pkg/brpb/backup_schema_version.go`
   (one bump per PR is sufficient).

## Non-schema changes

No version bump is needed for comment-only or formatting-only edits.
