//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/lantiq/mxl-gsw1xx.h
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
// Register definitions for MaxLinear GSW1xx series switches
//
// Copyright (C) 2025 Daniel Golle <daniel@makrotopia.org>
// Copyright (C) 2023 - 2024 MaxLinear Inc.
//

pub const GSW1XX_PORTS: c_int = 6;
pub const GSW150_PORTS: c_int = 7;
// Port used for RGMII or optional RMII
pub const GSW1XX_MII_PORT: c_int = 5;
// Port used for SGMII
pub const GSW1XX_SGMII_PORT: c_int = 4;
pub const GSW1XX_SYS_CLK_FREQ: c_int = 340000000;
// SMDIO switch register base address
pub const GSW1XX_SMDIO_BADR: c_uint = 0x1f;

// GSW1XX SGMII PCS
pub const GSW1XX_SGMII_BASE: c_uint = 0xd000;
pub const GSW1XX_SGMII_PHY_HWBU_CTRL: c_uint = 0x009;

pub const GSW1XX_SGMII_TBI_TXANEGH: c_uint = 0x300;
pub const GSW1XX_SGMII_TBI_TXANEGL: c_uint = 0x301;
pub const GSW1XX_SGMII_TBI_ANEGCTL: c_uint = 0x304;

pub const GSW1XX_SGMII_TBI_ANEGCTL_LT_10US: c_int = 0;
pub const GSW1XX_SGMII_TBI_ANEGCTL_LT_1_6MS: c_int = 1;
pub const GSW1XX_SGMII_TBI_ANEGCTL_LT_5MS: c_int = 2;
pub const GSW1XX_SGMII_TBI_ANEGCTL_LT_10MS: c_int = 3;

pub const GSW1XX_SGMII_TBI_ANEGCTL_ANMODE_1000BASEX: c_int = 1;
pub const GSW1XX_SGMII_TBI_ANEGCTL_ANMODE_SGMII_PHY: c_int = 2;
pub const GSW1XX_SGMII_TBI_ANEGCTL_ANMODE_SGMII_MAC: c_int = 3;

pub const GSW1XX_SGMII_TBI_TBICTL: c_uint = 0x305;

pub const GSW1XX_SGMII_TBI_TBISTAT: c_uint = 0x309;

pub const GSW1XX_SGMII_TBI_LPSTAT: c_uint = 0x30a;

pub const GSW1XX_SGMII_TBI_LPSTAT_SPEED_10: c_int = 0;
pub const GSW1XX_SGMII_TBI_LPSTAT_SPEED_100: c_int = 1;
pub const GSW1XX_SGMII_TBI_LPSTAT_SPEED_1000: c_int = 2;
pub const GSW1XX_SGMII_TBI_LPSTAT_SPEED_NOSGMII: c_int = 3;
pub const GSW1XX_SGMII_PHY_D: c_uint = 0x100;
pub const GSW1XX_SGMII_PHY_A: c_uint = 0x101;
pub const GSW1XX_SGMII_PHY_C: c_uint = 0x102;

pub const GSW1XX_SGMII_PCS_RXB_CTL: c_uint = 0x401;

pub const GSW1XX_SGMII_PCS_TXB_CTL: c_uint = 0x404;

pub const GSW1XX_SGMII_PHY_RX0_CFG2: c_uint = 0x004;

pub const GSW1XX_SGMII_PHY_RX0_CFG2_EQ_DEF: c_int = 2;

pub const GSW1XX_SGMII_PHY_RX0_CFG2_FILT_CNT_DEF: c_int = 20;
pub const GSW1XX_SGMII_PHY_TX0_CFG3: c_uint = 0x007;

pub const GSW1XX_SGMII_PHY_TX0_CFG3_VBOOST_LEVEL_DEF: c_int = 4;

// GSW1XX PDI Registers
pub const GSW1XX_SWITCH_BASE: c_uint = 0xe000;
// GSW1XX MII Registers
pub const GSW1XX_RGMII_BASE: c_uint = 0xf100;
// GSW1XX GPIO Registers
pub const GSW1XX_GPIO_BASE: c_uint = 0xf300;
pub const GPIO_ALTSEL0: c_uint = 0x83;
pub const GPIO_ALTSEL0_EXTPHY_MUX_VAL: c_uint = 0x03c3;
pub const GPIO_ALTSEL1: c_uint = 0x84;
pub const GPIO_ALTSEL1_EXTPHY_MUX_VAL: c_uint = 0x003f;
// MDIO bus controller
pub const GSW1XX_MMDIO_BASE: c_uint = 0xf400;
// generic IC registers
pub const GSW1XX_SHELL_BASE: c_uint = 0xfa00;
pub const GSW1XX_SHELL_RST_REQ: c_uint = 0x01;

pub const GSW1XX_SHELL_MANU_ID: c_uint = 0x10;

pub const GSW1XX_SHELL_MANU_ID_MANID_VAL: c_uint = 0x389;

pub const GSW1XX_SHELL_PNUM_ID: c_uint = 0x11;

// RGMII PAD Slew Control Register
pub const GSW1XX_SHELL_RGMII_SLEW_CFG: c_uint = 0x78;

// SGMII clock related settings
pub const GSW1XX_CLK_BASE: c_uint = 0xf900;
pub const GSW1XX_CLK_NCO_CTRL: c_uint = 0x68;

pub const GSW1XX_SGMII_1G: c_uint = 0x0;
pub const GSW1XX_SGMII_2G5: c_uint = 0xc;
pub const GSW1XX_SGMII_1G_NCO1: c_uint = 0x0;
pub const GSW1XX_SGMII_2G5_NCO2: c_uint = 0x2;
