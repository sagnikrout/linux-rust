//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/ptp.h
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
// Marvell PTP driver
//
// Copyright (C) 2020 Marvell.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp {
    pub pdev: *mut pci_dev,
    pub reg_base: *mut void __iomem,
    pub ptp): *mut *mut u64 (read_ptp_tstmp)(struct ptp,
    pub /: *mut *mut spinlock_t ptp_lock; / lock,
    pub hrtimer: hrtimer,
    pub last_ts: ktime_t,
    pub clock_rate: u32,
    pub clock_period: u32,
}

extern "C" {
    pub fn ptp_put(ptp: *mut ptp);
}
extern "C" {
    pub fn ptp_start(rvu: *mut rvu, sclk: u64, ext_clk_freq: u32, extts: u32);
}
