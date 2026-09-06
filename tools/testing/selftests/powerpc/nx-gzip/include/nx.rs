//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/nx-gzip/include/nx.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2020 IBM Corp.
//

pub const NX_FUNC_COMP_842: c_int = 1;
pub const NX_FUNC_COMP_GZIP: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_func_args {
    pub use_crc: bool,
    pub /: *mut *mut bool decompress; / true decompress; false compress,
    pub move_data: bool,
    pub /: *mut *mut int timeout; / seconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxbuf_t {
    pub len: c_int,
    pub buf: *mut c_char,
}

// @function should be EFT (aka 842), GZIP etc
extern "C" {
    pub fn nx_function_end(handle: *mut c_void) -> c_int;
}
