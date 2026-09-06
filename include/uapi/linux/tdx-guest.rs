//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tdx-guest.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Userspace interface for TDX guest driver
//
// Copyright (C) 2022 Intel Corporation
//

// Length of the REPORTDATA used in TDG.MR.REPORT TDCALL
pub const TDX_REPORTDATA_LEN: c_int = 64;
// Length of TDREPORT used in TDG.MR.REPORT TDCALL
pub const TDX_REPORT_LEN: c_int = 1024;
//
// struct tdx_report_req - Request struct for TDX_CMD_GET_REPORT0 IOCTL.
//
// @reportdata: User buffer with REPORTDATA to be included into TDREPORT.
// Typically it can be some nonce provided by attestation
// service, so the generated TDREPORT can be uniquely verified.
// @tdreport: User buffer to store TDREPORT output from TDCALL[TDG.MR.REPORT].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdx_report_req {
    pub reportdata: [__u8; TDX_REPORTDATA_LEN],
    pub tdreport: [__u8; TDX_REPORT_LEN],
}

//
// TDX_CMD_GET_REPORT0 - Get TDREPORT0 (a.k.a. TDREPORT subtype 0) using
// TDCALL[TDG.MR.REPORT]
//
// Return 0 on success, -EIO on TDCALL execution failure, and
// standard errno on other general error cases.
//

