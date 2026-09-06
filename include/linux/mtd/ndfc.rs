//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/ndfc.h
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
// Copyright (c) 2006 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
//
// Info:
// Contains defines, datastructures for ndfc nand controller
//
// NDFC Register definitions
pub const NDFC_CMD: c_uint = 0x00;
pub const NDFC_ALE: c_uint = 0x04;
pub const NDFC_DATA: c_uint = 0x08;
pub const NDFC_ECC: c_uint = 0x10;
pub const NDFC_BCFG0: c_uint = 0x30;
pub const NDFC_BCFG1: c_uint = 0x34;
pub const NDFC_BCFG2: c_uint = 0x38;
pub const NDFC_BCFG3: c_uint = 0x3c;
pub const NDFC_CCR: c_uint = 0x40;
pub const NDFC_STAT: c_uint = 0x44;
pub const NDFC_HWCTL: c_uint = 0x48;
pub const NDFC_REVID: c_uint = 0x50;
pub const NDFC_STAT_IS_READY: c_uint = 0x01000000;
pub const NDFC_CCR_RESET_CE: c_uint = 0x80000000 /* CE Reset */;
pub const NDFC_CCR_RESET_ECC: c_uint = 0x40000000 /* ECC Reset */;
pub const NDFC_CCR_RIE: c_uint = 0x20000000 /* Interrupt Enable on Device Rdy */;
pub const NDFC_CCR_REN: c_uint = 0x10000000 /* Enable wait for Rdy in LinearR */;
pub const NDFC_CCR_ROMEN: c_uint = 0x08000000 /* Enable ROM In LinearR */;
pub const NDFC_CCR_ARE: c_uint = 0x04000000 /* Auto-Read Enable */;

pub const NDFC_CCR_BS_MASK: c_uint = 0x03000000 /* Select Bank */;
pub const NDFC_CCR_ARAC0: c_uint = 0x00000000 /* 3 Addr, 1 Col 2 Row 512b page */;
pub const NDFC_CCR_ARAC1: c_uint = 0x00001000 /* 4 Addr, 1 Col 3 Row 512b page */;
pub const NDFC_CCR_ARAC2: c_uint = 0x00002000 /* 4 Addr, 2 Col 2 Row 2K page */;
pub const NDFC_CCR_ARAC3: c_uint = 0x00003000 /* 5 Addr, 2 Col 3 Row 2K page */;
pub const NDFC_CCR_ARAC_MASK: c_uint = 0x00003000 /* Auto-Read mode Addr Cycles */;
pub const NDFC_CCR_RPG: c_uint = 0x0000C000 /* Auto-Read Page */;
pub const NDFC_CCR_EBCC: c_uint = 0x00000004 /* EBC Configuration Completed */;
pub const NDFC_CCR_DHC: c_uint = 0x00000002 /* Direct Hardware Control Enable */;
pub const NDFC_BxCFG_EN: c_uint = 0x80000000 /* Bank Enable */;
pub const NDFC_BxCFG_CED: c_uint = 0x40000000 /* nCE Style */;
pub const NDFC_BxCFG_SZ_MASK: c_uint = 0x08000000 /* Bank Size */;
pub const NDFC_BxCFG_SZ_8BIT: c_uint = 0x00000000 /* 8bit */;
pub const NDFC_BxCFG_SZ_16BIT: c_uint = 0x08000000 /* 16bit */;
pub const NDFC_MAX_BANKS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndfc_controller_settings {
    pub ccr_settings: u32,
    pub ndfc_erpn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndfc_chip_settings {
    pub bank_settings: u32,
}
