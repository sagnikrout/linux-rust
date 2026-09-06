//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8723be/fw.h
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
// Copyright(c) 2009-2014  Realtek Corporation.
pub const FW_8192C_SIZE: c_uint = 0x8000;
pub const FW_8192C_START_ADDRESS: c_uint = 0x1000;
pub const FW_8192C_END_ADDRESS: c_uint = 0x5FFF;
pub const FW_8192C_PAGE_SIZE: c_int = 4096;
pub const FW_8192C_POLLING_DELAY: c_int = 5;
pub const USE_OLD_WOWLAN_DEBUG_FW: c_int = 0;
pub const H2C_PWEMODE_LENGTH: c_int = 7;
// Fw PS state for RPWM.
// BIT[2:0] = HW state
// BIT[3] = Protocol PS state, 1: register active state , 0: register sleep state
// BIT[4] = sub-state
//

// 8723BE RPWM value
// BIT[0] = 1: 32k, 0: 40M

// ISR_ENABLE, IMR_ENABLE, and PS mode should be inherited.

// ((FW_PS_RF_ON) | (FW_PS_REGISTER_ACTIVE))

// (FW_PS_RF_ON)

// 0x0

// (FW_PS_STATE_RF_OFF)

// For 8723BE H2C PwrMode Cmd ID 5.

pub const FW_PWR_STATE_RF_OFF: c_int = 0;

pub const FW_PWR_STATE_RF_OFF: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8723b_h2c_cmd {
    H2C_8723B_RSVDPAGE = 0,
    H2C_8723B_MSRRPT = 1,
    H2C_8723B_SCAN = 2,
    H2C_8723B_KEEP_ALIVE_CTRL = 3,
    H2C_8723B_DISCONNECT_DECISION = 4,
    H2C_8723B_BCN_RSVDPAGE = 9,
    H2C_8723B_PROBERSP_RSVDPAGE = 10,

    H2C_8723B_SETPWRMODE = 0x20,
    H2C_8723B_PS_LPS_PARA = 0x23,
    H2C_8723B_P2P_PS_OFFLOAD = 0x24,

    H2C_8723B_RA_MASK = 0x40,
    H2C_RSSIBE_REPORT = 0x42,
// Not defined CTW CMD for P2P yet
    H2C_8723B_P2P_PS_CTW_CMD,
    MAX_8723B_H2CCMD
}

// (u8 *)__ph2ccmd = __val

// (u8 *)(__ph2ccmd + 2) = __val

// (u8 *)(__ph2ccmd + 3) = __val

// (u8 *)(__ph2ccmd + 4) = __val

// (u8 *)(__ph2ccmd + 5) = __val

// (u8 *)(__ph2ccmd) = __val

// (u8 *)(__ph2ccmd + 1) = __val

// (u8 *)(__ph2ccmd + 2) = __val

// (u8 *)(__ph2ccmd + 3) = __val

// (u8 *)(__ph2ccmd + 4) = __val
extern "C" {
    pub fn rtl8723be_set_fw_pwrmode_cmd(hw: *mut ieee80211_hw, mode: u8);
}
extern "C" {
    pub fn rtl8723be_set_fw_media_status_rpt_cmd(hw: *mut ieee80211_hw, mstatus: u8);
}
extern "C" {
    pub fn rtl8723be_set_fw_rsvdpagepkt(hw: *mut ieee80211_hw, b_dl_finished: bool);
}
extern "C" {
    pub fn rtl8723be_set_p2p_ps_offload_cmd(hw: *mut ieee80211_hw, p2p_ps_state: u8);
}
