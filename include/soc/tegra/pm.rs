//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/pm.h
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
// Copyright (C) 2014 NVIDIA Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_suspend_mode {
    TEGRA_SUSPEND_NONE = 0,
    TEGRA_SUSPEND_LP2, /* CPU voltage off */
    TEGRA_SUSPEND_LP1, /* CPU voltage off, DRAM self-refresh */
    TEGRA_SUSPEND_LP0, /* CPU + core voltage off, DRAM self-refresh */
    TEGRA_MAX_SUSPEND_MODE,
    TEGRA_SUSPEND_NOT_READY,
}

// low-level resume entry point
extern "C" {
    pub fn tegra_resume();
}
extern "C" {
    pub fn tegra30_pm_secondary_cpu_suspend(arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn tegra_pm_clear_cpu_in_lp2();
}
extern "C" {
    pub fn tegra_pm_set_cpu_in_lp2();
}
extern "C" {
    pub fn tegra_pm_enter_lp2() -> c_int;
}
extern "C" {
    pub fn tegra_pm_park_secondary_cpu(cpu: c_ulong) -> c_int;
}
extern "C" {
    pub fn tegra_pm_init_suspend();
}

