//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/platform/seco/seco-cec.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// SECO X86 Boards CEC register defines
//
// Author:  Ettore Chimenti <ek5.chimenti@gmail.com>
// Copyright (C) 2018, SECO Spa.
// Copyright (C) 2018, Aidilab Srl.
//
pub const SECOCEC_MAX_ADDRS: c_int = 1;

pub const SECOCEC_LATEST_FW: c_uint = 0x0f0b;
pub const SMBTIMEOUT: c_uint = 0xfff;
pub const SMB_POLL_UDELAY: c_int = 10;
pub const SMBUS_WRITE: c_int = 0;
pub const SMBUS_READ: c_int = 1;
pub const CMD_BYTE_DATA: c_int = 0;
pub const CMD_WORD_DATA: c_int = 1;
//
// SMBus definitons for Braswell
//

pub const BRA_SMB_BASE_ADDR: c_uint = 0x2040;

//
// Microcontroller Address
//
pub const SECOCEC_MICRO_ADDRESS: c_uint = 0x40;
//
// STM32 SMBus Registers
//
pub const SECOCEC_VERSION: c_uint = 0x00;
pub const SECOCEC_ENABLE_REG_1: c_uint = 0x01;
pub const SECOCEC_ENABLE_REG_2: c_uint = 0x02;
pub const SECOCEC_STATUS_REG_1: c_uint = 0x03;
pub const SECOCEC_STATUS_REG_2: c_uint = 0x04;
pub const SECOCEC_STATUS: c_uint = 0x28;
pub const SECOCEC_DEVICE_LA: c_uint = 0x29;
pub const SECOCEC_READ_OPERATION_ID: c_uint = 0x2a;
pub const SECOCEC_READ_DATA_LENGTH: c_uint = 0x2b;
pub const SECOCEC_READ_DATA_00: c_uint = 0x2c;
pub const SECOCEC_READ_DATA_02: c_uint = 0x2d;
pub const SECOCEC_READ_DATA_04: c_uint = 0x2e;
pub const SECOCEC_READ_DATA_06: c_uint = 0x2f;
pub const SECOCEC_READ_DATA_08: c_uint = 0x30;
pub const SECOCEC_READ_DATA_10: c_uint = 0x31;
pub const SECOCEC_READ_DATA_12: c_uint = 0x32;
pub const SECOCEC_READ_BYTE0: c_uint = 0x33;
pub const SECOCEC_WRITE_OPERATION_ID: c_uint = 0x34;
pub const SECOCEC_WRITE_DATA_LENGTH: c_uint = 0x35;
pub const SECOCEC_WRITE_DATA_00: c_uint = 0x36;
pub const SECOCEC_WRITE_DATA_02: c_uint = 0x37;
pub const SECOCEC_WRITE_DATA_04: c_uint = 0x38;
pub const SECOCEC_WRITE_DATA_06: c_uint = 0x39;
pub const SECOCEC_WRITE_DATA_08: c_uint = 0x3a;
pub const SECOCEC_WRITE_DATA_10: c_uint = 0x3b;
pub const SECOCEC_WRITE_DATA_12: c_uint = 0x3c;
pub const SECOCEC_WRITE_BYTE0: c_uint = 0x3d;
pub const SECOCEC_IR_READ_DATA: c_uint = 0x3e;
//
// IR
//
pub const SECOCEC_IR_COMMAND_MASK: c_uint = 0x007F;
pub const SECOCEC_IR_COMMAND_SHL: c_int = 0;
pub const SECOCEC_IR_ADDRESS_MASK: c_uint = 0x1F00;
pub const SECOCEC_IR_ADDRESS_SHL: c_int = 8;
pub const SECOCEC_IR_TOGGLE_MASK: c_uint = 0x8000;
pub const SECOCEC_IR_TOGGLE_SHL: c_int = 15;
//
// Enabling register
//
pub const SECOCEC_ENABLE_REG_1_CEC: c_uint = 0x1000;
pub const SECOCEC_ENABLE_REG_1_IR: c_uint = 0x2000;
pub const SECOCEC_ENABLE_REG_1_IR_PASSTHROUGH: c_uint = 0x4000;
//
// Status register
//

//
// Status data
//

