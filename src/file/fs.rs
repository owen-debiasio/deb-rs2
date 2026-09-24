//! Shared functions and variables that assist with returning filesystem
//! properties.
//!
//! Copyright (C) 2026 Owen Debiasio <owen.debiasio@gmail.com>
//! SPDX-License-Identifier: GPL-3.0-or-later

use flate2::read::GzDecoder;
use std::{
    fs::{File, create_dir_all},
    io::{Error, copy},
    path::Path,
    str::from_utf8,
};
use tar::Archive;
use xz2::read::XzDecoder;

/// Extracts an archive using `tar`.
///
/// You just need to input the path of where it outputs to (`path` ([`str`])).
/// Extracts it using `archive_exec` ([`str`]) to the current working directory.
///
/// `archive_type` ([`str`]) determines which archive format to use.
///
/// Chooses one of the following:
///     - `gz`
///     - `xz`
///     - Leave empty for regular tar format
///
/// Using tar.gz:
/// ```
/// let archive = "archive.tar.gz";
/// let dest = ".";
/// let archive_type = "gz";
///
/// extract_tar_archive(archive, dest, archive_type);
/// ```
///
/// Using tar.xz:
/// ```
/// let archive = "archive.tar.xz";
/// let dest = ".";
/// let archive_type = "xz";
///
/// extract_tar_archive(archive, dest, archive_type);
/// ```
///
/// Other
/// ```
/// let archive = "archive.tar";
/// let dest = ".";
/// let archive_type = "";
///
/// extract_tar_archive(archive, dest, archive_type);
/// ```
pub fn extract_tar_archive(path: &str, dest: &str, archive_type: &str) -> Result<(), Error> {
    let archive_path = File::open(path)?;

    if !matches!(archive_type, "gz" | "xz" | "") {
        panic!("Invalid coded archive type: {archive_type}")
    }

    if archive_type == "gz" {
        Archive::new(GzDecoder::new(archive_path)).unpack(dest)?;
    } else if archive_type == "xz" {
        Archive::new(XzDecoder::new(archive_path)).unpack(dest)?;
    } else {
        Archive::new(archive_path).unpack(dest)?;
    };

    Ok(())
}

/// Extracts an archive using `ar`.
///
/// Takes the selected archive (`path` ([`str`])), then extracts it to `dest` ([`str`]).
///
/// ```
/// let path_of_archive = "archive.ar";
/// let destination = "dir/archive"
/// extract_ar_archive(path_of_archive, destination);
/// ```
pub fn extract_ar_archive(path: &str, dest: &str) -> Result<(), Error> {
    create_dir_all(dest)?;

    let mut archive = ar::Archive::new(File::open(path)?);

    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result?;

        let filename = from_utf8(entry.header().identifier())
            .expect("Failed to read header")
            .trim();

        let output = Path::new(dest).join(filename);

        let mut file = File::create(output)?;

        copy(&mut entry, &mut file)?;
    }

    Ok(())
}
