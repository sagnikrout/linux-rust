//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/rcar-gen3-cpg.h
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
// R-Car Gen3 Clock Pulse Generator
//
// Copyright (C) 2015-2018 Glider bvba
// Copyright (C) 2018 Renesas Electronics Corp.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcar_gen3_clk_types {
    CLK_TYPE_GEN3_MAIN = CLK_TYPE_CUSTOM,
    CLK_TYPE_GEN3_PLL0,
    CLK_TYPE_GEN3_PLL1,
    CLK_TYPE_GEN3_PLL2,
    CLK_TYPE_GEN3_PLL3,
    CLK_TYPE_GEN3_PLL4,
    CLK_TYPE_GEN3_SDH,
    CLK_TYPE_GEN3_SD,
    CLK_TYPE_GEN3_R,
    CLK_TYPE_GEN3_MDSEL,	/* Select parent/divider using mode pin */
    CLK_TYPE_GEN3_Z,
    CLK_TYPE_GEN3_ZG,
    CLK_TYPE_GEN3_OSC,	/* OSC EXTAL predivider and fixed divider */
    CLK_TYPE_GEN3_RCKSEL,	/* Select parent/divider using RCKCR.CKSEL */
    CLK_TYPE_GEN3_RPCSRC,
    CLK_TYPE_GEN3_E3_RPCSRC,/* Select parent/divider using RPCCKCR.DIV */
    CLK_TYPE_GEN3_RPC,
    CLK_TYPE_GEN3_RPCD2,

// SoC specific definitions start here
    CLK_TYPE_GEN3_SOC_BASE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen3_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll1_div: u8,
    pub pll3_mult: u8,
    pub pll3_div: u8,
    pub osc_prediv: u8,
}

pub const CPG_RPCCKCR: c_uint = 0x238;
pub const CPG_RCKCR: c_uint = 0x240;
