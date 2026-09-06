//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bcma/scan.h
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
pub const BCMA_ADDR_BASE: c_uint = 0x18000000;
pub const BCMA_WRAP_BASE: c_uint = 0x18100000;
pub const SCAN_ER_VALID: c_uint = 0x00000001;
pub const SCAN_ER_TAGX: c_uint = 0x00000006 /* we have to ignore 0x8 bit when checking tag for SCAN_ER_TAG_ADDR */;
pub const SCAN_ER_TAG: c_uint = 0x0000000E;
pub const SCAN_ER_TAG_CI: c_uint = 0x00000000;
pub const SCAN_ER_TAG_MP: c_uint = 0x00000002;
pub const SCAN_ER_TAG_ADDR: c_uint = 0x00000004;
pub const SCAN_ER_TAG_END: c_uint = 0x0000000E;
pub const SCAN_ER_BAD: c_uint = 0xFFFFFFFF;
pub const SCAN_CIA_CLASS: c_uint = 0x000000F0;
pub const SCAN_CIA_CLASS_SHIFT: c_int = 4;
pub const SCAN_CIA_ID: c_uint = 0x000FFF00;
pub const SCAN_CIA_ID_SHIFT: c_int = 8;
pub const SCAN_CIA_MANUF: c_uint = 0xFFF00000;
pub const SCAN_CIA_MANUF_SHIFT: c_int = 20;
pub const SCAN_CIB_NMP: c_uint = 0x000001F0;
pub const SCAN_CIB_NMP_SHIFT: c_int = 4;
pub const SCAN_CIB_NSP: c_uint = 0x00003E00;
pub const SCAN_CIB_NSP_SHIFT: c_int = 9;
pub const SCAN_CIB_NMW: c_uint = 0x0007C000;
pub const SCAN_CIB_NMW_SHIFT: c_int = 14;
pub const SCAN_CIB_NSW: c_uint = 0x00F80000;
pub const SCAN_CIB_NSW_SHIFT: c_int = 19;
pub const SCAN_CIB_REV: c_uint = 0xFF000000;
pub const SCAN_CIB_REV_SHIFT: c_int = 24;
pub const SCAN_ADDR_AG32: c_uint = 0x00000008;
pub const SCAN_ADDR_SZ: c_uint = 0x00000030;
pub const SCAN_ADDR_SZ_SHIFT: c_int = 4;
pub const SCAN_ADDR_SZ_4K: c_uint = 0x00000000;
pub const SCAN_ADDR_SZ_8K: c_uint = 0x00000010;
pub const SCAN_ADDR_SZ_16K: c_uint = 0x00000020;
pub const SCAN_ADDR_SZ_SZD: c_uint = 0x00000030;
pub const SCAN_ADDR_TYPE: c_uint = 0x000000C0;
pub const SCAN_ADDR_TYPE_SLAVE: c_uint = 0x00000000;
pub const SCAN_ADDR_TYPE_BRIDGE: c_uint = 0x00000040;
pub const SCAN_ADDR_TYPE_SWRAP: c_uint = 0x00000080;
pub const SCAN_ADDR_TYPE_MWRAP: c_uint = 0x000000C0;
pub const SCAN_ADDR_PORT: c_uint = 0x00000F00;
pub const SCAN_ADDR_PORT_SHIFT: c_int = 8;
pub const SCAN_ADDR_ADDR: c_uint = 0xFFFFF000;
pub const SCAN_ADDR_SZ_BASE: c_uint = 0x00001000	/* 4KB */;
pub const SCAN_SIZE_SZ_ALIGN: c_uint = 0x00000FFF;
pub const SCAN_SIZE_SZ: c_uint = 0xFFFFF000;
pub const SCAN_SIZE_SG32: c_uint = 0x00000008;
