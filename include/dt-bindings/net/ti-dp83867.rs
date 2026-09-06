//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/net/ti-dp83867.h
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
// Device Tree constants for the Texas Instruments DP83867 PHY
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Copyright (C) 2015-2024 Texas Instruments Incorporated - https://www.ti.com
//
// PHY CTRL bits
pub const DP83867_PHYCR_FIFO_DEPTH_3_B_NIB: c_uint = 0x00;
pub const DP83867_PHYCR_FIFO_DEPTH_4_B_NIB: c_uint = 0x01;
pub const DP83867_PHYCR_FIFO_DEPTH_6_B_NIB: c_uint = 0x02;
pub const DP83867_PHYCR_FIFO_DEPTH_8_B_NIB: c_uint = 0x03;
// RGMIIDCTL internal delay for rx and tx
pub const DP83867_RGMIIDCTL_250_PS: c_uint = 0x0;
pub const DP83867_RGMIIDCTL_500_PS: c_uint = 0x1;
pub const DP83867_RGMIIDCTL_750_PS: c_uint = 0x2;
pub const DP83867_RGMIIDCTL_1_NS: c_uint = 0x3;
pub const DP83867_RGMIIDCTL_1_25_NS: c_uint = 0x4;
pub const DP83867_RGMIIDCTL_1_50_NS: c_uint = 0x5;
pub const DP83867_RGMIIDCTL_1_75_NS: c_uint = 0x6;
pub const DP83867_RGMIIDCTL_2_00_NS: c_uint = 0x7;
pub const DP83867_RGMIIDCTL_2_25_NS: c_uint = 0x8;
pub const DP83867_RGMIIDCTL_2_50_NS: c_uint = 0x9;
pub const DP83867_RGMIIDCTL_2_75_NS: c_uint = 0xa;
pub const DP83867_RGMIIDCTL_3_00_NS: c_uint = 0xb;
pub const DP83867_RGMIIDCTL_3_25_NS: c_uint = 0xc;
pub const DP83867_RGMIIDCTL_3_50_NS: c_uint = 0xd;
pub const DP83867_RGMIIDCTL_3_75_NS: c_uint = 0xe;
pub const DP83867_RGMIIDCTL_4_00_NS: c_uint = 0xf;
// IO_MUX_CFG - Clock output selection
pub const DP83867_CLK_O_SEL_CHN_A_RCLK: c_uint = 0x0;
pub const DP83867_CLK_O_SEL_CHN_B_RCLK: c_uint = 0x1;
pub const DP83867_CLK_O_SEL_CHN_C_RCLK: c_uint = 0x2;
pub const DP83867_CLK_O_SEL_CHN_D_RCLK: c_uint = 0x3;
pub const DP83867_CLK_O_SEL_CHN_A_RCLK_DIV5: c_uint = 0x4;
pub const DP83867_CLK_O_SEL_CHN_B_RCLK_DIV5: c_uint = 0x5;
pub const DP83867_CLK_O_SEL_CHN_C_RCLK_DIV5: c_uint = 0x6;
pub const DP83867_CLK_O_SEL_CHN_D_RCLK_DIV5: c_uint = 0x7;
pub const DP83867_CLK_O_SEL_CHN_A_TCLK: c_uint = 0x8;
pub const DP83867_CLK_O_SEL_CHN_B_TCLK: c_uint = 0x9;
pub const DP83867_CLK_O_SEL_CHN_C_TCLK: c_uint = 0xA;
pub const DP83867_CLK_O_SEL_CHN_D_TCLK: c_uint = 0xB;
pub const DP83867_CLK_O_SEL_REF_CLK: c_uint = 0xC;
// Special flag to indicate clock should be off
pub const DP83867_CLK_O_SEL_OFF: c_uint = 0xFFFFFFFF;
