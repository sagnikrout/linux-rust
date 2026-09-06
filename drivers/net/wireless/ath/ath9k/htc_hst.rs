//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/htc_hst.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath9k_hif_transports {
    ATH9K_HIF_USB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath9k_htc_hif {
    pub list: list_head,
    pub transport: ath9k_hif_transports,
    pub name: *const c_char,
    pub control_dl_pipe: u8,
    pub control_ul_pipe: u8,
    pub hif_handle): *mut *mut void (start) (void,
    pub hif_handle): *mut *mut void (stop) (void,
    pub idx): *mut *mut *mut void (sta_drain) (void hif_handle, u8,
    pub buf): *mut *mut *mut int (send) (void hif_handle, u8 pipe, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_endpoint_id {
    ENDPOINT_UNUSED = -1,
    ENDPOINT0 = 0,
    ENDPOINT1 = 1,
    ENDPOINT2 = 2,
    ENDPOINT3 = 3,
    ENDPOINT4 = 4,
    ENDPOINT5 = 5,
    ENDPOINT6 = 6,
    ENDPOINT7 = 7,
    ENDPOINT8 = 8,
    ENDPOINT_MAX = 22
}

// Htc frame hdr flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_frame_hdr {
    pub endpoint_id: u8,
    pub flags: u8,
    pub payload_len: __be16,
    pub control: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_ready_msg {
    pub message_id: __be16,
    pub credits: __be16,
    pub credit_size: __be16,
    pub max_endpoints: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_config_pipe_msg {
    pub message_id: __be16,
    pub pipe_id: u8,
    pub credits: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_panic_bad_vaddr {
    pub pattern: __be32,
    pub exccause: __be32,
    pub pc: __be32,
    pub badvaddr: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_panic_bad_epid {
    pub pattern: __be32,
    pub epid: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_ep_callbacks {
    pub priv: *mut c_void,
    pub txok): *mut *mut *mut *mut void (tx) (void , struct sk_buff , enum htc_endpoint_id, bool,
    pub htc_endpoint_id): *mut *mut *mut *mut void (rx) (void , struct sk_buff , enum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_endpoint {
    pub service_id: u16,
    pub ep_callbacks: htc_ep_callbacks,
    pub max_txqdepth: u32,
    pub max_msglen: c_int,
    pub ul_pipeid: u8,
    pub dl_pipeid: u8,
}

pub const HTC_MAX_CONTROL_MESSAGE_LENGTH: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_target {
    pub hif_dev: *mut c_void,
    pub drv_priv: *mut ath9k_htc_priv,
    pub dev: *mut device,
    pub hif: *mut ath9k_htc_hif,
    pub endpoint: [htc_endpoint; ENDPOINT_MAX],
    pub target_wait: completion,
    pub cmd_wait: completion,
    pub list: list_head,
    pub conn_rsp_epid: htc_endpoint_id,
    pub credits: u16,
    pub credit_size: u16,
    pub htc_flags: u8,
    pub tgt_ready: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_msg_id {
    HTC_MSG_READY_ID = 1,
    HTC_MSG_CONNECT_SERVICE_ID,
    HTC_MSG_CONNECT_SERVICE_RESPONSE_ID,
    HTC_MSG_SETUP_COMPLETE_ID,
    HTC_MSG_CONFIG_PIPE_ID,
    HTC_MSG_CONFIG_PIPE_RESPONSE_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_service_connreq {
    pub service_id: u16,
    pub con_flags: u16,
    pub max_send_qdepth: u32,
    pub ep_callbacks: htc_ep_callbacks,
}

// Current service IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htc_service_group_ids {
    RSVD_SERVICE_GROUP = 0,
    WMI_SERVICE_GROUP = 1,

    HTC_SERVICE_GROUP_LAST = 255
}

// NOTE: service ID of 0x0000 is reserved and should never be used

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_conn_svc_msg {
    pub msg_id: __be16,
    pub service_id: __be16,
    pub con_flags: __be16,
    pub dl_pipeid: u8,
    pub ul_pipeid: u8,
    pub svc_meta_len: u8,
    pub pad: u8,
    pub __packed: },
// connect response status codes
pub const HTC_SERVICE_SUCCESS: c_int = 0;
pub const HTC_SERVICE_NOT_FOUND: c_int = 1;
pub const HTC_SERVICE_FAILED: c_int = 2;
pub const HTC_SERVICE_NO_RESOURCES: c_int = 3;
pub const HTC_SERVICE_NO_MORE_EP: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_conn_svc_rspmsg {
    pub msg_id: __be16,
    pub service_id: __be16,
    pub status: u8,
    pub endpoint_id: u8,
    pub max_msg_len: __be16,
    pub svc_meta_len: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htc_comp_msg {
    pub msg_id: __be16,
    pub __packed: },
    pub target): *mut int htc_init(struct htc_target,
    pub conn_rsp_eid): *mut htc_endpoint_id,
    pub skb): *mut *mut int htc_send(struct htc_target target, struct sk_buff,
    pub epid): htc_endpoint_id,
    pub target): *mut void htc_stop(struct htc_target,
    pub target): *mut void htc_start(struct htc_target,
    pub idx): *mut *mut void htc_sta_drain(struct htc_target target, u8,
    pub pipe_id): *mut *mut sk_buff skb, u32 len, u8,
    pub txok): *mut *mut sk_buff skb, bool,
    pub dev): *mut device,
    pub htc): *mut void ath9k_htc_hw_free(struct htc_target,
    pub drv_info): u32,
    pub hot_unplug): *mut *mut void ath9k_htc_hw_deinit(struct htc_target target, bool,
