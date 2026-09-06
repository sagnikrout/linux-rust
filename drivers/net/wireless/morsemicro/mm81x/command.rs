//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/command.h
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
// Copyright (c) 2017-2026 Morse Micro
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_cmd_return_code {
    MM81X_RET_SUCCESS = 0,
    MM81X_RET_EPERM = -1,
    MM81X_RET_ENOMEM = -12,
    MM81X_RET_CMD_NOT_HANDLED = -32757,
}

pub const HOST_CMD_HOST_ID_SEQ_MAX: c_uint = 0xFFF;
pub const HOST_CMD_HOST_ID_RETRY_MASK: c_uint = 0x000F;
pub const HOST_CMD_HOST_ID_SEQ_SHIFT: c_int = 4;
pub const HOST_CMD_HOST_ID_SEQ_MASK: c_uint = 0xFFF0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_req {
    pub hdr: host_cmd_header,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_resp {
    pub hdr: host_cmd_header,
    pub status: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_event {
    pub hdr: host_cmd_header,
    pub data: [u8; ],
    pub __packed: },
    pub skb): *mut *mut int mm81x_cmd_resp_process(struct mm81x mors, struct sk_buff,
    pub type): nl80211_iftype,
    pub capabilities): *mut mm81x_fw_caps,
    pub params): *mut *mut int mm81x_cmd_cfg_qos(struct mm81x mors, struct mm81x_queue_params,
    pub enabled): bool,
    pub cssid): u16 dtim_period, u32,
    pub power_mbm): *mut i32,
    pub out_power_mbm): *mut *mut int mm81x_cmd_get_max_txpower(struct mm81x mors, s32,
    pub txpower_mbm): c_int,
    pub store): bool,
    pub enabled): *mut *mut int mm81x_cmd_set_ps(struct mm81x mors, bool,
    pub mors_vif): *mut mm81x_vif,
    pub state): ieee80211_sta_state,
    pub length): host_cmd_aes_key_len,
    pub key): *mut u16 aid, struct ieee80211_key_conf,
    pub vif_id): *mut *mut int mm81x_cmd_rm_if(struct mm81x mors, u16,
    pub frag_threshold): *mut *mut int mm81x_cmd_set_frag_threshold(struct mm81x mors, u32,
    pub resp_len): c_uint,
