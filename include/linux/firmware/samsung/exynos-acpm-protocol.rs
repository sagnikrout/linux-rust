//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/samsung/exynos-acpm-protocol.h
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
// Copyright 2020 Samsung Electronics Co., Ltd.
// Copyright 2020 Google LLC.
// Copyright 2024 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_dvfs_ops {
    pub rate): unsigned int clk_id, unsigned long,
    pub clk_id): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_pmic_ops {
    pub buf): *mut u8 type, u8 reg, u8 chan, u8,
    pub buf): *mut u8 type, u8 reg, u8 chan, u8 count, u8,
    pub value): u8 type, u8 reg, u8 chan, u8,
    pub buf): *const u8 type, u8 reg, u8 chan, u8 count, u8,
    pub mask): u8 type, u8 reg, u8 chan, u8 value, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_tmu_ops {
    pub acpm_chan_id): *mut *mut *mut int (init)(struct acpm_handle handle, unsigned int,
    pub temp): *mut u8 tz, int,
    pub tlen): u8 temperature[8], size_t,
    pub inten): unsigned int acpm_chan_id, u8 tz, u8,
    pub enable): u8 tz, bool,
    pub tz): unsigned int acpm_chan_id, u8,
    pub acpm_chan_id): *mut *mut *mut int (suspend)(struct acpm_handle handle, unsigned int,
    pub acpm_chan_id): *mut *mut *mut int (resume)(struct acpm_handle handle, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_ops {
    pub dvfs: acpm_dvfs_ops,
    pub pmic: acpm_pmic_ops,
    pub tmu: acpm_tmu_ops,
}

//
// struct acpm_handle - Reference to an initialized protocol instance
// @ops:	pointer to the constant ACPM protocol operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpm_handle {
    pub ops: *const acpm_ops,
}
