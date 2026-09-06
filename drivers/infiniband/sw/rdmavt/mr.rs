//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rdmavt/mr.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mr {
    pub ibmr: ib_mr,
    pub umem: *mut ib_umem,
    pub /: *mut *mut rvt_mregion mr; / must be last,
}

extern "C" {
    pub fn container_of(_arg: ibmr, rvt_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn rvt_driver_mr_init(rdi: *mut rvt_dev_info) -> c_int;
}
extern "C" {
    pub fn rvt_mr_exit(rdi: *mut rvt_dev_info);
}
// Mem Regions
extern "C" {
    pub fn rvt_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
