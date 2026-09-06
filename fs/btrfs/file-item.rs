//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/file-item.h
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
// Return the number of bytes used by the item on disk, minus the size of any
// extent headers.  If a file is compressed on disk, this is the compressed
// size.
//
extern "C" {
    pub fn btrfs_lookup_bio_sums(bbio: *mut btrfs_bio) -> c_int;
}
extern "C" {
    pub fn btrfs_csum_one_bio(bbio: *mut btrfs_bio, async: bool) -> c_int;
}
extern "C" {
    pub fn btrfs_alloc_dummy_sum(bbio: *mut btrfs_bio) -> c_int;
}
extern "C" {
    pub fn btrfs_inode_set_file_extent_range(inode: *mut btrfs_inode, start: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_inode_safe_disk_i_size_write(inode: *mut btrfs_inode, new_i_size: u64);
}
extern "C" {
    pub fn btrfs_file_extent_end(path: *const btrfs_path) -> u64;
}
