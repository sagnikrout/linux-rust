//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/skbq.h
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

// Sync value of skb header to indicate a valid skb

// Sync value indicating that the chip owns this skb

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_tx_status_and_conf_flags {
    MM81X_TX_STATUS_FLAGS_NO_ACK = BIT(0),
    MM81X_TX_STATUS_FLAGS_NO_REPORT = BIT(1),
    MM81X_TX_CONF_FLAGS_CTL_AMPDU = BIT(2),
    MM81X_TX_CONF_FLAGS_HW_ENCRYPT = BIT(3),
    MM81X_TX_CONF_FLAGS_VIF_ID = (BIT(4) | BIT(5) | BIT(6) | BIT(7) |
    BIT(8) | BIT(9) | BIT(10) | BIT(11)),
    MM81X_TX_CONF_FLAGS_KEY_IDX = (BIT(12) | BIT(13) | BIT(14)),
    MM81X_TX_STATUS_FLAGS_PS_FILTERED = (BIT(15)),
    MM81X_TX_CONF_IGNORE_TWT = (BIT(16)),
    MM81X_TX_STATUS_PAGE_INVALID = (BIT(17)),
    MM81X_TX_CONF_NO_PS_BUFFER = (BIT(18)),
    MM81X_TX_STATUS_DUTY_CYCLE_CANT_SEND = (BIT(19)),
    MM81X_TX_CONF_HAS_PV1_BPN_IN_BODY = (BIT(21)),
    MM81X_TX_CONF_FLAGS_SEND_AFTER_DTIM = (BIT(22)),
    MM81X_TX_STATUS_WAS_AGGREGATED = (BIT(23)),
    MM81X_TX_CONF_FLAGS_FULLMAC_REPORT = BIT(24),
    MM81X_TX_CONF_FLAGS_IMMEDIATE_REPORT = (BIT(31))
}

// Getter and setter macros for vif id

// Getter and setter macros for key index

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_rx_status_flags {
    MM81X_RX_STATUS_FLAGS_ERROR = BIT(0),
    MM81X_RX_STATUS_FLAGS_DECRYPTED = BIT(1),
    MM81X_RX_STATUS_FLAGS_FCS_INCLUDED = BIT(2),
    MM81X_RX_STATUS_FLAGS_EOF = BIT(3),
    MM81X_RX_STATUS_FLAGS_AMPDU = BIT(4),
    MM81X_RX_STATUS_FLAGS_NDP = BIT(7),
    MM81X_RX_STATUS_FLAGS_UPLINK = BIT(8),
    MM81X_RX_STATUS_FLAGS_RI = (BIT(9) | BIT(10)),
    MM81X_RX_STATUS_FLAGS_NDP_TYPE = (BIT(11) | BIT(12) | BIT(13)),
    MM81X_RX_STATUS_FLAGS_CRC_ERROR = BIT(14),
    MM81X_RX_STATUS_FLAGS_VIF_ID = GENMASK(24, 17),
}

// Getter and Setter macros for vif id

// Getter macro for guard interval

// Getter macro for response indication

// Getter macro for NDP type

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_skb_channel {
    MM81X_SKB_CHAN_DATA = 0x0,
    MM81X_SKB_CHAN_NDP_FRAMES = 0x1,
    MM81X_SKB_CHAN_DATA_NOACK = 0x2,
    MM81X_SKB_CHAN_BEACON = 0x3,
    MM81X_SKB_CHAN_MGMT = 0x4,
    MM81X_SKB_CHAN_INTERNAL_CRIT_BEACON = 0x80,
    MM81X_SKB_CHAN_COMMAND = 0xFE,
    MM81X_SKB_CHAN_TX_STATUS = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skb_rate_info {
    pub mm81x_ratecode: mm81x_rate_code_t,
    pub count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skb_tx_status {
    pub flags: __le32,
    pub pkt_id: __le32,
    pub tid: u8,
    pub channel: u8,
    pub ampdu_info: __le16,
    pub rates: [mm81x_skb_rate_info; MM81X_SKB_MAX_RATES],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skb_tx_info {
    pub flags: __le32,
    pub pkt_id: __le32,
    pub tid: u8,
    pub tid_params: u8,
    pub mmss_params: u8,
    pub padding: [u8; 1],
    pub rates: [mm81x_skb_rate_info; MM81X_SKB_MAX_RATES],
    pub __packed: },
pub const TX_INFO_TID_PARAMS_MAX_REORDER_BUF: c_uint = 0x1f;
pub const TX_INFO_TID_PARAMS_AMPDU_ENABLED: c_uint = 0x20;
pub const TX_INFO_TID_PARAMS_AMSDU_SUPPORTED: c_uint = 0x40;
pub const TX_INFO_TID_PARAMS_USE_LEGACY_BA: c_uint = 0x80;
// Bitmap for MMSS (Minimum MPDU start spacing) parameters
// +-----------+-----------+
// | Morse     | MMSS set  |
// | MMSS      | by S1G cap|
// | offset    | IE        |
// |-----------|-----------|
// |b7|b6|b5|b4|b3|b2|b1|b0|
//

pub const TX_INFO_MMSS_PARAMS_MMSS_OFFSET_START: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skb_rx_status {
    pub flags: __le32,
    pub mm81x_ratecode: mm81x_rate_code_t,
    pub rssi: __le16,
    pub freq_100khz: __le16,
    pub bss_color: u8,
    pub noise_dbm: i8,
// Padding for word alignment
    pub padding: [u8; 2],
    pub rx_timestamp_us: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skb_hdr {
    pub sync: u8,
    pub channel: u8,
    pub len: __le16,
    pub offset: u8,
    pub checksum_lower: u8,
    pub checksum_upper: __le16,
    pub tx_info: mm81x_skb_tx_info,
    pub tx_status: mm81x_skb_tx_status,
    pub rx_status: mm81x_skb_rx_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_skbq {
    pub mors: *mut mm81x,
    pub /: *mut *mut u32 pkt_seq; / SKB sequence used in tx_status,
    pub flags: u16,
    pub /: *mut *mut u32 skbq_size; / current off loaded size,
    pub lock: spinlock_t,
    pub skbq: sk_buff_head,
    pub /: *mut *mut sk_buff_head pending; / packets sent pending feedback,
    pub dispatch_work: work_struct,
}

extern "C" {
    pub fn mm81x_skbq_purge(mq: *mut mm81x_skbq, skbq: *mut sk_buff_head);
}
extern "C" {
    pub fn mm81x_skbq_purge_aged(mors: *mut mm81x, mq: *mut mm81x_skbq);
}
extern "C" {
    pub fn mm81x_skbq_space(mq: *mut mm81x_skbq) -> u32;
}
extern "C" {
    pub fn mm81x_skbq_size(mq: *mut mm81x_skbq) -> u32;
}
extern "C" {
    pub fn mm81x_skbq_put(mq: *mut mm81x_skbq, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mm81x_skbq_enq(mq: *mut mm81x_skbq, skbq: *mut sk_buff_head);
}
extern "C" {
    pub fn mm81x_skbq_enq_prepend(mq: *mut mm81x_skbq, skbq: *mut sk_buff_head);
}
extern "C" {
    pub fn mm81x_skbq_tx_complete(mq: *mut mm81x_skbq, skbq: *mut sk_buff_head);
}
extern "C" {
    pub fn mm81x_skbq_init(mors: *mut mm81x, mq: *mut mm81x_skbq, flags: u16);
}
extern "C" {
    pub fn mm81x_skbq_finish(mq: *mut mm81x_skbq);
}
extern "C" {
    pub fn mm81x_skbq_pull_hdr_post_tx(skb: *mut sk_buff);
}
extern "C" {
    pub fn mm81x_skbq_mon_dump(mors: *mut mm81x, file: *mut seq_file);
}
extern "C" {
    pub fn mm81x_skbq_tx_flush(mq: *mut mm81x_skbq);
}
extern "C" {
    pub fn mm81x_skbq_check_for_stale_tx(mors: *mut mm81x, mq: *mut mm81x_skbq) -> c_int;
}
extern "C" {
    pub fn mm81x_skbq_may_wake_tx_queues(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_skbq_count_tx_ready(mq: *mut mm81x_skbq) -> u32;
}
extern "C" {
    pub fn mm81x_skbq_count(mq: *mut mm81x_skbq) -> u32;
}
extern "C" {
    pub fn mm81x_skbq_pending_count(mq: *mut mm81x_skbq) -> u32;
}
extern "C" {
    pub fn mm81x_skbq_data_traffic_pause(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_skbq_data_traffic_resume(mors: *mut mm81x);
}
extern "C" {
    pub fn mm81x_skbq_validate_checksum(data: *mut u8) -> bool;
}
