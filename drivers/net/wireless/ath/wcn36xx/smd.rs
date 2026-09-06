//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/smd.h
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
// Copyright (c) 2013 Eugene Krasnikov <k.eugene.e@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// Max shared size is 4k but we take less.
pub const WCN36XX_NV_FRAGMENT_SIZE: c_int = 3072;
pub const WCN36XX_HAL_BUF_SIZE: c_int = 4096;
pub const HAL_MSG_TIMEOUT: c_int = 10000;
pub const WCN36XX_SMSM_WLAN_TX_ENABLE: c_uint = 0x00000400;
pub const WCN36XX_SMSM_WLAN_TX_RINGS_EMPTY: c_uint = 0x00000200;
// The PNO version info be contained in the rsp msg
pub const WCN36XX_FW_MSG_PNO_VERSION_MASK: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_fw_msg_result {
    WCN36XX_FW_MSG_RESULT_SUCCESS			= 0,
    WCN36XX_FW_MSG_RESULT_SUCCESS_SYNC		= 1,

    WCN36XX_FW_MSG_RESULT_MEM_FAIL			= 5,
}

//
// SMD requests and responses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_fw_msg_status_rsp {
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_ind_msg {
    pub list: list_head,
    pub msg_len: usize,
    pub __counted_by(msg_len): u8 msg[],
}

extern "C" {
    pub fn wcn36xx_smd_open(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_close(wcn: *mut wcn36xx);
}
extern "C" {
    pub fn wcn36xx_smd_load_nv(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_start(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_stop(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_start_scan(wcn: *mut wcn36xx, scan_channel: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_end_scan(wcn: *mut wcn36xx, scan_channel: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_stop_hw_scan(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_update_channel_list(wcn: *mut wcn36xx, req: *mut cfg80211_scan_request) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_add_sta_self(wcn: *mut wcn36xx, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_delete_sta_self(wcn: *mut wcn36xx, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_delete_sta(wcn: *mut wcn36xx, sta_index: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_join(wcn: *mut wcn36xx, bssid: *const u8, vif: *mut u8, ch: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_delete_bss(wcn: *mut wcn36xx, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_enter_bmps(wcn: *mut wcn36xx, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_exit_bmps(wcn: *mut wcn36xx, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_set_power_params(wcn: *mut wcn36xx, ignore_dtim: bool) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_feature_caps_exchange(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_add_ba(wcn: *mut wcn36xx, session_id: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_del_ba(wcn: *mut wcn36xx, tid: u16, direction: u8, sta_index: u8) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_trigger_ba(wcn: *mut wcn36xx, sta_index: u8, tid: u16, ssn: *mut u16) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_update_cfg(wcn: *mut wcn36xx, cfg_id: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_wlan_host_suspend_ind(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_host_resume(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_enter_imps(wcn: *mut wcn36xx) -> c_int;
}
extern "C" {
    pub fn wcn36xx_smd_exit_imps(wcn: *mut wcn36xx) -> c_int;
}
