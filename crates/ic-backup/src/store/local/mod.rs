//! Local store layout, private file admission, publication and bounded inventory traversal.

use super::{BackupSummary, BackupTarget, ScanLimits, StoreError, manifest};
use ic_host_artifacts::Length;
use ic_host_fs::{Access, Directory, FilePolicy, Kind as EntryKind};
use std::{ffi::OsStr, fs::File, path::Path};

pub(super) const FORMAT: &[u8] = b"ic-backup/host-store/1\n";
pub(super) const PRIVATE: u32 = 0o077;
pub(super) const FILE_POLICY: FilePolicy = FilePolicy {
    forbidden: 0o177,
    single_link: true,
};
pub(super) const MANIFEST_LIMIT: usize = 1024 * 1024;

pub(super) fn initialize(path: &Path) -> Result<Directory, StoreError> {
    let root = Directory::open(path, PRIVATE)?.ok_or(StoreError::MissingStore)?;
    match root.entries(1) {
        Ok(entries) if entries.is_empty() => (),
        Ok(_) | Err(ic_host_fs::Error::EntryLimit) => return Err(StoreError::Occupied),
        Err(error) => return Err(error.into()),
    }
    let stage = root
        .stage(
            OsStr::new("format.pending"),
            OsStr::new("format"),
            ic_host_fs::Policy {
                destination: ic_host_fs::Destination::CreateNew,
                permissions: 0o600,
                retention: ic_host_fs::Retention::Retain,
            },
        )?
        .write_from(FORMAT, Length::Exactly(FORMAT.len() as u64))?
        .seal()?;
    for name in ["bundles", "operations", "targets"] {
        root.create_directory(OsStr::new(name), 0o700, PRIVATE)?;
    }
    commit(stage)?;

    Ok(root)
}

pub(super) fn open(path: &Path) -> Result<Directory, StoreError> {
    let root = Directory::open(path, PRIVATE)?.ok_or(StoreError::MissingStore)?;
    let Some(format) = read_optional(&root, "format", 128)? else {
        return Err(StoreError::IncompleteStore);
    };
    if format != FORMAT {
        return Err(StoreError::UnsupportedFormat);
    }
    let mut count = 0;
    for entry in root.entries(4)? {
        let expected = match entry.name.to_str() {
            Some("format") => EntryKind::File,
            Some("bundles" | "operations" | "targets") => EntryKind::Directory,
            _ => return Err(StoreError::InvalidLayout),
        };
        if entry.kind != expected {
            return Err(StoreError::InvalidLayout);
        }
        if expected == EntryKind::Directory {
            root.open_directory(&entry.name, PRIVATE)?
                .ok_or(StoreError::IncompleteStore)?;
        }
        count += 1;
    }
    if count != 4 {
        return Err(StoreError::IncompleteStore);
    }

    Ok(root)
}

pub(super) fn scan(
    root: &Directory,
    target: Option<&BackupTarget>,
    limits: ScanLimits,
    mut accept: impl FnMut(BackupSummary) -> Result<(), StoreError>,
) -> Result<(), StoreError> {
    let bundles = root
        .open_directory(OsStr::new("bundles"), PRIVATE)?
        .ok_or(StoreError::IncompleteStore)?;
    let mut remaining = limits.bytes.get();
    for entry in bundles.entries(limits.entries.get())? {
        let id = entry
            .name
            .to_str()
            .filter(|id| manifest::lower_hex(id, 32))
            .ok_or(StoreError::InvalidLayout)?;
        if entry.kind != EntryKind::Directory {
            return Err(StoreError::InvalidLayout);
        }
        let bundle = bundles
            .open_directory(&entry.name, PRIVATE)?
            .ok_or(StoreError::InvalidLayout)?;
        let Some(bytes) = read_optional(&bundle, "manifest.json", MANIFEST_LIMIT.min(remaining))?
        else {
            continue;
        };
        remaining -= bytes.len();
        let row = manifest::decode(id, &bytes)
            .map_err(|source| StoreError::Manifest {
                bundle: id.into(),
                source,
            })?
            .summary;

        if target.is_none_or(|target| *target == row.target) {
            accept(row)?;
        }
    }

    Ok(())
}

pub(super) fn read_optional(
    root: &Directory,
    name: &str,
    limit: usize,
) -> Result<Option<Vec<u8>>, StoreError> {
    open_optional(root, name)?
        .map(|file| {
            ic_host_artifacts::read(file, Length::AtMost(limit as u64))
                .map(|(bytes, _)| bytes)
                .map_err(StoreError::Read)
        })
        .transpose()
}

pub(super) fn open_optional(root: &Directory, name: &str) -> Result<Option<File>, StoreError> {
    Ok(root.open_file(OsStr::new(name), Access::Read, FILE_POLICY)?)
}

pub(super) fn commit(stage: ic_host_fs::Sealed) -> Result<(), StoreError> {
    match stage.commit() {
        ic_host_fs::Commit::Committed(_) => Ok(()),
        ic_host_fs::Commit::NotPublished { cause, .. } => Err(StoreError::NotPublished {
            cause,
            cleanup: None,
        }),
        ic_host_fs::Commit::PublishedUnconfirmed {
            receipt,
            phase,
            cause,
            sync_error,
        } => Err(StoreError::PublishedUnconfirmed {
            receipt: Box::new(receipt),
            phase,
            cause,
            sync_error,
        }),
    }
}
