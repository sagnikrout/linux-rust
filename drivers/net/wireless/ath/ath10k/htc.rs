//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/htc.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2016 Qualcomm Atheros, Inc.
//

//
// HTC protocol
//
// HTC - host-target control protocol
//
// tx packets are generally <htc_hdr><payload>
// rx packets are more complex: <htc_hdr><payload><trailer>
//
// The payload + trailer length is stored in len.
// To get payload-only length one needs to payload - trailer_len.
//
// Trailer contains (possibly) multiple <htc_record>.
// Each record is a id-len-value.
//
// HTC header flags, control_byte0, control_byte1
// have different meaning depending whether its tx
// or rx.
//
// Alignment: htc_hdr, payload and trailer are
// 4-byte aligned.
//
pub const HTC_HOST_MAX_MSG_PER_RX_BUNDLE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_tx_flags {
    ATH10K_HTC_FLAG_NEED_CREDIT_UPDATE = 0x01,
    ATH10K_HTC_FLAG_SEND_BUNDLE        = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_rx_flags {
    ATH10K_HTC_FLAGS_RECV_1MORE_BLOCK = 0x01,
    ATH10K_HTC_FLAG_TRAILER_PRESENT = 0x02,
}

// bits 2-3 are for extra bundle count bits 4-5

pub const ATH10K_HTC_BUNDLE_EXTRA_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_hdr {
    pub /: *mut *mut u8 eid; / @enum ath10k_htc_ep_id,
    pub /: *mut *mut u8 flags; / @enum ath10k_htc_tx_flags, ath10k_htc_rx_flags,
    pub len: __le16,
    pub /: *mut *mut u8 trailer_len; / for rx,
    pub control_byte0: u8,
    pub __packed: },
    pub /: *mut *mut u8 seq_no; / for tx,
    pub control_byte1: u8,
    pub __packed: },
    pub pad_len: __le16,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
    pub __packed: },
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_ath10k_htc_msg_id {
    ATH10K_HTC_MSG_READY_ID                = 1,
    ATH10K_HTC_MSG_CONNECT_SERVICE_ID      = 2,
    ATH10K_HTC_MSG_CONNECT_SERVICE_RESP_ID = 3,
    ATH10K_HTC_MSG_SETUP_COMPLETE_ID       = 4,
    ATH10K_HTC_MSG_SETUP_COMPLETE_EX_ID    = 5,
    ATH10K_HTC_MSG_SEND_SUSPEND_COMPLETE   = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_version {
    ATH10K_HTC_VERSION_2P0 = 0x00, /* 2.0 */
    ATH10K_HTC_VERSION_2P1 = 0x01, /* 2.1 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_conn_flags {
    ATH10K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_FOURTH    = 0x0,
    ATH10K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_ONE_HALF      = 0x1,
    ATH10K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_THREE_FOURTHS = 0x2,
    ATH10K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_UNITY         = 0x3,
pub const ATH10K_HTC_CONN_FLAGS_THRESHOLD_LEVEL_MASK: c_uint = 0x3;
    ATH10K_HTC_CONN_FLAGS_REDUCE_CREDIT_DRIBBLE    = 1 << 2,
    ATH10K_HTC_CONN_FLAGS_DISABLE_CREDIT_FLOW_CTRL = 1 << 3
pub const ATH10K_HTC_CONN_FLAGS_RECV_ALLOC_MASK: c_uint = 0xFF00;
pub const ATH10K_HTC_CONN_FLAGS_RECV_ALLOC_LSB: c_int = 8;
}

pub const ATH10K_HTC_MSG_READY_EXT_ALT_DATA_MASK: c_uint = 0xFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_conn_svc_status {
    ATH10K_HTC_CONN_SVC_STATUS_SUCCESS      = 0,
    ATH10K_HTC_CONN_SVC_STATUS_NOT_FOUND    = 1,
    ATH10K_HTC_CONN_SVC_STATUS_FAILED       = 2,
    ATH10K_HTC_CONN_SVC_STATUS_NO_RESOURCES = 3,
    ATH10K_HTC_CONN_SVC_STATUS_NO_MORE_EP   = 4
}

pub const ATH10K_MAX_MSG_PER_HTC_TX_BUNDLE: c_int = 32;
pub const ATH10K_MIN_MSG_PER_HTC_TX_BUNDLE: c_int = 2;
pub const ATH10K_MIN_CREDIT_PER_HTC_TX_BUNDLE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_setup_complete_flags {
    ATH10K_HTC_SETUP_COMPLETE_FLAGS_RX_BNDL_EN = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ath10k_htc_msg_hdr {
    pub /: *mut *mut __le16 message_id; / @enum htc_message_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_unknown {
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_ready {
    pub credit_count: __le16,
    pub credit_size: __le16,
    pub max_endpoints: u8,
    pub pad0: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_ready_extended {
    pub base: ath10k_htc_ready,
    pub /: *mut *mut u8 htc_version; / @enum ath10k_htc_version,
    pub max_msgs_per_htc_bundle: u8,
    pub reserved: __le16,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_conn_svc {
    pub service_id: __le16,
    pub /: *mut *mut __le16 flags; / @enum ath10k_htc_conn_flags,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_conn_svc_response {
    pub service_id: __le16,
    pub /: *mut *mut u8 status; / @enum ath10k_htc_conn_svc_status,
    pub eid: u8,
    pub max_msg_size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_setup_complete_extended {
    pub pad0: u8,
    pub pad1: u8,
    pub /: *mut *mut __le32 flags; / @enum htc_setup_complete_flags,
    pub max_msgs_per_bundled_recv: u8,
    pub pad2: u8,
    pub pad3: u8,
    pub pad4: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_msg {
    pub hdr: ath10k_ath10k_htc_msg_hdr,
// host-to-target
    pub connect_service: ath10k_htc_conn_svc,
    pub ready: ath10k_htc_ready,
    pub ready_ext: ath10k_htc_ready_extended,
    pub unknown: ath10k_htc_unknown,
    pub setup_complete_ext: ath10k_htc_setup_complete_extended,
// target-to-host
    pub connect_service_response: ath10k_htc_conn_svc_response,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_ath10k_htc_record_id {
    ATH10K_HTC_RECORD_NULL             = 0,
    ATH10K_HTC_RECORD_CREDITS          = 1,
    ATH10K_HTC_RECORD_LOOKAHEAD        = 2,
    ATH10K_HTC_RECORD_LOOKAHEAD_BUNDLE = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ath10k_htc_record_hdr {
    pub /: *mut *mut u8 id; / @enum ath10k_ath10k_htc_record_id,
    pub len: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_credit_report {
    pub /: *mut *mut u8 eid; / @enum ath10k_htc_ep_id,
    pub credits: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_lookahead_report {
    pub pre_valid: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub pad2: u8,
    pub lookahead: [u8; 4],
    pub post_valid: u8,
    pub pad3: u8,
    pub pad4: u8,
    pub pad5: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_lookahead_bundle {
    pub lookahead: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_record {
    pub hdr: ath10k_ath10k_htc_record_hdr,
    pub credit_report): DECLARE_FLEX_ARRAY(struct ath10k_htc_credit_report,,
    pub lookahead_report): DECLARE_FLEX_ARRAY(struct ath10k_htc_lookahead_report,,
    pub lookahead_bundle): DECLARE_FLEX_ARRAY(struct ath10k_htc_lookahead_bundle,,
}

//
// Host-side stuff
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_svc_gid {
    ATH10K_HTC_SVC_GRP_RSVD = 0,
    ATH10K_HTC_SVC_GRP_WMI = 1,
    ATH10K_HTC_SVC_GRP_NMI = 2,
    ATH10K_HTC_SVC_GRP_HTT = 3,
    ATH10K_LOG_SERVICE_GROUP = 6,

    ATH10K_HTC_SVC_GRP_TEST = 254,
    ATH10K_HTC_SVC_GRP_LAST = 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_svc_id {
// NOTE: service ID of 0x0000 is reserved and should never be used
    ATH10K_HTC_SVC_ID_RESERVED	= 0x0000,
    ATH10K_HTC_SVC_ID_UNUSED	= ATH10K_HTC_SVC_ID_RESERVED,

    ATH10K_HTC_SVC_ID_RSVD_CTRL	= SVC(ATH10K_HTC_SVC_GRP_RSVD, 1),
    ATH10K_HTC_SVC_ID_WMI_CONTROL	= SVC(ATH10K_HTC_SVC_GRP_WMI, 0),
    ATH10K_HTC_SVC_ID_WMI_DATA_BE	= SVC(ATH10K_HTC_SVC_GRP_WMI, 1),
    ATH10K_HTC_SVC_ID_WMI_DATA_BK	= SVC(ATH10K_HTC_SVC_GRP_WMI, 2),
    ATH10K_HTC_SVC_ID_WMI_DATA_VI	= SVC(ATH10K_HTC_SVC_GRP_WMI, 3),
    ATH10K_HTC_SVC_ID_WMI_DATA_VO	= SVC(ATH10K_HTC_SVC_GRP_WMI, 4),

    ATH10K_HTC_SVC_ID_NMI_CONTROL	= SVC(ATH10K_HTC_SVC_GRP_NMI, 0),
    ATH10K_HTC_SVC_ID_NMI_DATA	= SVC(ATH10K_HTC_SVC_GRP_NMI, 1),

    ATH10K_HTC_SVC_ID_HTT_DATA_MSG	= SVC(ATH10K_HTC_SVC_GRP_HTT, 0),

    ATH10K_HTC_SVC_ID_HTT_DATA2_MSG = SVC(ATH10K_HTC_SVC_GRP_HTT, 1),
    ATH10K_HTC_SVC_ID_HTT_DATA3_MSG = SVC(ATH10K_HTC_SVC_GRP_HTT, 2),
    ATH10K_HTC_SVC_ID_HTT_LOG_MSG = SVC(ATH10K_LOG_SERVICE_GROUP, 0),
// raw stream service (i.e. flash, tcmd, calibration apps)
    ATH10K_HTC_SVC_ID_TEST_RAW_STREAMS = SVC(ATH10K_HTC_SVC_GRP_TEST, 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath10k_htc_ep_id {
    ATH10K_HTC_EP_UNUSED = -1,
    ATH10K_HTC_EP_0 = 0,
    ATH10K_HTC_EP_1 = 1,
    ATH10K_HTC_EP_2,
    ATH10K_HTC_EP_3,
    ATH10K_HTC_EP_4,
    ATH10K_HTC_EP_5,
    ATH10K_HTC_EP_6,
    ATH10K_HTC_EP_7,
    ATH10K_HTC_EP_8,
    ATH10K_HTC_EP_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_ops {
    pub ar): *mut *mut void (target_send_suspend_complete)(struct ath10k,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_ep_ops {
    pub ): *mut *mut *mut void (ep_tx_complete)(struct ath10k , struct sk_buff,
    pub ): *mut *mut *mut void (ep_rx_complete)(struct ath10k , struct sk_buff,
    pub ): *mut *mut void (ep_tx_credits)(struct ath10k,
}

// service connection information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_svc_conn_req {
    pub service_id: u16,
    pub ep_ops: ath10k_htc_ep_ops,
    pub max_send_queue_depth: c_int,
}

// service connection response information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_svc_conn_resp {
    pub buffer_len: u8,
    pub actual_len: u8,
    pub eid: ath10k_htc_ep_id,
    pub max_msg_len: c_uint,
    pub connect_resp_code: u8,
}

pub const ATH10K_NUM_CONTROL_TX_BUFFERS: c_int = 2;
pub const ATH10K_HTC_MAX_LEN: c_int = 4096;
pub const ATH10K_HTC_MAX_CTRL_MSG_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_ep {
    pub htc: *mut ath10k_htc,
    pub eid: ath10k_htc_ep_id,
    pub service_id: ath10k_htc_svc_id,
    pub ep_ops: ath10k_htc_ep_ops,
    pub max_tx_queue_depth: c_int,
    pub max_ep_message_len: c_int,
    pub ul_pipe_id: u8,
    pub dl_pipe_id: u8,
    pub /: *mut *mut u8 seq_no; / for debugging,
    pub tx_credits: c_int,
    pub tx_credit_size: c_int,
    pub tx_credit_flow_enabled: bool,
    pub bundle_tx: bool,
    pub tx_req_head: sk_buff_head,
    pub tx_complete_head: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc_svc_tx_credits {
    pub service_id: u16,
    pub credit_allocation: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_htc {
    pub ar: *mut ath10k,
    pub endpoint: [ath10k_htc_ep; ATH10K_HTC_EP_COUNT],
// protects endpoints
    pub tx_lock: spinlock_t,
    pub htc_ops: ath10k_htc_ops,
    pub control_resp_buffer: [u8; ATH10K_HTC_MAX_CTRL_MSG_LEN],
    pub control_resp_len: c_int,
    pub ctl_resp: completion,
    pub total_transmit_credits: c_int,
    pub target_credit_size: c_int,
    pub max_msgs_per_htc_bundle: u8,
    pub alt_data_credit_size: c_int,
}

extern "C" {
    pub fn ath10k_htc_init(ar: *mut ath10k) -> c_int;
}
extern "C" {
    pub fn ath10k_htc_wait_target(htc: *mut ath10k_htc) -> c_int;
}
extern "C" {
    pub fn ath10k_htc_setup_tx_req(ep: *mut ath10k_htc_ep);
}
extern "C" {
    pub fn ath10k_htc_start(htc: *mut ath10k_htc) -> c_int;
}
extern "C" {
    pub fn ath10k_htc_stop_hl(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_htc_tx_completion_handler(ar: *mut ath10k, skb: *mut sk_buff);
}
extern "C" {
    pub fn ath10k_htc_rx_completion_handler(ar: *mut ath10k, skb: *mut sk_buff);
}
