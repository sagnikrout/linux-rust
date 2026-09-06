//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa/dpaa_eth.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2016 Freescale Semiconductor Inc.
//

// Number of prioritised traffic classes
pub const DPAA_TC_NUM: c_int = 4;
// More detailed FQ types - used for fine-grained WQ assignments
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpaa_fq_type {
    FQ_TYPE_RX_DEFAULT = 1, /* Rx Default FQs */
    FQ_TYPE_RX_ERROR,	/* Rx Error FQs */
    FQ_TYPE_RX_PCD,		/* Rx Parse Classify Distribute FQs */
    FQ_TYPE_TX,		/* "Real" Tx FQs */
    FQ_TYPE_TX_CONFIRM,	/* Tx default Conf FQ (actually an Rx FQ) */
    FQ_TYPE_TX_CONF_MQ,	/* Tx conf FQs (one for each Tx FQ) */
    FQ_TYPE_TX_ERROR,	/* Tx Error FQs (these are actually Rx FQs) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_fq {
    pub fq_base: qman_fq,
    pub list: list_head,
    pub net_dev: *mut net_device,
    pub init: bool,
    pub fqid: u32,
    pub flags: u32,
    pub channel: u16,
    pub wq: u8,
    pub fq_type: dpaa_fq_type,
    pub xdp_rxq: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_fq_cbs {
    pub rx_defq: qman_fq,
    pub tx_defq: qman_fq,
    pub rx_errq: qman_fq,
    pub tx_errq: qman_fq,
    pub egress_ern: qman_fq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_bp {
// used in the DMA mapping operations
    pub priv: *mut dpaa_priv,
// current number of buffers in the buffer pool alloted to each CPU
    pub percpu_count: *mut int __percpu,
// all buffers allocated for this pool have this raw size
    pub raw_size: usize,
// all buffers in this pool have this same usable size
    pub size: usize,
// the buffer pools are initialized with config_count buffers for each
// CPU; at runtime the number of buffers per CPU is constantly brought
// back to this level
//
    pub config_count: u16,
    pub bpid: u8,
    pub pool: *mut bman_pool,
// bpool can be seeded before use by this cb
    pub ): *mut *mut int (seed_cb)(struct dpaa_bp,
// bpool can be emptied before freeing by this cb
    pub ): *const *const *const void (free_buf_cb)(struct dpaa_bp , struct bm_buffer,
    pub refs: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_rx_errors {
    pub /: *mut *mut u64 dme; / DMA Error,
    pub /: *mut *mut u64 fpe; / Frame Physical Error,
    pub /: *mut *mut u64 fse; / Frame Size Error,
    pub /: *mut *mut u64 phe; / Header Error,
}

// Counters for QMan ERN frames - one counter per rejection code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_ern_cnt {
    pub /: *mut *mut u64 cg_tdrop; / Congestion group taildrop,
    pub /: *mut *mut u64 wred; / WRED congestion,
    pub /: *mut *mut u64 err_cond; / Error condition,
    pub /: *mut *mut u64 early_window; / Order restoration, frame too early,
    pub /: *mut *mut u64 late_window; / Order restoration, frame too late,
    pub /: *mut *mut u64 fq_tdrop; / FQ taildrop,
    pub /: *mut *mut u64 fq_retired; / FQ is retired,
    pub /: *mut *mut u64 orp_zero; / ORP disabled,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_napi_portal {
    pub napi: napi_struct,
    pub p: *mut qman_portal,
    pub down: bool,
    pub xdp_act: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_percpu_priv {
    pub net_dev: *mut net_device,
    pub np: dpaa_napi_portal,
    pub in_interrupt: u64,
    pub tx_confirm: u64,
// fragmented (non-linear) skbuffs received from the stack
    pub tx_frag_skbuffs: u64,
    pub stats: rtnl_link_stats64,
    pub rx_errors: dpaa_rx_errors,
    pub ern_cnt: dpaa_ern_cnt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_buffer_layout {
    pub priv_data_size: u16,
}

// Information to be used on the Tx confirmation path. Stored just
// before the start of the transmit buffer. Maximum size allowed
// is DPAA_TX_PRIV_DATA_SIZE bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_eth_swbp {
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_priv {
    pub percpu_priv: *mut dpaa_percpu_priv __percpu,
    pub dpaa_bp: *mut dpaa_bp,
// Store here the needed Tx headroom for convenience and speed
// (even though it can be computed based on the fields of buf_layout)
//
    pub tx_headroom: u16,
    pub net_dev: *mut net_device,
    pub mac_dev: *mut mac_device,
    pub rx_dma_dev: *mut device,
    pub tx_dma_dev: *mut device,
    pub egress_fqs: *mut qman_fq,
    pub conf_fqs: *mut qman_fq,
    pub channel: u16,
    pub dpaa_fq_list: list_head,
    pub num_tc: u8,
    pub keygen_in_use: bool,
    pub /: *mut *mut u32 msg_enable; / net_device message level,
// All egress queues to a given net device belong to one
// (and the same) congestion group.
//
    pub cgr: qman_cgr,
// If congested, when it began. Used for performance stats.
    pub congestion_start_jiffies: u32,
// Number of jiffies the Tx port was congested.
    pub congested_jiffies: u32,
// Counter for the number of times the CGR
// entered congestion state
//
    pub cgr_congested_count: u32,
    pub cgr_data: },
// Use a per-port CGR for ingress traffic.
    pub use_ingress_cgr: bool,
    pub ingress_cgr: qman_cgr,
    pub buf_layout: [dpaa_buffer_layout; 2],
    pub rx_headroom: u16,
    pub /: *mut *mut bool tx_tstamp; / Tx timestamping enabled,
    pub /: *mut *mut bool rx_tstamp; / Rx timestamping enabled,
    pub xdp_prog: *mut bpf_prog,
}

// from dpaa_ethtool.c
// from dpaa_eth_sysfs.c
extern "C" {
    pub fn dpaa_eth_sysfs_remove(dev: *mut device);
}
extern "C" {
    pub fn dpaa_eth_sysfs_init(dev: *mut device);
}
extern "C" {
    pub fn num_possible_cpus() -> return;
}
// Total number of Tx queues
extern "C" {
    pub fn dpaa_num_txqs_per_tc() -> *mut return DPAA_TC_NUM;
}
