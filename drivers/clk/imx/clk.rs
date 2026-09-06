//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/imx/clk.h
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

extern "C" {
    pub fn imx_check_clocks(clks[]: *mut clk, count: c_uint);
}
extern "C" {
    pub fn imx_check_clk_hws(clks[]: *mut clk_hw, count: c_uint);
}

extern "C" {
    pub fn imx_register_uart_clocks();
}

extern "C" {
    pub fn imx_mmdc_mask_handshake(ccm_base: *mut void __iomem, chn: c_uint);
}
extern "C" {
    pub fn imx_unregister_hw_clocks(hws[]: *mut clk_hw, count: c_uint);
}
extern "C" {
    pub fn imx_cscmr1_fixup(val: *mut u32);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_pllv1_type {
    IMX_PLLV1_IMX1,
    IMX_PLLV1_IMX21,
    IMX_PLLV1_IMX25,
    IMX_PLLV1_IMX27,
    IMX_PLLV1_IMX31,
    IMX_PLLV1_IMX35,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_sscg_pll_type {
    SCCG_PLL1,
    SCCG_PLL2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_pll14xx_type {
    PLL_1416X,
    PLL_1443X,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_pllv4_type {
    IMX_PLLV4_IMX7ULP,
    IMX_PLLV4_IMX8ULP,
    IMX_PLLV4_IMX8ULP_1GHZ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_pfdv2_type {
    IMX_PFDV2_IMX7ULP,
    IMX_PFDV2_IMX8ULP,
}

// NOTE: Rate table should be kept sorted in descending order.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pll14xx_rate_table {
    pub rate: c_uint,
    pub pdiv: c_uint,
    pub mdiv: c_uint,
    pub sdiv: c_uint,
    pub kdiv: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_pll14xx_clk {
    pub type: imx_pll14xx_type,
    pub rate_table: *const imx_pll14xx_rate_table,
    pub rate_count: c_int,
    pub flags: c_int,
}

// NOTE: Rate table should be kept sorted in descending order.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_fracn_gppll_rate_table {
    pub rate: c_uint,
    pub mfi: c_uint,
    pub mfn: c_uint,
    pub mfd: c_uint,
    pub rdiv: c_uint,
    pub odiv: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_fracn_gppll_clk {
    pub rate_table: *const imx_fracn_gppll_rate_table,
    pub rate_count: c_int,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_pllv3_type {
    IMX_PLLV3_GENERIC,
    IMX_PLLV3_SYS,
    IMX_PLLV3_USB,
    IMX_PLLV3_USB_VF610,
    IMX_PLLV3_AV,
    IMX_PLLV3_ENET,
    IMX_PLLV3_ENET_IMX7,
    IMX_PLLV3_SYS_VF610,
    IMX_PLLV3_DDR_IMX7,
    IMX_PLLV3_AV_IMX7,
}

extern "C" {
    pub fn ERR_CAST(_arg: hw) -> return;
}
extern "C" {
    pub fn clk_hw_register_fixed_rate(_arg: NULL, _arg: name, _arg: NULL, _arg: 0, _arg: rate) -> return;
}

extern "C" {
    pub fn imx_audio_pll_debug_init(hws[]: *mut clk_hw, num_plls: c_uint);
}
