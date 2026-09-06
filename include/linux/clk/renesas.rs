//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/renesas.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2013 Ideas On Board SPRL
// Copyright 2013, 2014 Horms Solutions Ltd.
//
// Contact: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Contact: Simon Horman <horms@verge.net.au>
//

extern "C" {
    pub fn cpg_mstp_add_clk_domain(np: *mut device_node);
}

extern "C" {
    pub fn cpg_mstp_attach_dev(unused: *mut generic_pm_domain, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cpg_mstp_detach_dev(unused: *mut generic_pm_domain, dev: *mut device);
}

extern "C" {
    pub fn cpg_mssr_attach_dev(unused: *mut generic_pm_domain, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cpg_mssr_detach_dev(unused: *mut generic_pm_domain, dev: *mut device);
}

extern "C" {
    pub fn rzg2l_cpg_dsi_div_set_divider(divider: u8, target: c_int);
}

//
// struct rzv2h_pll_limits - PLL parameter constraints
//
// This structure defines the minimum and maximum allowed values for
// various parameters used to configure a PLL. These limits ensure
// the PLL operates within valid and stable ranges.
//
// @input_fref: Reference input frequency to the PLL (in Hz). If set
// to 0, a default value of 24MHz is used.
//
// @fout: Output frequency range (in MHz)
// @fout.min: Minimum allowed output frequency
// @fout.max: Maximum allowed output frequency
//
// @fvco: PLL oscillation frequency range (in MHz)
// @fvco.min: Minimum allowed VCO frequency
// @fvco.max: Maximum allowed VCO frequency
//
// @m: Main-divider range
// @m.min: Minimum main-divider value
// @m.max: Maximum main-divider value
//
// @p: Pre-divider range
// @p.min: Minimum pre-divider value
// @p.max: Maximum pre-divider value
//
// @s: Divider range
// @s.min: Minimum divider value
// @s.max: Maximum divider value
//
// @k: Delta-sigma modulator range (signed)
// @k.min: Minimum delta-sigma value
// @k.max: Maximum delta-sigma value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_pll_limits {
    pub input_fref: u32,
    pub min: u32,
    pub max: u32,
    pub fout: },
    pub min: u32,
    pub max: u32,
    pub fvco: },
    pub min: u16,
    pub max: u16,
    pub m: },
    pub min: u8,
    pub max: u8,
    pub p: },
    pub min: u8,
    pub max: u8,
    pub s: },
    pub min: i16,
    pub max: i16,
    pub k: },
}

//
// struct rzv2h_pll_pars - PLL configuration parameters
//
// This structure contains the configuration parameters for the
// Phase-Locked Loop (PLL), used to achieve a specific output frequency.
//
// @m: Main divider value
// @p: Pre-divider value
// @s: Output divider value
// @k: Delta-sigma modulation value
// @freq_millihz: Calculated PLL output frequency in millihertz
// @error_millihz: Frequency error from target in millihertz (signed)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_pll_pars {
    pub m: u16,
    pub p: u8,
    pub s: u8,
    pub k: i16,
    pub freq_millihz: u64,
    pub error_millihz: i64,
}

//
// struct rzv2h_pll_div_pars - PLL parameters with post-divider
//
// This structure is used for PLLs that include an additional post-divider
// stage after the main PLL block. It contains both the PLL configuration
// parameters and the resulting frequency/error values after the divider.
//
// @pll: Main PLL configuration parameters (see struct rzv2h_pll_pars)
//
// @div: Post-divider configuration and result
// @div.divider_value: Divider applied to the PLL output
// @div.freq_millihz: Output frequency after divider in millihertz
// @div.error_millihz: Frequency error from target in millihertz (signed)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_pll_div_pars {
    pub pll: rzv2h_pll_pars,
    pub divider_value: u8,
    pub freq_millihz: u64,
    pub error_millihz: i64,
    pub div: },
}

