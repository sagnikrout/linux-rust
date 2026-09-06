//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/pinctrl-tegra.h
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
// This header provides constants for Tegra pinctrl bindings.
//
// Copyright (c) 2013, NVIDIA CORPORATION.  All rights reserved.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// Enable/disable for diffeent dt properties. This is applicable for
// properties nvidia,enable-input, nvidia,tristate, nvidia,open-drain,
// nvidia,lock, nvidia,rcv-sel, nvidia,high-speed-mode, nvidia,schmitt.
//
pub const TEGRA_PIN_DISABLE: c_int = 0;
pub const TEGRA_PIN_ENABLE: c_int = 1;
pub const TEGRA_PIN_PULL_NONE: c_int = 0;
pub const TEGRA_PIN_PULL_DOWN: c_int = 1;
pub const TEGRA_PIN_PULL_UP: c_int = 2;
// Low power mode driver
pub const TEGRA_PIN_LP_DRIVE_DIV_8: c_int = 0;
pub const TEGRA_PIN_LP_DRIVE_DIV_4: c_int = 1;
pub const TEGRA_PIN_LP_DRIVE_DIV_2: c_int = 2;
pub const TEGRA_PIN_LP_DRIVE_DIV_1: c_int = 3;
// Rising/Falling slew rate
pub const TEGRA_PIN_SLEW_RATE_FASTEST: c_int = 0;
pub const TEGRA_PIN_SLEW_RATE_FAST: c_int = 1;
pub const TEGRA_PIN_SLEW_RATE_SLOW: c_int = 2;
pub const TEGRA_PIN_SLEW_RATE_SLOWEST: c_int = 3;
