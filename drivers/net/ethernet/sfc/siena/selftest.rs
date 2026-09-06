//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/selftest.h
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
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2012 Solarflare Communications Inc.
//

//
// Self tests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_loopback_self_tests {
    pub tx_sent: [c_int; EFX_MAX_TXQ_PER_CHANNEL],
    pub tx_done: [c_int; EFX_MAX_TXQ_PER_CHANNEL],
    pub rx_good: c_int,
    pub rx_bad: c_int,
}

pub const EFX_MAX_PHY_TESTS: c_int = 20;
// Efx self test results
// For fields which are not counters, 1 indicates success and -1
// indicates failure; 0 indicates test could not be run.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_self_tests {
// online tests
    pub phy_alive: c_int,
    pub nvram: c_int,
    pub interrupt: c_int,
    pub eventq_dma: [c_int; EFX_MAX_CHANNELS],
    pub eventq_int: [c_int; EFX_MAX_CHANNELS],
// offline tests
    pub memory: c_int,
    pub registers: c_int,
    pub phy_ext: [c_int; EFX_MAX_PHY_TESTS],
    pub 1]: efx_loopback_self_tests loopback[LOOPBACK_TEST_MAX +,
}

extern "C" {
    pub fn efx_siena_selftest_async_init(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_selftest_async_start(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_selftest_async_cancel(efx: *mut efx_nic);
}
