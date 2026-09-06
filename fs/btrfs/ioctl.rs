//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/ioctl.h
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
    pub fn btrfs_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn btrfs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn btrfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn btrfs_ioctl_get_supported_features(arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn btrfs_sync_inode_flags_to_i_flags(inode: *mut btrfs_inode);
}
extern "C" {
    pub fn btrfs_uring_cmd(cmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn btrfs_uring_read_extent_endio(ctx: *mut c_void, err: c_int);
}
