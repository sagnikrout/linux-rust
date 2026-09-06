//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/vector_kern.h
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
// Copyright (C) 2002 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// Queue structure specially adapted for multiple enqueue/dequeue
// in a mmsgrecv/mmsgsend context
//
// Dequeue method
pub const QUEUE_SENDMSG: c_int = 0;
pub const QUEUE_SENDMMSG: c_int = 1;
pub const VECTOR_RX: c_int = 1;

pub const ETH_MAX_PACKET: c_int = 1500;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector_queue {
    pub mmsg_vector: *mut mmsghdr,
    pub skbuff_vector: *mut c_void,
// backlink to device which owns us
    pub dev: *mut net_device,
    pub head_lock: spinlock_t,
    pub tail_lock: spinlock_t,
    pub queue_depth: core::sync::atomic::AtomicI32,
    pub max_iov_frags: int head, tail, max_depth,,
    pub options: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector_estats {
    pub rx_queue_max: u64,
    pub rx_queue_running_average: u64,
    pub tx_queue_max: u64,
    pub tx_queue_running_average: u64,
    pub rx_encaps_errors: u64,
    pub tx_timeout_count: u64,
    pub tx_restart_queue: u64,
    pub tx_kicks: u64,
    pub tx_flow_control_xon: u64,
    pub tx_flow_control_xoff: u64,
    pub rx_csum_offload_good: u64,
    pub rx_csum_offload_errors: u64,
    pub sg_ok: u64,
    pub sg_linearized: u64,
}

pub const VERIFY_HEADER_OK: c_int = 0;
pub const VERIFY_CSUM_OK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vector_private {
    pub list: list_head,
    pub dev: *mut net_device,
    pub ____cacheline_aligned: napi_napi,
    pub unit: c_int,
// Timeout timer in TX
    pub tl: timer_list,
// Scheduled "remove device" work
    pub reset_tx: work_struct,
    pub fds: *mut vector_fds,
    pub rx_queue: *mut vector_queue,
    pub tx_queue: *mut vector_queue,
    pub rx_irq: c_int,
    pub tx_irq: c_int,
    pub parsed: *mut arglist,
    pub /: *mut *mut *mut void transport_data; / transport specific params if needed,
    pub max_packet: c_int,
    pub /: *mut *mut int req_size; / different from max packet - used for TSO,
    pub headroom: c_int,
    pub options: c_int,
// remote address if any - some transports will leave this as null
    pub header_size: c_int,
    pub rx_header_size: c_int,
    pub coalesce: c_int,
    pub header_rxbuffer: *mut c_void,
    pub header_txbuffer: *mut c_void,
    pub vp): *mut *mut sk_buff skb, vector_private,
    pub vp): *mut *mut sk_buff skb, vector_private,
    pub stats_lock: spinlock_t,
    pub rexmit_scheduled: bool,
    pub opened: bool,
    pub in_write_poll: bool,
    pub in_error: bool,
// guest allowed to use ethtool flash to load bpf
    pub bpf_via_flash: bool,
// ethtool stats
    pub estats: vector_estats,
    pub bpf: *mut sock_fprog,
    pub user: [c_char; ],
}

extern "C" {
    pub fn build_transport_data(vp: *mut vector_private) -> c_int;
}
