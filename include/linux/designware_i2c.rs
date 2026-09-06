//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/designware_i2c.h
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
//
// Synopsys DesignWare I2C register definitions
//
// Copyright (C) 2026, Intel Corporation
//

//
// Registers offset
//
pub const DW_IC_CON: c_uint = 0x00;
pub const DW_IC_TAR: c_uint = 0x04;
pub const DW_IC_SAR: c_uint = 0x08;
pub const DW_IC_DATA_CMD: c_uint = 0x10;
pub const DW_IC_SS_SCL_HCNT: c_uint = 0x14;
pub const DW_IC_SS_SCL_LCNT: c_uint = 0x18;
pub const DW_IC_FS_SCL_HCNT: c_uint = 0x1c;
pub const DW_IC_FS_SCL_LCNT: c_uint = 0x20;
pub const DW_IC_HS_SCL_HCNT: c_uint = 0x24;
pub const DW_IC_HS_SCL_LCNT: c_uint = 0x28;
pub const DW_IC_INTR_STAT: c_uint = 0x2c;
pub const DW_IC_INTR_MASK: c_uint = 0x30;
pub const DW_IC_RAW_INTR_STAT: c_uint = 0x34;
pub const DW_IC_RX_TL: c_uint = 0x38;
pub const DW_IC_TX_TL: c_uint = 0x3c;
pub const DW_IC_CLR_INTR: c_uint = 0x40;
pub const DW_IC_CLR_RX_UNDER: c_uint = 0x44;
pub const DW_IC_CLR_RX_OVER: c_uint = 0x48;
pub const DW_IC_CLR_TX_OVER: c_uint = 0x4c;
pub const DW_IC_CLR_RD_REQ: c_uint = 0x50;
pub const DW_IC_CLR_TX_ABRT: c_uint = 0x54;
pub const DW_IC_CLR_RX_DONE: c_uint = 0x58;
pub const DW_IC_CLR_ACTIVITY: c_uint = 0x5c;
pub const DW_IC_CLR_STOP_DET: c_uint = 0x60;
pub const DW_IC_CLR_START_DET: c_uint = 0x64;
pub const DW_IC_CLR_GEN_CALL: c_uint = 0x68;
pub const DW_IC_ENABLE: c_uint = 0x6c;
pub const DW_IC_STATUS: c_uint = 0x70;
pub const DW_IC_TXFLR: c_uint = 0x74;
pub const DW_IC_RXFLR: c_uint = 0x78;
pub const DW_IC_SDA_HOLD: c_uint = 0x7c;
pub const DW_IC_TX_ABRT_SOURCE: c_uint = 0x80;
pub const DW_IC_ENABLE_STATUS: c_uint = 0x9c;
pub const DW_IC_CLR_RESTART_DET: c_uint = 0xa8;
pub const DW_IC_SMBUS_INTR_STAT: c_uint = 0xc8;
pub const DW_IC_SMBUS_INTR_MASK: c_uint = 0xcc;
pub const DW_IC_CLR_SMBUS_INTR: c_uint = 0xd4;
pub const DW_IC_COMP_PARAM_1: c_uint = 0xf4;
pub const DW_IC_COMP_VERSION: c_uint = 0xf8;
pub const DW_IC_COMP_TYPE: c_uint = 0xfc;
// DW_IC_CON bits

// DW_IC_DATA_CMD bits

// DW_IC_INTR_* bits

// DW_IC_ENABLE bits

// DW_IC_STATUS bits

// DW_IC_SMBUS_INTR_* bits

