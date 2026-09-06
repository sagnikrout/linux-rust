//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_conn.h
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
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001, 2002 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

pub const LLC_EVENT: c_int = 1;
pub const LLC_PACKET: c_int = 2;
pub const LLC2_P_TIME: c_int = 2;
pub const LLC2_ACK_TIME: c_int = 1;
pub const LLC2_REJ_TIME: c_int = 3;
pub const LLC2_BUSY_TIME: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_timer {
    pub timer: timer_list,
    pub /: *mut *mut unsigned long expire; / timer expire time,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_sock {
// struct sock must be the first member of llc_sock
    pub sk: sock,
    pub /: *mut *mut sockaddr_llc addr; / address sock is bound to,
    pub /: *mut *mut u8 state; / state of connection,
    pub /: *mut *mut *mut llc_sap sap; / pointer to parent SAP,
    pub /: *mut *mut llc_addr laddr; / lsap/mac pair,
    pub /: *mut *mut llc_addr daddr; / dsap/mac pair,
    pub /: *mut *mut *mut net_device dev; / device to send to remote,
    pub dev_tracker: netdevice_tracker,
    pub /: *mut *mut u32 copied_seq; / head of yet unread data,
    pub /: *mut *mut u8 retry_count; / number of retries,
    pub ack_must_be_send: u8,
    pub first_pdu_Ns: u8,
    pub npta: u8,
    pub ack_timer: llc_timer,
    pub pf_cycle_timer: llc_timer,
    pub rej_sent_timer: llc_timer,
    pub /: *mut *mut llc_timer busy_state_timer; / ind busy clr at remote LLC,
    pub tx'd*/: *mut *mut u8 vS; / seq# next in-seq I-PDU,
    pub rx'd*/: *mut *mut u8 vR; / seq# next in-seq I-PDU,
    pub timeout*/: *mut *mut u32 n2; / max nbr re-tx's for,
    pub /: *mut *mut u32 n1; / max nbr octets in I PDU,
    pub /: *mut *mut u8 k; / tx window size; max = 127,
    pub /: *mut *mut u8 rw; / rx window size; max = 127,
    pub /: *mut *mut u8 p_flag; / state flags,
    pub f_flag: u8,
    pub s_flag: u8,
    pub data_flag: u8,
    pub remote_busy_flag: u8,
    pub cause_flag: u8,
    pub /: *mut *mut sk_buff_head pdu_unack_q; / PUDs sent/waiting ack,
    pub /: *mut *mut u16 link; / network layer link number,
    pub /: *mut *mut u8 X; / a temporary variable,
    pub is: *mut *mut u8 ack_pf; / this flag indicates what,
    pub a: *mut *mut u8 failed_data_req; / recognize that already exist,
    pub dec_step: u8,
    pub inc_cntr: u8,
    pub dec_cntr: u8,
    pub connect_step: u8,
    pub /: *mut *mut u8 last_nr; / NR of last pdu received,
    pub pdu: *mut *mut u32 rx_pdu_hdr; / used for saving header of last,
    pub cmsg_flags: u32,
    pub dev_hash_node: hlist_node,
}

extern "C" {
    pub fn llc_sk_stop_all_timers(sk: *mut sock, sync: bool);
}
extern "C" {
    pub fn llc_sk_free(sk: *mut sock);
}
extern "C" {
    pub fn llc_sk_reset(sk: *mut sock);
}
// Access to a connection
extern "C" {
    pub fn llc_conn_state_process(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_send_pdu(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn llc_conn_rtn_pdu(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn llc_conn_resend_i_pdu_as_cmd(sk: *mut sock, nr: u8, first_p_bit: u8);
}
extern "C" {
    pub fn llc_conn_resend_i_pdu_as_rsp(sk: *mut sock, nr: u8, first_f_bit: u8);
}
extern "C" {
    pub fn llc_conn_remove_acked_pdus(conn: *mut sock, nr: u8, how_many_unacked: *mut u16) -> c_int;
}
extern "C" {
    pub fn llc_sap_add_socket(sap: *mut llc_sap, sk: *mut sock);
}
extern "C" {
    pub fn llc_sap_remove_socket(sap: *mut llc_sap, sk: *mut sock);
}
extern "C" {
    pub fn llc_data_accept_state(state: u8) -> u8;
}
extern "C" {
    pub fn llc_build_offset_table();
}
