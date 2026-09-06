//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bcm6318-clock.h
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
pub const BCM6318_CLK_ADSL_ASB: c_int = 0;
pub const BCM6318_CLK_USB_ASB: c_int = 1;
pub const BCM6318_CLK_MIPS_ASB: c_int = 2;
pub const BCM6318_CLK_PCIE_ASB: c_int = 3;
pub const BCM6318_CLK_PHYMIPS_ASB: c_int = 4;
pub const BCM6318_CLK_ROBOSW_ASB: c_int = 5;
pub const BCM6318_CLK_SAR_ASB: c_int = 6;
pub const BCM6318_CLK_SDR_ASB: c_int = 7;
pub const BCM6318_CLK_SWREG_ASB: c_int = 8;
pub const BCM6318_CLK_PERIPH_ASB: c_int = 9;
pub const BCM6318_CLK_CPUBUS160: c_int = 10;
pub const BCM6318_CLK_ADSL: c_int = 11;
pub const BCM6318_CLK_SAR125: c_int = 12;
pub const BCM6318_CLK_MIPS: c_int = 13;
pub const BCM6318_CLK_PCIE: c_int = 14;
pub const BCM6318_CLK_ROBOSW250: c_int = 16;
pub const BCM6318_CLK_ROBOSW025: c_int = 17;
pub const BCM6318_CLK_SDR: c_int = 19;
pub const BCM6318_CLK_USBD: c_int = 20;
pub const BCM6318_CLK_HSSPI: c_int = 25;
pub const BCM6318_CLK_PCIE25: c_int = 27;
pub const BCM6318_CLK_PHYMIPS: c_int = 28;
pub const BCM6318_CLK_AFE: c_int = 29;
pub const BCM6318_CLK_QPROC: c_int = 30;
pub const BCM6318_UCLK_ADSL: c_int = 0;
pub const BCM6318_UCLK_ARB: c_int = 1;
pub const BCM6318_UCLK_MIPS: c_int = 2;
pub const BCM6318_UCLK_PCIE: c_int = 3;
pub const BCM6318_UCLK_PERIPH: c_int = 4;
pub const BCM6318_UCLK_PHYMIPS: c_int = 5;
pub const BCM6318_UCLK_ROBOSW: c_int = 6;
pub const BCM6318_UCLK_SAR: c_int = 7;
pub const BCM6318_UCLK_SDR: c_int = 8;
pub const BCM6318_UCLK_USB: c_int = 9;
