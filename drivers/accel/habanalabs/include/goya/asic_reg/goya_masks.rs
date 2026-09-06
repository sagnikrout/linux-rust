//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/goya_masks.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//

// Useful masks for bits in various registers

// RESETS

pub const GOYA_IRQ_HBW_ID_MASK: c_uint = 0x1FFF;
pub const GOYA_IRQ_HBW_ID_SHIFT: c_int = 0;
pub const GOYA_IRQ_HBW_INTERNAL_ID_MASK: c_uint = 0xE000;
pub const GOYA_IRQ_HBW_INTERNAL_ID_SHIFT: c_int = 13;
pub const GOYA_IRQ_HBW_AGENT_ID_MASK: c_uint = 0x1F0000;
pub const GOYA_IRQ_HBW_AGENT_ID_SHIFT: c_int = 16;
pub const GOYA_IRQ_HBW_Y_MASK: c_uint = 0xE00000;
pub const GOYA_IRQ_HBW_Y_SHIFT: c_int = 21;
pub const GOYA_IRQ_HBW_X_MASK: c_uint = 0x7000000;
pub const GOYA_IRQ_HBW_X_SHIFT: c_int = 24;
pub const GOYA_IRQ_LBW_ID_MASK: c_uint = 0xFF;
pub const GOYA_IRQ_LBW_ID_SHIFT: c_int = 0;
pub const GOYA_IRQ_LBW_INTERNAL_ID_MASK: c_uint = 0x700;
pub const GOYA_IRQ_LBW_INTERNAL_ID_SHIFT: c_int = 8;
pub const GOYA_IRQ_LBW_AGENT_ID_MASK: c_uint = 0xF800;
pub const GOYA_IRQ_LBW_AGENT_ID_SHIFT: c_int = 11;
pub const GOYA_IRQ_LBW_Y_MASK: c_uint = 0x70000;
pub const GOYA_IRQ_LBW_Y_SHIFT: c_int = 16;
pub const GOYA_IRQ_LBW_X_MASK: c_uint = 0x380000;
pub const GOYA_IRQ_LBW_X_SHIFT: c_int = 19;

pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_SHIFT: c_int = 1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT0_MASK: c_uint = 0x1;
pub const PSOC_ETR_AXICTL_PROTCTRLBIT1_MASK: c_uint = 0x2;
pub const PSOC_ETR_AXICTL_WRBURSTLEN_MASK: c_uint = 0xF00;
