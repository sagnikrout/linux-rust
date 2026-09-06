//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_fips.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_fips_status {
    CC_FIPS_SYNC_MODULE_OK = 0x0,
    CC_FIPS_SYNC_MODULE_ERROR = 0x1,
    CC_FIPS_SYNC_REE_STATUS = 0x4,
    CC_FIPS_SYNC_TEE_STATUS = 0x8,
    CC_FIPS_SYNC_STATUS_RESERVE32B = S32_MAX
}

extern "C" {
    pub fn cc_fips_init(p_drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn cc_fips_fini(drvdata: *mut cc_drvdata);
}
extern "C" {
    pub fn fips_handler(drvdata: *mut cc_drvdata);
}
extern "C" {
    pub fn cc_set_ree_fips_status(drvdata: *mut cc_drvdata, ok: bool);
}
extern "C" {
    pub fn cc_tee_handle_fips_error(p_drvdata: *mut cc_drvdata);
}

