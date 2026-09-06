//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/htc.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_tx_flags {
    ATH12K_HTC_FLAG_NEED_CREDIT_UPDATE = 0x01,
    ATH12K_HTC_FLAG_SEND_BUNDLE        = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_rx_flags {
    ATH12K_HTC_FLAG_TRAILER_PRESENT = 0x02,
    ATH12K_HTC_FLAG_BUNDLE_MASK     = 0xF0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_hdr {
    pub htc_info: __le32,
    pub ctrl_info: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_msg_id {
    ATH12K_HTC_MSG_READY_ID                = 1,
    ATH12K_HTC_MSG_CONNECT_SERVICE_ID      = 2,
    ATH12K_HTC_MSG_CONNECT_SERVICE_RESP_ID = 3,
    ATH12K_HTC_MSG_SETUP_COMPLETE_ID       = 4,
    ATH12K_HTC_MSG_SETUP_COMPLETE_EX_ID    = 5,
    ATH12K_HTC_MSG_SEND_SUSPEND_COMPLETE   = 6,
    ATH12K_HTC_MSG_NACK_SUSPEND	       = 7,
    ATH12K_HTC_MSG_WAKEUP_FROM_SUSPEND_ID  = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_version {
    ATH12K_HTC_VERSION_2P0 = 0x00, /* 2.0 */
    ATH12K_HTC_VERSION_2P1 = 0x01, /* 2.1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_conn_flag_threshold_level {
    ATH12K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_FOURTH,
    ATH12K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_HALF,
    ATH12K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_THREE_FOURTHS,
    ATH12K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_UNITY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_conn_svc_status {
    ATH12K_HTC_CONN_SVC_STATUS_SUCCESS      = 0,
    ATH12K_HTC_CONN_SVC_STATUS_NOT_FOUND    = 1,
    ATH12K_HTC_CONN_SVC_STATUS_FAILED       = 2,
    ATH12K_HTC_CONN_SVC_STATUS_NO_RESOURCES = 3,
    ATH12K_HTC_CONN_SVC_STATUS_NO_MORE_EP   = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_ready {
    pub id_credit_count: __le32,
    pub size_ep: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_ready_extended {
    pub base: ath12k_htc_ready,
    pub ver_bundle: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_conn_svc {
    pub msg_svc_id: __le32,
    pub flags_len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_conn_svc_resp {
    pub msg_svc_id: __le32,
    pub flags_len: __le32,
    pub svc_meta_pad: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_setup_complete_extended {
    pub msg_id: __le32,
    pub flags: __le32,
    pub max_msgs_per_bundled_recv: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_msg {
    pub msg_svc_id: __le32,
    pub flags_len: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_record_id {
    ATH12K_HTC_RECORD_NULL    = 0,
    ATH12K_HTC_RECORD_CREDITS = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_record_hdr {
    pub /: *mut *mut u8 id; / @enum ath12k_htc_record_id,
    pub len: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_credit_report {
    pub /: *mut *mut u8 eid; / @enum ath12k_htc_ep_id,
    pub credits: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_record {
    pub hdr: ath12k_htc_record_hdr,
    pub credit_report: [ath12k_htc_credit_report; ],
    pub __aligned(4): } __packed,
// HTC FRAME structure layout draft
//
// note: the trailer offset is dynamic depending
// on payload length. this is only a struct layout draft
//
// =======================================================
//
// HTC HEADER
//
// =======================================================
// |
// HTC message     |        payload
// (variable length)  |    (variable length)
// =======================================================
//
// HTC Record
//
// =======================================================
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_svc_gid {
    ATH12K_HTC_SVC_GRP_RSVD = 0,
    ATH12K_HTC_SVC_GRP_WMI = 1,
    ATH12K_HTC_SVC_GRP_NMI = 2,
    ATH12K_HTC_SVC_GRP_HTT = 3,
    ATH12K_HTC_SVC_GRP_CFG = 4,
    ATH12K_HTC_SVC_GRP_IPA = 5,
    ATH12K_HTC_SVC_GRP_PKTLOG = 6,

    ATH12K_HTC_SVC_GRP_TEST = 254,
    ATH12K_HTC_SVC_GRP_LAST = 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_svc_id {
// NOTE: service ID of 0x0000 is reserved and should never be used
    ATH12K_HTC_SVC_ID_RESERVED	= 0x0000,
    ATH12K_HTC_SVC_ID_UNUSED	= ATH12K_HTC_SVC_ID_RESERVED,

    ATH12K_HTC_SVC_ID_RSVD_CTRL	= SVC(ATH12K_HTC_SVC_GRP_RSVD, 1),
    ATH12K_HTC_SVC_ID_WMI_CONTROL	= SVC(ATH12K_HTC_SVC_GRP_WMI, 0),
    ATH12K_HTC_SVC_ID_WMI_DATA_BE	= SVC(ATH12K_HTC_SVC_GRP_WMI, 1),
    ATH12K_HTC_SVC_ID_WMI_DATA_BK	= SVC(ATH12K_HTC_SVC_GRP_WMI, 2),
    ATH12K_HTC_SVC_ID_WMI_DATA_VI	= SVC(ATH12K_HTC_SVC_GRP_WMI, 3),
    ATH12K_HTC_SVC_ID_WMI_DATA_VO	= SVC(ATH12K_HTC_SVC_GRP_WMI, 4),
    ATH12K_HTC_SVC_ID_WMI_CONTROL_MAC1 = SVC(ATH12K_HTC_SVC_GRP_WMI, 5),
    ATH12K_HTC_SVC_ID_WMI_CONTROL_MAC2 = SVC(ATH12K_HTC_SVC_GRP_WMI, 6),
    ATH12K_HTC_SVC_ID_WMI_CONTROL_DIAG = SVC(ATH12K_HTC_SVC_GRP_WMI, 7),

    ATH12K_HTC_SVC_ID_NMI_CONTROL	= SVC(ATH12K_HTC_SVC_GRP_NMI, 0),
    ATH12K_HTC_SVC_ID_NMI_DATA	= SVC(ATH12K_HTC_SVC_GRP_NMI, 1),

    ATH12K_HTC_SVC_ID_HTT_DATA_MSG	= SVC(ATH12K_HTC_SVC_GRP_HTT, 0),

// raw stream service (i.e. flash, tcmd, calibration apps)
    ATH12K_HTC_SVC_ID_TEST_RAW_STREAMS = SVC(ATH12K_HTC_SVC_GRP_TEST, 0),
    ATH12K_HTC_SVC_ID_IPA_TX = SVC(ATH12K_HTC_SVC_GRP_IPA, 0),
    ATH12K_HTC_SVC_ID_PKT_LOG = SVC(ATH12K_HTC_SVC_GRP_PKTLOG, 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_htc_ep_id {
    ATH12K_HTC_EP_UNUSED = -1,
    ATH12K_HTC_EP_0 = 0,
    ATH12K_HTC_EP_1 = 1,
    ATH12K_HTC_EP_2,
    ATH12K_HTC_EP_3,
    ATH12K_HTC_EP_4,
    ATH12K_HTC_EP_5,
    ATH12K_HTC_EP_6,
    ATH12K_HTC_EP_7,
    ATH12K_HTC_EP_8,
    ATH12K_HTC_EP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_ep_ops {
    pub skb): *mut *mut *mut void (ep_tx_complete)(struct ath12k_base ab, struct sk_buff,
    pub skb): *mut *mut *mut void (ep_rx_complete)(struct ath12k_base ab, struct sk_buff,
    pub ab): *mut *mut void (ep_tx_credits)(struct ath12k_base,
}

// service connection information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_svc_conn_req {
    pub service_id: u16,
    pub ep_ops: ath12k_htc_ep_ops,
    pub max_send_queue_depth: c_int,
}

// service connection response information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_svc_conn_resp {
    pub buffer_len: u8,
    pub actual_len: u8,
    pub eid: ath12k_htc_ep_id,
    pub max_msg_len: c_uint,
    pub connect_resp_code: u8,
}

pub const ATH12K_NUM_CONTROL_TX_BUFFERS: c_int = 2;
pub const ATH12K_HTC_MAX_LEN: c_int = 4096;
pub const ATH12K_HTC_MAX_CTRL_MSG_LEN: c_int = 256;

pub const ATH12K_HTC_MAX_SERVICE_ALLOC_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_ep {
    pub htc: *mut ath12k_htc,
    pub eid: ath12k_htc_ep_id,
    pub service_id: ath12k_htc_svc_id,
    pub ep_ops: ath12k_htc_ep_ops,
    pub max_tx_queue_depth: c_int,
    pub max_ep_message_len: c_int,
    pub ul_pipe_id: u8,
    pub dl_pipe_id: u8,
    pub /: *mut *mut u8 seq_no; / for debugging,
    pub tx_credits: c_int,
    pub tx_credit_flow_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc_svc_tx_credits {
    pub service_id: u16,
    pub credit_allocation: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htc {
    pub ab: *mut ath12k_base,
    pub endpoint: [ath12k_htc_ep; ATH12K_HTC_EP_COUNT],
// protects endpoints
    pub tx_lock: spinlock_t,
    pub control_resp_buffer: [u8; ATH12K_HTC_MAX_CTRL_MSG_LEN],
    pub control_resp_len: c_int,
    pub ctl_resp: completion,
    pub total_transmit_credits: c_int,
    pub target_credit_size: c_int,
    pub wmi_ep_count: u8,
}

extern "C" {
    pub fn ath12k_htc_init(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_htc_wait_target(htc: *mut ath12k_htc) -> c_int;
}
extern "C" {
    pub fn ath12k_htc_start(htc: *mut ath12k_htc) -> c_int;
}
