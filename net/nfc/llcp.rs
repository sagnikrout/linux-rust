//! Automatically rewritten from C Header to Rust Module
//! Source: net/nfc/llcp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2011  Intel Corporation. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum llcp_state {
    LLCP_CONNECTED = 1, /* wait_for_packet() wants that */
    LLCP_CONNECTING,
    LLCP_CLOSED,
    LLCP_BOUND,
    LLCP_LISTEN,
}

pub const LLCP_DEFAULT_LTO: c_int = 100;
pub const LLCP_DEFAULT_RW: c_int = 1;
pub const LLCP_DEFAULT_MIU: c_int = 128;
pub const LLCP_MAX_LTO: c_uint = 0xff;
pub const LLCP_MAX_RW: c_int = 15;
pub const LLCP_MAX_MIUX: c_uint = 0x7ff;

pub const LLCP_WKS_NUM_SAP: c_int = 16;
pub const LLCP_SDP_NUM_SAP: c_int = 16;
pub const LLCP_LOCAL_NUM_SAP: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llcp_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llcp_sdp_tlv {
    pub tlv: *mut u8,
    pub tlv_len: u8,
    pub uri: *mut c_char,
    pub tid: u8,
    pub sap: u8,
    pub time: c_ulong,
    pub node: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llcp_local {
    pub list: list_head,
    pub dev: *mut nfc_dev,
    pub ref: kref,
    pub sdp_lock: mutex,
    pub link_timer: timer_list,
    pub tx_queue: sk_buff_head,
    pub tx_work: work_struct,
    pub rx_work: work_struct,
    pub rx_pending: *mut sk_buff,
    pub timeout_work: work_struct,
    pub target_idx: u32,
    pub rf_mode: u8,
    pub comm_mode: u8,
    pub lto: u8,
    pub rw: u8,
    pub miux: __be16,
    pub /: *mut *mut unsigned long local_wks; / Well known services,
    pub /: *mut *mut unsigned long local_sdp; / Local services,
    pub /: *mut *mut unsigned long local_sap; / Local SAPs, not available for discovery,
    pub local_sdp_cnt: [core::sync::atomic::AtomicI32; LLCP_SDP_NUM_SAP],
// local
    pub gb: [u8; NFC_MAX_GT_LEN],
    pub gb_len: u8,
// remote
    pub remote_gb: [u8; NFC_MAX_GT_LEN],
    pub remote_gb_len: u8,
    pub remote_version: u8,
    pub remote_miu: u16,
    pub remote_lto: u16,
    pub remote_opt: u8,
    pub remote_wks: u16,
    pub sdreq_lock: mutex,
    pub pending_sdreqs: hlist_head,
    pub sdreq_timer: timer_list,
    pub sdreq_timeout_work: work_struct,
    pub sdreq_next_tid: u8,
// sockets array
    pub sockets: llcp_sock_list,
    pub connecting_sockets: llcp_sock_list,
    pub raw_sockets: llcp_sock_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llcp_sock {
    pub sk: sock,
    pub dev: *mut nfc_dev,
    pub local: *mut nfc_llcp_local,
    pub target_idx: u32,
    pub nfc_protocol: u32,
// Link parameters
    pub ssap: u8,
    pub dsap: u8,
    pub service_name: *mut c_char,
    pub service_name_len: usize,
    pub rw: u8,
    pub miux: __be16,
// Remote link parameters
    pub remote_rw: u8,
    pub remote_miu: u16,
// Link variables
    pub send_n: u8,
    pub send_ack_n: u8,
    pub recv_n: u8,
    pub recv_ack_n: u8,
// Is the remote peer ready to receive
    pub remote_ready: u8,
// Reserved source SAP
    pub reserved_ssap: u8,
    pub tx_queue: sk_buff_head,
    pub tx_pending_queue: sk_buff_head,
    pub accept_queue: list_head,
    pub parent: *mut sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_llcp_ui_cb {
    pub dsap: __u8,
    pub ssap: __u8,
}

pub const LLCP_HEADER_SIZE: c_int = 2;
pub const LLCP_SEQUENCE_SIZE: c_int = 1;
pub const LLCP_AGF_PDU_HEADER_SIZE: c_int = 2;
// LLCP versions: 1.1 is 1.0 plus SDP
pub const LLCP_VERSION_10: c_uint = 0x10;
pub const LLCP_VERSION_11: c_uint = 0x11;
// LLCP PDU types
pub const LLCP_PDU_SYMM: c_uint = 0x0;
pub const LLCP_PDU_PAX: c_uint = 0x1;
pub const LLCP_PDU_AGF: c_uint = 0x2;
pub const LLCP_PDU_UI: c_uint = 0x3;
pub const LLCP_PDU_CONNECT: c_uint = 0x4;
pub const LLCP_PDU_DISC: c_uint = 0x5;
pub const LLCP_PDU_CC: c_uint = 0x6;
pub const LLCP_PDU_DM: c_uint = 0x7;
pub const LLCP_PDU_FRMR: c_uint = 0x8;
pub const LLCP_PDU_SNL: c_uint = 0x9;
pub const LLCP_PDU_I: c_uint = 0xc;
pub const LLCP_PDU_RR: c_uint = 0xd;
pub const LLCP_PDU_RNR: c_uint = 0xe;
// Parameters TLV types
pub const LLCP_TLV_VERSION: c_uint = 0x1;
pub const LLCP_TLV_MIUX: c_uint = 0x2;
pub const LLCP_TLV_WKS: c_uint = 0x3;
pub const LLCP_TLV_LTO: c_uint = 0x4;
pub const LLCP_TLV_RW: c_uint = 0x5;
pub const LLCP_TLV_SN: c_uint = 0x6;
pub const LLCP_TLV_OPT: c_uint = 0x7;
pub const LLCP_TLV_SDREQ: c_uint = 0x8;
pub const LLCP_TLV_SDRES: c_uint = 0x9;
pub const LLCP_TLV_MAX: c_uint = 0xa;
// Well known LLCP SAP
pub const LLCP_SAP_SDP: c_uint = 0x1;
pub const LLCP_SAP_IP: c_uint = 0x2;
pub const LLCP_SAP_OBEX: c_uint = 0x3;
pub const LLCP_SAP_SNEP: c_uint = 0x4;
pub const LLCP_SAP_MAX: c_uint = 0xff;
// Disconnection reason code
pub const LLCP_DM_DISC: c_uint = 0x00;
pub const LLCP_DM_NOCONN: c_uint = 0x01;
pub const LLCP_DM_NOBOUND: c_uint = 0x02;
pub const LLCP_DM_REJ: c_uint = 0x03;
extern "C" {
    pub fn nfc_llcp_sock_link(l: *mut llcp_sock_list, s: *mut sock);
}
extern "C" {
    pub fn nfc_llcp_sock_unlink(l: *mut llcp_sock_list, s: *mut sock);
}
extern "C" {
    pub fn nfc_llcp_socket_remote_param_init(sock: *mut nfc_llcp_sock);
}
extern "C" {
    pub fn nfc_llcp_local_put(local: *mut nfc_llcp_local) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_get_local_ssap(local: *mut nfc_llcp_local) -> u8;
}
extern "C" {
    pub fn nfc_llcp_put_ssap(local: *mut nfc_llcp_local, ssap: u8);
}
extern "C" {
    pub fn nfc_llcp_queue_i_frames(sock: *mut nfc_llcp_sock) -> c_int;
}
// Sock API
extern "C" {
    pub fn nfc_llcp_sock_free(sock: *mut nfc_llcp_sock);
}
extern "C" {
    pub fn nfc_llcp_accept_unlink(sk: *mut sock);
}
extern "C" {
    pub fn nfc_llcp_accept_enqueue(parent: *mut sock, sk: *mut sock);
}
// TLV API
// Commands API
extern "C" {
    pub fn nfc_llcp_recv(data: *mut c_void, skb: *mut sk_buff, err: c_int);
}
extern "C" {
    pub fn nfc_llcp_free_sdp_tlv(sdp: *mut nfc_llcp_sdp_tlv);
}
extern "C" {
    pub fn nfc_llcp_free_sdp_tlv_list(sdp_head: *mut hlist_head);
}
extern "C" {
    pub fn nfc_llcp_recv(data: *mut c_void, skb: *mut sk_buff, err: c_int);
}
extern "C" {
    pub fn nfc_llcp_send_symm(dev: *mut nfc_dev) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_send_connect(sock: *mut nfc_llcp_sock) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_send_cc(sock: *mut nfc_llcp_sock) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_send_dm(local: *mut nfc_llcp_local, ssap: u8, dsap: u8, reason: u8) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_send_disconnect(sock: *mut nfc_llcp_sock) -> c_int;
}
extern "C" {
    pub fn nfc_llcp_send_rr(sock: *mut nfc_llcp_sock) -> c_int;
}
// Socket API
extern "C" {
    pub fn nfc_llcp_sock_init() -> int __init;
}
extern "C" {
    pub fn nfc_llcp_sock_exit();
}
