//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/cs48l32.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Device Tree defines for CS48L32 DSP.
//
// Copyright (C) 2016-2018, 2022, 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//
// Values for cirrus,in-type
pub const CS48L32_IN_TYPE_DIFF: c_int = 0;
pub const CS48L32_IN_TYPE_SE: c_int = 1;
// Values for cirrus,pdm-sup
pub const CS48L32_PDM_SUP_VOUT_MIC: c_int = 0;
pub const CS48L32_PDM_SUP_MICBIAS1: c_int = 1;
