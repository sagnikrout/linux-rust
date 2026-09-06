//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2014-2016 Broadcom Corporation
// Copyright (c) 2016-2018 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

// DO NOT CHANGE DRV_VER_* defines
// FIXME: Delete them
//
pub const DRV_VER_MAJ: c_int = 1;
pub const DRV_VER_MIN: c_int = 10;
pub const DRV_VER_UPD: c_int = 3;

pub const BNXT_MIN_RX_HDR_BUF: c_int = 256;
pub const BNXT_MAX_RX_HDR_BUF: c_int = 1024;
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
pub const TX_OPAQUE_IDX_MASK: c_uint = 0x0000ffff;
pub const TX_OPAQUE_BDS_MASK: c_uint = 0x00ff0000;
pub const TX_OPAQUE_BDS_SHIFT: c_int = 16;
pub const TX_OPAQUE_RING_MASK: c_uint = 0xff000000;
pub const TX_OPAQUE_RING_SHIFT: c_int = 24;

pub const TX_MAX_BD_CNT: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_bd_ext {
    pub tx_bd_hsize_lflags: __le32,

pub const TX_BD_HSIZE_SHIFT: c_int = 16;
    pub tx_bd_mss: __le32,
    pub tx_bd_cfa_action: __le32,

pub const TX_BD_CFA_ACTION_SHIFT: c_int = 16;
    pub tx_bd_cfa_meta: __le32,
pub const TX_BD_CFA_META_MASK: c_uint = 0xfffffff;
pub const TX_BD_CFA_META_VID_MASK: c_uint = 0xfff;

pub const TX_BD_CFA_META_PRI_SHIFT: c_int = 12;

pub const TX_BD_CFA_META_TPID_SHIFT: c_int = 16;

pub const TX_BD_CFA_META_KEY_SHIFT: c_int = 28;

}

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
pub struct tx_ts_cmp {
    pub tx_ts_cmp_flags_type: __le32,

pub const TX_TS_CMP_TS_NS_MID_SFT: c_int = 16;
    pub tx_ts_cmp_opaque: u32,
    pub tx_ts_cmp_errors_v: __le32,

    pub tx_ts_cmp_ts_ns_lo: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_cmp {
    pub rx_cmp_len_flags_type: __le32,

pub const RX_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;
pub const RX_CMP_FLAGS_ITYPES_MASK: c_uint = 0xf000;

pub const RX_CMP_LEN_SHIFT: c_int = 16;
    pub rx_cmp_opaque: u32,
    pub rx_cmp_misc_v1: __le32,

pub const RX_CMP_AGG_BUFS_SHIFT: c_int = 1;

pub const RX_CMP_RSS_HASH_TYPE_SHIFT: c_int = 9;

pub const RX_CMP_V3_RSS_EXT_OP_LEGACY_SHIFT: c_int = 12;

pub const RX_CMP_V3_RSS_EXT_OP_NEW_SHIFT: c_int = 8;

pub const RX_CMP_PAYLOAD_OFFSET_SHIFT: c_int = 16;

pub const RX_CMP_SUB_NS_TS_SHIFT: c_int = 16;

pub const RX_CMP_METADATA1_SHIFT: c_int = 28;

    pub rx_cmp_rss_hash: __le32,
}

pub const RSS_PROFILE_ID_MASK: c_uint = 0x1f;

pub const EXT_OP_INNER_4: c_uint = 0x0;
pub const EXT_OP_OUTER_4: c_uint = 0x2;
pub const EXT_OP_INNFL_3: c_uint = 0x8;
pub const EXT_OP_OUTFL_3: c_uint = 0xa;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_cmp_ext {
    pub rx_cmp_flags2: __le32,
pub const RX_CMP_FLAGS2_IP_CS_CALC: c_uint = 0x1;

    pub rx_cmp_meta_data: __le32,
pub const RX_CMP_FLAGS2_METADATA_TCI_MASK: c_uint = 0xffff;
pub const RX_CMP_FLAGS2_METADATA_VID_MASK: c_uint = 0xfff;
pub const RX_CMP_FLAGS2_METADATA_TPID_MASK: c_uint = 0xffff0000;
pub const RX_CMP_FLAGS2_METADATA_TPID_SFT: c_int = 16;
    pub rx_cmp_cfa_code_errors_v2: __le32,

pub const RX_CMPL_ERRORS_SFT: c_int = 1;

pub const RX_CMPL_CFA_CODE_SFT: c_int = 16;

pub const RX_CMPL_METADATA0_SFT: c_int = 16;
    pub rx_cmp_timestamp: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_agg_cmp {
    pub rx_agg_cmp_len_flags_type: __le32,

pub const RX_AGG_CMP_LEN_SHIFT: c_int = 16;
    pub rx_agg_cmp_opaque: u32,
    pub rx_agg_cmp_v: __le32,

pub const RX_AGG_CMP_AGG_ID_SHIFT: c_int = 16;
    pub rx_agg_cmp_unused: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_start_cmp {
    pub rx_tpa_start_cmp_len_flags_type: __le32,

pub const RX_TPA_START_CMP_FLAGS_SHIFT: c_int = 6;

pub const RX_TPA_START_CMP_FLAGS_PLACEMENT_SHIFT: c_int = 7;

pub const RX_TPA_START_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;

pub const RX_TPA_START_CMP_LEN_SHIFT: c_int = 16;
    pub rx_tpa_start_cmp_opaque: u32,
    pub rx_tpa_start_cmp_misc_v1: __le32,

pub const RX_TPA_START_CMP_RSS_HASH_TYPE_SHIFT: c_int = 9;

pub const RX_TPA_START_CMP_V3_RSS_HASH_TYPE_SHIFT: c_int = 7;

pub const RX_TPA_START_CMP_AGG_ID_SHIFT: c_int = 25;

pub const RX_TPA_START_CMP_AGG_ID_SHIFT_P5: c_int = 16;

pub const RX_TPA_START_CMP_METADATA1_SHIFT: c_int = 28;

    pub rx_tpa_start_cmp_rss_hash: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_start_cmp_ext {
    pub rx_tpa_start_cmp_flags2: __le32,

pub const RX_TPA_START_CMP_FLAGS2_EXT_META_FORMAT_SHIFT: c_int = 10;

pub const RX_TPA_START_CMP_FLAGS2_CSUM_CMPL_SHIFT: c_int = 16;
    pub rx_tpa_start_cmp_metadata: __le32,
    pub rx_tpa_start_cmp_cfa_code_v2: __le32,

pub const RX_TPA_START_CMP_ERRORS_BUFFER_ERROR_SHIFT: c_int = 1;

pub const RX_TPA_START_CMPL_CFA_CODE_SHIFT: c_int = 16;

pub const RX_TPA_START_CMP_METADATA0_SFT: c_int = 16;
    pub rx_tpa_start_cmp_hdr_info: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_end_cmp {
    pub rx_tpa_end_cmp_len_flags_type: __le32,

pub const RX_TPA_END_CMP_FLAGS_SHIFT: c_int = 6;

pub const RX_TPA_END_CMP_FLAGS_PLACEMENT_SHIFT: c_int = 7;

pub const RX_TPA_END_CMP_FLAGS_ITYPES_SHIFT: c_int = 12;

pub const RX_TPA_END_CMP_LEN_SHIFT: c_int = 16;
    pub rx_tpa_end_cmp_opaque: u32,
    pub rx_tpa_end_cmp_misc_v1: __le32,

pub const RX_TPA_END_CMP_AGG_BUFS_SHIFT: c_int = 1;

pub const RX_TPA_END_CMP_TPA_SEGS_SHIFT: c_int = 8;

pub const RX_TPA_END_CMP_PAYLOAD_OFFSET_SHIFT: c_int = 16;

pub const RX_TPA_END_CMP_AGG_ID_SHIFT: c_int = 25;

pub const RX_TPA_END_CMP_AGG_ID_SHIFT_P5: c_int = 16;
    pub rx_tpa_end_cmp_tsdelta: __le32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_tpa_end_cmp_ext {
    pub rx_tpa_end_cmp_dup_acks: __le32,

pub const RX_TPA_END_CMP_PAYLOAD_OFFSET_SHIFT_P5: c_int = 16;

pub const RX_TPA_END_CMP_AGG_BUFS_SHIFT_P5: c_int = 24;
    pub rx_tpa_end_cmp_seg_len: __le32,

    pub rx_tpa_end_cmp_errors_v2: __le32,

pub const RX_TPA_END_CMPL_ERRORS_SHIFT: c_int = 1;

    pub rx_tpa_end_cmp_start_opaque: u32,
}

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

pub const BNXT_NQ_HDL_IDX_MASK: c_uint = 0x00ffffff;
pub const BNXT_NQ_HDL_TYPE_MASK: c_uint = 0xff000000;
pub const BNXT_NQ_HDL_TYPE_SHIFT: c_int = 24;
pub const BNXT_NQ_HDL_TYPE_RX: c_uint = 0x00;
pub const BNXT_NQ_HDL_TYPE_TX: c_uint = 0x01;

pub const DB_IDX_MASK: c_uint = 0xffffff;

pub const BNXT_MIN_ROCE_CP_RINGS: c_int = 2;
pub const BNXT_MIN_ROCE_STAT_CTXS: c_int = 1;
// 64-bit doorbell
pub const DBR_INDEX_MASK: c_uint = 0x0000000000ffffffULL;
pub const DBR_EPOCH_MASK: c_uint = 0x01000000UL;
pub const DBR_EPOCH_SFT: c_int = 24;
pub const DBR_TOGGLE_MASK: c_uint = 0x06000000UL;
pub const DBR_TOGGLE_SFT: c_int = 25;
pub const DBR_XID_MASK: c_uint = 0x000fffff00000000ULL;
pub const DBR_XID_SFT: c_int = 32;

pub const DB_PF_OFFSET_P5: c_uint = 0x10000;
pub const DB_VF_OFFSET_P5: c_uint = 0x4000;

// The hardware supports certain page sizes.  Use the supported page sizes
// to allocate the rings.
//

pub const BNXT_PAGE_SHIFT: c_int = 12;

pub const BNXT_PAGE_SHIFT: c_int = 13;

pub const BNXT_PAGE_SHIFT: c_int = 16;

// The RXBD length is 16-bit so we can only support page sizes < 64K

pub const BNXT_RX_PAGE_SHIFT: c_int = 15;

pub const BNXT_MAX_MTU: c_int = 9500;
// First RX buffer page in XDP multi-buf mode
//
// +-------------------------------------------------------------------------+
// | XDP_PACKET_HEADROOM | bp->rx_buf_use_size              | skb_shared_info|
// | (bp->rx_dma_offset) |                                  |                |
// +-------------------------------------------------------------------------+
//

pub const BNXT_MIN_PKT_SIZE: c_int = 52;
pub const BNXT_DEFAULT_RX_RING_SIZE: c_int = 511;
pub const BNXT_DEFAULT_TX_RING_SIZE: c_int = 511;
pub const MAX_TPA: c_int = 64;
pub const MAX_TPA_P5: c_int = 256;

pub const MAX_TPA_SEGS_P5: c_uint = 0x3f;

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

// Minimum TX BDs for a TX packet with MAX_SKB_FRAGS + 1.  We need one extra
// BD because the first TX BD is always a long BD.
//

pub const DFLT_HWRM_CMD_TIMEOUT: c_int = 500;
pub const BNXT_RX_EVENT: c_int = 1;
pub const BNXT_AGG_EVENT: c_int = 2;
pub const BNXT_TX_EVENT: c_int = 4;
pub const BNXT_REDIRECT_EVENT: c_int = 8;
pub const BNXT_TX_CMP_EVENT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_sw_tx_bd {
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

pub const BNXT_SW_GSO_MID: c_int = 1;
pub const BNXT_SW_GSO_LAST: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_sw_rx_bd {
    pub data: *mut c_void,
    pub data_ptr: *mut u8,
    pub mapping: dma_addr_t,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_sw_rx_agg_bd {
    pub netmem: netmem_ref,
    pub offset: c_uint,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ring_mem_info {
    pub nr_pages: c_int,
    pub page_size: c_int,
    pub flags: u16,
pub const BNXT_RMEM_VALID_PTE_FLAG: c_int = 1;
pub const BNXT_RMEM_RING_PTE_FLAG: c_int = 2;
pub const BNXT_RMEM_USE_FULL_PAGE_FLAG: c_int = 4;
    pub depth: u16,
    pub ctx_mem: *mut bnxt_ctx_mem_type,
    pub pg_arr: *mut c_void,
    pub dma_arr: *mut dma_addr_t,
    pub pg_tbl: *mut __le64,
    pub pg_tbl_map: dma_addr_t,
    pub vmem_size: c_int,
    pub vmem: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ring_struct {
    pub ring_mem: bnxt_ring_mem_info,
    pub /: *mut *mut u16 fw_ring_id; / Ring id filled by Chimp FW,
    pub grp_idx: u16,
    pub /: *mut *mut u16 map_idx; / Used by cmpl rings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_push_bd {
    pub doorbell: __le32,
    pub tx_bd_len_flags_type: __le32,
    pub tx_bd_opaque: u32,
    pub txbd2: tx_bd_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_push_buffer {
    pub push_bd: tx_push_bd,
    pub data: [u32; 25],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_db_info {
    pub doorbell: *mut void __iomem,
    pub db_key64: u64,
    pub db_key32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tx_ring_info {
    pub bnapi: *mut bnxt_napi,
    pub tx_cpr: *mut bnxt_cp_ring_info,
    pub tx_prod: u16,
    pub tx_cons: u16,
    pub tx_hw_cons: u16,
    pub txq_index: u16,
    pub tx_napi_idx: u8,
    pub kick_pending: u8,
    pub tx_db: bnxt_db_info,
    pub tx_desc_ring: [*mut tx_bd; MAX_TX_PAGES],
    pub tx_buf_ring: *mut bnxt_sw_tx_bd,
    pub tx_desc_mapping: [dma_addr_t; MAX_TX_PAGES],
    pub tx_push: *mut tx_push_buffer,
    pub tx_push_mapping: dma_addr_t,
    pub data_mapping: __le64,
    pub tx_inline_buf: *mut c_void,
    pub tx_inline_dma: dma_addr_t,
    pub tx_inline_size: c_uint,
    pub tx_inline_prod: u16,
    pub tx_inline_cons: u16,
pub const BNXT_DEV_STATE_CLOSING: c_uint = 0x1;
    pub dev_state: u32,
    pub tx_ring_struct: bnxt_ring_struct,
// Synchronize simultaneous xdp_xmit on same ring
    pub xdp_tx_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_coal_cap {
    pub cmpl_params: u32,
    pub nq_params: u32,
    pub num_cmpl_dma_aggr_max: u16,
    pub num_cmpl_dma_aggr_during_int_max: u16,
    pub cmpl_aggr_dma_tmr_max: u16,
    pub cmpl_aggr_dma_tmr_during_int_max: u16,
    pub int_lat_tmr_min_max: u16,
    pub int_lat_tmr_max_max: u16,
    pub num_cmpl_aggr_int_max: u16,
    pub timer_units: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_coal {
    pub coal_ticks: u16,
    pub coal_ticks_irq: u16,
    pub coal_bufs: u16,
    pub coal_bufs_irq: u16,
// RING_IDLE enabled when coal ticks < idle_thresh
    pub idle_thresh: u16,
    pub bufs_per_record: u8,
    pub budget: u8,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tpa_info {
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
    pub vlan_valid:1: u8,
    pub cfa_code_valid:1: u8,
    pub agg_arr: *mut rx_agg_cmp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tpa_idx_map {
    pub agg_id_tbl: [u16; 1024],
    pub MAX_TPA_P5): DECLARE_BITMAP(agg_idx_bmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_rx_ring_info {
    pub bnapi: *mut bnxt_napi,
    pub rx_cpr: *mut bnxt_cp_ring_info,
    pub rx_prod: u16,
    pub rx_agg_prod: u16,
    pub rx_sw_agg_prod: u16,
    pub rx_next_cons: u16,
    pub rx_db: bnxt_db_info,
    pub rx_agg_db: bnxt_db_info,
    pub xdp_prog: *mut bpf_prog,
    pub rx_desc_ring: [*mut rx_bd; MAX_RX_PAGES],
    pub rx_buf_ring: *mut bnxt_sw_rx_bd,
    pub rx_agg_desc_ring: [*mut rx_bd; MAX_RX_AGG_PAGES],
    pub rx_agg_ring: *mut bnxt_sw_rx_agg_bd,
    pub rx_agg_bmap: *mut c_ulong,
    pub rx_agg_bmap_size: u16,
    pub rx_page_size: u32,
    pub need_head_pool: bool,
    pub rx_desc_mapping: [dma_addr_t; MAX_RX_PAGES],
    pub rx_agg_desc_mapping: [dma_addr_t; MAX_RX_AGG_PAGES],
    pub rx_tpa: *mut bnxt_tpa_info,
    pub rx_tpa_idx_map: *mut bnxt_tpa_idx_map,
    pub rx_ring_struct: bnxt_ring_struct,
    pub rx_agg_ring_struct: bnxt_ring_struct,
    pub xdp_rxq: xdp_rxq_info,
    pub page_pool: *mut page_pool,
    pub head_pool: *mut page_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_rx_sw_stats {
    pub rx_l4_csum_errors: u64,
    pub rx_resets: u64,
    pub rx_buf_errors: u64,
// end of ethtool -S stats
    pub rx_oom_discards: u64,
    pub rx_netpoll_discards: u64,
    pub rx_hw_gro_packets: u64,
    pub rx_hw_gro_wire_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tx_sw_stats {
    pub tx_resets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_cmn_sw_stats {
    pub missed_irqs: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_sw_stats {
    pub rx: bnxt_rx_sw_stats,
    pub tx: bnxt_tx_sw_stats,
    pub cmn: bnxt_cmn_sw_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_total_ring_drv_stats {
    pub rx_total_l4_csum_errors: u64,
    pub rx_total_resets: u64,
    pub rx_total_buf_errors: u64,
    pub rx_total_oom_discards: u64,
    pub rx_total_netpoll_discards: u64,
    pub rx_total_ring_discards: u64,
    pub tx_total_resets: u64,
    pub tx_total_ring_discards: u64,
    pub total_missed_irqs: u64,
// end of ethtool -S stats
    pub rx_total_hw_gro_packets: u64,
    pub rx_total_hw_gro_wire_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_stats_mem {
    pub sw_stats: *mut u64,
    pub hw_masks: *mut u64,
    pub hw_stats: *mut c_void,
    pub hw_stats_map: dma_addr_t,
    pub len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_cp_ring_info {
    pub bnapi: *mut bnxt_napi,
    pub cp_raw_cons: u32,
    pub cp_db: bnxt_db_info,
    pub had_work_done:1: u8,
    pub has_more_work:1: u8,
    pub had_nqe_notify:1: u8,
    pub toggle: u8,
    pub cp_ring_type: u8,
    pub cp_idx: u8,
    pub last_cp_raw_cons: u32,
    pub rx_ring_coal: bnxt_coal,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub event_ctr: u64,
    pub dim: dim,
    pub cp_desc_ring: *mut tx_cmp,
    pub nq_desc_ring: *mut nqe_cn,
}

pub const BNXT_MAX_QUEUE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_napi {
    pub napi: napi_struct,
    pub bp: *mut bnxt,
    pub index: c_int,
    pub cp_ring: bnxt_cp_ring_info,
    pub rx_ring: *mut bnxt_rx_ring_info,
    pub tx_ring: [*mut bnxt_tx_ring_info; BNXT_MAX_TXR_PER_NAPI],
    pub budget): c_int,
    pub events: u8,
    pub tx_fault:1: u8,
    pub flags: u32,
pub const BNXT_NAPI_FLAG_XDP: c_uint = 0x1;
    pub in_reset: bool,
}

// "TxRx", 2 hypens, plus maximum integer
pub const BNXT_IRQ_NAME_EXTRA: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_irq {
    pub handler: irq_handler_t,
    pub vector: c_uint,
    pub requested:1: u8,
    pub BNXT_IRQ_NAME_EXTRA]: char name[IFNAMSIZ +,
    pub bp: *mut bnxt,
    pub msix_nr: c_int,
    pub ring_nr: c_int,
    pub tag: u16,
    pub new_tag: u16,
    pub affinity_notify: irq_affinity_notify,
}

pub const HWRM_RING_ALLOC_TX: c_uint = 0x1;
pub const HWRM_RING_ALLOC_RX: c_uint = 0x2;
pub const HWRM_RING_ALLOC_AGG: c_uint = 0x4;
pub const HWRM_RING_ALLOC_CMPL: c_uint = 0x8;
pub const HWRM_RING_ALLOC_NQ: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ring_grp_info {
    pub fw_stats_ctx: u16,
    pub fw_grp_id: u16,
    pub rx_fw_ring_id: u16,
    pub agg_fw_ring_id: u16,
    pub cp_fw_ring_id: u16,
}

pub const BNXT_VNIC_DEFAULT: c_int = 0;
pub const BNXT_VNIC_NTUPLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_vnic_info {
    pub /: *mut *mut u16 fw_vnic_id; / returned by Chimp during alloc,
pub const BNXT_MAX_CTX_PER_VNIC: c_int = 8;
    pub fw_rss_cos_lb_ctx: [u16; BNXT_MAX_CTX_PER_VNIC],
    pub fw_l2_ctx_id: u16,
    pub mru: u16,
pub const BNXT_MAX_UC_ADDRS: c_int = 4;
    pub l2_filters: [*mut bnxt_l2_filter; BNXT_MAX_UC_ADDRS],
// index 0 always dev_addr
    pub uc_filter_count: u16,
    pub uc_list: *mut u8,
    pub fw_grp_ids: *mut u16,
    pub rss_table_dma_addr: dma_addr_t,
    pub rss_table: *mut __le16,
    pub rss_hash_key_dma_addr: dma_addr_t,
    pub rss_hash_key: *mut u64,
    pub rss_table_size: c_int,
pub const BNXT_RSS_TABLE_ENTRIES_P5: c_int = 64;

pub const BNXT_RSS_TABLE_MAX_TBL_P5: c_int = 8;

    pub rx_mask: u32,
    pub mc_list: *mut u8,
    pub mc_list_size: c_int,
    pub mc_list_count: c_int,
    pub mc_list_mapping: dma_addr_t,
pub const BNXT_MAX_MC_ADDRS: c_int = 16;
    pub flags: u32,
pub const BNXT_VNIC_RSS_FLAG: c_int = 1;
pub const BNXT_VNIC_RFS_FLAG: c_int = 2;
pub const BNXT_VNIC_MCAST_FLAG: c_int = 4;
pub const BNXT_VNIC_UCAST_FLAG: c_int = 8;
pub const BNXT_VNIC_RFS_NEW_RSS_FLAG: c_uint = 0x10;
pub const BNXT_VNIC_NTUPLE_FLAG: c_uint = 0x20;
pub const BNXT_VNIC_RSSCTX_FLAG: c_uint = 0x40;
    pub rss_ctx: *mut ethtool_rxfh_context,
    pub vnic_id: u32,
    pub default_rx_ring: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_rss_ctx {
    pub vnic: bnxt_vnic_info,
    pub index: u8,
}

pub const BNXT_MAX_ETH_RSS_CTX: c_int = 32;
pub const BNXT_VNIC_ID_INVALID: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_hw_rings {
    pub tx: c_int,
    pub rx: c_int,
    pub grp: c_int,
    pub cp: c_int,
    pub cp_p5: c_int,
    pub stat: c_int,
    pub vnic: c_int,
    pub rss_ctx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_hw_resc {
    pub min_rsscos_ctxs: u16,
    pub max_rsscos_ctxs: u16,
    pub resv_rsscos_ctxs: u16,
    pub min_cp_rings: u16,
    pub max_cp_rings: u16,
    pub resv_cp_rings: u16,
    pub min_tx_rings: u16,
    pub max_tx_rings: u16,
    pub resv_tx_rings: u16,
    pub max_tx_sch_inputs: u16,
    pub min_rx_rings: u16,
    pub max_rx_rings: u16,
    pub resv_rx_rings: u16,
    pub min_hw_ring_grps: u16,
    pub max_hw_ring_grps: u16,
    pub resv_hw_ring_grps: u16,
    pub min_l2_ctxs: u16,
    pub max_l2_ctxs: u16,
    pub min_vnics: u16,
    pub max_vnics: u16,
    pub resv_vnics: u16,
    pub min_stat_ctxs: u16,
    pub max_stat_ctxs: u16,
    pub resv_stat_ctxs: u16,
    pub max_nqs: u16,
    pub max_irqs: u16,
    pub resv_irqs: u16,
    pub max_encap_records: u32,
    pub max_decap_records: u32,
    pub max_tx_em_flows: u32,
    pub max_tx_wm_flows: u32,
    pub max_rx_em_flows: u32,
    pub max_rx_wm_flows: u32,
}

pub const BNXT_LARGE_RSS_TO_VNIC_RATIO: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_vf_info {
    pub fw_fid: u16,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / PF assigned MAC Address,
    pub only: *mut *mut u8 vf_mac_addr[ETH_ALEN]; / VF assigned MAC address,,
// stored by PF.
//
    pub vlan: u16,
    pub func_qcfg_flags: u16,
    pub flags: u32,
pub const BNXT_VF_SPOOFCHK: c_uint = 0x2;
pub const BNXT_VF_LINK_FORCED: c_uint = 0x4;
pub const BNXT_VF_LINK_UP: c_uint = 0x8;
pub const BNXT_VF_TRUST: c_uint = 0x10;
    pub min_tx_rate: u32,
    pub max_tx_rate: u32,
    pub hwrm_cmd_req_addr: *mut c_void,
    pub hwrm_cmd_req_dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_pf_info {
pub const BNXT_FIRST_PF_FID: c_int = 1;
pub const BNXT_FIRST_VF_FID: c_int = 128;
    pub fw_fid: u16,
    pub port_id: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub first_vf_id: u32,
    pub active_vfs: u16,
    pub registered_vfs: u16,
    pub max_vfs: u16,
    pub vf_event_bmap: *mut c_ulong,
    pub hwrm_cmd_req_pages: u16,
    pub vf_resv_strategy: u8,
pub const BNXT_VF_RESV_STRATEGY_MAXIMAL: c_int = 0;
pub const BNXT_VF_RESV_STRATEGY_MINIMAL: c_int = 1;
pub const BNXT_VF_RESV_STRATEGY_MINIMAL_STATIC: c_int = 2;
    pub hwrm_cmd_req_addr: [*mut c_void; 4],
    pub hwrm_cmd_req_dma_addr: [dma_addr_t; 4],
    pub vf: *mut bnxt_vf_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_filter_base {
    pub hash: hlist_node,
    pub list: list_head,
    pub filter_id: __le64,
    pub type: u8,
pub const BNXT_FLTR_TYPE_NTUPLE: c_int = 1;
pub const BNXT_FLTR_TYPE_L2: c_int = 2;
    pub flags: u8,
pub const BNXT_ACT_DROP: c_int = 1;
pub const BNXT_ACT_RING_DST: c_int = 2;
pub const BNXT_ACT_FUNC_DST: c_int = 4;
pub const BNXT_ACT_NO_AGING: c_int = 8;
pub const BNXT_ACT_RSS_CTX: c_uint = 0x10;
    pub sw_id: u16,
    pub rxq: u16,
    pub fw_vnic_id: u16,
    pub vf_idx: u16,
    pub state: c_ulong,
pub const BNXT_FLTR_VALID: c_int = 0;
pub const BNXT_FLTR_INSERTED: c_int = 1;
pub const BNXT_FLTR_FW_DELETED: c_int = 2;
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_flow_masks {
    pub ports: flow_dissector_key_ports,
    pub addrs: flow_dissector_key_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ntuple_filter {
// base filter must be the first member
    pub base: bnxt_filter_base,
    pub fkeys: flow_keys,
    pub fmasks: bnxt_flow_masks,
    pub l2_fltr: *mut bnxt_l2_filter,
    pub flow_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_l2_key {
    pub dst_mac_addr: [u8; ETH_ALEN],
    pub vlan: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ipv4_tuple {
    pub v4addrs: flow_dissector_key_ipv4_addrs,
    pub ports: flow_dissector_key_ports,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ipv6_tuple {
    pub v6addrs: flow_dissector_key_ipv6_addrs,
    pub ports: flow_dissector_key_ports,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_l2_filter {
// base filter must be the first member
    pub base: bnxt_filter_base,
    pub l2_key: bnxt_l2_key,
    pub refcnt: core::sync::atomic::AtomicI32,
}

// Compat version of hwrm_port_phy_qcfg_output capped at 96 bytes.  The
// first 95 bytes are identical to hwrm_port_phy_qcfg_output in bnxt_hsi.h.
// The last valid byte in the compat version is different.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwrm_port_phy_qcfg_output_compat {
    pub error_code: __le16,
    pub req_type: __le16,
    pub seq_id: __le16,
    pub resp_len: __le16,
    pub link: u8,
    pub active_fec_signal_mode: u8,
    pub link_speed: __le16,
    pub duplex_cfg: u8,
    pub pause: u8,
    pub support_speeds: __le16,
    pub force_link_speed: __le16,
    pub auto_mode: u8,
    pub auto_pause: u8,
    pub auto_link_speed: __le16,
    pub auto_link_speed_mask: __le16,
    pub wirespeed: u8,
    pub lpbk: u8,
    pub force_pause: u8,
    pub module_status: u8,
    pub preemphasis: __le32,
    pub phy_maj: u8,
    pub phy_min: u8,
    pub phy_bld: u8,
    pub phy_type: u8,
    pub media_type: u8,
    pub xcvr_pkg_type: u8,
    pub eee_config_phy_addr: u8,
    pub parallel_detect: u8,
    pub link_partner_adv_speeds: __le16,
    pub link_partner_adv_auto_mode: u8,
    pub link_partner_adv_pause: u8,
    pub adv_eee_link_speed_mask: __le16,
    pub link_partner_adv_eee_link_speed_mask: __le16,
    pub xcvr_identifier_type_tx_lpi_timer: __le32,
    pub fec_cfg: __le16,
    pub duplex_state: u8,
    pub option_flags: u8,
    pub phy_vendor_name: [c_char; 16],
    pub phy_vendor_partnumber: [c_char; 16],
    pub support_pam4_speeds: __le16,
    pub force_pam4_link_speed: __le16,
    pub auto_pam4_link_speed_mask: __le16,
    pub link_partner_pam4_adv_speeds: u8,
    pub valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_link_info {
    pub phy_type: u8,
    pub media_type: u8,
    pub transceiver: u8,
    pub phy_addr: u8,
    pub phy_link_status: u8,

    pub wire_speed: u8,
    pub phy_state: u8,
pub const BNXT_PHY_STATE_ENABLED: c_int = 0;
pub const BNXT_PHY_STATE_DISABLED: c_int = 1;
    pub link_state: u8,
pub const BNXT_LINK_STATE_UNKNOWN: c_int = 0;
pub const BNXT_LINK_STATE_DOWN: c_int = 1;
pub const BNXT_LINK_STATE_UP: c_int = 2;

    pub link_down_reason: u8,
    pub active_lanes: u8,
    pub duplex: u8,

    pub pause: u8,

    pub lp_pause: u8,
    pub auto_pause_setting: u8,
    pub force_pause_setting: u8,
    pub duplex_setting: u8,
    pub auto_mode: u8,

pub const PHY_VER_LEN: c_int = 3;
    pub phy_ver: [u8; PHY_VER_LEN],
    pub link_speed: u16,

    pub support_speeds: u16,
    pub support_pam4_speeds: u16,
    pub support_speeds2: u16,
    pub /: *mut *mut u16 auto_link_speeds; / fw adv setting,

    pub auto_pam4_link_speeds: u16,

    pub auto_link_speeds2: u16,

    pub support_auto_speeds: u16,
    pub support_pam4_auto_speeds: u16,
    pub support_auto_speeds2: u16,
    pub lp_auto_link_speeds: u16,
    pub lp_auto_pam4_link_speeds: u16,
    pub force_link_speed: u16,
    pub force_pam4_link_speed: u16,
    pub force_link_speed2: u16,

    pub preemphasis: u32,
    pub module_status: u8,
    pub active_fec_sig_mode: u8,
    pub fec_cfg: u16,

// copy of requested setting from ethtool cmd
    pub autoneg: u8,
pub const BNXT_AUTONEG_SPEED: c_int = 1;
pub const BNXT_AUTONEG_FLOW_CTRL: c_int = 2;
    pub req_signal_mode: u8,

    pub req_duplex: u8,
    pub req_flow_ctrl: u8,
    pub req_link_speed: u16,
    pub /: *mut *mut u16 advertising; / user adv setting,
    pub advertising_pam4: u16,
    pub force_link_chng: bool,
    pub phy_retry: bool,
    pub phy_retry_expires: c_ulong,
// a copy of phy_qcfg output used to report link
// info to VF
//
    pub phy_qcfg_resp: hwrm_port_phy_qcfg_output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_queue_info {
    pub queue_id: u8,
    pub queue_profile: u8,
}

pub const BNXT_MAX_LED: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_led_info {
    pub led_id: u8,
    pub led_type: u8,
    pub led_group_id: u8,
    pub unused: u8,
    pub led_state_caps: __le16,

    pub led_color_caps: __le16,
}

pub const BNXT_MAX_TEST: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_test_info {
    pub offline_mask: u8,
    pub timeout: u16,
    pub string: [c_char; BNXT_MAX_TEST][ETH_GSTRING_LEN],
}

pub const BNXT_GRCPF_REG_CHIMP_COMM: c_uint = 0x0;
pub const BNXT_GRCPF_REG_CHIMP_COMM_TRIGGER: c_uint = 0x100;
pub const BNXT_GRCPF_REG_WINDOW_BASE_OUT: c_uint = 0x400;
pub const BNXT_GRC_REG_STATUS_P5: c_uint = 0x520;
pub const BNXT_GRCPF_REG_KONG_COMM: c_uint = 0xA00;
pub const BNXT_GRCPF_REG_KONG_COMM_TRIGGER: c_uint = 0xB00;
pub const BNXT_GRC_REG_CHIP_NUM: c_uint = 0x48;
pub const BNXT_GRC_REG_BASE: c_uint = 0x260000;
pub const BNXT_TS_REG_TIMESYNC_TS0_LOWER: c_uint = 0x640180c;
pub const BNXT_TS_REG_TIMESYNC_TS0_UPPER: c_uint = 0x6401810;
pub const BNXT_GRC_BASE_MASK: c_uint = 0xfffff000;
pub const BNXT_GRC_OFFSET_MASK: c_uint = 0x00000ffc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_flow_stats {
    pub packets: u64,
    pub bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_flower_indr_block_cb_priv {
    pub tunnel_netdev: *mut net_device,
    pub bp: *mut bnxt,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_info {
    pub enabled: bool,
// hash table to store TC offloaded flows
    pub flow_table: rhashtable,
    pub flow_ht_params: rhashtable_params,
// hash table to store L2 keys of TC flows
    pub l2_table: rhashtable,
    pub l2_ht_params: rhashtable_params,
// hash table to store L2 keys for TC tunnel decap
    pub decap_l2_table: rhashtable,
    pub decap_l2_ht_params: rhashtable_params,
// hash table to store tunnel decap entries
    pub decap_table: rhashtable,
    pub decap_ht_params: rhashtable_params,
// hash table to store tunnel encap entries
    pub encap_table: rhashtable,
    pub encap_ht_params: rhashtable_params,
// lock to atomically add/del an l2 node when a flow is
// added or deleted.
//
    pub lock: mutex,
// Fields used for batching stats query
    pub iter: rhashtable_iter,
pub const BNXT_FLOW_STATS_BATCH_MAX: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_tc_stats_batch {
    pub flow_node: *mut c_void,
    pub hw_stats: bnxt_tc_flow_stats,
    pub stats_batch: [}; BNXT_FLOW_STATS_BATCH_MAX],
// Stat counter mask (width)
    pub bytes_mask: u64,
    pub packets_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_vf_rep_stats {
    pub packets: u64,
    pub bytes: u64,
    pub dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_vf_rep {
    pub bp: *mut bnxt,
    pub dev: *mut net_device,
    pub dst: *mut metadata_dst,
    pub vf_idx: u16,
    pub tx_cfa_action: u16,
    pub rx_cfa_code: u16,
    pub rx_stats: bnxt_vf_rep_stats,
    pub tx_stats: bnxt_vf_rep_stats,
}

pub const PTU_PTE_VALID: c_uint = 0x1UL;
pub const PTU_PTE_LAST: c_uint = 0x2UL;
pub const PTU_PTE_NEXT_TO_LAST: c_uint = 0x4UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ctx_pg_info {
    pub entries: u32,
    pub nr_pages: u32,
    pub ctx_pg_arr: [*mut c_void; MAX_CTX_PAGES],
    pub ctx_dma_arr: [dma_addr_t; MAX_CTX_PAGES],
    pub ring_mem: bnxt_ring_mem_info,
    pub ctx_pg_tbl: *mut bnxt_ctx_pg_info,
}

pub const BNXT_MAX_TQM_SP_RINGS: c_int = 1;
pub const BNXT_MAX_TQM_FP_RINGS: c_int = 8;

pub const BNXT_BACKING_STORE_CFG_LEGACY_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ctx_mem_type {
    pub type: u16,
    pub entry_size: u16,
    pub flags: u32,

    pub instance_bmap: u32,
    pub init_value: u8,
    pub entry_multiple: u8,
    pub init_offset: u16,
pub const BNXT_CTX_INIT_INVALID_OFFSET: c_uint = 0xffff;
    pub max_entries: u32,
    pub min_entries: u32,
    pub last:1: u8,
    pub mem_valid:1: u8,
    pub split_entry_cnt: u8,
pub const BNXT_MAX_SPLIT_ENTRY: c_int = 4;
    pub qp_l2_entries: u32,
    pub qp_qp1_entries: u32,
    pub qp_fast_qpmd_entries: u32,
}

pub const BNXT_CTX_MRAV_AV_SPLIT_ENTRY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_ctx_mem_info {
    pub tqm_fp_rings_count: u8,
    pub flags: u32,
pub const BNXT_CTX_FLAG_INITED: c_uint = 0x01;
    pub ctx_arr: [bnxt_ctx_mem_type; BNXT_CTX_V2_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_health_severity {
    SEVERITY_NORMAL = 0,
    SEVERITY_WARNING,
    SEVERITY_RECOVERABLE,
    SEVERITY_FATAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnxt_health_remedy {
    REMEDY_DEVLINK_RECOVER,
    REMEDY_POWER_CYCLE_DEVICE,
    REMEDY_POWER_CYCLE_HOST,
    REMEDY_FW_UPDATE,
    REMEDY_HW_REPLACE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_fw_health {
    pub flags: u32,
    pub polling_dsecs: u32,
    pub master_func_wait_dsecs: u32,
    pub normal_func_wait_dsecs: u32,
    pub post_reset_wait_dsecs: u32,
    pub post_reset_max_wait_dsecs: u32,
    pub regs: [u32; 4],
    pub mapped_regs: [u32; 4],
pub const BNXT_FW_HEALTH_REG: c_int = 0;
pub const BNXT_FW_HEARTBEAT_REG: c_int = 1;
pub const BNXT_FW_RESET_CNT_REG: c_int = 2;
pub const BNXT_FW_RESET_INPROG_REG: c_int = 3;
    pub fw_reset_inprog_reg_mask: u32,
    pub last_fw_heartbeat: u32,
    pub last_fw_reset_cnt: u32,
    pub enabled:1: u8,
    pub primary:1: u8,
    pub status_reliable:1: u8,
    pub resets_reliable:1: u8,
    pub tmr_multiplier: u8,
    pub tmr_counter: u8,
    pub fw_reset_seq_cnt: u8,
    pub fw_reset_seq_regs: [u32; 16],
    pub fw_reset_seq_vals: [u32; 16],
    pub fw_reset_seq_delay_msec: [u32; 16],
    pub echo_req_data1: u32,
    pub echo_req_data2: u32,
    pub fw_reporter: *mut devlink_health_reporter,
// Protects severity and remedy
    pub lock: mutex,
    pub severity: bnxt_health_severity,
    pub remedy: bnxt_health_remedy,
    pub arrests: u32,
    pub discoveries: u32,
    pub survivals: u32,
    pub fatalities: u32,
    pub diagnoses: u32,
}

pub const BNXT_FW_HEALTH_REG_TYPE_MASK: c_int = 3;
pub const BNXT_FW_HEALTH_REG_TYPE_CFG: c_int = 0;
pub const BNXT_FW_HEALTH_REG_TYPE_GRC: c_int = 1;
pub const BNXT_FW_HEALTH_REG_TYPE_BAR0: c_int = 2;
pub const BNXT_FW_HEALTH_REG_TYPE_BAR1: c_int = 3;

pub const BNXT_FW_HEALTH_WIN_BASE: c_uint = 0x3000;
pub const BNXT_FW_HEALTH_WIN_MAP_OFF: c_int = 8;

pub const BNXT_FW_STATUS_HEALTH_MSK: c_uint = 0xffff;
pub const BNXT_FW_STATUS_HEALTHY: c_uint = 0x8000;
pub const BNXT_FW_STATUS_SHUTDOWN: c_uint = 0x100000;
pub const BNXT_FW_STATUS_RECOVERING: c_uint = 0x400000;

pub const BNXT_FW_RETRY: c_int = 5;
pub const BNXT_FW_IF_RETRY: c_int = 10;
pub const BNXT_FW_SLOT_RESET_RETRY: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_idx {
    BCM57301,
    BCM57302,
    BCM57304,
    BCM57417_NPAR,
    BCM58700,
    BCM57311,
    BCM57312,
    BCM57402,
    BCM57404,
    BCM57406,
    BCM57402_NPAR,
    BCM57407,
    BCM57412,
    BCM57414,
    BCM57416,
    BCM57417,
    BCM57412_NPAR,
    BCM57314,
    BCM57417_SFP,
    BCM57416_SFP,
    BCM57404_NPAR,
    BCM57406_NPAR,
    BCM57407_SFP,
    BCM57407_NPAR,
    BCM57414_NPAR,
    BCM57416_NPAR,
    BCM57452,
    BCM57454,
    BCM5745x_NPAR,
    BCM57508,
    BCM57504,
    BCM57502,
    BCM57508_NPAR,
    BCM57504_NPAR,
    BCM57502_NPAR,
    BCM57608,
    BCM57604,
    BCM57602,
    BCM57601,
    BCM58802,
    BCM58804,
    BCM58808,
    NETXTREME_E_VF,
    NETXTREME_C_VF,
    NETXTREME_S_VF,
    NETXTREME_C_VF_HV,
    NETXTREME_E_VF_HV,
    NETXTREME_E_P5_VF,
    NETXTREME_E_P5_VF_HV,
    NETXTREME_E_P7_VF,
    NETXTREME_E_P7_VF_HV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_bs_trace_info {
    pub magic_byte: *mut u8,
    pub last_offset: u32,
    pub wrapped:1: u8,
    pub ctx_type: u16,
    pub trace_type: u16,
}

// bs_trace->magic_byte != BNXT_TRACE_BUF_MAGIC_BYTE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt {
    pub bar0: *mut void __iomem,
    pub bar1: *mut void __iomem,
    pub bar2: *mut void __iomem,
    pub reg_base: u32,
    pub chip_num: u16,
pub const CHIP_NUM_57301: c_uint = 0x16c8;
pub const CHIP_NUM_57302: c_uint = 0x16c9;
pub const CHIP_NUM_57304: c_uint = 0x16ca;
pub const CHIP_NUM_58700: c_uint = 0x16cd;
pub const CHIP_NUM_57402: c_uint = 0x16d0;
pub const CHIP_NUM_57404: c_uint = 0x16d1;
pub const CHIP_NUM_57406: c_uint = 0x16d2;
pub const CHIP_NUM_57407: c_uint = 0x16d5;
pub const CHIP_NUM_57311: c_uint = 0x16ce;
pub const CHIP_NUM_57312: c_uint = 0x16cf;
pub const CHIP_NUM_57314: c_uint = 0x16df;
pub const CHIP_NUM_57317: c_uint = 0x16e0;
pub const CHIP_NUM_57412: c_uint = 0x16d6;
pub const CHIP_NUM_57414: c_uint = 0x16d7;
pub const CHIP_NUM_57416: c_uint = 0x16d8;
pub const CHIP_NUM_57417: c_uint = 0x16d9;
pub const CHIP_NUM_57412L: c_uint = 0x16da;
pub const CHIP_NUM_57414L: c_uint = 0x16db;
pub const CHIP_NUM_5745X: c_uint = 0xd730;
pub const CHIP_NUM_57452: c_uint = 0xc452;
pub const CHIP_NUM_57454: c_uint = 0xc454;
pub const CHIP_NUM_57508: c_uint = 0x1750;
pub const CHIP_NUM_57504: c_uint = 0x1751;
pub const CHIP_NUM_57502: c_uint = 0x1752;
pub const CHIP_NUM_57608: c_uint = 0x1760;
pub const CHIP_NUM_58802: c_uint = 0xd802;
pub const CHIP_NUM_58804: c_uint = 0xd804;
pub const CHIP_NUM_58808: c_uint = 0xd808;
    pub chip_rev: u8,

pub const BNXT_VPD_FLD_LEN: c_int = 32;
    pub board_partno: [c_char; BNXT_VPD_FLD_LEN],
    pub board_serialno: [c_char; BNXT_VPD_FLD_LEN],
    pub dev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub intr_sem: core::sync::atomic::AtomicI32,
    pub flags: u32,
pub const BNXT_FLAG_CHIP_P5_PLUS: c_uint = 0x1;
pub const BNXT_FLAG_VF: c_uint = 0x2;
pub const BNXT_FLAG_LRO: c_uint = 0x4;

pub const BNXT_FLAG_GRO: c_uint = 0x8;

// Cannot support hardware GRO if CONFIG_INET is not set
pub const BNXT_FLAG_GRO: c_uint = 0x0;

pub const BNXT_FLAG_JUMBO: c_uint = 0x10;
pub const BNXT_FLAG_STRIP_VLAN: c_uint = 0x20;
pub const BNXT_FLAG_RFS: c_uint = 0x100;
pub const BNXT_FLAG_SHARED_RINGS: c_uint = 0x200;
pub const BNXT_FLAG_PORT_STATS: c_uint = 0x400;
pub const BNXT_FLAG_WOL_CAP: c_uint = 0x4000;
pub const BNXT_FLAG_ROCEV1_CAP: c_uint = 0x8000;
pub const BNXT_FLAG_ROCEV2_CAP: c_uint = 0x10000;

pub const BNXT_FLAG_NO_AGG_RINGS: c_uint = 0x20000;
pub const BNXT_FLAG_RX_PAGE_MODE: c_uint = 0x40000;
pub const BNXT_FLAG_CHIP_P7: c_uint = 0x80000;
pub const BNXT_FLAG_MULTI_HOST: c_uint = 0x100000;
pub const BNXT_FLAG_DSN_VALID: c_uint = 0x200000;
pub const BNXT_FLAG_DOUBLE_DB: c_uint = 0x400000;
pub const BNXT_FLAG_UDP_GSO_CAP: c_uint = 0x800000;
pub const BNXT_FLAG_CHIP_NITRO_A0: c_uint = 0x1000000;
pub const BNXT_FLAG_DIM: c_uint = 0x2000000;
pub const BNXT_FLAG_ROCE_MIRROR_CAP: c_uint = 0x4000000;
pub const BNXT_FLAG_TX_COAL_CMPL: c_uint = 0x8000000;
pub const BNXT_FLAG_PORT_STATS_EXT: c_uint = 0x10000000;
pub const BNXT_FLAG_HDS: c_uint = 0x20000000;

pub const BNXT_VF_IS_TRUSTED(bp): c_int = 0;

// Chip class phase 5

// Chip class phase 4.x

// Chip class phase 3.x
    pub aux_priv: [*mut bnxt_aux_priv; __BNXT_AUXDEV_MAX],
    pub edev: [*mut bnxt_en_dev; __BNXT_AUXDEV_MAX],
    pub bnapi: *mut bnxt_napi,
    pub rx_ring: *mut bnxt_rx_ring_info,
    pub tx_ring: *mut bnxt_tx_ring_info,
    pub tx_ring_map: *mut u16,
    pub ): *mut sk_buff,
    pub int): unsigned,
    pub max_tpa_v2: u16,
    pub max_tpa: u16,
    pub rx_buf_size: u32,
    pub /: *mut *mut u32 rx_buf_use_size; / useable size,
    pub rx_offset: u16,
    pub rx_dma_offset: u16,
    pub rx_dir: dma_data_direction,
    pub rx_ring_size: u32,
    pub rx_agg_ring_size: u32,
    pub rx_copybreak: u32,
    pub rx_ring_mask: u32,
    pub rx_agg_ring_mask: u32,
    pub rx_nr_pages: c_int,
    pub rx_agg_nr_pages: c_int,
    pub rx_nr_rings: c_int,
    pub rsscos_nr_ctxs: c_int,
    pub tx_ring_size: u32,
    pub tx_ring_mask: u32,
    pub tx_nr_pages: c_int,
    pub tx_nr_rings: c_int,
    pub tx_nr_rings_per_tc: c_int,
    pub tx_nr_rings_xdp: c_int,
    pub tx_wake_thresh: c_int,
    pub tx_push_thresh: c_int,
    pub tx_push_size: c_int,
    pub cp_ring_size: u32,
    pub cp_ring_mask: u32,
    pub cp_bit: u32,
    pub cp_nr_pages: c_int,
    pub cp_nr_rings: c_int,
// grp_info indexed by completion ring index
    pub grp_info: *mut bnxt_ring_grp_info,
    pub vnic_info: *mut bnxt_vnic_info,
    pub num_rss_ctx: u32,
    pub nr_vnics: c_int,
    pub rss_indir_tbl: *mut u32,
    pub rss_indir_tbl_entries: u16,
    pub rss_hash_cfg: u32,
    pub rss_hash_delta: u32,
    pub rss_cap: u32,
    pub rss_hash_key: [u8; HW_HASH_KEY_SIZE],
    pub rss_hash_key_valid:1: u8,
    pub rss_hash_key_updated:1: u8,
    pub max_mtu: u16,
    pub tso_max_segs: u16,
    pub max_tc: u8,
    pub /: *mut *mut u8 max_lltc; / lossless TCs,
    pub q_info: [bnxt_queue_info; BNXT_MAX_QUEUE],
    pub tc_to_qidx: [u8; BNXT_MAX_QUEUE],
    pub q_ids: [u8; BNXT_MAX_QUEUE],
    pub max_q: u8,
    pub cos0_cos1_shared: u8,
    pub num_tc: u8,
    pub max_pfcwd_tmo_ms: u16,
    pub tph_mode: u8,
    pub current_interval: c_uint,

    pub timer: timer_list,
    pub state: c_ulong,
pub const BNXT_STATE_OPEN: c_int = 0;
pub const BNXT_STATE_IN_SP_TASK: c_int = 1;
pub const BNXT_STATE_READ_STATS: c_int = 2;
pub const BNXT_STATE_FW_RESET_DET: c_int = 3;
pub const BNXT_STATE_IN_FW_RESET: c_int = 4;
pub const BNXT_STATE_ABORT_ERR: c_int = 5;
pub const BNXT_STATE_FW_FATAL_COND: c_int = 6;
pub const BNXT_STATE_DRV_REGISTERED: c_int = 7;
pub const BNXT_STATE_PCI_CHANNEL_IO_FROZEN: c_int = 8;
pub const BNXT_STATE_NAPI_DISABLED: c_int = 9;
pub const BNXT_STATE_FW_ACTIVATE: c_int = 11;
pub const BNXT_STATE_RECOVER: c_int = 12;
pub const BNXT_STATE_FW_NON_FATAL_COND: c_int = 13;
pub const BNXT_STATE_FW_ACTIVATE_RESET: c_int = 14;

    pub irq_tbl: *mut bnxt_irq,
// IRQ affinity, indexed by completion ring. Kept across IRQ
// reallocation, the MSI-X vector index is not stable.
//
    pub ring_cpu_mask: *mut cpumask_var_t,
// Rings for which the mask above was configured from the outside,
// rather than being our own default placement.
//
    pub ring_affinity_set: *mut c_ulong,
    pub max_irqs: c_int,
    pub total_irqs: c_int,
    pub ulp_num_msix_want: c_int,
    pub mac_addr: [u8; ETH_ALEN],
    pub ieee_pfc: *mut ieee_pfc,
    pub ieee_ets: *mut ieee_ets,
    pub dcbx_cap: u8,
    pub default_pri: u8,
    pub max_dscp_value: u8,

    pub msg_enable: u32,
    pub fw_cap: u64,

    pub fw_dbg_cap: u32,

    pub hwrm_spec_code: u32,
    pub hwrm_cmd_seq: u16,
    pub hwrm_cmd_kong_seq: u16,
    pub hwrm_dma_pool: *mut dma_pool,
    pub hwrm_pending_list: hlist_head,
    pub net_stats_prev: rtnl_link_stats64,
    pub port_stats: bnxt_stats_mem,
    pub rx_port_stats_ext: bnxt_stats_mem,
    pub tx_port_stats_ext: bnxt_stats_mem,
    pub fw_rx_stats_ext_size: u16,
    pub fw_tx_stats_ext_size: u16,
    pub hw_ring_stats_size: u16,
    pub pcie_stat_len: u16,
    pub pri2cos_idx: [u8; 8],
    pub pri2cos_valid: u8,
    pub ring_drv_stats_prev: bnxt_total_ring_drv_stats,
    pub hwrm_max_req_len: u16,
    pub hwrm_max_ext_req_len: u16,
    pub hwrm_cmd_timeout: c_uint,
    pub hwrm_cmd_max_timeout: c_uint,
    pub /: *mut *mut mutex hwrm_cmd_lock; / serialize hwrm messages,
    pub ver_resp: hwrm_ver_get_output,
pub const FW_VER_STR_LEN: c_int = 32;
pub const BC_HWRM_STR_LEN: c_int = 21;
    pub fw_ver_str: [c_char; FW_VER_STR_LEN],
    pub hwrm_ver_supp: [c_char; FW_VER_STR_LEN],
    pub nvm_cfg_ver: [c_char; FW_VER_STR_LEN],
    pub fw_ver_code: u64,

    pub vxlan_fw_dst_port_id: u16,
    pub nge_fw_dst_port_id: u16,
    pub vxlan_gpe_fw_dst_port_id: u16,
    pub vxlan_port: __be16,
    pub nge_port: __be16,
    pub vxlan_gpe_port: __be16,
    pub port_partition_type: u8,
    pub port_count: u8,
    pub br_mode: u16,
    pub coal_cap: bnxt_coal_cap,
    pub rx_coal: bnxt_coal,
    pub tx_coal: bnxt_coal,
    pub stats_coal_ticks: u32,
pub const BNXT_DEF_STATS_COAL_TICKS: c_int = 1000000;
pub const BNXT_MIN_STATS_COAL_TICKS: c_int = 250000;
pub const BNXT_MAX_STATS_COAL_TICKS: c_int = 1000000;
// Protects stats_updated_jiffies and writes to sw_stats
    pub stats_lock: spinlock_t,
    pub stats_updated_jiffies: c_ulong,
    pub sp_task: work_struct,
    pub sp_event: c_ulong,
pub const BNXT_RX_NTP_FLTR_SP_EVENT: c_int = 1;
pub const BNXT_LINK_CHNG_SP_EVENT: c_int = 2;
pub const BNXT_HWRM_EXEC_FWD_REQ_SP_EVENT: c_int = 3;
pub const BNXT_RESET_TASK_SP_EVENT: c_int = 6;
pub const BNXT_RST_RING_SP_EVENT: c_int = 7;
pub const BNXT_HWRM_PF_UNLOAD_SP_EVENT: c_int = 8;
pub const BNXT_PERIODIC_STATS_SP_EVENT: c_int = 9;
pub const BNXT_HWRM_PORT_MODULE_SP_EVENT: c_int = 10;
pub const BNXT_RESET_TASK_SILENT_SP_EVENT: c_int = 11;
pub const BNXT_LINK_SPEED_CHNG_SP_EVENT: c_int = 14;
pub const BNXT_FLOW_STATS_SP_EVENT: c_int = 15;
pub const BNXT_UPDATE_PHY_SP_EVENT: c_int = 16;
pub const BNXT_RING_COAL_NOW_SP_EVENT: c_int = 17;
pub const BNXT_FW_RESET_NOTIFY_SP_EVENT: c_int = 18;
pub const BNXT_FW_EXCEPTION_SP_EVENT: c_int = 19;
pub const BNXT_TPH_UPDATE_SP_EVENT: c_int = 20;
pub const BNXT_LINK_CFG_CHANGE_SP_EVENT: c_int = 21;
pub const BNXT_THERMAL_THRESHOLD_SP_EVENT: c_int = 22;
pub const BNXT_FW_ECHO_REQUEST_SP_EVENT: c_int = 23;
pub const BNXT_RESTART_ULP_SP_EVENT: c_int = 24;
    pub fw_reset_task: delayed_work,
    pub fw_reset_state: c_int,
pub const BNXT_FW_RESET_STATE_POLL_VF: c_int = 1;
pub const BNXT_FW_RESET_STATE_RESET_FW: c_int = 2;
pub const BNXT_FW_RESET_STATE_ENABLE_DEV: c_int = 3;
pub const BNXT_FW_RESET_STATE_POLL_FW: c_int = 4;
pub const BNXT_FW_RESET_STATE_OPENING: c_int = 5;
pub const BNXT_FW_RESET_STATE_POLL_FW_DOWN: c_int = 6;
pub const BNXT_FW_RESET_STATE_ABORT: c_int = 7;
    pub fw_reset_min_dsecs: u16,
pub const BNXT_DFLT_FW_RST_MIN_DSECS: c_int = 20;
    pub fw_reset_max_dsecs: u16,
pub const BNXT_DFLT_FW_RST_MAX_DSECS: c_int = 60;
    pub fw_reset_timestamp: c_ulong,
    pub fw_health: *mut bnxt_fw_health,
    pub hw_resc: bnxt_hw_resc,
    pub pf: bnxt_pf_info,
    pub ctx: *mut bnxt_ctx_mem_info,

    pub nr_vfs: c_int,
    pub vf: bnxt_vf_info,
    pub sriov_cfg_wait: wait_queue_head_t,
    pub sriov_cfg: bool,

// ensure atomic 64-bit doorbell writes on 32-bit systems.
    pub db_lock: spinlock_t,

    pub /: *mut *mut int db_offset; / db_offset within db_size,
    pub db_size: c_int,
pub const BNXT_NTP_FLTR_MAX_FLTR: c_int = 4096;

pub const BNXT_NTP_FLTR_HASH_SIZE: c_int = 512;
    pub ntp_fltr_hash_tbl: [hlist_head; BNXT_NTP_FLTR_HASH_SIZE],
    pub /: *mut *mut spinlock_t ntp_fltr_lock; / for hash table add, del,
    pub ntp_fltr_bmap: *mut c_ulong,
    pub ntp_fltr_count: c_int,
    pub max_fltr: c_int,
pub const BNXT_L2_FLTR_MAX_FLTR: c_int = 1024;
pub const BNXT_L2_FLTR_HASH_SIZE: c_int = 32;
    pub l2_fltr_hash_tbl: [hlist_head; BNXT_L2_FLTR_HASH_SIZE],
    pub hash_seed: u32,
    pub toeplitz_prefix: u64,
    pub usr_fltr_list: list_head,
// To protect link related settings during link changes and
// ethtool settings changes.
//
    pub link_lock: mutex,
    pub link_info: bnxt_link_info,
    pub eee: ethtool_keee,
    pub lpi_tmr_lo: u32,
    pub lpi_tmr_hi: u32,
// copied from flags and flags2 in hwrm_port_phy_qcaps_output
    pub phy_flags: u32,

// copied from flags in hwrm_port_mac_qcaps_output
    pub mac_flags: u8,

    pub num_tests: u8,
    pub test_info: *mut bnxt_test_info,
    pub wol_filter_id: u8,
    pub wol: u8,
    pub num_leds: u8,
    pub leds: [bnxt_led_info; BNXT_MAX_LED],
    pub dump_flag: u16,
pub const BNXT_DUMP_LIVE: c_int = 0;
pub const BNXT_DUMP_CRASH: c_int = 1;
pub const BNXT_DUMP_DRIVER: c_int = 2;
pub const BNXT_DUMP_LIVE_WITH_CTX_L1_CACHE: c_int = 3;
    pub xdp_prog: *mut bpf_prog,
    pub ptp_cfg: *mut bnxt_ptp_cfg,
    pub ptp_all_rx_tstamp: u8,
// devlink interface and vf-rep structs
    pub dl: *mut devlink,
    pub dl_port: devlink_port,
    pub eswitch_mode: devlink_eswitch_mode,
    pub /: *mut *mut *mut *mut bnxt_vf_rep vf_reps; / array of vf-rep ptrs,
    pub /: *mut *mut *mut u16 cfa_code_map; / cfa_code -> vf_idx map,
    pub dsn: [u8; 8],
    pub tc_info: *mut bnxt_tc_info,
    pub tc_indr_block_list: list_head,
    pub debugfs_pdev: *mut dentry,

    pub hwmon_dev: *mut device,
    pub warn_thresh_temp: u8,
    pub crit_thresh_temp: u8,
    pub fatal_thresh_temp: u8,
    pub shutdown_thresh_temp: u8,

    pub thermal_threshold_type: u32,
    pub board_idx: board_idx,
    pub fw_crash_mem: *mut bnxt_ctx_pg_info,
    pub fw_crash_len: u32,
    pub bs_trace: [bnxt_bs_trace_info; BNXT_TRACE_MAX],
    pub auxdev_id: c_int,
// synchronize validity checks of available aux devices
    pub auxdev_lock: mutex,
    pub auxdev_state: [u8; __BNXT_AUXDEV_MAX],
pub const BNXT_ADEV_STATE_NONE: c_int = 0;
pub const BNXT_ADEV_STATE_INIT: c_int = 1;
pub const BNXT_ADEV_STATE_ADD: c_int = 2;
}

pub const BNXT_NUM_RX_RING_STATS: c_int = 8;
pub const BNXT_NUM_TX_RING_STATS: c_int = 8;
pub const BNXT_NUM_TPA_RING_STATS: c_int = 4;
pub const BNXT_NUM_TPA_RING_STATS_P5: c_int = 5;
pub const BNXT_NUM_TPA_RING_STATS_P7: c_int = 6;

pub const I2C_DEV_ADDR_A0: c_uint = 0xa0;
pub const I2C_DEV_ADDR_A2: c_uint = 0xa2;
pub const SFF_DIAG_SUPPORT_OFFSET: c_uint = 0x5c;
pub const SFF_MODULE_ID_SFP: c_uint = 0x3;
pub const SFF_MODULE_ID_QSFP: c_uint = 0xc;
pub const SFF_MODULE_ID_QSFP_PLUS: c_uint = 0xd;
pub const SFF_MODULE_ID_QSFP28: c_uint = 0x11;
pub const BNXT_MAX_PHY_I2C_RESP_SIZE: c_int = 64;
pub const BNXT_HDS_THRESHOLD_MAX: c_int = 1023;

// For TX and RX ring doorbells with no ordering guarantee
// For TX and RX ring doorbells
// Must hold rtnl_lock

extern "C" {
    pub fn BNXT_PF(bp->sriov_cfg: bp) && (bp->pf.active_vfs ||) -> return;
}

extern "C" {
    pub fn bnxt_reuse_rx_data(rxr: *mut bnxt_rx_ring_info, cons: u16, data: *mut c_void);
}
extern "C" {
    pub fn bnxt_fw_health_readl(bp: *mut bnxt, reg_idx: c_int) -> u32;
}
extern "C" {
    pub fn bnxt_bs_trace_avail(bp: *mut bnxt, type: u16) -> bool;
}
extern "C" {
    pub fn bnxt_set_tpa_flags(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_set_ring_params(: *mut bnxt);
}
extern "C" {
    pub fn bnxt_set_rx_skb_mode(bp: *mut bnxt, page_mode: bool);
}
extern "C" {
    pub fn bnxt_insert_usr_fltr(bp: *mut bnxt, fltr: *mut bnxt_filter_base);
}
extern "C" {
    pub fn bnxt_del_one_usr_fltr(bp: *mut bnxt, fltr: *mut bnxt_filter_base);
}
extern "C" {
    pub fn bnxt_hwrm_func_drv_unrgtr(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_del_l2_filter(bp: *mut bnxt, fltr: *mut bnxt_l2_filter);
}
extern "C" {
    pub fn bnxt_hwrm_l2_filter_free(bp: *mut bnxt, fltr: *mut bnxt_l2_filter) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_l2_filter_alloc(bp: *mut bnxt, fltr: *mut bnxt_l2_filter) -> c_int;
}
extern "C" {
    pub fn bnxt_fill_ipv6_mask(mask[4]: __be32);
}
extern "C" {
    pub fn bnxt_get_nr_rss_ctxs(bp: *mut bnxt, rx_rings: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_vnic_cfg(bp: *mut bnxt, vnic: *mut bnxt_vnic_info) -> c_int;
}
extern "C" {
    pub fn __bnxt_hwrm_get_tx_rings(bp: *mut bnxt, fid: u16, tx_rings: *mut c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_nq_rings_in_use(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_set_coal(: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_free_ctx_mem(bp: *mut bnxt, force: bool);
}
extern "C" {
    pub fn bnxt_num_tx_to_cp(bp: *mut bnxt, tx: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_get_max_func_stat_ctxs(bp: *mut bnxt) -> c_uint;
}
extern "C" {
    pub fn bnxt_get_avail_stat_ctxs_for_en(bp: *mut bnxt) -> c_uint;
}
extern "C" {
    pub fn bnxt_get_max_func_cp_rings(bp: *mut bnxt) -> c_uint;
}
extern "C" {
    pub fn bnxt_get_avail_cp_rings_for_en(bp: *mut bnxt) -> c_uint;
}
extern "C" {
    pub fn bnxt_reserve_rings(bp: *mut bnxt, irq_re_init: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_tx_disable(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_tx_enable(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_xmit_get_cfa_action(skb: *mut sk_buff) -> u16;
}
extern "C" {
    pub fn bnxt_report_link(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_update_link(bp: *mut bnxt, chng_link_state: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_set_pause(: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_set_link_setting(: *mut bnxt, _arg: bool, _arg: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_clear_reservations(bp: *mut bnxt, fw_reset: bool);
}
extern "C" {
    pub fn bnxt_cancel_reservations(bp: *mut bnxt, fw_reset: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_alloc_wol_fltr(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_free_wol_fltr(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_func_resc_qcaps(bp: *mut bnxt, all: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_func_qcaps(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_fw_set_time(: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_vnic_rss_cfg_p5(bp: *mut bnxt, vnic: *mut bnxt_vnic_info) -> c_int;
}
extern "C" {
    pub fn __bnxt_setup_vnic_p5(bp: *mut bnxt, vnic: *mut bnxt_vnic_info) -> c_int;
}
extern "C" {
    pub fn bnxt_open_nic(: *mut bnxt, _arg: bool, _arg: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_half_open_nic(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_half_close_nic(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_reenable_sriov(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_close_nic(: *mut bnxt, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn bnxt_sync_ring_stats(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_rfs_capable(bp: *mut bnxt, new_rss_ctx: bool) -> bool;
}
extern "C" {
    pub fn bnxt_fw_exception(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_fw_reset(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_fw_init_one(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_hwrm_reset_permitted(bp: *mut bnxt) -> bool;
}
extern "C" {
    pub fn bnxt_set_cp_rings(bp: *mut bnxt, sh: bool);
}
extern "C" {
    pub fn bnxt_setup_mq_tc(dev: *mut net_device, tc: u8) -> c_int;
}
extern "C" {
    pub fn bnxt_del_ntp_filter(bp: *mut bnxt, fltr: *mut bnxt_ntuple_filter);
}
extern "C" {
    pub fn bnxt_get_max_rings(: *mut bnxt, : *mut c_int, : *mut c_int, _arg: bool) -> c_int;
}
extern "C" {
    pub fn bnxt_restore_pf_fw_resources(bp: *mut bnxt) -> c_int;
}
extern "C" {
    pub fn bnxt_dim_work(work: *mut work_struct);
}
extern "C" {
    pub fn bnxt_hwrm_set_ring_coal(bp: *mut bnxt, bnapi: *mut bnxt_napi) -> c_int;
}
extern "C" {
    pub fn bnxt_print_device_info(bp: *mut bnxt);
}
