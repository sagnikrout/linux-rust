//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/ipoib.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation.
//
// This file contains HFI1 support for IPOIB functionality
//

pub const HFI1_IPOIB_ENTROPY_SHIFT: c_int = 24;
pub const HFI1_IPOIB_TXREQ_NAME_LEN: c_int = 32;
pub const HFI1_IPOIB_PSEUDO_LEN: c_int = 20;
pub const HFI1_IPOIB_ENCAP_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hfi1_ipoib_flow {
    pub as_int: u16,
    pub tx_queue: u8,
    pub sc5: u8,
    pub __attribute__((__packed__)): },
}

//
// struct ipoib_txreq - IPOIB transmit descriptor
// @txreq: sdma transmit request
// @sdma_hdr: 9b ib headers
// @sdma_status: status returned by sdma engine
// @complete: non-zero implies complete
// @priv: ipoib netdev private data
// @txq: txq on which skb was output
// @skb: skb to send
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipoib_txreq {
    pub txreq: sdma_txreq,
    pub sdma_hdr: *mut hfi1_sdma_header,
    pub sdma_status: c_int,
    pub complete: c_int,
    pub priv: *mut hfi1_ipoib_dev_priv,
    pub txq: *mut hfi1_ipoib_txq,
    pub skb: *mut sk_buff,
}

//
// struct hfi1_ipoib_circ_buf - List of items to be processed
// @items: ring of items each a power of two size
// @max_items: max items + 1 that the ring can contain
// @shift: log2 of size for getting txreq
// @sent_txreqs: count of txreqs posted to sdma
// @tail: ring tail
// @stops: count of stops of queue
// @ring_full: ring has been filled
// @no_desc: descriptor shortage seen
// @complete_txreqs: count of txreqs completed by sdma
// @head: ring head
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ipoib_circ_buf {
    pub items: *mut c_void,
    pub max_items: u32,
    pub shift: u32,
// consumer cache line
    pub sent_txreqs: u64 ____cacheline_aligned_in_smp,
    pub avail: u32,
    pub tail: u32,
    pub stops: core::sync::atomic::AtomicI32,
    pub ring_full: core::sync::atomic::AtomicI32,
    pub no_desc: core::sync::atomic::AtomicI32,
// producer cache line
    pub complete_txreqs: u64 ____cacheline_aligned_in_smp,
    pub head: u32,
}

//
// struct hfi1_ipoib_txq - IPOIB per Tx queue information
// @priv: private pointer
// @sde: sdma engine
// @tx_list: tx request list
// @sent_txreqs: count of txreqs posted to sdma
// @flow: tracks when list needs to be flushed for a flow change
// @q_idx: ipoib Tx queue index
// @pkts_sent: indicator packets have been sent from this queue
// @wait: iowait structure
// @napi: pointer to tx napi interface
// @tx_ring: ring of ipoib txreqs to be reaped by napi callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ipoib_txq {
    pub napi: napi_struct,
    pub priv: *mut hfi1_ipoib_dev_priv,
    pub sde: *mut sdma_engine,
    pub tx_list: list_head,
    pub flow: hfi1_ipoib_flow,
    pub q_idx: u8,
    pub pkts_sent: bool,
    pub wait: iowait,
    pub tx_ring: hfi1_ipoib_circ_buf ____cacheline_aligned_in_smp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ipoib_dev_priv {
    pub dd: *mut hfi1_devdata,
    pub netdev: *mut net_device,
    pub device: *mut ib_device,
    pub txqs: *mut hfi1_ipoib_txq,
    pub netdev_ops: *const net_device_ops,
    pub qp: *mut rvt_qp,
    pub qkey: u32,
    pub pkey: u16,
    pub pkey_index: u16,
    pub port_num: u8,
}

// hfi1 ipoib rdma netdev's private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_ipoib_rdma_netdev {
    pub /: *mut *mut rdma_netdev rn; / keep this first,
// followed by device private data
    pub dev_priv: hfi1_ipoib_dev_priv,
}

extern "C" {
    pub fn hfi1_ipoib_txreq_init(priv: *mut hfi1_ipoib_dev_priv) -> c_int;
}
extern "C" {
    pub fn hfi1_ipoib_txreq_deinit(priv: *mut hfi1_ipoib_dev_priv);
}
extern "C" {
    pub fn hfi1_ipoib_rxq_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn hfi1_ipoib_rxq_deinit(dev: *mut net_device);
}
extern "C" {
    pub fn hfi1_ipoib_napi_tx_enable(dev: *mut net_device);
}
extern "C" {
    pub fn hfi1_ipoib_napi_tx_disable(dev: *mut net_device);
}
extern "C" {
    pub fn hfi1_ipoib_tx_timeout(dev: *mut net_device, q: c_uint);
}
