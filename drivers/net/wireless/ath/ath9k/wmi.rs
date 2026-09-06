//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/wmi.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_event_txrate {
    pub txrate: __be32,
    pub rssi_thresh: u8,
    pub per: u8,
    pub rc_stats: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_hdr {
    pub command_id: __be16,
    pub seq_no: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fw_version {
    pub major: __be16,
    pub minor: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_event_swba {
    pub tsf: __be64,
    pub beacon_pending: u8,
    pub __packed: },
//
// 64 - HTC header - WMI header - 1 / txstatus
// And some other hdr. space is also accounted for.
// 12 seems to be the magic number.
//
pub const HTC_MAX_TX_STATUS: c_int = 12;

//
// Legacy rates are indicated as indices.
// HT rates are indicated as dot11 numbers.
// This allows us to resrict the rate field
// to 4 bits.
//
pub const ATH9K_HTC_TXSTAT_RATE: c_uint = 0x0f;
pub const ATH9K_HTC_TXSTAT_RATE_S: c_int = 0;
pub const ATH9K_HTC_TXSTAT_EPID: c_uint = 0xf0;
pub const ATH9K_HTC_TXSTAT_EPID_S: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __wmi_event_txstatus {
    pub cookie: u8,
    pub /: *mut *mut u8 ts_rate; / Also holds EP ID,
    pub ts_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_event_txstatus {
    pub cnt: u8,
    pub txstatus: [__wmi_event_txstatus; HTC_MAX_TX_STATUS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cmd_id {
    WMI_ECHO_CMDID = 0x0001,
    WMI_ACCESS_MEMORY_CMDID,

// Commands to Target
    WMI_GET_FW_VERSION,
    WMI_DISABLE_INTR_CMDID,
    WMI_ENABLE_INTR_CMDID,
    WMI_ATH_INIT_CMDID,
    WMI_ABORT_TXQ_CMDID,
    WMI_STOP_TX_DMA_CMDID,
    WMI_ABORT_TX_DMA_CMDID,
    WMI_DRAIN_TXQ_CMDID,
    WMI_DRAIN_TXQ_ALL_CMDID,
    WMI_START_RECV_CMDID,
    WMI_STOP_RECV_CMDID,
    WMI_FLUSH_RECV_CMDID,
    WMI_SET_MODE_CMDID,
    WMI_NODE_CREATE_CMDID,
    WMI_NODE_REMOVE_CMDID,
    WMI_VAP_REMOVE_CMDID,
    WMI_VAP_CREATE_CMDID,
    WMI_REG_READ_CMDID,
    WMI_REG_WRITE_CMDID,
    WMI_RC_STATE_CHANGE_CMDID,
    WMI_RC_RATE_UPDATE_CMDID,
    WMI_TARGET_IC_UPDATE_CMDID,
    WMI_TX_AGGR_ENABLE_CMDID,
    WMI_TGT_DETACH_CMDID,
    WMI_NODE_UPDATE_CMDID,
    WMI_INT_STATS_CMDID,
    WMI_TX_STATS_CMDID,
    WMI_RX_STATS_CMDID,
    WMI_BITRATE_MASK_CMDID,
    WMI_REG_RMW_CMDID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_event_id {
    WMI_TGT_RDY_EVENTID = 0x1001,
    WMI_SWBA_EVENTID,
    WMI_FATAL_EVENTID,
    WMI_TXTO_EVENTID,
    WMI_BMISS_EVENTID,
    WMI_DELBA_EVENTID,
    WMI_TXSTATUS_EVENTID,
}

pub const MAX_CMD_NUMBER: c_int = 62;
pub const MAX_RMW_CMD_NUMBER: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct register_write {
    pub reg: __be32,
    pub val: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct register_rmw {
    pub reg: __be32,
    pub set: __be32,
    pub clr: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_tx_event {
    pub count: c_int,
    pub txs: __wmi_event_txstatus,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi {
    pub drv_priv: *mut ath9k_htc_priv,
    pub htc: *mut htc_target,
    pub ctrl_epid: htc_endpoint_id,
    pub op_mutex: mutex,
    pub cmd_wait: completion,
    pub last_seq_id: u16,
    pub wmi_event_queue: sk_buff_head,
    pub wmi_event_tasklet: tasklet_struct,
    pub tx_seq_id: u16,
    pub cmd_rsp_buf: *mut u8,
    pub cmd_rsp_len: u32,
    pub stopped: bool,
    pub pending_tx_events: list_head,
    pub event_lock: spinlock_t,
    pub wmi_lock: spinlock_t,
// multi write section
    pub mwrite_cnt: core::sync::atomic::AtomicI32,
    pub multi_write: [register_write; MAX_CMD_NUMBER],
    pub multi_write_idx: u32,
    pub multi_write_mutex: mutex,
// multi rmw section
    pub m_rmw_cnt: core::sync::atomic::AtomicI32,
    pub multi_rmw: [register_rmw; MAX_RMW_CMD_NUMBER],
    pub multi_rmw_idx: u32,
    pub multi_rmw_mutex: mutex,
}

extern "C" {
    pub fn ath9k_wmi_event_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ath9k_fatal_work(work: *mut work_struct);
}
extern "C" {
    pub fn ath9k_wmi_event_drain(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_stop_wmi(priv: *mut ath9k_htc_priv);
}
extern "C" {
    pub fn ath9k_destroy_wmi(priv: *mut ath9k_htc_priv);
}

