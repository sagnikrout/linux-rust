//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/spear/clk.h
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
// Clock framework definitions for SPEAr platform
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//

// Auxiliary Synth clk
// Default masks
pub const AUX_EQ_SEL_SHIFT: c_int = 30;
pub const AUX_EQ_SEL_MASK: c_int = 1;
pub const AUX_EQ1_SEL: c_int = 0;
pub const AUX_EQ2_SEL: c_int = 1;
pub const AUX_XSCALE_SHIFT: c_int = 16;
pub const AUX_XSCALE_MASK: c_uint = 0xFFF;
pub const AUX_YSCALE_SHIFT: c_int = 0;
pub const AUX_YSCALE_MASK: c_uint = 0xFFF;
pub const AUX_SYNT_ENB: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_clk_masks {
    pub eq_sel_mask: u32,
    pub eq_sel_shift: u32,
    pub eq1_mask: u32,
    pub eq2_mask: u32,
    pub xscale_sel_mask: u32,
    pub xscale_sel_shift: u32,
    pub yscale_sel_mask: u32,
    pub yscale_sel_shift: u32,
    pub enable_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_rate_tbl {
    pub xscale: u16,
    pub yscale: u16,
    pub eq: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_aux {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub masks: *const aux_clk_masks,
    pub rtbl: *mut aux_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

// Fractional Synth clk
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frac_rate_tbl {
    pub div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_frac {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub rtbl: *mut frac_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

// GPT clk
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpt_rate_tbl {
    pub mscale: u16,
    pub nscale: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gpt {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub rtbl: *mut gpt_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

// VCO-PLL clk
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_rate_tbl {
    pub mode: u8,
    pub m: u16,
    pub n: u8,
    pub p: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_vco {
    pub hw: clk_hw,
    pub mode_reg: *mut void __iomem,
    pub cfg_reg: *mut void __iomem,
    pub rtbl: *mut pll_rate_tbl,
    pub rtbl_cnt: u8,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub vco: *mut clk_vco,
    pub parent: [*const c_char; 1],
    pub lock: *mut spinlock_t,
}

// clk register routines
// cfg_reg, struct pll_rate_tbl *rtbl, u8 rtbl_cnt,
