//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mvebu/common.h
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
// Marvell EBU SoC common clock handling
//
// Copyright (C) 2012 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Andrew Lunn <andrew@lunn.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coreclk_ratio {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coreclk_soc_desc {
    pub sar): *mut *mut u32 (get_tclk_freq)(void __iomem,
    pub sar): *mut *mut u32 (get_cpu_freq)(void __iomem,
    pub div): *mut *mut *mut *mut void (get_clk_ratio)(void __iomem sar, int id, int mult, int,
    pub sar): *mut *mut u32 (get_refclk_freq)(void __iomem,
    pub sar): *mut *mut bool (is_sscg_enabled)(void __iomem,
    pub system_clk): *mut *mut u32 (fix_sscg_deviation)(u32,
    pub ratios: *const coreclk_ratio,
    pub num_ratios: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gating_soc_desc {
    pub name: *const c_char,
    pub parent: *const c_char,
    pub bit_idx: c_int,
    pub flags: c_ulong,
}

//
// This function is shared among the Kirkwood, Armada 370, Armada XP
// and Armada 375 SoC
//
extern "C" {
    pub fn kirkwood_fix_sscg_deviation(system_clk: u32) -> u32;
}
