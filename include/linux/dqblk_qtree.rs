//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dqblk_qtree.h
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
// Definitions of structures and functions for quota formats using trie
//

// Numbers of blocks needed for updates - we count with the smallest
// possible block size (1024)
pub const QTREE_INIT_ALLOC: c_int = 4;
pub const QTREE_INIT_REWRITE: c_int = 2;
pub const QTREE_DEL_ALLOC: c_int = 0;
pub const QTREE_DEL_REWRITE: c_int = 6;
// Operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtree_fmt_operations {
    pub /: *mut *mut *mut *mut *mut void (mem2disk_dqblk)(void disk, struct dquot dquot); / Convert given entry from in memory format to disk one,
    pub /: *mut *mut *mut *mut *mut void (disk2mem_dqblk)(struct dquot dquot, void disk); / Convert given entry from disk format to in memory one,
    pub /: *mut *mut *mut *mut *mut int (is_id)(void disk, struct dquot dquot); / Is this structure for given id?,
}

// Inmemory copy of version specific information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtree_mem_dqinfo {
    pub /: *mut *mut *mut super_block dqi_sb; / Sb quota is on,
    pub /: *mut *mut int dqi_type; / Quota type,
    pub /: *mut *mut unsigned int dqi_blocks; / # of blocks in quota file,
    pub /: *mut *mut unsigned int dqi_free_blk; / First block in list of free blocks,
    pub /: *mut *mut unsigned int dqi_free_entry; / First block with free entry,
    pub /: *mut *mut unsigned int dqi_blocksize_bits; / Block size of quota file,
    pub /: *mut *mut unsigned int dqi_entry_size; / Size of quota entry in quota file,
    pub /: *mut *mut unsigned int dqi_usable_bs; / Space usable in block for quota data,
    pub /: *mut *mut unsigned int dqi_qtree_depth; / Precomputed depth of quota tree,
    pub /: *const *const *const qtree_fmt_operations dqi_ops; / Operations for entry manipulation,
}

extern "C" {
    pub fn qtree_write_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn qtree_read_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn qtree_delete_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn qtree_release_dquot(info: *mut qtree_mem_dqinfo, dquot: *mut dquot) -> c_int;
}
extern "C" {
    pub fn qtree_entry_unused(info: *mut qtree_mem_dqinfo, disk: *mut c_char) -> c_int;
}
extern "C" {
    pub fn qtree_get_next_id(info: *mut qtree_mem_dqinfo, qid: *mut kqid) -> c_int;
}
