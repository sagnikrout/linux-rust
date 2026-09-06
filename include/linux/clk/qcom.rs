//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/qcom.h
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
// Copyright (c) 2026, Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// struct qcom_clk_ref_desc - descriptor for a clkref_en gate clock
// @name: clock name exposed to the common clock framework
// @offset: clkref_en register offset from the block base
// @regulator_names: optional supply names enabled while preparing the clock
// @num_regulators: number of entries in @regulator_names
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_clk_ref_desc {
    pub name: *const c_char,
    pub offset: u32,
    pub regulator_names: *const *const c_char,
    pub num_regulators: c_uint,
}

//
// struct qcom_clk_ref - per-clock data for a clkref_en gate clock
// @hw: common clock framework hardware clock handle
// @regmap: register map backing the clkref_en register
// @desc: clock descriptor copied at registration time
// @regulators: optional bulk regulator handles for @desc.regulator_names
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_clk_ref {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub desc: qcom_clk_ref_desc,
    pub regulators: *mut regulator_bulk_data,
}

