//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/clk-pmc-atom.h
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
// Intel Atom platform clocks for BayTrail and CherryTrail SoC.
//
// Copyright (C) 2016, Intel Corporation
// Author: Irina Tirdea <irina.tirdea@intel.com>
//
// struct pmc_clk - PMC platform clock configuration
//
// @name:	identified, typically pmc_plt_clk_<x>, x=[0..5]
// @freq:	in Hz, 19.2MHz  and 25MHz (Baytrail only) supported
// @parent_name: one of 'xtal' or 'osc'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_clk {
    pub name: *const c_char,
    pub freq: c_ulong,
    pub parent_name: *const c_char,
}

//
// struct pmc_clk_data - common PMC clock configuration
//
// @base:	PMC clock register base offset
// @clks:	pointer to set of registered clocks, typically 0..5
// @critical:	flag to indicate if firmware enabled pmc_plt_clks
// should be marked as critial or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_clk_data {
    pub base: *mut void __iomem,
    pub clks: *const pmc_clk,
    pub critical: bool,
}
