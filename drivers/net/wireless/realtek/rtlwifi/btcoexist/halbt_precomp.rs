//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/btcoexist/halbt_precomp.h
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
// Copyright(c) 2007-2011  Realtek Corporation.
//
// include files
//

// Interface type
pub const RT_PCI_INTERFACE: c_int = 1;
pub const RT_USB_INTERFACE: c_int = 2;
pub const RT_SDIO_INTERFACE: c_int = 3;

pub const BIT0: c_uint = 0x00000001;
pub const BIT1: c_uint = 0x00000002;
pub const BIT2: c_uint = 0x00000004;
pub const BIT3: c_uint = 0x00000008;
pub const BIT4: c_uint = 0x00000010;
pub const BIT5: c_uint = 0x00000020;
pub const BIT6: c_uint = 0x00000040;
pub const BIT7: c_uint = 0x00000080;
pub const BIT8: c_uint = 0x00000100;
pub const BIT9: c_uint = 0x00000200;
pub const BIT10: c_uint = 0x00000400;
pub const BIT11: c_uint = 0x00000800;
pub const BIT12: c_uint = 0x00001000;
pub const BIT13: c_uint = 0x00002000;
pub const BIT14: c_uint = 0x00004000;
pub const BIT15: c_uint = 0x00008000;
pub const BIT16: c_uint = 0x00010000;
pub const BIT17: c_uint = 0x00020000;
pub const BIT18: c_uint = 0x00040000;
pub const BIT19: c_uint = 0x00080000;
pub const BIT20: c_uint = 0x00100000;
pub const BIT21: c_uint = 0x00200000;
pub const BIT22: c_uint = 0x00400000;
pub const BIT23: c_uint = 0x00800000;
pub const BIT24: c_uint = 0x01000000;
pub const BIT25: c_uint = 0x02000000;
pub const BIT26: c_uint = 0x04000000;
pub const BIT27: c_uint = 0x08000000;
pub const BIT28: c_uint = 0x10000000;
pub const BIT29: c_uint = 0x20000000;
pub const BIT30: c_uint = 0x40000000;
pub const BIT31: c_uint = 0x80000000;
