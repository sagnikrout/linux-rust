//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/rcar-gen2-cpg.h
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
// R-Car Gen2 Clock Pulse Generator
//
// Copyright (C) 2016 Cogent Embedded Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcar_gen2_clk_types {
    CLK_TYPE_GEN2_MAIN = CLK_TYPE_CUSTOM,
    CLK_TYPE_GEN2_PLL0,
    CLK_TYPE_GEN2_PLL1,
    CLK_TYPE_GEN2_PLL3,
    CLK_TYPE_GEN2_Z,
    CLK_TYPE_GEN2_LB,
    CLK_TYPE_GEN2_ADSP,
    CLK_TYPE_GEN2_SDH,
    CLK_TYPE_GEN2_SD0,
    CLK_TYPE_GEN2_SD1,
    CLK_TYPE_GEN2_QSPI,
    CLK_TYPE_GEN2_RCAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen2_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll3_mult: u8,
    pub /: *mut *mut u8 pll0_mult; / leave as zero if PLL0CR exists,
}
