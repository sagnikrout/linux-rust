//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs-pll.h
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
// drivers/media/i2c/ccs-pll.h
//
// Generic MIPI CCS/SMIA/SMIA++ PLL calculator
//
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@linux.intel.com>
//

// CSI-2 or CCP-2
pub const CCS_PLL_BUS_TYPE_CSI2_DPHY: c_uint = 0x00;
pub const CCS_PLL_BUS_TYPE_CSI2_CPHY: c_uint = 0x01;
// Old SMIA and implementation specific flags.
// OP PIX clock is for all lanes in total normally.

// If set, the PLL multipliers are required to be even.

// CCS PLL flags
// The sensor doesn't have OP clocks at all.

// System speed model if this flag is unset.

// If set, the pre-PLL divider may have odd values, too.

//
// If set, the OP PIX clock doesn't have to exactly match with data rate, it may
// be higher. See "OP Domain Formulas" in MIPI CCS 1.1 spec.
//

// If set, the VT domain may run faster than the OP domain.

// If set, the VT domain may run slower than the OP domain.

// If set, the PLL tree has two PLLs instead of one.

//
// If set, the OP SYS clock is a dual data rate clock, transferring two bits per
// cycle instead of one.
//

//
// If set, the OP PIX clock is a dual data rate clock, transferring two pixels
// per cycle instead of one.
//

//
// struct ccs_pll_branch_fr - CCS PLL configuration (front)
//
// A single branch front-end of the CCS PLL tree.
//
// @pre_pll_clk_div: Pre-PLL clock divisor
// @pll_multiplier: PLL multiplier
// @pll_ip_clk_freq_hz: PLL input clock frequency
// @pll_op_clk_freq_hz: PLL output clock frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll_branch_fr {
    pub pre_pll_clk_div: u16,
    pub pll_multiplier: u16,
    pub pll_ip_clk_freq_hz: u32,
    pub pll_op_clk_freq_hz: u32,
}

//
// struct ccs_pll_branch_bk - CCS PLL configuration (back)
//
// A single branch back-end of the CCS PLL tree.
//
// @sys_clk_div: System clock divider
// @pix_clk_div: Pixel clock divider
// @sys_clk_freq_hz: System clock frequency
// @pix_clk_freq_hz: Pixel clock frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll_branch_bk {
    pub sys_clk_div: u16,
    pub pix_clk_div: u16,
    pub sys_clk_freq_hz: u32,
    pub pix_clk_freq_hz: u32,
}

//
// struct ccs_pll - Full CCS PLL configuration
//
// All information required to calculate CCS PLL configuration.
//
// @bus_type: Type of the data bus, CCS_PLL_BUS_TYPE_* (input)
// @op_lanes: Number of operational lanes (input)
// @vt_lanes: Number of video timing lanes (input)
// @csi2: CSI-2 related parameters
// @csi2.lanes: The number of the CSI-2 data lanes (input)
// @binning_vertical: Vertical binning factor (input)
// @binning_horizontal: Horizontal binning factor (input)
// @scale_m: Downscaling factor, M component, [16, max] (input)
// @scale_n: Downscaling factor, N component, typically 16 (input)
// @bits_per_pixel: Bits per pixel on the output data bus (input)
// @op_bits_per_lane: Number of bits per OP lane (input)
// @flags: CCS_PLL_FLAG_* (input)
// @link_freq: Chosen link frequency (input)
// @ext_clk_freq_hz: External clock frequency, i.e. the sensor's input clock
// (input)
// @vt_fr: Video timing front-end configuration (output)
// @vt_bk: Video timing back-end configuration (output)
// @op_fr: Operational timing front-end configuration (output)
// @op_bk: Operational timing back-end configuration (output)
// @pixel_rate_csi: Pixel rate on the output data bus (output)
// @pixel_rate_pixel_array: Nominal pixel rate in the sensor's pixel array
// (output)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll {
// input values
    pub bus_type: u8,
    pub op_lanes: u8,
    pub vt_lanes: u8,
    pub lanes: u8,
    pub csi2: },
    pub binning_horizontal: u8,
    pub binning_vertical: u8,
    pub scale_m: u8,
    pub scale_n: u8,
    pub bits_per_pixel: u8,
    pub op_bits_per_lane: u8,
    pub flags: u16,
    pub link_freq: u32,
    pub ext_clk_freq_hz: u32,
// output values
    pub vt_fr: ccs_pll_branch_fr,
    pub vt_bk: ccs_pll_branch_bk,
    pub op_fr: ccs_pll_branch_fr,
    pub op_bk: ccs_pll_branch_bk,
    pub pixel_rate_csi: u32,
    pub pixel_rate_pixel_array: u32,
}

//
// struct ccs_pll_branch_limits_fr - CCS PLL front-end limits
//
// @min_pre_pll_clk_div: Minimum pre-PLL clock divider
// @max_pre_pll_clk_div: Maximum pre-PLL clock divider
// @min_pll_ip_clk_freq_hz: Minimum PLL input clock frequency
// @max_pll_ip_clk_freq_hz: Maximum PLL input clock frequency
// @min_pll_multiplier: Minimum PLL multiplier
// @max_pll_multiplier: Maximum PLL multiplier
// @min_pll_op_clk_freq_hz: Minimum PLL output clock frequency
// @max_pll_op_clk_freq_hz: Maximum PLL output clock frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll_branch_limits_fr {
    pub min_pre_pll_clk_div: u16,
    pub max_pre_pll_clk_div: u16,
    pub min_pll_ip_clk_freq_hz: u32,
    pub max_pll_ip_clk_freq_hz: u32,
    pub min_pll_multiplier: u16,
    pub max_pll_multiplier: u16,
    pub min_pll_op_clk_freq_hz: u32,
    pub max_pll_op_clk_freq_hz: u32,
}

//
// struct ccs_pll_branch_limits_bk - CCS PLL back-end limits
//
// @min_sys_clk_div: Minimum system clock divider
// @max_sys_clk_div: Maximum system clock divider
// @min_sys_clk_freq_hz: Minimum system clock frequency
// @max_sys_clk_freq_hz: Maximum system clock frequency
// @min_pix_clk_div: Minimum pixel clock divider
// @max_pix_clk_div: Maximum pixel clock divider
// @min_pix_clk_freq_hz: Minimum pixel clock frequency
// @max_pix_clk_freq_hz: Maximum pixel clock frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll_branch_limits_bk {
    pub min_sys_clk_div: u16,
    pub max_sys_clk_div: u16,
    pub min_sys_clk_freq_hz: u32,
    pub max_sys_clk_freq_hz: u32,
    pub min_pix_clk_div: u16,
    pub max_pix_clk_div: u16,
    pub min_pix_clk_freq_hz: u32,
    pub max_pix_clk_freq_hz: u32,
}

//
// struct ccs_pll_limits - CCS PLL limits
//
// @min_ext_clk_freq_hz: Minimum external clock frequency
// @max_ext_clk_freq_hz: Maximum external clock frequency
// @vt_fr: Video timing front-end limits
// @vt_bk: Video timing back-end limits
// @op_fr: Operational timing front-end limits
// @op_bk: Operational timing back-end limits
// @min_line_length_pck_bin: Minimum line length in pixels, with binning
// @min_line_length_pck: Minimum line length in pixels without binning
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pll_limits {
// Strict PLL limits
    pub min_ext_clk_freq_hz: u32,
    pub max_ext_clk_freq_hz: u32,
    pub vt_fr: ccs_pll_branch_limits_fr,
    pub vt_bk: ccs_pll_branch_limits_bk,
    pub op_fr: ccs_pll_branch_limits_fr,
    pub op_bk: ccs_pll_branch_limits_bk,
// Other relevant limits
    pub min_line_length_pck_bin: u32,
    pub min_line_length_pck: u32,
}

//
// ccs_pll_calculate - Calculate CCS PLL configuration based on input parameters
//
// @dev: Device pointer, used for printing messages
// @limits: Limits specific to the sensor
// @pll: Given PLL configuration
//
// Calculate the CCS PLL configuration based on the limits as well as given
// device specific, system specific or user configured input data.
//
