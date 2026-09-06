//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/ultrarisc/clk-ultrarisc.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_pll_layout {
    pub cfg1_offset: u32,
    pub cfg2_offset: u32,
    pub frac_mask: u32,
    pub m_mask: u32,
    pub n_mask: u32,
    pub oddiv1_mask: u32,
    pub oddiv2_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_pll_desc {
    pub id: u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_fixed_factor_desc {
    pub id: u32,
    pub name: *const c_char,
    pub parent_id: u32,
    pub mult: u32,
    pub div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_divider_desc {
    pub id: u32,
    pub name: *const c_char,
    pub offset: u32,
    pub parent_id: u32,
    pub max_rate: c_ulong,
    pub load_mask: u32,
    pub div_shift: u8,
    pub div_width: u8,
    pub gate_bit: u8,
    pub divider_flags: u16,
    pub gate_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_gate_desc {
    pub id: u32,
    pub name: *const c_char,
    pub offset: u32,
    pub parent_id: u32,
    pub gate_bit: u8,
    pub gate_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ultrarisc_clk_soc_data {
    pub pll_layout: *const ultrarisc_pll_layout,
    pub plls: *const ultrarisc_pll_desc,
    pub num_plls: u32,
    pub fixed_factors: *const ultrarisc_fixed_factor_desc,
    pub num_fixed_factors: u32,
    pub dividers: *const ultrarisc_divider_desc,
    pub num_dividers: u32,
    pub gates: *const ultrarisc_gate_desc,
    pub num_gates: u32,
    pub num_clks: u32,
}
