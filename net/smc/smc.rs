//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc.h
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
//
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// Definitions for the SMC module (socket related)
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
//

pub const SMC_RELEASE_0: c_int = 0;
pub const SMC_RELEASE_1: c_int = 1;

pub const SMC_AUTOCORKING_DEFAULT_SIZE: c_uint = 0x10000	/* 64K by default */;
extern "C" {
    pub fn smc_hash_sk(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn smc_unhash_sk(sk: *mut sock);
}
extern "C" {
    pub fn smc_release_cb(sk: *mut sock);
}
extern "C" {
    pub fn smc_release(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn smc_listen(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn smc_shutdown(sock: *mut socket, how: c_int) -> c_int;
}
extern "C" {
    pub fn smc_sendmsg(sock: *mut socket, msg: *mut msghdr, len: usize) -> c_int;
}
// smc sock initialization
extern "C" {
    pub fn smc_sk_init(net: *mut net, sk: *mut sock, protocol: c_int);
}
// clcsock initialization
extern "C" {
    pub fn smc_create_clcsk(net: *mut net, sk: *mut sock, family: c_int) -> c_int;
}

// Macro flag: #define KERNEL_HAS_ATOMIC64

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_state {
    SMC_ACTIVE	= 1,
    SMC_INIT	= 2,
    SMC_CLOSED	= 7,
    SMC_LISTEN	= 10,
// normal close
    SMC_PEERCLOSEWAIT1	= 20,
    SMC_PEERCLOSEWAIT2	= 21,
    SMC_APPFINCLOSEWAIT	= 24,
    SMC_APPCLOSEWAIT1	= 22,
    SMC_APPCLOSEWAIT2	= 23,
    SMC_PEERFINCLOSEWAIT	= 25,
// abnormal close
    SMC_PEERABORTWAIT	= 26,
    SMC_PROCESSABORT	= 27,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_supplemental_features {
    SMC_SPF_EMULATED_ISM_DEV	= 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_wr_rx_hdr {
    pub type: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_conn_state_flags {

    pub /: *mut *mut u8 peer_done_writing : 1; / Sending done indicator,
    pub /: *mut *mut u8 peer_conn_closed : 1; / Peer connection closed indicator,
    pub /: *mut *mut u8 peer_conn_abort : 1; / Abnormal close indicator,
    pub 5: u8 reserved :,

    pub 5: u8 reserved :,
    pub 1: u8 peer_conn_abort :,
    pub 1: u8 peer_conn_closed :,
    pub 1: u8 peer_done_writing :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_producer_flags {

    pub /: *mut *mut u8 write_blocked : 1; / Writing Blocked, no rx buf space,
    pub /: *mut *mut u8 urg_data_pending : 1; / Urgent Data Pending,
    pub /: *mut *mut u8 urg_data_present : 1; / Urgent Data Present,
    pub /: *mut *mut u8 cons_curs_upd_req : 1; / cursor update requested,
    pub /: *mut *mut u8 failover_validation : 1;/ message replay due to failover,
    pub 3: u8 reserved :,

    pub 3: u8 reserved :,
    pub 1: u8 failover_validation :,
    pub 1: u8 cons_curs_upd_req :,
    pub 1: u8 urg_data_present :,
    pub 1: u8 urg_data_pending :,
    pub 1: u8 write_blocked :,

}

// in host byte order
#[repr(C)]
#[derive(Copy, Clone)]
pub union smc_host_cursor {
    pub reserved: u16,
    pub /: *mut *mut u16 wrap; / window wrap sequence number,
    pub /: *mut *mut u32 count; / cursor (= offset) part,
}

// in host byte order, except for flag bitfields in network byte order
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_host_cdc_msg {
    pub /: *mut *mut smc_wr_rx_hdr common; / .type = 0xFE,
    pub /: *mut *mut u8 len; / length = 44,
    pub /: *mut *mut u16 seqno; / connection seq #,
    pub /: *mut *mut u32 token; / alert_token,
    pub /: *mut *mut smc_host_cursor prod; / producer cursor,
    pub cursor,: *mut *mut smc_host_cursor cons; / consumer,
// piggy backed "ack"
//
    pub /: *mut *mut smc_cdc_producer_flags prod_flags; / conn. tx/rx status,
    pub status*/: *mut *mut smc_cdc_conn_state_flags conn_state_flags; / peer conn.,
    pub reserved: [u8; 18],
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smc_urg_state {
    SMC_URG_VALID	= 1,			/* data present */
    SMC_URG_NOTYET	= 2,			/* data pending */
    SMC_URG_READ	= 3,			/* data was already read */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_mark_woken {
    pub woken: bool,
    pub key: *mut c_void,
    pub wait_entry: wait_queue_entry_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_connection {
    pub alert_node: rb_node,
    pub /: *mut *mut *mut smc_link_group lgr; / link group of connection,
    pub /: *mut *mut *mut smc_link lnk; / assigned SMC-R link,
    pub /: *mut *mut u32 alert_token_local; / unique conn. id,
    pub /: *mut *mut u8 peer_rmbe_idx; / from tcp handshake,
    pub /: *mut *mut int peer_rmbe_size; / size of peer rx buffer,
    pub peer: *mut *mut atomic_t peer_rmbe_space;/ remaining free bytes in,
// rmbe
//
    pub /: *mut *mut int rtoken_idx; / idx to peer RMB rkey/addr,
    pub /: *mut *mut *mut smc_buf_desc sndbuf_desc; / send buffer descriptor,
    pub /: *mut *mut *mut smc_buf_desc rmb_desc; / RMBE descriptor,
    pub /: *mut *mut int rmbe_size_comp; / compressed notation,
    pub rmbe_update_limit: c_int,
// lower limit for consumer
// cursor update
//
    pub staging: *mut *mut smc_host_cdc_msg local_tx_ctrl; / host byte order,
// buffer for CDC msg send
// .prod cf. TCP snd_nxt
// .cons cf. TCP sends ack
//
    pub local_tx_ctrl_fin: smc_host_cursor,
// prod crsr - confirmed by peer
//
    pub data: *mut *mut smc_host_cursor tx_curs_prep; / tx - prepared,
// snd_max..wmem_alloc
//
    pub data: *mut *mut smc_host_cursor tx_curs_sent; / tx - sent,
// snd_nxt ?
//
    pub peer: *mut *mut smc_host_cursor tx_curs_fin; / tx - confirmed by,
// snd-wnd-begin ?
//
    pub /: *mut *mut atomic_t sndbuf_space; / remaining space in sndbuf,
    pub /: *mut *mut u16 tx_cdc_seq; / sequence # for CDC send,
    pub /: *mut *mut u16 tx_cdc_seq_fin; / sequence # - tx completed,
    pub /: *mut *mut spinlock_t send_lock; / protect wr_sends,
    pub wqe: *mut *mut atomic_t cdc_pend_tx_wr; / number of pending tx CDC,
// - inc when post wqe,
// - dec on polled tx cqe
//
    pub cdc_pend_tx_wr*/: *mut *mut wait_queue_head_t cdc_pend_tx_wq; / wakeup on no,
    pub /: *mut *mut delayed_work tx_work; / retry of smc_cdc_msg_send,
    pub /: *mut *mut u32 tx_off; / base offset in peer rmb,
    pub event_handl.: *mut *mut smc_host_cdc_msg local_rx_ctrl; / filled during,
// .prod cf. TCP rcv_nxt
// .cons cf. TCP snd_una
//
    pub peer: *mut *mut smc_host_cursor rx_curs_confirmed; / confirmed to,
// source of snd_una ?
//
    pub /: *mut *mut smc_host_cursor urg_curs; / points at urgent byte,
    pub urg_state: smc_urg_state,
    pub /: *mut *mut bool urg_tx_pend; / urgent data staged,
    pub urg_rx_skip_pend: bool,
// indicate urgent oob data
// read, but previous regular
// data still pending
//
    pub /: *mut *mut char urg_rx_byte; / urgent byte,
    pub tx_in_release_sock: bool,
// flush pending tx data in
// sock release_cb()
//
    pub data,: *mut *mut atomic_t bytes_to_rcv; / arrived,
// not yet received
//
    pub bytes: *mut *mut atomic_t splice_pending; / number of spliced,
// pending processing
//

    pub /: *mut *mut spinlock_t acurs_lock; / protect cursors,

    pub /: *mut *mut work_close_work; / peer sent some closing,
    pub /: *mut *mut work_abort_work; / abort the connection,
    pub /: *mut *mut tasklet_rx_tsklet; / Receiver tasklet for SMC-D,
    pub offset:: *mut *mut u8 rx_off; / receive,
// 0 for SMC-R, 32 for SMC-D
//
    pub /: *mut *mut u64 peer_token; / SMC-D token of peer,
    pub /: *mut *mut u8 killed; / abnormal termination,
    pub /: *mut *mut u8 freed; / normal termination,
    pub /: *mut *mut u8 out_of_sync; / out of sync with peer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_sock {
    pub sk: sock,
    pub icsk_inet: inet_sock,
}

// original stat_change fct.
// original data_ready fct.
// original write_space fct.
// original error_report fct.
// original af ops
// sockopt TCP_DEFER_ACCEPT
// value
//
// shutdown wr or close
// started, waiting for unsent
// data to be sent
//
// non-blocking connect in
// flight
//
// protects clcsock of a listen
// socket
//

// save target_cb in saved_cb, and replace target_cb with new_cb
// only save once
// saved_cb = *target_cb;
// target_cb = new_cb;
// restore target_cb to saved_cb, and reset saved_cb to NULL
// target_cb = *saved_cb;
// saved_cb = NULL;
pub const SMC_SYSTEMID_LEN: c_int = 8;

// convert an u32 value into network byte order, store it into a 3 byte field
// convert a received 3 byte field into host byte order
extern "C" {
    pub fn be32_to_cpu(_arg: t) -> return;
}

extern "C" {
    pub fn smc_close_non_accepted(sk: *mut sock);
}
// smc handshake limitation interface for netlink
extern "C" {
    pub fn smc_nl_dump_hs_limitation(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn smc_nl_enable_hs_limitation(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn smc_nl_disable_hs_limitation(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
