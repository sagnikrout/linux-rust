//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/debug.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

// message levels
pub const BRCMF_TRACE_VAL: c_uint = 0x00000002;
pub const BRCMF_INFO_VAL: c_uint = 0x00000004;
pub const BRCMF_DATA_VAL: c_uint = 0x00000008;
pub const BRCMF_CTL_VAL: c_uint = 0x00000010;
pub const BRCMF_TIMER_VAL: c_uint = 0x00000020;
pub const BRCMF_HDRS_VAL: c_uint = 0x00000040;
pub const BRCMF_BYTES_VAL: c_uint = 0x00000080;
pub const BRCMF_INTR_VAL: c_uint = 0x00000100;
pub const BRCMF_GLOM_VAL: c_uint = 0x00000200;
pub const BRCMF_EVENT_VAL: c_uint = 0x00000400;
pub const BRCMF_BTA_VAL: c_uint = 0x00000800;
pub const BRCMF_FIL_VAL: c_uint = 0x00001000;
pub const BRCMF_USB_VAL: c_uint = 0x00002000;
pub const BRCMF_SCAN_VAL: c_uint = 0x00004000;
pub const BRCMF_CONN_VAL: c_uint = 0x00008000;
pub const BRCMF_BCDC_VAL: c_uint = 0x00010000;
pub const BRCMF_SDIO_VAL: c_uint = 0x00020000;
pub const BRCMF_MSGBUF_VAL: c_uint = 0x00040000;
pub const BRCMF_PCIE_VAL: c_uint = 0x00080000;
pub const BRCMF_FWCON_VAL: c_uint = 0x00100000;
// set default print format

extern "C" {
    pub fn __brcmf_err(bus: *mut brcmf_bus, func: *const c_char, fmt: *const c_char, ...);
}
// Macro for error messages. When debugging / tracing the driver all error
// messages are important to us.
//

// For debug/tracing purposes treat info messages as errors

extern "C" {
    pub fn __brcmf_dbg(level: u32, func: *const c_char, fmt: *const c_char, ...);
}

pub const BRCMF_DATA_ON(): c_int = 0;
pub const BRCMF_CTL_ON(): c_int = 0;
pub const BRCMF_HDRS_ON(): c_int = 0;
pub const BRCMF_BYTES_ON(): c_int = 0;
pub const BRCMF_GLOM_ON(): c_int = 0;
pub const BRCMF_EVENT_ON(): c_int = 0;
pub const BRCMF_FIL_ON(): c_int = 0;
pub const BRCMF_FWCON_ON(): c_int = 0;
pub const BRCMF_SCAN_ON(): c_int = 0;

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

