//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/rockchip/clk.h
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
// Copyright (c) 2014 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
// Copyright (c) 2015 Rockchip Electronics Co. Ltd.
// Author: Xing Zheng <zhengxing@rock-chips.com>
//
// based on
//
// samsung/clk.h
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Copyright (c) 2013 Linaro Ltd.
// Author: Thomas Abraham <thomas.ab@samsung.com>
//

// register positions shared by PX30, RV1108, RK2928, RK3036, RK3066, RK3188 and RK3228

pub const BOOST_CLK_CON: c_uint = 0x0008;
pub const BOOST_BOOST_CON: c_uint = 0x000c;
pub const BOOST_SWITCH_CNT: c_uint = 0x0010;
pub const BOOST_HIGH_PERF_CNT0: c_uint = 0x0014;
pub const BOOST_HIGH_PERF_CNT1: c_uint = 0x0018;
pub const BOOST_STATIS_THRESHOLD: c_uint = 0x001c;
pub const BOOST_SHORT_SWITCH_CNT: c_uint = 0x0020;
pub const BOOST_SWITCH_THRESHOLD: c_uint = 0x0024;
pub const BOOST_FSM_STATUS: c_uint = 0x0028;

pub const BOOST_RECOVERY_MASK: c_uint = 0x1;
pub const BOOST_RECOVERY_SHIFT: c_int = 1;
pub const BOOST_SW_CTRL_MASK: c_uint = 0x1;
pub const BOOST_SW_CTRL_SHIFT: c_int = 2;
pub const BOOST_LOW_FREQ_EN_MASK: c_uint = 0x1;
pub const BOOST_LOW_FREQ_EN_SHIFT: c_int = 3;

pub const PX30_GLB_SRST_FST: c_uint = 0xb8;
pub const PX30_GLB_SRST_SND: c_uint = 0xbc;

pub const PX30_MODE_CON: c_uint = 0xa0;
pub const PX30_MISC_CON: c_uint = 0xa4;
pub const PX30_SDMMC_CON0: c_uint = 0x380;
pub const PX30_SDMMC_CON1: c_uint = 0x384;
pub const PX30_SDIO_CON0: c_uint = 0x388;
pub const PX30_SDIO_CON1: c_uint = 0x38c;
pub const PX30_EMMC_CON0: c_uint = 0x390;
pub const PX30_EMMC_CON1: c_uint = 0x394;

pub const PX30_PMU_MODE: c_uint = 0x0020;
pub const RV1103B_TOPCRU_BASE: c_uint = 0x60000;
pub const RV1103B_PERICRU_BASE: c_uint = 0x0;
pub const RV1103B_VICRU_BASE: c_uint = 0x30000;
pub const RV1103B_NPUCRU_BASE: c_uint = 0x20000;
pub const RV1103B_CORECRU_BASE: c_uint = 0x40000;
pub const RV1103B_VEPUCRU_BASE: c_uint = 0x10000;
pub const RV1103B_DDRCRU_BASE: c_uint = 0x50000;
pub const RV1103B_SUBDDRCRU_BASE: c_uint = 0x58000;
pub const RV1103B_PMUCRU_BASE: c_uint = 0x70000;
pub const RV1103B_PMU1CRU_BASE: c_uint = 0x80000;

pub const RV1108_GLB_SRST_FST: c_uint = 0x1c0;
pub const RV1108_GLB_SRST_SND: c_uint = 0x1c4;
pub const RV1108_MISC_CON: c_uint = 0x1cc;
pub const RV1108_SDMMC_CON0: c_uint = 0x1d8;
pub const RV1108_SDMMC_CON1: c_uint = 0x1dc;
pub const RV1108_SDIO_CON0: c_uint = 0x1e0;
pub const RV1108_SDIO_CON1: c_uint = 0x1e4;
pub const RV1108_EMMC_CON0: c_uint = 0x1e8;
pub const RV1108_EMMC_CON1: c_uint = 0x1ec;
pub const RV1126_PMU_MODE: c_uint = 0x0;

pub const RV1126_MODE_CON: c_uint = 0x90;

pub const RV1126_GLB_SRST_FST: c_uint = 0x408;
pub const RV1126_GLB_SRST_SND: c_uint = 0x40c;
pub const RV1126_SDMMC_CON0: c_uint = 0x440;
pub const RV1126_SDMMC_CON1: c_uint = 0x444;
pub const RV1126_SDIO_CON0: c_uint = 0x448;
pub const RV1126_SDIO_CON1: c_uint = 0x44c;
pub const RV1126_EMMC_CON0: c_uint = 0x450;
pub const RV1126_EMMC_CON1: c_uint = 0x454;
pub const RV1126B_TOPCRU_BASE: c_uint = 0x0;
pub const RV1126B_BUSCRU_BASE: c_uint = 0x10000;
pub const RV1126B_PERICRU_BASE: c_uint = 0x20000;
pub const RV1126B_CORECRU_BASE: c_uint = 0x30000;
pub const RV1126B_PMUCRU_BASE: c_uint = 0x40000;
pub const RV1126B_PMU1CRU_BASE: c_uint = 0x50000;
pub const RV1126B_DDRCRU_BASE: c_uint = 0x60000;
pub const RV1126B_SUBDDRCRU_BASE: c_uint = 0x68000;
pub const RV1126B_VICRU_BASE: c_uint = 0x70000;
pub const RV1126B_VEPUCRU_BASE: c_uint = 0x80000;
pub const RV1126B_NPUCRU_BASE: c_uint = 0x90000;
pub const RV1126B_VDOCRU_BASE: c_uint = 0xA0000;
pub const RV1126B_VCPCRU_BASE: c_uint = 0xB0000;

pub const RK2928_MODE_CON: c_uint = 0x40;

pub const RK2928_GLB_SRST_FST: c_uint = 0x100;
pub const RK2928_GLB_SRST_SND: c_uint = 0x104;

pub const RK2928_MISC_CON: c_uint = 0x134;
pub const RK3036_SDMMC_CON0: c_uint = 0x144;
pub const RK3036_SDMMC_CON1: c_uint = 0x148;
pub const RK3036_SDIO_CON0: c_uint = 0x14c;
pub const RK3036_SDIO_CON1: c_uint = 0x150;
pub const RK3036_EMMC_CON0: c_uint = 0x154;
pub const RK3036_EMMC_CON1: c_uint = 0x158;
pub const RK3228_GLB_SRST_FST: c_uint = 0x1f0;
pub const RK3228_GLB_SRST_SND: c_uint = 0x1f4;
pub const RK3228_SDMMC_CON0: c_uint = 0x1c0;
pub const RK3228_SDMMC_CON1: c_uint = 0x1c4;
pub const RK3228_SDIO_CON0: c_uint = 0x1c8;
pub const RK3228_SDIO_CON1: c_uint = 0x1cc;
pub const RK3228_EMMC_CON0: c_uint = 0x1d8;
pub const RK3228_EMMC_CON1: c_uint = 0x1dc;

pub const RK3288_MODE_CON: c_uint = 0x50;

pub const RK3288_GLB_SRST_FST: c_uint = 0x1b0;
pub const RK3288_GLB_SRST_SND: c_uint = 0x1b4;

pub const RK3288_MISC_CON: c_uint = 0x1e8;
pub const RK3288_SDMMC_CON0: c_uint = 0x200;
pub const RK3288_SDMMC_CON1: c_uint = 0x204;
pub const RK3288_SDIO0_CON0: c_uint = 0x208;
pub const RK3288_SDIO0_CON1: c_uint = 0x20c;
pub const RK3288_SDIO1_CON0: c_uint = 0x210;
pub const RK3288_SDIO1_CON1: c_uint = 0x214;
pub const RK3288_EMMC_CON0: c_uint = 0x218;
pub const RK3288_EMMC_CON1: c_uint = 0x21c;

pub const RK3308_GLB_SRST_FST: c_uint = 0xb8;

pub const RK3308_MODE_CON: c_uint = 0xa0;
pub const RK3308_SDMMC_CON0: c_uint = 0x480;
pub const RK3308_SDMMC_CON1: c_uint = 0x484;
pub const RK3308_SDIO_CON0: c_uint = 0x488;
pub const RK3308_SDIO_CON1: c_uint = 0x48c;
pub const RK3308_EMMC_CON0: c_uint = 0x490;
pub const RK3308_EMMC_CON1: c_uint = 0x494;

pub const RK3328_GLB_SRST_FST: c_uint = 0x9c;
pub const RK3328_GLB_SRST_SND: c_uint = 0x98;

pub const RK3328_MODE_CON: c_uint = 0x80;
pub const RK3328_MISC_CON: c_uint = 0x84;
pub const RK3328_SDMMC_CON0: c_uint = 0x380;
pub const RK3328_SDMMC_CON1: c_uint = 0x384;
pub const RK3328_SDIO_CON0: c_uint = 0x388;
pub const RK3328_SDIO_CON1: c_uint = 0x38c;
pub const RK3328_EMMC_CON0: c_uint = 0x390;
pub const RK3328_EMMC_CON1: c_uint = 0x394;
pub const RK3328_SDMMC_EXT_CON0: c_uint = 0x398;
pub const RK3328_SDMMC_EXT_CON1: c_uint = 0x39C;

pub const RK3368_GLB_SRST_FST: c_uint = 0x280;
pub const RK3368_GLB_SRST_SND: c_uint = 0x284;

pub const RK3368_MISC_CON: c_uint = 0x380;
pub const RK3368_SDMMC_CON0: c_uint = 0x400;
pub const RK3368_SDMMC_CON1: c_uint = 0x404;
pub const RK3368_SDIO0_CON0: c_uint = 0x408;
pub const RK3368_SDIO0_CON1: c_uint = 0x40c;
pub const RK3368_SDIO1_CON0: c_uint = 0x410;
pub const RK3368_SDIO1_CON1: c_uint = 0x414;
pub const RK3368_EMMC_CON0: c_uint = 0x418;
pub const RK3368_EMMC_CON1: c_uint = 0x41c;

pub const RK3399_GLB_SRST_FST: c_uint = 0x500;
pub const RK3399_GLB_SRST_SND: c_uint = 0x504;
pub const RK3399_GLB_CNT_TH: c_uint = 0x508;
pub const RK3399_MISC_CON: c_uint = 0x50c;
pub const RK3399_RST_CON: c_uint = 0x510;
pub const RK3399_RST_ST: c_uint = 0x514;
pub const RK3399_SDMMC_CON0: c_uint = 0x580;
pub const RK3399_SDMMC_CON1: c_uint = 0x584;
pub const RK3399_SDIO_CON0: c_uint = 0x588;
pub const RK3399_SDIO_CON1: c_uint = 0x58c;

pub const RK3506_PMU_CRU_BASE: c_uint = 0x10000;

pub const RK3506_MODE_CON: c_uint = 0x280;
pub const RK3506_GLB_CNT_TH: c_uint = 0xc00;
pub const RK3506_GLB_SRST_FST: c_uint = 0xc08;
pub const RK3506_GLB_SRST_SND: c_uint = 0xc0c;
pub const RK3528_PMU_CRU_BASE: c_uint = 0x10000;
pub const RK3528_PCIE_CRU_BASE: c_uint = 0x20000;
pub const RK3528_DDRPHY_CRU_BASE: c_uint = 0x28000;

pub const RK3528_MODE_CON: c_uint = 0x280;

pub const RK3528_GLB_CNT_TH: c_uint = 0xc00;
pub const RK3528_GLB_SRST_FST: c_uint = 0xc08;
pub const RK3528_GLB_SRST_SND: c_uint = 0xc0c;
pub const RK3562_PMU0_CRU_BASE: c_uint = 0x10000;
pub const RK3562_PMU1_CRU_BASE: c_uint = 0x18000;
pub const RK3562_DDR_CRU_BASE: c_uint = 0x20000;
pub const RK3562_SUBDDR_CRU_BASE: c_uint = 0x28000;
pub const RK3562_PERI_CRU_BASE: c_uint = 0x30000;

pub const RK3562_MODE_CON: c_uint = 0x600;

pub const RK3562_GLB_SRST_FST: c_uint = 0x614;
pub const RK3562_GLB_SRST_SND: c_uint = 0x618;
pub const RK3562_GLB_RST_CON: c_uint = 0x61c;
pub const RK3562_GLB_RST_ST: c_uint = 0x620;
pub const RK3562_SDMMC0_CON0: c_uint = 0x624;
pub const RK3562_SDMMC0_CON1: c_uint = 0x628;
pub const RK3562_SDMMC1_CON0: c_uint = 0x62c;
pub const RK3562_SDMMC1_CON1: c_uint = 0x630;

pub const RK3568_MODE_CON0: c_uint = 0xc0;
pub const RK3568_MISC_CON0: c_uint = 0xc4;
pub const RK3568_MISC_CON1: c_uint = 0xc8;
pub const RK3568_MISC_CON2: c_uint = 0xcc;
pub const RK3568_GLB_CNT_TH: c_uint = 0xd0;
pub const RK3568_GLB_SRST_FST: c_uint = 0xd4;
pub const RK3568_GLB_SRST_SND: c_uint = 0xd8;
pub const RK3568_GLB_RST_CON: c_uint = 0xdc;
pub const RK3568_GLB_RST_ST: c_uint = 0xe0;

pub const RK3568_SDMMC0_CON0: c_uint = 0x580;
pub const RK3568_SDMMC0_CON1: c_uint = 0x584;
pub const RK3568_SDMMC1_CON0: c_uint = 0x588;
pub const RK3568_SDMMC1_CON1: c_uint = 0x58c;
pub const RK3568_SDMMC2_CON0: c_uint = 0x590;
pub const RK3568_SDMMC2_CON1: c_uint = 0x594;
pub const RK3568_EMMC_CON0: c_uint = 0x598;
pub const RK3568_EMMC_CON1: c_uint = 0x59c;

pub const RK3568_PMU_MODE_CON0: c_uint = 0x80;

pub const RK3576_PHP_CRU_BASE: c_uint = 0x8000;
pub const RK3576_SECURE_NS_CRU_BASE: c_uint = 0x10000;
pub const RK3576_PMU_CRU_BASE: c_uint = 0x20000;
pub const RK3576_BIGCORE_CRU_BASE: c_uint = 0x38000;
pub const RK3576_LITCORE_CRU_BASE: c_uint = 0x40000;
pub const RK3576_CCI_CRU_BASE: c_uint = 0x48000;

pub const RK3576_MODE_CON0: c_uint = 0x280;

pub const RK3576_GLB_CNT_TH: c_uint = 0xc00;
pub const RK3576_GLB_SRST_FST: c_uint = 0xc08;
pub const RK3576_GLB_SRST_SND: c_uint = 0xc0c;
pub const RK3576_GLB_RST_CON: c_uint = 0xc10;
pub const RK3576_GLB_RST_ST: c_uint = 0xc04;
pub const RK3576_SDIO_CON0: c_uint = 0xC24;
pub const RK3576_SDIO_CON1: c_uint = 0xC28;
pub const RK3576_SDMMC_CON0: c_uint = 0xC30;
pub const RK3576_SDMMC_CON1: c_uint = 0xC34;

pub const RK3576_NON_SECURE_GATING_CON00: c_uint = 0xc48;
pub const RK3588_PHP_CRU_BASE: c_uint = 0x8000;
pub const RK3588_PMU_CRU_BASE: c_uint = 0x30000;
pub const RK3588_BIGCORE0_CRU_BASE: c_uint = 0x50000;
pub const RK3588_BIGCORE1_CRU_BASE: c_uint = 0x52000;
pub const RK3588_DSU_CRU_BASE: c_uint = 0x58000;

pub const RK3588_MODE_CON0: c_uint = 0x280;

pub const RK3588_GLB_CNT_TH: c_uint = 0xc00;
pub const RK3588_GLB_SRST_FST: c_uint = 0xc08;
pub const RK3588_GLB_SRST_SND: c_uint = 0xc0c;
pub const RK3588_GLB_RST_CON: c_uint = 0xc10;
pub const RK3588_GLB_RST_ST: c_uint = 0xc04;
pub const RK3588_SDIO_CON0: c_uint = 0xC24;
pub const RK3588_SDIO_CON1: c_uint = 0xC28;
pub const RK3588_SDMMC_CON0: c_uint = 0xC30;
pub const RK3588_SDMMC_CON1: c_uint = 0xC34;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_pll_type {
    pll_rk3036,
    pll_rk3066,
    pll_rk3328,
    pll_rk3399,
    pll_rk3588,
    pll_rk3588_core,
    pll_rk3588_ddr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_grf_type {
    grf_type_sys = 0,
    grf_type_pmu0,
    grf_type_pmu1,
    grf_type_ioc,
    grf_type_vo,
    grf_type_vpu,
}

// ceil(sqrt(enums in rockchip_grf_type - 1))
pub const GRF_HASH_ORDER: c_int = 2;
//
// struct rockchip_aux_grf - entry for the aux_grf_table hashtable
// @grf: pointer to the grf this entry references
// @type: what type of GRF this is
// @node: hlist node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_aux_grf {
    pub grf: *mut regmap,
    pub type: rockchip_grf_type,
    pub node: hlist_node,
}

//
// struct rockchip_clk_provider - information about clock provider
// @reg_base: virtual address for the register base.
// @clk_data: holds clock related data like clk* and number of clocks.
// @cru_node: device-node of the clock-provider
// @grf: regmap of the general-register-files syscon
// @aux_grf_table: hashtable of auxiliary GRF regmaps, indexed by grf_type
// @lock: maintains exclusion between callbacks for a given clock-provider.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_clk_provider {
    pub reg_base: *mut void __iomem,
    pub clk_data: clk_onecell_data,
    pub cru_node: *mut device_node,
    pub grf: *mut regmap,
    pub GRF_HASH_ORDER): DECLARE_HASHTABLE(aux_grf_table,,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pll_rate_table {
    pub rate: c_ulong,
// for RK3066
    pub nr: c_uint,
    pub nf: c_uint,
    pub no: c_uint,
    pub nb: c_uint,
}

// for RK3036/RK3399
// for RK3588
//
// struct rockchip_pll_clock - information about pll clock
// @id: platform specific id of the clock.
// @name: name of this pll clock.
// @parent_names: name of the parent clock.
// @num_parents: number of parents
// @flags: optional flags for basic clock.
// @con_offset: offset of the register for configuring the PLL.
// @mode_offset: offset of the register for configuring the PLL-mode.
// @mode_shift: offset inside the mode-register for the mode of this pll.
// @lock_shift: offset inside the lock register for the lock status.
// @type: Type of PLL to be registered.
// @pll_flags: hardware-specific flags
// @rate_table: Table of usable pll rates
//
// Flags:
// ROCKCHIP_PLL_SYNC_RATE - check rate parameters to match against the
// rate_table parameters and adjust them if necessary.
// ROCKCHIP_PLL_FIXED_MODE - the pll operates in normal mode only
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pll_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub con_offset: c_int,
    pub mode_offset: c_int,
    pub mode_shift: c_int,
    pub lock_shift: c_int,
    pub type: rockchip_pll_type,
    pub pll_flags: u8,
    pub rate_table: *mut rockchip_pll_rate_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_cpuclk_clksel {
    pub reg: c_int,
    pub val: u32,
}

pub const ROCKCHIP_CPUCLK_NUM_DIVIDERS: c_int = 6;
pub const ROCKCHIP_CPUCLK_MAX_CORES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_cpuclk_rate_table {
    pub prate: c_ulong,
    pub divs: [rockchip_cpuclk_clksel; ROCKCHIP_CPUCLK_NUM_DIVIDERS],
    pub pre_muxs: [rockchip_cpuclk_clksel; ROCKCHIP_CPUCLK_NUM_DIVIDERS],
    pub post_muxs: [rockchip_cpuclk_clksel; ROCKCHIP_CPUCLK_NUM_DIVIDERS],
}

//
// struct rockchip_cpuclk_reg_data - register offsets and masks of the cpuclock
// @core_reg[]:	register offset of the cores setting register
// @div_core_shift[]:	cores divider offset used to divide the pll value
// @div_core_mask[]:	cores divider mask
// @num_cores:	number of cpu cores
// @mux_core_reg:       register offset of the cores select parent
// @mux_core_alt:       mux value to select alternate parent
// @mux_core_main:	mux value to select main parent of core
// @mux_core_shift:	offset of the core multiplexer
// @mux_core_mask:	core multiplexer mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_cpuclk_reg_data {
    pub core_reg: [c_int; ROCKCHIP_CPUCLK_MAX_CORES],
    pub div_core_shift: [u8; ROCKCHIP_CPUCLK_MAX_CORES],
    pub div_core_mask: [u32; ROCKCHIP_CPUCLK_MAX_CORES],
    pub num_cores: c_int,
    pub mux_core_reg: c_int,
    pub mux_core_alt: u8,
    pub mux_core_main: u8,
    pub mux_core_shift: u8,
    pub mux_core_mask: u32,
}

//
// DDRCLK flags, including method of setting the rate
// ROCKCHIP_DDRCLK_SIP: use SIP call to bl31 to change ddrclk rate.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rockchip_clk_branch_type {
    branch_composite,
    branch_mux,
    branch_grf_mux,
    branch_divider,
    branch_fraction_divider,
    branch_gate,
    branch_grf_gate,
    branch_linked_gate,
    branch_mmc,
    branch_grf_mmc,
    branch_inverter,
    branch_factor,
    branch_ddrclk,
    branch_half_divider,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_clk_branch {
    pub id: c_uint,
    pub branch_type: rockchip_clk_branch_type,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub muxdiv_offset: c_int,
    pub mux_shift: u8,
    pub mux_width: u8,
    pub mux_flags: u8,
    pub mux_table: *mut u32,
    pub div_offset: c_int,
    pub div_shift: u8,
    pub div_width: u8,
    pub div_flags: u8,
    pub div_table: *mut clk_div_table,
    pub gate_offset: c_int,
    pub gate_shift: u8,
    pub gate_flags: u8,
    pub linked_clk_id: c_uint,
    pub grf_type: rockchip_grf_type,
    pub child: *mut rockchip_clk_branch,
}

// SGRF clocks are only accessible from secure mode, so not controllable

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_gate_link_platdata {
    pub ctx: *mut rockchip_clk_provider,
    pub clkbr: *mut rockchip_clk_branch,
}

extern "C" {
    pub fn rockchip_clk_finalize(ctx: *mut rockchip_clk_provider);
}
extern "C" {
    pub fn rockchip_clk_protect_critical(clocks[]: *const *const c_char, nclocks: c_int);
}

extern "C" {
    pub fn rockchip_register_softrst_lut(_arg: np, _arg: NULL, _arg: num_regs, _arg: base, _arg: flags) -> return;
}
extern "C" {
    pub fn rv1126b_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
extern "C" {
    pub fn rk3506_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
extern "C" {
    pub fn rk3528_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
extern "C" {
    pub fn rk3562_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
extern "C" {
    pub fn rk3576_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
extern "C" {
    pub fn rk3588_rst_init(np: *mut device_node, reg_base: *mut void __iomem);
}
