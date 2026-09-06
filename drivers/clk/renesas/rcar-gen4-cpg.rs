//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/rcar-gen4-cpg.h
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
// R-Car Gen4 Clock Pulse Generator
//
// Copyright (C) 2021 Renesas Electronics Corp.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcar_gen4_clk_types {
    CLK_TYPE_GEN4_MAIN = CLK_TYPE_CUSTOM,
    CLK_TYPE_GEN4_PLL1,
    CLK_TYPE_GEN4_PLL2X_3X,	/* r8a779a0 only */
    CLK_TYPE_GEN4_PLL5,
    CLK_TYPE_GEN4_PLL_F8_25,	/* Fixed fractional 8.25 PLL */
    CLK_TYPE_GEN4_PLL_V8_25,	/* Variable fractional 8.25 PLL */
    CLK_TYPE_GEN4_PLL_F9_24,	/* Fixed fractional 9.24 PLL */
    CLK_TYPE_GEN4_PLL_V9_24,	/* Variable fractional 9.24 PLL */
    CLK_TYPE_GEN4_SDSRC,
    CLK_TYPE_GEN4_SDH,
    CLK_TYPE_GEN4_SD,
    CLK_TYPE_GEN4_MDSEL,	/* Select parent/divider using mode pin */
    CLK_TYPE_GEN4_Z,
    CLK_TYPE_GEN4_OSC,	/* OSC EXTAL predivider and fixed divider */
    CLK_TYPE_GEN4_RPCSRC,
    CLK_TYPE_GEN4_RPC,
    CLK_TYPE_GEN4_RPCD2,

// SoC specific definitions start here
    CLK_TYPE_GEN4_SOC_BASE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen4_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll1_div: u8,
    pub pll5_mult: u8,
    pub pll5_div: u8,
    pub osc_prediv: u8,
}

pub const CPG_SD0CKCR: c_uint = 0x870	/* SD-IF0 Clock Frequency Control Register */;
pub const CPG_CANFDCKCR: c_uint = 0x878	/* CAN-FD Clock Frequency Control Register */;
pub const CPG_MSOCKCR: c_uint = 0x87c	/* MSIOF Clock Frequency Control Register */;
pub const CPG_CSICKCR: c_uint = 0x880	/* CSI Clock Frequency Control Register */;
pub const CPG_DSIEXTCKCR: c_uint = 0x884	/* DSI Clock Frequency Control Register */;
