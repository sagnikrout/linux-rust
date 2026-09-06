//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/tree-mod-log.h
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

// Represents a tree mod log user.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_seq_list {
    pub list: list_head,
    pub seq: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_mod_log_op {
    BTRFS_MOD_LOG_KEY_REPLACE,
    BTRFS_MOD_LOG_KEY_ADD,
    BTRFS_MOD_LOG_KEY_REMOVE,
    BTRFS_MOD_LOG_KEY_REMOVE_WHILE_FREEING,
    BTRFS_MOD_LOG_KEY_REMOVE_WHILE_MOVING,
    BTRFS_MOD_LOG_MOVE_KEYS,
    BTRFS_MOD_LOG_ROOT_REPLACE,
}

extern "C" {
    pub fn btrfs_tree_mod_log_free_eb(eb: *mut extent_buffer) -> c_int;
}
extern "C" {
    pub fn btrfs_old_root_level(root: *mut btrfs_root, time_seq: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_tree_mod_log_lowest_seq(fs_info: *mut btrfs_fs_info) -> u64;
}
