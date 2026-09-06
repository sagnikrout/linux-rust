//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_dec0_cmd_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DCORE0_DEC0_CMD
// (Prototype: VSI_CMD)
//
pub const mmDCORE0_DEC0_CMD_SWREG0: c_uint = 0x41E0000;
pub const mmDCORE0_DEC0_CMD_SWREG1: c_uint = 0x41E0004;
pub const mmDCORE0_DEC0_CMD_SWREG2: c_uint = 0x41E0008;
pub const mmDCORE0_DEC0_CMD_SWREG3: c_uint = 0x41E000C;
pub const mmDCORE0_DEC0_CMD_SWREG4: c_uint = 0x41E0010;
pub const mmDCORE0_DEC0_CMD_SWREG5: c_uint = 0x41E0014;
pub const mmDCORE0_DEC0_CMD_SWREG6: c_uint = 0x41E0018;
pub const mmDCORE0_DEC0_CMD_SWREG7: c_uint = 0x41E001C;
pub const mmDCORE0_DEC0_CMD_SWREG8: c_uint = 0x41E0020;
pub const mmDCORE0_DEC0_CMD_SWREG9: c_uint = 0x41E0024;
pub const mmDCORE0_DEC0_CMD_SWREG10: c_uint = 0x41E0028;
pub const mmDCORE0_DEC0_CMD_SWREG11: c_uint = 0x41E002C;
pub const mmDCORE0_DEC0_CMD_SWREG12: c_uint = 0x41E0030;
pub const mmDCORE0_DEC0_CMD_SWREG13: c_uint = 0x41E0034;
pub const mmDCORE0_DEC0_CMD_SWREG14: c_uint = 0x41E0038;
pub const mmDCORE0_DEC0_CMD_SWREG15: c_uint = 0x41E003C;
pub const mmDCORE0_DEC0_CMD_SWREG16: c_uint = 0x41E0040;
pub const mmDCORE0_DEC0_CMD_SWREG17: c_uint = 0x41E0044;
pub const mmDCORE0_DEC0_CMD_SWREG18: c_uint = 0x41E0048;
pub const mmDCORE0_DEC0_CMD_SWREG19: c_uint = 0x41E004C;
pub const mmDCORE0_DEC0_CMD_SWREG20: c_uint = 0x41E0050;
pub const mmDCORE0_DEC0_CMD_SWREG21: c_uint = 0x41E0054;
pub const mmDCORE0_DEC0_CMD_SWREG22: c_uint = 0x41E0058;
pub const mmDCORE0_DEC0_CMD_SWREG23: c_uint = 0x41E005C;
pub const mmDCORE0_DEC0_CMD_SWREG24: c_uint = 0x41E0060;
pub const mmDCORE0_DEC0_CMD_SWREG25: c_uint = 0x41E0064;
pub const mmDCORE0_DEC0_CMD_SWREG26: c_uint = 0x41E0068;
pub const mmDCORE0_DEC0_CMD_SWREG64: c_uint = 0x41E0100;
pub const mmDCORE0_DEC0_CMD_SWREG65: c_uint = 0x41E0104;
pub const mmDCORE0_DEC0_CMD_SWREG66: c_uint = 0x41E0108;
pub const mmDCORE0_DEC0_CMD_SWREG67: c_uint = 0x41E010C;
