//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/rx.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_rx_desc_enc {
    RX_DESC_ENC_NONE	= 0,
    RX_DESC_ENC_WEP40	= 1,
    RX_DESC_ENC_TKIP_WO_MIC	= 2,
    RX_DESC_ENC_TKIP_MIC	= 3,
    RX_DESC_ENC_AES		= 4,
    RX_DESC_ENC_WEP104	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_rx_desc {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub __packed: },

    pub skb): *mut sk_buff,
    pub rx_status): *mut ieee80211_rx_status,
    pub pkt_stat): *mut rtw_rx_pkt_stat,
    pub pkt_stat): rtw_update_rx_freq_from_ie(rtwdev, skb, rx_status,,
