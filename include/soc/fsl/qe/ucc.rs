//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/qe/ucc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2006 Freescale Semiconductor, Inc. All rights reserved.
//
// Authors: 	Shlomi Gridish <gridish@freescale.com>
// Li Yang <leoli@freescale.com>
//
// Description:
// Internal header file for UCC unit routines.
//

// Macro flag: #define STATISTICS
pub const UCC_MAX_NUM: c_int = 8;
// Slow or fast type for UCCs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucc_speed_type {
    UCC_SPEED_TYPE_FAST = UCC_GUEMR_MODE_FAST_RX | UCC_GUEMR_MODE_FAST_TX,
    UCC_SPEED_TYPE_SLOW = UCC_GUEMR_MODE_SLOW_RX | UCC_GUEMR_MODE_SLOW_TX
}

// ucc_set_type
// Sets UCC to slow or fast mode.
//
// ucc_num - (In) number of UCC (0-7).
// speed   - (In) slow or fast mode for UCC.
//
extern "C" {
    pub fn ucc_set_type(ucc_num: c_uint, speed: ucc_speed_type) -> c_int;
}
extern "C" {
    pub fn ucc_set_qe_mux_mii_mng(ucc_num: c_uint) -> c_int;
}
extern "C" {
    pub fn ucc_mux_set_grant_tsa_bkpt(ucc_num: c_uint, set: c_int, mask: u32) -> c_int;
}
// QE MUX clock routing for UCC
//
extern "C" {
    pub fn ucc_mux_set_grant_tsa_bkpt(_arg: ucc_num, _arg: set, _arg: QE_CMXUCR_GRANT) -> return;
}
extern "C" {
    pub fn ucc_mux_set_grant_tsa_bkpt(_arg: ucc_num, _arg: set, _arg: QE_CMXUCR_TSA) -> return;
}
extern "C" {
    pub fn ucc_mux_set_grant_tsa_bkpt(_arg: ucc_num, _arg: set, _arg: QE_CMXUCR_BKPT) -> return;
}
