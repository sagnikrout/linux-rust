//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath11k/htc.h
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
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_tx_flags {
    ATH11K_HTC_FLAG_NEED_CREDIT_UPDATE = 0x01,
    ATH11K_HTC_FLAG_SEND_BUNDLE        = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_rx_flags {
    ATH11K_HTC_FLAG_TRAILER_PRESENT = 0x02,
    ATH11K_HTC_FLAG_BUNDLE_MASK     = 0xF0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_hdr {
    pub htc_info: u32,
    pub ctrl_info: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_msg_id {
    ATH11K_HTC_MSG_READY_ID                = 1,
    ATH11K_HTC_MSG_CONNECT_SERVICE_ID      = 2,
    ATH11K_HTC_MSG_CONNECT_SERVICE_RESP_ID = 3,
    ATH11K_HTC_MSG_SETUP_COMPLETE_ID       = 4,
    ATH11K_HTC_MSG_SETUP_COMPLETE_EX_ID    = 5,
    ATH11K_HTC_MSG_SEND_SUSPEND_COMPLETE   = 6,
    ATH11K_HTC_MSG_NACK_SUSPEND	       = 7,
    ATH11K_HTC_MSG_WAKEUP_FROM_SUSPEND_ID  = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_version {
    ATH11K_HTC_VERSION_2P0 = 0x00, /* 2.0 */
    ATH11K_HTC_VERSION_2P1 = 0x01, /* 2.1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_conn_flags {
    ATH11K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_FOURTH    = 0x0,
    ATH11K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_HALF      = 0x1,
    ATH11K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_THREE_FOURTHS = 0x2,
    ATH11K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_UNITY         = 0x3,
    ATH11K_HTC_CONN_FLAGS_REDUCE_CREDIT_DRIBBLE	    = 0x4,
    ATH11K_HTC_CONN_FLAGS_DISABLE_CREDIT_FLOW_CTRL	    = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_conn_svc_status {
    ATH11K_HTC_CONN_SVC_STATUS_SUCCESS      = 0,
    ATH11K_HTC_CONN_SVC_STATUS_NOT_FOUND    = 1,
    ATH11K_HTC_CONN_SVC_STATUS_FAILED       = 2,
    ATH11K_HTC_CONN_SVC_STATUS_NO_RESOURCES = 3,
    ATH11K_HTC_CONN_SVC_STATUS_NO_MORE_EP   = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_ready {
    pub id_credit_count: u32,
    pub size_ep: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_ready_extended {
    pub base: ath11k_htc_ready,
    pub ver_bundle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_conn_svc {
    pub msg_svc_id: u32,
    pub flags_len: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_conn_svc_resp {
    pub msg_svc_id: u32,
    pub flags_len: u32,
    pub svc_meta_pad: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_setup_complete_extended {
    pub msg_id: u32,
    pub flags: u32,
    pub max_msgs_per_bundled_recv: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_msg {
    pub msg_svc_id: u32,
    pub flags_len: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_record_id {
    ATH11K_HTC_RECORD_NULL    = 0,
    ATH11K_HTC_RECORD_CREDITS = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_record_hdr {
    pub /: *mut *mut u8 id; / @enum ath11k_htc_record_id,
    pub len: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_credit_report {
    pub /: *mut *mut u8 eid; / @enum ath11k_htc_ep_id,
    pub credits: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_record {
    pub hdr: ath11k_htc_record_hdr,
    pub credit_report: [ath11k_htc_credit_report; ],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_svc_gid {
    ATH11K_HTC_SVC_GRP_RSVD = 0,
    ATH11K_HTC_SVC_GRP_WMI = 1,
    ATH11K_HTC_SVC_GRP_NMI = 2,
    ATH11K_HTC_SVC_GRP_HTT = 3,
    ATH11K_HTC_SVC_GRP_CFG = 4,
    ATH11K_HTC_SVC_GRP_IPA = 5,
    ATH11K_HTC_SVC_GRP_PKTLOG = 6,

    ATH11K_HTC_SVC_GRP_TEST = 254,
    ATH11K_HTC_SVC_GRP_LAST = 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_svc_id {
// NOTE: service ID of 0x0000 is reserved and should never be used
    ATH11K_HTC_SVC_ID_RESERVED	= 0x0000,
    ATH11K_HTC_SVC_ID_UNUSED	= ATH11K_HTC_SVC_ID_RESERVED,

    ATH11K_HTC_SVC_ID_RSVD_CTRL	= SVC(ATH11K_HTC_SVC_GRP_RSVD, 1),
    ATH11K_HTC_SVC_ID_WMI_CONTROL	= SVC(ATH11K_HTC_SVC_GRP_WMI, 0),
    ATH11K_HTC_SVC_ID_WMI_DATA_BE	= SVC(ATH11K_HTC_SVC_GRP_WMI, 1),
    ATH11K_HTC_SVC_ID_WMI_DATA_BK	= SVC(ATH11K_HTC_SVC_GRP_WMI, 2),
    ATH11K_HTC_SVC_ID_WMI_DATA_VI	= SVC(ATH11K_HTC_SVC_GRP_WMI, 3),
    ATH11K_HTC_SVC_ID_WMI_DATA_VO	= SVC(ATH11K_HTC_SVC_GRP_WMI, 4),
    ATH11K_HTC_SVC_ID_WMI_CONTROL_MAC1 = SVC(ATH11K_HTC_SVC_GRP_WMI, 5),
    ATH11K_HTC_SVC_ID_WMI_CONTROL_MAC2 = SVC(ATH11K_HTC_SVC_GRP_WMI, 6),

    ATH11K_HTC_SVC_ID_NMI_CONTROL	= SVC(ATH11K_HTC_SVC_GRP_NMI, 0),
    ATH11K_HTC_SVC_ID_NMI_DATA	= SVC(ATH11K_HTC_SVC_GRP_NMI, 1),

    ATH11K_HTC_SVC_ID_HTT_DATA_MSG	= SVC(ATH11K_HTC_SVC_GRP_HTT, 0),

// raw stream service (i.e. flash, tcmd, calibration apps)
    ATH11K_HTC_SVC_ID_TEST_RAW_STREAMS = SVC(ATH11K_HTC_SVC_GRP_TEST, 0),
    ATH11K_HTC_SVC_ID_IPA_TX = SVC(ATH11K_HTC_SVC_GRP_IPA, 0),
    ATH11K_HTC_SVC_ID_PKT_LOG = SVC(ATH11K_HTC_SVC_GRP_PKTLOG, 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath11k_htc_ep_id {
    ATH11K_HTC_EP_UNUSED = -1,
    ATH11K_HTC_EP_0 = 0,
    ATH11K_HTC_EP_1 = 1,
    ATH11K_HTC_EP_2,
    ATH11K_HTC_EP_3,
    ATH11K_HTC_EP_4,
    ATH11K_HTC_EP_5,
    ATH11K_HTC_EP_6,
    ATH11K_HTC_EP_7,
    ATH11K_HTC_EP_8,
    ATH11K_HTC_EP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_ep_ops {
    pub ): *mut *mut *mut void (ep_tx_complete)(struct ath11k_base , struct sk_buff,
    pub ): *mut *mut *mut void (ep_rx_complete)(struct ath11k_base , struct sk_buff,
    pub ): *mut *mut void (ep_tx_credits)(struct ath11k_base,
}

// service connection information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_svc_conn_req {
    pub service_id: u16,
    pub ep_ops: ath11k_htc_ep_ops,
    pub max_send_queue_depth: c_int,
}

// service connection response information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_svc_conn_resp {
    pub buffer_len: u8,
    pub actual_len: u8,
    pub eid: ath11k_htc_ep_id,
    pub max_msg_len: c_uint,
    pub connect_resp_code: u8,
}

pub const ATH11K_NUM_CONTROL_TX_BUFFERS: c_int = 2;
pub const ATH11K_HTC_MAX_LEN: c_int = 4096;
pub const ATH11K_HTC_MAX_CTRL_MSG_LEN: c_int = 256;

pub const ATH11K_HTC_MAX_SERVICE_ALLOC_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc_ep {
    pub htc: *mut ath11k_htc,
    pub eid: ath11k_htc_ep_id,
    pub service_id: ath11k_htc_svc_id,
    pub ep_ops: ath11k_htc_ep_ops,
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
pub struct ath11k_htc_svc_tx_credits {
    pub service_id: u16,
    pub credit_allocation: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath11k_htc {
    pub ab: *mut ath11k_base,
    pub endpoint: [ath11k_htc_ep; ATH11K_HTC_EP_COUNT],
// protects endpoints
    pub tx_lock: spinlock_t,
    pub control_resp_buffer: [u8; ATH11K_HTC_MAX_CTRL_MSG_LEN],
    pub control_resp_len: c_int,
    pub ctl_resp: completion,
    pub total_transmit_credits: c_int,
    pub target_credit_size: c_int,
    pub wmi_ep_count: u8,
}

extern "C" {
    pub fn ath11k_htc_init(ar: *mut ath11k_base) -> c_int;
}
extern "C" {
    pub fn ath11k_htc_wait_target(htc: *mut ath11k_htc) -> c_int;
}
extern "C" {
    pub fn ath11k_htc_start(htc: *mut ath11k_htc) -> c_int;
}
