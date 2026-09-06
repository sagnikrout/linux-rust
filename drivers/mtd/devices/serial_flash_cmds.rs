//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/devices/serial_flash_cmds.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Generic/SFDP Flash Commands and Device Capabilities
//
// Copyright (C) 2013 Lee Jones <lee.jones@lianro.org>
//
// Generic Flash Commands/OPCODEs
pub const SPINOR_OP_WRVCR: c_uint = 0x81;
pub const SPINOR_OP_RDVCR: c_uint = 0x85;
// JEDEC Standard - Serial Flash Discoverable Parmeters (SFDP) Commands
pub const SPINOR_OP_WRITE: c_uint = 0x02	/* PAGE PROGRAM */;
pub const SPINOR_OP_WRITE_1_1_2: c_uint = 0xa2	/* DUAL INPUT PROGRAM */;
pub const SPINOR_OP_WRITE_1_2_2: c_uint = 0xd2	/* DUAL INPUT EXT PROGRAM */;
pub const SPINOR_OP_WRITE_1_1_4: c_uint = 0x32	/* QUAD INPUT PROGRAM */;
pub const SPINOR_OP_WRITE_1_4_4: c_uint = 0x12	/* QUAD INPUT EXT PROGRAM */;
// Configuration flags
pub const FLASH_FLAG_SINGLE: c_uint = 0x000000ff;
pub const FLASH_FLAG_READ_WRITE: c_uint = 0x00000001;
pub const FLASH_FLAG_READ_FAST: c_uint = 0x00000002;
pub const FLASH_FLAG_SE_4K: c_uint = 0x00000004;
pub const FLASH_FLAG_SE_32K: c_uint = 0x00000008;
pub const FLASH_FLAG_CE: c_uint = 0x00000010;
pub const FLASH_FLAG_32BIT_ADDR: c_uint = 0x00000020;
pub const FLASH_FLAG_RESET: c_uint = 0x00000040;
pub const FLASH_FLAG_DYB_LOCKING: c_uint = 0x00000080;
pub const FLASH_FLAG_DUAL: c_uint = 0x0000ff00;
pub const FLASH_FLAG_READ_1_1_2: c_uint = 0x00000100;
pub const FLASH_FLAG_READ_1_2_2: c_uint = 0x00000200;
pub const FLASH_FLAG_READ_2_2_2: c_uint = 0x00000400;
pub const FLASH_FLAG_WRITE_1_1_2: c_uint = 0x00001000;
pub const FLASH_FLAG_WRITE_1_2_2: c_uint = 0x00002000;
pub const FLASH_FLAG_WRITE_2_2_2: c_uint = 0x00004000;
pub const FLASH_FLAG_QUAD: c_uint = 0x00ff0000;
pub const FLASH_FLAG_READ_1_1_4: c_uint = 0x00010000;
pub const FLASH_FLAG_READ_1_4_4: c_uint = 0x00020000;
pub const FLASH_FLAG_READ_4_4_4: c_uint = 0x00040000;
pub const FLASH_FLAG_WRITE_1_1_4: c_uint = 0x00100000;
pub const FLASH_FLAG_WRITE_1_4_4: c_uint = 0x00200000;
pub const FLASH_FLAG_WRITE_4_4_4: c_uint = 0x00400000;
