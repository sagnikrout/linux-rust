//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_resources.h
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
extern "C" {
    pub fn iris_opp_set_rate(dev: *mut device, freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn iris_enable_power_domains(core: *mut iris_core, pd_dev: *mut device) -> c_int;
}
extern "C" {
    pub fn iris_disable_power_domains(core: *mut iris_core, pd_dev: *mut device) -> c_int;
}
extern "C" {
    pub fn iris_unset_icc_bw(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_set_icc_bw(core: *mut iris_core, icc_bw: c_ulong) -> c_int;
}
extern "C" {
    pub fn iris_disable_unprepare_clock(core: *mut iris_core, clk_type: platform_clk_type) -> c_int;
}
extern "C" {
    pub fn iris_prepare_enable_clock(core: *mut iris_core, clk_type: platform_clk_type) -> c_int;
}
