//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi_port.h
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
//
// Driver for Solarflare network controllers and boards
// Copyright 2008-2013 Solarflare Communications Inc.
// Copyright 2019-2020 Xilinx Inc.
//

extern "C" {
    pub fn efx_mcdi_phy_get_caps(efx: *mut efx_nic) -> u32;
}
extern "C" {
    pub fn efx_mcdi_mac_check_fault(efx: *mut efx_nic) -> bool;
}
extern "C" {
    pub fn efx_mcdi_port_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_port_remove(efx: *mut efx_nic);
}
