//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/gpmi-nand/bch-regs.h
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
pub const HW_BCH_CTRL: c_uint = 0x00000000;
pub const HW_BCH_CTRL_SET: c_uint = 0x00000004;
pub const HW_BCH_CTRL_CLR: c_uint = 0x00000008;
pub const HW_BCH_CTRL_TOG: c_uint = 0x0000000c;

pub const HW_BCH_STATUS0: c_uint = 0x00000010;
pub const HW_BCH_MODE: c_uint = 0x00000020;
pub const HW_BCH_ENCODEPTR: c_uint = 0x00000030;
pub const HW_BCH_DATAPTR: c_uint = 0x00000040;
pub const HW_BCH_METAPTR: c_uint = 0x00000050;
pub const HW_BCH_LAYOUTSELECT: c_uint = 0x00000070;
pub const HW_BCH_FLASH0LAYOUT0: c_uint = 0x00000080;
pub const BP_BCH_FLASH0LAYOUT0_NBLOCKS: c_int = 24;

pub const BP_BCH_FLASH0LAYOUT0_META_SIZE: c_int = 16;

pub const BP_BCH_FLASH0LAYOUT0_ECC0: c_int = 12;

pub const MX6Q_BP_BCH_FLASH0LAYOUT0_ECC0: c_int = 11;

pub const MX6Q_BP_BCH_FLASH0LAYOUT0_GF_13_14: c_int = 10;

pub const BP_BCH_FLASH0LAYOUT0_DATA0_SIZE: c_int = 0;

pub const HW_BCH_FLASH0LAYOUT1: c_uint = 0x00000090;
pub const BP_BCH_FLASH0LAYOUT1_PAGE_SIZE: c_int = 16;

pub const BP_BCH_FLASH0LAYOUT1_ECCN: c_int = 12;

pub const MX6Q_BP_BCH_FLASH0LAYOUT1_ECCN: c_int = 11;

pub const MX6Q_BP_BCH_FLASH0LAYOUT1_GF_13_14: c_int = 10;

pub const BP_BCH_FLASH0LAYOUT1_DATAN_SIZE: c_int = 0;

pub const HW_BCH_VERSION: c_uint = 0x00000160;
