//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/btt.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Block Translation Table library
// Copyright (c) 2014-2015, Intel Corporation.
//

pub const BTT_SIG_LEN: c_int = 16;

pub const MAP_ENT_SIZE: c_int = 4;
pub const MAP_TRIM_SHIFT: c_int = 31;

pub const MAP_ERR_SHIFT: c_int = 30;

pub const MAP_ENT_NORMAL: c_uint = 0xC0000000;

pub const RTT_INVALID: c_int = 0;
pub const BTT_PG_SIZE: c_int = 4096;

pub const LOG_SEQ_INIT: c_int = 1;
pub const IB_FLAG_ERROR: c_uint = 0x00000001;
pub const IB_FLAG_ERROR_MASK: c_uint = 0x00000001;

// 'normal' is both e and z flags set

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btt_init_state {
    INIT_UNCHECKED = 0,
    INIT_NOTFOUND,
    INIT_READY
}

//
// A log group represents one log 'lane', and consists of four log entries.
// Two of the four entries are valid entries, and the remaining two are
// padding. Due to an old bug in the padding location, we need to perform a
// test to determine the padding scheme being used, and use that scheme
// thereafter.
//
// In kernels prior to 4.15, 'log group' would have actual log entries at
// indices (0, 2) and padding at indices (1, 3), where as the correct/updated
// format has log entries at indices (0, 1) and padding at indices (2, 3).
//
// Old (pre 4.15) format:
// +-----------------+-----------------+
// |      ent[0]     |      ent[1]     |
// |       16B       |       16B       |
// | lba/old/new/seq |       pad       |
// +-----------------------------------+
// |      ent[2]     |      ent[3]     |
// |       16B       |       16B       |
// | lba/old/new/seq |       pad       |
// +-----------------+-----------------+
//
// New format:
// +-----------------+-----------------+
// |      ent[0]     |      ent[1]     |
// |       16B       |       16B       |
// | lba/old/new/seq | lba/old/new/seq |
// +-----------------------------------+
// |      ent[2]     |      ent[3]     |
// |       16B       |       16B       |
// |       pad       |       pad       |
// +-----------------+-----------------+
//
// We detect during start-up which format is in use, and set
// arena->log_index[(0, 1)] with the detected format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_entry {
    pub lba: __le32,
    pub old_map: __le32,
    pub new_map: __le32,
    pub seq: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct log_group {
    pub ent: [log_entry; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btt_sb {
    pub signature: [u8; BTT_SIG_LEN],
    pub uuid: [u8; 16],
    pub parent_uuid: [u8; 16],
    pub flags: __le32,
    pub version_major: __le16,
    pub version_minor: __le16,
    pub external_lbasize: __le32,
    pub external_nlba: __le32,
    pub internal_lbasize: __le32,
    pub internal_nlba: __le32,
    pub nfree: __le32,
    pub infosize: __le32,
    pub nextoff: __le64,
    pub dataoff: __le64,
    pub mapoff: __le64,
    pub logoff: __le64,
    pub info2off: __le64,
    pub padding: [u8; 3968],
    pub checksum: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct free_entry {
    pub block: u32,
    pub sub: u8,
    pub seq: u8,
    pub has_err: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aligned_lock {
    pub lock: spinlock_t,
    pub cacheline_padding: [u8; L1_CACHE_BYTES],
}

//
// struct arena_info - handle for an arena
// @size:		Size in bytes this arena occupies on the raw device.
// This includes arena metadata.
// @external_lba_start:	The first external LBA in this arena.
// @internal_nlba:	Number of internal blocks available in the arena
// including nfree reserved blocks
// @internal_lbasize:	Internal and external lba sizes may be different as
// we can round up 'odd' external lbasizes such as 520B
// to be aligned.
// @external_nlba:	Number of blocks contributed by the arena to the number
// reported to upper layers. (internal_nlba - nfree)
// @external_lbasize:	LBA size as exposed to upper layers.
// @nfree:		A reserve number of 'free' blocks that is used to
// handle incoming writes.
// @version_major:	Metadata layout version major.
// @version_minor:	Metadata layout version minor.
// @sector_size:	The Linux sector size - 512 or 4096
// @nextoff:		Offset in bytes to the start of the next arena.
// @infooff:		Offset in bytes to the info block of this arena.
// @dataoff:		Offset in bytes to the data area of this arena.
// @mapoff:		Offset in bytes to the map area of this arena.
// @logoff:		Offset in bytes to the log area of this arena.
// @info2off:		Offset in bytes to the backup info block of this arena.
// @freelist:		Pointer to in-memory list of free blocks
// @rtt:		Pointer to in-memory "Read Tracking Table"
// @map_locks:		Spinlocks protecting concurrent map writes
// @nd_btt:		Pointer to parent nd_btt structure.
// @list:		List head for list of arenas
// @debugfs_dir:	Debugfs dentry
// @flags:		Arena flags - may signify error states.
// @err_lock:		Mutex for synchronizing error clearing.
// @log_index:		Indices of the valid log entries in a log_group
//
// arena_info is a per-arena handle. Once an arena is narrowed down for an
// IO, this struct is passed around for the duration of the IO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arena_info {
    pub /: *mut *mut u64 size; / Total bytes for this arena,
    pub external_lba_start: u64,
    pub internal_nlba: u32,
    pub internal_lbasize: u32,
    pub external_nlba: u32,
    pub external_lbasize: u32,
    pub nfree: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub sector_size: u32,
// Byte offsets to the different on-media structures
    pub nextoff: u64,
    pub infooff: u64,
    pub dataoff: u64,
    pub mapoff: u64,
    pub logoff: u64,
    pub info2off: u64,
// Pointers to other in-memory structures for this arena
    pub freelist: *mut free_entry,
    pub rtt: *mut u32,
    pub map_locks: *mut aligned_lock,
    pub nd_btt: *mut nd_btt,
    pub list: list_head,
    pub debugfs_dir: *mut dentry,
// Arena flags
    pub flags: u32,
    pub err_lock: mutex,
    pub log_index: [c_int; 2],
}

//
// struct btt - handle for a BTT instance
// @btt_disk:		Pointer to the gendisk for BTT device
// @arena_list:		Head of the list of arenas
// @debugfs_dir:	Debugfs dentry
// @nd_btt:		Parent nd_btt struct
// @nlba:		Number of logical blocks exposed to the	upper layers
// after removing the amount of space needed by metadata
// @rawsize:		Total size in bytes of the available backing device
// @lbasize:		LBA size as requested and presented to upper layers.
// This is sector_size + size of any metadata.
// @sector_size:	The Linux sector size - 512 or 4096
// @nd_region:		&struct nd_region pointer
// @init_lock:		Mutex used for the BTT initialization
// @init_state:		Flag describing the initialization state for the BTT
// @num_arenas:		Number of arenas in the BTT instance
// @phys_bb:		Pointer to the namespace's badblocks structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btt {
    pub btt_disk: *mut gendisk,
    pub arena_list: list_head,
    pub debugfs_dir: *mut dentry,
    pub nd_btt: *mut nd_btt,
    pub nlba: u64,
    pub rawsize: c_ulonglong,
    pub lbasize: u32,
    pub sector_size: u32,
    pub nd_region: *mut nd_region,
    pub init_lock: mutex,
    pub init_state: c_int,
    pub num_arenas: c_int,
    pub phys_bb: *mut badblocks,
}

extern "C" {
    pub fn nd_btt_arena_is_valid(nd_btt: *mut nd_btt, super: *mut btt_sb) -> bool;
}
