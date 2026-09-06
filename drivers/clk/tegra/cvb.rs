//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/tegra/cvb.h
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
// Utility functions for parsing Tegra CVB voltage tables
//

pub const MAX_DVFS_FREQS: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rail_alignment {
    pub offset_uv: c_int,
    pub step_uv: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvb_coefficients {
    pub c0: c_int,
    pub c1: c_int,
    pub c2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvb_table_freq_entry {
    pub freq: c_ulong,
    pub coefficients: cvb_coefficients,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvb_cpu_dfll_data {
    pub tune0_low: u32,
    pub tune0_high: u32,
    pub tune1: u32,
    pub tune_high_min_millivolts: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvb_table {
    pub speedo_id: c_int,
    pub process_id: c_int,
    pub min_millivolts: c_int,
    pub max_millivolts: c_int,
    pub speedo_scale: c_int,
    pub voltage_scale: c_int,
    pub entries: [cvb_table_freq_entry; MAX_DVFS_FREQS],
    pub cpu_dfll_data: cvb_cpu_dfll_data,
}
