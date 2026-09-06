//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mscc/ocelot_police.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2019 Microsemi Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mscc_qos_rate_mode {
    MSCC_QOS_RATE_MODE_DISABLED, /* Policer/shaper disabled */
    MSCC_QOS_RATE_MODE_LINE, /* Measure line rate in kbps incl. IPG */
    MSCC_QOS_RATE_MODE_DATA, /* Measures data rate in kbps excl. IPG */
    MSCC_QOS_RATE_MODE_FRAME, /* Measures frame rate in fps */
    __MSCC_QOS_RATE_MODE_END,
    NUM_MSCC_QOS_RATE_MODE = __MSCC_QOS_RATE_MODE_END,
    MSCC_QOS_RATE_MODE_MAX = __MSCC_QOS_RATE_MODE_END - 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qos_policer_conf {
    pub mode: mscc_qos_rate_mode,
    pub /: *mut *mut bool dlb; / Enable DLB (dual leaky bucket mode,
    pub /: *mut *mut bool cf; / Coupling flag (ignored in SLB mode),
    pub /: *mut *mut u32 cir; / CIR in kbps/fps (ignored in SLB mode),
    pub /: *mut *mut u32 cbs; / CBS in bytes/frames (ignored in SLB mode),
    pub /: *mut *mut u32 pir; / PIR in kbps/fps,
    pub /: *mut *mut u32 pbs; / PBS in bytes/frames,
    pub /: *mut *mut u8 ipg; / Size of IPG when MSCC_QOS_RATE_MODE_LINE is chosen,
}
