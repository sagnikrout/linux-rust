//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/sgx/main.h
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
// Copyright(c) 2016-20 Intel Corporation.
//
pub const ENCL_HEAP_SIZE_DEFAULT: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl_segment {
    pub src: *mut c_void,
    pub offset: off_t,
    pub size: usize,
    pub prot: c_uint,
    pub flags: c_uint,
    pub measure: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encl {
    pub fd: c_int,
    pub bin: *mut c_void,
    pub bin_size: off_t,
    pub src: *mut c_void,
    pub src_size: usize,
    pub encl_size: usize,
    pub encl_base: off_t,
    pub nr_segments: c_uint,
    pub segment_tbl: *mut encl_segment,
    pub secs: sgx_secs,
    pub sigstruct: sgx_sigstruct,
}

extern "C" {
    pub fn encl_delete(ctx: *mut encl);
}
extern "C" {
    pub fn encl_load(path: *const c_char, encl: *mut encl, heap_size: c_ulong) -> bool;
}
extern "C" {
    pub fn encl_measure(encl: *mut encl) -> bool;
}
extern "C" {
    pub fn encl_build(encl: *mut encl) -> bool;
}
extern "C" {
    pub fn encl_get_entry(encl: *mut encl, symbol: *const c_char) -> u64;
}
