//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/bio.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
// Copyright (C) 2022 Christoph Hellwig.
//

pub const BTRFS_BIO_INLINE_CSUM_SIZE: c_int = 64;
extern "C" {
    pub fn void(bbio: *mut *mut btrfs_bio_end_io_t)(struct btrfs_bio) -> typedef;
}
//
// Highlevel btrfs I/O structure.  It is allocated by btrfs_bio_alloc and
// passed to btrfs_submit_bbio() for mapping to the physical devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_bio {
//
// Inode and offset into it that this I/O operates on.
//
// If the inode is a data one, csum verification and read-repair
// will be done automatically.
// If the inode is a metadata one, everything is handled by the caller.
//
    pub inode: *mut btrfs_inode,
    pub file_offset: u64,
//
// For data reads: checksumming and original I/O information.
// (for internal use in the btrfs_submit_bbio() machinery only)
//
    pub csum: *mut u8,
    pub csum_inline: [u8; BTRFS_BIO_INLINE_CSUM_SIZE],
    pub saved_iter: bvec_iter,
}

//
// For data writes:
// - ordered extent covering the bio
// - pointer to the checksums for this bio
// - original physical address from the allocator
// (for zone append only)
// - original logical address, used for checksumming fscrypt bios
//
// For metadata reads: parentness verification.
// For internal use in read end I/O handling
// End I/O information supplied to btrfs_bio_alloc
// Save the first error status of split bio.
// Use the commit root to look up csums (data read bio only).
//
// Since scrub will reuse btree inode, we need this flag to distinguish
// scrub bios.
//
// Whether the bio is coming from copy_remapped_data_io().
// Whether the csum generation for data write is async.
// Whether the bio is written using zone append.
//
// This member must come last, bio_alloc_bioset will allocate enough
// bytes for entire btrfs_bio but relies on bio being last.
//
extern "C" {
    pub fn container_of(_arg: bio, btrfs_bio: struct, _arg: bio) -> return;
}
extern "C" {
    pub fn btrfs_bioset_init() -> int __init;
}
extern "C" {
    pub fn btrfs_bioset_exit() -> void __cold;
}
extern "C" {
    pub fn btrfs_bio_end_io(bbio: *mut btrfs_bio, status: blk_status_t);
}
// Submit using blkcg_punt_bio_submit.

extern "C" {
    pub fn btrfs_submit_bbio(bbio: *mut btrfs_bio, mirror_num: c_int);
}
extern "C" {
    pub fn btrfs_submit_repair_write(bbio: *mut btrfs_bio, mirror_num: c_int, dev_replace: bool);
}
