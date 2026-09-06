//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/tx.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//
pub const TX_HW_MGMT_PKT_LIFETIME_TU: c_int = 2000;
pub const TX_HW_AP_MODE_PKT_LIFETIME_TU: c_int = 8000;

pub const TX_HW_ATTR_OFST_SAVE_RETRIES: c_int = 0;
pub const TX_HW_ATTR_OFST_HEADER_PAD: c_int = 1;
pub const TX_HW_ATTR_OFST_SESSION_COUNTER: c_int = 2;
pub const TX_HW_ATTR_OFST_RATE_POLICY: c_int = 5;
pub const TX_HW_ATTR_OFST_LAST_WORD_PAD: c_int = 10;
pub const TX_HW_ATTR_OFST_TX_CMPLT_REQ: c_int = 12;
pub const TX_HW_RESULT_QUEUE_LEN: c_int = 16;
pub const TX_HW_RESULT_QUEUE_LEN_MASK: c_uint = 0xf;
pub const WL1271_TX_ALIGN_TO: c_int = 4;
pub const WL1271_EXTRA_SPACE_TKIP: c_int = 4;
pub const WL1271_EXTRA_SPACE_AES: c_int = 8;
pub const WL1271_EXTRA_SPACE_MAX: c_int = 8;
// Used for management frames and dummy packets
pub const WL1271_TID_MGMT: c_int = 7;
// stop a ROC for pending authentication reply after this time (ms)
pub const WLCORE_PEND_AUTH_ROC_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl127x_tx_mem {
//
// Number of extra memory blocks to allocate for this packet
// in addition to the number of blocks derived from the packet
// length.
//
    pub extra_blocks: u8,
//
// Total number of memory blocks allocated by the host for
// this packet. Must be equal or greater than the actual
// blocks number allocated by HW.
//
    pub total_mem_blocks: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl128x_tx_mem {
//
// Total number of memory blocks allocated by the host for
// this packet.
//
    pub total_mem_blocks: u8,
//
// Number of extra bytes, at the end of the frame. the host
// uses this padding to complete each frame to integer number
// of SDIO blocks.
//
    pub extra_bytes: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_tx_mem {
//
// Total number of memory blocks allocated by the host for
// this packet.
//
    pub total_mem_blocks: u8,
//
// control bits
//
    pub ctrl: u8,
    pub __packed: },
//
// On wl128x based devices, when TX packets are aggregated, each packet
// size must be aligned to the SDIO block size. The maximum block size
// is bounded by the type of the padded bytes field that is sent to the
// FW. Currently the type is u8, so the maximum block size is 256 bytes.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_tx_hw_descr {
// Length of packet in words, including descriptor+header+data
    pub length: __le16,
    pub wl127x_mem: wl127x_tx_mem,
    pub wl128x_mem: wl128x_tx_mem,
    pub wl18xx_mem: wl18xx_tx_mem,
    pub __packed: },
// Device time (in us) when the packet arrived to the driver
    pub start_time: __le32,
//
// Max delay in TUs until transmission. The last device time the
// packet can be transmitted is: start_time + (1024 * life_time)
//
    pub life_time: __le16,
// Bitwise fields - see TX_ATTR... definitions above.
    pub tx_attr: __le16,
// Packet identifier used also in the Tx-Result.
    pub id: u8,
// The packet TID value (as User-Priority)
    pub tid: u8,
// host link ID (HLID)
    pub hlid: u8,
    pub wl12xx_reserved: u8,
//
// bit 0   -> 0 = udp, 1 = tcp
// bit 1:7 -> IP header offset
//
    pub wl18xx_checksum_data: u8,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_tx_hw_res_status {
    TX_SUCCESS          = 0,
    TX_HW_ERROR         = 1,
    TX_DISABLED         = 2,
    TX_RETRY_EXCEEDED   = 3,
    TX_TIMEOUT          = 4,
    TX_KEY_NOT_FOUND    = 5,
    TX_PEER_NOT_FOUND   = 6,
    TX_SESSION_MISMATCH = 7,
    TX_LINK_NOT_VALID   = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_tx_hw_res_descr {
// Packet Identifier - same value used in the Tx descriptor.
    pub id: u8,
// The status of the transmission, indicating success or one of
    pub status: u8,
// Total air access duration including all retrys and overheads.
    pub medium_usage: __le16,
// The time passed from host xfer to Tx-complete.
    pub fw_handling_time: __le32,
// Total media delay
    pub medium_delay: __le32,
// LS-byte of last TKIP seq-num (saved per AC for recovery).
    pub tx_security_sequence_number_lsb: u8,
// Retry count - number of transmissions without successful ACK.
    pub ack_failures: u8,
// The rate that succeeded getting ACK
    pub rate_class_index: u8,
// for 4-byte alignment.
    pub spare: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_tx_hw_res_if {
    pub tx_result_fw_counter: __le32,
    pub tx_result_host_counter: __le32,
    pub tx_results_queue: [wl1271_tx_hw_res_descr; TX_HW_RESULT_QUEUE_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_queue_stop_reason {
    WLCORE_QUEUE_STOP_REASON_WATERMARK,
    WLCORE_QUEUE_STOP_REASON_FW_RESTART,
    WLCORE_QUEUE_STOP_REASON_FLUSH,
    WLCORE_QUEUE_STOP_REASON_SPARE_BLK, /* 18xx specific */
}

    pub CONF_TX_AC_VO: return,
    pub CONF_TX_AC_VI: return,
    pub CONF_TX_AC_BE: return,
    pub CONF_TX_AC_BK: return,
    pub CONF_TX_AC_BE: return,
    pub wlvif->hw_queue_base: int mac_queue =,
    pub 0: return mac_queue +,
    pub 1: return mac_queue +,
    pub 2: return mac_queue +,
    pub 3: return mac_queue +,
    pub 2: return mac_queue +,
    pub 0: int i, count =,
    pub i++): for (i = 0; i < NUM_TX_QUEUES;,
    pub wl->tx_queue_count[i]: count +=,
    pub count: return,
    pub work): *mut void wl1271_tx_work(struct work_struct,
    pub wl): *mut int wlcore_tx_work_locked(struct wl1271,
    pub wl): *mut int wlcore_tx_complete(struct wl1271,
    pub wlvif): *mut *mut void wl12xx_tx_reset_wlvif(struct wl1271 wl, struct wl12xx_vif,
    pub wl): *mut void wl12xx_tx_reset(struct wl1271,
    pub wl): *mut void wl1271_tx_flush(struct wl1271,
    pub band): *mut *mut u8 wlcore_rate_to_idx(struct wl1271 wl, u8 rate, enum nl80211_band,
    pub rate_band): nl80211_band,
    pub rate_set): *mut *mut u32 wl1271_tx_min_rate_get(struct wl1271 wl, u32,
    pub sta): *mut *mut sk_buff skb, ieee80211_sta,
    pub hlid): *mut *mut void wl1271_tx_reset_link_queues(struct wl1271 wl, u8,
    pub wl): *mut void wl1271_handle_tx_low_watermark(struct wl1271,
    pub skb): *mut *mut bool wl12xx_is_dummy_packet(struct wl1271 wl, struct sk_buff,
    pub active_hlids): *mut *mut void wl12xx_rearm_rx_streaming(struct wl1271 wl, unsigned long,
    pub packet_length): c_uint,
    pub id): *mut *mut void wl1271_free_tx_id(struct wl1271 wl, int,
    pub reason): u8 queue, enum wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub reason): wlcore_queue_stop_reason,
    pub queue): u8,
// from main.c
    pub hlid): *mut *mut *mut void wl1271_free_sta(struct wl1271 wl, struct wl12xx_vif wlvif, u8,
    pub wl): *mut void wl12xx_rearm_tx_watchdog_locked(struct wl1271,
