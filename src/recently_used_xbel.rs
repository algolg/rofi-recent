// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

/* 
 * This is a modified version of System76's recently-used-xbel package.
 *
 * This modification makes it possible to parse recently-used.xbel 
 * for more parameters than only those found in the original release.
 *
 * The original source can be found at 
 * https://github.com/pop-os/recently-used-xbel.
 */

use serde::Deserialize;
use std::path::PathBuf;

// Stores recently-opened files accessed by the desktop user.
#[derive(Debug, Clone, Deserialize)]
pub struct RecentlyUsed {
    // Files that have been recently used.
    #[serde(rename = "bookmark")]
    pub bookmarks: Vec<Bookmark>,
}

// A file that was recently opened by the desktop user.
#[derive(Debug, Clone, Deserialize)]
pub struct Bookmark {
    // The location of the file.
    pub href: String,
    // Info element.
    #[serde(rename = "info")]
    pub info: Info,
}

// Info element.
#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    // Metadata element.
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}

// Metadata element.
#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    // Contains data with type.
    #[serde(rename = "mime-type")]
    pub mime_type: MimeType,
    // Contains applications.
    #[serde(rename = "applications")]
    pub app_parent: ApplicationParent,
}

// Contains data with type.
#[derive(Debug, Clone, Deserialize)]
pub struct MimeType {
    #[serde(rename = "type")]
    pub mimetype: String,
}

// Parent of Application elements.
#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationParent {
    // Contains Application elements.
    #[serde(rename = "application")]
    pub apps: Vec<Application>,
}

// Stores data about application.
#[derive(Debug, Clone, Deserialize)]
pub struct Application {
    // Full application name.
    pub name: String,
    // Shell command to execute application.
    pub exec: String,
}

// An error that can occur when accessing recently-used files.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("~/.local/share/recently-used.xbel: file does not exist")]
    DoesNotExist,
    #[error("~/.local/share/recently-used.xbel: could not deserialize")]
    Deserialization(#[source] serde_xml_rs::Error),
}

// The path where the recently-used.xbel file is expected to be found.
pub fn dir() -> Option<PathBuf> {
    dirs::home_dir().map(|dir| dir.join(".local/share/recently-used.xbel"))
}

// Convenience function for parsing the recently-used.xbel file in its default location.
pub fn parse_file() -> Result<RecentlyUsed, Error> {
    let path = dir().ok_or(Error::DoesNotExist)?;
    let file = std::fs::File::open(&*path).map_err(|_| Error::DoesNotExist)?;
    serde_xml_rs::from_reader(file).map_err(Error::Deserialization)
}
