//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77714.h
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
// Maxim MAX77714 Register and data structures definition.
//
// Copyright (C) 2022 Luca Ceresoli
// Author: Luca Ceresoli <luca.ceresoli@bootlin.com>
//

pub const MAX77714_INT_TOP: c_uint = 0x00;
pub const MAX77714_INT_TOPM: c_uint = 0x07 /* Datasheet says "read only", but it is RW */;

pub const MAX77714_32K_STATUS: c_uint = 0x30;

pub const MAX77714_32K_STATUS_32KLOAD_MSK: c_uint = 0x3;
pub const MAX77714_32K_STATUS_32KLOAD_SHF: c_int = 1;

pub const MAX77714_32K_CONFIG: c_uint = 0x31;

pub const MAX77714_CNFG_GLBL2: c_uint = 0x91;

pub const MAX77714_TWD_MASK: c_uint = 0x3;
pub const MAX77714_TWD_2s: c_uint = 0x0;
pub const MAX77714_TWD_16s: c_uint = 0x1;
pub const MAX77714_TWD_64s: c_uint = 0x2;
pub const MAX77714_TWD_128s: c_uint = 0x3;
pub const MAX77714_CNFG_GLBL3: c_uint = 0x92;

pub const MAX77714_CNFG2_ONOFF: c_uint = 0x94;

// Interrupts
