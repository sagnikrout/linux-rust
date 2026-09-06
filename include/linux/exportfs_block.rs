//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/exportfs_block.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2014-2026 Christoph Hellwig.
//
// Support for exportfs-based layout grants for direct block device access.
//
pub const LINUX_EXPORTFS_BLOCK_H: c_int = 1;

//
// There are the two types of block-style layout support:
// - In-band implies a device identified by a unique cookie inside the actual
// device address space checked by the ->get_uuid method as used by the pNFS
// block layout.  This is a bit dangerous and deprecated.
// - Out of band implies identification by out of band unique identifiers
// specified by the storage protocol, which is much safer and used by the
// pNFS SCSI/NVMe layouts.
//
pub type expfs_block_layouts_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exportfs_block_ops {
//
// Returns the EXPFS_BLOCK_* bitmap of supported layout types.
//
    pub sb): *mut *mut expfs_block_layouts_t (layouts_supported)(struct super_block,
//
// Get the in-band device unique signature exposed to clients.
//
    pub offset): *mut *mut *mut *mut *mut int (get_uuid)(struct super_block sb, u8 buf, u32 len, u64,
//
// Map blocks for direct block access.
// If @write is %true, also allocate the blocks for the range if needed.
//
    pub device_generation): *mut u32,
//
// Commit blocks previously handed out by ->map_blocks and written to by
// the client.
//
    pub new_size): int nr_iomaps, loff_t,
}

