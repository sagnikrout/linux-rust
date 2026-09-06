//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,x1e80100-dispcc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
//
// DISP_CC clocks
pub const DISP_CC_MDSS_ACCU_CLK: c_int = 0;
pub const DISP_CC_MDSS_AHB1_CLK: c_int = 1;
pub const DISP_CC_MDSS_AHB_CLK: c_int = 2;
pub const DISP_CC_MDSS_AHB_CLK_SRC: c_int = 3;
pub const DISP_CC_MDSS_BYTE0_CLK: c_int = 4;
pub const DISP_CC_MDSS_BYTE0_CLK_SRC: c_int = 5;
pub const DISP_CC_MDSS_BYTE0_DIV_CLK_SRC: c_int = 6;
pub const DISP_CC_MDSS_BYTE0_INTF_CLK: c_int = 7;
pub const DISP_CC_MDSS_BYTE1_CLK: c_int = 8;
pub const DISP_CC_MDSS_BYTE1_CLK_SRC: c_int = 9;
pub const DISP_CC_MDSS_BYTE1_DIV_CLK_SRC: c_int = 10;
pub const DISP_CC_MDSS_BYTE1_INTF_CLK: c_int = 11;
pub const DISP_CC_MDSS_DPTX0_AUX_CLK: c_int = 12;
pub const DISP_CC_MDSS_DPTX0_AUX_CLK_SRC: c_int = 13;
pub const DISP_CC_MDSS_DPTX0_LINK_CLK: c_int = 14;
pub const DISP_CC_MDSS_DPTX0_LINK_CLK_SRC: c_int = 15;
pub const DISP_CC_MDSS_DPTX0_LINK_DIV_CLK_SRC: c_int = 16;
pub const DISP_CC_MDSS_DPTX0_LINK_INTF_CLK: c_int = 17;
pub const DISP_CC_MDSS_DPTX0_PIXEL0_CLK: c_int = 18;
pub const DISP_CC_MDSS_DPTX0_PIXEL0_CLK_SRC: c_int = 19;
pub const DISP_CC_MDSS_DPTX0_PIXEL1_CLK: c_int = 20;
pub const DISP_CC_MDSS_DPTX0_PIXEL1_CLK_SRC: c_int = 21;
pub const DISP_CC_MDSS_DPTX0_USB_ROUTER_LINK_INTF_CLK: c_int = 22;
pub const DISP_CC_MDSS_DPTX1_AUX_CLK: c_int = 23;
pub const DISP_CC_MDSS_DPTX1_AUX_CLK_SRC: c_int = 24;
pub const DISP_CC_MDSS_DPTX1_LINK_CLK: c_int = 25;
pub const DISP_CC_MDSS_DPTX1_LINK_CLK_SRC: c_int = 26;
pub const DISP_CC_MDSS_DPTX1_LINK_DIV_CLK_SRC: c_int = 27;
pub const DISP_CC_MDSS_DPTX1_LINK_INTF_CLK: c_int = 28;
pub const DISP_CC_MDSS_DPTX1_PIXEL0_CLK: c_int = 29;
pub const DISP_CC_MDSS_DPTX1_PIXEL0_CLK_SRC: c_int = 30;
pub const DISP_CC_MDSS_DPTX1_PIXEL1_CLK: c_int = 31;
pub const DISP_CC_MDSS_DPTX1_PIXEL1_CLK_SRC: c_int = 32;
pub const DISP_CC_MDSS_DPTX1_USB_ROUTER_LINK_INTF_CLK: c_int = 33;
pub const DISP_CC_MDSS_DPTX2_AUX_CLK: c_int = 34;
pub const DISP_CC_MDSS_DPTX2_AUX_CLK_SRC: c_int = 35;
pub const DISP_CC_MDSS_DPTX2_LINK_CLK: c_int = 36;
pub const DISP_CC_MDSS_DPTX2_LINK_CLK_SRC: c_int = 37;
pub const DISP_CC_MDSS_DPTX2_LINK_DIV_CLK_SRC: c_int = 38;
pub const DISP_CC_MDSS_DPTX2_LINK_INTF_CLK: c_int = 39;
pub const DISP_CC_MDSS_DPTX2_PIXEL0_CLK: c_int = 40;
pub const DISP_CC_MDSS_DPTX2_PIXEL0_CLK_SRC: c_int = 41;
pub const DISP_CC_MDSS_DPTX2_PIXEL1_CLK: c_int = 42;
pub const DISP_CC_MDSS_DPTX2_PIXEL1_CLK_SRC: c_int = 43;
pub const DISP_CC_MDSS_DPTX2_USB_ROUTER_LINK_INTF_CLK: c_int = 44;
pub const DISP_CC_MDSS_DPTX3_AUX_CLK: c_int = 45;
pub const DISP_CC_MDSS_DPTX3_AUX_CLK_SRC: c_int = 46;
pub const DISP_CC_MDSS_DPTX3_LINK_CLK: c_int = 47;
pub const DISP_CC_MDSS_DPTX3_LINK_CLK_SRC: c_int = 48;
pub const DISP_CC_MDSS_DPTX3_LINK_DIV_CLK_SRC: c_int = 49;
pub const DISP_CC_MDSS_DPTX3_LINK_INTF_CLK: c_int = 50;
pub const DISP_CC_MDSS_DPTX3_PIXEL0_CLK: c_int = 51;
pub const DISP_CC_MDSS_DPTX3_PIXEL0_CLK_SRC: c_int = 52;
pub const DISP_CC_MDSS_ESC0_CLK: c_int = 53;
pub const DISP_CC_MDSS_ESC0_CLK_SRC: c_int = 54;
pub const DISP_CC_MDSS_ESC1_CLK: c_int = 55;
pub const DISP_CC_MDSS_ESC1_CLK_SRC: c_int = 56;
pub const DISP_CC_MDSS_MDP1_CLK: c_int = 57;
pub const DISP_CC_MDSS_MDP_CLK: c_int = 58;
pub const DISP_CC_MDSS_MDP_CLK_SRC: c_int = 59;
pub const DISP_CC_MDSS_MDP_LUT1_CLK: c_int = 60;
pub const DISP_CC_MDSS_MDP_LUT_CLK: c_int = 61;
pub const DISP_CC_MDSS_NON_GDSC_AHB_CLK: c_int = 62;
pub const DISP_CC_MDSS_PCLK0_CLK: c_int = 63;
pub const DISP_CC_MDSS_PCLK0_CLK_SRC: c_int = 64;
pub const DISP_CC_MDSS_PCLK1_CLK: c_int = 65;
pub const DISP_CC_MDSS_PCLK1_CLK_SRC: c_int = 66;
pub const DISP_CC_MDSS_RSCC_AHB_CLK: c_int = 67;
pub const DISP_CC_MDSS_RSCC_VSYNC_CLK: c_int = 68;
pub const DISP_CC_MDSS_VSYNC1_CLK: c_int = 69;
pub const DISP_CC_MDSS_VSYNC_CLK: c_int = 70;
pub const DISP_CC_MDSS_VSYNC_CLK_SRC: c_int = 71;
pub const DISP_CC_PLL0: c_int = 72;
pub const DISP_CC_PLL1: c_int = 73;
pub const DISP_CC_SLEEP_CLK: c_int = 74;
pub const DISP_CC_SLEEP_CLK_SRC: c_int = 75;
pub const DISP_CC_XO_CLK: c_int = 76;
pub const DISP_CC_XO_CLK_SRC: c_int = 77;
// DISP_CC resets
pub const DISP_CC_MDSS_CORE_BCR: c_int = 0;
pub const DISP_CC_MDSS_CORE_INT2_BCR: c_int = 1;
pub const DISP_CC_MDSS_RSCC_BCR: c_int = 2;
pub const DISP_CC_MDSS_DPTX0_USB_ROUTER_LINK_INTF_CLK_ARES: c_int = 3;
pub const DISP_CC_MDSS_DPTX1_USB_ROUTER_LINK_INTF_CLK_ARES: c_int = 4;
pub const DISP_CC_MDSS_DPTX2_USB_ROUTER_LINK_INTF_CLK_ARES: c_int = 5;
// DISP_CC GDSCR
pub const MDSS_GDSC: c_int = 0;
pub const MDSS_INT2_GDSC: c_int = 1;
