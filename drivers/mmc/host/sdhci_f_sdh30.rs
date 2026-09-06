//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci_f_sdh30.h
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
// Copyright (C) 2013 - 2015 Fujitsu Semiconductor, Ltd
// Vincent Yang <vincent.yang@tw.fujitsu.com>
// Copyright (C) 2015 Linaro Ltd  Andy Green <andy.green@linaro.org>
// Copyright (C) 2019 Socionext Inc.
//
// F_SDH30 extended Controller registers
pub const F_SDH30_AHB_CONFIG: c_uint = 0x100;

pub const F_SDH30_TUNING_SETTING: c_uint = 0x108;

pub const F_SDH30_IO_CONTROL2: c_uint = 0x114;

pub const F_SDH30_ESD_CONTROL: c_uint = 0x124;

pub const F_SDH30_TEST: c_uint = 0x158;

pub const F_SDH30_MIN_CLOCK: c_int = 400000;
