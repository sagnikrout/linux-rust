//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_txrx.h
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
// Marvell RVU Ethernet driver
//
// Copyright (C) 2020 Marvell.
//

pub const LBK_CHAN_BASE: c_uint = 0x000;
pub const SDP_CHAN_BASE: c_uint = 0x700;
pub const CGX_CHAN_BASE: c_uint = 0x800;

pub const OTX2_MIN_MTU: c_int = 60;
pub const OTX2_PAGE_POOL_SZ: c_int = 2048;
pub const OTX2_MAX_GSO_SEGS: c_int = 255;
pub const OTX2_MAX_FRAGS_IN_SQE: c_int = 9;

// Rx buffer size should be in multiples of 128bytes

// Prefer 2048 byte buffers for better last level cache
// utilization or data distribution across regions.
//

// IRQ triggered when NIX_LF_CINTX_CNT[ECOUNT]
// is equal to this value.
//
pub const CQ_CQE_THRESH_DEFAULT: c_int = 10;
// IRQ triggered when NIX_LF_CINTX_CNT[ECOUNT]
// is nonzero and this much time elapses after that.
//

// Min number of CQs (of the ones mapped to this CINT)
// with valid CQEs.
//
pub const CQ_QCOUNT_DEFAULT: c_int = 1;
pub const CQ_OP_STAT_OP_ERR: c_int = 63;
pub const CQ_OP_STAT_CQ_ERR: c_int = 46;
// Packet mark mask
pub const OTX2_RX_MATCH_ID_MASK: c_uint = 0x0000ffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_stats {
    pub bytes: u64,
    pub pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_rcv_queue {
    pub stats: queue_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg_list {
    pub num_segs: u16,
    pub flags: u16,
    pub skb: u64,
    pub size: [u64; OTX2_MAX_FRAGS_IN_SQE],
    pub dma_addr: [u64; OTX2_MAX_FRAGS_IN_SQE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_snd_queue {
    pub aura_id: u8,
    pub head: u16,
    pub cons_head: u16,
    pub sqe_size: u16,
    pub sqe_cnt: u32,
    pub num_sqbs: u16,
    pub sqe_thresh: u16,
    pub sqe_per_sqb: u8,
    pub io_addr: u64,
    pub aura_fc_addr: *mut u64,
    pub lmt_addr: *mut u64,
    pub sqe_base: *mut c_void,
    pub sqe: *mut qmem,
    pub tso_hdrs: *mut qmem,
    pub sg: *mut sg_list,
    pub timestamps: *mut qmem,
    pub stats: queue_stats,
    pub sqb_count: u16,
    pub sqb_ptrs: *mut u64,
// SQE ring and CPT response queue for Inline IPSEC
    pub sqe_ring: *mut qmem,
    pub cpt_resp: *mut qmem,
// Buffer pool for af_xdp zero-copy
    pub xsk_pool: *mut xsk_buff_pool,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cq_type {
    CQ_RX,
    CQ_TX,
    CQ_XDP,
    CQ_QOS,
    CQS_PER_CINT = 4, /* RQ + SQ + XDP + QOS_SQ */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cq_poll {
    pub dev: *mut c_void,
pub const CINT_INVALID_CQ: c_int = 255;
    pub cint_idx: u8,
    pub cq_ids: [u8; CQS_PER_CINT],
    pub dim: dim,
    pub napi: napi_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_pool {
    pub stack: *mut qmem,
    pub fc_addr: *mut qmem,
    pub page_pool: *mut page_pool,
    pub xsk_pool: *mut xsk_buff_pool,
    pub xdp: *mut xdp_buff,
    pub xdp_cnt: u16,
    pub rbsize: u16,
    pub xdp_top: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cq_queue {
    pub cq_idx: u8,
    pub cq_type: u8,
    pub /: *mut *mut u8 cint_idx; / CQ interrupt id,
    pub refill_task_sched: u8,
    pub cqe_size: u16,
    pub pool_ptrs: u16,
    pub cqe_cnt: u32,
    pub cq_head: u32,
    pub cq_tail: u32,
    pub pend_cqe: u32,
    pub cqe_base: *mut c_void,
    pub cqe: *mut qmem,
    pub rbpool: *mut otx2_pool,
    pub xsk_zc_en: bool,
    pub xdp_rxq: xdp_rxq_info,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_qset {
    pub rqe_cnt: u32,
    pub /: *mut *mut u32 sqe_cnt; / Keep these two at top,
pub const OTX2_MAX_CQ_CNT: c_int = 64;
    pub cq_cnt: u16,
    pub xqe_size: u16,
    pub pool: *mut otx2_pool,
    pub napi: *mut otx2_cq_poll,
    pub cq: *mut otx2_cq_queue,
    pub sq: *mut otx2_snd_queue,
    pub rq: *mut otx2_rcv_queue,
}

// Translate IOVA to physical address
// Translation is installed only when IOMMU is present
extern "C" {
    pub fn iommu_iova_to_phys(_arg: iommu_domain, _arg: dma_addr) -> return;
}
extern "C" {
    pub fn otx2_napi_handler(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_refill_pool_ptrs(dev: *mut c_void, cq: *mut otx2_cq_queue) -> c_int;
}
extern "C" {
    pub fn cn10k_refill_pool_ptrs(dev: *mut c_void, cq: *mut otx2_cq_queue) -> c_int;
}
