//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/bcm/util.h
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
// Copyright 2016 Broadcom
//

extern "C" {
    pub fn __dump_sg(sg: *mut scatterlist, skip: c_uint, len: c_uint);
}

// Copy sg data, from skip, length len, to dest
// Copy src into scatterlist from offset, length len
extern "C" {
    pub fn spu_sg_count(sg_list: *mut scatterlist, skip: c_uint, nbytes: c_int) -> c_int;
}
extern "C" {
    pub fn add_to_ctr(ctr_pos: *mut u8, increment: c_uint);
}
// produce a message digest from data of length n bytes
extern "C" {
    pub fn spu_setup_debugfs();
}
extern "C" {
    pub fn spu_free_debugfs();
}
extern "C" {
    pub fn format_value_ccm(val: c_uint, buf: *mut u8, len: u8);
}
