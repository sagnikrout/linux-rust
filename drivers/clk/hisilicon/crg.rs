//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/hisilicon/crg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HiSilicon Clock and Reset Driver Header
//
// Copyright (c) 2016 HiSilicon Limited.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_crg_funcs {
    pub pdev): *mut *mut *mut hisi_clock_data (register_clks)(platform_device,
    pub pdev): *mut *mut void (unregister_clks)(struct platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_crg_dev {
    pub clk_data: *mut hisi_clock_data,
    pub rstc: *mut hisi_reset_controller,
    pub funcs: *const hisi_crg_funcs,
}
