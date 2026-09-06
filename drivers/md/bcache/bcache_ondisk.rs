//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/bcache_ondisk.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Bcache on disk data structures
//

// Btree keys - all units are in sectors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bkey {
    pub high: __u64,
    pub low: __u64,
    pub ptr: [__u64; ],
}

pub const KEY_SIZE_BITS: c_int = 16;
pub const KEY_MAX_U64S: c_int = 8;
// Next time I change the on disk format, KEY_OFFSET() won't be 64 bits
//
// The high bit being set is a relic from when we used it to do binary
// searches - it told you where a key started. It's not used anymore,
// and can probably be safely dropped.
//

pub const PTR_DEV_BITS: c_int = 12;

// Bkey utility code
extern "C" {
    pub fn bkey_u64s(sizeof(__u64: *mut *mut k)) -> return;
}

// bkey is always padded */)
// Enough for a key with 6 pointers
pub const BKEY_PAD: c_int = 8;

// Superblock
// Version 0: Cache device
// Version 1: Backing device
// Version 2: Seed pointer into btree node checksum
// Version 3: Cache device with new UUID format
// Version 4: Backing device with data offset
//
pub const BCACHE_SB_VERSION_CDEV: c_int = 0;
pub const BCACHE_SB_VERSION_BDEV: c_int = 1;
pub const BCACHE_SB_VERSION_CDEV_WITH_UUID: c_int = 3;
pub const BCACHE_SB_VERSION_BDEV_WITH_OFFSET: c_int = 4;
pub const BCACHE_SB_VERSION_CDEV_WITH_FEATURES: c_int = 5;
pub const BCACHE_SB_VERSION_BDEV_WITH_FEATURES: c_int = 6;
pub const BCACHE_SB_MAX_VERSION: c_int = 6;
pub const SB_SECTOR: c_int = 8;

pub const SB_SIZE: c_int = 4096;
pub const SB_LABEL_SIZE: c_int = 32;

// SB_JOURNAL_BUCKETS must be divisible by BITS_PER_LONG
pub const MAX_CACHES_PER_SET: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_sb_disk {
    pub csum: __le64,
    pub /: *mut *mut __le64 offset; / sector where this sb was written,
    pub version: __le64,
    pub magic: [__u8; 16],
    pub uuid: [__u8; 16],
    pub set_uuid: [__u8; 16],
    pub set_magic: __le64,
}

// Cache devices
// Backing devices
//
// block_size from the cache device section is still used by
// backing devices, so don't add anything here until we fix
// things to not need it for backing devices anymore
//
// This is for in-memory bcache super block.
// NOTE: cache_sb is NOT exactly mapping to cache_sb_disk, the member
// size, ordering and even whole struct size may be different
// from cache_sb_disk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_sb {
    pub /: *mut *mut __u64 offset; / sector where this sb was written,
    pub version: __u64,
    pub magic: [__u8; 16],
    pub uuid: [__u8; 16],
    pub set_uuid: [__u8; 16],
    pub set_magic: __u64,
}

// Cache devices
// Backing devices
//
// block_size from the cache device section is still used by
// backing devices, so don't add anything here until we fix
// things to not need it for backing devices anymore
//

//
// Magic numbers
//
// The various other data structures have their own magic numbers, which are
// xored with the first part of the cache set's UUID
//
pub const JSET_MAGIC: c_uint = 0x245235c1a3625032ULL;
pub const PSET_MAGIC: c_uint = 0x6750e15f87337f91ULL;
pub const BSET_MAGIC: c_uint = 0x90135c78b99e07f5ULL;
//
// Journal
//
// On disk format for a journal entry:
// seq is monotonically increasing; every journal entry has its own unique
// sequence number.
//
// last_seq is the oldest journal entry that still has keys the btree hasn't
// flushed to disk yet.
//
// version is for on disk format changes.
//
pub const BCACHE_JSET_VERSION_UUIDv1: c_int = 1;

pub const BCACHE_JSET_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jset {
    pub csum: __u64,
    pub magic: __u64,
    pub seq: __u64,
    pub version: __u32,
    pub keys: __u32,
    pub last_seq: __u64,
    pub btree_level: __u16,
    pub pad: [__u16; 3],
    pub prio_bucket: [__u64; MAX_CACHES_PER_SET],
    pub start): DECLARE_FLEX_ARRAY(struct bkey,,
    pub d): DECLARE_FLEX_ARRAY(__u64,,
}

// Bucket prios/gens
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prio_set {
    pub csum: __u64,
    pub magic: __u64,
    pub seq: __u64,
    pub version: __u32,
    pub pad: __u32,
    pub next_bucket: __u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bucket_disk {
    pub prio: __u16,
    pub gen: __u8,
    pub data: [} __attribute((packed)); ],
}

// UUIDS - per backing device/flash only volume metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuid_entry {
    pub uuid: [__u8; 16],
    pub label: [__u8; 32],
    pub /: *mut *mut __u32 first_reg; / time overflow in y2106,
    pub last_reg: __u32,
    pub invalidated: __u32,
    pub flags: __u32,
// Size of flash only volumes
    pub sectors: __u64,
}

// Btree nodes
// Version 1: Seed pointer into btree node checksum
//
pub const BCACHE_BSET_CSUM: c_int = 1;
pub const BCACHE_BSET_VERSION: c_int = 1;
//
// Btree nodes
//
// On disk a btree node is a list/log of these; within each set the keys are
// sorted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bset {
    pub csum: __u64,
    pub magic: __u64,
    pub seq: __u64,
    pub version: __u32,
    pub keys: __u32,
    pub start): DECLARE_FLEX_ARRAY(struct bkey,,
    pub d): DECLARE_FLEX_ARRAY(__u64,,
}

// OBSOLETE
// UUIDS - per backing device/flash only volume metadata
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uuid_entry_v0 {
    pub uuid: [__u8; 16],
    pub label: [__u8; 32],
    pub first_reg: __u32,
    pub last_reg: __u32,
    pub invalidated: __u32,
    pub pad: __u32,
}
