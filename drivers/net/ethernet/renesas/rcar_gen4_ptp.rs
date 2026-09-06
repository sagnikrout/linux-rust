//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/renesas/rcar_gen4_ptp.h
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
// Renesas R-Car Gen4 gPTP device driver
//
// Copyright (C) 2022 Renesas Electronics Corporation
//
extern "C" {
    pub fn rcar_gen4_ptp_register(ptp_priv: *mut rcar_gen4_ptp_private, rate: u32) -> c_int;
}
extern "C" {
    pub fn rcar_gen4_ptp_unregister(ptp_priv: *mut rcar_gen4_ptp_private) -> c_int;
}
extern "C" {
    pub fn rcar_gen4_ptp_clock_index(priv: *mut rcar_gen4_ptp_private) -> c_int;
}
