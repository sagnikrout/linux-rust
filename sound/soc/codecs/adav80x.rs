//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/adav80x.h
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
// header file for ADAV80X parts
//
// Copyright 2011 Analog Devices Inc.
//

extern "C" {
    pub fn adav80x_bus_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adav80x_pll_src {
    ADAV80X_PLL_SRC_XIN,
    ADAV80X_PLL_SRC_XTAL,
    ADAV80X_PLL_SRC_MCLKI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adav80x_pll {
    ADAV80X_PLL1 = 0,
    ADAV80X_PLL2 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adav80x_clk_src {
    ADAV80X_CLK_XIN = 0,
    ADAV80X_CLK_MCLKI = 1,
    ADAV80X_CLK_PLL1 = 2,
    ADAV80X_CLK_PLL2 = 3,
    ADAV80X_CLK_XTAL = 6,

    ADAV80X_CLK_SYSCLK1 = 6,
    ADAV80X_CLK_SYSCLK2 = 7,
    ADAV80X_CLK_SYSCLK3 = 8,
}
