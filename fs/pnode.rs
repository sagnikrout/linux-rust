//! Automatically rewritten from C Header to Rust Module
//! Source: fs/pnode.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/fs/pnode.h
//
// (C) Copyright IBM Corporation 2005.
//

pub const CL_EXPIRE: c_uint = 0x01;
pub const CL_SLAVE: c_uint = 0x02;
pub const CL_COPY_UNBINDABLE: c_uint = 0x04;
pub const CL_MAKE_SHARED: c_uint = 0x08;
pub const CL_PRIVATE: c_uint = 0x10;
pub const CL_COPY_MNT_NS_FILE: c_uint = 0x40;
//
// EXCL[namespace_sem]
//
extern "C" {
    pub fn change_mnt_propagation(: *mut mount, _arg: c_int);
}
extern "C" {
    pub fn bulk_make_private(: *mut list_head);
}
extern "C" {
    pub fn propagate_umount(: *mut list_head);
}
extern "C" {
    pub fn propagate_mount_busy(: *mut mount, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn propagate_mount_unlock(: *mut mount);
}
extern "C" {
    pub fn mnt_release_group_id(: *mut mount);
}
extern "C" {
    pub fn get_dominating_id(mnt: *mut mount, root: *const path) -> c_int;
}
extern "C" {
    pub fn mnt_get_count(mnt: *mut mount) -> c_int;
}
extern "C" {
    pub fn count_mounts(ns: *mut mnt_namespace, mnt: *mut mount) -> c_int;
}
