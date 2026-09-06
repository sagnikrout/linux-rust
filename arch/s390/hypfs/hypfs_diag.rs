//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/hypfs/hypfs_diag.h
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
// Hypervisor filesystem for Linux on s390. Diag 204 and 224
// implementation.
//
// Copyright IBM Corp. 2006, 2008
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//

extern "C" {
    pub fn diag204_get_info_type() -> diag204_format;
}
extern "C" {
    pub fn diag204_store(buf: *mut c_void, pages: c_int) -> c_int;
}
extern "C" {
    pub fn __hypfs_diag_fs_init() -> c_int;
}
extern "C" {
    pub fn __hypfs_diag_fs_exit();
}
extern "C" {
    pub fn __hypfs_diag_fs_init() -> return;
}
