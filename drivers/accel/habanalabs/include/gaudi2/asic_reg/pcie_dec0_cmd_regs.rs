//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_dec0_cmd_regs.h
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
// PCIE_DEC0_CMD
// (Prototype: VSI_CMD)
//
pub const mmPCIE_DEC0_CMD_SWREG0: c_uint = 0x4F00000;
pub const mmPCIE_DEC0_CMD_SWREG1: c_uint = 0x4F00004;
pub const mmPCIE_DEC0_CMD_SWREG2: c_uint = 0x4F00008;
pub const mmPCIE_DEC0_CMD_SWREG3: c_uint = 0x4F0000C;
pub const mmPCIE_DEC0_CMD_SWREG4: c_uint = 0x4F00010;
pub const mmPCIE_DEC0_CMD_SWREG5: c_uint = 0x4F00014;
pub const mmPCIE_DEC0_CMD_SWREG6: c_uint = 0x4F00018;
pub const mmPCIE_DEC0_CMD_SWREG7: c_uint = 0x4F0001C;
pub const mmPCIE_DEC0_CMD_SWREG8: c_uint = 0x4F00020;
pub const mmPCIE_DEC0_CMD_SWREG9: c_uint = 0x4F00024;
pub const mmPCIE_DEC0_CMD_SWREG10: c_uint = 0x4F00028;
pub const mmPCIE_DEC0_CMD_SWREG11: c_uint = 0x4F0002C;
pub const mmPCIE_DEC0_CMD_SWREG12: c_uint = 0x4F00030;
pub const mmPCIE_DEC0_CMD_SWREG13: c_uint = 0x4F00034;
pub const mmPCIE_DEC0_CMD_SWREG14: c_uint = 0x4F00038;
pub const mmPCIE_DEC0_CMD_SWREG15: c_uint = 0x4F0003C;
pub const mmPCIE_DEC0_CMD_SWREG16: c_uint = 0x4F00040;
pub const mmPCIE_DEC0_CMD_SWREG17: c_uint = 0x4F00044;
pub const mmPCIE_DEC0_CMD_SWREG18: c_uint = 0x4F00048;
pub const mmPCIE_DEC0_CMD_SWREG19: c_uint = 0x4F0004C;
pub const mmPCIE_DEC0_CMD_SWREG20: c_uint = 0x4F00050;
pub const mmPCIE_DEC0_CMD_SWREG21: c_uint = 0x4F00054;
pub const mmPCIE_DEC0_CMD_SWREG22: c_uint = 0x4F00058;
pub const mmPCIE_DEC0_CMD_SWREG23: c_uint = 0x4F0005C;
pub const mmPCIE_DEC0_CMD_SWREG24: c_uint = 0x4F00060;
pub const mmPCIE_DEC0_CMD_SWREG25: c_uint = 0x4F00064;
pub const mmPCIE_DEC0_CMD_SWREG26: c_uint = 0x4F00068;
pub const mmPCIE_DEC0_CMD_SWREG64: c_uint = 0x4F00100;
pub const mmPCIE_DEC0_CMD_SWREG65: c_uint = 0x4F00104;
pub const mmPCIE_DEC0_CMD_SWREG66: c_uint = 0x4F00108;
pub const mmPCIE_DEC0_CMD_SWREG67: c_uint = 0x4F0010C;
