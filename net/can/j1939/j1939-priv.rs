//! Automatically rewritten from C Header to Rust Module
//! Source: net/can/j1939/j1939-priv.h
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
// Copyright (c) 2010-2011 EIA Electronics,
// Kurt Van Dijck <kurt.van.dijck@eia.be>
// Copyright (c) 2017-2019 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2017-2019 Pengutronix,
// Oleksij Rempel <kernel@pengutronix.de>

// Timeout to receive the abort signal over loop back. In case CAN
// bus is open, the timeout should be triggered.
//
pub const J1939_XTP_ABORT_TIMEOUT_MS: c_int = 500;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum j1939_sk_errqueue_type {
    J1939_ERRQUEUE_TX_ACK,
    J1939_ERRQUEUE_TX_SCHED,
    J1939_ERRQUEUE_TX_ABORT,
    J1939_ERRQUEUE_RX_RTS,
    J1939_ERRQUEUE_RX_DPO,
    J1939_ERRQUEUE_RX_ABORT,
}

// j1939 devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_ecu {
    pub list: list_head,
    pub name: name_t,
    pub addr: u8,
// indicates that this ecu successfully claimed @sa as its address
    pub ac_timer: hrtimer,
    pub kref: kref,
    pub priv: *mut j1939_priv,
    pub priv_dev_tracker: netdevice_tracker,
// count users, to help transport protocol decide for interaction
    pub nusers: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_priv {
    pub ecus: list_head,
// local list entry in priv
// These allow irq (& softirq) context lookups on j1939 devices
// This approach (separate lists) is done as the other 2 alternatives
// are not easier or even wrong
// 1) using the pure kobject methods involves mutexes, which are not
// allowed in irq context.
// 2) duplicating data structures would require a lot of synchronization
// code
// usage:
//
// segments need a lock to protect the above list
    pub lock: rwlock_t,
    pub ndev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
// list of 256 ecu ptrs, that cache the claimed addresses.
// also protected by the above lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_addr_ent {
    pub ecu: *mut j1939_ecu,
// count users, to help transport protocol
    pub nusers: c_int,
    pub ents: [}; 256],
    pub kref: kref,
// List of active sessions to prevent start of conflicting
// one.
//
// Do not start two sessions of same type, addresses and
// direction.
//
    pub active_session_list: list_head,
// protects active_session_list
    pub active_session_list_lock: spinlock_t,
    pub tp_max_packet_size: c_uint,
// lock for j1939_socks list
    pub j1939_socks_lock: rwlock_t,
    pub j1939_socks: list_head,
    pub rx_kref: kref,
    pub rx_tskey: u32,
}

extern "C" {
    pub fn j1939_ecu_put(ecu: *mut j1939_ecu);
}
// keep the cache of what is local
extern "C" {
    pub fn j1939_local_ecu_get(priv: *mut j1939_priv, name: name_t, sa: u8) -> c_int;
}
extern "C" {
    pub fn j1939_local_ecu_put(priv: *mut j1939_priv, name: name_t, sa: u8);
}
// ignore dp & res bits for this
// utility to correctly unmap an ECU
extern "C" {
    pub fn j1939_ecu_unmap_locked(ecu: *mut j1939_ecu);
}
extern "C" {
    pub fn j1939_ecu_unmap(ecu: *mut j1939_ecu);
}
extern "C" {
    pub fn j1939_name_to_addr(priv: *mut j1939_priv, name: name_t) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum j1939_transfer_type {
    J1939_TP,
    J1939_ETP,
    J1939_SIMPLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_addr {
    pub src_name: name_t,
    pub dst_name: name_t,
    pub pgn: pgn_t,
    pub sa: u8,
    pub da: u8,
    pub type: u8,
}

// control buffer of the sk_buff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_sk_buff_cb {
// Offset in bytes within one ETP session
    pub offset: u32,
// for tx, MSG_SYN will be used to sync on sockets
    pub msg_flags: u32,
    pub tskey: u32,
    pub addr: j1939_addr,
// Flags for quick lookups during skb processing.
// These are set in the receive path only.
//

    pub flags: u8,
    pub priority: priority_t,
}

extern "C" {
    pub fn j1939_send_one(priv: *mut j1939_priv, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn j1939_sk_recv(priv: *mut j1939_priv, skb: *mut sk_buff);
}
extern "C" {
    pub fn j1939_sk_send_loop_abort(sk: *mut sock, err: c_int);
}
extern "C" {
    pub fn j1939_sk_queue_activate_next(session: *mut j1939_session);
}
// stack entries
extern "C" {
    pub fn j1939_tp_recv(priv: *mut j1939_priv, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn j1939_ac_fixup(priv: *mut j1939_priv, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn j1939_ac_recv(priv: *mut j1939_priv, skb: *mut sk_buff);
}
extern "C" {
    pub fn j1939_simple_recv(priv: *mut j1939_priv, skb: *mut sk_buff);
}
// network management
extern "C" {
    pub fn j1939_ecu_timer_start(ecu: *mut j1939_ecu);
}
extern "C" {
    pub fn j1939_ecu_timer_cancel(ecu: *mut j1939_ecu);
}
extern "C" {
    pub fn j1939_ecu_unmap_all(priv: *mut j1939_priv);
}
extern "C" {
    pub fn j1939_netdev_stop(priv: *mut j1939_priv);
}
extern "C" {
    pub fn j1939_priv_put(priv: *mut j1939_priv);
}
extern "C" {
    pub fn j1939_priv_get(priv: *mut j1939_priv);
}
// notify/alert all j1939 sockets bound to ifindex
extern "C" {
    pub fn j1939_sk_netdev_event_netdown(priv: *mut j1939_priv);
}
extern "C" {
    pub fn j1939_sk_netdev_event_unregister(priv: *mut j1939_priv);
}
extern "C" {
    pub fn j1939_cancel_active_session(priv: *mut j1939_priv, sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn j1939_tp_init(priv: *mut j1939_priv);
}
// decrement pending skb for a j1939 socket
extern "C" {
    pub fn j1939_sock_pending_del(sk: *mut sock);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum j1939_session_state {
    J1939_SESSION_NEW,
    J1939_SESSION_ACTIVE,
// waiting for abort signal on the bus
    J1939_SESSION_WAITING_ABORT,
    J1939_SESSION_ACTIVE_MAX,
    J1939_SESSION_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_session {
    pub priv: *mut j1939_priv,
    pub priv_dev_tracker: netdevice_tracker,
    pub active_session_list_entry: list_head,
    pub sk_session_queue_entry: list_head,
    pub kref: kref,
    pub sk: *mut sock,
// ifindex, src, dst, pgn define the session block
// the are _never_ modified after insertion in the list
// this decreases locking problems a _lot_
//
    pub skcb: j1939_sk_buff_cb,
    pub skb_queue: sk_buff_head,
// all tx related stuff (last_txcmd, pkt.tx)
// is protected (modified only) with the txtimer hrtimer
// 'total' & 'block' are never changed,
// last_cmd, last & block are protected by ->lock
// this means that the tx may run after cts is received that should
// have stopped tx, but this time discrepancy is never avoided anyhow
//
    pub last_txcmd: u8 last_cmd,,
    pub transmission: bool,
    pub extd: bool,
// Total message size, number of bytes
    pub total_message_size: c_uint,
// Total number of bytes queue from socket to the session
    pub total_queued_size: c_uint,
    pub tx_retry: c_uint,
    pub err: c_int,
    pub tskey: u32,
    pub state: j1939_session_state,
// Packets counters for a (extended) transfer session. The packet is
// maximal of 7 bytes.
//
// total - total number of packets for this session
    pub total: c_uint,
// last - last packet of a transfer block after which
// responder should send ETP.CM_CTS and originator
// ETP.CM_DPO
//
    pub last: c_uint,
// tx - number of packets send by originator node.
// this counter can be set back if responder node
// didn't received all packets send by originator.
//
    pub tx: c_uint,
    pub tx_acked: c_uint,
// rx - number of packets received
    pub rx: c_uint,
// block - amount of packets expected in one block
    pub block: c_uint,
// dpo - ETP.CM_DPO, Data Packet Offset
    pub dpo: c_uint,
    pub pkt: },
    pub rxtimer: hrtimer txtimer,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct j1939_sock {
    pub /: *mut *mut sock sk; / must be first to skip with memset,
    pub priv: *mut j1939_priv,
    pub list: list_head,

    pub state: c_int,
    pub ifindex: c_int,
    pub addr: j1939_addr,
    pub filters_lock: spinlock_t,
    pub filters: *mut j1939_filter,
    pub nfilters: c_int,
    pub pgn_rx_filter: pgn_t,
// j1939 may emit equal PGN (!= equal CAN-id's) out of order
// when transport protocol comes in.
// To allow emitting in order, keep a 'pending' nr. of packets
//
    pub skb_pending: core::sync::atomic::AtomicI32,
    pub waitq: wait_queue_head_t,
// lock for the sk_session_queue list
    pub sk_session_queue_lock: spinlock_t,
    pub sk_session_queue: list_head,
}

extern "C" {
    pub fn container_of(_arg: sk, j1939_sock: struct, _arg: sk) -> return;
}
extern "C" {
    pub fn j1939_session_get(session: *mut j1939_session);
}
extern "C" {
    pub fn j1939_session_put(session: *mut j1939_session);
}
extern "C" {
    pub fn j1939_session_activate(session: *mut j1939_session) -> c_int;
}
extern "C" {
    pub fn j1939_tp_schedule_txtimer(session: *mut j1939_session, msec: c_int);
}
extern "C" {
    pub fn j1939_session_timers_cancel(session: *mut j1939_session);
}
pub const J1939_MIN_TP_PACKET_SIZE: c_int = 9;

pub const J1939_REGULAR: c_int = 0;
pub const J1939_EXTENDED: c_int = 1;
// CAN protocol
