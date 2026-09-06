//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sifive/sifive-prci.h
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
// Copyright (C) 2018-2019 SiFive, Inc.
// Wesley Terpstra
// Paul Walmsley
// Zong Li
//

//
// EXPECTED_CLK_PARENT_COUNT: how many parent clocks this driver expects:
// hfclk and rtcclk
//
pub const EXPECTED_CLK_PARENT_COUNT: c_int = 2;
//
// Register offsets and bitmasks
//
// COREPLLCFG0
pub const PRCI_COREPLLCFG0_OFFSET: c_uint = 0x4;
pub const PRCI_COREPLLCFG0_DIVR_SHIFT: c_int = 0;

pub const PRCI_COREPLLCFG0_DIVF_SHIFT: c_int = 6;

pub const PRCI_COREPLLCFG0_DIVQ_SHIFT: c_int = 15;

pub const PRCI_COREPLLCFG0_RANGE_SHIFT: c_int = 18;

pub const PRCI_COREPLLCFG0_BYPASS_SHIFT: c_int = 24;

pub const PRCI_COREPLLCFG0_FSE_SHIFT: c_int = 25;

pub const PRCI_COREPLLCFG0_LOCK_SHIFT: c_int = 31;

// COREPLLCFG1
pub const PRCI_COREPLLCFG1_OFFSET: c_uint = 0x8;
pub const PRCI_COREPLLCFG1_CKE_SHIFT: c_int = 31;

// DDRPLLCFG0
pub const PRCI_DDRPLLCFG0_OFFSET: c_uint = 0xc;
pub const PRCI_DDRPLLCFG0_DIVR_SHIFT: c_int = 0;

pub const PRCI_DDRPLLCFG0_DIVF_SHIFT: c_int = 6;

pub const PRCI_DDRPLLCFG0_DIVQ_SHIFT: c_int = 15;

pub const PRCI_DDRPLLCFG0_RANGE_SHIFT: c_int = 18;

pub const PRCI_DDRPLLCFG0_BYPASS_SHIFT: c_int = 24;

pub const PRCI_DDRPLLCFG0_FSE_SHIFT: c_int = 25;

pub const PRCI_DDRPLLCFG0_LOCK_SHIFT: c_int = 31;

// DDRPLLCFG1
pub const PRCI_DDRPLLCFG1_OFFSET: c_uint = 0x10;
pub const PRCI_DDRPLLCFG1_CKE_SHIFT: c_int = 31;

// PCIEAUX
pub const PRCI_PCIE_AUX_OFFSET: c_uint = 0x14;
pub const PRCI_PCIE_AUX_EN_SHIFT: c_int = 0;

// GEMGXLPLLCFG0
pub const PRCI_GEMGXLPLLCFG0_OFFSET: c_uint = 0x1c;
pub const PRCI_GEMGXLPLLCFG0_DIVR_SHIFT: c_int = 0;

pub const PRCI_GEMGXLPLLCFG0_DIVF_SHIFT: c_int = 6;

pub const PRCI_GEMGXLPLLCFG0_DIVQ_SHIFT: c_int = 15;

pub const PRCI_GEMGXLPLLCFG0_RANGE_SHIFT: c_int = 18;

pub const PRCI_GEMGXLPLLCFG0_BYPASS_SHIFT: c_int = 24;

pub const PRCI_GEMGXLPLLCFG0_FSE_SHIFT: c_int = 25;

pub const PRCI_GEMGXLPLLCFG0_LOCK_SHIFT: c_int = 31;

// GEMGXLPLLCFG1
pub const PRCI_GEMGXLPLLCFG1_OFFSET: c_uint = 0x20;
pub const PRCI_GEMGXLPLLCFG1_CKE_SHIFT: c_int = 31;

// CORECLKSEL
pub const PRCI_CORECLKSEL_OFFSET: c_uint = 0x24;
pub const PRCI_CORECLKSEL_CORECLKSEL_SHIFT: c_int = 0;

// DEVICESRESETREG
pub const PRCI_DEVICESRESETREG_OFFSET: c_uint = 0x28;
pub const PRCI_DEVICESRESETREG_DDR_CTRL_RST_N_SHIFT: c_int = 0;

pub const PRCI_DEVICESRESETREG_DDR_AXI_RST_N_SHIFT: c_int = 1;

pub const PRCI_DEVICESRESETREG_DDR_AHB_RST_N_SHIFT: c_int = 2;

pub const PRCI_DEVICESRESETREG_DDR_PHY_RST_N_SHIFT: c_int = 3;

pub const PRCI_DEVICESRESETREG_GEMGXL_RST_N_SHIFT: c_int = 5;

pub const PRCI_DEVICESRESETREG_CHIPLINK_RST_N_SHIFT: c_int = 6;

pub const PRCI_RST_NR: c_int = 7;
// CLKMUXSTATUSREG
pub const PRCI_CLKMUXSTATUSREG_OFFSET: c_uint = 0x2c;
pub const PRCI_CLKMUXSTATUSREG_TLCLKSEL_STATUS_SHIFT: c_int = 1;

// CLTXPLLCFG0
pub const PRCI_CLTXPLLCFG0_OFFSET: c_uint = 0x30;
pub const PRCI_CLTXPLLCFG0_DIVR_SHIFT: c_int = 0;

pub const PRCI_CLTXPLLCFG0_DIVF_SHIFT: c_int = 6;

pub const PRCI_CLTXPLLCFG0_DIVQ_SHIFT: c_int = 15;

pub const PRCI_CLTXPLLCFG0_RANGE_SHIFT: c_int = 18;

pub const PRCI_CLTXPLLCFG0_BYPASS_SHIFT: c_int = 24;

pub const PRCI_CLTXPLLCFG0_FSE_SHIFT: c_int = 25;

pub const PRCI_CLTXPLLCFG0_LOCK_SHIFT: c_int = 31;

// CLTXPLLCFG1
pub const PRCI_CLTXPLLCFG1_OFFSET: c_uint = 0x34;
pub const PRCI_CLTXPLLCFG1_CKE_SHIFT: c_int = 31;

// DVFSCOREPLLCFG0
pub const PRCI_DVFSCOREPLLCFG0_OFFSET: c_uint = 0x38;
// DVFSCOREPLLCFG1
pub const PRCI_DVFSCOREPLLCFG1_OFFSET: c_uint = 0x3c;
pub const PRCI_DVFSCOREPLLCFG1_CKE_SHIFT: c_int = 31;

// COREPLLSEL
pub const PRCI_COREPLLSEL_OFFSET: c_uint = 0x40;
pub const PRCI_COREPLLSEL_COREPLLSEL_SHIFT: c_int = 0;

// HFPCLKPLLCFG0
pub const PRCI_HFPCLKPLLCFG0_OFFSET: c_uint = 0x50;
pub const PRCI_HFPCLKPLL_CFG0_DIVR_SHIFT: c_int = 0;

pub const PRCI_HFPCLKPLL_CFG0_DIVF_SHIFT: c_int = 6;

pub const PRCI_HFPCLKPLL_CFG0_DIVQ_SHIFT: c_int = 15;

pub const PRCI_HFPCLKPLL_CFG0_RANGE_SHIFT: c_int = 18;

pub const PRCI_HFPCLKPLL_CFG0_BYPASS_SHIFT: c_int = 24;

pub const PRCI_HFPCLKPLL_CFG0_FSE_SHIFT: c_int = 25;

pub const PRCI_HFPCLKPLL_CFG0_LOCK_SHIFT: c_int = 31;

// HFPCLKPLLCFG1
pub const PRCI_HFPCLKPLLCFG1_OFFSET: c_uint = 0x54;
pub const PRCI_HFPCLKPLLCFG1_CKE_SHIFT: c_int = 31;

// HFPCLKPLLSEL
pub const PRCI_HFPCLKPLLSEL_OFFSET: c_uint = 0x58;
pub const PRCI_HFPCLKPLLSEL_HFPCLKPLLSEL_SHIFT: c_int = 0;

// HFPCLKPLLDIV
pub const PRCI_HFPCLKPLLDIV_OFFSET: c_uint = 0x5c;
// PRCIPLL
pub const PRCI_PRCIPLL_OFFSET: c_uint = 0xe0;
// PROCMONCFG
pub const PRCI_PROCMONCFG_OFFSET: c_uint = 0xf0;
//
// Private structures
//
// struct __prci_data - per-device-instance data
// @va: base virtual address of the PRCI IP block
// @hw_clks: encapsulates struct clk_hw records
//
// PRCI per-device instance data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __prci_data {
    pub va: *mut void __iomem,
    pub reset: reset_simple_data,
    pub hw_clks: clk_hw_onecell_data,
}

//
// struct __prci_wrpll_data - WRPLL configuration and integration data
// @c: WRPLL current configuration record
// @enable_bypass: fn ptr to code to bypass the WRPLL (if applicable; else NULL)
// @disable_bypass: fn ptr to code to not bypass the WRPLL (or NULL)
// @cfg0_offs: WRPLL CFG0 register offset (in bytes) from the PRCI base address
// @cfg1_offs: WRPLL CFG1 register offset (in bytes) from the PRCI base address
//
// @enable_bypass and @disable_bypass are used for WRPLL instances
// that contain a separate external glitchless clock mux downstream
// from the PLL.  The WRPLL internal bypass mux is not glitchless.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __prci_wrpll_data {
    pub c: wrpll_cfg,
    pub pd): *mut *mut void (enable_bypass)(struct __prci_data,
    pub pd): *mut *mut void (disable_bypass)(struct __prci_data,
    pub cfg0_offs: u8,
    pub cfg1_offs: u8,
}

//
// struct __prci_clock - describes a clock device managed by PRCI
// @name: user-readable clock name string - should match the manual
// @parent_name: parent name for this clock
// @ops: struct clk_ops for the Linux clock framework to use for control
// @hw: Linux-private clock data
// @pwd: WRPLL-specific data, associated with this clock (if not NULL)
// @pd: PRCI-specific data associated with this clock (if not NULL)
//
// PRCI clock data.  Used by the PRCI driver to register PRCI-provided
// clocks to the Linux clock infrastructure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __prci_clock {
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub ops: *const clk_ops,
    pub hw: clk_hw,
    pub pwd: *mut __prci_wrpll_data,
    pub pd: *mut __prci_data,
}

//
// struct prci_clk_desc - describes the information of clocks of each SoCs
// @clks: point to a array of __prci_clock
// @num_clks: the number of element of clks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prci_clk_desc {
    pub clks: *mut __prci_clock,
    pub num_clks: usize,
}

// Core clock mux control
extern "C" {
    pub fn sifive_prci_coreclksel_use_hfclk(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_coreclksel_use_corepll(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_coreclksel_use_final_corepll(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_corepllsel_use_dvfscorepll(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_corepllsel_use_corepll(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_hfpclkpllsel_use_hfclk(pd: *mut __prci_data);
}
extern "C" {
    pub fn sifive_prci_hfpclkpllsel_use_hfpclkpll(pd: *mut __prci_data);
}
// Linux clock framework integration
extern "C" {
    pub fn sifive_clk_is_enabled(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn sifive_prci_clock_enable(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn sifive_prci_clock_disable(hw: *mut clk_hw);
}
extern "C" {
    pub fn sifive_prci_pcie_aux_clock_is_enabled(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn sifive_prci_pcie_aux_clock_enable(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn sifive_prci_pcie_aux_clock_disable(hw: *mut clk_hw);
}
