//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mana/mana.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2021, Microsoft Corporation.

// Microsoft Azure Network Adapter (MANA)'s definitions
//
// Structures labeled with "HW DATA" are exchanged with the hardware. All of
// them are naturally aligned and hence don't need __packed.
//
// MANA protocol version
pub const MANA_MAJOR_VERSION: c_int = 0;
pub const MANA_MINOR_VERSION: c_int = 1;
pub const MANA_MICRO_VERSION: c_int = 1;
pub type mana_handle_t = u64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TRI_STATE {
    TRI_STATE_UNKNOWN = -1,
    TRI_STATE_FALSE = 0,
    TRI_STATE_TRUE = 1
}

// MANA ethtool private flag bit positions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_priv_flag_bits {
    MANA_PRIV_FLAG_USE_FULL_PAGE_RXBUF = 0,
    MANA_PRIV_FLAG_MAX,
}

// Number of entries for hardware indirection table must be in power of 2
pub const MANA_INDIRECT_TABLE_MAX_SIZE: c_int = 512;
pub const MANA_INDIRECT_TABLE_DEF_SIZE: c_int = 64;
// The Toeplitz hash key's length in bytes: should be multiple of 8
pub const MANA_HASH_KEY_SIZE: c_int = 40;
pub const COMP_ENTRY_SIZE: c_int = 64;
// This Max value for RX buffers is derived from __alloc_page()'s max page
// allocation calculation. It allows maximum 2^(MAX_ORDER -1) pages. RX buffer
// size beyond this value gets rejected by __alloc_page() call.
//
pub const MAX_RX_BUFFERS_PER_QUEUE: c_int = 8192;
pub const DEF_RX_BUFFERS_PER_QUEUE: c_int = 1024;
pub const MIN_RX_BUFFERS_PER_QUEUE: c_int = 128;
// This max value for TX buffers is derived as the maximum allocatable
// pages supported on host per guest through testing. TX buffer size beyond
// this value is rejected by the hardware.
//
pub const MAX_TX_BUFFERS_PER_QUEUE: c_int = 16384;
pub const DEF_TX_BUFFERS_PER_QUEUE: c_int = 256;
pub const MIN_TX_BUFFERS_PER_QUEUE: c_int = 128;

pub const LOG2_EQ_THROTTLE: c_int = 3;
pub const MAX_PORTS_IN_MANA_DEV: c_int = 256;
// Maximum number of PPIs per coalesced CQE
pub const MANA_RXCOMP_OOB_NUM_PPI: c_int = 4;
// 8-pkt mode packs up to 2 packets per PPI entry
pub const MANA_CQE_COAL_PKTS_8: c_int = 8;
// Default/max interrupt moderation settings
pub const MANA_INTR_MODR_USEC_DEF: c_int = 0;
pub const MANA_INTR_MODR_COMP_DEF: c_int = 0;

// DIM doorbell value field layout

// Update this count whenever the respective structures are changed

pub const MANA_STATS_TX_COUNT: c_int = 11;
pub const MANA_RX_FRAG_ALIGNMENT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_stats_rx {
    pub packets: u64,
    pub bytes: u64,
    pub xdp_drop: u64,
    pub xdp_tx: u64,
    pub xdp_redirect: u64,
    pub pkt_len0_err: u64,
    pub 1]: u64 coalesced_cqe[MANA_CQE_COAL_PKTS_8 -,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_stats_tx {
    pub packets: u64,
    pub bytes: u64,
    pub xdp_xmit: u64,
    pub tso_packets: u64,
    pub tso_bytes: u64,
    pub tso_inner_packets: u64,
    pub tso_inner_bytes: u64,
    pub short_pkt_fmt: u64,
    pub long_pkt_fmt: u64,
    pub csum_partial: u64,
    pub mana_map_err: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_txq {
    pub gdma_sq: *mut gdma_queue,
    pub gdma_txq_id: u32,
    pub 10: u32 reserved1 :,
    pub 14: u32 vsq_frame :,
    pub 8: u32 reserved2 :,
}

// The SKBs are sent to the HW and we are waiting for the CQEs.
// skb data and frags dma mappings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_skb_head {
// GSO pkts may have 2 SGEs for the linear part
    pub 2]: dma_addr_t dma_handle[MAX_SKB_FRAGS +,
    pub 2]: u32 size[MAX_SKB_FRAGS +,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_tx_pkt_format {
    MANA_SHORT_PKT_FMT	= 0,
    MANA_LONG_PKT_FMT	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_short_oob {
    pub 2: u32 pkt_fmt :,
    pub 1: u32 is_outer_ipv4 :,
    pub 1: u32 is_outer_ipv6 :,
    pub 1: u32 comp_iphdr_csum :,
    pub 1: u32 comp_tcp_csum :,
    pub 1: u32 comp_udp_csum :,
    pub 1: u32 supress_txcqe_gen :,
    pub 24: u32 vcq_num :,
    pub /: *mut *mut u32 trans_off : 10; / Transport header offset,
    pub 14: u32 vsq_frame :,
    pub 8: u32 short_vp_offset :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_long_oob {
    pub 1: u32 is_encap :,
    pub 1: u32 inner_is_ipv6 :,
    pub 1: u32 inner_tcp_opt :,
    pub 1: u32 inject_vlan_pri_tag :,
    pub 12: u32 reserved1 :,
    pub /: *mut *mut u32 pcp : 3; / 802.1Q,
    pub /: *mut *mut u32 dei : 1; / 802.1Q,
    pub /: *mut *mut u32 vlan_id : 12; / 802.1Q,
    pub 10: u32 inner_frame_offset :,
    pub 6: u32 inner_ip_rel_offset :,
    pub 12: u32 long_vp_offset :,
    pub 4: u32 reserved2 :,
    pub reserved3: u32,
    pub reserved4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_oob {
    pub s_oob: mana_tx_short_oob,
    pub l_oob: mana_tx_long_oob,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_cq_type {
    MANA_CQ_TYPE_RX,
    MANA_CQ_TYPE_TX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_cqe_type {
    CQE_INVALID			= 0,
    CQE_RX_OKAY			= 1,
    CQE_RX_COALESCED_4		= 2,
    CQE_RX_OBJECT_FENCE		= 3,
    CQE_RX_TRUNCATED		= 4,
    CQE_RX_COALESCED_8		= 7,

    CQE_TX_OKAY			= 32,
    CQE_TX_SA_DROP			= 33,
    CQE_TX_MTU_DROP			= 34,
    CQE_TX_INVALID_OOB		= 35,
    CQE_TX_INVALID_ETH_TYPE		= 36,
    CQE_TX_HDR_PROCESSING_ERROR	= 37,
    CQE_TX_VF_DISABLED		= 38,
    CQE_TX_VPORT_IDX_OUT_OF_RANGE	= 39,
    CQE_TX_VPORT_DISABLED		= 40,
    CQE_TX_VLAN_TAGGING_VIOLATION	= 41,
}

pub const MANA_CQE_COMPLETION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_cqe_header {
    pub 6: u32 cqe_type :,
    pub 2: u32 client_type :,
    pub 24: u32 vendor_err :,
}

// NDIS HASH Types

// Read PPI in two different layouts based on cqe_type
#[repr(C)]
#[derive(Copy, Clone)]
pub union mana_rxcomp_perpkt_info {
    pub 16: u32 pkt_len :,
    pub 16: u32 reserved1 :,
    pub reserved2: u32,
    pub pkt_hash: u32,
}

// Up to two pkts per PPI entry
// Receive completion OOB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rxcomp_oob {
    pub cqe_hdr: mana_cqe_header,
    pub 12: u32 rx_vlan_id :,
    pub 1: u32 rx_vlantag_present :,
    pub 1: u32 rx_outer_iphdr_csum_succeed :,
    pub 1: u32 rx_outer_iphdr_csum_fail :,
    pub 1: u32 reserved1 :,
    pub 9: u32 rx_hashtype :,
    pub 1: u32 rx_iphdr_csum_succeed :,
    pub 1: u32 rx_iphdr_csum_fail :,
    pub 1: u32 rx_tcp_csum_succeed :,
    pub 1: u32 rx_tcp_csum_fail :,
    pub 1: u32 rx_udp_csum_succeed :,
    pub 1: u32 rx_udp_csum_fail :,
    pub 1: u32 reserved2 :,
    pub ppi: [mana_rxcomp_perpkt_info; MANA_RXCOMP_OOB_NUM_PPI],
    pub rx_wqe_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_comp_oob {
    pub cqe_hdr: mana_cqe_header,
    pub tx_data_offset: u32,
    pub 5: u32 tx_sgl_offset :,
    pub 27: u32 tx_wqe_offset :,
    pub reserved: [u32; 12],
}

pub const CQE_POLLING_BUFFER: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_cq {
    pub gdma_cq: *mut gdma_queue,
// Cache the CQ id (used to verify if each CQE comes to the right CQ.
    pub gdma_id: u32,
// Type of the CQ: TX or RX
    pub type: mana_cq_type,
// Pointer to the mana_rxq that is pushing RX CQEs to the queue.
// Only and must be non-NULL if type is MANA_CQ_TYPE_RX.
//
    pub rxq: *mut mana_rxq,
// Pointer to the mana_txq that is pushing TX CQEs to the queue.
// Only and must be non-NULL if type is MANA_CQ_TYPE_TX.
//
    pub txq: *mut mana_txq,
// Buffer which the CQ handler can copy the CQE's into.
    pub gdma_comp_buf: [gdma_comp; CQE_POLLING_BUFFER],
// NAPI data
    pub napi: napi_struct,
    pub work_done: c_int,
    pub work_done_since_doorbell: c_int,
    pub budget: c_int,
// DIM - Dynamic Interrupt Moderation
    pub dim: dim,
    pub dim_event_ctr: u16,
// Cumulative TX completions fed to DIM. Updated and read only in
// NAPI context (mana_poll_tx_cq() / mana_update_tx_dim()), so they
// measure the hardware completion rate and need no u64_stats_sync.
//
    pub tx_dim_pkts: u64,
    pub tx_dim_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_recv_buf_oob {
// A valid GDMA work request representing the data buffer.
    pub wqe_req: gdma_wqe_request,
    pub buf_va: *mut c_void,
    pub /: *mut *mut bool from_pool; / allocated from a page pool,
// head page of the page_pool fragment; valid only when
// from_pool && frag_count > 1.
//
    pub pp_page: *mut page,
// Fragment offset plus rxq->headroom, passed to
// page_pool_dma_sync_for_cpu().
//
    pub dma_sync_offset: u32,
// SGL of the buffer going to be sent as part of the work request.
    pub num_sge: u32,
    pub sgl: [gdma_sge; MAX_RX_WQE_SGL_ENTRIES],
// Required to store the result of mana_gd_post_work_request.
// gdma_posted_wqe_info.wqe_size_in_bu is required for progressing the
// work queue when the WQE is consumed.
//
    pub wqe_inf: gdma_posted_wqe_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_rxq {
    pub gdma_rq: *mut gdma_queue,
// Cache the gdma receive queue id
    pub gdma_id: u32,
// Index of RQ in the vPort, not gdma receive queue id
    pub rxq_idx: u32,
    pub datasize: u32,
    pub alloc_size: u32,
    pub headroom: u32,
    pub frag_count: u32,
    pub rxobj: mana_handle_t,
    pub rx_cq: mana_cq,
    pub fence_event: completion,
    pub ndev: *mut net_device,
// Total number of receive buffers to be allocated
    pub num_rx_buf: u32,
    pub buf_index: u32,
    pub stats: mana_stats_rx,
    pub bpf_prog: *mut bpf_prog __rcu,
    pub xdp_rxq: xdp_rxq_info,
    pub /: *mut *mut *mut void xdp_save_va; / for reusing,
    pub xdp_flush: bool,
    pub /: *mut *mut int xdp_rc; / XDP redirect return code,
    pub page_pool: *mut page_pool,
    pub mana_rx_debugfs: *mut dentry,
// MUST BE THE LAST MEMBER:
// Each receive buffer has an associated mana_recv_buf_oob.
//
    pub __counted_by(num_rx_buf): mana_recv_buf_oob rx_oobs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_qp {
    pub txq: mana_txq,
    pub tx_cq: mana_cq,
    pub tx_object: mana_handle_t,
    pub mana_tx_debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ethtool_stats {
    pub stop_queue: u64,
    pub wake_queue: u64,
    pub tx_cqe_err: u64,
    pub tx_cqe_unknown_type: u64,
    pub tx_linear_pkt_cnt: u64,
    pub rx_cqe_unknown_type: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ethtool_hc_stats {
    pub hc_rx_discards_no_wqe: u64,
    pub hc_rx_err_vport_disabled: u64,
    pub hc_rx_bytes: u64,
    pub hc_rx_ucast_pkts: u64,
    pub hc_rx_ucast_bytes: u64,
    pub hc_rx_bcast_pkts: u64,
    pub hc_rx_bcast_bytes: u64,
    pub hc_rx_mcast_pkts: u64,
    pub hc_rx_mcast_bytes: u64,
    pub hc_tx_err_gf_disabled: u64,
    pub hc_tx_err_vport_disabled: u64,
    pub hc_tx_err_inval_vportoffset_pkt: u64,
    pub hc_tx_err_vlan_enforcement: u64,
    pub hc_tx_err_eth_type_enforcement: u64,
    pub hc_tx_err_sa_enforcement: u64,
    pub hc_tx_err_sqpdid_enforcement: u64,
    pub hc_tx_err_cqpdid_enforcement: u64,
    pub hc_tx_err_mtu_violation: u64,
    pub hc_tx_err_inval_oob: u64,
    pub hc_tx_bytes: u64,
    pub hc_tx_ucast_pkts: u64,
    pub hc_tx_ucast_bytes: u64,
    pub hc_tx_bcast_pkts: u64,
    pub hc_tx_bcast_bytes: u64,
    pub hc_tx_mcast_pkts: u64,
    pub hc_tx_mcast_bytes: u64,
    pub hc_tx_err_gdma: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_ethtool_phy_stats {
// Drop Counters
    pub rx_pkt_drop_phy: u64,
    pub tx_pkt_drop_phy: u64,
// Per TC traffic Counters
    pub rx_pkt_tc0_phy: u64,
    pub tx_pkt_tc0_phy: u64,
    pub rx_pkt_tc1_phy: u64,
    pub tx_pkt_tc1_phy: u64,
    pub rx_pkt_tc2_phy: u64,
    pub tx_pkt_tc2_phy: u64,
    pub rx_pkt_tc3_phy: u64,
    pub tx_pkt_tc3_phy: u64,
    pub rx_pkt_tc4_phy: u64,
    pub tx_pkt_tc4_phy: u64,
    pub rx_pkt_tc5_phy: u64,
    pub tx_pkt_tc5_phy: u64,
    pub rx_pkt_tc6_phy: u64,
    pub tx_pkt_tc6_phy: u64,
    pub rx_pkt_tc7_phy: u64,
    pub tx_pkt_tc7_phy: u64,
    pub rx_byte_tc0_phy: u64,
    pub tx_byte_tc0_phy: u64,
    pub rx_byte_tc1_phy: u64,
    pub tx_byte_tc1_phy: u64,
    pub rx_byte_tc2_phy: u64,
    pub tx_byte_tc2_phy: u64,
    pub rx_byte_tc3_phy: u64,
    pub tx_byte_tc3_phy: u64,
    pub rx_byte_tc4_phy: u64,
    pub tx_byte_tc4_phy: u64,
    pub rx_byte_tc5_phy: u64,
    pub tx_byte_tc5_phy: u64,
    pub rx_byte_tc6_phy: u64,
    pub tx_byte_tc6_phy: u64,
    pub rx_byte_tc7_phy: u64,
    pub tx_byte_tc7_phy: u64,
// Per TC pause Counters
    pub rx_pause_tc0_phy: u64,
    pub tx_pause_tc0_phy: u64,
    pub rx_pause_tc1_phy: u64,
    pub tx_pause_tc1_phy: u64,
    pub rx_pause_tc2_phy: u64,
    pub tx_pause_tc2_phy: u64,
    pub rx_pause_tc3_phy: u64,
    pub tx_pause_tc3_phy: u64,
    pub rx_pause_tc4_phy: u64,
    pub tx_pause_tc4_phy: u64,
    pub rx_pause_tc5_phy: u64,
    pub tx_pause_tc5_phy: u64,
    pub rx_pause_tc6_phy: u64,
    pub tx_pause_tc6_phy: u64,
    pub rx_pause_tc7_phy: u64,
    pub tx_pause_tc7_phy: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_context {
    pub gdma_dev: *mut gdma_dev,
    pub num_ports: u16,
    pub bm_hostmode: u8,
    pub hc_stats: mana_ethtool_hc_stats,
    pub per_port_queue_reset_wq: *mut workqueue_struct,
// Workqueue for querying hardware stats
    pub gf_stats_work: delayed_work,
    pub hwc_timeout_occurred: bool,
    pub ports: [*mut net_device; MAX_PORTS_IN_MANA_DEV],
// Link state change work
    pub link_change_work: work_struct,
    pub link_event: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_port_context {
    pub ac: *mut mana_context,
    pub ndev: *mut net_device,
    pub queue_reset_work: work_struct,
// Debug knob to log TX timeout but skip recovery reset
    pub tx_timeout_skip_reset: bool,
    pub mac_addr: [u8; ETH_ALEN],
    pub eqs: *mut mana_eq,
    pub mana_eqs_debugfs: *mut dentry,
    pub rss_state: TRI_STATE,
    pub default_rxobj: mana_handle_t,
    pub tx_shortform_allowed: bool,
    pub tx_vp_offset: u16,
    pub tx_qp: *mut mana_tx_qp,
// Indirection Table for RX & TX. The values are queue indexes
    pub indir_table: *mut u32,
    pub indir_table_sz: u32,
// Indirection table containing RxObject Handles
    pub rxobj_table: *mut mana_handle_t,
// Hash key used by the NIC
    pub hashkey: [u8; MANA_HASH_KEY_SIZE],
// This points to an array of num_queues of RQ pointers.
    pub rxqs: *mut mana_rxq,
// pre-allocated rx buffer array
    pub rxbufs_pre: *mut c_void,
    pub das_pre: *mut dma_addr_t,
    pub rxbpre_total: c_int,
    pub rxbpre_datasize: u32,
    pub rxbpre_alloc_size: u32,
    pub rxbpre_headroom: u32,
    pub rxbpre_frag_count: u32,
    pub priv_flags: u32,
    pub bpf_prog: *mut bpf_prog,
// Create num_queues EQs, SQs, SQ-CQs, RQs and RQ-CQs, respectively.
    pub max_queues: c_uint,
    pub num_queues: c_uint,
    pub rx_queue_size: c_uint,
    pub tx_queue_size: c_uint,
    pub port_handle: mana_handle_t,
    pub pf_filter_handle: mana_handle_t,
// Mutex for sharing access to vport_use_count
    pub vport_mutex: mutex,
    pub vport_use_count: c_int,
// Set by mana_set_channels() under vport_mutex to block RDMA
// from grabbing the vport during the detach/attach window.
// Checked by mana_cfg_vport() when called from the RDMA path.
//
    pub channel_changing: bool,
// Net shaper handle
    pub handle: net_shaper_handle,
    pub port_idx: u16,
// Currently configured speed (mbps)
    pub speed: u32,
// Maximum speed supported by the SKU (mbps)
    pub max_speed: u32,
// 1 = not queried, 0 = cached success, negative = permanent error.
// Protected by the netdev instance lock.
//
    pub link_cfg_error: c_int,
    pub port_is_up: bool,
    pub /: *mut *mut bool port_st_save; / Saved port state,
    pub cqe_coalescing_enable: u8,
    pub cqe8_coalescing_enable: u8,
    pub cqe_coalescing_timeout_ns: u32,
// Interrupt moderation settings
    pub intr_modr_rx_usec: u16,
    pub intr_modr_rx_comp: u16,
    pub intr_modr_tx_usec: u16,
    pub intr_modr_tx_comp: u16,
    pub rx_dim_enabled: bool,
    pub tx_dim_enabled: bool,
    pub eth_stats: mana_ethtool_stats,
    pub phy_stats: mana_ethtool_phy_stats,
// Debugfs
    pub mana_port_debugfs: *mut dentry,
// Cached vport/steering config for debugfs
    pub vport_max_sq: u32,
    pub vport_max_rq: u32,
    pub steer_rx: u32,
    pub steer_rss: u32,
    pub steer_update_tab: bool,
    pub steer_cqe_coalescing: u32,
}

extern "C" {
    pub fn mana_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn mana_disable_vport_rx(apc: *mut mana_port_context) -> c_int;
}
extern "C" {
    pub fn mana_alloc_queues(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mana_attach(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mana_detach(ndev: *mut net_device, from_close: bool) -> c_int;
}
extern "C" {
    pub fn mana_dim_change(cq: *mut mana_cq, enable: bool);
}
extern "C" {
    pub fn mana_probe(gd: *mut gdma_dev, resuming: bool) -> c_int;
}
extern "C" {
    pub fn mana_remove(gd: *mut gdma_dev, suspending: bool);
}
extern "C" {
    pub fn mana_rdma_probe(gd: *mut gdma_dev) -> c_int;
}
extern "C" {
    pub fn mana_rdma_remove(gd: *mut gdma_dev);
}
extern "C" {
    pub fn mana_xdp_tx(skb: *mut sk_buff, ndev: *mut net_device);
}
extern "C" {
    pub fn mana_chn_setxdp(apc: *mut mana_port_context, prog: *mut bpf_prog);
}
extern "C" {
    pub fn mana_bpf(ndev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn mana_query_gf_stats(ac: *mut mana_context) -> c_int;
}
extern "C" {
    pub fn mana_query_link_cfg(apc: *mut mana_port_context) -> c_int;
}
extern "C" {
    pub fn mana_query_phy_stats(apc: *mut mana_port_context);
}
extern "C" {
    pub fn mana_pre_alloc_rxbufs(apc: *mut mana_port_context, mtu: c_int, num_queues: c_int) -> c_int;
}
extern "C" {
    pub fn mana_pre_dealloc_rxbufs(apc: *mut mana_port_context);
}
extern "C" {
    pub fn mana_unmap_skb(skb: *mut sk_buff, apc: *mut mana_port_context);
}
// A CQ can be created not associated with any EQ
pub const GDMA_CQ_NO_EQ: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_obj_spec {
    pub queue_index: u32,
    pub gdma_region: u64,
    pub queue_size: u32,
    pub attached_eq: u32,
    pub modr_ctx_id: u32,
    pub req_cq_moderation: u8,
    pub cq_moderation_comp: u16,
    pub cq_moderation_usec: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mana_command_code {
    MANA_QUERY_DEV_CONFIG	= 0x20001,
    MANA_QUERY_GF_STAT	= 0x20002,
    MANA_CONFIG_VPORT_TX	= 0x20003,
    MANA_CREATE_WQ_OBJ	= 0x20004,
    MANA_DESTROY_WQ_OBJ	= 0x20005,
    MANA_FENCE_RQ		= 0x20006,
    MANA_CONFIG_VPORT_RX	= 0x20007,
    MANA_QUERY_VPORT_CONFIG	= 0x20008,
    MANA_QUERY_LINK_CONFIG	= 0x2000A,
    MANA_SET_BW_CLAMP	= 0x2000B,
    MANA_QUERY_PHY_STAT     = 0x2000c,

// Privileged commands for the PF mode
    MANA_REGISTER_FILTER	= 0x28000,
    MANA_DEREGISTER_FILTER	= 0x28001,
    MANA_REGISTER_HW_PORT	= 0x28003,
    MANA_DEREGISTER_HW_PORT	= 0x28004,
}

// Query Link Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_link_config_req {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_link_config_resp {
    pub hdr: gdma_resp_hdr,
    pub qos_speed_mbps: u32,
    pub qos_unconfigured: u8,
    pub reserved1: [u8; 3],
    pub link_speed_mbps: u32,
    pub reserved2: [u8; 4],
}

// Set Bandwidth Clamp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_set_bw_clamp_req {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
    pub enable_clamping: TRI_STATE,
    pub link_speed_mbps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_set_bw_clamp_resp {
    pub hdr: gdma_resp_hdr,
    pub qos_unconfigured: u8,
    pub reserved: [u8; 7],
}

// Query Device Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_device_cfg_req {
    pub hdr: gdma_req_hdr,
// MANA Nic Driver Capability flags
    pub mn_drv_cap_flags1: u64,
    pub mn_drv_cap_flags2: u64,
    pub mn_drv_cap_flags3: u64,
    pub mn_drv_cap_flags4: u64,
    pub proto_major_ver: u32,
    pub proto_minor_ver: u32,
    pub proto_micro_ver: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_device_cfg_resp {
    pub hdr: gdma_resp_hdr,
    pub pf_cap_flags1: u64,
    pub pf_cap_flags2: u64,
    pub pf_cap_flags3: u64,
    pub pf_cap_flags4: u64,
    pub max_num_vports: u16,
    pub /: *mut *mut u8 bm_hostmode; / response v3: Bare Metal Host Mode,
    pub reserved: u8,
    pub max_num_eqs: u32,
// response v2:
    pub adapter_mtu: u16,
    pub reserved2: u16,
    pub reserved3: u32,
}

// Query vPort Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_vport_cfg_req {
    pub hdr: gdma_req_hdr,
    pub vport_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_vport_cfg_resp {
    pub hdr: gdma_resp_hdr,
    pub max_num_sq: u32,
    pub max_num_rq: u32,
    pub num_indirection_ent: u32,
    pub reserved1: u32,
    pub mac_addr: [u8; 6],
    pub reserved2: [u8; 2],
    pub vport: mana_handle_t,
}

// Configure vPort
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_config_vport_req {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
    pub pdid: u32,
    pub doorbell_pageid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_config_vport_resp {
    pub hdr: gdma_resp_hdr,
    pub tx_vport_offset: u16,
    pub short_form_allowed: u8,
    pub reserved: u8,
}

// Create WQ Object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_create_wqobj_req {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
    pub wq_type: u32,
    pub reserved: u32,
    pub wq_gdma_region: u64,
    pub cq_gdma_region: u64,
    pub wq_size: u32,
    pub cq_size: u32,
    pub cq_moderation_ctx_id: u32,
    pub cq_parent_qid: u32,
// V2
    pub allow_rqwqe_chain: u8,
// V3
    pub req_cq_moderation: u8,
    pub cq_moderation_comp: u16,
    pub cq_moderation_usec: u16,
    pub reserved2: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_create_wqobj_resp {
    pub hdr: gdma_resp_hdr,
    pub wq_id: u32,
    pub cq_id: u32,
    pub wq_obj: mana_handle_t,
// V2
    pub cq_moderation_comp: u16,
    pub cq_moderation_usec: u16,
    pub cq_moderation_enabled: u8,
    pub reserved1: [u8; 3],
}

// Destroy WQ Object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_destroy_wqobj_req {
    pub hdr: gdma_req_hdr,
    pub wq_type: u32,
    pub reserved: u32,
    pub wq_obj_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_destroy_wqobj_resp {
    pub hdr: gdma_resp_hdr,
}

// Fence RQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_fence_rq_req {
    pub hdr: gdma_req_hdr,
    pub wq_obj_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_fence_rq_resp {
    pub hdr: gdma_resp_hdr,
}

// Query stats RQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_gf_stat_req {
    pub hdr: gdma_req_hdr,
    pub req_stats: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_gf_stat_resp {
    pub hdr: gdma_resp_hdr,
    pub reported_stats: u64,
// rx errors/discards
    pub rx_discards_nowqe: u64,
    pub rx_err_vport_disabled: u64,
// rx bytes/packets
    pub hc_rx_bytes: u64,
    pub hc_rx_ucast_pkts: u64,
    pub hc_rx_ucast_bytes: u64,
    pub hc_rx_bcast_pkts: u64,
    pub hc_rx_bcast_bytes: u64,
    pub hc_rx_mcast_pkts: u64,
    pub hc_rx_mcast_bytes: u64,
// tx errors
    pub tx_err_gf_disabled: u64,
    pub tx_err_vport_disabled: u64,
    pub tx_err_inval_vport_offset_pkt: u64,
    pub tx_err_vlan_enforcement: u64,
    pub tx_err_ethtype_enforcement: u64,
    pub tx_err_SA_enforcement: u64,
    pub tx_err_SQPDID_enforcement: u64,
    pub tx_err_CQPDID_enforcement: u64,
    pub tx_err_mtu_violation: u64,
    pub tx_err_inval_oob: u64,
// tx bytes/packets
    pub hc_tx_bytes: u64,
    pub hc_tx_ucast_pkts: u64,
    pub hc_tx_ucast_bytes: u64,
    pub hc_tx_bcast_pkts: u64,
    pub hc_tx_bcast_bytes: u64,
    pub hc_tx_mcast_pkts: u64,
    pub hc_tx_mcast_bytes: u64,
// tx error
    pub tx_err_gdma: u64,
}

// Query phy stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_phy_stat_req {
    pub hdr: gdma_req_hdr,
    pub req_stats: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_query_phy_stat_resp {
    pub hdr: gdma_resp_hdr,
    pub reported_stats: u64,
// Aggregate Drop Counters
    pub rx_pkt_drop_phy: u64,
    pub tx_pkt_drop_phy: u64,
// Per TC(Traffic class) traffic Counters
    pub rx_pkt_tc0_phy: u64,
    pub tx_pkt_tc0_phy: u64,
    pub rx_pkt_tc1_phy: u64,
    pub tx_pkt_tc1_phy: u64,
    pub rx_pkt_tc2_phy: u64,
    pub tx_pkt_tc2_phy: u64,
    pub rx_pkt_tc3_phy: u64,
    pub tx_pkt_tc3_phy: u64,
    pub rx_pkt_tc4_phy: u64,
    pub tx_pkt_tc4_phy: u64,
    pub rx_pkt_tc5_phy: u64,
    pub tx_pkt_tc5_phy: u64,
    pub rx_pkt_tc6_phy: u64,
    pub tx_pkt_tc6_phy: u64,
    pub rx_pkt_tc7_phy: u64,
    pub tx_pkt_tc7_phy: u64,
    pub rx_byte_tc0_phy: u64,
    pub tx_byte_tc0_phy: u64,
    pub rx_byte_tc1_phy: u64,
    pub tx_byte_tc1_phy: u64,
    pub rx_byte_tc2_phy: u64,
    pub tx_byte_tc2_phy: u64,
    pub rx_byte_tc3_phy: u64,
    pub tx_byte_tc3_phy: u64,
    pub rx_byte_tc4_phy: u64,
    pub tx_byte_tc4_phy: u64,
    pub rx_byte_tc5_phy: u64,
    pub tx_byte_tc5_phy: u64,
    pub rx_byte_tc6_phy: u64,
    pub tx_byte_tc6_phy: u64,
    pub rx_byte_tc7_phy: u64,
    pub tx_byte_tc7_phy: u64,
// Per TC(Traffic Class) pause Counters
    pub rx_pause_tc0_phy: u64,
    pub tx_pause_tc0_phy: u64,
    pub rx_pause_tc1_phy: u64,
    pub tx_pause_tc1_phy: u64,
    pub rx_pause_tc2_phy: u64,
    pub tx_pause_tc2_phy: u64,
    pub rx_pause_tc3_phy: u64,
    pub tx_pause_tc3_phy: u64,
    pub rx_pause_tc4_phy: u64,
    pub tx_pause_tc4_phy: u64,
    pub rx_pause_tc5_phy: u64,
    pub tx_pause_tc5_phy: u64,
    pub rx_pause_tc6_phy: u64,
    pub tx_pause_tc6_phy: u64,
    pub rx_pause_tc7_phy: u64,
    pub tx_pause_tc7_phy: u64,
}

// Configure vPort Rx Steering
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_cfg_rx_steer_req_v2 {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
    pub num_indir_entries: u16,
    pub indir_tab_offset: u16,
    pub rx_enable: u32,
    pub rss_enable: u32,
    pub update_default_rxobj: u8,
    pub update_hashkey: u8,
    pub update_indir_tab: u8,
    pub reserved: u8,
    pub default_rxobj: mana_handle_t,
    pub hashkey: [u8; MANA_HASH_KEY_SIZE],
    pub cqe_coalescing_enable: u8,
    pub reserved2: [u8; 3],
    pub rss_hash_types: u16,
    pub /: *mut *mut u8 cqe8_coalescing_enable; / v5 message,
    pub reserved3: u8,
    pub __counted_by(num_indir_entries): mana_handle_t indir_tab[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_cfg_rx_steer_resp {
    pub hdr: gdma_resp_hdr,
// V2
    pub cqe_coalescing_timeout_ns: u32,
    pub reserved1: u32,
}

// Register HW vPort
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_register_hw_vport_req {
    pub hdr: gdma_req_hdr,
    pub attached_gfid: u16,
    pub is_pf_default_vport: u8,
    pub reserved1: u8,
    pub allow_all_ether_types: u8,
    pub reserved2: u8,
    pub reserved3: u8,
    pub reserved4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_register_hw_vport_resp {
    pub hdr: gdma_resp_hdr,
    pub hw_vport_handle: mana_handle_t,
}

// Deregister HW vPort
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_deregister_hw_vport_req {
    pub hdr: gdma_req_hdr,
    pub hw_vport_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_deregister_hw_vport_resp {
    pub hdr: gdma_resp_hdr,
}

// Register filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_register_filter_req {
    pub hdr: gdma_req_hdr,
    pub vport: mana_handle_t,
    pub mac_addr: [u8; 6],
    pub reserved1: u8,
    pub reserved2: u8,
    pub reserved3: u8,
    pub reserved4: u8,
    pub reserved5: u16,
    pub reserved6: u32,
    pub reserved7: u32,
    pub reserved8: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_register_filter_resp {
    pub hdr: gdma_resp_hdr,
    pub filter_handle: mana_handle_t,
}

// Deregister filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_deregister_filter_req {
    pub hdr: gdma_req_hdr,
    pub filter_handle: mana_handle_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_deregister_filter_resp {
    pub hdr: gdma_resp_hdr,
}

// Requested GF stats Flags
// Rx discards/Errors
pub const STATISTICS_FLAGS_RX_DISCARDS_NO_WQE: c_uint = 0x0000000000000001;
pub const STATISTICS_FLAGS_RX_ERRORS_VPORT_DISABLED: c_uint = 0x0000000000000002;
// Rx bytes/pkts
pub const STATISTICS_FLAGS_HC_RX_BYTES: c_uint = 0x0000000000000004;
pub const STATISTICS_FLAGS_HC_RX_UCAST_PACKETS: c_uint = 0x0000000000000008;
pub const STATISTICS_FLAGS_HC_RX_UCAST_BYTES: c_uint = 0x0000000000000010;
pub const STATISTICS_FLAGS_HC_RX_MCAST_PACKETS: c_uint = 0x0000000000000020;
pub const STATISTICS_FLAGS_HC_RX_MCAST_BYTES: c_uint = 0x0000000000000040;
pub const STATISTICS_FLAGS_HC_RX_BCAST_PACKETS: c_uint = 0x0000000000000080;
pub const STATISTICS_FLAGS_HC_RX_BCAST_BYTES: c_uint = 0x0000000000000100;
// Tx errors
pub const STATISTICS_FLAGS_TX_ERRORS_GF_DISABLED: c_uint = 0x0000000000000200;
pub const STATISTICS_FLAGS_TX_ERRORS_VPORT_DISABLED: c_uint = 0x0000000000000400;

pub const STATISTICS_FLAGS_TX_ERRORS_VLAN_ENFORCEMENT: c_uint = 0x0000000000001000;

pub const STATISTICS_FLAGS_TX_ERRORS_SA_ENFORCEMENT: c_uint = 0x0000000000004000;
pub const STATISTICS_FLAGS_TX_ERRORS_SQPDID_ENFORCEMENT: c_uint = 0x0000000000008000;
pub const STATISTICS_FLAGS_TX_ERRORS_CQPDID_ENFORCEMENT: c_uint = 0x0000000000010000;
pub const STATISTICS_FLAGS_TX_ERRORS_MTU_VIOLATION: c_uint = 0x0000000000020000;
pub const STATISTICS_FLAGS_TX_ERRORS_INVALID_OOB: c_uint = 0x0000000000040000;
// Tx bytes/pkts
pub const STATISTICS_FLAGS_HC_TX_BYTES: c_uint = 0x0000000000080000;
pub const STATISTICS_FLAGS_HC_TX_UCAST_PACKETS: c_uint = 0x0000000000100000;
pub const STATISTICS_FLAGS_HC_TX_UCAST_BYTES: c_uint = 0x0000000000200000;
pub const STATISTICS_FLAGS_HC_TX_MCAST_PACKETS: c_uint = 0x0000000000400000;
pub const STATISTICS_FLAGS_HC_TX_MCAST_BYTES: c_uint = 0x0000000000800000;
pub const STATISTICS_FLAGS_HC_TX_BCAST_PACKETS: c_uint = 0x0000000001000000;
pub const STATISTICS_FLAGS_HC_TX_BCAST_BYTES: c_uint = 0x0000000002000000;
// Tx error
pub const STATISTICS_FLAGS_TX_ERRORS_GDMA_ERROR: c_uint = 0x0000000004000000;
pub const MANA_MAX_NUM_QUEUES: c_int = 64;
pub const MANA_DEF_NUM_QUEUES: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_tx_package {
    pub wqe_req: gdma_wqe_request,
    pub sgl_array: [gdma_sge; 5],
    pub sgl_ptr: *mut gdma_sge,
    pub tx_oob: mana_tx_oob,
    pub wqe_info: gdma_posted_wqe_info,
}

extern "C" {
    pub fn mana_uncfg_vport(apc: *mut mana_port_context);
}
extern "C" {
    pub fn mana_create_eq(apc: *mut mana_port_context) -> c_int;
}
extern "C" {
    pub fn mana_destroy_eq(apc: *mut mana_port_context);
}
