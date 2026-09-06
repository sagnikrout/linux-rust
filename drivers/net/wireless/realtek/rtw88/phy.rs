//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/phy.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//

extern "C" {
    pub fn rtw_phy_init(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_dynamic_mechanism(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_rf_power_2_rssi(rf_power: *mut i8, path_num: u8) -> u8;
}
extern "C" {
    pub fn rtw_phy_setup_phy_cond(rtwdev: *mut rtw_dev, pkg: u32);
}
extern "C" {
    pub fn rtw_parse_tbl_phy_cond(rtwdev: *mut rtw_dev, tbl: *const rtw_table);
}
extern "C" {
    pub fn rtw_parse_tbl_bb_pg(rtwdev: *mut rtw_dev, tbl: *const rtw_table);
}
extern "C" {
    pub fn rtw_parse_tbl_txpwr_lmt(rtwdev: *mut rtw_dev, tbl: *const rtw_table);
}
extern "C" {
    pub fn rtw_phy_init_tx_power(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_load_tables(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_set_tx_power_level(rtwdev: *mut rtw_dev, channel: u8);
}
extern "C" {
    pub fn rtw_phy_tx_power_by_rate_config(hal: *mut rtw_hal);
}
extern "C" {
    pub fn rtw_phy_tx_power_limit_config(hal: *mut rtw_hal);
}
extern "C" {
    pub fn rtw_phy_pwrtrack_avg(rtwdev: *mut rtw_dev, thermal: u8, path: u8);
}
extern "C" {
    pub fn rtw_phy_pwrtrack_get_delta(rtwdev: *mut rtw_dev, path: u8) -> u8;
}
extern "C" {
    pub fn rtw_phy_pwrtrack_need_lck(rtwdev: *mut rtw_dev) -> bool;
}
extern "C" {
    pub fn rtw_phy_pwrtrack_need_iqk(rtwdev: *mut rtw_dev) -> bool;
}
extern "C" {
    pub fn rtw_phy_set_edcca_th(rtwdev: *mut rtw_dev, l2h: u8, h2l: u8);
}
extern "C" {
    pub fn rtw_phy_adaptivity_set_mode(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_tx_path_diversity(rtwdev: *mut rtw_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_txpwr_lmt_cfg_pair {
    pub regd: u8,
    pub band: u8,
    pub bw: u8,
    pub rs: u8,
    pub ch: u8,
    pub txpwr_lmt: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_phy_pg_cfg_pair {
    pub band: u32,
    pub rf_path: u32,
    pub tx_num: u32,
    pub addr: u32,
    pub bitmask: u32,
    pub data: u32,
}

extern "C" {
    pub fn rtw_phy_dig_write(rtwdev: *mut rtw_dev, igi: u8);
}
extern "C" {
    pub fn rtw_phy_dig_reset(rtwdev: *mut rtw_dev);
}
extern "C" {
    pub fn rtw_phy_dig_set_max_coverage(rtwdev: *mut rtw_dev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_power_params {
    pub pwr_base: u8,
    pub pwr_offset: i8,
    pub pwr_limit: i8,
    pub pwr_remnant: i8,
    pub pwr_sar: i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_phy_cck_pd_lv {
    CCK_PD_LV0,
    CCK_PD_LV1,
    CCK_PD_LV2,
    CCK_PD_LV3,
    CCK_PD_LV4,
    CCK_PD_LV_MAX,
}

pub const MASKBYTE0: c_uint = 0xff;
pub const MASKBYTE1: c_uint = 0xff00;
pub const MASKBYTE2: c_uint = 0xff0000;
pub const MASKBYTE3: c_uint = 0xff000000;
pub const MASKHWORD: c_uint = 0xffff0000;
pub const MASKLWORD: c_uint = 0x0000ffff;
pub const MASKDWORD: c_uint = 0xffffffff;
pub const RFREG_MASK: c_uint = 0xfffff;
pub const MASK7BITS: c_uint = 0x7f;
pub const MASK12BITS: c_uint = 0xfff;
pub const MASKH4BITS: c_uint = 0xf0000000;
pub const MASK20BITS: c_uint = 0xfffff;
pub const MASK24BITS: c_uint = 0xffffff;
pub const MASKH3BYTES: c_uint = 0xffffff00;
pub const MASKL3BYTES: c_uint = 0x00ffffff;
pub const MASKBYTE2HIGHNIBBLE: c_uint = 0x00f00000;
pub const MASKBYTE3LOWNIBBLE: c_uint = 0x0f000000;
pub const MASKL3BYTES: c_uint = 0x00ffffff;
pub const CCK_FA_AVG_RESET: c_uint = 0xffffffff;
pub const LSSI_READ_ADDR_MASK: c_uint = 0x7f800000;
pub const LSSI_READ_EDGE_MASK: c_uint = 0x80000000;
pub const LSSI_READ_DATA_MASK: c_uint = 0xfffff;
pub const RRSR_RATE_ORDER_MAX: c_uint = 0xfffff;
pub const RRSR_RATE_ORDER_CCK_LEN: c_int = 4;
