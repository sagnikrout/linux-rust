//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/onenand/samsung.h
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
// Copyright (C) 2008-2010 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
//
// OneNAND Controller
//
pub const MEM_CFG_OFFSET: c_uint = 0x0000;
pub const BURST_LEN_OFFSET: c_uint = 0x0010;
pub const MEM_RESET_OFFSET: c_uint = 0x0020;
pub const INT_ERR_STAT_OFFSET: c_uint = 0x0030;
pub const INT_ERR_MASK_OFFSET: c_uint = 0x0040;
pub const INT_ERR_ACK_OFFSET: c_uint = 0x0050;
pub const ECC_ERR_STAT_OFFSET: c_uint = 0x0060;
pub const MANUFACT_ID_OFFSET: c_uint = 0x0070;
pub const DEVICE_ID_OFFSET: c_uint = 0x0080;
pub const DATA_BUF_SIZE_OFFSET: c_uint = 0x0090;
pub const BOOT_BUF_SIZE_OFFSET: c_uint = 0x00A0;
pub const BUF_AMOUNT_OFFSET: c_uint = 0x00B0;
pub const TECH_OFFSET: c_uint = 0x00C0;
pub const FBA_WIDTH_OFFSET: c_uint = 0x00D0;
pub const FPA_WIDTH_OFFSET: c_uint = 0x00E0;
pub const FSA_WIDTH_OFFSET: c_uint = 0x00F0;
pub const TRANS_SPARE_OFFSET: c_uint = 0x0140;
pub const DBS_DFS_WIDTH_OFFSET: c_uint = 0x0160;
pub const INT_PIN_ENABLE_OFFSET: c_uint = 0x01A0;
pub const ACC_CLOCK_OFFSET: c_uint = 0x01C0;
pub const FLASH_VER_ID_OFFSET: c_uint = 0x01F0;
pub const FLASH_AUX_CNTRL_OFFSET: c_uint = 0x0300		/* s3c64xx only */;
pub const ONENAND_MEM_RESET_HOT: c_uint = 0x3;
pub const ONENAND_MEM_RESET_COLD: c_uint = 0x2;
pub const ONENAND_MEM_RESET_WARM: c_uint = 0x1;

