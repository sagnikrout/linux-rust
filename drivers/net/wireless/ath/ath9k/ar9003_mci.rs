//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/ar9003_mci.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
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
pub const MCI_FLAG_DISABLE_TIMESTAMP: c_uint = 0x00000001      /* Disable time stamp */;

// Default remote BT device MCI COEX version
pub const MCI_GPM_COEX_MAJOR_VERSION_DEFAULT: c_int = 3;
pub const MCI_GPM_COEX_MINOR_VERSION_DEFAULT: c_int = 0;
// Local WLAN MCI COEX version
pub const MCI_GPM_COEX_MAJOR_VERSION_WLAN: c_int = 3;
pub const MCI_GPM_COEX_MINOR_VERSION_WLAN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_gpm_coex_query_type {
    MCI_GPM_COEX_QUERY_BT_ALL_INFO      = BIT(0),
    MCI_GPM_COEX_QUERY_BT_TOPOLOGY      = BIT(1),
    MCI_GPM_COEX_QUERY_BT_DEBUG         = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_gpm_coex_halt_bt_gpm {
    MCI_GPM_COEX_BT_GPM_UNHALT,
    MCI_GPM_COEX_BT_GPM_HALT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_gpm_coex_bt_update_flags_op {
    MCI_GPM_COEX_BT_FLAGS_READ,
    MCI_GPM_COEX_BT_FLAGS_SET,
    MCI_GPM_COEX_BT_FLAGS_CLEAR
}

pub const MCI_NUM_BT_CHANNELS: c_int = 79;
pub const MCI_BT_MCI_FLAGS_UPDATE_CORR: c_uint = 0x00000002;
pub const MCI_BT_MCI_FLAGS_UPDATE_HDR: c_uint = 0x00000004;
pub const MCI_BT_MCI_FLAGS_UPDATE_PLD: c_uint = 0x00000008;
pub const MCI_BT_MCI_FLAGS_LNA_CTRL: c_uint = 0x00000010;
pub const MCI_BT_MCI_FLAGS_DEBUG: c_uint = 0x00000020;
pub const MCI_BT_MCI_FLAGS_SCHED_MSG: c_uint = 0x00000040;
pub const MCI_BT_MCI_FLAGS_CONT_MSG: c_uint = 0x00000080;
pub const MCI_BT_MCI_FLAGS_COEX_GPM: c_uint = 0x00000100;
pub const MCI_BT_MCI_FLAGS_CPU_INT_MSG: c_uint = 0x00000200;
pub const MCI_BT_MCI_FLAGS_MCI_MODE: c_uint = 0x00000400;
pub const MCI_BT_MCI_FLAGS_AR9462_MODE: c_uint = 0x00001000;
pub const MCI_BT_MCI_FLAGS_OTHER: c_uint = 0x00010000;
pub const MCI_DEFAULT_BT_MCI_FLAGS: c_uint = 0x00011dde;

pub const MCI_2G_FLAGS_CLEAR_MASK: c_uint = 0x00000000;

pub const MCI_5G_FLAGS_SET_MASK: c_uint = 0x00000000;

//
// Default value for AR9462 is 0x00002201
//
pub const ATH_MCI_CONFIG_CONCUR_TX: c_uint = 0x00000003;
pub const ATH_MCI_CONFIG_MCI_OBS_MCI: c_uint = 0x00000004;
pub const ATH_MCI_CONFIG_MCI_OBS_TXRX: c_uint = 0x00000008;
pub const ATH_MCI_CONFIG_MCI_OBS_BT: c_uint = 0x00000010;
pub const ATH_MCI_CONFIG_DISABLE_MCI_CAL: c_uint = 0x00000020;
pub const ATH_MCI_CONFIG_DISABLE_OSLA: c_uint = 0x00000040;
pub const ATH_MCI_CONFIG_DISABLE_FTP_STOMP: c_uint = 0x00000080;
pub const ATH_MCI_CONFIG_AGGR_THRESH: c_uint = 0x00000700;
pub const ATH_MCI_CONFIG_AGGR_THRESH_S: c_int = 8;
pub const ATH_MCI_CONFIG_DISABLE_AGGR_THRESH: c_uint = 0x00000800;
pub const ATH_MCI_CONFIG_CLK_DIV: c_uint = 0x00003000;
pub const ATH_MCI_CONFIG_CLK_DIV_S: c_int = 12;
pub const ATH_MCI_CONFIG_DISABLE_TUNING: c_uint = 0x00004000;
pub const ATH_MCI_CONFIG_DISABLE_AIC: c_uint = 0x00008000;
pub const ATH_MCI_CONFIG_AIC_CAL_NUM_CHAN: c_uint = 0x007f0000;
pub const ATH_MCI_CONFIG_AIC_CAL_NUM_CHAN_S: c_int = 16;
pub const ATH_MCI_CONFIG_NO_QUIET_ACK: c_uint = 0x00800000;
pub const ATH_MCI_CONFIG_NO_QUIET_ACK_S: c_int = 23;
pub const ATH_MCI_CONFIG_ANT_ARCH: c_uint = 0x07000000;
pub const ATH_MCI_CONFIG_ANT_ARCH_S: c_int = 24;
pub const ATH_MCI_CONFIG_FORCE_QUIET_ACK: c_uint = 0x08000000;
pub const ATH_MCI_CONFIG_FORCE_QUIET_ACK_S: c_int = 27;
pub const ATH_MCI_CONFIG_FORCE_2CHAIN_ACK: c_uint = 0x10000000;
pub const ATH_MCI_CONFIG_MCI_STAT_DBG: c_uint = 0x20000000;
pub const ATH_MCI_CONFIG_MCI_WEIGHT_DBG: c_uint = 0x40000000;
pub const ATH_MCI_CONFIG_DISABLE_MCI: c_uint = 0x80000000;

pub const ATH_MCI_CONFIG_MCI_OBS_GPIO: c_uint = 0x0000002F;
pub const ATH_MCI_ANT_ARCH_1_ANT_PA_LNA_NON_SHARED: c_uint = 0x00;
pub const ATH_MCI_ANT_ARCH_1_ANT_PA_LNA_SHARED: c_uint = 0x01;
pub const ATH_MCI_ANT_ARCH_2_ANT_PA_LNA_NON_SHARED: c_uint = 0x02;
pub const ATH_MCI_ANT_ARCH_2_ANT_PA_LNA_SHARED: c_uint = 0x03;
pub const ATH_MCI_ANT_ARCH_3_ANT: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_message_header {
    MCI_LNA_CTRL     = 0x10,        /* len = 0 */
    MCI_CONT_NACK    = 0x20,        /* len = 0 */
    MCI_CONT_INFO    = 0x30,        /* len = 4 */
    MCI_CONT_RST     = 0x40,        /* len = 0 */
    MCI_SCHD_INFO    = 0x50,        /* len = 16 */
    MCI_CPU_INT      = 0x60,        /* len = 4 */
    MCI_SYS_WAKING   = 0x70,        /* len = 0 */
    MCI_GPM          = 0x80,        /* len = 16 */
    MCI_LNA_INFO     = 0x90,        /* len = 1 */
    MCI_LNA_STATE    = 0x94,
    MCI_LNA_TAKE     = 0x98,
    MCI_LNA_TRANS    = 0x9c,
    MCI_SYS_SLEEPING = 0xa0,        /* len = 0 */
    MCI_REQ_WAKE     = 0xc0,        /* len = 0 */
    MCI_DEBUG_16     = 0xfe,        /* len = 2 */
    MCI_REMOTE_RESET = 0xff         /* len = 16 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath_mci_gpm_coex_profile_type {
    MCI_GPM_COEX_PROFILE_UNKNOWN,
    MCI_GPM_COEX_PROFILE_RFCOMM,
    MCI_GPM_COEX_PROFILE_A2DP,
    MCI_GPM_COEX_PROFILE_HID,
    MCI_GPM_COEX_PROFILE_BNEP,
    MCI_GPM_COEX_PROFILE_VOICE,
    MCI_GPM_COEX_PROFILE_A2DPVO,
    MCI_GPM_COEX_PROFILE_MAX
}

// MCI GPM/Coex opcode/type definitions
// MCI_GPM_WLAN_CAL_REQ, MCI_GPM_WLAN_CAL_DONE
// MCI_GPM_COEX_VERSION_QUERY
// MCI_GPM_COEX_VERSION_RESPONSE
// MCI_GPM_COEX_STATUS_QUERY
// MCI_GPM_COEX_HALT_BT_GPM
// MCI_GPM_COEX_WLAN_CHANNELS
// MCI_GPM_COEX_BT_PROFILE_INFO
// MCI_GPM_COEX_BT_STATUS_UPDATE
// MCI_GPM_COEX_BT_UPDATE_FLAGS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_gpm_subtype {
    MCI_GPM_BT_CAL_REQ      = 0,
    MCI_GPM_BT_CAL_GRANT    = 1,
    MCI_GPM_BT_CAL_DONE     = 2,
    MCI_GPM_WLAN_CAL_REQ    = 3,
    MCI_GPM_WLAN_CAL_GRANT  = 4,
    MCI_GPM_WLAN_CAL_DONE   = 5,
    MCI_GPM_COEX_AGENT      = 0x0c,
    MCI_GPM_RSVD_PATTERN    = 0xfe,
    MCI_GPM_RSVD_PATTERN32  = 0xfefefefe,
    MCI_GPM_BT_DEBUG        = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_bt_state {
    MCI_BT_SLEEP,
    MCI_BT_AWAKE,
    MCI_BT_CAL_START,
    MCI_BT_CAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_ps_state {
    MCI_PS_DISABLE,
    MCI_PS_ENABLE,
    MCI_PS_ENABLE_OFF,
    MCI_PS_ENABLE_ON
}

// Type of state query
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_state_type {
    MCI_STATE_ENABLE,
    MCI_STATE_INIT_GPM_OFFSET,
    MCI_STATE_CHECK_GPM_OFFSET,
    MCI_STATE_NEXT_GPM_OFFSET,
    MCI_STATE_LAST_GPM_OFFSET,
    MCI_STATE_BT,
    MCI_STATE_SET_BT_SLEEP,
    MCI_STATE_SET_BT_AWAKE,
    MCI_STATE_SET_BT_CAL_START,
    MCI_STATE_SET_BT_CAL,
    MCI_STATE_LAST_SCHD_MSG_OFFSET,
    MCI_STATE_REMOTE_SLEEP,
    MCI_STATE_CONT_STATUS,
    MCI_STATE_RESET_REQ_WAKE,
    MCI_STATE_SEND_WLAN_COEX_VERSION,
    MCI_STATE_SET_BT_COEX_VERSION,
    MCI_STATE_SEND_WLAN_CHANNELS,
    MCI_STATE_SEND_VERSION_QUERY,
    MCI_STATE_SEND_STATUS_QUERY,
    MCI_STATE_NEED_FLUSH_BT_INFO,
    MCI_STATE_SET_CONCUR_TX_PRI,
    MCI_STATE_RECOVER_RX,
    MCI_STATE_NEED_FTP_STOMP,
    MCI_STATE_NEED_TUNING,
    MCI_STATE_NEED_STAT_DEBUG,
    MCI_STATE_SHARED_CHAIN_CONCUR_TX,
    MCI_STATE_AIC_CAL,
    MCI_STATE_AIC_START,
    MCI_STATE_AIC_CAL_RESET,
    MCI_STATE_AIC_CAL_SINGLE,
    MCI_STATE_IS_AR9462,
    MCI_STATE_IS_AR9565_1ANT,
    MCI_STATE_IS_AR9565_2ANT,
    MCI_STATE_WLAN_WEAK_SIGNAL,
    MCI_STATE_SET_WLAN_PS_STATE,
    MCI_STATE_GET_WLAN_PS_STATE,
    MCI_STATE_DEBUG,
    MCI_STATE_STAT_DEBUG,
    MCI_STATE_ALLOW_FCS,
    MCI_STATE_SET_2G_CONTENTION,
    MCI_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mci_gpm_coex_opcode {
    MCI_GPM_COEX_VERSION_QUERY,
    MCI_GPM_COEX_VERSION_RESPONSE,
    MCI_GPM_COEX_STATUS_QUERY,
    MCI_GPM_COEX_HALT_BT_GPM,
    MCI_GPM_COEX_WLAN_CHANNELS,
    MCI_GPM_COEX_BT_PROFILE_INFO,
    MCI_GPM_COEX_BT_STATUS_UPDATE,
    MCI_GPM_COEX_BT_UPDATE_FLAGS,
    MCI_GPM_COEX_NOOP,
}

pub const MCI_GPM_NOMORE: c_int = 0;
pub const MCI_GPM_MORE: c_int = 1;
pub const MCI_GPM_INVALID: c_uint = 0xffffffff;

// (((u32 *)_p_gpm) + MCI_GPM_COEX_W_GPM_PAYLOAD) = \

// (((u8 *)(_p_gpm)) + MCI_GPM_COEX_B_GPM_TYPE) = (_cal_type) & 0xff;\

// (((u8 *)(_p_gpm)) + MCI_GPM_COEX_B_GPM_TYPE) = (_type) & 0xff;	   \
// (((u8 *)(_p_gpm)) + MCI_GPM_COEX_B_GPM_OPCODE) = (_opcode) & 0xff;\

//
// Functions that are available to the MCI driver core.
//
extern "C" {
    pub fn ar9003_mci_state(ah: *mut ath_hw, state_type: u32) -> u32;
}
extern "C" {
    pub fn ar9003_mci_cleanup(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_get_next_gpm_offset(ah: *mut ath_hw, more: *mut u32) -> u32;
}
extern "C" {
    pub fn ar9003_mci_set_bt_version(ah: *mut ath_hw, major: u8, minor: u8);
}
extern "C" {
    pub fn ar9003_mci_send_wlan_channels(ah: *mut ath_hw);
}
//
// These functions are used by ath9k_hw.
//

extern "C" {
    pub fn ar9003_mci_stop_bt(ah: *mut ath_hw, save_fullsleep: bool);
}
extern "C" {
    pub fn ar9003_mci_init_cal_req(ah: *mut ath_hw, is_reusable: *mut bool);
}
extern "C" {
    pub fn ar9003_mci_init_cal_done(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_set_full_sleep(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_2g5g_switch(ah: *mut ath_hw, force: bool);
}
extern "C" {
    pub fn ar9003_mci_check_bt(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_start_reset(ah: *mut ath_hw, chan: *mut ath9k_channel) -> bool;
}
extern "C" {
    pub fn ar9003_mci_get_isr(ah: *mut ath_hw, masked: *mut ath9k_int);
}
extern "C" {
    pub fn ar9003_mci_bt_gain_ctrl(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_set_power_awake(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_check_gpm_offset(ah: *mut ath_hw);
}
extern "C" {
    pub fn ar9003_mci_get_max_txpower(ah: *mut ath_hw, ctlmode: u8) -> u16;
}

