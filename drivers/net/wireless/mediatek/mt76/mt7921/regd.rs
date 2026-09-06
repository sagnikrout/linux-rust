//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/regd.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2025 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_regd_rule_header {
    pub alpha2: [u8; 2],
    pub dfs_region: u8,
    pub rsv: [u8; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_regd_rule {
    pub start_freq: __le32,
    pub end_freq: __le32,
    pub max_bw: __le32,
    pub eirp: __le32,
    pub flags: __le32,
    pub rsv: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_regd_cc {
    pub alpha2: [u8; 2],
    pub ver: u8,
    pub rsv: u8,
    pub n_reg_rules: __le32,
    pub sign_type: u8,
    pub rsv1: [u8; 7],
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_regd_rule_ev {
    pub tag: __le16,
    pub len: __le16,
    pub n_reg_rules: __le32,
    pub dfs_region: u8,
    pub rsv: [u8; 15],
    pub reg_rule: [mt7921_regd_rule; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_regd_query_req {
    pub ver: u8,
    pub sign_type: u8,
    pub rsv1: [u8; 2],
    pub size: __le32,
    pub alpha2: [u8; 2],
    pub rsv2: [u8; 2],
    pub n_reg_rules: __le32,
    pub rsv3: [u8; 64],
    pub data: [u8; ],
}

extern "C" {
    pub fn mt7921_regd_clc_supported(dev: *mut mt792x_dev) -> bool;
}
extern "C" {
    pub fn mt7921_regd_change(phy: *mut mt792x_phy, alpha2: *mut c_char) -> c_int;
}
extern "C" {
    pub fn mt7921_regd_init(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7921_regd_update(phy: *mut mt792x_phy, alpha2: *mut c_char) -> c_int;
}
