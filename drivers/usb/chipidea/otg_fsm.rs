//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/chipidea/otg_fsm.h
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
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Jun Li
//

//
// A-DEVICE timing  constants
//
// Wait for VBUS Rise

// a_wait_vrise_tmr: section 7.4.5.1
// TA_VBUS_RISE <= 100ms, section 4.4
// Table 4-1: Electrical Characteristics
// ->DC Electrical Timing
//
// Wait for VBUS Fall

// a_wait_vfall_tmr: section: 7.4.5.2
//
// Wait for B-Connect

// TA_WAIT_BCON: should be between 1100
// and 30000 ms, section 5.5, Table 5-1
//
// A-Idle to B-Disconnect

// TA_AIDL_BDIS: section 5.5, Table 5-1
//
// B-Idle to A-Disconnect

// 500ms is used for B switch to host
// for safe
//
// B-device timing constants
//
// Data-Line Pulse Time

// section:5.1.3
//
// SRP Fail Time

// section:5.1.6
//
// A-SE0 to B-Reset

// SE0 Time Before SRP

// SSEND time before SRP

extern "C" {
    pub fn ci_hdrc_otg_fsm_init(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn ci_otg_fsm_work(ci: *mut ci_hdrc) -> c_int;
}
extern "C" {
    pub fn ci_otg_fsm_irq(ci: *mut ci_hdrc) -> irqreturn_t;
}
extern "C" {
    pub fn ci_hdrc_otg_fsm_start(ci: *mut ci_hdrc);
}
extern "C" {
    pub fn ci_hdrc_otg_fsm_remove(ci: *mut ci_hdrc);
}

