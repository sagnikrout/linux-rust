//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/bcm/clk-iproc.h
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
// Copyright (C) 2014 Broadcom Corporation

pub const IPROC_CLK_NAME_LEN: c_int = 25;
pub const IPROC_CLK_INVALID_OFFSET: c_uint = 0xffffffff;

// clocks that should not be disabled at runtime

// PLL that requires gating through ASIU

// PLL that has fractional part of the NDIV

//
// Some of the iProc PLL/clocks may have an ASIC bug that requires read back
// of the same register following the write to flush the write transaction into
// the intended register
//

//
// Some PLLs require the PLL SW override bit to be set before changes can be
// applied to the PLL
//

//
// Some PLLs use a different way to control clock power, via the PWRDWN bit in
// the PLL control register
//

//
// Some PLLs have separate registers for Status and Control.  Identify this to
// let the driver know if additional registers need to be used
//

//
// Some PLLs have an additional divide by 2 in master clock calculation;
// MCLK = VCO_freq / (Mdiv * 2). Identify this to let the driver know
// of modified calculations
//

//
// Some PLLs provide a look up table for the leaf clock frequencies and
// auto calculates VCO frequency parameters based on the provided leaf
// clock frequencies. They have a user mode that allows the divider
// controls to be determined by the user
//

//
// Some PLLs have an active low reset
//

//
// Calculate the PLL parameters are runtime, instead of using table
//

//
// Parameters for VCO frequency configuration
//
// VCO frequency =
// ((ndiv_int + ndiv_frac / 2^20) * (ref frequency  / pdiv)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_vco_param {
    pub rate: c_ulong,
    pub ndiv_int: c_uint,
    pub ndiv_frac: c_uint,
    pub pdiv: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_clk_reg_op {
    pub offset: c_uint,
    pub shift: c_uint,
    pub width: c_uint,
}

//
// Clock gating control at the top ASIU level
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_asiu_gate {
    pub offset: c_uint,
    pub en_shift: c_uint,
}

//
// Control of powering on/off of a PLL
//
// Before powering off a PLL, input isolation (ISO) needs to be enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_aon_pwr_ctrl {
    pub offset: c_uint,
    pub pwr_width: c_uint,
    pub pwr_shift: c_uint,
    pub iso_shift: c_uint,
}

//
// Control of the PLL reset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_reset_ctrl {
    pub offset: c_uint,
    pub reset_shift: c_uint,
    pub p_reset_shift: c_uint,
}

//
// Control of the Ki, Kp, and Ka parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_dig_filter_ctrl {
    pub offset: c_uint,
    pub ki_shift: c_uint,
    pub ki_width: c_uint,
    pub kp_shift: c_uint,
    pub kp_width: c_uint,
    pub ka_shift: c_uint,
    pub ka_width: c_uint,
}

//
// To enable SW control of the PLL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_sw_ctrl {
    pub offset: c_uint,
    pub shift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_vco_ctrl {
    pub u_offset: c_uint,
    pub l_offset: c_uint,
}

//
// Main PLL control parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_pll_ctrl {
    pub flags: c_ulong,
    pub aon: iproc_pll_aon_pwr_ctrl,
    pub asiu: iproc_asiu_gate,
    pub reset: iproc_pll_reset_ctrl,
    pub dig_filter: iproc_pll_dig_filter_ctrl,
    pub sw_ctrl: iproc_pll_sw_ctrl,
    pub ndiv_int: iproc_clk_reg_op,
    pub ndiv_frac: iproc_clk_reg_op,
    pub pdiv: iproc_clk_reg_op,
    pub vco_ctrl: iproc_pll_vco_ctrl,
    pub status: iproc_clk_reg_op,
    pub macro_mode: iproc_clk_reg_op,
}

//
// Controls enabling/disabling a PLL derived clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_clk_enable_ctrl {
    pub offset: c_uint,
    pub enable_shift: c_uint,
    pub hold_shift: c_uint,
    pub bypass_shift: c_uint,
}

//
// Main clock control parameters for clocks derived from the PLLs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_clk_ctrl {
    pub channel: c_uint,
    pub flags: c_ulong,
    pub enable: iproc_clk_enable_ctrl,
    pub mdiv: iproc_clk_reg_op,
}

//
// Divisor of the ASIU clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_asiu_div {
    pub offset: c_uint,
    pub en_shift: c_uint,
    pub high_shift: c_uint,
    pub high_width: c_uint,
    pub low_shift: c_uint,
    pub low_width: c_uint,
}

extern "C" {
    pub fn iproc_armpll_setup(node: *mut device_node);
}
