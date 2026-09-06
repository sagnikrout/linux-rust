//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_arm_c9.h
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
// DMU (Device Management Unit)
pub const BCMA_DMU_CRU_USB2_CONTROL: c_uint = 0x0164;
pub const BCMA_DMU_CRU_USB2_CONTROL_USB_PLL_NDIV_MASK: c_uint = 0x00000FFC;
pub const BCMA_DMU_CRU_USB2_CONTROL_USB_PLL_NDIV_SHIFT: c_int = 2;
pub const BCMA_DMU_CRU_USB2_CONTROL_USB_PLL_PDIV_MASK: c_uint = 0x00007000;
pub const BCMA_DMU_CRU_USB2_CONTROL_USB_PLL_PDIV_SHIFT: c_int = 12;
pub const BCMA_DMU_CRU_CLKSET_KEY: c_uint = 0x0180;
pub const BCMA_DMU_CRU_STRAPS_CTRL: c_uint = 0x02A0;
pub const BCMA_DMU_CRU_STRAPS_CTRL_USB3: c_uint = 0x00000010;
pub const BCMA_DMU_CRU_STRAPS_CTRL_4BYTE: c_uint = 0x00008000;
