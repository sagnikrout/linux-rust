//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/xdp.h
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
// Copyright (C) 2025 Intel Corporation

extern "C" {
    pub fn idpf_xdp_rxq_info_init(rxq: *mut idpf_rx_queue) -> c_int;
}
extern "C" {
    pub fn idpf_xdp_rxq_info_init_all(rsrc: *const idpf_q_vec_rsrc) -> c_int;
}
extern "C" {
    pub fn idpf_xdp_rxq_info_deinit(rxq: *mut idpf_rx_queue, model: u32);
}
extern "C" {
    pub fn idpf_xdp_rxq_info_deinit_all(rsrc: *const idpf_q_vec_rsrc);
}
extern "C" {
    pub fn idpf_xdpsqs_get(vport: *const idpf_vport) -> c_int;
}
extern "C" {
    pub fn idpf_xdpsqs_put(vport: *const idpf_vport);
}
extern "C" {
    pub fn idpf_xdpsq_poll(xdpsq: *mut idpf_tx_queue, budget: u32) -> u32;
}
extern "C" {
    pub fn idpf_xdp_tx_flush_bulk(bq: *mut libeth_xdp_tx_bulk, flags: u32) -> bool;
}
//
// idpf_xdp_tx_xmit - produce a single HW Tx descriptor out of XDP desc
// @desc: XDP descriptor to pull the DMA address and length from
// @i: descriptor index on the queue to fill
// @sq: XDP queue to produce the HW Tx descriptor on
// @priv: &xsk_tx_metadata_ops on XSk xmit or %NULL
//

// (u64 *)&tx_desc->qw1 = ((u64)desc.len << 48) | cmd;

// (u64 *)&xdpsq->flex_tx[ntu - 1].q.qw1 |= cmd;

//
// idpf_xdp_tx_finalize - finalize sending over XDPSQ
// @_xdpsq: XDP Tx queue
// @sent: whether any frames were sent
// @flush: whether to update RS bit and the tail register
//
// Set the RS bit ("end of batch"), bump the tail, and queue the cleanup timer.
// To be called after a NAPI polling loop, at the end of .ndo_xdp_xmit() etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_xdp_rx_desc {
    pub qw0: aligned_u64,

    pub qw1: aligned_u64,

    pub qw2: aligned_u64,

    pub qw3: aligned_u64,

    pub sizeof(u64)): *mut *mut } __aligned(4,
    pub virtchnl2_rx_flex_desc_adv_nic_3)): sizeof(struct,

    pub typeof(desc))rxd)->qw0: desc->qw0 = ((const,

    pub 16): ((u64)le16_to_cpu(rxd->ptype_err_fflags0) <<,

    pub typeof(desc))rxd)->qw1: desc->qw1 = ((const,

    pub typeof(desc))rxd)->qw2: desc->qw2 = ((const,

    pub typeof(desc))rxd)->qw3: desc->qw3 = ((const,

    pub vport): *const void idpf_xdp_set_features(struct idpf_vport,
    pub xdp): *mut *mut int idpf_xdp(struct net_device dev, struct netdev_bpf,
    pub flags): u32,
