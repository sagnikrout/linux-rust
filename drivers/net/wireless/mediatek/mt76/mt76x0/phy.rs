//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x0/phy.h
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
// (c) Copyright 2002-2010, Ralink Technology, Inc.
// Copyright (C) 2018 Stanislaw Gruszka <stf_xl@wp.pl>
//
pub const RF_G_BAND: c_uint = 0x0100;
pub const RF_A_BAND: c_uint = 0x0200;
pub const RF_A_BAND_LB: c_uint = 0x0400;
pub const RF_A_BAND_MB: c_uint = 0x0800;
pub const RF_A_BAND_HB: c_uint = 0x1000;
pub const RF_A_BAND_11J: c_uint = 0x2000;
pub const RF_BW_20: c_int = 1;
pub const RF_BW_40: c_int = 2;
pub const RF_BW_10: c_int = 4;
pub const RF_BW_80: c_int = 8;

pub const MT_RF_START_TIME: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x0_bbp_switch_item {
    pub bw_band: u16,
    pub reg_pair: mt76_reg_pair,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x0_rf_switch_item {
    pub rf_bank_reg: u32,
    pub bw_band: u16,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x0_freq_item {
    pub channel: u8,
    pub band: u32,
    pub pllR37: u8,
    pub pllR36: u8,
    pub pllR35: u8,
    pub pllR34: u8,
    pub pllR33: u8,
    pub pllR32_b7b5: u8,
    pub /: *mut *mut u8 pllR32_b4b0; / PLL_DEN (Denomina - 8),
    pub pllR31_b7b5: u8,
    pub )*/: *mut *mut u8 pllR31_b4b0; / PLL_K (Nominator,
    pub /: *mut *mut u8 pllR30_b7; / sdm_reset_n,
    pub /: *mut *mut u8 pllR30_b6b2; / sdmmash_prbs,sin,
    pub /: *mut *mut u8 pllR30_b1; / sdm_bp,
    pub /: *mut *mut u16 pll_n; / R30<0>, R29<7:0> (hex),
    pub /: *mut *mut u8 pllR28_b7b6; / isi,iso,
    pub /: *mut *mut u8 pllR28_b5b4; / pfd_dly,
    pub /: *mut *mut u8 pllR28_b3b2; / clksel option,
    pub /: *mut *mut u32 pll_sdm_k; / R28<1:0>, R27<7:0>, R26<7:0> (hex) SDM_k,
    pub /: *mut *mut u8 pllR24_b1b0; / xo_div,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x0_rate_pwr_item {
    pub mcs_power: i8,
    pub rf_pa_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x0_rate_pwr_tab {
    pub cck: [mt76x0_rate_pwr_item; 4],
    pub ofdm: [mt76x0_rate_pwr_item; 8],
    pub ht: [mt76x0_rate_pwr_item; 8],
    pub vht: [mt76x0_rate_pwr_item; 10],
    pub stbc: [mt76x0_rate_pwr_item; 8],
    pub mcs32: mt76x0_rate_pwr_item,
}
