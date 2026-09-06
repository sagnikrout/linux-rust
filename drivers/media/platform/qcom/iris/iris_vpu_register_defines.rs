//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_vpu_register_defines.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const VCODEC_BASE_OFFS: c_uint = 0x00000000;
pub const AON_MVP_NOC_RESET: c_uint = 0x0001F000;
pub const CPU_BASE_OFFS: c_uint = 0x000A0000;
pub const WRAPPER_BASE_OFFS: c_uint = 0x000B0000;
pub const WRAPPER_TZ_BASE_OFFS: c_uint = 0x000C0000;
pub const AON_BASE_OFFS: c_uint = 0x000E0000;

pub const CPU_IC_SOFTINT_H2A_SHFT: c_uint = 0x0;

pub const CORE_CLK_RUN: c_uint = 0x0;

