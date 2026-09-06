//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/imx8mq-power.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (C) 2018 Pengutronix, Lucas Stach <kernel@pengutronix.de>
//
pub const IMX8M_POWER_DOMAIN_MIPI: c_int = 0;
pub const IMX8M_POWER_DOMAIN_PCIE1: c_int = 1;
pub const IMX8M_POWER_DOMAIN_USB_OTG1: c_int = 2;
pub const IMX8M_POWER_DOMAIN_USB_OTG2: c_int = 3;
pub const IMX8M_POWER_DOMAIN_DDR1: c_int = 4;
pub const IMX8M_POWER_DOMAIN_GPU: c_int = 5;
pub const IMX8M_POWER_DOMAIN_VPU: c_int = 6;
pub const IMX8M_POWER_DOMAIN_DISP: c_int = 7;
pub const IMX8M_POWER_DOMAIN_MIPI_CSI1: c_int = 8;
pub const IMX8M_POWER_DOMAIN_MIPI_CSI2: c_int = 9;
pub const IMX8M_POWER_DOMAIN_PCIE2: c_int = 10;
pub const IMX8MQ_VPUBLK_PD_G1: c_int = 0;
pub const IMX8MQ_VPUBLK_PD_G2: c_int = 1;
