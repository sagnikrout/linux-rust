//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/relocation.h
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

extern "C" {
    pub fn btrfs_init_reloc_root(trans: *mut btrfs_trans_handle, root: *mut btrfs_root) -> c_int;
}
extern "C" {
    pub fn btrfs_recover_relocation(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_reloc_clone_csums(ordered: *mut btrfs_ordered_extent) -> c_int;
}
extern "C" {
    pub fn btrfs_should_cancel_balance(fs_info: *const btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_should_ignore_reloc_root(root: *const btrfs_root) -> bool;
}
extern "C" {
    pub fn btrfs_get_reloc_bg_bytenr(fs_info: *mut btrfs_fs_info) -> u64;
}
extern "C" {
    pub fn btrfs_translate_remap(fs_info: *mut btrfs_fs_info, logical: *mut u64, length: *mut u64) -> c_int;
}
