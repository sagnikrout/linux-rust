//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/adau1373.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau1373_pll_src {
    ADAU1373_PLL_SRC_MCLK1 = 0,
    ADAU1373_PLL_SRC_BCLK1 = 1,
    ADAU1373_PLL_SRC_BCLK2 = 2,
    ADAU1373_PLL_SRC_BCLK3 = 3,
    ADAU1373_PLL_SRC_LRCLK1 = 4,
    ADAU1373_PLL_SRC_LRCLK2 = 5,
    ADAU1373_PLL_SRC_LRCLK3 = 6,
    ADAU1373_PLL_SRC_GPIO1 = 7,
    ADAU1373_PLL_SRC_GPIO2 = 8,
    ADAU1373_PLL_SRC_GPIO3 = 9,
    ADAU1373_PLL_SRC_GPIO4 = 10,
    ADAU1373_PLL_SRC_MCLK2 = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau1373_pll {
    ADAU1373_PLL1 = 0,
    ADAU1373_PLL2 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adau1373_clk_src {
    ADAU1373_CLK_SRC_PLL1 = 0,
    ADAU1373_CLK_SRC_PLL2 = 1,
}
