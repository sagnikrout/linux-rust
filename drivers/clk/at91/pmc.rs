//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/at91/pmc.h
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
// drivers/clk/at91/pmc.h
//
// Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_data {
    pub ncore: c_uint,
    pub chws: *mut clk_hw,
    pub nsystem: c_uint,
    pub shws: *mut clk_hw,
    pub nperiph: c_uint,
    pub phws: *mut clk_hw,
    pub ngck: c_uint,
    pub ghws: *mut clk_hw,
    pub npck: c_uint,
    pub pchws: *mut clk_hw,
    pub hwtable: [*mut clk_hw; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_range {
    pub min: c_ulong,
    pub max: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_master_layout {
    pub offset: u32,
    pub mask: u32,
    pub pres_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_master_characteristics {
    pub output: clk_range,
    pub divisors: [u32; 5],
    pub have_div3_pres: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll_layout {
    pub pllr_mask: u32,
    pub mul_mask: u32,
    pub frac_mask: u32,
    pub div_mask: u32,
    pub endiv_mask: u32,
    pub mul_shift: u8,
    pub frac_shift: u8,
    pub div_shift: u8,
    pub endiv_shift: u8,
    pub div2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll_characteristics {
    pub input: clk_range,
    pub num_output: c_int,
    pub output: *const clk_range,
    pub core_output: *const clk_range,
    pub icpll: *mut u16,
    pub out: *mut u8,
    pub 1: u8 upll :,
    pub acr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_programmable_layout {
    pub pres_mask: u8,
    pub pres_shift: u8,
    pub css_mask: u8,
    pub have_slck_mck: u8,
    pub is_pres_direct: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pcr_layout {
    pub offset: u32,
    pub cmd: u32,
    pub div_mask: u32,
    pub gckcss_mask: u32,
    pub pid_mask: u32,
}

//
// struct at91_clk_pms - Power management state for AT91 clock
// @rate: clock rate
// @parent_rate: clock parent rate
// @status: clock status (enabled or disabled)
// @parent: clock parent index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_clk_pms {
    pub rate: c_ulong,
    pub parent_rate: c_ulong,
    pub status: c_uint,
    pub parent: c_uint,
}

