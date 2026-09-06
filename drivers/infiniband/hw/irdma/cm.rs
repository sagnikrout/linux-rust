//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/cm.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2021 Intel Corporation
pub const IRDMA_MPA_REQUEST_ACCEPT: c_int = 1;
pub const IRDMA_MPA_REQUEST_REJECT: c_int = 2;
// IETF MPA -- defines

pub const IETF_MPA_KEY_SIZE: c_int = 16;
pub const IETF_MPA_VER: c_int = 1;
pub const IETF_MAX_PRIV_DATA_LEN: c_int = 512;
pub const IETF_MPA_FRAME_SIZE: c_int = 20;
pub const IETF_RTR_MSG_SIZE: c_int = 4;
pub const IETF_MPA_V2_FLAG: c_uint = 0x10;
pub const SNDMARKER_SEQNMASK: c_uint = 0x000001ff;
pub const IRDMA_MAX_IETF_SIZE: c_int = 32;
// IETF RTR MSG Fields
pub const IETF_PEER_TO_PEER: c_uint = 0x8000;
pub const IETF_FLPDU_ZERO_LEN: c_uint = 0x4000;
pub const IETF_RDMA0_WRITE: c_uint = 0x8000;
pub const IETF_RDMA0_READ: c_uint = 0x4000;
pub const IETF_NO_IRD_ORD: c_uint = 0x3fff;
pub const MAX_PORTS: c_int = 65536;
pub const IRDMA_PASSIVE_STATE_INDICATED: c_int = 0;
pub const IRDMA_DO_NOT_SEND_RESET_EVENT: c_int = 1;
pub const IRDMA_SEND_RESET_EVENT: c_int = 2;
pub const MAX_IRDMA_IFS: c_int = 4;
pub const SET_ACK: c_int = 1;
pub const SET_SYN: c_int = 2;
pub const SET_FIN: c_int = 4;
pub const SET_RST: c_int = 8;
pub const TCP_OPTIONS_PADDING: c_int = 3;
pub const IRDMA_DEFAULT_RETRYS: c_int = 64;
pub const IRDMA_DEFAULT_RETRANS: c_int = 32;
pub const IRDMA_DEFAULT_TTL: c_uint = 0x40;
pub const IRDMA_DEFAULT_RTT_VAR: c_int = 6;
pub const IRDMA_DEFAULT_SS_THRESH: c_uint = 0x3fffffff;
pub const IRDMA_DEFAULT_REXMIT_THRESH: c_int = 8;

pub const IRDMA_SHORT_TIME: c_int = 10;

pub const IRDMA_CM_HASHTABLE_SIZE: c_int = 1024;
pub const IRDMA_CM_TCP_TIMER_INTERVAL: c_int = 3000;
pub const IRDMA_CM_DEFAULT_MTU: c_int = 1540;
pub const IRDMA_CM_DEFAULT_FRAME_CNT: c_int = 10;
pub const IRDMA_CM_THREAD_STACK_SIZE: c_int = 256;
pub const IRDMA_CM_DEFAULT_RCV_WND: c_int = 64240;
pub const IRDMA_CM_DEFAULT_RCV_WND_SCALED: c_uint = 0x3FFFC;
pub const IRDMA_CM_DEFAULT_RCV_WND_SCALE: c_int = 2;
pub const IRDMA_CM_DEFAULT_FREE_PKTS: c_int = 10;
pub const IRDMA_CM_FREE_PKT_LO_WATERMARK: c_int = 2;
pub const IRDMA_CM_DEFAULT_MSS: c_int = 536;
pub const IRDMA_CM_DEFAULT_MPA_VER: c_int = 2;
pub const IRDMA_CM_DEFAULT_SEQ: c_uint = 0x159bf75f;
pub const IRDMA_CM_DEFAULT_LOCAL_ID: c_uint = 0x3b47;
pub const IRDMA_CM_DEFAULT_SEQ2: c_uint = 0x18ed5740;
pub const IRDMA_CM_DEFAULT_LOCAL_ID2: c_uint = 0xb807;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ietf_mpa_flags {
    IETF_MPA_FLAGS_REJECT  = 0x20,
    IETF_MPA_FLAGS_CRC     = 0x40,
    IETF_MPA_FLAGS_MARKERS = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_timer_type {
    IRDMA_TIMER_TYPE_SEND,
    IRDMA_TIMER_TYPE_CLOSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum option_nums {
    OPTION_NUM_EOL,
    OPTION_NUM_NONE,
    OPTION_NUM_MSS,
    OPTION_NUM_WINDOW_SCALE,
    OPTION_NUM_SACK_PERM,
    OPTION_NUM_SACK,
    OPTION_NUM_WRITE0 = 0xbc,
}

// cm node transition states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cm_node_state {
    IRDMA_CM_STATE_UNKNOWN,
    IRDMA_CM_STATE_INITED,
    IRDMA_CM_STATE_LISTENING,
    IRDMA_CM_STATE_SYN_RCVD,
    IRDMA_CM_STATE_SYN_SENT,
    IRDMA_CM_STATE_ONE_SIDE_ESTABLISHED,
    IRDMA_CM_STATE_ESTABLISHED,
    IRDMA_CM_STATE_ACCEPTING,
    IRDMA_CM_STATE_MPAREQ_SENT,
    IRDMA_CM_STATE_MPAREQ_RCVD,
    IRDMA_CM_STATE_MPAREJ_RCVD,
    IRDMA_CM_STATE_OFFLOADED,
    IRDMA_CM_STATE_FIN_WAIT1,
    IRDMA_CM_STATE_FIN_WAIT2,
    IRDMA_CM_STATE_CLOSE_WAIT,
    IRDMA_CM_STATE_TIME_WAIT,
    IRDMA_CM_STATE_LAST_ACK,
    IRDMA_CM_STATE_CLOSING,
    IRDMA_CM_STATE_LISTENER_DESTROYED,
    IRDMA_CM_STATE_CLOSED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpa_frame_ver {
    IETF_MPA_V1 = 1,
    IETF_MPA_V2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpa_frame_key {
    MPA_KEY_REQUEST,
    MPA_KEY_REPLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_rdma0 {
    SEND_RDMA_READ_ZERO  = 1,
    SEND_RDMA_WRITE_ZERO = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_tcpip_pkt_type {
    IRDMA_PKT_TYPE_UNKNOWN,
    IRDMA_PKT_TYPE_SYN,
    IRDMA_PKT_TYPE_SYNACK,
    IRDMA_PKT_TYPE_ACK,
    IRDMA_PKT_TYPE_FIN,
    IRDMA_PKT_TYPE_RST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cm_listener_state {
    IRDMA_CM_LISTENER_PASSIVE_STATE = 1,
    IRDMA_CM_LISTENER_ACTIVE_STATE  = 2,
    IRDMA_CM_LISTENER_EITHER_STATE  = 3,
}

// CM event codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cm_event_type {
    IRDMA_CM_EVENT_UNKNOWN,
    IRDMA_CM_EVENT_ESTABLISHED,
    IRDMA_CM_EVENT_MPA_REQ,
    IRDMA_CM_EVENT_MPA_CONNECT,
    IRDMA_CM_EVENT_MPA_ACCEPT,
    IRDMA_CM_EVENT_MPA_REJECT,
    IRDMA_CM_EVENT_MPA_ESTABLISHED,
    IRDMA_CM_EVENT_CONNECTED,
    IRDMA_CM_EVENT_RESET,
    IRDMA_CM_EVENT_ABORTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ietf_mpa_v1 {
    pub key: [u8; IETF_MPA_KEY_SIZE],
    pub flags: u8,
    pub rev: u8,
    pub priv_data_len: __be16,
    pub priv_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ietf_rtr_msg {
    pub ctrl_ird: __be16,
    pub ctrl_ord: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ietf_mpa_v2 {
    pub key: [u8; IETF_MPA_KEY_SIZE],
    pub flags: u8,
    pub rev: u8,
    pub priv_data_len: __be16,
    pub rtr_msg: ietf_rtr_msg,
    pub priv_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct option_base {
    pub optionnum: u8,
    pub len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct option_mss {
    pub optionnum: u8,
    pub len: u8,
    pub mss: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct option_windowscale {
    pub optionnum: u8,
    pub len: u8,
    pub shiftcount: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union all_known_options {
    pub eol: c_char,
    pub base: option_base,
    pub mss: option_mss,
    pub windowscale: option_windowscale,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_timer_entry {
    pub list: list_head,
    pub /: *mut *mut unsigned long timetosend; / jiffies,
    pub sqbuf: *mut irdma_puda_buf,
    pub type: u32,
    pub retrycount: u32,
    pub retranscount: u32,
    pub context: u32,
    pub send_retrans: u32,
    pub close_when_complete: c_int,
}

// CM context params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_tcp_context {
    pub client: u8,
    pub loc_seq_num: u32,
    pub loc_ack_num: u32,
    pub rem_ack_num: u32,
    pub rcv_nxt: u32,
    pub loc_id: u32,
    pub rem_id: u32,
    pub snd_wnd: u32,
    pub max_snd_wnd: u32,
    pub rcv_wnd: u32,
    pub mss: u32,
    pub snd_wscale: u8,
    pub rcv_wscale: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_apbvt_entry {
    pub hlist: hlist_node,
    pub use_cnt: u32,
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_listener {
    pub list: list_head,
    pub cm_id: *mut iw_cm_id,
    pub cm_core: *mut irdma_cm_core,
    pub iwdev: *mut irdma_device,
    pub child_listen_list: list_head,
    pub apbvt_entry: *mut irdma_apbvt_entry,
    pub listener_state: irdma_cm_listener_state,
    pub refcnt: refcount_t,
    pub pend_accepts_cnt: core::sync::atomic::AtomicI32,
    pub loc_addr: [u32; 4],
    pub reused_node: u32,
    pub backlog: c_int,
    pub loc_port: u16,
    pub vlan_id: u16,
    pub loc_mac: [u8; ETH_ALEN],
    pub user_pri: u8,
    pub tos: u8,
    pub qhash_set:1: bool,
    pub ipv4:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_kmem_info {
    pub addr: *mut c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mpa_priv_info {
    pub addr: *const c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_node {
    pub iwqp: *mut irdma_qp,
    pub iwdev: *mut irdma_device,
    pub dev: *mut irdma_sc_dev,
    pub tcp_cntxt: irdma_cm_tcp_context,
    pub cm_core: *mut irdma_cm_core,
    pub send_entry: *mut irdma_timer_entry,
    pub close_entry: *mut irdma_timer_entry,
    pub listener: *mut irdma_cm_listener,
    pub timer_entry: list_head,
    pub reset_entry: list_head,
    pub teardown_entry: list_head,
    pub apbvt_entry: *mut irdma_apbvt_entry,
    pub rcu_head: rcu_head,
    pub pdata: irdma_mpa_priv_info,
    pub ah: *mut irdma_sc_ah,
    pub mpa_frame: ietf_mpa_v1,
    pub mpa_v2_frame: ietf_mpa_v2,
}

// Used by internal CM APIs to pass CM information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_info {
    pub cm_id: *mut iw_cm_id,
    pub loc_port: u16,
    pub rem_port: u16,
    pub loc_addr: [u32; 4],
    pub rem_addr: [u32; 4],
    pub qh_qpid: u32,
    pub vlan_id: u16,
    pub backlog: c_int,
    pub user_pri: u8,
    pub tos: u8,
    pub ipv4: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_event {
    pub type: irdma_cm_event_type,
    pub cm_info: irdma_cm_info,
    pub event_work: work_struct,
    pub cm_node: *mut irdma_cm_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cm_core {
    pub iwdev: *mut irdma_device,
    pub dev: *mut irdma_sc_dev,
    pub listen_list: list_head,
    pub 8): DECLARE_HASHTABLE(cm_hash_tbl,,
    pub 8): DECLARE_HASHTABLE(apbvt_hash_tbl,,
    pub tcp_timer: timer_list,
    pub event_wq: *mut workqueue_struct,
    pub /: *mut *mut spinlock_t ht_lock; / protect CM node (active side) list,
    pub /: *mut *mut spinlock_t listen_list_lock; / protect listener list,
    pub entries*/: *mut *mut spinlock_t apbvt_lock; /serialize apbvt add/del,
    pub stats_nodes_created: u64,
    pub stats_nodes_destroyed: u64,
    pub stats_listen_created: u64,
    pub stats_listen_destroyed: u64,
    pub stats_listen_nodes_created: u64,
    pub stats_listen_nodes_destroyed: u64,
    pub stats_lpbs: u64,
    pub stats_accepts: u64,
    pub stats_rejects: u64,
    pub stats_connect_errs: u64,
    pub stats_passive_errs: u64,
    pub stats_pkt_retrans: u64,
    pub stats_backlog_drops: u64,
    pub flags): u8,
    pub wait): *mut *mut *mut int (cm_create_ah)(struct irdma_cm_node cm_node, bool,
    pub cm_node): *mut *mut void (cm_free_ah)(struct irdma_cm_node,
}

extern "C" {
    pub fn irdma_accept(cm_id: *mut iw_cm_id, conn_param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn irdma_reject(cm_id: *mut iw_cm_id, pdata: *const c_void, pdata_len: u8) -> c_int;
}
extern "C" {
    pub fn irdma_connect(cm_id: *mut iw_cm_id, conn_param: *mut iw_cm_conn_param) -> c_int;
}
extern "C" {
    pub fn irdma_create_listen(cm_id: *mut iw_cm_id, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn irdma_destroy_listen(cm_id: *mut iw_cm_id) -> c_int;
}
extern "C" {
    pub fn irdma_add_arp(rf: *mut irdma_pci_f, ip: *mut u32, ipv4: bool, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn irdma_cm_start(dev: *mut irdma_device) -> c_int;
}
extern "C" {
    pub fn irdma_cm_stop(dev: *mut irdma_device) -> c_int;
}
extern "C" {
    pub fn irdma_ipv4_is_lpb(loc_addr: u32, rem_addr: u32) -> bool;
}
extern "C" {
    pub fn irdma_ipv6_is_lpb(loc_addr: *mut u32, rem_addr: *mut u32) -> bool;
}
extern "C" {
    pub fn irdma_port_in_use(cm_core: *mut irdma_cm_core, port: u16) -> bool;
}
extern "C" {
    pub fn irdma_send_ack(cm_node: *mut irdma_cm_node);
}
extern "C" {
    pub fn irdma_lpb_nop(qp: *mut irdma_sc_qp);
}
extern "C" {
    pub fn irdma_rem_ref_cm_node(cm_node: *mut irdma_cm_node);
}
extern "C" {
    pub fn irdma_add_conn_est_qh(cm_node: *mut irdma_cm_node);
}
