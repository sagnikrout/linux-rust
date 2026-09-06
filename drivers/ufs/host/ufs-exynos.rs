//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-exynos.h
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
// UFS Host Controller driver for Exynos specific extensions
//
// Copyright (C) 2014-2015 Samsung Electronics Co., Ltd.
//
// Component registers
//
pub const COMP_CLK_PERIOD: c_uint = 0x44;
//
// UNIPRO registers
//
pub const UNIPRO_DBG_FORCE_DME_CTRL_STATE: c_uint = 0x150;
//
// MIBs for PA debug registers
//
pub const PA_DBG_CLK_PERIOD: c_uint = 0x9514;
pub const PA_DBG_TXPHY_CFGUPDT: c_uint = 0x9518;
pub const PA_DBG_RXPHY_CFGUPDT: c_uint = 0x9519;
pub const PA_DBG_MODE: c_uint = 0x9529;
pub const PA_DBG_SKIP_RESET_PHY: c_uint = 0x9539;
pub const PA_DBG_AUTOMODE_THLD: c_uint = 0x9536;
pub const PA_DBG_OV_TM: c_uint = 0x9540;
pub const PA_DBG_SKIP_LINE_RESET: c_uint = 0x9541;
pub const PA_DBG_LINE_RESET_REQ: c_uint = 0x9543;
pub const PA_DBG_OPTION_SUITE: c_uint = 0x9564;
pub const PA_DBG_OPTION_SUITE_DYN: c_uint = 0x9565;
//
// Note: GS101_DBG_OPTION offsets below differ from the TRM
// but match the downstream driver. Following the TRM
// results in non-functioning UFS.
//
pub const PA_GS101_DBG_OPTION_SUITE1: c_uint = 0x956a;
pub const PA_GS101_DBG_OPTION_SUITE2: c_uint = 0x956d;
//
// MIBs for Transport Layer debug registers
//
pub const T_DBG_SKIP_INIT_HIBERN8_EXIT: c_uint = 0xc001;
//
// Exynos MPHY attributes
//
pub const TX_LINERESET_N_VAL: c_uint = 0x0277;

pub const TX_LINERESET_P_VAL: c_uint = 0x027D;

pub const TX_OV_SLEEP_CNT_TIMER: c_uint = 0x028E;

pub const TX_HIGH_Z_CNT_11_08: c_uint = 0x028C;

pub const TX_HIGH_Z_CNT_07_00: c_uint = 0x028D;

pub const TX_BASE_NVAL_07_00: c_uint = 0x0293;

pub const TX_BASE_NVAL_15_08: c_uint = 0x0294;

pub const TX_GRAN_NVAL_07_00: c_uint = 0x0295;

pub const TX_GRAN_NVAL_10_08: c_uint = 0x0296;

pub const VND_TX_CLK_PRD: c_uint = 0xAA;
pub const VND_TX_CLK_PRD_EN: c_uint = 0xA9;
pub const VND_TX_LINERESET_PVALUE0: c_uint = 0xAD;
pub const VND_TX_LINERESET_PVALUE1: c_uint = 0xAC;
pub const VND_TX_LINERESET_PVALUE2: c_uint = 0xAB;
pub const TX_LINE_RESET_TIME: c_int = 3200;
pub const VND_RX_CLK_PRD: c_uint = 0x12;
pub const VND_RX_CLK_PRD_EN: c_uint = 0x11;
pub const VND_RX_LINERESET_VALUE0: c_uint = 0x1D;
pub const VND_RX_LINERESET_VALUE1: c_uint = 0x1C;
pub const VND_RX_LINERESET_VALUE2: c_uint = 0x1B;
pub const RX_LINE_RESET_TIME: c_int = 1000;
pub const RX_FILLER_ENABLE: c_uint = 0x0316;

pub const RX_LINERESET_VAL: c_uint = 0x0317;

pub const RX_LCC_IGNORE: c_uint = 0x0318;
pub const RX_SYNC_MASK_LENGTH: c_uint = 0x0321;
pub const RX_HIBERN8_WAIT_VAL_BIT_20_16: c_uint = 0x0331;
pub const RX_HIBERN8_WAIT_VAL_BIT_15_08: c_uint = 0x0332;
pub const RX_HIBERN8_WAIT_VAL_BIT_07_00: c_uint = 0x0333;
pub const RX_OV_SLEEP_CNT_TIMER: c_uint = 0x0340;

pub const RX_OV_STALL_CNT_TIMER: c_uint = 0x0341;

pub const RX_BASE_NVAL_07_00: c_uint = 0x0355;

pub const RX_BASE_NVAL_15_08: c_uint = 0x0354;

pub const RX_GRAN_NVAL_07_00: c_uint = 0x0353;

pub const RX_GRAN_NVAL_10_08: c_uint = 0x0352;

pub const CMN_PWM_CLK_CTRL: c_uint = 0x0402;
pub const PWM_CLK_CTRL_MASK: c_uint = 0x3;

// vendor specific pre-defined parameters
pub const SLOW: c_int = 1;
pub const FAST: c_int = 2;
pub const RX_ADV_FINE_GRAN_SUP_EN: c_uint = 0x1;
pub const RX_ADV_FINE_GRAN_STEP_VAL: c_uint = 0x3;
pub const RX_ADV_MIN_ACTV_TIME_CAP: c_uint = 0x9;
pub const PA_GRANULARITY_VAL: c_uint = 0x6;
pub const PA_TACTIVATE_VAL: c_uint = 0x3;
pub const PA_HIBERN8TIME_VAL: c_uint = 0x20;
pub const PCLK_AVAIL_MIN: c_int = 70000000;
pub const PCLK_AVAIL_MAX: c_int = 267000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_ufs_uic_attr {
// TX Attributes
    pub tx_trailingclks: c_uint,
    pub tx_dif_p_nsec: c_uint,
    pub tx_dif_n_nsec: c_uint,
    pub tx_high_z_cnt_nsec: c_uint,
    pub tx_base_unit_nsec: c_uint,
    pub tx_gran_unit_nsec: c_uint,
    pub tx_sleep_cnt: c_uint,
    pub tx_min_activatetime: c_uint,
// RX Attributes
    pub rx_filler_enable: c_uint,
    pub rx_dif_p_nsec: c_uint,
    pub rx_hibern8_wait_nsec: c_uint,
    pub rx_base_unit_nsec: c_uint,
    pub rx_gran_unit_nsec: c_uint,
    pub rx_sleep_cnt: c_uint,
    pub rx_stall_cnt: c_uint,
    pub rx_hs_g1_sync_len_cap: c_uint,
    pub rx_hs_g2_sync_len_cap: c_uint,
    pub rx_hs_g3_sync_len_cap: c_uint,
    pub rx_hs_g1_prep_sync_len_cap: c_uint,
    pub rx_hs_g2_prep_sync_len_cap: c_uint,
    pub rx_hs_g3_prep_sync_len_cap: c_uint,
// Common Attributes
    pub cmn_pwm_clk_ctrl: c_uint,
// Internal Attributes
    pub pa_dbg_clk_period_off: c_uint,
    pub pa_dbg_opt_suite1_val: c_uint,
    pub pa_dbg_opt_suite1_off: c_uint,
    pub pa_dbg_opt_suite2_val: c_uint,
    pub pa_dbg_opt_suite2_off: c_uint,
// Changeable Attributes
    pub rx_adv_fine_gran_sup_en: c_uint,
    pub rx_adv_fine_gran_step: c_uint,
    pub rx_min_actv_time_cap: c_uint,
    pub rx_hibern8_time_cap: c_uint,
    pub rx_adv_min_actv_time_cap: c_uint,
    pub rx_adv_hibern8_time_cap: c_uint,
    pub pa_granularity: c_uint,
    pub pa_tactivate: c_uint,
    pub pa_hibern8time: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_ufs_drv_data {
    pub vops: *const ufs_hba_variant_ops,
    pub uic_attr: *mut exynos_ufs_uic_attr,
    pub quirks: c_uint,
    pub opts: c_uint,
    pub iocc_mask: u32,
// SoC's specific operations
    pub ufs): *mut *mut int (drv_init)(struct exynos_ufs,
    pub ufs): *mut *mut int (pre_link)(struct exynos_ufs,
    pub ufs): *mut *mut int (post_link)(struct exynos_ufs,
    pub pwr): *mut ufs_pa_layer_attr,
    pub pwr): *const ufs_pa_layer_attr,
    pub ufs): *mut *mut int (pre_hce_enable)(struct exynos_ufs,
    pub ufs): *mut *mut int (post_hce_enable)(struct exynos_ufs,
    pub ufs): *mut *mut int (suspend)(struct exynos_ufs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_phy_time_cfg {
    pub tx_linereset_p: u32,
    pub tx_linereset_n: u32,
    pub tx_high_z_cnt: u32,
    pub tx_base_n_val: u32,
    pub tx_gran_n_val: u32,
    pub tx_sleep_cnt: u32,
    pub rx_linereset: u32,
    pub rx_hibern8_wait: u32,
    pub rx_base_n_val: u32,
    pub rx_gran_n_val: u32,
    pub rx_sleep_cnt: u32,
    pub rx_stall_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_ufs {
    pub hba: *mut ufs_hba,
    pub phy: *mut phy,
    pub reg_hci: *mut void __iomem,
    pub reg_unipro: *mut void __iomem,
    pub reg_ufsp: *mut void __iomem,
    pub clk_hci_core: *mut clk,
    pub clk_unipro_main: *mut clk,
    pub clk_apb: *mut clk,
    pub pclk_rate: u32,
    pub pclk_div: u32,
    pub pclk_avail_min: u32,
    pub pclk_avail_max: u32,
    pub mclk_rate: c_ulong,
    pub avail_ln_rx: c_int,
    pub avail_ln_tx: c_int,
    pub rx_sel_idx: c_int,
    pub dev_req_params: ufs_pa_layer_attr,
    pub t_cfg: ufs_phy_time_cfg,
    pub entry_hibern8_t: ktime_t,
    pub drv_data: *const exynos_ufs_drv_data,
    pub sysreg: *mut regmap,
    pub iocc_offset: u32,
    pub iocc_mask: u32,
    pub iocc_val: u32,
    pub opts: u32,

}

extern "C" {
    pub fn exynos_ufs_calc_time_cntr(: *mut exynos_ufs, _arg: c_long) -> c_long;
}
