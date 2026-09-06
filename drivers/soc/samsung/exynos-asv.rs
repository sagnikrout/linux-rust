//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/samsung/exynos-asv.h
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
// Copyright (c) 2019 Samsung Electronics Co., Ltd.
// http://www.samsung.com
// Author: Sylwester Nawrocki <s.nawrocki@samsung.com>
//
// Samsung Exynos SoC Adaptive Supply Voltage support
//
// HPM, IDS values to select target group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asv_limit_entry {
    pub hpm: c_uint,
    pub ids: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_asv_table {
    pub num_rows: c_uint,
    pub num_cols: c_uint,
    pub buf: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_asv_subsys {
    pub asv: *mut exynos_asv,
    pub cpu_dt_compat: *const c_char,
    pub id: c_int,
    pub table: exynos_asv_table,
    pub base_volt: c_uint,
    pub offset_volt_h: c_uint,
    pub offset_volt_l: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_asv {
    pub dev: *mut device,
    pub chipid_regmap: *mut regmap,
    pub subsys: [exynos_asv_subsys; 2],
    pub voltage): int level, unsigned int,
    pub group: c_uint,
    pub table: c_uint,
// True if SG fields from PKG_ID register should be used
    pub use_sg: bool,
// ASV bin read from DT
    pub of_bin: c_int,
}

extern "C" {
    pub fn __asv_get_table_entry(_arg: &subsys->table, _arg: level, 1: group +) -> return;
}
extern "C" {
    pub fn __asv_get_table_entry(_arg: &subsys->table, _arg: level, _arg: 0) -> return;
}
extern "C" {
    pub fn exynos_asv_init(dev: *mut device, regmap: *mut regmap) -> c_int;
}
