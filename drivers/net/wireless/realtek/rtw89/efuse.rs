//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/efuse.h
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
// Copyright(c) 2019-2020  Realtek Corporation
//

pub const RTW89_EFUSE_MAX_BLOCK_SIZE: c_uint = 0x10000;
pub const EF_FV_OFSET: c_uint = 0x5EA;
pub const EF_FV_OFSET_BE_V1: c_uint = 0x17CA;

pub const EF_CV_INV: c_int = 15;
pub const EFUSE_THERMAL_K_OFFSET_BE: c_uint = 0x17CC;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_efuse_block_cfg {
    pub offset: u32,
    pub size: u32,
}

extern "C" {
    pub fn rtw89_parse_efuse_map_ax(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_parse_phycap_map_ax(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_cnv_efuse_state_ax(rtwdev: *mut rtw89_dev, idle: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_parse_efuse_map_be(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_parse_phycap_map_be(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_cnv_efuse_state_be(rtwdev: *mut rtw89_dev, idle: bool) -> c_int;
}
extern "C" {
    pub fn rtw89_read_efuse_ver(rtwdev: *mut rtw89_dev, efv: *mut u8) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_recognize_mss_info_v1(rtwdev: *mut rtw89_dev, b1: u8, b2: u8) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_read_fw_secure_ax(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_read_fw_secure_be(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_read_ecv_be(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_read_thermal_k_be(rtwdev: *mut rtw89_dev) -> c_int;
}
extern "C" {
    pub fn rtw89_efuse_read_pwr_data_be(rtwdev: *mut rtw89_dev) -> c_int;
}
