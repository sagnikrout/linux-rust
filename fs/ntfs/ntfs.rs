//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/ntfs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Defines for NTFS Linux kernel driver.
//
// Copyright (c) 2001-2014 Anton Altaparmakov and Tuxera Inc.
// Copyright (C) 2002 Richard Russon
// Copyright (c) 2025 LG Electronics Co., Ltd.
//

//
// Default pre-allocation size is optimize runlist merge overhead
// with small chunk size.
//
pub const NTFS_DEF_PREALLOC_SIZE: c_int = 65536;
//
// The log2 of the standard number of clusters per compression block.
// A value of 4 corresponds to 16 clusters (1 << 4), which is the
// default chunk size used by NTFS LZNT1 compression.
//
pub const STANDARD_COMPRESSION_UNIT: c_int = 4;
//
// The maximum cluster size (4KB) allowed for compression to be enabled.
// By design, NTFS does not support compression on volumes where the
// cluster size exceeds 4096 bytes.
//
pub const MAX_COMPRESSION_CLUSTER_SIZE: c_int = 4096;

//
// Conversion helpers for NTFS units.
//
// Convert bytes to cluster count
// Convert cluster count to bytes
// Get the byte offset within a cluster from a linear byte address
// Calculate the physical cluster number containing a specific MFT record.
// Calculate the folio index where the MFT record resides.
// Calculate the byte offset within a folio for an MFT record.
// Convert folio index to cluster number.
// Convert cluster number to folio index.
// Get the byte offset within a folio from a cluster number
// Convert a byte offset on the volume to a bio sector number.
// Global variables.
// Slab caches (from super.c).
// The various operations structs defined throughout the driver files.
//
// NTFS_SB - return the ntfs volume given a vfs super block
// @sb:		VFS super block
//
// NTFS_SB() returns the ntfs volume associated with the VFS super block @sb.
//
// Declarations of functions and global variables.
// From fs/ntfs/compress.c
extern "C" {
    pub fn ntfs_read_compressed_block(folio: *mut folio) -> c_int;
}

extern "C" {
    pub fn ntfs_read_wof_compressed_block(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ntfs_wof_free_workspaces();
}

extern "C" {
    pub fn allocate_compression_buffers() -> c_int;
}
extern "C" {
    pub fn free_compression_buffers();
}
// From fs/ntfs/super.c
pub const default_upcase_len: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct option_t {
    pub val: c_int,
    pub str: *mut c_char,
}

extern "C" {
    pub fn ntfs_set_volume_flags(vol: *mut ntfs_volume, flags: __le16) -> c_int;
}
extern "C" {
    pub fn ntfs_clear_volume_flags(vol: *mut ntfs_volume, flags: __le16) -> c_int;
}
extern "C" {
    pub fn ntfs_write_volume_label(vol: *mut ntfs_volume, label: *mut c_char) -> c_int;
}
// From fs/ntfs/mst.c
extern "C" {
    pub fn post_read_mst_fixup(b: *mut ntfs_record, size: u32) -> c_int;
}
extern "C" {
    pub fn pre_write_mst_fixup(b: *mut ntfs_record, size: u32) -> c_int;
}
extern "C" {
    pub fn post_write_mst_fixup(b: *mut ntfs_record);
}
// From fs/ntfs/unistr.c
extern "C" {
    pub fn ntfs_ucsncmp(s1: *const __le16, s2: *const __le16, n: usize) -> c_int;
}
extern "C" {
    pub fn ntfs_force_shutdown(sb: *mut super_block, flags: u32) -> c_int;
}
extern "C" {
    pub fn ntfs_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}

// From fs/ntfs/upcase.c
// From fs/ntfs/bdev-io.c
extern "C" {
    pub fn ntfs_bdev_read(bdev: *mut block_device, data: *mut c_char, start: loff_t, size: usize) -> c_int;
}
extern "C" {
    pub fn ntfs_bdev_write(sb: *mut super_block, buf: *mut c_void, start: loff_t, size: usize) -> c_int;
}
