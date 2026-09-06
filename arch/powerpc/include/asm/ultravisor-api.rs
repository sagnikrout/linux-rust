//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ultravisor-api.h
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
// Ultravisor API.
//
// Copyright 2019, IBM Corporation.
//

// Return codes

// opcodes
pub const UV_WRITE_PATE: c_uint = 0xF104;
pub const UV_RETURN: c_uint = 0xF11C;
pub const UV_ESM: c_uint = 0xF110;
pub const UV_REGISTER_MEM_SLOT: c_uint = 0xF120;
pub const UV_UNREGISTER_MEM_SLOT: c_uint = 0xF124;
pub const UV_PAGE_IN: c_uint = 0xF128;
pub const UV_PAGE_OUT: c_uint = 0xF12C;
pub const UV_SHARE_PAGE: c_uint = 0xF130;
pub const UV_UNSHARE_PAGE: c_uint = 0xF134;
pub const UV_UNSHARE_ALL_PAGES: c_uint = 0xF140;
pub const UV_PAGE_INVAL: c_uint = 0xF138;
pub const UV_SVM_TERMINATE: c_uint = 0xF13C;
