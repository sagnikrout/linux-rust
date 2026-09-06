//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mv88e6xxx/smi.h
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
// Marvell 88E6xxx System Management Interface (SMI) support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2019 Vivien Didelot <vivien.didelot@gmail.com>
//

// Offset 0x00: SMI Command Register
pub const MV88E6XXX_SMI_CMD: c_uint = 0x00;
pub const MV88E6XXX_SMI_CMD_BUSY: c_uint = 0x8000;
pub const MV88E6XXX_SMI_CMD_MODE_MASK: c_uint = 0x1000;
pub const MV88E6XXX_SMI_CMD_MODE_45: c_uint = 0x0000;
pub const MV88E6XXX_SMI_CMD_MODE_22: c_uint = 0x1000;
pub const MV88E6XXX_SMI_CMD_OP_MASK: c_uint = 0x0c00;
pub const MV88E6XXX_SMI_CMD_OP_22_WRITE: c_uint = 0x0400;
pub const MV88E6XXX_SMI_CMD_OP_22_READ: c_uint = 0x0800;
pub const MV88E6XXX_SMI_CMD_OP_45_WRITE_ADDR: c_uint = 0x0000;
pub const MV88E6XXX_SMI_CMD_OP_45_WRITE_DATA: c_uint = 0x0400;
pub const MV88E6XXX_SMI_CMD_OP_45_READ_DATA: c_uint = 0x0800;
pub const MV88E6XXX_SMI_CMD_OP_45_READ_DATA_INC: c_uint = 0x0c00;
pub const MV88E6XXX_SMI_CMD_DEV_ADDR_MASK: c_uint = 0x003e;
pub const MV88E6XXX_SMI_CMD_REG_ADDR_MASK: c_uint = 0x001f;
// Offset 0x01: SMI Data Register
pub const MV88E6XXX_SMI_DATA: c_uint = 0x01;
