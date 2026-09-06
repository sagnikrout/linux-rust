//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw88261.h
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
// aw88261.h  --  AW88261 ALSA SoC Audio driver
//
// Copyright (c) 2023 awinic Technology CO., LTD
//
// Author: Jimmy Zhang <zhangjianming@awinic.com>
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

// BST_LOOPR bit 1:0 (BSTCTRL6 0x65)

// RSQN_DLY bit 15:14 (BSTCTRL7 0x66)

// BURST_SSMODE bit 3 (BSTCTRL8 0x67)

// BST_BURST bit 9:7 (BSTCTRL9 0x68)

// NOTE: 192000 has a reg value donwstream but not listed in datasheet

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aw88261_id {
    AW88261_CHIP_ID = 0x2113,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw88261 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub efuse_check: c_int,
    pub frcset_en: c_int,
    pub mute_st: c_uint,
    pub amppd_st: c_uint,
    pub sr_value: c_uint,
    pub cco_mux_value: c_uint,
    pub fs_value: c_uint,
    pub bck_value: c_uint,
    pub bck_inv_value: c_uint,
    pub tdm_bck_value: c_uint,
    pub md_value: c_uint,
    pub slot_num_value: c_uint,
    pub tx_slotvld_mask: c_uint,
    pub rxl_slotvld_mask: c_uint,
    pub rxr_slotvld_mask: c_uint,
    pub phase_sync: bool,
}
