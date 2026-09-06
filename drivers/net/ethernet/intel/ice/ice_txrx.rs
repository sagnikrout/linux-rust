//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_txrx.h
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
// Copyright (c) 2018, Intel Corporation.

pub const ICE_DFLT_IRQ_WORK: c_int = 256;
pub const ICE_RXBUF_3072: c_int = 3072;
pub const ICE_RXBUF_2048: c_int = 2048;
pub const ICE_RXBUF_1664: c_int = 1664;
pub const ICE_RXBUF_1536: c_int = 1536;
pub const ICE_MAX_CHAINED_RX_BUFS: c_int = 5;
pub const ICE_MAX_BUF_TXD: c_int = 8;
pub const ICE_MIN_TX_LEN: c_int = 17;
pub const ICE_MAX_FRAME_LEGACY_RX: c_int = 8320;
// The size limit for a transmit buffer in a descriptor is (16K - 1).
// In order to align with the read requests we will align the value to
// the nearest 4K which represents our maximum read request size.
//
pub const ICE_MAX_READ_REQ_SIZE: c_int = 4096;

pub const ICE_MAX_TXQ_PER_TXQG: c_int = 128;
// We are assuming that the cache line is always 64 Bytes here for ice.
// In order to make sure that is a correct assumption there is a check in probe
// to print a warning if the read from GLPCI_CNF2 tells us that the cache line
// size is 128 bytes. We do it this way because we do not want to read the
// GLPCI_CNF2 register or a variable containing the value on every pass through
// the Tx path.
//
pub const ICE_CACHE_LINE_BYTES: c_int = 64;

pub const ICE_DESCS_FOR_CTX_DESC: c_int = 1;
pub const ICE_DESCS_FOR_SKB_DATA_PTR: c_int = 1;
// Tx descriptors needed, worst case

// Free, was ICE_TX_FLAGS_DUMMY_PKT

pub const ICE_XDP_PASS: c_int = 0;

//
// enum ice_tx_buf_type - type of &ice_tx_buf to act on Tx completion
// @ICE_TX_BUF_EMPTY: unused OR XSk frame, no action required
// @ICE_TX_BUF_DUMMY: dummy Flow Director packet, unmap and kfree()
// @ICE_TX_BUF_FRAG: mapped skb OR &xdp_buff frag, only unmap DMA
// @ICE_TX_BUF_SKB: &sk_buff, unmap and consume_skb(), update stats
// @ICE_TX_BUF_XDP_TX: &xdp_buff, unmap and page_frag_free(), stats
// @ICE_TX_BUF_XDP_XMIT: &xdp_frame, unmap and xdp_return_frame(), stats
// @ICE_TX_BUF_XSK_TX: &xdp_buff on XSk queue, xsk_buff_free(), stats
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_buf_type {
    ICE_TX_BUF_EMPTY	= 0U,
    ICE_TX_BUF_DUMMY,
    ICE_TX_BUF_FRAG,
    ICE_TX_BUF_SKB,
    ICE_TX_BUF_XDP_TX,
    ICE_TX_BUF_XDP_XMIT,
    ICE_TX_BUF_XSK_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_buf {
    pub next_to_watch: *mut ice_tx_desc,
    pub rs_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_offload_params {
    pub cd_qw1: u64,
    pub tx_ring: *mut ice_tx_ring,
    pub td_cmd: u32,
    pub td_offset: u32,
    pub td_l2tag1: u32,
    pub cd_tunnel_params: u32,
    pub cd_l2tag2: u16,
    pub cd_gcs_params: u16,
    pub header_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ring_stats {
    pub /: *mut *mut rcu_head rcu; / to avoid race on free,
    pub syncp: u64_stats_sync,
    pub pkts: u64_stats_t,
    pub bytes: u64_stats_t,
    pub tx_restart_q: u64_stats_t,
    pub tx_busy: u64_stats_t,
    pub tx_linearize: u64_stats_t,
// negative if no pending Tx descriptors
    pub prev_pkt: c_int,
    pub rx_non_eop_descs: u64_stats_t,
    pub rx_page_failed: u64_stats_t,
    pub rx_buf_failed: u64_stats_t,
}

//
// ice_stats_read - Read a single ring stat value
// @stats: pointer to ring_stats structure for a queue
// @member: the ice_ring_stats member to read
//
// Shorthand for reading a single 64-bit stat value from struct
// ice_ring_stats.
//
// Return: the value of the requested stat.
//

//
// ice_stats_inc - Increment a single ring stat value
// @stats: pointer to the ring_stats structure for a queue
// @member: the ice_ring_stats member to increment
//
// Shorthand for incrementing a single 64-bit stat value in struct
// ice_ring_stats.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_ring_state_t {
    ICE_TX_XPS_INIT_DONE,
    ICE_TX_NBITS,
}

// this enum matches hardware bits and is meant to be used by DYN_CTLN
// registers and QINT registers or more generally anywhere in the manual
// mentioning ITR_INDX, ITR_NONE cannot be used as an index 'n' into any
// register but instead is a special value meaning "don't update" ITR0/1/2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_dyn_idx_t {
    ICE_IDX_ITR0 = 0,
    ICE_IDX_ITR1 = 1,
    ICE_IDX_ITR2 = 2,
    ICE_ITR_NONE = 3	/* ITR_NONE must not be used as an index */
}

// Header split modes defined by DTYPE field of Rx RLAN context
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_rx_dtype {
    ICE_RX_DTYPE_NO_SPLIT		= 0,
    ICE_RX_DTYPE_HEADER_SPLIT	= 1,
    ICE_RX_DTYPE_SPLIT_ALWAYS	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_tx_ring_flags {
    ICE_TX_RING_FLAGS_XDP,
    ICE_TX_RING_FLAGS_VLAN_L2TAG1,
    ICE_TX_RING_FLAGS_VLAN_L2TAG2,
    ICE_TX_RING_FLAGS_TXTIME,
    ICE_TX_RING_FLAGS_NBITS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pkt_ctx {
    pub cached_phctime: u64,
    pub vlan_proto: __be16,
}

// indices into GLINT_ITR registers

pub const ICE_ITR_8K: c_int = 124;
pub const ICE_ITR_20K: c_int = 50;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_dynamic_itr {
    ITR_STATIC = 0,
    ITR_DYNAMIC = 1
}

pub const ICE_ITR_MASK: c_uint = 0x1FFE	/* ITR register value alignment mask */;

pub const ICE_DFLT_INTRL: c_int = 0;
pub const ICE_MAX_INTRL: c_int = 236;
pub const ICE_IN_WB_ON_ITR_MODE: c_int = 255;
// Sets WB_ON_ITR and assumes INTENA bit is already cleared, which allows
// setting the MSK_M bit to tell hardware to ignore the INTENA_M bit. Also,
// set the write-back latency to the usecs passed in.
//

// Legacy or Advanced Mode Queue
pub const ICE_TX_ADVANCED: c_int = 0;
pub const ICE_TX_LEGACY: c_int = 1;
// descriptor ring, associated with a VSI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tstamp_ring {
    pub /: *mut *mut *mut ice_tx_ring tx_ring; / Backreference to associated Tx ring,
    pub /: *mut *mut dma_addr_t dma; / physical address of ring,
    pub /: *mut *mut rcu_head rcu; / to avoid race on free,
    pub tail: *mut u8 __iomem,
    pub desc: *mut c_void,
    pub next_to_use: u16,
    pub count: u16,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rx_ring {
    pub /: *mut *mut *mut void desc; / Descriptor ring memory,
    pub pp: *mut page_pool,
    pub /: *mut *mut *mut net_device netdev; / netdev ring maps to,
    pub /: *mut *mut *mut ice_q_vector q_vector; / Backreference to associated vector,
    pub tail: *mut u8 __iomem,
    pub rx_fqes: *mut libeth_fqe,
    pub xdp_buf: *mut xdp_buff,
}

// stats structs
// used in interrupt processing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tx_ring {
    pub /: *mut *mut *mut void desc; / Descriptor ring memory,
    pub /: *mut *mut *mut device dev; / Used for DMA mapping,
    pub tail: *mut u8 __iomem,
    pub tx_buf: *mut ice_tx_buf,
    pub /: *mut *mut *mut ice_q_vector q_vector; / Backreference to associated vector,
    pub /: *mut *mut *mut net_device netdev; / netdev ring maps to,
    pub /: *mut *mut *mut ice_vsi vsi; / Backreference to associated VSI,
    pub /: *mut *mut u16 count; / Number of descriptors,
    pub /: *mut *mut u16 q_index; / Queue number of ring,
    pub ICE_TX_RING_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub xsk_pool: *mut xsk_buff_pool,
// stats structs
    pub ring_stats: *mut ice_ring_stats,
    pub /: *mut *mut *mut ice_tx_ring next; / pointer to next ring in q_vector,
    pub tstamp_ring: *mut ice_tstamp_ring,
    pub tx_tstamps: *mut ice_ptp_tx,
    pub next_to_use: u16,
    pub next_to_clean: u16,
    pub xdp_tx_active: u16,
    pub tx_lock: spinlock_t,
    pub /: *mut *mut rcu_head rcu; / to avoid race on free,
    pub /: *mut *mut DECLARE_BITMAP(xps_state, ICE_TX_NBITS); / XPS Config State,
    pub ch: *mut ice_channel,
    pub /: *mut *mut dma_addr_t dma; / physical address of ring,
    pub /: *mut *mut u16 q_handle; / Queue handle per TC,
    pub /: *mut *mut u16 reg_idx; / HW register index of the ring,
    pub /: *mut *mut u8 dcb_tc; / Traffic class of ring,
    pub quanta_prof_id: u16,
    pub /: *mut *mut u32 txq_teid; / Added Tx queue TEID,
    pub ____cacheline_internodealigned_in_smp: },
    pub !!ring->ch: return,
    pub ring->flags): return test_bit(ICE_TX_RING_FLAGS_XDP,,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_container_type {
    ICE_RX_CONTAINER,
    ICE_TX_CONTAINER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ring_container {
// head of linked-list of rings
    pub rx_ring: *mut ice_rx_ring,
    pub tx_ring: *mut ice_tx_ring,
}

// this matches the maximum number of ITR bits, but in usec
// values, so it is shifted left one bit (bit zero is ignored)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_coalesce_stored {
    pub itr_tx: u16,
    pub itr_rx: u16,
    pub intrl: u8,
    pub tx_valid: u8,
    pub rx_valid: u8,
}

// iterator for handling rings in ring container

extern "C" {
    pub fn ice_init_ctrl_rx_descs(rx_ring: *mut ice_rx_ring, num_descs: u32);
}
extern "C" {
    pub fn ice_rxq_pp_destroy(rq: *mut ice_rx_ring);
}
extern "C" {
    pub fn ice_alloc_rx_bufs(rxr: *mut ice_rx_ring, cleaned_count: c_uint) -> bool;
}
extern "C" {
    pub fn ice_start_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn ice_clean_tx_ring(tx_ring: *mut ice_tx_ring);
}
extern "C" {
    pub fn ice_clean_rx_ring(rx_ring: *mut ice_rx_ring);
}
extern "C" {
    pub fn ice_setup_tx_ring(tx_ring: *mut ice_tx_ring) -> c_int;
}
extern "C" {
    pub fn ice_setup_rx_ring(rx_ring: *mut ice_rx_ring) -> c_int;
}
extern "C" {
    pub fn ice_alloc_setup_tstamp_ring(tx_ring: *mut ice_tx_ring) -> c_int;
}
extern "C" {
    pub fn ice_free_tx_ring(tx_ring: *mut ice_tx_ring);
}
extern "C" {
    pub fn ice_free_rx_ring(rx_ring: *mut ice_rx_ring);
}
extern "C" {
    pub fn ice_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn ice_clean_ctrl_tx_irq(tx_ring: *mut ice_tx_ring);
}
extern "C" {
    pub fn ice_clean_ctrl_rx_irq(rx_ring: *mut ice_rx_ring);
}
extern "C" {
    pub fn ice_free_tx_tstamp_ring(tx_ring: *mut ice_tx_ring);
}
extern "C" {
    pub fn ice_free_tstamp_ring(tx_ring: *mut ice_tx_ring);
}
