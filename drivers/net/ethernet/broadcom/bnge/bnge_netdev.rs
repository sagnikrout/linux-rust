//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_netdev.h
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
// Copyright (c) 2025 Broadcom

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_bd {
    pub tx_bd_len_flags_type: __le32,

pub const TX_BD_FLAGS_BD_CNT_SHIFT: c_int = 8;

pub const TX_BD_FLAGS_LHINT_SHIFT: c_int = 13;

pub const TX_BD_LEN_SHIFT: c_int = 16;
    pub tx_bd_opaque: u32,
    pub tx_bd_haddr: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_bd {
    pub rx_bd_len_flags_type: __le32,

pub const RX_BD_TYPE_RX_PACKET_BD: c_uint = 0x4;
pub const RX_BD_TYPE_RX_BUFFER_BD: c_uint = 0x5;
pub const RX_BD_TYPE_RX_AGG_BD: c_uint = 0x6;

pub const RX_BD_LEN_SHIFT: c_int = 16;
    pub rx_bd_opaque: u32,
    pub rx_bd_haddr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_cmp {
    pub tx_cmp_flags_type: __le32,

pub const CMP_TYPE_TX_L2_CMP: c_int = 0;
pub const CMP_TYPE_TX_L2_COAL_CMP: c_int = 2;
pub const CMP_TYPE_TX_L2_PKT_TS_CMP: c_int = 4;
pub const CMP_TYPE_RX_L2_CMP: c_int = 17;
pub const CMP_TYPE_RX_AGG_CMP: c_int = 18;
pub const CMP_TYPE_RX_L2_TPA_START_CMP: c_int = 19;
pub const CMP_TYPE_RX_L2_TPA_END_CMP: c_int = 21;
pub const CMP_TYPE_RX_TPA_AGG_CMP: c_int = 22;
pub const CMP_TYPE_RX_L2_V3_CMP: c_int = 23;
pub const CMP_TYPE_RX_L2_TPA_START_V3_CMP: c_int = 25;
pub const CMP_TYPE_STATUS_CMP: c_int = 32;
pub const CMP_TYPE_REMOTE_DRIVER_REQ: c_int = 34;
pub const CMP_TYPE_REMOTE_DRIVER_RESP: c_int = 36;
pub const CMP_TYPE_ERROR_STATUS: c_int = 48;
pub const CMPL_BASE_TYPE_STAT_EJECT: c_uint = 0x1aUL;
pub const CMPL_BASE_TYPE_HWRM_DONE: c_uint = 0x20UL;
pub const CMPL_BASE_TYPE_HWRM_FWD_REQ: c_uint = 0x22UL;
pub const CMPL_BASE_TYPE_HWRM_FWD_RESP: c_uint = 0x24UL;
pub const CMPL_BASE_TYPE_HWRM_ASYNC_EVENT: c_uint = 0x2eUL;

    pub tx_cmp_opaque: u32,
    pub tx_cmp_errors_v: __le32,

pub const TX_CMP_ERRORS_BUFFER_ERROR_NO_ERROR: c_int = 0;
pub const TX_CMP_ERRORS_BUFFER_ERROR_BAD_FORMAT: c_int = 2;
pub const TX_CMP_ERRORS_BUFFER_ERROR_INVALID_STAG: c_int = 4;
pub const TX_CMP_ERRORS_BUFFER_ERROR_STAG_BOUNDS: c_int = 5;

    pub sq_cons_idx: __le32,
pub const TX_CMP_SQ_CONS_IDX_MASK: c_uint = 0x00ffffff;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_sw_tx_bd {
    pub skb: *mut sk_buff,
    pub page: *mut page,
    pub is_ts_pkt: u8,
    pub is_push: u8,
    pub action: u8,
    pub nr_frags: c_ushort,
    pub rx_prod: u16,
    pub txts_prod: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_sw_rx_bd {
    pub data: *mut c_void,
    pub data_ptr: *mut u8,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_sw_rx_agg_bd {
    pub netmem: netmem_ref,
    pub offset: c_uint,
    pub mapping: dma_addr_t,
}

pub const HWRM_RING_ALLOC_TX: c_uint = 0x1;
pub const HWRM_RING_ALLOC_RX: c_uint = 0x2;
pub const HWRM_RING_ALLOC_AGG: c_uint = 0x4;
pub const HWRM_RING_ALLOC_CMPL: c_uint = 0x8;
pub const HWRM_RING_ALLOC_NQ: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_ring_grp_info {
    pub fw_stats_ctx: u16,
    pub fw_grp_id: u16,
    pub rx_fw_ring_id: u16,
    pub agg_fw_ring_id: u16,
    pub nq_fw_ring_id: u16,
}

pub const BNGE_DEFAULT_RX_COPYBREAK: c_int = 256;
pub const BNGE_MAX_RX_COPYBREAK: c_int = 1024;

pub const MAX_TPA: c_int = 256;

pub const MAX_TPA_SEGS: c_uint = 0x3f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_tpa_idx_map {
    pub agg_id_tbl: [u16; 1024],
    pub MAX_TPA): DECLARE_BITMAP(agg_idx_bmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_tpa_info {
    pub data: *mut c_void,
    pub data_ptr: *mut u8,
    pub mapping: dma_addr_t,
    pub len: u16,
    pub gso_type: c_ushort,
    pub flags2: u32,
    pub metadata: u32,
    pub hash_type: pkt_hash_types,
    pub rss_hash: u32,
    pub hdr_info: u32,
    pub /: *mut *mut u16 cfa_code; / cfa_code in TPA start compl,
    pub agg_count: u8,
    pub vlan_valid: bool,
    pub cfa_code_valid: bool,
    pub agg_arr: *mut rx_agg_cmp,
}

// Minimum TX BDs for a TX packet with MAX_SKB_FRAGS + 1. We need one extra
// BD because the first TX BD is always a long BD.
//

pub const BNGE_NQ_HDL_IDX_MASK: c_uint = 0x00ffffff;
pub const BNGE_NQ_HDL_TYPE_MASK: c_uint = 0xff000000;
pub const BNGE_NQ_HDL_TYPE_SHIFT: c_int = 24;
pub const BNGE_NQ_HDL_TYPE_RX: c_uint = 0x00;
pub const BNGE_NQ_HDL_TYPE_TX: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_stats_mem {
    pub sw_stats: *mut u64,
    pub hw_masks: *mut u64,
    pub hw_stats: *mut c_void,
    pub hw_stats_map: dma_addr_t,
    pub len: u32,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_net_state {
    BNGE_STATE_NAPI_DISABLED,
    BNGE_STATE_STATS_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_net_flag {
    BNGE_FLAG_PORT_STATS		= BIT(0),
    BNGE_FLAG_PORT_STATS_EXT	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnge_sp_event {
    BNGE_LINK_CHNG_SP_EVENT,
    BNGE_LINK_SPEED_CHNG_SP_EVENT,
    BNGE_LINK_CFG_CHANGE_SP_EVENT,
    BNGE_UPDATE_PHY_SP_EVENT,
    BNGE_PERIODIC_STATS_SP_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_net {
    pub bd: *mut bnge_dev,
    pub netdev: *mut net_device,
    pub priv_flags: u32,
    pub rx_ring_size: u32,
    pub rx_buf_size: u32,
    pub /: *mut *mut u32 rx_buf_use_size; / usable size,
    pub rx_agg_ring_size: u32,
    pub rx_copybreak: u32,
    pub rx_ring_mask: u32,
    pub rx_agg_ring_mask: u32,
    pub rx_nr_pages: u16,
    pub rx_agg_nr_pages: u16,
    pub tx_ring_size: u32,
    pub tx_ring_mask: u32,
    pub tx_nr_pages: u16,
// NQs and Completion rings
    pub cp_ring_size: u32,
    pub cp_ring_mask: u32,
    pub cp_bit: u32,
    pub cp_nr_pages: u16,
pub const BNGE_L2_FLTR_HASH_SIZE: c_int = 32;
    pub l2_fltr_hash_tbl: [hlist_head; BNGE_L2_FLTR_HASH_SIZE],
    pub hash_seed: u32,
    pub toeplitz_prefix: u64,
    pub bnapi: *mut bnge_napi,
    pub rx_ring: *mut bnge_rx_ring_info,
    pub tx_ring: *mut bnge_tx_ring_info,
    pub tx_ring_map: *mut u16,
    pub rx_dir: dma_data_direction,
// grp_info indexed by napi/nq index
    pub grp_info: *mut bnge_ring_grp_info,
    pub vnic_info: *mut bnge_vnic_info,
    pub nr_vnics: c_int,
    pub total_irqs: c_int,
    pub tx_wake_thresh: u32,
    pub rx_offset: u16,
    pub rx_dma_offset: u16,
    pub rss_hash_key: [u8; HW_HASH_KEY_SIZE],
    pub rss_hash_key_valid:1: u8,
    pub rss_hash_key_updated:1: u8,
    pub rsscos_nr_ctxs: c_int,
    pub stats_coal_ticks: u32,
    pub state: c_ulong,
    pub msg_enable: u32,
    pub max_tpa: u16,
    pub vxlan_port: __be16,
    pub nge_port: __be16,
    pub vxlan_gpe_port: __be16,
    pub current_interval: c_uint,
    pub timer: timer_list,
    pub bnge_pf_wq: *mut workqueue_struct,
    pub sp_task: work_struct,
    pub sp_event: c_ulong,
    pub eth_link_info: bnge_ethtool_link_info,
    pub flags: u64,
    pub port_stats: bnge_stats_mem,
    pub rx_port_stats_ext: bnge_stats_mem,
    pub tx_port_stats_ext: bnge_stats_mem,
    pub fw_rx_stats_ext_size: u16,
    pub fw_tx_stats_ext_size: u16,
    pub rxq_prv_stats: netdev_queue_stats_rx,
    pub txq_prv_stats: netdev_queue_stats_tx,
    pub prv_stats64: rtnl_link_stats64,
    pub stats_lock: spinlock_t,
    pub pri2cos_idx: [u8; 8],
    pub pri2cos_valid: bool,
}

pub const BNGE_DEFAULT_RX_RING_SIZE: c_int = 511;
pub const BNGE_DEFAULT_TX_RING_SIZE: c_int = 511;
extern "C" {
    pub fn bnge_netdev_alloc(bd: *mut bnge_dev, max_irqs: c_int) -> c_int;
}
extern "C" {
    pub fn bnge_netdev_free(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_set_ring_params(bd: *mut bnge_dev);
}

pub const MAX_RX_PAGES_AGG_ENA: c_int = 1;
pub const MAX_RX_PAGES: c_int = 4;
pub const MAX_RX_AGG_PAGES: c_int = 4;
pub const MAX_TX_PAGES: c_int = 1;
pub const MAX_CP_PAGES: c_int = 16;

pub const MAX_RX_PAGES_AGG_ENA: c_int = 8;
pub const MAX_RX_PAGES: c_int = 32;
pub const MAX_RX_AGG_PAGES: c_int = 32;
pub const MAX_TX_PAGES: c_int = 8;
pub const MAX_CP_PAGES: c_int = 128;

pub const BNGE_MAX_TXR_PER_NAPI: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nqe_cn {
    pub type: __le16,
pub const NQ_CN_TYPE_MASK: c_uint = 0x3fUL;
pub const NQ_CN_TYPE_SFT: c_int = 0;
pub const NQ_CN_TYPE_CQ_NOTIFICATION: c_uint = 0x30UL;

pub const NQ_CN_TOGGLE_MASK: c_uint = 0xc0UL;
pub const NQ_CN_TOGGLE_SFT: c_int = 6;
    pub reserved16: __le16,
    pub cq_handle_low: __le32,
    pub v: __le32,
pub const NQ_CN_V: c_uint = 0x1UL;
    pub cq_handle_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_cp_ring_info {
    pub bnapi: *mut bnge_napi,
    pub desc_mapping: *mut dma_addr_t,
    pub desc_ring: *mut tx_cmp,
    pub ring_struct: bnge_ring_struct,
    pub cp_ring_type: u8,
    pub cp_idx: u8,
    pub cp_raw_cons: u32,
    pub cp_db: bnge_db_info,
    pub had_work_done: bool,
    pub has_more_work: bool,
    pub had_nqe_notify: bool,
    pub toggle: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_nq_ring_info {
    pub bnapi: *mut bnge_napi,
    pub desc_mapping: *mut dma_addr_t,
    pub desc_ring: *mut nqe_cn,
    pub ring_struct: bnge_ring_struct,
    pub nq_raw_cons: u32,
    pub nq_db: bnge_db_info,
    pub stats: bnge_stats_mem,
    pub hw_stats_ctx_id: u32,
    pub has_more_work: bool,
    pub cp_ring_count: u16,
    pub cp_ring_arr: *mut bnge_cp_ring_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_rx_ring_info {
    pub bnapi: *mut bnge_napi,
    pub rx_cpr: *mut bnge_cp_ring_info,
    pub rx_prod: u16,
    pub rx_agg_prod: u16,
    pub rx_sw_agg_prod: u16,
    pub rx_next_cons: u16,
    pub rx_db: bnge_db_info,
    pub rx_agg_db: bnge_db_info,
    pub rx_desc_ring: [*mut rx_bd; MAX_RX_PAGES],
    pub rx_buf_ring: *mut bnge_sw_rx_bd,
    pub rx_agg_desc_ring: [*mut rx_bd; MAX_RX_AGG_PAGES],
    pub rx_agg_buf_ring: *mut bnge_sw_rx_agg_bd,
    pub rx_agg_bmap: *mut c_ulong,
    pub rx_agg_bmap_size: u16,
    pub rx_desc_mapping: [dma_addr_t; MAX_RX_PAGES],
    pub rx_agg_desc_mapping: [dma_addr_t; MAX_RX_AGG_PAGES],
    pub rx_tpa: *mut bnge_tpa_info,
    pub rx_tpa_idx_map: *mut bnge_tpa_idx_map,
    pub rx_ring_struct: bnge_ring_struct,
    pub rx_agg_ring_struct: bnge_ring_struct,
    pub page_pool: *mut page_pool,
    pub head_pool: *mut page_pool,
    pub need_head_pool: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_tx_ring_info {
    pub bnapi: *mut bnge_napi,
    pub tx_cpr: *mut bnge_cp_ring_info,
    pub tx_prod: u16,
    pub tx_cons: u16,
    pub tx_hw_cons: u16,
    pub txq_index: u16,
    pub tx_napi_idx: u8,
    pub kick_pending: u8,
    pub tx_db: bnge_db_info,
    pub tx_desc_ring: [*mut tx_bd; MAX_TX_PAGES],
    pub tx_buf_ring: *mut bnge_sw_tx_bd,
    pub tx_desc_mapping: [dma_addr_t; MAX_TX_PAGES],
    pub dev_state: u32,
pub const BNGE_DEV_STATE_CLOSING: c_uint = 0x1;
    pub tx_ring_struct: bnge_ring_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_napi {
    pub napi: napi_struct,
    pub bn: *mut bnge_net,
    pub index: c_int,
    pub nq_ring: bnge_nq_ring_info,
    pub rx_ring: *mut bnge_rx_ring_info,
    pub tx_ring: [*mut bnge_tx_ring_info; BNGE_MAX_TXR_PER_NAPI],
    pub events: u8,
pub const BNGE_RX_EVENT: c_int = 1;
pub const BNGE_AGG_EVENT: c_int = 2;
pub const BNGE_TX_EVENT: c_int = 4;
pub const BNGE_REDIRECT_EVENT: c_int = 8;
pub const BNGE_TX_CMP_EVENT: c_uint = 0x10;
    pub in_reset: bool,
    pub tx_fault: bool,
}

pub const BNGE_VNIC_DEFAULT: c_int = 0;
pub const BNGE_MAX_UC_ADDRS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_vnic_info {
    pub fw_vnic_id: u16,
pub const BNGE_MAX_CTX_PER_VNIC: c_int = 8;
    pub fw_rss_cos_lb_ctx: [u16; BNGE_MAX_CTX_PER_VNIC],
    pub mru: u16,
// index 0 always dev_addr
    pub l2_filters: [*mut bnge_l2_filter; BNGE_MAX_UC_ADDRS],
    pub uc_filter_count: u16,
    pub uc_list: *mut u8,
    pub rss_table_dma_addr: dma_addr_t,
    pub rss_table: *mut __le16,
    pub rss_hash_key_dma_addr: dma_addr_t,
    pub rss_hash_key: *mut u64,
    pub rss_table_size: c_int,
pub const BNGE_RSS_TABLE_ENTRIES: c_int = 64;

pub const BNGE_RSS_TABLE_MAX_TBL: c_int = 8;

    pub rx_mask: u32,
    pub mc_list: *mut u8,
    pub mc_list_size: c_int,
    pub mc_list_count: c_int,
    pub mc_list_mapping: dma_addr_t,
pub const BNGE_MAX_MC_ADDRS: c_int = 16;
    pub flags: u32,
pub const BNGE_VNIC_RSS_FLAG: c_int = 1;
pub const BNGE_VNIC_MCAST_FLAG: c_int = 4;
pub const BNGE_VNIC_UCAST_FLAG: c_int = 8;
    pub vnic_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_filter_base {
    pub hash: hlist_node,
    pub list: list_head,
    pub filter_id: __le64,
    pub type: u8,
pub const BNGE_FLTR_TYPE_L2: c_int = 2;
    pub flags: u8,
    pub rxq: u16,
    pub fw_vnic_id: u16,
    pub vf_idx: u16,
    pub state: c_ulong,
pub const BNGE_FLTR_VALID: c_int = 0;
pub const BNGE_FLTR_FW_DELETED: c_int = 2;
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_l2_key {
    pub dst_mac_addr: [u8; ETH_ALEN],
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_l2_filter {
// base filter must be the first member
    pub base: bnge_filter_base,
    pub l2_key: bnge_l2_key,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn bnge_cp_ring_for_rx(rxr: *mut bnge_rx_ring_info) -> u32;
}
extern "C" {
    pub fn bnge_cp_ring_for_tx(txr: *mut bnge_tx_ring_info) -> u32;
}
extern "C" {
    pub fn bnge_fill_hw_rss_tbl(bn: *mut bnge_net, vnic: *mut bnge_vnic_info);
}
extern "C" {
    pub fn bnge_find_next_agg_idx(rxr: *mut bnge_rx_ring_info, idx: u16) -> u16;
}
extern "C" {
    pub fn __bnge_queue_sp_work(bn: *mut bnge_net);
}
extern "C" {
    pub fn bnge_copy_hw_masks(mask_arr: *mut u64, hw_mask_arr: *mut __le64, count: c_int);
}
