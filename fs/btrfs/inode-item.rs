//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/inode-item.h
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
// Return this if we need to call truncate_block for the last bit of the
// truncate.
//
pub const BTRFS_NEED_TRUNCATE_BLOCK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_truncate_control {
//
// IN: the inode we're operating on, this can be NULL if
// ->clear_extent_range is false.
//
    pub inode: *mut btrfs_inode,
// IN: the size we're truncating to.
    pub new_size: u64,
// OUT: the number of extents truncated.
    pub extents_found: u64,
// OUT: the last size we truncated this inode to.
    pub last_size: u64,
// OUT: the number of bytes to sub from this inode.
    pub sub_bytes: u64,
// IN: the ino we are truncating.
    pub ino: u64,
//
// IN: minimum key type to remove.  All key types with this type are
// removed only if their offset >= new_size.
//
    pub min_type: u32,
//
// IN: true if we don't want to do extent reference updates for any file
// extents we drop.
//
    pub skip_ref_updates: bool,
//
// IN: true if we need to clear the file extent range for the inode as
// we drop the file extent items.
//
    pub clear_extent_range: bool,
}

//
// btrfs_inode_item stores flags in a u64, btrfs_inode stores them in two
// separate u32s. These two functions convert between the two representations.
//
// flags = (u32)inode_item_flags;
// ro_flags = (u32)(inode_item_flags >> 32);
// Figure the key offset of an extended inode ref.
