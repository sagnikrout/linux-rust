//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/cirrus/ep93xx.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep93xx_soc_model {
    EP93XX_9301_SOC,
    EP93XX_9307_SOC,
    EP93XX_9312_SOC,
}

pub const EP93XX_CHIP_REV_D0: c_int = 3;
pub const EP93XX_CHIP_REV_D1: c_int = 4;
pub const EP93XX_CHIP_REV_E0: c_int = 5;
pub const EP93XX_CHIP_REV_E1: c_int = 6;
pub const EP93XX_CHIP_REV_E2: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_regmap_adev {
    pub adev: auxiliary_device,
    pub map: *mut regmap,
    pub base: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub val): c_uint,
    pub val): unsigned int reg, unsigned int mask, unsigned int,
}

