//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/zip.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Represents an open zip archive.
// Only basic ZIP files are supported, in particular the following are not
// supported:
// - encryption
// - streaming
// - multi-part ZIP files
// - ZIP64
//
// Carries information on name, compression method, and data corresponding to a
// file in a zip archive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zip_entry {
// Compression method as defined in pkzip spec. 0 means data is uncompressed.
    pub compression: __u16,
// Non-null terminated name of the file.
    pub name: *const c_char,
// Length of the file name.
    pub name_length: __u16,
// Pointer to the file data.
    pub data: *const c_void,
// Length of the file data.
    pub data_length: __u32,
// Offset of the file data within the archive.
    pub data_offset: __u32,
}

// Open a zip archive. Returns NULL in case of an error.
// Close a zip archive and release resources.
extern "C" {
    pub fn zip_archive_close(archive: *mut zip_archive);
}
// Look up an entry corresponding to a file in given zip archive.
extern "C" {
    pub fn zip_archive_find_entry(archive: *mut zip_archive, name: *const c_char, out: *mut zip_entry) -> c_int;
}
