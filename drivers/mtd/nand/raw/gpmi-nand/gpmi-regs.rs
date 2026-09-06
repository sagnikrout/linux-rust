//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/gpmi-nand/gpmi-regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Freescale GPMI NAND Flash Driver
//
// Copyright 2008-2011 Freescale Semiconductor, Inc.
// Copyright 2008 Embedded Alley Solutions, Inc.
//
pub const HW_GPMI_CTRL0: c_uint = 0x00000000;
pub const HW_GPMI_CTRL0_SET: c_uint = 0x00000004;
pub const HW_GPMI_CTRL0_CLR: c_uint = 0x00000008;
pub const HW_GPMI_CTRL0_TOG: c_uint = 0x0000000c;
pub const BP_GPMI_CTRL0_COMMAND_MODE: c_int = 24;

pub const BV_GPMI_CTRL0_COMMAND_MODE__WRITE: c_uint = 0x0;
pub const BV_GPMI_CTRL0_COMMAND_MODE__READ: c_uint = 0x1;
pub const BV_GPMI_CTRL0_COMMAND_MODE__READ_AND_COMPARE: c_uint = 0x2;
pub const BV_GPMI_CTRL0_COMMAND_MODE__WAIT_FOR_READY: c_uint = 0x3;

pub const BV_GPMI_CTRL0_WORD_LENGTH__16_BIT: c_uint = 0x0;
pub const BV_GPMI_CTRL0_WORD_LENGTH__8_BIT: c_uint = 0x1;
//
// Difference in LOCK_CS between imx23 and imx28 :
// This bit may impact the _POWER_ consumption. So some chips
// do not set it.
//
pub const MX23_BP_GPMI_CTRL0_LOCK_CS: c_int = 22;
pub const MX28_BP_GPMI_CTRL0_LOCK_CS: c_int = 27;
pub const LOCK_CS_ENABLE: c_uint = 0x1;

// Difference in CS between imx23 and imx28
pub const BP_GPMI_CTRL0_CS: c_int = 20;

pub const BP_GPMI_CTRL0_ADDRESS: c_int = 17;

pub const BV_GPMI_CTRL0_ADDRESS__NAND_DATA: c_uint = 0x0;
pub const BV_GPMI_CTRL0_ADDRESS__NAND_CLE: c_uint = 0x1;
pub const BV_GPMI_CTRL0_ADDRESS__NAND_ALE: c_uint = 0x2;

pub const BV_GPMI_CTRL0_ADDRESS_INCREMENT__DISABLED: c_uint = 0x0;
pub const BV_GPMI_CTRL0_ADDRESS_INCREMENT__ENABLED: c_uint = 0x1;
pub const BP_GPMI_CTRL0_XFER_COUNT: c_int = 0;

pub const HW_GPMI_COMPARE: c_uint = 0x00000010;
pub const HW_GPMI_ECCCTRL: c_uint = 0x00000020;
pub const HW_GPMI_ECCCTRL_SET: c_uint = 0x00000024;
pub const HW_GPMI_ECCCTRL_CLR: c_uint = 0x00000028;
pub const HW_GPMI_ECCCTRL_TOG: c_uint = 0x0000002c;
pub const BP_GPMI_ECCCTRL_ECC_CMD: c_int = 13;

pub const BV_GPMI_ECCCTRL_ECC_CMD__BCH_DECODE: c_uint = 0x0;
pub const BV_GPMI_ECCCTRL_ECC_CMD__BCH_ENCODE: c_uint = 0x1;

pub const BV_GPMI_ECCCTRL_ENABLE_ECC__ENABLE: c_uint = 0x1;
pub const BV_GPMI_ECCCTRL_ENABLE_ECC__DISABLE: c_uint = 0x0;
pub const BP_GPMI_ECCCTRL_BUFFER_MASK: c_int = 0;

pub const BV_GPMI_ECCCTRL_BUFFER_MASK__BCH_AUXONLY: c_uint = 0x100;
pub const BV_GPMI_ECCCTRL_BUFFER_MASK__BCH_PAGE: c_uint = 0x1FF;
pub const HW_GPMI_ECCCOUNT: c_uint = 0x00000030;
pub const HW_GPMI_PAYLOAD: c_uint = 0x00000040;
pub const HW_GPMI_AUXILIARY: c_uint = 0x00000050;
pub const HW_GPMI_CTRL1: c_uint = 0x00000060;
pub const HW_GPMI_CTRL1_SET: c_uint = 0x00000064;
pub const HW_GPMI_CTRL1_CLR: c_uint = 0x00000068;
pub const HW_GPMI_CTRL1_TOG: c_uint = 0x0000006c;
pub const BP_GPMI_CTRL1_DECOUPLE_CS: c_int = 24;

pub const BP_GPMI_CTRL1_WRN_DLY_SEL: c_int = 22;

pub const BV_GPMI_CTRL1_WRN_DLY_SEL_4_TO_8NS: c_uint = 0x0;
pub const BV_GPMI_CTRL1_WRN_DLY_SEL_6_TO_10NS: c_uint = 0x1;
pub const BV_GPMI_CTRL1_WRN_DLY_SEL_7_TO_12NS: c_uint = 0x2;
pub const BV_GPMI_CTRL1_WRN_DLY_SEL_NO_DELAY: c_uint = 0x3;

pub const BP_GPMI_CTRL1_DLL_ENABLE: c_int = 17;

pub const BP_GPMI_CTRL1_HALF_PERIOD: c_int = 16;

pub const BP_GPMI_CTRL1_RDN_DELAY: c_int = 12;

pub const BV_GPMI_CTRL1_DEV_RESET__ENABLED: c_uint = 0x0;
pub const BV_GPMI_CTRL1_DEV_RESET__DISABLED: c_uint = 0x1;

pub const BV_GPMI_CTRL1_ATA_IRQRDY_POLARITY__ACTIVELOW: c_uint = 0x0;
pub const BV_GPMI_CTRL1_ATA_IRQRDY_POLARITY__ACTIVEHIGH: c_uint = 0x1;

pub const BV_GPMI_CTRL1_GPMI_MODE__NAND: c_uint = 0x0;
pub const BV_GPMI_CTRL1_GPMI_MODE__ATA: c_uint = 0x1;

pub const HW_GPMI_TIMING0: c_uint = 0x00000070;
pub const BP_GPMI_TIMING0_ADDRESS_SETUP: c_int = 16;

pub const BP_GPMI_TIMING0_DATA_HOLD: c_int = 8;

pub const BP_GPMI_TIMING0_DATA_SETUP: c_int = 0;

pub const HW_GPMI_TIMING1: c_uint = 0x00000080;
pub const BP_GPMI_TIMING1_BUSY_TIMEOUT: c_int = 16;

pub const HW_GPMI_TIMING2: c_uint = 0x00000090;
pub const HW_GPMI_DATA: c_uint = 0x000000a0;
// MX28 uses this to detect READY.
pub const HW_GPMI_STAT: c_uint = 0x000000b0;
pub const MX28_BP_GPMI_STAT_READY_BUSY: c_int = 24;

// MX23 uses this to detect READY.
pub const HW_GPMI_DEBUG: c_uint = 0x000000c0;
pub const MX23_BP_GPMI_DEBUG_READY0: c_int = 28;

