//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mxl862xx/mxl862xx-cmd.h
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
pub const MXL862XX_MMD_DEV: c_int = 30;
pub const MXL862XX_MMD_REG_CTRL: c_int = 0;
pub const MXL862XX_MMD_REG_LEN_RET: c_int = 1;
pub const MXL862XX_MMD_REG_DATA_FIRST: c_int = 2;
pub const MXL862XX_MMD_REG_DATA_LAST: c_int = 95;

pub const MXL862XX_COMMON_MAGIC: c_uint = 0x100;
pub const MXL862XX_BRDG_MAGIC: c_uint = 0x300;
pub const MXL862XX_BRDGPORT_MAGIC: c_uint = 0x400;
pub const MXL862XX_CTP_MAGIC: c_uint = 0x500;
pub const MXL862XX_QOS_MAGIC: c_uint = 0x600;
pub const MXL862XX_RMON_MAGIC: c_uint = 0x700;
pub const MXL862XX_SWMAC_MAGIC: c_uint = 0xa00;
pub const MXL862XX_EXTVLAN_MAGIC: c_uint = 0xb00;
pub const MXL862XX_VLANFILTER_MAGIC: c_uint = 0xc00;
pub const MXL862XX_STP_MAGIC: c_uint = 0xf00;
pub const MXL862XX_SS_MAGIC: c_uint = 0x1600;
pub const GPY_GPY2XX_MAGIC: c_uint = 0x1800;
pub const SYS_MISC_MAGIC: c_uint = 0x1900;
pub const MXL862XX_XPCS_MAGIC: c_uint = 0x1a00;

pub const MMD_API_MAXIMUM_ID: c_uint = 0x7fff;
