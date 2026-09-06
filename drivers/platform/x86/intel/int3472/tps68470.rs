//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/int3472/tps68470.h
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
// TI TPS68470 PMIC platform data definition.
//
// Copyright (c) 2021 Red Hat Inc.
//
// Red Hat authors:
// Hans de Goede <hdegoede@redhat.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3472_tps68470_board_data {
    pub dev_name: *const c_char,
    pub tps68470_regulator_pdata: *const tps68470_regulator_platform_data,
    pub tps68470_gpio_swnode: *const software_node,
    pub n_gpiod_lookups: c_uint,
    pub tps68470_gpio_lookup_tables: [*mut gpiod_lookup_table; ],
}
