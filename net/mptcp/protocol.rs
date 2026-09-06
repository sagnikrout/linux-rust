//! Automatically rewritten from C Header to Rust Module
//! Source: net/mptcp/protocol.h
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


// SPDX-License-Identifier: GPL-2.0
// Multipath TCP
//
// Copyright (c) 2017 - 2019, Intel Corporation.
//

pub const MPTCP_SUPPORTED_VERSION: c_int = 1;
// MPTCP option bits

// MPTCP option subtypes
pub const MPTCPOPT_MP_CAPABLE: c_int = 0;
pub const MPTCPOPT_MP_JOIN: c_int = 1;
pub const MPTCPOPT_DSS: c_int = 2;
pub const MPTCPOPT_ADD_ADDR: c_int = 3;
pub const MPTCPOPT_RM_ADDR: c_int = 4;
pub const MPTCPOPT_MP_PRIO: c_int = 5;
pub const MPTCPOPT_MP_FAIL: c_int = 6;
pub const MPTCPOPT_MP_FASTCLOSE: c_int = 7;
pub const MPTCPOPT_RST: c_int = 8;
// MPTCP suboption lengths
pub const TCPOLEN_MPTCP_MPC_SYN: c_int = 4;
pub const TCPOLEN_MPTCP_MPC_SYNACK: c_int = 12;
pub const TCPOLEN_MPTCP_MPC_ACK: c_int = 20;
pub const TCPOLEN_MPTCP_MPC_ACK_DATA: c_int = 22;
pub const TCPOLEN_MPTCP_MPJ_SYN: c_int = 12;
pub const TCPOLEN_MPTCP_MPJ_SYNACK: c_int = 16;
pub const TCPOLEN_MPTCP_MPJ_ACK: c_int = 24;
pub const TCPOLEN_MPTCP_DSS_BASE: c_int = 4;
pub const TCPOLEN_MPTCP_DSS_ACK32: c_int = 4;
pub const TCPOLEN_MPTCP_DSS_ACK64: c_int = 8;
pub const TCPOLEN_MPTCP_DSS_MAP32: c_int = 10;
pub const TCPOLEN_MPTCP_DSS_MAP64: c_int = 14;
pub const TCPOLEN_MPTCP_DSS_CHECKSUM: c_int = 2;
pub const TCPOLEN_MPTCP_ADD_ADDR: c_int = 16;
pub const TCPOLEN_MPTCP_ADD_ADDR_PORT: c_int = 18;
pub const TCPOLEN_MPTCP_ADD_ADDR_BASE: c_int = 8;
pub const TCPOLEN_MPTCP_ADD_ADDR_BASE_PORT: c_int = 10;
pub const TCPOLEN_MPTCP_ADD_ADDR6: c_int = 28;
pub const TCPOLEN_MPTCP_ADD_ADDR6_PORT: c_int = 30;
pub const TCPOLEN_MPTCP_ADD_ADDR6_BASE: c_int = 20;
pub const TCPOLEN_MPTCP_ADD_ADDR6_BASE_PORT: c_int = 22;
pub const TCPOLEN_MPTCP_PORT_LEN: c_int = 2;
pub const TCPOLEN_MPTCP_PORT_ALIGN: c_int = 2;
pub const TCPOLEN_MPTCP_RM_ADDR_BASE: c_int = 3;
pub const TCPOLEN_MPTCP_PRIO: c_int = 3;
pub const TCPOLEN_MPTCP_PRIO_ALIGN: c_int = 4;
pub const TCPOLEN_MPTCP_FASTCLOSE: c_int = 12;
pub const TCPOLEN_MPTCP_RST: c_int = 4;
pub const TCPOLEN_MPTCP_FAIL: c_int = 12;

// MPTCP MP_JOIN flags

pub const MPTCPOPT_THMAC_LEN: c_int = 8;
// MPTCP MP_CAPABLE flags

// MPTCP DSS flags

// MPTCP ADD_ADDR flags

// MPTCP MP_PRIO flags

// MPTCP TCPRST flags

// MPTCP socket atomic flags
pub const MPTCP_WORK_RTX: c_int = 1;
pub const MPTCP_FALLBACK_DONE: c_int = 2;
pub const MPTCP_WORK_CLOSE_SUBFLOW: c_int = 3;
// MPTCP socket release cb flags
pub const MPTCP_PUSH_PENDING: c_int = 1;
pub const MPTCP_CLEAN_UNA: c_int = 2;
pub const MPTCP_ERROR_REPORT: c_int = 3;
pub const MPTCP_RETRANSMIT: c_int = 4;
pub const MPTCP_FLUSH_JOIN_LIST: c_int = 5;
pub const MPTCP_SYNC_STATE: c_int = 6;
pub const MPTCP_SYNC_SNDBUF: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_skb_cb {
    pub map_seq: u64,
    pub end_seq: u64,
    pub offset: u32,
    pub has_rxtstamp: u8,
    pub cant_coalesce: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_options_received {
    pub sndr_key: u64,
    pub rcvr_key: u64,
    pub data_ack: u64,
    pub data_seq: u64,
    pub subflow_seq: u32,
    pub data_len: u16,
    pub csum: __sum16,
    pub suboptions: u16,
    pub join_id: u8,
    pub token: u32,
    pub nonce: u32,
    pub thmac: u64,
    pub hmac: [u8; MPTCPOPT_HMAC_LEN],
    pub addr: mptcp_addr_info,
    pub rm_list: mptcp_rm_list,
    pub ahmac: u64,
    pub fail_seq: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptcp_pm_status {
    MPTCP_PM_ADD_ADDR_RECEIVED,
    MPTCP_PM_ADD_ADDR_SEND_ACK,
    MPTCP_PM_RM_ADDR_RECEIVED,
    MPTCP_PM_ESTABLISHED,
    MPTCP_PM_SUBFLOW_ESTABLISHED,
    MPTCP_PM_ALREADY_ESTABLISHED,	/* persistent status, set after ESTABLISHED event */
    MPTCP_PM_MPC_ENDPOINT_ACCOUNTED, /* persistent status, set after MPC local address is
// accounted int id_avail_bitmap
//
    MPTCP_PM_DESTROYING,		/* To fence out PM list allocs */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptcp_pm_type {
    MPTCP_PM_TYPE_KERNEL = 0,
    MPTCP_PM_TYPE_USERSPACE,

    __MPTCP_PM_TYPE_NR,
    __MPTCP_PM_TYPE_MAX = __MPTCP_PM_TYPE_NR - 1,
}

// Status bits below MPTCP_PM_ALREADY_ESTABLISHED need pm worker actions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptcp_addr_signal_status {
    MPTCP_ADD_ADDR_SIGNAL,
    MPTCP_ADD_ADDR_ECHO,
    MPTCP_RM_ADDR_SIGNAL,
}

// max value of mptcp_addr_info.id

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_pm_data {
    pub local: mptcp_addr_info,
    pub remote: mptcp_addr_info,
    pub anno_list: list_head,
    pub userspace_pm_local_addr_list: list_head,
    pub /: *mut *mut spinlock_t lock; /protects the whole PM data,
    pub addr_signal: u8,
    pub server_side: bool,
    pub work_pending: bool,
    pub accept_addr: bool,
    pub accept_subflow: bool,
    pub remote_deny_join_id0: bool,
    pub add_addr_signaled: u8,
    pub add_addr_accepted: u8,
    pub local_addr_used: u8,
    pub pm_type: u8,
    pub extra_subflows: u8,
    pub status: u8,
    pub 1): DECLARE_BITMAP(id_avail_bitmap, MPTCP_PM_MAX_ADDR_ID +,
    pub rm_list_tx: mptcp_rm_list,
    pub rm_list_rx: mptcp_rm_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_pm_local {
    pub addr: mptcp_addr_info,
    pub flags: u32,
    pub ifindex: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_pm_addr_entry {
    pub list: list_head,
    pub addr: mptcp_addr_info,
    pub flags: u32,
    pub ifindex: c_int,
    pub lsk: *mut socket,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_data_frag {
    pub list: list_head,
    pub data_seq: u64,
    pub data_len: u16,
    pub offset: u16,
    pub overhead: u8,
    pub /: *mut *mut u8 eor; / currently using 1 bit,
    pub already_sent: u16,
    pub page: *mut page,
}

// Arbitrary compromise between as low as possible to react timely to subflow
// close event and as big as possible to avoid being fouled by biased large
// samples due to peer sending data on a different subflow WRT to the incoming
// ack.
//
pub const MPTCP_RTT_SAMPLES: c_int = 5;
// MPTCP connection sock
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_sock {
// inet_connection_sock must be the first member
    pub sk: inet_connection_sock,
    pub lock: *mut *mut u64 local_key; / protected by the first subflow socket,
// lockless access read
//
    pub /: *mut *mut u64 remote_key; / same as above,
    pub write_seq: u64,
    pub bytes_sent: u64,
    pub snd_nxt: u64,
    pub bytes_received: u64,
    pub ack_seq: u64,
    pub rcv_wnd_sent: core::sync::atomic::AtomicI64,
    pub rcv_data_fin_seq: u64,
    pub bytes_retrans: u64,
    pub bytes_consumed: u64,
    pub snd_burst: c_int,
    pub old_wspace: c_int,
    pub seq: *mut *mut u64 recovery_snd_nxt; / in recovery mode accept up to this,
// recovery related fields are under data_lock
// protection
//
    pub bytes_acked: u64,
    pub snd_una: u64,
    pub wnd_end: u64,
    pub last_data_sent: u32,
    pub last_data_recv: u32,
    pub last_ack_recv: u32,
    pub timer_ival: c_ulong,
    pub token: u32,
    pub flags: c_ulong,
    pub cb_flags: c_ulong,
    pub /: *mut *mut bool recovery; / closing subflow write queue reinjected,
    pub can_ack: bool,
    pub fully_established: bool,
    pub rcv_data_fin: bool,
    pub snd_data_fin_enable: bool,
    pub rcv_fastclose: bool,
    pub /: *mut *mut bool use_64bit_ack; / Set when we received a 64-bit DSN,
    pub csum_enabled: bool,
    pub allow_infinite_fallback: bool,
    pub sk_state,: *mut *mut u8 pending_state; / A subflow asked to set this,
// protected by the msk data lock
//
    pub mpc_endpoint_id: u8,
    pub notsent_lowat: u32,
    pub keepalive_cnt: c_int,
    pub keepalive_idle: c_int,
    pub keepalive_intvl: c_int,
    pub maxseg: c_int,
    pub work: work_struct,
    pub ooo_last_skb: *mut sk_buff,
    pub out_of_order_queue: rb_root,
    pub conn_list: list_head,
    pub rtx_queue: list_head,
    pub first_pending: *mut mptcp_data_frag,
    pub join_list: list_head,
    pub suitable: *mut *mut *mut sock first; / The mptcp ops can safely dereference, using,
// ONCE annotation, the subflow outside the socket
// lock as such sock is freed after close().
//
    pub pm: mptcp_pm_data,
    pub sched: *mut mptcp_sched_ops,
// Most recent rtt_us observed by in use incoming subflows.
    pub samples: [u32; MPTCP_RTT_SAMPLES],
    pub next_sample: u32,
    pub rcv_rtt_est: },
    pub /: *mut *mut int space; / bytes copied in last measurement window,
    pub /: *mut *mut int copied; / bytes copied in this measurement window,
    pub /: *mut *mut u64 time; / start time of measurement window,
    pub rcvq_space: },
    pub scaling_ratio: u8,
    pub allow_subflows: bool,
    pub subflow_id: u32,
    pub setsockopt_seq: u32,
    pub ca_name: [c_char; TCP_CA_NAME_MAX],
    pub fallback,: *mut *mut spinlock_t fallback_lock; / protects,
// allow_infinite_fallback and
// allow_join
//
    pub /: *mut *mut list_head backlog_list; / protected by the data lock,
    pub backlog_len: u32,
    pub backlog_unaccounted: u32,
}

// MPTCP-specific: we might (indirectly) call this helper with the wrong sk

extern "C" {
    pub fn __tcp_win_from_space(_arg: mptcp_sk(sk)->scaling_ratio, _arg: space) -> return;
}
extern "C" {
    pub fn __tcp_space_from_win(_arg: mptcp_sk(sk)->scaling_ratio, _arg: win) -> return;
}
// Lockless access of collected samples.
extern "C" {
    pub fn list_last_entry(_arg: &msk->rtx_queue, mptcp_data_frag: struct, _arg: list) -> return;
}
extern "C" {
    pub fn list_first_entry_or_null(_arg: &msk->rtx_queue, mptcp_data_frag: struct, _arg: list) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csum_pseudo_header {
    pub data_seq: __be64,
    pub subflow_seq: __be32,
    pub data_len: __be16,
    pub csum: __sum16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_request_sock {
    pub sk: tcp_request_sock,
    pub 1: allow_join_id0 :,
    pub local_id: u8,
    pub remote_id: u8,
    pub local_key: u64,
    pub idsn: u64,
    pub token: u32,
    pub ssn_offset: u32,
    pub thmac: u64,
    pub local_nonce: u32,
    pub remote_nonce: u32,
    pub msk: *mut mptcp_sock,
    pub token_node: hlist_nulls_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_delegated_action {
    pub napi: napi_struct,
    pub bh_lock: local_lock_t,
    pub head: list_head,
}

pub const MPTCP_DELEGATE_SCHEDULED: c_int = 0;
pub const MPTCP_DELEGATE_SEND: c_int = 1;
pub const MPTCP_DELEGATE_ACK: c_int = 2;
pub const MPTCP_DELEGATE_SNDBUF: c_int = 3;

// MPTCP subflow context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_context {
    pub /: *mut *mut list_head node;/ conn_list of subflows,
    pub /: *mut *mut unsigned long avg_pacing_rate; / protected by msk socket lock,
    pub local_key: u64,
    pub remote_key: u64,
    pub idsn: u64,
    pub map_seq: u64,
    pub rcv_wnd_sent: u64,
    pub snd_isn: u32,
    pub token: u32,
    pub rel_write_seq: u32,
    pub map_subflow_seq: u32,
    pub ssn_offset: u32,
    pub map_data_len: u32,
    pub map_data_csum: __wsum,
    pub map_csum_len: u32,
    pub prev_rtt_seq: u32,
    pub 9: __unused :,
    pub data_avail: bool,
    pub scheduled: bool,
    pub /: *mut *mut bool pm_listener; / a listener managed by the kernel PM?,
    pub /: *mut *mut bool fully_established; / path validated,
    pub lent_mem_frag: u32,
    pub remote_nonce: u32,
    pub thmac: u64,
    pub local_nonce: u32,
    pub remote_token: u32,
    pub /: *mut *mut u8 hmac[MPTCPOPT_HMAC_LEN]; / MPJ subflow only,
    pub /: *mut *mut u64 iasn; / initial ack sequence number, MPC subflows only,
}

// if set the subflow is unable to snd/rcv
// data, the schedule should skip it
//
// protected by the msk socket lock
//
// Use RCU on icsk_ulp_data only for sock diag code
// Convert reset reasons in MPTCP to enum sk_rst_reason type
// It should not happen, or else errors may occur
// in MPTCP layer
//
// Made the fwd mem carried by the given skb available to the msk,
// To be paired with a previous mptcp_subflow_lend_fwdmem() before freeing
// the skb or setting the skb ownership.
//
// The subflow just lend the skb fwd memory; if the subflow meanwhile
// closed, mptcp_close_ssk() already released the ssk rcv memory.
//
extern "C" {
    pub fn mptcp_subflow_process_delegated(ssk: *mut sock, actions: c_long);
}
// the caller held the subflow bh socket lock
// The implied barrier pairs with tcp_release_cb_override()
// mptcp_napi_poll(), and ensures the below list check sees list
// updates done prior to delegated status bits changes
//
extern "C" {
    pub fn __mptcp_inherit_memcg(sk: *mut sock, ssk: *mut sock, gfp: gfp_t);
}
extern "C" {
    pub fn __mptcp_inherit_cgrp_data(sk: *mut sock, ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_is_enabled(net: *const net) -> c_int;
}
extern "C" {
    pub fn mptcp_get_add_addr_timeout(net: *const net) -> c_uint;
}
extern "C" {
    pub fn mptcp_is_checksum_enabled(net: *const net) -> c_int;
}
extern "C" {
    pub fn mptcp_allow_join_id0(net: *const net) -> c_int;
}
extern "C" {
    pub fn mptcp_stale_loss_cnt(net: *const net) -> c_uint;
}
extern "C" {
    pub fn mptcp_close_timeout(sk: *const sock) -> c_uint;
}
extern "C" {
    pub fn mptcp_get_pm_type(net: *const net) -> c_int;
}
extern "C" {
    pub fn mptcp_add_addr_v6_port_drop_ts(net: *const net) -> c_uint;
}
extern "C" {
    pub fn mptcp_active_disable(sk: *mut sock);
}
extern "C" {
    pub fn mptcp_active_should_disable(ssk: *mut sock) -> bool;
}
extern "C" {
    pub fn mptcp_active_enable(sk: *mut sock);
}
extern "C" {
    pub fn mptcp_get_available_schedulers(buf: *mut c_char, maxlen: usize);
}
extern "C" {
    pub fn __mptcp_retransmit_pending_data(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn mptcp_check_and_set_pending(sk: *mut sock);
}
extern "C" {
    pub fn __mptcp_push_pending(sk: *mut sock, flags: c_uint);
}
extern "C" {
    pub fn mptcp_subflow_data_available(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn mptcp_subflow_init() -> void __init;
}
extern "C" {
    pub fn mptcp_subflow_shutdown(sk: *mut sock, ssk: *mut sock, how: c_int);
}
extern "C" {
    pub fn __mptcp_subflow_send_ack(ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_subflow_reset(ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_subflow_queue_clean(sk: *mut sock, ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_sock_graft(sk: *mut sock, parent: *mut socket);
}
extern "C" {
    pub fn __mptcp_close(sk: *mut sock, timeout: c_long) -> bool;
}
extern "C" {
    pub fn mptcp_cancel_work(sk: *mut sock);
}
extern "C" {
    pub fn __mptcp_unaccepted_force_close(sk: *mut sock);
}
extern "C" {
    pub fn mptcp_set_state(sk: *mut sock, state: c_int);
}
// called with sk socket lock held
extern "C" {
    pub fn mptcp_validate_scheduler(sched: *mut mptcp_sched_ops) -> c_int;
}
extern "C" {
    pub fn mptcp_register_scheduler(sched: *mut mptcp_sched_ops) -> c_int;
}
extern "C" {
    pub fn mptcp_unregister_scheduler(sched: *mut mptcp_sched_ops);
}
extern "C" {
    pub fn mptcp_sched_init();
}
extern "C" {
    pub fn mptcp_release_sched(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_sched_get_send(msk: *mut mptcp_sock) -> c_int;
}
extern "C" {
    pub fn mptcp_sched_get_retrans(msk: *mut mptcp_sock) -> c_int;
}
extern "C" {
    pub fn READ_ONCE(READ_ONCE(msk->bytes_consumed: msk->bytes_received) -) -> return;
}
// mptcp doesn't have to deal with small skbs in the receive queue,
// as it can always coalesce them
//
extern "C" {
    pub fn mptcp_set_rcvlowat(sk: *mut sock, val: c_int) -> c_int;
}
// only send if our side has not closed yet
// can't send if JOIN hasn't completed yet (i.e. is usable for mptcp)
extern "C" {
    pub fn __tcp_can_send(_arg: mptcp_subflow_tcp_sock(subflow)) -> return;
}
extern "C" {
    pub fn mptcp_subflow_set_active(subflow: *mut mptcp_subflow_context);
}
extern "C" {
    pub fn mptcp_subflow_active(subflow: *mut mptcp_subflow_context) -> bool;
}
extern "C" {
    pub fn mptcp_subflow_drop_ctx(ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_proto_init() -> void __init;
}

extern "C" {
    pub fn mptcp_proto_v6_init() -> int __init;
}
extern "C" {
    pub fn mptcp_subflow_v6_init() -> void __init;
}

extern "C" {
    pub fn mptcp_finish_connect(sk: *mut sock);
}
extern "C" {
    pub fn __mptcp_sync_state(sk: *mut sock, state: c_int);
}
extern "C" {
    pub fn mptcp_reset_tout_timer(msk: *mut mptcp_sock, fail_tout: c_ulong);
}
// avoid 0 timestamp, as that means no close timeout
extern "C" {
    pub fn div_u64(_arg: tcp_clock_ns(), _arg: NSEC_PER_USEC) -> return;
}
extern "C" {
    pub fn mptcp_data_ready(sk: *mut sock, ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_finish_join(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn mptcp_schedule_work(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn __mptcp_expand_seq(old_seq: u64, cur_seq: u64) -> u64;
}
extern "C" {
    pub fn __mptcp_expand_seq(_arg: old_seq, _arg: cur_seq) -> return;
}
extern "C" {
    pub fn __mptcp_check_push(sk: *mut sock, ssk: *mut sock);
}
extern "C" {
    pub fn __mptcp_data_acked(sk: *mut sock);
}
extern "C" {
    pub fn __mptcp_error_report(sk: *mut sock);
}
extern "C" {
    pub fn mptcp_update_rcv_data_fin(msk: *mut mptcp_sock, data_fin_seq: u64, use_64bit: bool) -> bool;
}
// pairs with memory barrier in mptcp_poll
// the msk max wmem limit is <nr_subflows> * tcp wmem[2]
// The called held both the msk socket and the subflow socket locks,
// possibly under BH
//
// the caller held only the subflow socket lock, either in process or
// BH context. Additionally this can be called under the msk data lock,
// so we can't acquire such lock here: let the delegate action acquires
// the needed locks in suitable order.
//
pub const MPTCP_TOKEN_MAX_RETRIES: c_int = 4;
extern "C" {
    pub fn mptcp_token_init() -> void __init;
}
extern "C" {
    pub fn mptcp_token_new_request(req: *mut request_sock) -> c_int;
}
extern "C" {
    pub fn mptcp_token_destroy_request(req: *mut request_sock);
}
extern "C" {
    pub fn mptcp_token_new_connect(ssk: *mut sock) -> c_int;
}
extern "C" {
    pub fn mptcp_token_exists(token: u32) -> bool;
}
extern "C" {
    pub fn mptcp_token_destroy(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_crypto_key_sha(key: u64, token: *mut u32, idsn: *mut u64);
}
extern "C" {
    pub fn mptcp_crypto_hmac_sha(key1: u64, key2: u64, msg: *mut u8, len: c_int, hmac: *mut c_void);
}
extern "C" {
    pub fn __mptcp_make_csum(data_seq: u64, subflow_seq: u32, data_len: u16, sum: __wsum) -> __sum16;
}
extern "C" {
    pub fn mptcp_pm_init() -> void __init;
}
extern "C" {
    pub fn mptcp_pm_data_init(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_data_reset(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_destroy(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_chk_stale(msk: *const mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_new_connection(msk: *mut mptcp_sock, ssk: *const sock, server_side: c_int);
}
extern "C" {
    pub fn mptcp_pm_fully_established(msk: *mut mptcp_sock, ssk: *const sock);
}
extern "C" {
    pub fn mptcp_pm_allow_new_subflow(msk: *mut mptcp_sock) -> bool;
}
extern "C" {
    pub fn mptcp_pm_connection_closed(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_subflow_established(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_nl_check_work_pending(msk: *mut mptcp_sock) -> bool;
}
extern "C" {
    pub fn mptcp_pm_addr_send_ack(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_nl_rm_addr(msk: *mut mptcp_sock, rm_id: u8);
}
extern "C" {
    pub fn mptcp_pm_mp_prio_received(sk: *mut sock, bkup: u8);
}
extern "C" {
    pub fn mptcp_pm_mp_fail_received(sk: *mut sock, fail_seq: u64);
}
extern "C" {
    pub fn mptcp_pm_announced_has_ssk(msk: *mut mptcp_sock, ssk: *const sock) -> bool;
}
extern "C" {
    pub fn mptcp_pm_remove_addr(msk: *mut mptcp_sock, rm_list: *const mptcp_rm_list) -> c_int;
}
// the default path manager, used in mptcp_pm_unregister
extern "C" {
    pub fn mptcp_pm_register(pm_ops: *mut mptcp_pm_ops) -> c_int;
}
extern "C" {
    pub fn mptcp_pm_unregister(pm_ops: *mut mptcp_pm_ops);
}
extern "C" {
    pub fn mptcp_pm_validate(pm_ops: *mut mptcp_pm_ops) -> c_int;
}
extern "C" {
    pub fn mptcp_pm_get_available(buf: *mut c_char, maxlen: usize);
}
extern "C" {
    pub fn mptcp_userspace_pm_free_local_addr_list(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_event_addr_announced(ssk: *const sock, info: *const mptcp_addr_info);
}
extern "C" {
    pub fn mptcp_event_addr_removed(msk: *const mptcp_sock, id: u8);
}
extern "C" {
    pub fn mptcp_userspace_pm_active(msk: *const mptcp_sock) -> bool;
}
extern "C" {
    pub fn READ_ONCE(BIT(MPTCP_ADD_ADDR_SIGNAL: msk->pm.addr_signal) &) -> return;
}
extern "C" {
    pub fn READ_ONCE(BIT(MPTCP_ADD_ADDR_ECHO: msk->pm.addr_signal) &) -> return;
}
extern "C" {
    pub fn READ_ONCE(BIT(MPTCP_RM_ADDR_SIGNAL: msk->pm.addr_signal) &) -> return;
}
extern "C" {
    pub fn mptcp_pm_get_local_id(msk: *mut mptcp_sock, skc: *mut sock_common) -> c_int;
}
extern "C" {
    pub fn mptcp_pm_is_backup(msk: *mut mptcp_sock, skc: *mut sock_common) -> bool;
}
extern "C" {
    pub fn mptcp_pm_nl_is_backup(msk: *mut mptcp_sock, skc: *mut mptcp_addr_info) -> bool;
}
extern "C" {
    pub fn mptcp_userspace_pm_is_backup(msk: *mut mptcp_sock, skc: *mut mptcp_addr_info) -> bool;
}
extern "C" {
    pub fn mptcp_pm_kernel_register() -> void __init;
}
extern "C" {
    pub fn mptcp_pm_userspace_register() -> void __init;
}
extern "C" {
    pub fn mptcp_pm_nl_init() -> void __init;
}
extern "C" {
    pub fn mptcp_pm_worker(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn __mptcp_pm_kernel_worker(msk: *mut mptcp_sock);
}
extern "C" {
    pub fn mptcp_pm_get_endp_signal_max(msk: *const mptcp_sock) -> u8;
}
extern "C" {
    pub fn mptcp_pm_get_endp_subflow_max(msk: *const mptcp_sock) -> u8;
}
extern "C" {
    pub fn mptcp_pm_get_endp_laminar_max(msk: *const mptcp_sock) -> u8;
}
extern "C" {
    pub fn mptcp_pm_get_endp_fullmesh_max(msk: *const mptcp_sock) -> u8;
}
extern "C" {
    pub fn mptcp_pm_get_limit_add_addr_accepted(msk: *const mptcp_sock) -> u8;
}
extern "C" {
    pub fn mptcp_pm_get_limit_extra_subflows(msk: *const mptcp_sock) -> u8;
}
// called under PM lock
extern "C" {
    pub fn mptcp_sockopt_sync_locked(msk: *mut mptcp_sock, ssk: *mut sock);
}
extern "C" {
    pub fn mptcp_diag_subflow_init(ops: *mut tcp_ulp_ops);
}
extern "C" {
    pub fn test_bit(_arg: MPTCP_FALLBACK_DONE, _arg: &msk->flags) -> return;
}
extern "C" {
    pub fn __mptcp_check_fallback(_arg: msk) -> return;
}
extern "C" {
    pub fn __mptcp_try_fallback(msk: *mut mptcp_sock, fb_mib: c_int) -> bool;
}
// we are in a atomic (BH) scope, override ssk default for data
// fin allocation
//
// Note that the sk state implies !subflow->conn_finished.

extern "C" {
    pub fn mptcp_join_cookie_init() -> void __init;
}

