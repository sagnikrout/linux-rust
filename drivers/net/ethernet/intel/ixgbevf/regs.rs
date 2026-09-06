//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbevf/regs.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
pub const IXGBE_VFCTRL: c_uint = 0x00000;
pub const IXGBE_VFSTATUS: c_uint = 0x00008;
pub const IXGBE_VFLINKS: c_uint = 0x00010;
pub const IXGBE_VFFRTIMER: c_uint = 0x00048;
pub const IXGBE_VFRXMEMWRAP: c_uint = 0x03190;
pub const IXGBE_VTEICR: c_uint = 0x00100;
pub const IXGBE_VTEICS: c_uint = 0x00104;
pub const IXGBE_VTEIMS: c_uint = 0x00108;
pub const IXGBE_VTEIMC: c_uint = 0x0010C;
pub const IXGBE_VTEIAC: c_uint = 0x00110;
pub const IXGBE_VTEIAM: c_uint = 0x00114;

pub const IXGBE_VTIVAR_MISC: c_uint = 0x00140;

pub const IXGBE_VFPSRTYPE: c_uint = 0x00300;

pub const IXGBE_VFGPRC: c_uint = 0x0101C;
pub const IXGBE_VFGPTC: c_uint = 0x0201C;
pub const IXGBE_VFGORC_LSB: c_uint = 0x01020;
pub const IXGBE_VFGORC_MSB: c_uint = 0x01024;
pub const IXGBE_VFGOTC_LSB: c_uint = 0x02020;
pub const IXGBE_VFGOTC_MSB: c_uint = 0x02024;
pub const IXGBE_VFMPRC: c_uint = 0x01034;
pub const IXGBE_VFMRQC: c_uint = 0x3000;

// VFMRQC bits
pub const IXGBE_VFMRQC_RSSEN: c_uint = 0x00000001  /* RSS Enable */;
pub const IXGBE_VFMRQC_RSS_FIELD_IPV4_TCP: c_uint = 0x00010000;
pub const IXGBE_VFMRQC_RSS_FIELD_IPV4: c_uint = 0x00020000;
pub const IXGBE_VFMRQC_RSS_FIELD_IPV6: c_uint = 0x00100000;
pub const IXGBE_VFMRQC_RSS_FIELD_IPV6_TCP: c_uint = 0x00200000;

