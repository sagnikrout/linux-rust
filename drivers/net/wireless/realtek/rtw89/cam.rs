//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/cam.h
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

pub const RTW89_SEC_CAM_LEN: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_addr_cam_v0 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_addr_cam {
    pub v0: rtw89_h2c_addr_cam_v0,
    pub w15: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_dctlinfo_ud_v1 {
    pub c0: __le32,
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub m0: __le32,
    pub m1: __le32,
    pub m2: __le32,
    pub m3: __le32,
    pub m4: __le32,
    pub m5: __le32,
    pub m6: __le32,
    pub m7: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_dctlinfo_ud_v2 {
    pub c0: __le32,
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub m0: __le32,
    pub m1: __le32,
    pub m2: __le32,
    pub m3: __le32,
    pub m4: __le32,
    pub m5: __le32,
    pub m6: __le32,
    pub m7: __le32,
    pub m8: __le32,
    pub m9: __le32,
    pub m10: __le32,
    pub m11: __le32,
    pub m12: __le32,
    pub m13: __le32,
    pub m14: __le32,
    pub m15: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_h2c_dctlinfo_ud_v3 {
    pub c0: __le32,
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub w10: __le32,
    pub w11: __le32,
    pub w12: __le32,
    pub w13: __le32,
    pub w14: __le32,
    pub w15: __le32,
    pub m0: __le32,
    pub m1: __le32,
    pub m2: __le32,
    pub m3: __le32,
    pub m4: __le32,
    pub m5: __le32,
    pub m6: __le32,
    pub m7: __le32,
    pub m8: __le32,
    pub m9: __le32,
    pub m10: __le32,
    pub m11: __le32,
    pub m12: __le32,
    pub m13: __le32,
    pub m14: __le32,
    pub m15: __le32,
    pub __packed: },

    pub vif): *mut *mut int rtw89_cam_init(struct rtw89_dev rtwdev, struct rtw89_vif_link,
    pub vif): *mut *mut void rtw89_cam_deinit(struct rtw89_dev rtwdev, struct rtw89_vif_link,
    pub bssid_cam): *const rtw89_bssid_cam_entry,
    pub addr_cam): *mut rtw89_addr_cam_entry,
    pub bssid): *const u8,
    pub bssid_cam): *mut rtw89_bssid_cam_entry,
    pub h2c): *mut rtw89_h2c_addr_cam_v0,
    pub h2c): *mut rtw89_h2c_dctlinfo_ud_v1,
    pub h2c): *mut rtw89_h2c_dctlinfo_ud_v2,
    pub h2c): *mut rtw89_h2c_dctlinfo_ud_v3,
    pub h2c): *mut rtw89_h2c_addr_cam_v0,
    pub key): *mut ieee80211_key_conf,
    pub inform_fw): bool,
    pub rtwvif_link): *mut rtw89_vif_link,
    pub rtwdev): *mut void rtw89_cam_reset_keys(struct rtw89_dev,
    pub sec_cam_idx): u8,
