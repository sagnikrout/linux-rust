//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/cxgbit/cxgbit.h
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
// Copyright (c) 2016 Chelsio Communications, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_cmd {
    pub sg: scatterlist,
    pub ttinfo: cxgbi_task_tag_info,
    pub setup_ddp: bool,
    pub release: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_iso_info {
    pub flags: u8,
    pub mpdu: u32,
    pub len: u32,
    pub burst_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbit_skcb_flags {
    SKCBF_TX_NEED_HDR	= (1 << 0), /* packet needs a header */
    SKCBF_TX_FLAG_COMPL	= (1 << 1), /* wr completion flag */
    SKCBF_TX_ISO		= (1 << 2), /* iso cpl in tx skb */
    SKCBF_RX_LRO		= (1 << 3), /* lro skb */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_skb_rx_cb {
    pub opcode: u8,
    pub pdu_cb: *mut c_void,
    pub ): *mut *mut *mut void (backlog_fn)(struct cxgbit_sock , struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_skb_tx_cb {
    pub submode: u8,
    pub extra_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cxgbit_skb_cb {
    pub flags: u8,
    pub tx: cxgbit_skb_tx_cb,
    pub rx: cxgbit_skb_rx_cb,
}

// This member must be first.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbit_cdev_flags {
    CDEV_STATE_UP = 0,
    CDEV_ISO_ENABLE,
    CDEV_DDP_ENABLE,
}

pub const NP_INFO_HASH_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct np_info {
    pub next: *mut np_info,
    pub cnp: *mut cxgbit_np,
    pub stid: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_list_head {
    pub list: list_head,
// device lock
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_device {
    pub list: list_head,
    pub lldi: cxgb4_lld_info,
    pub np_hash_tab: [*mut np_info; NP_INFO_HASH_SIZE],
// np lock
    pub np_lock: spinlock_t,
    pub selectq: [u8; MAX_NPORTS][2],
    pub cskq: cxgbit_list_head,
    pub mdsl: u32,
    pub kref: kref,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_wr_wait {
    pub completion: completion,
    pub ret: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbit_csk_state {
    CSK_STATE_IDLE = 0,
    CSK_STATE_LISTEN,
    CSK_STATE_CONNECTING,
    CSK_STATE_ESTABLISHED,
    CSK_STATE_ABORTING,
    CSK_STATE_CLOSING,
    CSK_STATE_MORIBUND,
    CSK_STATE_DEAD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgbit_csk_flags {
    CSK_TX_DATA_SENT = 0,
    CSK_LOGIN_PDU_DONE,
    CSK_LOGIN_DONE,
    CSK_DDP_ENABLE,
    CSK_ABORT_RPL_WAIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_sock_common {
    pub cdev: *mut cxgbit_device,
    pub local_addr: sockaddr_storage,
    pub remote_addr: sockaddr_storage,
    pub wr_wait: cxgbit_wr_wait,
    pub state: cxgbit_csk_state,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_np {
    pub com: cxgbit_sock_common,
    pub accept_wait: wait_queue_head_t,
    pub np: *mut iscsi_np,
    pub accept_comp: completion,
    pub np_accept_list: list_head,
// np accept lock
    pub np_accept_lock: spinlock_t,
    pub kref: kref,
    pub stid: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgbit_sock {
    pub com: cxgbit_sock_common,
    pub cnp: *mut cxgbit_np,
    pub conn: *mut iscsit_conn,
    pub l2t: *mut l2t_entry,
    pub dst: *mut dst_entry,
    pub list: list_head,
    pub rxq: sk_buff_head,
    pub txq: sk_buff_head,
    pub ppodq: sk_buff_head,
    pub backlogq: sk_buff_head,
    pub skbq: sk_buff_head,
    pub wr_pending_head: *mut sk_buff,
    pub wr_pending_tail: *mut sk_buff,
    pub skb: *mut sk_buff,
    pub lro_skb: *mut sk_buff,
    pub lro_hskb: *mut sk_buff,
    pub accept_node: list_head,
// socket lock
    pub lock: spinlock_t,
    pub waitq: wait_queue_head_t,
    pub lock_owner: bool,
    pub kref: kref,
    pub max_iso_npdu: u32,
    pub wr_cred: u32,
    pub wr_una_cred: u32,
    pub wr_max_cred: u32,
    pub snd_una: u32,
    pub tid: u32,
    pub snd_nxt: u32,
    pub rcv_nxt: u32,
    pub smac_idx: u32,
    pub tx_chan: u32,
    pub mtu: u32,
    pub write_seq: u32,
    pub rx_credits: u32,
    pub snd_win: u32,
    pub rcv_win: u32,
    pub mss: u16,
    pub emss: u16,
    pub plen: u16,
    pub rss_qid: u16,
    pub txq_idx: u16,
    pub ctrlq_idx: u16,
    pub tos: u8,
    pub port_id: u8,
pub const CXGBIT_SUBMODE_HCRC: c_uint = 0x1;
pub const CXGBIT_SUBMODE_DCRC: c_uint = 0x2;
    pub submode: u8,

    pub dcb_priority: u8,

    pub snd_wscale: u8,
}

extern "C" {
    pub fn _cxgbit_free_cdev(kref: *mut kref);
}
extern "C" {
    pub fn _cxgbit_free_csk(kref: *mut kref);
}
extern "C" {
    pub fn _cxgbit_free_cnp(kref: *mut kref);
}
extern "C" {
    pub fn cxgbit_setup_np(: *mut iscsi_np, : *mut sockaddr_storage) -> c_int;
}
extern "C" {
    pub fn cxgbit_setup_conn_digest(: *mut cxgbit_sock) -> c_int;
}
extern "C" {
    pub fn cxgbit_accept_np(: *mut iscsi_np, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn cxgbit_free_np(: *mut iscsi_np);
}
extern "C" {
    pub fn cxgbit_abort_conn(csk: *mut cxgbit_sock);
}
extern "C" {
    pub fn cxgbit_free_conn(: *mut iscsit_conn);
}
extern "C" {
    pub fn cxgbit_get_login_rx(: *mut iscsit_conn, : *mut iscsi_login) -> c_int;
}
extern "C" {
    pub fn cxgbit_rx_data_ack(: *mut cxgbit_sock) -> c_int;
}
extern "C" {
    pub fn cxgbit_push_tx_frames(: *mut cxgbit_sock);
}
extern "C" {
    pub fn cxgbit_put_login_tx(: *mut iscsit_conn, : *mut iscsi_login, _arg: u32) -> c_int;
}
extern "C" {
    pub fn cxgbit_send_tx_flowc_wr(: *mut cxgbit_sock) -> u32;
}
extern "C" {
    pub fn cxgbit_ofld_send(: *mut cxgbit_device, : *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn cxgbit_get_rx_pdu(: *mut iscsit_conn);
}
extern "C" {
    pub fn cxgbit_validate_params(: *mut iscsit_conn) -> c_int;
}
// DDP
extern "C" {
    pub fn cxgbit_ddp_init(: *mut cxgbit_device) -> c_int;
}
extern "C" {
    pub fn cxgbit_setup_conn_pgidx(: *mut cxgbit_sock, _arg: u32) -> c_int;
}
extern "C" {
    pub fn cxgbit_reserve_ttt(: *mut cxgbit_sock, : *mut iscsit_cmd) -> c_int;
}
extern "C" {
    pub fn cxgbit_unmap_cmd(: *mut iscsit_conn, : *mut iscsit_cmd);
}
