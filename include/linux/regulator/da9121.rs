//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/da9121.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// DA9121 Single-channel dual-phase 10A buck converter
// DA9130 Single-channel dual-phase 10A buck converter (Automotive)
// DA9217 Single-channel dual-phase  6A buck converter
// DA9122 Dual-channel single-phase  5A buck converter
// DA9131 Dual-channel single-phase  5A buck converter (Automotive)
// DA9220 Dual-channel single-phase  3A buck converter
// DA9132 Dual-channel single-phase  3A buck converter (Automotive)
//
// Copyright (C) 2020  Dialog Semiconductor
//
// Authors: Adam Ward, Dialog Semiconductor
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9121_pdata {
    pub num_buck: c_int,
    pub gpiod_ren: [*mut gpio_desc; DA9121_IDX_MAX],
    pub reg_node: [*mut device_node; DA9121_IDX_MAX],
    pub init_data: [*mut regulator_init_data; DA9121_IDX_MAX],
}
