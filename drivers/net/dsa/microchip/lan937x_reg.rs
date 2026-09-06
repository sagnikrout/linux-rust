//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/lan937x_reg.h
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
// Microchip LAN937X switch register definitions
// Copyright (C) 2019-2024 Microchip Technology Inc.
//

// 0 - Operation
pub const REG_GLOBAL_CTRL_0: c_uint = 0x0007;

pub const REG_SW_INT_STATUS__4: c_uint = 0x0010;
pub const REG_SW_INT_MASK__4: c_uint = 0x0014;

pub const REG_SW_PORT_INT_STATUS__4: c_uint = 0x0018;
pub const REG_SW_PORT_INT_MASK__4: c_uint = 0x001C;
// 1 - Global
pub const REG_SW_GLOBAL_OUTPUT_CTRL__1: c_uint = 0x0103;

pub const REG_SW_CFG_STRAP_VAL: c_uint = 0x0200;

// 2 - PHY Control
pub const REG_SW_CFG_STRAP_OVR: c_uint = 0x0214;

// 3 - Operation Control
pub const REG_SW_OPERATION: c_uint = 0x0300;

pub const REG_SW_LUE_CTRL_0: c_uint = 0x0310;

pub const REG_SW_LUE_CTRL_1: c_uint = 0x0311;

pub const REG_SW_LUE_CTRL_2: c_uint = 0x0312;

pub const REG_SW_AGE_PERIOD__1: c_uint = 0x0313;

pub const REG_SW_AGE_PERIOD__2: c_uint = 0x0320;

pub const REG_SW_MAC_CTRL_0: c_uint = 0x0330;

pub const REG_SW_MAC_CTRL_1: c_uint = 0x0331;

pub const REG_SW_MAC_CTRL_6: c_uint = 0x0336;

// 4 - LUE
pub const REG_SW_ALU_STAT_CTRL__4: c_uint = 0x041C;
pub const REG_SW_ALU_VAL_B: c_uint = 0x0424;

pub const ALU_V_PORT_MAP: c_uint = 0xFF;
// 7 - VPhy
pub const REG_VPHY_IND_ADDR__2: c_uint = 0x075C;
pub const REG_VPHY_IND_DATA__2: c_uint = 0x0760;
pub const REG_VPHY_IND_CTRL__2: c_uint = 0x0768;

pub const REG_VPHY_SPECIAL_CTRL__2: c_uint = 0x077C;

pub const VPHY_PORT_MODE_M: c_uint = 0x3;
pub const VPHY_PORT_MODE_S: c_int = 8;
pub const VPHY_MODE_RGMII: c_int = 0;
pub const VPHY_MODE_MII_PHY: c_int = 1;
pub const VPHY_MODE_SGMII: c_int = 2;
pub const VPHY_MODE_RMII_PHY: c_int = 3;

pub const VPHY_SPEED_DUPLEX_STAT_M: c_uint = 0x7;
pub const VPHY_SPEED_DUPLEX_STAT_S: c_int = 2;

// Port Registers
// 0 - Operation
pub const REG_PORT_INT_STATUS: c_uint = 0x001B;
pub const REG_PORT_INT_MASK: c_uint = 0x001F;

pub const PORT_SRC_PHY_INT: c_int = 1;
pub const REG_PORT_CTRL_0: c_uint = 0x0020;

pub const PORT_QUEUE_SPLIT_ENABLE: c_uint = 0x3;
// 1 - Phy
pub const REG_PORT_T1_PHY_CTRL_BASE: c_uint = 0x0100;
pub const REG_PORT_TX_PHY_CTRL_BASE: c_uint = 0x0280;
// 3 - xMII

pub const REG_PORT_XMII_CTRL_4: c_uint = 0x0304;
pub const REG_PORT_XMII_CTRL_5: c_uint = 0x0306;

// 4 - MAC
pub const REG_PORT_MAC_CTRL_0: c_uint = 0x0400;

pub const REG_PORT_MAC_CTRL_1: c_uint = 0x0401;

pub const PORT_MAX_FR_SIZE: c_uint = 0x404;
pub const FR_MIN_SIZE: c_int = 1522;
// 8 - Classification and Policing
pub const REG_PORT_MRI_PRIO_CTRL: c_uint = 0x0801;

// 9 - Shaping
pub const REG_PORT_MTI_CREDIT_INCREMENT: c_uint = 0x091C;
// The port number as per the datasheet
pub const RGMII_2_PORT_NUM: c_int = 5;
pub const RGMII_1_PORT_NUM: c_int = 6;

pub const RGMII_1_TX_DELAY_2NS: c_int = 2;
pub const RGMII_2_TX_DELAY_2NS: c_int = 0;
pub const RGMII_1_RX_DELAY_2NS: c_uint = 0x1B;
pub const RGMII_2_RX_DELAY_2NS: c_uint = 0x14;
pub const LAN937X_TAG_LEN: c_int = 2;
