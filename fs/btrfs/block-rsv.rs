//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/block-rsv.h
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
// Types of block reserves
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_rsv_type {
    BTRFS_BLOCK_RSV_GLOBAL,
    BTRFS_BLOCK_RSV_DELALLOC,
    BTRFS_BLOCK_RSV_TRANS,
    BTRFS_BLOCK_RSV_CHUNK,
    BTRFS_BLOCK_RSV_REMAP,
    BTRFS_BLOCK_RSV_DELOPS,
    BTRFS_BLOCK_RSV_DELREFS,
    BTRFS_BLOCK_RSV_TREELOG,
    BTRFS_BLOCK_RSV_EMPTY,
    BTRFS_BLOCK_RSV_TEMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_block_rsv {
    pub size: u64,
    pub reserved: u64,
    pub space_info: *mut btrfs_space_info,
    pub lock: spinlock_t,
    pub full: bool,
    pub failfast: bool,
// Block reserve type, one of BTRFS_BLOCK_RSV_*
    pub type:8: btrfs_rsv_type,
//
// Qgroup equivalent for @size @reserved
//
// Unlike normal @size/@reserved for inode rsv, qgroup doesn't care
// about things like csum size nor how many tree blocks it will need to
// reserve.
//
// Qgroup cares more about net change of the extent usage.
//
// So for one newly inserted file extent, in worst case it will cause
// leaf split and level increase, nodesize for each file extent is
// already too much.
//
// In short, qgroup_size/reserved is the upper limit of possible needed
// qgroup metadata reservation.
//
    pub qgroup_rsv_size: u64,
    pub qgroup_rsv_reserved: u64,
}

extern "C" {
    pub fn btrfs_init_block_rsv(rsv: *mut btrfs_block_rsv, type: btrfs_rsv_type);
}
extern "C" {
    pub fn btrfs_init_root_block_rsv(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_block_rsv_check(block_rsv: *mut btrfs_block_rsv, min_percent: c_int) -> c_int;
}
extern "C" {
    pub fn btrfs_block_rsv_use_bytes(block_rsv: *mut btrfs_block_rsv, num_bytes: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_update_global_block_rsv(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_init_global_block_rsv(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_release_global_block_rsv(fs_info: *mut btrfs_fs_info);
}
//
// Fast path to check if the reserve is full, may be carefully used outside of
// locks.
//
extern "C" {
    pub fn data_race(_arg: rsv->full) -> return;
}
//
// Get the reserved mount of a block reserve in a context where getting a stale
// value is acceptable, instead of accessing it directly and trigger data race
// warning from KCSAN.
//
// Get the size of a block reserve in a context where getting a stale value is
// acceptable, instead of accessing it directly and trigger data race warning
// from KCSAN.
//
