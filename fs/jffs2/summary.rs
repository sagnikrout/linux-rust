//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/summary.h
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


//
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2004  Ferenc Havasi <havasi@inf.u-szeged.hu>,
// Zoltan Sogor <weth@inf.u-szeged.hu>,
// Patrik Kluba <pajko@halom.u-szeged.hu>,
// University of Szeged, Hungary
//
// For licensing information, see the file 'LICENCE' in this directory.
//
// Limit summary size to 64KiB so that we can kmalloc it. If the summary
pub const MAX_SUMMARY_SIZE: c_int = 65536;

pub const BLK_STATE_ALLFF: c_int = 0;
pub const BLK_STATE_CLEAN: c_int = 1;
pub const BLK_STATE_PARTDIRTY: c_int = 2;
pub const BLK_STATE_CLEANMARKER: c_int = 3;
pub const BLK_STATE_ALLDIRTY: c_int = 4;
pub const BLK_STATE_BADBLOCK: c_int = 5;
pub const JFFS2_SUMMARY_NOSUM_SIZE: c_uint = 0xffffffff;

// Summary structures used on flash
// Summary structures used in the memory
// Summary related information stored in superblock
// Summary marker is stored at the end of every sumarized erase block

extern "C" {
    pub fn jffs2_sum_init(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_exit(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_sum_disable_collecting(s: *mut jffs2_summary);
}
extern "C" {
    pub fn jffs2_sum_is_disabled(s: *mut jffs2_summary) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_reset_collected(s: *mut jffs2_summary);
}
extern "C" {
    pub fn jffs2_sum_move_collected(c: *mut jffs2_sb_info, s: *mut jffs2_summary);
}
extern "C" {
    pub fn jffs2_sum_write_sumnode(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_add_padding_mem(s: *mut jffs2_summary, size: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_add_inode_mem(s: *mut jffs2_summary, ri: *mut jffs2_raw_inode, ofs: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_add_dirent_mem(s: *mut jffs2_summary, rd: *mut jffs2_raw_dirent, ofs: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_add_xattr_mem(s: *mut jffs2_summary, rx: *mut jffs2_raw_xattr, ofs: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_sum_add_xref_mem(s: *mut jffs2_summary, rr: *mut jffs2_raw_xref, ofs: u32) -> c_int;
}

// Macro flag: #define jffs2_sum_disable_collecting(a)

