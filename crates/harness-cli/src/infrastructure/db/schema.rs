use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension};

use crate::infrastructure::errors::{HarnessInfraError, Result};

pub fn schema_version(connection: &Connection) -> Result<i64> {
    let version = connection
        .query_row(
            "SELECT COALESCE(MAX(version),0) FROM schema_version;",
            [],
            |row| row.get::<_, i64>(0),
        )
        .optional()?
        .unwrap_or(0);
    Ok(version)
}

pub fn apply_schema_v1(schema_dir: &Path, connection: &Connection) -> Result<()> {
    let schema_path = schema_dir.join("001-init.sql");
    if !schema_path.exists() {
        return Err(HarnessInfraError::MissingSchema(
            schema_path.display().to_string(),
        ));
    }

    let schema = fs::read_to_string(schema_path)?;
    connection.execute_batch(&schema)?;
    Ok(())
}

pub fn apply_pending_migrations(
    schema_dir: &Path,
    connection: &Connection,
    current_version: i64,
) -> Result<Vec<i64>> {
    let mut applied = Vec::new();
    for (version, path) in migration_files(schema_dir)? {
        if version > current_version {
            let sql = fs::read_to_string(path)?;
            connection.execute_batch(&sql)?;
            applied.push(version);
        }
    }
    Ok(applied)
}

pub fn migration_files(schema_dir: &Path) -> Result<Vec<(i64, PathBuf)>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(schema_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("sql") {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        let Some(prefix) = file_name.split('-').next() else {
            continue;
        };
        let Ok(version) = prefix.trim_start_matches('0').parse::<i64>() else {
            continue;
        };
        files.push((version, path));
    }
    files.sort_by_key(|(version, _)| *version);
    Ok(files)
}
