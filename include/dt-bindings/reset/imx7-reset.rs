//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/imx7-reset.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2017 Impinj, Inc.
//
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//
pub const IMX7_RESET_A7_CORE_POR_RESET0: c_int = 0;
pub const IMX7_RESET_A7_CORE_POR_RESET1: c_int = 1;
pub const IMX7_RESET_A7_CORE_RESET0: c_int = 2;
pub const IMX7_RESET_A7_CORE_RESET1: c_int = 3;
pub const IMX7_RESET_A7_DBG_RESET0: c_int = 4;
pub const IMX7_RESET_A7_DBG_RESET1: c_int = 5;
pub const IMX7_RESET_A7_ETM_RESET0: c_int = 6;
pub const IMX7_RESET_A7_ETM_RESET1: c_int = 7;
pub const IMX7_RESET_A7_SOC_DBG_RESET: c_int = 8;
pub const IMX7_RESET_A7_L2RESET: c_int = 9;
pub const IMX7_RESET_SW_M4C_RST: c_int = 10;
pub const IMX7_RESET_SW_M4P_RST: c_int = 11;
pub const IMX7_RESET_EIM_RST: c_int = 12;
pub const IMX7_RESET_HSICPHY_PORT_RST: c_int = 13;
pub const IMX7_RESET_USBPHY1_POR: c_int = 14;
pub const IMX7_RESET_USBPHY1_PORT_RST: c_int = 15;
pub const IMX7_RESET_USBPHY2_POR: c_int = 16;
pub const IMX7_RESET_USBPHY2_PORT_RST: c_int = 17;
pub const IMX7_RESET_MIPI_PHY_MRST: c_int = 18;
pub const IMX7_RESET_MIPI_PHY_SRST: c_int = 19;
//
// IMX7_RESET_PCIEPHY is a logical reset line combining PCIEPHY_BTN
// and PCIEPHY_G_RST
//
pub const IMX7_RESET_PCIEPHY: c_int = 20;
pub const IMX7_RESET_PCIEPHY_PERST: c_int = 21;
//
// IMX7_RESET_PCIE_CTRL_APPS_EN is not strictly a reset line, but it
// can be used to inhibit PCIe LTTSM, so, in a way, it can be thoguht
// of as one
//
pub const IMX7_RESET_PCIE_CTRL_APPS_EN: c_int = 22;
pub const IMX7_RESET_DDRC_PRST: c_int = 23;
pub const IMX7_RESET_DDRC_CORE_RST: c_int = 24;
pub const IMX7_RESET_PCIE_CTRL_APPS_TURNOFF: c_int = 25;
pub const IMX7_RESET_NUM: c_int = 26;
