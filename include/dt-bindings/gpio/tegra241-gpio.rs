//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/tegra241-gpio.h
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
// Copyright (c) 2021, NVIDIA CORPORATION. All rights reserved.
//
// This header provides constants for the nvidia,tegra241-gpio DT binding.
//
// The first cell in Tegra's GPIO specifier is the GPIO ID. The macros below
// provide names for this.
//
// The second cell contains standard flag values specified in gpio.h.
//

// GPIOs implemented by main GPIO controller
pub const TEGRA241_MAIN_GPIO_PORT_A: c_int = 0;
pub const TEGRA241_MAIN_GPIO_PORT_B: c_int = 1;
pub const TEGRA241_MAIN_GPIO_PORT_C: c_int = 2;
pub const TEGRA241_MAIN_GPIO_PORT_D: c_int = 3;
pub const TEGRA241_MAIN_GPIO_PORT_E: c_int = 4;
pub const TEGRA241_MAIN_GPIO_PORT_F: c_int = 5;
pub const TEGRA241_MAIN_GPIO_PORT_G: c_int = 6;
pub const TEGRA241_MAIN_GPIO_PORT_H: c_int = 7;
pub const TEGRA241_MAIN_GPIO_PORT_I: c_int = 8;
pub const TEGRA241_MAIN_GPIO_PORT_J: c_int = 9;
pub const TEGRA241_MAIN_GPIO_PORT_K: c_int = 10;
pub const TEGRA241_MAIN_GPIO_PORT_L: c_int = 11;

// GPIOs implemented by AON GPIO controller
pub const TEGRA241_AON_GPIO_PORT_AA: c_int = 0;
pub const TEGRA241_AON_GPIO_PORT_BB: c_int = 1;

