//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_cdc.h
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
// Connection Data Control (CDC)
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <ubraun@linux.vnet.ibm.com>
//

pub const SMC_CDC_MSG_TYPE: c_uint = 0xFE;
// in network byte order
#[repr(C)]
#[derive(Copy, Clone)]
pub union smc_cdc_cursor {
    pub reserved: __be16,
    pub wrap: __be16,
    pub count: __be32,
}

// in network byte order
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_msg {
    pub /: *mut *mut smc_wr_rx_hdr common; / .type = 0xFE,
    pub /: *mut *mut u8 len; / 44,
    pub seqno: __be16,
    pub token: __be32,
    pub prod: smc_cdc_cursor,
    pub /: *mut *mut smc_cdc_cursor cons; / piggy backed "ack",
    pub prod_flags: smc_cdc_producer_flags,
    pub conn_state_flags: smc_cdc_conn_state_flags,
    pub reserved: [u8; 18],
}

// SMC-D cursor format
#[repr(C)]
#[derive(Copy, Clone)]
pub union smcd_cdc_cursor {
    pub wrap: u16,
    pub count: u32,
    pub prod_flags: smc_cdc_producer_flags,
    pub conn_state_flags: smc_cdc_conn_state_flags,
    pub __packed: },

    pub /: *mut *mut atomic64_t acurs; / for atomic processing,

    pub /: *mut *mut u64 acurs; / for atomic processing,

    pub __aligned(8): },
// CDC message for SMC-D
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_cdc_msg {
    pub /: *mut *mut smc_wr_rx_hdr common; / Type = 0xFE,
    pub res1: [u8; 7],
    pub prod: smcd_cdc_cursor,
    pub cons: smcd_cdc_cursor,
    pub res3: [u8; 8],
    pub __aligned(8): },
    pub value: curs->count +=,
    pub size: curs->count -=,
// Copy cursor src into tgt

    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&conn->acurs_lock,,
    pub src->acurs: tgt->acurs =,
    pub flags): spin_unlock_irqrestore(&conn->acurs_lock,,

    pub atomic64_read(&src->acurs)): atomic64_set(&tgt->acurs,,

    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&conn->acurs_lock,,
    pub src->acurs: tgt->acurs =,
    pub flags): spin_unlock_irqrestore(&conn->acurs_lock,,

    pub atomic64_read(&src->acurs)): atomic64_set(&tgt->acurs,,

    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&conn->acurs_lock,,
    pub src->acurs: tgt->acurs =,
    pub flags): spin_unlock_irqrestore(&conn->acurs_lock,,

    pub atomic64_read(&src->acurs)): atomic64_set(&tgt->acurs,,

// calculate cursor difference between old and new, where old <= new and
// difference cannot exceed size
//
    pub new->count)): ((size - old->count) +,
    pub old->count)): return max_t(int, 0, (new->count -,
// calculate cursor difference between old and new - returns negative
// value in case old > new
//
    pub old): return -smc_curs_diff(size, new,,
    pub new): return smc_curs_diff(size, old,,
// calculate cursor difference between old and new, where old <= new and
// difference may exceed size
//
    pub old->count)): return max_t(int, 0, (new->count -,
    pub conn): smc_curs_copy(save, local,,
    pub htonl(save->count): peer->count =,
    pub htons(save->wrap): peer->wrap =,
// peer->reserved = htons(0); must be ensured by caller
    pub &conn->local_tx_ctrl: *mut *mut smc_host_cdc_msg local =,
    pub local->common.type: peer->common.type =,
    pub local->len: peer->len =,
    pub htons(local->seqno): peer->seqno =,
    pub htonl(local->token): peer->token =,
    pub conn): smc_host_cursor_to_cdc(&peer->prod, &local->prod, save,,
    pub conn): smc_host_cursor_to_cdc(&peer->cons, &local->cons, save,,
    pub local->prod_flags: peer->prod_flags =,
    pub local->conn_state_flags: peer->conn_state_flags =,
    pub old: smc_host_cursor temp,,
    pub net: smc_cdc_cursor,
    pub conn): smc_curs_copy(&old, local,,
    pub conn): smc_curs_copy_net(&net, peer,,
    pub ntohl(net.count): temp.count =,
    pub ntohs(net.wrap): temp.wrap =,
    pub conn): smc_curs_copy(local, &temp,,
    pub peer->common.type: local->common.type =,
    pub peer->len: local->len =,
    pub ntohs(peer->seqno): local->seqno =,
    pub ntohl(peer->token): local->token =,
    pub conn): smc_cdc_cursor_to_host(&local->prod, &peer->prod,,
    pub conn): smc_cdc_cursor_to_host(&local->cons, &peer->cons,,
    pub peer->prod_flags: local->prod_flags =,
    pub peer->conn_state_flags: local->conn_state_flags =,
    pub temp: smc_host_cursor,
    pub peer->prod.wrap: temp.wrap =,
    pub peer->prod.count: temp.count =,
    pub conn): smc_curs_copy(&local->prod, &temp,,
    pub peer->cons.wrap: temp.wrap =,
    pub peer->cons.count: temp.count =,
    pub conn): smc_curs_copy(&local->cons, &temp,,
    pub peer->cons.prod_flags: local->prod_flags =,
    pub peer->cons.conn_state_flags: local->conn_state_flags =,
    pub conn): *mut *mut smcd_cdc_msg_to_host(local, (struct smcd_cdc_msg )peer,,
    pub conn): smcr_cdc_msg_to_host(local, peer,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_cdc_tx_pend {
    pub /: *mut *mut *mut smc_connection conn; / socket connection,
    pub /: *mut *mut smc_host_cursor cursor; / tx sndbuf cursor sent,
    pub /: *mut *mut smc_host_cursor p_cursor; / rx RMBE cursor produced,
    pub /: *mut *mut u16 ctrl_seq; / conn. tx sequence #,
}

extern "C" {
    pub fn smc_cdc_wait_pend_tx_wr(conn: *mut smc_connection);
}
extern "C" {
    pub fn smc_cdc_get_slot_and_msg_send(conn: *mut smc_connection) -> c_int;
}
extern "C" {
    pub fn smcd_cdc_msg_send(conn: *mut smc_connection) -> c_int;
}
extern "C" {
    pub fn smcd_cdc_rx_init(conn: *mut smc_connection);
}
