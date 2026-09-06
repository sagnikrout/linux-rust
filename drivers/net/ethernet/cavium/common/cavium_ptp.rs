//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/common/cavium_ptp.h
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
// cavium_ptp.h - PTP 1588 clock on Cavium hardware
// Copyright (c) 2003-2015, 2017 Cavium, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cavium_ptp {
    pub pdev: *mut pci_dev,
// Serialize access to cycle_counter, time_counter and hw_registers
    pub spin_lock: spinlock_t,
    pub cycle_counter: cyclecounter,
    pub time_counter: timecounter,
    pub reg_base: *mut void __iomem,
    pub clock_rate: u32,
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
}

extern "C" {
    pub fn cavium_ptp_put(ptp: *mut cavium_ptp);
}
extern "C" {
    pub fn ptp_clock_index(_arg: clock->ptp_clock) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

