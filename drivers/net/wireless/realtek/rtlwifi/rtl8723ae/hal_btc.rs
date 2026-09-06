//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723ae/hal_btc.h
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

pub const BT_TXRX_CNT_THRES_1: c_int = 1200;
pub const BT_TXRX_CNT_THRES_2: c_int = 1400;
pub const BT_TXRX_CNT_THRES_3: c_int = 3000;
// < 1200
pub const BT_TXRX_CNT_LEVEL_0: c_int = 0;
// >= 1200 && < 1400
pub const BT_TXRX_CNT_LEVEL_1: c_int = 1;
// >= 1400
pub const BT_TXRX_CNT_LEVEL_2: c_int = 2;
pub const BT_TXRX_CNT_LEVEL_3: c_int = 3;
pub const BT_COEX_DISABLE: c_int = 0;
pub const BT_Q_PKT_OFF: c_int = 0;
pub const BT_Q_PKT_ON: c_int = 1;
pub const BT_TX_PWR_OFF: c_int = 0;
pub const BT_TX_PWR_ON: c_int = 1;
// TDMA mode definition
pub const TDMA_2ANT: c_int = 0;
pub const TDMA_1ANT: c_int = 1;
pub const TDMA_NAV_OFF: c_int = 0;
pub const TDMA_NAV_ON: c_int = 1;
pub const TDMA_DAC_SWING_OFF: c_int = 0;
pub const TDMA_DAC_SWING_ON: c_int = 1;
// PTA mode related definition
pub const BT_PTA_MODE_OFF: c_int = 0;
pub const BT_PTA_MODE_ON: c_int = 1;
// Penalty Tx Rate Adaptive
pub const BT_TX_RATE_ADAPTIVE_NORMAL: c_int = 0;
pub const BT_TX_RATE_ADAPTIVE_LOW_PENALTY: c_int = 1;
// RF Corner
pub const BT_RF_RX_LPF_CORNER_RESUME: c_int = 0;
pub const BT_RF_RX_LPF_CORNER_SHRINK: c_int = 1;
pub const C2H_EVT_HOST_CLOSE: c_uint = 0x00;
pub const C2H_EVT_FW_CLOSE: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_traffic_mode {
    BT_MOTOR_EXT_BE = 0x00,
    BT_MOTOR_EXT_GUL = 0x01,
    BT_MOTOR_EXT_GUB = 0x02,
    BT_MOTOR_EXT_GULB = 0x03
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_traffic_mode_profile {
    BT_PROFILE_NONE,
    BT_PROFILE_A2DP,
    BT_PROFILE_PAN,
    BT_PROFILE_HID,
    BT_PROFILE_SCO
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hci_ext_bt_operation {
    HCI_BT_OP_NONE = 0x0,
    HCI_BT_OP_INQUIRE_START	= 0x1,
    HCI_BT_OP_INQUIRE_FINISH = 0x2,
    HCI_BT_OP_PAGING_START = 0x3,
    HCI_BT_OP_PAGING_SUCCESS = 0x4,
    HCI_BT_OP_PAGING_UNSUCCESS = 0x5,
    HCI_BT_OP_PAIRING_START = 0x6,
    HCI_BT_OP_PAIRING_FINISH = 0x7,
    HCI_BT_OP_BT_DEV_ENABLE = 0x8,
    HCI_BT_OP_BT_DEV_DISABLE = 0x9,
    HCI_BT_OP_MAX,
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_spec {
    BT_SPEC_1_0_b = 0x00,
    BT_SPEC_1_1 = 0x01,
    BT_SPEC_1_2 = 0x02,
    BT_SPEC_2_0_EDR = 0x03,
    BT_SPEC_2_1_EDR = 0x04,
    BT_SPEC_3_0_HS = 0x05,
    BT_SPEC_4_0 = 0x06
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2h_evt_hdr {
    pub cmd_id: u8,
    pub cmd_len: u8,
    pub cmd_seq: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_state {
    BT_INFO_STATE_DISABLED = 0,
    BT_INFO_STATE_NO_CONNECTION = 1,
    BT_INFO_STATE_CONNECT_IDLE = 2,
    BT_INFO_STATE_INQ_OR_PAG = 3,
    BT_INFO_STATE_ACL_ONLY_BUSY = 4,
    BT_INFO_STATE_SCO_ONLY_BUSY = 5,
    BT_INFO_STATE_ACL_SCO_BUSY = 6,
    BT_INFO_STATE_HID_BUSY = 7,
    BT_INFO_STATE_HID_SCO_BUSY = 8,
    BT_INFO_STATE_MAX = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8723e_c2h_evt_v0 {
    C2H_V0_DBG = 0,
    C2H_V0_TSF = 1,
    C2H_V0_AP_RPT_RSP = 2,
// The FW notify the report of the specific tx packet.
    C2H_V0_CCX_TX_RPT = 3,
    C2H_V0_BT_RSSI = 4,
    C2H_V0_BT_OP_MODE = 5,
    C2H_V0_HW_INFO_EXCH = 10,
    C2H_V0_C2H_H2C_TEST = 11,
    C2H_V0_BT_INFO = 12,
    MAX_C2HEVENT
}

extern "C" {
    pub fn rtl8723e_dm_bt_fw_coex_all_off_8723a(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_sw_coex_all_off_8723a(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_hw_coex_all_off_8723a(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8723e_dm_bt_coexist_8723(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl_8723e_c2h_command_handle(hw: *mut ieee80211_hw);
}
