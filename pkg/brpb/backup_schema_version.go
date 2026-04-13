// Copyright 2026 PingCAP, Inc.
// Backup Version Control, required mannually bump when backup meta schema changes.

package backup

const (
	// BackupSchemaVersion is the compatibility version of BackupMeta schema.
	// 0 means the backup was created before this field existed.
	// When adding a new field in BackupMeta or nested backup metadata messages
	// that affects backup meta format/semantics, bump this version manually.
	BackupSchemaVersion uint32 = 1
)
