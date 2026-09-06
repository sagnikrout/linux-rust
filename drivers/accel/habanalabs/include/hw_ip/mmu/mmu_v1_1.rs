//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/hw_ip/mmu/mmu_v1_1.h
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
// Copyright 2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const MMU_V1_1_HOP0_MASK: c_uint = 0x3000000000000ull;
pub const MMU_V1_1_HOP1_MASK: c_uint = 0x0FF8000000000ull;
pub const MMU_V1_1_HOP2_MASK: c_uint = 0x0007FC0000000ull;
pub const MMU_V1_1_HOP3_MASK: c_uint = 0x000003FE00000ull;
pub const MMU_V1_1_HOP4_MASK: c_uint = 0x00000001FF000ull;
pub const MMU_V1_1_HOP0_SHIFT: c_int = 48;
pub const MMU_V1_1_HOP1_SHIFT: c_int = 39;
pub const MMU_V1_1_HOP2_SHIFT: c_int = 30;
pub const MMU_V1_1_HOP3_SHIFT: c_int = 21;
pub const MMU_V1_1_HOP4_SHIFT: c_int = 12;
pub const MMU_ASID: c_uint = 0xC12004;
pub const MMU_HOP0_PA43_12: c_uint = 0xC12008;
pub const MMU_HOP0_PA49_44: c_uint = 0xC1200C;
pub const MMU_BUSY: c_uint = 0xC12000;
