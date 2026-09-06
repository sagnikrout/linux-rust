//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fiemap.h
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
pub const _LINUX_FIEMAP_H: c_int = 1;

//
// struct fiemap_extent_info - fiemap request to a filesystem
// @fi_flags:		Flags as passed from user
// @fi_extents_mapped:	Number of mapped extents
// @fi_extents_max:	Size of fiemap_extent array
// @fi_extents_start:	Start of fiemap_extent array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fiemap_extent_info {
    pub fi_flags: c_uint,
    pub fi_extents_mapped: c_uint,
    pub fi_extents_max: c_uint,
    pub fi_extents_start: *mut fiemap_extent __user,
}
