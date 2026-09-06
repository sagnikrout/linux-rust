//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/xdp_sock.h
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
// AF_XDP internal functions
// Copyright(c) 2018 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_umem {
    pub addrs: *mut c_void,
    pub size: u64,
    pub headroom: u32,
    pub chunk_size: u32,
    pub chunks: u32,
    pub npgs: u32,
    pub user: *mut user_struct,
    pub users: refcount_t,
    pub flags: u8,
    pub tx_metadata_len: u8,
    pub zc: bool,
    pub pgs: *mut page,
    pub id: c_int,
    pub xsk_dma_list: list_head,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_map {
    pub map: bpf_map,
    pub /: *mut *mut spinlock_t lock; / Synchronize map updates,
    pub count: core::sync::atomic::AtomicI32,
    pub xsk_map: [*mut xdp_sock __rcu; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_sock {
// struct sock must be the first member of struct xdp_sock
    pub sk: sock,
    pub ____cacheline_aligned_in_smp: *mut *mut xsk_queue rx,
    pub dev: *mut net_device,
    pub umem: *mut xdp_umem,
    pub flush_node: list_head,
    pub pool: *mut xsk_buff_pool,
    pub queue_id: u16,
    pub zc: bool,
    pub sg: bool,
    pub state: },
    pub ____cacheline_aligned_in_smp: *mut *mut xsk_queue tx,
    pub tx_list: list_head,
// record the number of tx descriptors sent by this xsk and
// when it exceeds MAX_PER_SOCKET_BUDGET, an opportunity needs
// to be given to other xsks for sending tx descriptors, thereby
// preventing other XSKs from being starved.
//
    pub tx_budget_spent: u32,
// Statistics
    pub rx_dropped: u64,
    pub rx_queue_full: u64,
// When __xsk_generic_xmit() must return before it sees the EOP descriptor for the current
// packet, the partially built skb is saved here so that packet building can resume in next
// call of __xsk_generic_xmit().
//
    pub skb: *mut sk_buff,
    pub drain_cont: bool,
    pub map_list: list_head,
// Protects map_list
    pub map_list_lock: spinlock_t,
    pub max_tx_budget: u32,
// Protects multiple processes in the control path
    pub mutex: mutex,
    pub /: *mut *mut *mut xsk_queue fq_tmp; / Only as tmp storage before bind,
    pub /: *mut *mut *mut xsk_queue cq_tmp; / Only as tmp storage before bind,
}

//
// AF_XDP TX metadata hooks for network devices.
// The following hooks can be defined; unless noted otherwise, they are
// optional and can be filled with a null pointer.
//
// void (*tmo_request_timestamp)(void *priv)
// Called when AF_XDP frame requested egress timestamp.
//
// u64 (*tmo_fill_timestamp)(void *priv)
// Called when AF_XDP frame, that had requested egress timestamp,
// received a completion. The hook needs to return the actual HW timestamp.
//
// void (*tmo_request_checksum)(u16 csum_start, u16 csum_offset, void *priv)
// Called when AF_XDP frame requested HW checksum offload. csum_start
// indicates position where checksumming should start.
// csum_offset indicates position where checksum should be stored.
//
// void (*tmo_request_launch_time)(u64 launch_time, void *priv)
// Called when AF_XDP frame requested launch time HW offload support.
// launch_time indicates the PTP time at which the device can schedule the
// packet for transmission.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_tx_metadata_ops {
    pub priv): *mut *mut void (tmo_request_timestamp)(void,
    pub priv): *mut *mut u64 (tmo_fill_timestamp)(void,
    pub priv): *mut *mut void (tmo_request_checksum)(u16 csum_start, u16 csum_offset, void,
    pub priv): *mut *mut void (tmo_request_launch_time)(u64 launch_time, void,
}

extern "C" {
    pub fn xsk_generic_rcv(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> c_int;
}
extern "C" {
    pub fn __xsk_map_redirect(xs: *mut xdp_sock, xdp: *mut xdp_buff) -> c_int;
}
extern "C" {
    pub fn __xsk_map_flush(flush_list: *mut list_head);
}
//
// xsk_tx_metadata_to_compl - Save enough relevant metadata information
// to perform tx completion in the future.
// @meta: pointer to AF_XDP metadata area
// @compl: pointer to output struct xsk_tx_metadata_to_compl
//
// This function should be called by the networking device when
// it prepares AF_XDP egress packet. The value of @compl should be stored
// and passed to xsk_tx_metadata_complete upon TX completion.
//
// we can only arrive here if the completion timestamp has been
// requested via XDP_TXMD_FLAGS_TIMESTAMP, see xsk_tx_metadata_request
//
// xsk_tx_metadata_complete - Evaluate AF_XDP TX metadata at completion
// and call appropriate xsk_tx_metadata_ops operation.
// @compl: pointer to completion metadata produced from xsk_tx_metadata_to_compl
// @ops: pointer to struct xsk_tx_metadata_ops
// @priv: pointer to driver-private aread
//
// This function should be called by the networking device upon
// AF_XDP egress completion.
//
// compl->tx_timestamp = ops->tmo_fill_timestamp(priv);

