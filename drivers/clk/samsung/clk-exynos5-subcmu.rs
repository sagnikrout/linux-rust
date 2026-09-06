//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/samsung/clk-exynos5-subcmu.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_subcmu_reg_dump {
    pub offset: u32,
    pub value: u32,
    pub mask: u32,
    pub save: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos5_subcmu_info {
    pub div_clks: *const samsung_div_clock,
    pub nr_div_clks: c_uint,
    pub gate_clks: *const samsung_gate_clock,
    pub nr_gate_clks: c_uint,
    pub suspend_regs: *mut exynos5_subcmu_reg_dump,
    pub nr_suspend_regs: c_uint,
    pub pd_name: *const c_char,
}
