//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/hw_ip/mmu/mmu_v2_0.h
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
// Copyright 2019 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const HOP0_MASK_4K: c_uint = 0xFE00000000000000ull;
pub const HOP1_MASK_4K: c_uint = 0x01FF000000000000ull;
pub const HOP2_MASK_4K: c_uint = 0x0000FF8000000000ull;
pub const HOP3_MASK_4K: c_uint = 0x0000007FC0000000ull;
pub const HOP4_MASK_4K: c_uint = 0x000000003FE00000ull;
pub const HOP5_MASK_4K: c_uint = 0x00000000001FF000ull;
pub const HOP0_MASK_64K: c_uint = 0xFF00000000000000ull;
pub const HOP1_MASK_64K: c_uint = 0x00FF000000000000ull;
pub const HOP2_MASK_64K: c_uint = 0x0000FF0000000000ull;
pub const HOP3_MASK_64K: c_uint = 0x000000FF00000000ull;
pub const HOP4_MASK_64K: c_uint = 0x00000000FF000000ull;
pub const HOP5_MASK_64K: c_uint = 0x0000000000FF0000ull;
pub const HOP0_SHIFT_4K: c_int = 57;
pub const HOP1_SHIFT_4K: c_int = 48;
pub const HOP2_SHIFT_4K: c_int = 39;
pub const HOP3_SHIFT_4K: c_int = 30;
pub const HOP4_SHIFT_4K: c_int = 21;
pub const HOP5_SHIFT_4K: c_int = 12;
pub const HOP0_SHIFT_64K: c_int = 56;
pub const HOP1_SHIFT_64K: c_int = 48;
pub const HOP2_SHIFT_64K: c_int = 40;
pub const HOP3_SHIFT_64K: c_int = 32;
pub const HOP4_SHIFT_64K: c_int = 24;
pub const HOP5_SHIFT_64K: c_int = 16;

pub const DHOP4_MASK: c_uint = 0x000003C000000ull;

pub const DHOP4_SHIFT: c_int = 26;
