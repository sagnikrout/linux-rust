//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/pinctrl-tegra-io-pad.h
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
// pinctrl-tegra-io-pad.h: Tegra I/O pad source voltage configuration constants
// pinctrl bindings.
//
// Copyright (c) 2018, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Aapo Vienamo <avienamo@nvidia.com>
//
// Voltage levels of the I/O pad's source rail
pub const TEGRA_IO_PAD_VOLTAGE_1V8: c_int = 0;
pub const TEGRA_IO_PAD_VOLTAGE_3V3: c_int = 1;
