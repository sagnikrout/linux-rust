//! Automatically rewritten from C Header to Rust Module
//! Source: fs/omfs/omfs.h
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

// In-memory structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omfs_sb_info {
    pub s_num_blocks: u64,
    pub s_bitmap_ino: u64,
    pub s_root_ino: u64,
    pub s_blocksize: u32,
    pub s_mirrors: u32,
    pub s_sys_blocksize: u32,
    pub s_clustersize: u32,
    pub s_block_shift: c_int,
    pub s_imap: *mut c_ulong,
    pub s_imap_size: c_int,
    pub s_bitmap_lock: mutex,
    pub s_uid: kuid_t,
    pub s_gid: kgid_t,
    pub s_dmask: c_int,
    pub s_fmask: c_int,
}

// convert a cluster number to a scaled block number
// bitmap.c
extern "C" {
    pub fn omfs_count_free(sb: *mut super_block) -> c_ulong;
}
extern "C" {
    pub fn omfs_allocate_block(sb: *mut super_block, block: u64) -> c_int;
}
extern "C" {
    pub fn omfs_clear_range(sb: *mut super_block, block: u64, count: c_int) -> c_int;
}
// dir.c
extern "C" {
    pub fn omfs_make_empty(inode: *mut inode, sb: *mut super_block) -> c_int;
}
// file.c
extern "C" {
    pub fn omfs_make_empty_table(bh: *mut buffer_head, offset: c_int);
}
extern "C" {
    pub fn omfs_shrink_inode(inode: *mut inode) -> c_int;
}
// inode.c
extern "C" {
    pub fn omfs_reserve_block(sb: *mut super_block, block: sector_t) -> c_int;
}
extern "C" {
    pub fn omfs_find_empty_block(sb: *mut super_block, mode: c_int, ino: *mut ino_t) -> c_int;
}
extern "C" {
    pub fn omfs_sync_inode(inode: *mut inode) -> c_int;
}
