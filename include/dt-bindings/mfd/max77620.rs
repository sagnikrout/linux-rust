//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/max77620.h
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
// This header provides macros for MAXIM MAX77620 device bindings.
//
// Copyright (c) 2016, NVIDIA Corporation.
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// MAX77620 interrupts

// FPS event source
pub const MAX77620_FPS_EVENT_SRC_EN0: c_int = 0;
pub const MAX77620_FPS_EVENT_SRC_EN1: c_int = 1;
pub const MAX77620_FPS_EVENT_SRC_SW: c_int = 2;
// Device state when FPS event LOW
pub const MAX77620_FPS_INACTIVE_STATE_SLEEP: c_int = 0;
pub const MAX77620_FPS_INACTIVE_STATE_LOW_POWER: c_int = 1;
// FPS source
pub const MAX77620_FPS_SRC_0: c_int = 0;
pub const MAX77620_FPS_SRC_1: c_int = 1;
pub const MAX77620_FPS_SRC_2: c_int = 2;
pub const MAX77620_FPS_SRC_NONE: c_int = 3;
pub const MAX77620_FPS_SRC_DEF: c_int = 4;
