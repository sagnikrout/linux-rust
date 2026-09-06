//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_txclk.h
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
// Copyright (C) 2026 Intel Corporation
//
// ice_txclk_any_port_uses - check if any port on a PHY uses this TX refclk
// @ctrl_pf: control PF (owner of the shared tx_refclks map)
// @phy: PHY index
// @clk: TX reference clock
//
// Return: true if any bit (port) is set for this clock on this PHY
//
extern "C" {
    pub fn ice_txclk_set_clk(pf: *mut ice_pf, clk: ice_e825c_ref_clk) -> c_int;
}
extern "C" {
    pub fn ice_txclk_update_and_notify(pf: *mut ice_pf);
}
