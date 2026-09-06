//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_tnrdmd.h
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
// cxd2880_tnrdmd.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// common control interface
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

pub const CXD2880_TNRDMD_MAX_CFG_MEM_COUNT: c_int = 100;

pub const CXD2880_TNRDMD_INTERRUPT_TYPE_BUF_UNDERFLOW: c_uint = 0x0001;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_BUF_OVERFLOW: c_uint = 0x0002;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_BUF_ALMOST_EMPTY: c_uint = 0x0004;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_BUF_ALMOST_FULL: c_uint = 0x0008;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_BUF_RRDY: c_uint = 0x0010;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_ILLEGAL_COMMAND: c_uint = 0x0020;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_ILLEGAL_ACCESS: c_uint = 0x0040;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_CPU_ERROR: c_uint = 0x0100;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_LOCK: c_uint = 0x0200;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_INV_LOCK: c_uint = 0x0400;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_NOOFDM: c_uint = 0x0800;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_EWS: c_uint = 0x1000;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_EEW: c_uint = 0x2000;
pub const CXD2880_TNRDMD_INTERRUPT_TYPE_FEC_FAIL: c_uint = 0x4000;
pub const CXD2880_TNRDMD_INTERRUPT_LOCK_SEL_L1POST_OK: c_uint = 0x01;
pub const CXD2880_TNRDMD_INTERRUPT_LOCK_SEL_DMD_LOCK: c_uint = 0x02;
pub const CXD2880_TNRDMD_INTERRUPT_LOCK_SEL_TS_LOCK: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_chip_id {
    CXD2880_TNRDMD_CHIP_ID_UNKNOWN = 0x00,
    CXD2880_TNRDMD_CHIP_ID_CXD2880_ES1_0X = 0x62,
    CXD2880_TNRDMD_CHIP_ID_CXD2880_ES1_11 = 0x6a
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_state {
    CXD2880_TNRDMD_STATE_UNKNOWN,
    CXD2880_TNRDMD_STATE_SLEEP,
    CXD2880_TNRDMD_STATE_ACTIVE,
    CXD2880_TNRDMD_STATE_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_divermode {
    CXD2880_TNRDMD_DIVERMODE_SINGLE,
    CXD2880_TNRDMD_DIVERMODE_MAIN,
    CXD2880_TNRDMD_DIVERMODE_SUB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_clockmode {
    CXD2880_TNRDMD_CLOCKMODE_UNKNOWN,
    CXD2880_TNRDMD_CLOCKMODE_A,
    CXD2880_TNRDMD_CLOCKMODE_B,
    CXD2880_TNRDMD_CLOCKMODE_C
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_tsout_if {
    CXD2880_TNRDMD_TSOUT_IF_TS,
    CXD2880_TNRDMD_TSOUT_IF_SPI,
    CXD2880_TNRDMD_TSOUT_IF_SDIO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_xtal_share {
    CXD2880_TNRDMD_XTAL_SHARE_NONE,
    CXD2880_TNRDMD_XTAL_SHARE_EXTREF,
    CXD2880_TNRDMD_XTAL_SHARE_MASTER,
    CXD2880_TNRDMD_XTAL_SHARE_SLAVE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_spectrum_sense {
    CXD2880_TNRDMD_SPECTRUM_NORMAL,
    CXD2880_TNRDMD_SPECTRUM_INV
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_cfg_id {
    CXD2880_TNRDMD_CFG_OUTPUT_SEL_MSB,
    CXD2880_TNRDMD_CFG_TSVALID_ACTIVE_HI,
    CXD2880_TNRDMD_CFG_TSSYNC_ACTIVE_HI,
    CXD2880_TNRDMD_CFG_TSERR_ACTIVE_HI,
    CXD2880_TNRDMD_CFG_LATCH_ON_POSEDGE,
    CXD2880_TNRDMD_CFG_TSCLK_CONT,
    CXD2880_TNRDMD_CFG_TSCLK_MASK,
    CXD2880_TNRDMD_CFG_TSVALID_MASK,
    CXD2880_TNRDMD_CFG_TSERR_MASK,
    CXD2880_TNRDMD_CFG_TSERR_VALID_DIS,
    CXD2880_TNRDMD_CFG_TSPIN_CURRENT,
    CXD2880_TNRDMD_CFG_TSPIN_PULLUP_MANUAL,
    CXD2880_TNRDMD_CFG_TSPIN_PULLUP,
    CXD2880_TNRDMD_CFG_TSCLK_FREQ,
    CXD2880_TNRDMD_CFG_TSBYTECLK_MANUAL,
    CXD2880_TNRDMD_CFG_TS_PACKET_GAP,
    CXD2880_TNRDMD_CFG_TS_BACKWARDS_COMPATIBLE,
    CXD2880_TNRDMD_CFG_PWM_VALUE,
    CXD2880_TNRDMD_CFG_INTERRUPT,
    CXD2880_TNRDMD_CFG_INTERRUPT_LOCK_SEL,
    CXD2880_TNRDMD_CFG_INTERRUPT_INV_LOCK_SEL,
    CXD2880_TNRDMD_CFG_TS_BUF_ALMOST_EMPTY_THRS,
    CXD2880_TNRDMD_CFG_TS_BUF_ALMOST_FULL_THRS,
    CXD2880_TNRDMD_CFG_TS_BUF_RRDY_THRS,
    CXD2880_TNRDMD_CFG_FIXED_CLOCKMODE,
    CXD2880_TNRDMD_CFG_CABLE_INPUT,
    CXD2880_TNRDMD_CFG_DVBT2_FEF_INTERMITTENT_BASE,
    CXD2880_TNRDMD_CFG_DVBT2_FEF_INTERMITTENT_LITE,
    CXD2880_TNRDMD_CFG_BLINDTUNE_DVBT2_FIRST,
    CXD2880_TNRDMD_CFG_DVBT_BERN_PERIOD,
    CXD2880_TNRDMD_CFG_DVBT_VBER_PERIOD,
    CXD2880_TNRDMD_CFG_DVBT_PER_MES,
    CXD2880_TNRDMD_CFG_DVBT2_BBER_MES,
    CXD2880_TNRDMD_CFG_DVBT2_LBER_MES,
    CXD2880_TNRDMD_CFG_DVBT2_PER_MES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_lock_result {
    CXD2880_TNRDMD_LOCK_RESULT_NOTDETECT,
    CXD2880_TNRDMD_LOCK_RESULT_LOCKED,
    CXD2880_TNRDMD_LOCK_RESULT_UNLOCKED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_gpio_mode {
    CXD2880_TNRDMD_GPIO_MODE_OUTPUT = 0x00,
    CXD2880_TNRDMD_GPIO_MODE_INPUT = 0x01,
    CXD2880_TNRDMD_GPIO_MODE_INT = 0x02,
    CXD2880_TNRDMD_GPIO_MODE_FEC_FAIL = 0x03,
    CXD2880_TNRDMD_GPIO_MODE_PWM = 0x04,
    CXD2880_TNRDMD_GPIO_MODE_EWS = 0x05,
    CXD2880_TNRDMD_GPIO_MODE_EEW = 0x06
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_tnrdmd_serial_ts_clk {
    CXD2880_TNRDMD_SERIAL_TS_CLK_FULL,
    CXD2880_TNRDMD_SERIAL_TS_CLK_HALF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_cfg_mem {
    pub tgt: cxd2880_io_tgt,
    pub bank: u8,
    pub address: u8,
    pub value: u8,
    pub bit_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_pid_cfg {
    pub is_en: u8,
    pub pid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_pid_ftr_cfg {
    pub is_negative: u8,
    pub pid_cfg: [cxd2880_tnrdmd_pid_cfg; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_lna_thrs {
    pub off_on: u8,
    pub on_off: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_lna_thrs_tbl_air {
    pub thrs: [cxd2880_tnrdmd_lna_thrs; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_lna_thrs_tbl_cable {
    pub thrs: [cxd2880_tnrdmd_lna_thrs; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_create_param {
    pub ts_output_if: cxd2880_tnrdmd_tsout_if,
    pub en_internal_ldo: u8,
    pub xtal_share_type: cxd2880_tnrdmd_xtal_share,
    pub xosc_cap: u8,
    pub xosc_i: u8,
    pub is_cxd2881gg: u8,
    pub stationary_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd_diver_create_param {
    pub ts_output_if: cxd2880_tnrdmd_tsout_if,
    pub en_internal_ldo: u8,
    pub xosc_cap_main: u8,
    pub xosc_i_main: u8,
    pub xosc_i_sub: u8,
    pub is_cxd2881gg: u8,
    pub stationary_use: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_tnrdmd {
    pub diver_sub: *mut cxd2880_tnrdmd,
    pub io: *mut cxd2880_io,
    pub create_param: cxd2880_tnrdmd_create_param,
    pub diver_mode: cxd2880_tnrdmd_divermode,
    pub fixed_clk_mode: cxd2880_tnrdmd_clockmode,
    pub is_cable_input: u8,
    pub en_fef_intmtnt_base: u8,
    pub en_fef_intmtnt_lite: u8,
    pub blind_tune_dvbt2_first: u8,
    pub rf_lvl_db): *mut c_int,
    pub lna_thrs_tbl_air: *mut cxd2880_tnrdmd_lna_thrs_tbl_air,
    pub lna_thrs_tbl_cable: *mut cxd2880_tnrdmd_lna_thrs_tbl_cable,
    pub srl_ts_clk_mod_cnts: u8,
    pub srl_ts_clk_frq: cxd2880_tnrdmd_serial_ts_clk,
    pub ts_byte_clk_manual_setting: u8,
    pub is_ts_backwards_compatible_mode: u8,
    pub cfg_mem: [cxd2880_tnrdmd_cfg_mem; CXD2880_TNRDMD_MAX_CFG_MEM_COUNT],
    pub cfg_mem_last_entry: u8,
    pub pid_ftr_cfg: cxd2880_tnrdmd_pid_ftr_cfg,
    pub pid_ftr_cfg_en: u8,
    pub user: *mut c_void,
    pub chip_id: cxd2880_tnrdmd_chip_id,
    pub state: cxd2880_tnrdmd_state,
    pub clk_mode: cxd2880_tnrdmd_clockmode,
    pub frequency_khz: u32,
    pub sys: cxd2880_dtv_sys,
    pub bandwidth: cxd2880_dtv_bandwidth,
    pub scan_mode: u8,
    pub cancel: core::sync::atomic::AtomicI32,
}

// create_param);
// tnr_dmd_main,
// create_param);
extern "C" {
    pub fn cxd2880_tnrdmd_init1(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
extern "C" {
    pub fn cxd2880_tnrdmd_init2(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
// tnr_dmd,
extern "C" {
    pub fn cxd2880_tnrdmd_sleep(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
// tnr_dmd,
// pid_ftr_cfg);
// tnr_dmd,
// tbl_air,
// tbl_cable);
// tbl_air,
// tbl_cable);
// tnr_dmd, u8 en, u8 value);
extern "C" {
    pub fn slvt_freeze_reg(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
