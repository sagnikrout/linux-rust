//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/realtek,rtd1295.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// Realtek RTD1295 reset controllers
//
// Copyright (c) 2017 Andreas Färber
//
// soft reset 1
pub const RTD1295_RSTN_MISC: c_int = 0;
pub const RTD1295_RSTN_NAT: c_int = 1;
pub const RTD1295_RSTN_USB3_PHY0_POW: c_int = 2;
pub const RTD1295_RSTN_GSPI: c_int = 3;
pub const RTD1295_RSTN_USB3_P0_MDIO: c_int = 4;
pub const RTD1295_RSTN_SATA_0: c_int = 5;
pub const RTD1295_RSTN_USB: c_int = 6;
pub const RTD1295_RSTN_SATA_PHY_0: c_int = 7;
pub const RTD1295_RSTN_USB_PHY0: c_int = 8;
pub const RTD1295_RSTN_USB_PHY1: c_int = 9;
pub const RTD1295_RSTN_SATA_PHY_POW_0: c_int = 10;
pub const RTD1295_RSTN_SATA_FUNC_EXIST_0: c_int = 11;
pub const RTD1295_RSTN_HDMI: c_int = 12;
pub const RTD1295_RSTN_VE1: c_int = 13;
pub const RTD1295_RSTN_VE2: c_int = 14;
pub const RTD1295_RSTN_VE3: c_int = 15;
pub const RTD1295_RSTN_ETN: c_int = 16;
pub const RTD1295_RSTN_AIO: c_int = 17;
pub const RTD1295_RSTN_GPU: c_int = 18;
pub const RTD1295_RSTN_TVE: c_int = 19;
pub const RTD1295_RSTN_VO: c_int = 20;
pub const RTD1295_RSTN_LVDS: c_int = 21;
pub const RTD1295_RSTN_SE: c_int = 22;
pub const RTD1295_RSTN_DCU: c_int = 23;
pub const RTD1295_RSTN_DC_PHY: c_int = 24;
pub const RTD1295_RSTN_CP: c_int = 25;
pub const RTD1295_RSTN_MD: c_int = 26;
pub const RTD1295_RSTN_TP: c_int = 27;
pub const RTD1295_RSTN_AE: c_int = 28;
pub const RTD1295_RSTN_NF: c_int = 29;
pub const RTD1295_RSTN_MIPI: c_int = 30;
pub const RTD1295_RSTN_RSA: c_int = 31;
// soft reset 2
pub const RTD1295_RSTN_ACPU: c_int = 0;
pub const RTD1295_RSTN_JPEG: c_int = 1;
pub const RTD1295_RSTN_USB_PHY3: c_int = 2;
pub const RTD1295_RSTN_USB_PHY2: c_int = 3;
pub const RTD1295_RSTN_USB3_PHY1_POW: c_int = 4;
pub const RTD1295_RSTN_USB3_P1_MDIO: c_int = 5;
pub const RTD1295_RSTN_PCIE0_STITCH: c_int = 6;
pub const RTD1295_RSTN_PCIE0_PHY: c_int = 7;
pub const RTD1295_RSTN_PCIE0: c_int = 8;
pub const RTD1295_RSTN_PCR_CNT: c_int = 9;
pub const RTD1295_RSTN_CR: c_int = 10;
pub const RTD1295_RSTN_EMMC: c_int = 11;
pub const RTD1295_RSTN_SDIO: c_int = 12;
pub const RTD1295_RSTN_PCIE0_CORE: c_int = 13;
pub const RTD1295_RSTN_PCIE0_POWER: c_int = 14;
pub const RTD1295_RSTN_PCIE0_NONSTICH: c_int = 15;
pub const RTD1295_RSTN_PCIE1_PHY: c_int = 16;
pub const RTD1295_RSTN_PCIE1: c_int = 17;
pub const RTD1295_RSTN_I2C_5: c_int = 18;
pub const RTD1295_RSTN_PCIE1_STITCH: c_int = 19;
pub const RTD1295_RSTN_PCIE1_CORE: c_int = 20;
pub const RTD1295_RSTN_PCIE1_POWER: c_int = 21;
pub const RTD1295_RSTN_PCIE1_NONSTICH: c_int = 22;
pub const RTD1295_RSTN_I2C_4: c_int = 23;
pub const RTD1295_RSTN_I2C_3: c_int = 24;
pub const RTD1295_RSTN_I2C_2: c_int = 25;
pub const RTD1295_RSTN_I2C_1: c_int = 26;
pub const RTD1295_RSTN_UR2: c_int = 27;
pub const RTD1295_RSTN_UR1: c_int = 28;
pub const RTD1295_RSTN_MISC_SC: c_int = 29;
pub const RTD1295_RSTN_CBUS_TX: c_int = 30;
pub const RTD1295_RSTN_SDS_PHY: c_int = 31;
// soft reset 3
pub const RTD1295_RSTN_SB2: c_int = 0;
// soft reset 4
pub const RTD1295_RSTN_DCPHY_CRT: c_int = 0;
pub const RTD1295_RSTN_DCPHY_ALERT_RX: c_int = 1;
pub const RTD1295_RSTN_DCPHY_PTR: c_int = 2;
pub const RTD1295_RSTN_DCPHY_LDO: c_int = 3;
pub const RTD1295_RSTN_DCPHY_SSC_DIG: c_int = 4;
pub const RTD1295_RSTN_HDMIRX: c_int = 5;
pub const RTD1295_RSTN_CBUSRX: c_int = 6;
pub const RTD1295_RSTN_SATA_PHY_POW_1: c_int = 7;
pub const RTD1295_RSTN_SATA_FUNC_EXIST_1: c_int = 8;
pub const RTD1295_RSTN_SATA_PHY_1: c_int = 9;
pub const RTD1295_RSTN_SATA_1: c_int = 10;
pub const RTD1295_RSTN_FAN: c_int = 11;
pub const RTD1295_RSTN_HDMIRX_WRAP: c_int = 12;
pub const RTD1295_RSTN_PCIE0_PHY_MDIO: c_int = 13;
pub const RTD1295_RSTN_PCIE1_PHY_MDIO: c_int = 14;
pub const RTD1295_RSTN_DISP: c_int = 15;
// iso reset
pub const RTD1295_ISO_RSTN_IR: c_int = 1;
pub const RTD1295_ISO_RSTN_CEC0: c_int = 2;
pub const RTD1295_ISO_RSTN_CEC1: c_int = 3;
pub const RTD1295_ISO_RSTN_DP: c_int = 4;
pub const RTD1295_ISO_RSTN_CBUSTX: c_int = 5;
pub const RTD1295_ISO_RSTN_CBUSRX: c_int = 6;
pub const RTD1295_ISO_RSTN_EFUSE: c_int = 7;
pub const RTD1295_ISO_RSTN_UR0: c_int = 8;
pub const RTD1295_ISO_RSTN_GMAC: c_int = 9;
pub const RTD1295_ISO_RSTN_GPHY: c_int = 10;
pub const RTD1295_ISO_RSTN_I2C_0: c_int = 11;
pub const RTD1295_ISO_RSTN_I2C_1: c_int = 12;
pub const RTD1295_ISO_RSTN_CBUS: c_int = 13;
