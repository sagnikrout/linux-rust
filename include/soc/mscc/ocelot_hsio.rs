//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_hsio.h
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
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//
pub const HSIO_PLL5G_CFG0: c_uint = 0x0000;
pub const HSIO_PLL5G_CFG1: c_uint = 0x0004;
pub const HSIO_PLL5G_CFG2: c_uint = 0x0008;
pub const HSIO_PLL5G_CFG3: c_uint = 0x000c;
pub const HSIO_PLL5G_CFG4: c_uint = 0x0010;
pub const HSIO_PLL5G_CFG5: c_uint = 0x0014;
pub const HSIO_PLL5G_CFG6: c_uint = 0x0018;
pub const HSIO_PLL5G_STATUS0: c_uint = 0x001c;
pub const HSIO_PLL5G_STATUS1: c_uint = 0x0020;
pub const HSIO_PLL5G_BIST_CFG0: c_uint = 0x0024;
pub const HSIO_PLL5G_BIST_CFG1: c_uint = 0x0028;
pub const HSIO_PLL5G_BIST_CFG2: c_uint = 0x002c;
pub const HSIO_PLL5G_BIST_STAT0: c_uint = 0x0030;
pub const HSIO_PLL5G_BIST_STAT1: c_uint = 0x0034;
pub const HSIO_RCOMP_CFG0: c_uint = 0x0038;
pub const HSIO_RCOMP_STATUS: c_uint = 0x003c;
pub const HSIO_SYNC_ETH_CFG: c_uint = 0x0040;
pub const HSIO_SYNC_ETH_PLL_CFG: c_uint = 0x0048;
pub const HSIO_S1G_DES_CFG: c_uint = 0x004c;
pub const HSIO_S1G_IB_CFG: c_uint = 0x0050;
pub const HSIO_S1G_OB_CFG: c_uint = 0x0054;
pub const HSIO_S1G_SER_CFG: c_uint = 0x0058;
pub const HSIO_S1G_COMMON_CFG: c_uint = 0x005c;
pub const HSIO_S1G_PLL_CFG: c_uint = 0x0060;
pub const HSIO_S1G_PLL_STATUS: c_uint = 0x0064;
pub const HSIO_S1G_DFT_CFG0: c_uint = 0x0068;
pub const HSIO_S1G_DFT_CFG1: c_uint = 0x006c;
pub const HSIO_S1G_DFT_CFG2: c_uint = 0x0070;
pub const HSIO_S1G_TP_CFG: c_uint = 0x0074;
pub const HSIO_S1G_RC_PLL_BIST_CFG: c_uint = 0x0078;
pub const HSIO_S1G_MISC_CFG: c_uint = 0x007c;
pub const HSIO_S1G_DFT_STATUS: c_uint = 0x0080;
pub const HSIO_S1G_MISC_STATUS: c_uint = 0x0084;
pub const HSIO_MCB_S1G_ADDR_CFG: c_uint = 0x0088;
pub const HSIO_S6G_DIG_CFG: c_uint = 0x008c;
pub const HSIO_S6G_DFT_CFG0: c_uint = 0x0090;
pub const HSIO_S6G_DFT_CFG1: c_uint = 0x0094;
pub const HSIO_S6G_DFT_CFG2: c_uint = 0x0098;
pub const HSIO_S6G_TP_CFG0: c_uint = 0x009c;
pub const HSIO_S6G_TP_CFG1: c_uint = 0x00a0;
pub const HSIO_S6G_RC_PLL_BIST_CFG: c_uint = 0x00a4;
pub const HSIO_S6G_MISC_CFG: c_uint = 0x00a8;
pub const HSIO_S6G_OB_ANEG_CFG: c_uint = 0x00ac;
pub const HSIO_S6G_DFT_STATUS: c_uint = 0x00b0;
pub const HSIO_S6G_ERR_CNT: c_uint = 0x00b4;
pub const HSIO_S6G_MISC_STATUS: c_uint = 0x00b8;
pub const HSIO_S6G_DES_CFG: c_uint = 0x00bc;
pub const HSIO_S6G_IB_CFG: c_uint = 0x00c0;
pub const HSIO_S6G_IB_CFG1: c_uint = 0x00c4;
pub const HSIO_S6G_IB_CFG2: c_uint = 0x00c8;
pub const HSIO_S6G_IB_CFG3: c_uint = 0x00cc;
pub const HSIO_S6G_IB_CFG4: c_uint = 0x00d0;
pub const HSIO_S6G_IB_CFG5: c_uint = 0x00d4;
pub const HSIO_S6G_OB_CFG: c_uint = 0x00d8;
pub const HSIO_S6G_OB_CFG1: c_uint = 0x00dc;
pub const HSIO_S6G_SER_CFG: c_uint = 0x00e0;
pub const HSIO_S6G_COMMON_CFG: c_uint = 0x00e4;
pub const HSIO_S6G_PLL_CFG: c_uint = 0x00e8;
pub const HSIO_S6G_ACJTAG_CFG: c_uint = 0x00ec;
pub const HSIO_S6G_GP_CFG: c_uint = 0x00f0;
pub const HSIO_S6G_IB_STATUS0: c_uint = 0x00f4;
pub const HSIO_S6G_IB_STATUS1: c_uint = 0x00f8;
pub const HSIO_S6G_ACJTAG_STATUS: c_uint = 0x00fc;
pub const HSIO_S6G_PLL_STATUS: c_uint = 0x0100;
pub const HSIO_S6G_REVID: c_uint = 0x0104;
pub const HSIO_MCB_S6G_ADDR_CFG: c_uint = 0x0108;
pub const HSIO_HW_CFG: c_uint = 0x010c;
pub const HSIO_HW_QSGMII_CFG: c_uint = 0x0110;
pub const HSIO_HW_QSGMII_STAT: c_uint = 0x0114;
pub const HSIO_CLK_CFG: c_uint = 0x0118;
pub const HSIO_TEMP_SENSOR_CTRL: c_uint = 0x011c;
pub const HSIO_TEMP_SENSOR_CFG: c_uint = 0x0120;
pub const HSIO_TEMP_SENSOR_STAT: c_uint = 0x0124;

pub const HSIO_SYNC_ETH_CFG_RSZ: c_uint = 0x4;

