//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/bcm2835-pm.h
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


// SPDX-License-Identifier: GPL-2.0+

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm2835_soc {
    BCM2835_PM_SOC_BCM2835,
    BCM2835_PM_SOC_BCM2711,
    BCM2835_PM_SOC_BCM2712,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_pm {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub asb: *mut void __iomem,
    pub rpivid_asb: *mut void __iomem,
    pub soc: bcm2835_soc,
}
