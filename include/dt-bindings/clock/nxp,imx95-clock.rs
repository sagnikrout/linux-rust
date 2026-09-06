//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/nxp,imx95-clock.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright 2024 NXP
//
pub const IMX95_CLK_VPUBLK_WAVE: c_int = 0;
pub const IMX95_CLK_VPUBLK_JPEG_ENC: c_int = 1;
pub const IMX95_CLK_VPUBLK_JPEG_DEC: c_int = 2;
pub const IMX95_CLK_CAMBLK_CSI2_FOR0: c_int = 0;
pub const IMX95_CLK_CAMBLK_CSI2_FOR1: c_int = 1;
pub const IMX95_CLK_CAMBLK_ISP_AXI: c_int = 2;
pub const IMX95_CLK_CAMBLK_ISP_PIXEL: c_int = 3;
pub const IMX95_CLK_CAMBLK_ISP: c_int = 4;
pub const IMX95_CLK_DISPMIX_LVDS_PHY_DIV: c_int = 0;
pub const IMX95_CLK_DISPMIX_LVDS_CH0_GATE: c_int = 1;
pub const IMX95_CLK_DISPMIX_LVDS_CH1_GATE: c_int = 2;
pub const IMX95_CLK_DISPMIX_PIX_DI0_GATE: c_int = 3;
pub const IMX95_CLK_DISPMIX_PIX_DI1_GATE: c_int = 4;
pub const IMX95_CLK_DISPMIX_ENG0_SEL: c_int = 0;
pub const IMX95_CLK_DISPMIX_ENG1_SEL: c_int = 1;
pub const IMX95_CLK_NETCMIX_ENETC0_RMII: c_int = 0;
pub const IMX95_CLK_NETCMIX_ENETC1_RMII: c_int = 1;
