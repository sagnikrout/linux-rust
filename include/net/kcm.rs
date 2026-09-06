//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/kcm.h
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
// Kernel Connection Multiplexor
//
// Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_psock_stats {
    pub tx_msgs: c_ulonglong,
    pub tx_bytes: c_ulonglong,
    pub reserved: c_ulonglong,
    pub unreserved: c_ulonglong,
    pub tx_aborts: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_mux_stats {
    pub rx_msgs: c_ulonglong,
    pub rx_bytes: c_ulonglong,
    pub tx_msgs: c_ulonglong,
    pub tx_bytes: c_ulonglong,
    pub rx_ready_drops: c_uint,
    pub tx_retries: c_uint,
    pub psock_attach: c_uint,
    pub psock_unattach_rsvd: c_uint,
    pub psock_unattach: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_stats {
    pub rx_msgs: c_ulonglong,
    pub rx_bytes: c_ulonglong,
    pub tx_msgs: c_ulonglong,
    pub tx_bytes: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_tx_msg {
    pub sent: c_uint,
    pub frag_offset: c_uint,
    pub msg_flags: c_uint,
    pub started_tx: bool,
    pub frag_skb: *mut sk_buff,
    pub last_skb: *mut sk_buff,
}

// Socket structure for KCM client sockets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_sock {
    pub sk: sock,
    pub mux: *mut kcm_mux,
    pub kcm_sock_list: list_head,
    pub index: c_int,
    pub 1: u32 done :,
    pub done_work: work_struct,
    pub stats: kcm_stats,
// Transmit
    pub tx_psock: *mut kcm_psock,
    pub tx_work: work_struct,
    pub wait_psock_list: list_head,
    pub seq_skb: *mut sk_buff,
    pub tx_mutex: mutex,
// Don't use bit fields here, these are set under different locks
    pub tx_wait: bool,
    pub tx_wait_more: bool,
// Receive
    pub rx_psock: *mut kcm_psock,
    pub /: *mut *mut list_head wait_rx_list; / KCMs waiting for receiving,
    pub rx_wait: bool,
    pub 1: u32 rx_disabled :,
}

// Structure for an attached lower socket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_psock {
    pub sk: *mut sock,
    pub strp: strparser,
    pub mux: *mut kcm_mux,
    pub index: c_int,
    pub 1: u32 tx_stopped :,
    pub 1: u32 done :,
    pub 1: u32 unattaching :,
    pub sk): *mut *mut void (save_state_change)(struct sock,
    pub sk): *mut *mut void (save_data_ready)(struct sock,
    pub sk): *mut *mut void (save_write_space)(struct sock,
    pub psock_list: list_head,
    pub stats: kcm_psock_stats,
// Receive
    pub psock_ready_list: list_head,
    pub bpf_prog: *mut bpf_prog,
    pub rx_kcm: *mut kcm_sock,
    pub saved_rx_bytes: c_ulonglong,
    pub saved_rx_msgs: c_ulonglong,
    pub ready_rx_msg: *mut sk_buff,
// Transmit
    pub tx_kcm: *mut kcm_sock,
    pub psock_avail_list: list_head,
    pub saved_tx_bytes: c_ulonglong,
    pub saved_tx_msgs: c_ulonglong,
}

// Per net MUX list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_net {
    pub mutex: mutex,
    pub aggregate_psock_stats: kcm_psock_stats,
    pub aggregate_mux_stats: kcm_mux_stats,
    pub aggregate_strp_stats: strp_aggr_stats,
    pub mux_list: list_head,
    pub count: c_int,
}

// Structure for a MUX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcm_mux {
    pub kcm_mux_list: list_head,
    pub rcu: rcu_head,
    pub knet: *mut kcm_net,
    pub /: *mut *mut list_head kcm_socks; / All KCM sockets on MUX,
    pub /: *mut *mut int kcm_socks_cnt; / Total KCM socket count for MUX,
    pub /: *mut *mut list_head psocks; / List of all psocks on MUX,
    pub /: *mut *mut int psocks_cnt; / Total attached sockets,
    pub stats: kcm_mux_stats,
    pub aggregate_psock_stats: kcm_psock_stats,
    pub aggregate_strp_stats: strp_aggr_stats,
// Receive
    pub ____cacheline_aligned_in_smp: spinlock_t rx_lock,
    pub /: *mut *mut list_head kcm_rx_waiters; / KCMs waiting for receiving,
    pub /: *mut *mut list_head psocks_ready; / List of psocks with a msg ready,
    pub rx_hold_queue: sk_buff_head,
// Transmit
    pub /: *mut *mut spinlock_t lock ____cacheline_aligned_in_smp; / TX and mux locking,
    pub /: *mut *mut list_head psocks_avail; / List of available psocks,
    pub /: *mut *mut list_head kcm_tx_waiters; / KCMs waiting for a TX psock,
}

extern "C" {
    pub fn kcm_proc_init() -> c_int;
}
extern "C" {
    pub fn kcm_proc_exit();
}

// Save psock statistics in the mux when psock is being unattached.

// Save psock statistics in the mux when psock is being unattached.

