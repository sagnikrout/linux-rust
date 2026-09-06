//! Automatically rewritten from C Header to Rust Module
//! Source: include/misc/ocxl-config.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.
//
// This file lists the various constants used to read the
// configuration space of an opencapi adapter.
//
// It follows the specification for opencapi 3.0
//
pub const OCXL_EXT_CAP_ID_DVSEC: c_uint = 0x23;
pub const OCXL_DVSEC_VENDOR_OFFSET: c_uint = 0x4;
pub const OCXL_DVSEC_ID_OFFSET: c_uint = 0x8;
pub const OCXL_DVSEC_TL_ID: c_uint = 0xF000;
pub const OCXL_DVSEC_TL_BACKOFF_TIMERS: c_uint = 0x10;
pub const OCXL_DVSEC_TL_RECV_CAP: c_uint = 0x18;
pub const OCXL_DVSEC_TL_SEND_CAP: c_uint = 0x20;
pub const OCXL_DVSEC_TL_RECV_RATE: c_uint = 0x30;
pub const OCXL_DVSEC_TL_SEND_RATE: c_uint = 0x50;
pub const OCXL_DVSEC_FUNC_ID: c_uint = 0xF001;
pub const OCXL_DVSEC_FUNC_OFF_INDEX: c_uint = 0x08;
pub const OCXL_DVSEC_FUNC_OFF_ACTAG: c_uint = 0x0C;
pub const OCXL_DVSEC_AFU_INFO_ID: c_uint = 0xF003;
pub const OCXL_DVSEC_AFU_INFO_AFU_IDX: c_uint = 0x0A;
pub const OCXL_DVSEC_AFU_INFO_OFF: c_uint = 0x0C;
pub const OCXL_DVSEC_AFU_INFO_DATA: c_uint = 0x10;
pub const OCXL_DVSEC_AFU_CTRL_ID: c_uint = 0xF004;
pub const OCXL_DVSEC_AFU_CTRL_AFU_IDX: c_uint = 0x0A;
pub const OCXL_DVSEC_AFU_CTRL_TERM_PASID: c_uint = 0x0C;
pub const OCXL_DVSEC_AFU_CTRL_ENABLE: c_uint = 0x0F;
pub const OCXL_DVSEC_AFU_CTRL_PASID_SUP: c_uint = 0x10;
pub const OCXL_DVSEC_AFU_CTRL_PASID_EN: c_uint = 0x11;
pub const OCXL_DVSEC_AFU_CTRL_PASID_BASE: c_uint = 0x14;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_SUP: c_uint = 0x18;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_EN: c_uint = 0x1A;
pub const OCXL_DVSEC_AFU_CTRL_ACTAG_BASE: c_uint = 0x1C;
pub const OCXL_DVSEC_VENDOR_ID: c_uint = 0xF0F0;
pub const OCXL_DVSEC_VENDOR_CFG_VERS: c_uint = 0x0C;
pub const OCXL_DVSEC_VENDOR_TLX_VERS: c_uint = 0x10;
pub const OCXL_DVSEC_VENDOR_DLX_VERS: c_uint = 0x20;
pub const OCXL_DVSEC_VENDOR_RESET_RELOAD: c_uint = 0x38;
