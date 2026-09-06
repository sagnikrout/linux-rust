//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/extent_map.h
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

// bits for the extent_map::flags field
// this entry not yet on disk, don't free it
// pre-allocated extent
// Logging this extent
// This em is merged from two or more physically adjacent ems
//
// This structure represents file extents and holes.
//
// Unlike on-disk file extent items, extent maps can be merged to save memory.
// This means members only match file extent items before any merging.
//
// Keep this structure as compact as possible, as we can have really large
// amounts of allocated extent maps at any time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_map {
    pub rb_node: rb_node,
// All of these are in bytes.
// File offset matching the offset of a BTRFS_EXTENT_ITEM_KEY key.
    pub start: u64,
//
// Length of the file extent.
//
// For non-inlined file extents it's btrfs_file_extent_item::num_bytes.
// For inline extents it's sectorsize, since inline data starts at
// offsetof(struct btrfs_file_extent_item, disk_bytenr) thus
// btrfs_file_extent_item::num_bytes is not valid.
//
    pub len: u64,
//
// The bytenr of the full on-disk extent.
//
// For regular extents it's btrfs_file_extent_item::disk_bytenr.
// For holes it's EXTENT_MAP_HOLE and for inline extents it's
// EXTENT_MAP_INLINE.
//
    pub disk_bytenr: u64,
//
// The full on-disk extent length, matching
// btrfs_file_extent_item::disk_num_bytes.
//
    pub disk_num_bytes: u64,
//
// Offset inside the decompressed extent.
//
// For regular extents it's btrfs_file_extent_item::offset.
// For holes and inline extents it's 0.
//
    pub offset: u64,
//
// The decompressed size of the whole on-disk extent, matching
// btrfs_file_extent_item::ram_bytes.
//
    pub ram_bytes: u64,
//
// Generation of the extent map, for merged em it's the highest
// generation of all merged ems.
// For non-merged extents, it's from btrfs_file_extent_item::generation.
//
    pub generation: u64,
    pub flags: u32,
    pub refs: refcount_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_map_tree {
    pub root: rb_root,
    pub modified_extents: list_head,
    pub lock: rwlock_t,
}

//
// More efficient way to determine if extent is compressed, instead of using
// 'extent_map_compression() != BTRFS_COMPRESS_NONE'.
//
extern "C" {
    pub fn btrfs_extent_map_tree_init(tree: *mut extent_map_tree);
}
extern "C" {
    pub fn btrfs_remove_extent_mapping(inode: *mut btrfs_inode, em: *mut extent_map);
}
extern "C" {
    pub fn btrfs_free_extent_map(em: *mut extent_map);
}
extern "C" {
    pub fn btrfs_extent_map_init() -> int __init;
}
extern "C" {
    pub fn btrfs_extent_map_exit() -> void __cold;
}
extern "C" {
    pub fn btrfs_unpin_extent_cache(inode: *mut btrfs_inode, start: u64, len: u64, gen: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_clear_em_logging(inode: *mut btrfs_inode, em: *mut extent_map);
}
extern "C" {
    pub fn btrfs_free_extent_maps(fs_info: *mut btrfs_fs_info, nr_to_scan: c_long);
}
extern "C" {
    pub fn btrfs_init_extent_map_shrinker_work(fs_info: *mut btrfs_fs_info);
}
