//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/htc.h
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
// Copyright (c) 2004-2011 Atheros Communications Inc.
// Copyright (c) 2011 Qualcomm Atheros, Inc.
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

// frame header flags
// send direction

// receive direction

// Bundle count maske and shift

pub const HTC_FLG_RX_BNDL_CNT_S: c_int = 4;

// HTC control message IDs
pub const HTC_MSG_READY_ID: c_int = 1;
pub const HTC_MSG_CONN_SVC_ID: c_int = 2;
pub const HTC_MSG_CONN_SVC_RESP_ID: c_int = 3;
pub const HTC_MSG_SETUP_COMPLETE_ID: c_int = 4;
pub const HTC_MSG_SETUP_COMPLETE_EX_ID: c_int = 5;
pub const HTC_MAX_CTRL_MSG_LEN: c_int = 256;
pub const HTC_VERSION_2P0: c_uint = 0x00;
pub const HTC_VERSION_2P1: c_uint = 0x01;
pub const HTC_SERVICE_META_DATA_MAX_LENGTH: c_int = 128;
pub const HTC_CONN_FLGS_THRESH_LVL_QUAT: c_uint = 0x0;
pub const HTC_CONN_FLGS_THRESH_LVL_HALF: c_uint = 0x1;
pub const HTC_CONN_FLGS_THRESH_LVL_THREE_QUAT: c_uint = 0x2;
pub const HTC_CONN_FLGS_REDUCE_CRED_DRIB: c_uint = 0x4;
pub const HTC_CONN_FLGS_THRESH_MASK: c_uint = 0x3;
// disable credit flow control on a specific service

pub const HTC_CONN_FLGS_SET_RECV_ALLOC_SHIFT: c_int = 8;
pub const HTC_CONN_FLGS_SET_RECV_ALLOC_MASK: c_uint = 0xFF00U;
// connect response status codes
pub const HTC_SERVICE_SUCCESS: c_int = 0;
pub const HTC_SERVICE_NOT_FOUND: c_int = 1;
pub const HTC_SERVICE_FAILED: c_int = 2;
// no resources (i.e. no more endpoints)
pub const HTC_SERVICE_NO_RESOURCES: c_int = 3;
// specific service is not allowing any more endpoints
pub const HTC_SERVICE_NO_MORE_EP: c_int = 4;
// report record IDs
pub const HTC_RECORD_NULL: c_int = 0;
pub const HTC_RECORD_CREDITS: c_int = 1;
pub const HTC_RECORD_LOOKAHEAD: c_int = 2;
pub const HTC_RECORD_LOOKAHEAD_BUNDLE: c_int = 3;

// NOTE: service ID of 0x0000 is reserved and should never be used

pub const WMI_MAX_SERVICES: c_int = 5;
pub const WMM_NUM_AC: c_int = 4;
// reserved and used to flush ALL packets
pub const HTC_TX_PACKET_TAG_ALL: c_int = 0;
pub const HTC_SERVICE_TX_PACKET_TAG: c_int = 1;

// more packets on this endpoint are being fetched

// TODO.. for BMI
pub const ENDPOINT1: c_int = 0;
// TODO -remove me, but we have to fix BMI first
pub const HTC_MAILBOX_NUM_MAX: c_int = 4;
// enable send bundle padding for this endpoint

// HTC operational parameters

pub const HTC_TARGET_RESPONSE_POLL_WAIT: c_int = 10;
pub const HTC_TARGET_RESPONSE_POLL_COUNT: c_int = 200;
pub const HTC_TARGET_DEBUG_INTR_MASK: c_uint = 0x01;
pub const HTC_TARGET_CREDIT_INTR_MASK: c_uint = 0xF0;
pub const HTC_HOST_MAX_MSG_PER_BUNDLE: c_int = 8;
pub const HTC_MIN_HTC_MSGS_TO_BUNDLE: c_int = 2;
// packet flags

pub const NUM_CONTROL_BUFFERS: c_int = 8;
pub const NUM_CONTROL_TX_BUFFERS: c_int = 2;

//
// The frame header length and message formats defined herein were selected
// to accommodate optimal alignment for target processing. This reduces
// code size and improves performance. Any changes to the header length may
// alter the alignment and cause exceptions on the target. When adding to
// the messagestructures insure that fields are properly aligned.
//
// HTC frame header
//
// NOTE: do not remove or re-arrange the fields, these are minimally
// required to take advantage of 4-byte lookaheads in some hardware
// implementations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_frame_hdr {
    pub eid: u8,
    pub flags: u8,
// length of data (including trailer) that follows the header
    pub payld_len: __le16,
}

// end of 4-byte lookahead
// HTC ready message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_ready_msg {
    pub msg_id: __le16,
    pub cred_cnt: __le16,
    pub cred_sz: __le16,
    pub max_ep: u8,
    pub pad: u8,
    pub __packed: },
// extended HTC ready message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_ready_ext_msg {
    pub ver2_0_info: htc_ready_msg,
    pub htc_ver: u8,
    pub msg_per_htc_bndl: u8,
    pub __packed: },
// connect service
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_conn_service_msg {
    pub msg_id: __le16,
    pub svc_id: __le16,
    pub conn_flags: __le16,
    pub svc_meta_len: u8,
    pub pad: u8,
    pub __packed: },
// connect response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_conn_service_resp {
    pub msg_id: __le16,
    pub svc_id: __le16,
    pub status: u8,
    pub eid: u8,
    pub max_msg_sz: __le16,
    pub svc_meta_len: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_setup_comp_msg {
    pub msg_id: __le16,
    pub __packed: },
// extended setup completion message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_setup_comp_ext_msg {
    pub msg_id: __le16,
    pub flags: __le32,
    pub msg_per_rxbndl: u8,
    pub Rsvd: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_record_hdr {
    pub rec_id: u8,
    pub len: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_credit_report {
    pub eid: u8,
    pub credits: u8,
    pub __packed: },
//
// NOTE: The lk_ahd array is guarded by a pre_valid
// and Post Valid guard bytes. The pre_valid bytes must
// equal the inverse of the post_valid byte.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_lookahead_report {
    pub pre_valid: u8,
    pub lk_ahd: [u8; 4],
    pub post_valid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_bundle_lkahd_rpt {
    pub lk_ahd: [u8; 4],
    pub __packed: },
// Current service IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_service_grp_ids {
    RSVD_SERVICE_GROUP = 0,
    WMI_SERVICE_GROUP = 1,

    HTC_TEST_GROUP = 254,
    HTC_SERVICE_GROUP_LAST = 255
}

// ------ endpoint IDS ------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_endpoint_id {
    ENDPOINT_UNUSED = -1,
    ENDPOINT_0 = 0,
    ENDPOINT_1 = 1,
    ENDPOINT_2 = 2,
    ENDPOINT_3,
    ENDPOINT_4,
    ENDPOINT_5,
    ENDPOINT_6,
    ENDPOINT_7,
    ENDPOINT_8,
    ENDPOINT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_tx_packet_info {
    pub tag: u16,
    pub cred_used: c_int,
    pub flags: u8,
    pub seqno: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_rx_packet_info {
    pub exp_hdr: u32,
    pub rx_flags: u32,
    pub indicat_flags: u32,
}

// wrapper around endpoint-specific packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_packet {
    pub list: list_head,
// caller's per packet specific context
    pub pkt_cntxt: *mut c_void,
//
// the true buffer start , the caller can store the real
// buffer start here.  In receive callbacks, the HTC layer
// sets buf to the start of the payload past the header.
// This field allows the caller to reset buf when it recycles
// receive packets back to HTC.
//
    pub buf_start: *mut u8,
//
// Pointer to the start of the buffer. In the transmit
// direction this points to the start of the payload. In the
// receive direction, however, the buffer when queued up
// points to the start of the HTC header but when returned
// to the caller points to the start of the payload
//
    pub buf: *mut u8,
    pub buf_len: u32,
// actual length of payload
    pub act_len: u32,
// endpoint that this packet was sent/recv'd from
    pub endpoint: htc_endpoint_id,
// completion status
    pub status: c_int,
    pub tx: htc_tx_packet_info,
    pub rx: htc_rx_packet_info,
    pub info: },
    pub ): *mut *mut *mut void (completion) (struct htc_target , struct htc_packet,
    pub context: *mut htc_target,
//
// optimization for network-oriented data, the HTC packet
// can pass the network buffer corresponding to the HTC packet
// lower layers may optimized the transfer knowing this is
// a network buffer
//
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_send_full_action {
    HTC_SEND_FULL_KEEP = 0,
    HTC_SEND_FULL_DROP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_ep_callbacks {
    pub ): *mut *mut *mut void (tx_complete) (struct htc_target , struct htc_packet,
    pub ): *mut *mut *mut void (rx) (struct htc_target , struct htc_packet,
    pub endpoint): *mut *mut *mut void (rx_refill) (struct htc_target , enum htc_endpoint_id,
    pub ): *mut htc_packet,
    pub int): htc_endpoint_id,,
    pub ): *mut *mut *mut void (tx_comp_multi) (struct htc_target , struct list_head,
    pub rx_alloc_thresh: c_int,
    pub rx_refill_thresh: c_int,
}

// service connection information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_service_connect_req {
    pub svc_id: u16,
    pub conn_flags: u16,
    pub ep_cb: htc_ep_callbacks,
    pub max_txq_depth: c_int,
    pub flags: u32,
    pub max_rxmsg_sz: c_uint,
}

// service connection response information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_service_connect_resp {
    pub buf_len: u8,
    pub act_len: u8,
    pub endpoint: htc_endpoint_id,
    pub len_max: c_uint,
    pub resp_code: u8,
}

// endpoint distributionstructure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_endpoint_credit_dist {
    pub list: list_head,
// Service ID (set by HTC)
    pub svc_id: u16,
// endpoint for this distributionstruct (set by HTC)
    pub endpoint: htc_endpoint_id,
    pub dist_flags: u32,
//
// credits for normal operation, anything above this
// indicates the endpoint is over-subscribed.
//
    pub cred_norm: c_int,
// floor for credit distribution
    pub cred_min: c_int,
    pub cred_assngd: c_int,
// current credits available
    pub credits: c_int,
//
// pending credits to distribute on this endpoint, this
// is set by HTC when credit reports arrive.  The credit
// distribution functions sets this to zero when it distributes
// the credits.
//
    pub cred_to_dist: c_int,
//
// the number of credits that the current pending TX packet needs
// to transmit. This is set by HTC when endpoint needs credits in
// order to transmit.
//
    pub seek_cred: c_int,
// size in bytes of each credit
    pub cred_sz: c_int,
// credits required for a maximum sized messages
    pub cred_per_msg: c_int,
// reserved for HTC use
    pub htc_ep: *mut htc_endpoint,
//
// current depth of TX queue , i.e. messages waiting for credits
// This field is valid only when HTC_CREDIT_DIST_ACTIVITY_CHANGE
// or HTC_CREDIT_DIST_SEND_COMPLETE is indicated on an endpoint
// that has non-zero credits to recover.
//
    pub txq_depth: c_int,
}

//
// credit distribution code that is passed into the distribution function,
// there are mandatory and optional codes that must be handled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_credit_dist_reason {
    HTC_CREDIT_DIST_SEND_COMPLETE = 0,
    HTC_CREDIT_DIST_ACTIVITY_CHANGE = 1,
    HTC_CREDIT_DIST_SEEK_CREDITS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_htc_credit_info {
    pub total_avail_credits: c_int,
    pub cur_free_credits: c_int,
// list of lowest priority endpoints
    pub lowestpri_ep_dist: list_head,
}

// endpoint statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_endpoint_stats {
//
// number of times the host set the credit-low flag in a send
// message on this endpoint
//
    pub cred_low_indicate: u32,
    pub tx_issued: u32,
    pub tx_pkt_bundled: u32,
    pub tx_bundles: u32,
    pub tx_dropped: u32,
// running count of total credit reports received for this endpoint
    pub tx_cred_rpt: u32,
// credit reports received from this endpoint's RX packets
    pub cred_rpt_from_rx: u32,
// credit reports received from RX packets of other endpoints
    pub cred_rpt_from_other: u32,
// credit reports received from endpoint 0 RX packets
    pub cred_rpt_ep0: u32,
// count of credits received via Rx packets on this endpoint
    pub cred_from_rx: u32,
// count of credits received via another endpoint
    pub cred_from_other: u32,
// count of credits received via another endpoint
    pub cred_from_ep0: u32,
// count of consumed credits
    pub cred_cosumd: u32,
// count of credits returned
    pub cred_retnd: u32,
    pub rx_pkts: u32,
// count of lookahead records found in Rx msg
    pub rx_lkahds: u32,
// count of recv packets received in a bundle
    pub rx_bundl: u32,
// count of number of bundled lookaheads
    pub rx_bundle_lkahd: u32,
// count of the number of bundle indications from the HTC header
    pub rx_bundle_from_hdr: u32,
// the number of times the recv allocation threshold was hit
    pub rx_alloc_thresh_hit: u32,
// total number of bytes
    pub rxalloc_thresh_byte: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_endpoint {
    pub eid: htc_endpoint_id,
    pub svc_id: u16,
    pub txq: list_head,
    pub rx_bufq: list_head,
    pub cred_dist: htc_endpoint_credit_dist,
    pub ep_cb: htc_ep_callbacks,
    pub max_txq_depth: c_int,
    pub len_max: c_int,
    pub tx_proc_cnt: c_int,
    pub rx_proc_cnt: c_int,
    pub target: *mut htc_target,
    pub seqno: u8,
    pub conn_flags: u32,
    pub ep_st: htc_endpoint_stats,
    pub tx_drop_packet_threshold: u16,
    pub pipeid_ul: u8,
    pub pipeid_dl: u8,
    pub tx_lookup_queue: list_head,
    pub tx_credit_flow_enabled: bool,
    pub pipe: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_control_buffer {
    pub packet: htc_packet,
    pub buf: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_pipe_txcredit_alloc {
    pub service_id: u16,
    pub credit_alloc: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_send_queue_result {
    HTC_SEND_QUEUE_OK = 0,	/* packet was queued */
    HTC_SEND_QUEUE_DROP = 1,	/* this packet should be dropped */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_htc_ops {
    pub ar): *mut *mut *mut void (create)(struct ath6kl,
    pub target): *mut *mut int (wait_target)(struct htc_target,
    pub target): *mut *mut int (start)(struct htc_target,
    pub resp): *mut htc_service_connect_resp,
    pub packet): *mut *mut *mut int (tx)(struct htc_target target, struct htc_packet,
    pub target): *mut *mut void (stop)(struct htc_target,
    pub target): *mut *mut void (cleanup)(struct htc_target,
    pub tag): htc_endpoint_id endpoint, u16,
    pub target): *mut *mut void (flush_rx_buf)(struct htc_target,
    pub active): bool,
    pub endpoint): htc_endpoint_id,
    pub pktq): *mut list_head,
    pub cred_info): *mut ath6kl_htc_credit_info,
    pub skb): *mut *mut *mut int (tx_complete)(struct ath6kl ar, struct sk_buff,
    pub pipe): *mut *mut *mut *mut int (rx_complete)(struct ath6kl ar, struct sk_buff skb, u8,
}

// our HTC target state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_target {
    pub endpoint: [htc_endpoint; ENDPOINT_MAX],
// contains struct htc_endpoint_credit_dist
    pub cred_dist_list: list_head,
    pub free_ctrl_txbuf: list_head,
    pub free_ctrl_rxbuf: list_head,
    pub credit_info: *mut ath6kl_htc_credit_info,
    pub tgt_creds: c_int,
    pub tgt_cred_sz: c_uint,
// protects free_ctrl_txbuf and free_ctrl_rxbuf
    pub htc_lock: spinlock_t,
// FIXME: does this protect rx_bufq and endpoint structures or what?
    pub rx_lock: spinlock_t,
// protects endpoint->txq
    pub tx_lock: spinlock_t,
    pub dev: *mut ath6kl_device,
    pub htc_flags: u32,
    pub rx_st_flags: u32,
    pub ep_waiting: htc_endpoint_id,
    pub htc_tgt_ver: u8,
// max messages per bundle for HTC
    pub msg_per_bndl_max: c_int,
    pub tx_bndl_mask: u32,
    pub rx_bndl_enable: c_int,
    pub max_rx_bndl_sz: c_int,
    pub max_tx_bndl_sz: c_int,
    pub block_sz: u32,
    pub block_mask: u32,
    pub max_scat_entries: c_int,
    pub max_xfer_szper_scatreq: c_int,
    pub chk_irq_status_cnt: c_int,
// counts the number of Tx without bundling continuously per AC
    pub ac_tx_count: [u32; WMM_NUM_AC],
    pub htc_packet_pool: *mut htc_packet,
    pub ctrl_response_buf: [u8; HTC_MAX_CTRL_MSG_LEN],
    pub ctrl_response_len: c_int,
    pub ctrl_response_valid: bool,
    pub txcredit_alloc: [htc_pipe_txcredit_alloc; ENDPOINT_MAX],
    pub pipe: },
}

extern "C" {
    pub fn ath6kl_htc_pipe_attach(ar: *mut ath6kl);
}
extern "C" {
    pub fn ath6kl_htc_mbox_attach(ar: *mut ath6kl);
}
