//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_anti_rb.h
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
// Copyright(c) 2026 Intel Corporation

pub const ADF_SVN_NO_STS: c_uint = 0x00;
pub const ADF_SVN_PASS_STS: c_uint = 0x01;
pub const ADF_SVN_RETRY_STS: c_uint = 0x02;
pub const ADF_SVN_FAIL_STS: c_uint = 0x03;
pub const ADF_SVN_RETRY_MS: c_int = 250;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum anti_rb {
    ARB_ENFORCED_MIN_SVN,
    ARB_PERMANENT_MIN_SVN,
    ARB_ACTIVE_SVN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_anti_rb_hw_data {
    pub accel_dev): *mut *mut bool (anti_rb_enabled)(struct adf_accel_dev,
    pub svncheck_offset: u32,
    pub svncheck_retry: u32,
    pub sysfs_added: bool,
}

extern "C" {
    pub fn adf_anti_rb_commit(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_anti_rb_query(accel_dev: *mut adf_accel_dev, cmd: anti_rb, svn: *mut u8) -> c_int;
}
extern "C" {
    pub fn adf_anti_rb_check(pdev: *mut pci_dev) -> c_int;
}
