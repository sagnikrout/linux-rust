//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/efuse.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const EFUSE_IC_ID_OFFSET: c_int = 506;
pub const EFUSE_MAX_WORD_UNIT: c_int = 4;
pub const EFUSE_INIT_MAP: c_int = 0;
pub const EFUSE_MODIFY_MAP: c_int = 1;
pub const PG_STATE_HEADER: c_uint = 0x01;
pub const PG_STATE_WORD_0: c_uint = 0x02;
pub const PG_STATE_WORD_1: c_uint = 0x04;
pub const PG_STATE_WORD_2: c_uint = 0x08;
pub const PG_STATE_WORD_3: c_uint = 0x10;
pub const PG_STATE_DATA: c_uint = 0x20;
pub const EFUSE_REPEAT_THRESHOLD_: c_int = 3;
pub const EFUSE_ERROE_HANDLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efuse_map {
    pub offset: u8,
    pub word_start: u8,
    pub byte_start: u8,
    pub byte_cnts: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgpkt_struct {
    pub offset: u8,
    pub word_en: u8,
    pub data: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efuse_data_item {
    EFUSE_CHIP_ID = 0,
    EFUSE_LDO_SETTING,
    EFUSE_CLK_SETTING,
    EFUSE_SDIO_SETTING,
    EFUSE_CCCR,
    EFUSE_SDIO_MODE,
    EFUSE_OCR,
    EFUSE_F0CIS,
    EFUSE_F1CIS,
    EFUSE_MAC_ADDR,
    EFUSE_EEPROM_VER,
    EFUSE_CHAN_PLAN,
    EFUSE_TXPW_TAB
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efuse_priv {
    pub id: [u8; 2],
    pub ldo_setting: [u8; 2],
    pub clk_setting: [u8; 2],
    pub cccr: u8,
    pub sdio_mode: u8,
    pub ocr: [u8; 3],
    pub cis0: [u8; 17],
    pub cis1: [u8; 48],
    pub mac_addr: [u8; 6],
    pub eeprom_verno: u8,
    pub channel_plan: u8,
    pub tx_power_b: [u8; 14],
    pub tx_power_g: [u8; 14],
}

extern "C" {
    pub fn read_efuse_byte(hw: *mut ieee80211_hw, _offset: u16, pbuf: *mut u8);
}
extern "C" {
    pub fn efuse_initialize(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn efuse_read_1byte(hw: *mut ieee80211_hw, address: u16) -> u8;
}
extern "C" {
    pub fn efuse_one_byte_read(hw: *mut ieee80211_hw, addr: u16, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn efuse_write_1byte(hw: *mut ieee80211_hw, address: u16, value: u8);
}
extern "C" {
    pub fn efuse_shadow_update(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn efuse_shadow_update_chk(hw: *mut ieee80211_hw) -> bool;
}
extern "C" {
    pub fn rtl_efuse_shadow_map_update(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn efuse_force_write_vendor_id(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn efuse_re_pg_section(hw: *mut ieee80211_hw, section_idx: u8);
}
extern "C" {
    pub fn efuse_power_switch(hw: *mut ieee80211_hw, write: u8, pwrstate: u8);
}
extern "C" {
    pub fn rtl_fill_dummy(pfwbuf: *mut u8, pfwlen: *mut u32);
}
extern "C" {
    pub fn rtl_fw_block_write(hw: *mut ieee80211_hw, buffer: *mut u8, size: u32);
}
extern "C" {
    pub fn rtl_efuse_ops_init(hw: *mut ieee80211_hw);
}
