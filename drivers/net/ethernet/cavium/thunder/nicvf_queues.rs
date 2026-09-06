//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/thunder/nicvf_queues.h
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
// Copyright (C) 2015 Cavium, Inc.
//

pub const MAX_QUEUE_SET: c_int = 128;
pub const MAX_RCV_QUEUES_PER_QS: c_int = 8;
pub const MAX_RCV_BUF_DESC_RINGS_PER_QS: c_int = 2;
pub const MAX_SND_QUEUES_PER_QS: c_int = 8;
pub const MAX_CMP_QUEUES_PER_QS: c_int = 8;
// VF's queue interrupt ranges
pub const NICVF_INTR_ID_CQ: c_int = 0;
pub const NICVF_INTR_ID_SQ: c_int = 8;
pub const NICVF_INTR_ID_RBDR: c_int = 16;
pub const NICVF_INTR_ID_MISC: c_int = 18;
pub const NICVF_INTR_ID_QS_ERR: c_int = 19;

// Default queue count per QS, its lengths and threshold values
pub const DEFAULT_RBDR_CNT: c_int = 1;

pub const MIN_SQ_DESC_PER_PKT_XMIT: c_int = 2;
// Since timestamp not enabled, otherwise 2
pub const MAX_CQE_PER_PKT_XMIT: c_int = 1;
// Keep CQ and SQ sizes same, if timestamping
// is enabled this equation will change.
//

// No of CQEs that might anyway gets used by HW due to pipelining
// effects irrespective of PASS/DROP/LEVELS being configured
//
pub const CMP_QUEUE_PIPELINE_RSVD: c_int = 544;

// RED and Backpressure levels of CQ for pkt reception
// For CQ, level is a measure of emptiness i.e 0x0 means full
// eg: For CQ of size 4K, and for pass/drop levels of 160/144
// HW accepts pkt if unused CQE >= 2560
// RED accepts pkt if unused CQE < 2304 & >= 2560
// DROPs pkts if unused CQE < 2304
//

// RED and Backpressure levels of RBDR for pkt reception
// For RBDR, level is a measure of fullness i.e 0x0 means empty
// eg: For RBDR of size 8K, and for pass/drop levels of 4/0
// HW accepts pkt if unused RBs >= 256
// RED accepts pkt if unused RBs < 256 & >= 0
// DROPs pkts if unused RBs < 0
//

// Descriptor size in bytes
pub const SND_QUEUE_DESC_SIZE: c_int = 16;
pub const CMP_QUEUE_DESC_SIZE: c_int = 512;
// Buffer / descriptor alignments
pub const NICVF_RCV_BUF_ALIGN: c_int = 7;

// Queue enable/disable

// Queue reset

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CQ_RX_ERRLVL_E {
    CQ_ERRLVL_MAC,
    CQ_ERRLVL_L2,
    CQ_ERRLVL_L3,
    CQ_ERRLVL_L4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CQ_RX_ERROP_E {
    CQ_RX_ERROP_RE_NONE = 0x0,
    CQ_RX_ERROP_RE_PARTIAL = 0x1,
    CQ_RX_ERROP_RE_JABBER = 0x2,
    CQ_RX_ERROP_RE_FCS = 0x7,
    CQ_RX_ERROP_RE_TERMINATE = 0x9,
    CQ_RX_ERROP_RE_RX_CTL = 0xb,
    CQ_RX_ERROP_PREL2_ERR = 0x1f,
    CQ_RX_ERROP_L2_FRAGMENT = 0x20,
    CQ_RX_ERROP_L2_OVERRUN = 0x21,
    CQ_RX_ERROP_L2_PFCS = 0x22,
    CQ_RX_ERROP_L2_PUNY = 0x23,
    CQ_RX_ERROP_L2_MAL = 0x24,
    CQ_RX_ERROP_L2_OVERSIZE = 0x25,
    CQ_RX_ERROP_L2_UNDERSIZE = 0x26,
    CQ_RX_ERROP_L2_LENMISM = 0x27,
    CQ_RX_ERROP_L2_PCLP = 0x28,
    CQ_RX_ERROP_IP_NOT = 0x41,
    CQ_RX_ERROP_IP_CSUM_ERR = 0x42,
    CQ_RX_ERROP_IP_MAL = 0x43,
    CQ_RX_ERROP_IP_MALD = 0x44,
    CQ_RX_ERROP_IP_HOP = 0x45,
    CQ_RX_ERROP_L3_ICRC = 0x46,
    CQ_RX_ERROP_L3_PCLP = 0x47,
    CQ_RX_ERROP_L4_MAL = 0x61,
    CQ_RX_ERROP_L4_CHK = 0x62,
    CQ_RX_ERROP_UDP_LEN = 0x63,
    CQ_RX_ERROP_L4_PORT = 0x64,
    CQ_RX_ERROP_TCP_FLAG = 0x65,
    CQ_RX_ERROP_TCP_OFFSET = 0x66,
    CQ_RX_ERROP_L4_PCLP = 0x67,
    CQ_RX_ERROP_RBDR_TRUNC = 0x70,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CQ_TX_ERROP_E {
    CQ_TX_ERROP_GOOD = 0x0,
    CQ_TX_ERROP_DESC_FAULT = 0x10,
    CQ_TX_ERROP_HDR_CONS_ERR = 0x11,
    CQ_TX_ERROP_SUBDC_ERR = 0x12,
    CQ_TX_ERROP_MAX_SIZE_VIOL = 0x13,
    CQ_TX_ERROP_IMM_SIZE_OFLOW = 0x80,
    CQ_TX_ERROP_DATA_SEQUENCE_ERR = 0x81,
    CQ_TX_ERROP_MEM_SEQUENCE_ERR = 0x82,
    CQ_TX_ERROP_LOCK_VIOL = 0x83,
    CQ_TX_ERROP_DATA_FAULT = 0x84,
    CQ_TX_ERROP_TSTMP_CONFLICT = 0x85,
    CQ_TX_ERROP_TSTMP_TIMEOUT = 0x86,
    CQ_TX_ERROP_MEM_FAULT = 0x87,
    CQ_TX_ERROP_CK_OVERLAP = 0x88,
    CQ_TX_ERROP_CK_OFLOW = 0x89,
    CQ_TX_ERROP_ENUM_LAST = 0x8a,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RQ_SQ_STATS {
    RQ_SQ_STATS_OCTS,
    RQ_SQ_STATS_PKTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tx_queue_stats {
    pub bytes: u64,
    pub pkts: u64,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q_desc_mem {
    pub dma: dma_addr_t,
    pub size: u64,
    pub q_len: u32,
    pub phys_base: dma_addr_t,
    pub base: *mut c_void,
    pub unalign_base: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgcache {
    pub page: *mut page,
    pub ref_count: c_int,
    pub dma_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbdr {
    pub enable: bool,
    pub dma_size: u32,
    pub frag_len: u32,
    pub /: *mut *mut u32 thresh; / Threshold level for interrupt,
    pub desc: *mut c_void,
    pub head: u32,
    pub tail: u32,
    pub dmem: q_desc_mem,
    pub is_xdp: bool,
// For page recycling
    pub pgidx: c_int,
    pub pgcnt: c_int,
    pub pgalloc: c_int,
    pub pgcache: *mut pgcache,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcv_queue {
    pub enable: bool,
    pub rbdr_start: *mut rbdr,
    pub rbdr_cont: *mut rbdr,
    pub en_tcp_reassembly: bool,
    pub /: *mut *mut u8 cq_qs; / CQ's QS to which this RQ is assigned,
    pub /: *mut *mut u8 cq_idx; / CQ index (0 to 7) in the QS,
    pub /: *mut *mut u8 cont_rbdr_qs; / Continue buffer ptrs - QS num,
    pub /: *mut *mut u8 cont_qs_rbdr_idx; / RBDR idx in the cont QS,
    pub /: *mut *mut u8 start_rbdr_qs; / First buffer ptrs - QS num,
    pub /: *mut *mut u8 start_qs_rbdr_idx; / RBDR idx in the above QS,
    pub caching: u8,
    pub stats: rx_tx_queue_stats,
    pub xdp_rxq: xdp_rxq_info,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmp_queue {
    pub enable: bool,
    pub thresh: u16,
    pub /: *mut *mut spinlock_t lock; / lock to serialize processing CQEs,
    pub desc: *mut c_void,
    pub dmem: q_desc_mem,
    pub irq: c_int,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_queue {
    pub enable: bool,
    pub /: *mut *mut u8 cq_qs; / CQ's QS to which this SQ is pointing,
    pub /: *mut *mut u8 cq_idx; / CQ index (0 to 7) in the above QS,
    pub thresh: u16,
    pub free_cnt: core::sync::atomic::AtomicI32,
    pub head: u32,
    pub tail: u32,
    pub skbuff: *mut u64,
    pub desc: *mut c_void,
    pub xdp_page: *mut u64,
    pub xdp_desc_cnt: u16,
    pub xdp_free_cnt: u16,
    pub is_xdp: bool,
// For TSO segment's header
    pub tso_hdrs: *mut c_char,
    pub tso_hdrs_phys: dma_addr_t,
    pub affinity_mask: cpumask_t,
    pub dmem: q_desc_mem,
    pub stats: rx_tx_queue_stats,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_set {
    pub enable: bool,
    pub be_en: bool,
    pub vnic_id: u8,
    pub rq_cnt: u8,
    pub cq_cnt: u8,
    pub cq_len: u64,
    pub sq_cnt: u8,
    pub sq_len: u64,
    pub rbdr_cnt: u8,
    pub rbdr_len: u64,
    pub rq: [rcv_queue; MAX_RCV_QUEUES_PER_QS],
    pub cq: [cmp_queue; MAX_CMP_QUEUES_PER_QS],
    pub sq: [snd_queue; MAX_SND_QUEUES_PER_QS],
    pub rbdr: [rbdr; MAX_RCV_BUF_DESC_RINGS_PER_QS],
    pub ____cacheline_aligned_in_smp: },

// CQ status bits

// Translation is installed only when IOMMU is present
    pub dma_addr): return iommu_iova_to_phys(nic->iommu_domain,,
    pub dma_addr: return,
    pub subdesc_cnt): int hdr_sqe, u8,
    pub features): netdev_features_t,
    pub nic): *mut int nicvf_set_qset_resources(struct nicvf,
    pub enable): *mut *mut int nicvf_config_data_transfer(struct nicvf nic, bool,
    pub enable): *mut *mut void nicvf_qset_config(struct nicvf nic, bool,
    pub enable): int qidx, bool,
    pub qidx): *mut *mut *mut void nicvf_sq_enable(struct nicvf nic, struct snd_queue sq, int,
    pub qidx): *mut *mut void nicvf_sq_disable(struct nicvf nic, int,
    pub desc_cnt): *mut *mut void nicvf_put_sq_desc(struct snd_queue sq, int,
    pub qidx): *mut *mut snd_queue sq, int,
    pub sq_num): *mut *mut sk_buff skb, u8,
    pub len): u64 bufaddr, u64 dma_addr, u16,
    pub sq_num): *mut *mut *mut void nicvf_xdp_sq_doorbell(struct nicvf nic, struct snd_queue sq, int,
    pub xdp): *mut *mut cqe_rx_t cqe_rx, bool,
    pub t): *mut void nicvf_rbdr_task(struct tasklet_struct,
    pub work): *mut void nicvf_rbdr_work(struct work_struct,
    pub q_idx): *mut *mut void nicvf_enable_intr(struct nicvf nic, int int_type, int,
    pub q_idx): *mut *mut void nicvf_disable_intr(struct nicvf nic, int int_type, int,
    pub q_idx): *mut *mut void nicvf_clear_intr(struct nicvf nic, int int_type, int,
    pub q_idx): *mut *mut int nicvf_is_intr_enabled(struct nicvf nic, int int_type, int,
// Register access APIs
    pub val): *mut *mut void nicvf_reg_write(struct nicvf nic, u64 offset, u64,
    pub offset): *mut *mut u64 nicvf_reg_read(struct nicvf nic, u64,
    pub val): u64 qidx, u64,
    pub qidx): u64 offset, u64,
// Stats
    pub rq_idx): *mut *mut void nicvf_update_rq_stats(struct nicvf nic, int,
    pub sq_idx): *mut *mut void nicvf_update_sq_stats(struct nicvf nic, int,
    pub cqe_rx): *mut *mut int nicvf_check_cqe_rx_errs(struct nicvf nic, struct cqe_rx_t,
    pub cqe_tx): *mut *mut int nicvf_check_cqe_tx_errs(struct nicvf nic, struct cqe_send_t,
