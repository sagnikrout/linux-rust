//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/btcoex.h
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


//
// Copyright (c) 2009-2011 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const ATH_WLANACTIVE_GPIO_9280: c_int = 5;
pub const ATH_BTACTIVE_GPIO_9280: c_int = 6;
pub const ATH_BTPRIORITY_GPIO_9285: c_int = 7;
pub const ATH_WLANACTIVE_GPIO_9300: c_int = 5;
pub const ATH_BTACTIVE_GPIO_9300: c_int = 4;
pub const ATH_BTPRIORITY_GPIO_9300: c_int = 8;
pub const ATH_BTCOEX_DEF_BT_PERIOD: c_int = 45;
pub const ATH_BTCOEX_DEF_DUTY_CYCLE: c_int = 55;
pub const ATH_BTCOEX_BTSCAN_DUTY_CYCLE: c_int = 90;
pub const ATH_BTCOEX_BMISS_THRESH: c_int = 50;

pub const ATH_BT_CNT_THRESHOLD: c_int = 3;
pub const ATH_BT_CNT_SCAN_THRESHOLD: c_int = 15;
pub const ATH_BTCOEX_RX_WAIT_TIME: c_int = 100;
pub const ATH_BTCOEX_STOMP_FTP_THRESH: c_int = 5;
pub const ATH_BTCOEX_HT20_MAX_TXPOWER: c_uint = 0x14;
pub const ATH_BTCOEX_HT40_MAX_TXPOWER: c_uint = 0x10;
pub const AR9300_NUM_BT_WEIGHTS: c_int = 4;
pub const AR9300_NUM_WLAN_WEIGHTS: c_int = 4;
pub const ATH_AIC_MAX_BT_CHANNEL: c_int = 79;
// Defines the BT AR_BT_COEX_WGHT used
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_stomp_type {
    ATH_BTCOEX_STOMP_ALL,
    ATH_BTCOEX_STOMP_LOW,
    ATH_BTCOEX_STOMP_NONE,
    ATH_BTCOEX_STOMP_LOW_FTP,
    ATH_BTCOEX_STOMP_AUDIO,
    ATH_BTCOEX_STOMP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_btcoex_scheme {
    ATH_BTCOEX_CFG_NONE,
    ATH_BTCOEX_CFG_2WIRE,
    ATH_BTCOEX_CFG_3WIRE,
    ATH_BTCOEX_CFG_MCI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_mci {
    pub raw_intr: u32,
    pub rx_msg_intr: u32,
    pub cont_status: u32,
    pub gpm_addr: u32,
    pub gpm_len: u32,
    pub gpm_idx: u32,
    pub sched_addr: u32,
    pub wlan_channels: [u32; 4],
    pub wlan_cal_seq: u32,
    pub wlan_cal_done: u32,
    pub config: u32,
    pub gpm_buf: *mut u8,
    pub ready: bool,
    pub update_2g5g: bool,
    pub is_2g: bool,
    pub query_bt: bool,
    pub /: *mut *mut bool unhalt_bt_gpm; / need send UNHALT,
    pub /: *mut *mut bool halted_bt_gpm; / HALT sent,
    pub need_flush_btinfo: bool,
    pub bt_version_known: bool,
    pub wlan_channels_update: bool,
    pub wlan_ver_major: u8,
    pub wlan_ver_minor: u8,
    pub bt_ver_major: u8,
    pub bt_ver_minor: u8,
    pub bt_state: u8,
    pub stomp_ftp: u8,
    pub concur_tx: bool,
    pub last_recovery: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_hw_aic {
    pub aic_enabled: bool,
    pub aic_cal_state: u8,
    pub aic_caled_chan: u8,
    pub aic_sram: [u32; ATH_AIC_MAX_BT_CHANNEL],
    pub aic_cal_start_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_btcoex_hw {
    pub scheme: ath_btcoex_scheme,
    pub mci: ath9k_hw_mci,
    pub aic: ath9k_hw_aic,
    pub enabled: bool,
    pub wlanactive_gpio: u8,
    pub btactive_gpio: u8,
    pub btpriority_gpio: u8,
    pub /: *mut *mut u32 bt_coex_mode; / Register setting for AR_BT_COEX_MODE,
    pub /: *mut *mut u32 bt_coex_weights; / Register setting for AR_BT_COEX_WEIGHT,
    pub /: *mut *mut u32 bt_coex_mode2; / Register setting for AR_BT_COEX_MODE2,
    pub /: *mut *mut u32 bt_coex_mode3; / Register setting for AR_BT_COEX_MODE3,
    pub bt_weight: [u32; AR9300_NUM_BT_WEIGHTS],
    pub wlan_weight: [u32; AR9300_NUM_WLAN_WEIGHTS],
    pub tx_prio: [u8; ATH_BTCOEX_STOMP_MAX],
}

extern "C" {
    pub fn ath9k_hw_btcoex_init_scheme(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_init_2wire(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_init_3wire(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_deinit(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_init_mci(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_init_btcoex_hw(ah: *mut ath_hw, qnum: c_int);
}
extern "C" {
    pub fn ath9k_hw_btcoex_disable(ah: *mut ath_hw);
}
extern "C" {
    pub fn ath9k_hw_btcoex_set_concur_txprio(ah: *mut ath_hw, stomp_txprio: *mut u8);
}
