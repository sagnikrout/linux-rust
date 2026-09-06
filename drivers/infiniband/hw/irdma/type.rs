//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/type.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2015 - 2021 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_page_size {
    IRDMA_PAGE_SIZE_4K = 0,
    IRDMA_PAGE_SIZE_2M,
    IRDMA_PAGE_SIZE_1G,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_hdrct_flags {
    DDP_LEN_FLAG  = 0x80,
    DDP_HDR_FLAG  = 0x40,
    RDMA_HDR_FLAG = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_term_layers {
    LAYER_RDMA = 0,
    LAYER_DDP  = 1,
    LAYER_MPA  = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_term_error_types {
    RDMAP_REMOTE_PROT = 1,
    RDMAP_REMOTE_OP   = 2,
    DDP_CATASTROPHIC  = 0,
    DDP_TAGGED_BUF    = 1,
    DDP_UNTAGGED_BUF  = 2,
    DDP_LLP		  = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_term_rdma_errors {
    RDMAP_INV_STAG		  = 0x00,
    RDMAP_INV_BOUNDS	  = 0x01,
    RDMAP_ACCESS		  = 0x02,
    RDMAP_UNASSOC_STAG	  = 0x03,
    RDMAP_TO_WRAP		  = 0x04,
    RDMAP_INV_RDMAP_VER       = 0x05,
    RDMAP_UNEXPECTED_OP       = 0x06,
    RDMAP_CATASTROPHIC_LOCAL  = 0x07,
    RDMAP_CATASTROPHIC_GLOBAL = 0x08,
    RDMAP_CANT_INV_STAG       = 0x09,
    RDMAP_UNSPECIFIED	  = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_term_ddp_errors {
    DDP_CATASTROPHIC_LOCAL      = 0x00,
    DDP_TAGGED_INV_STAG	    = 0x00,
    DDP_TAGGED_BOUNDS	    = 0x01,
    DDP_TAGGED_UNASSOC_STAG     = 0x02,
    DDP_TAGGED_TO_WRAP	    = 0x03,
    DDP_TAGGED_INV_DDP_VER      = 0x04,
    DDP_UNTAGGED_INV_QN	    = 0x01,
    DDP_UNTAGGED_INV_MSN_NO_BUF = 0x02,
    DDP_UNTAGGED_INV_MSN_RANGE  = 0x03,
    DDP_UNTAGGED_INV_MO	    = 0x04,
    DDP_UNTAGGED_INV_TOO_LONG   = 0x05,
    DDP_UNTAGGED_INV_DDP_VER    = 0x06,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_term_mpa_errors {
    MPA_CLOSED  = 0x01,
    MPA_CRC     = 0x02,
    MPA_MARKER  = 0x03,
    MPA_REQ_RSP = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_hw_stats_index {
// gen1 - 32-bit
    IRDMA_HW_STAT_INDEX_IP4RXDISCARD	= 0,
    IRDMA_HW_STAT_INDEX_IP4RXTRUNC		= 1,
    IRDMA_HW_STAT_INDEX_IP4TXNOROUTE	= 2,
    IRDMA_HW_STAT_INDEX_IP6RXDISCARD	= 3,
    IRDMA_HW_STAT_INDEX_IP6RXTRUNC		= 4,
    IRDMA_HW_STAT_INDEX_IP6TXNOROUTE	= 5,
    IRDMA_HW_STAT_INDEX_TCPRTXSEG		= 6,
    IRDMA_HW_STAT_INDEX_TCPRXOPTERR		= 7,
    IRDMA_HW_STAT_INDEX_TCPRXPROTOERR	= 8,
    IRDMA_HW_STAT_INDEX_RXVLANERR		= 9,
// gen1 - 64-bit
    IRDMA_HW_STAT_INDEX_IP4RXOCTS		= 10,
    IRDMA_HW_STAT_INDEX_IP4RXPKTS		= 11,
    IRDMA_HW_STAT_INDEX_IP4RXFRAGS		= 12,
    IRDMA_HW_STAT_INDEX_IP4RXMCPKTS		= 13,
    IRDMA_HW_STAT_INDEX_IP4TXOCTS		= 14,
    IRDMA_HW_STAT_INDEX_IP4TXPKTS		= 15,
    IRDMA_HW_STAT_INDEX_IP4TXFRAGS		= 16,
    IRDMA_HW_STAT_INDEX_IP4TXMCPKTS		= 17,
    IRDMA_HW_STAT_INDEX_IP6RXOCTS		= 18,
    IRDMA_HW_STAT_INDEX_IP6RXPKTS		= 19,
    IRDMA_HW_STAT_INDEX_IP6RXFRAGS		= 20,
    IRDMA_HW_STAT_INDEX_IP6RXMCPKTS		= 21,
    IRDMA_HW_STAT_INDEX_IP6TXOCTS		= 22,
    IRDMA_HW_STAT_INDEX_IP6TXPKTS		= 23,
    IRDMA_HW_STAT_INDEX_IP6TXFRAGS		= 24,
    IRDMA_HW_STAT_INDEX_IP6TXMCPKTS		= 25,
    IRDMA_HW_STAT_INDEX_TCPRXSEGS		= 26,
    IRDMA_HW_STAT_INDEX_TCPTXSEG		= 27,
    IRDMA_HW_STAT_INDEX_RDMARXRDS		= 28,
    IRDMA_HW_STAT_INDEX_RDMARXSNDS		= 29,
    IRDMA_HW_STAT_INDEX_RDMARXWRS		= 30,
    IRDMA_HW_STAT_INDEX_RDMATXRDS		= 31,
    IRDMA_HW_STAT_INDEX_RDMATXSNDS		= 32,
    IRDMA_HW_STAT_INDEX_RDMATXWRS		= 33,
    IRDMA_HW_STAT_INDEX_RDMAVBND		= 34,
    IRDMA_HW_STAT_INDEX_RDMAVINV		= 35,
    IRDMA_HW_STAT_INDEX_IP4RXMCOCTS         = 36,
    IRDMA_HW_STAT_INDEX_IP4TXMCOCTS         = 37,
    IRDMA_HW_STAT_INDEX_IP6RXMCOCTS         = 38,
    IRDMA_HW_STAT_INDEX_IP6TXMCOCTS         = 39,
    IRDMA_HW_STAT_INDEX_UDPRXPKTS           = 40,
    IRDMA_HW_STAT_INDEX_UDPTXPKTS           = 41,
    IRDMA_HW_STAT_INDEX_MAX_GEN_1           = 42, /* Must be same value as next entry */
// gen2 - 64-bit
    IRDMA_HW_STAT_INDEX_RXNPECNMARKEDPKTS   = 42,
// gen2 - 32-bit
    IRDMA_HW_STAT_INDEX_RXRPCNPHANDLED      = 43,
    IRDMA_HW_STAT_INDEX_RXRPCNPIGNORED      = 44,
    IRDMA_HW_STAT_INDEX_TXNPCNPSENT         = 45,
    IRDMA_HW_STAT_INDEX_MAX_GEN_2		= 46,

// gen3
    IRDMA_HW_STAT_INDEX_RNR_SENT		= 46,
    IRDMA_HW_STAT_INDEX_RNR_RCVD		= 47,
    IRDMA_HW_STAT_INDEX_RDMAORDLMTCNT	= 48,
    IRDMA_HW_STAT_INDEX_RDMAIRDLMTCNT	= 49,
    IRDMA_HW_STAT_INDEX_RDMARXATS		= 50,
    IRDMA_HW_STAT_INDEX_RDMATXATS		= 51,
    IRDMA_HW_STAT_INDEX_NAKSEQERR		= 52,
    IRDMA_HW_STAT_INDEX_NAKSEQERR_IMPLIED	= 53,
    IRDMA_HW_STAT_INDEX_RTO			= 54,
    IRDMA_HW_STAT_INDEX_RXOOOPKTS		= 55,
    IRDMA_HW_STAT_INDEX_ICRCERR		= 56,

    IRDMA_HW_STAT_INDEX_MAX_GEN_3		= 57,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_feature_type {
    IRDMA_FEATURE_FW_INFO = 0,
    IRDMA_HW_VERSION_INFO = 1,
    IRDMA_QP_MAX_INCR     = 2,
    IRDMA_CQ_MAX_INCR     = 3,
    IRDMA_CEQ_MAX_INCR    = 4,
    IRDMA_SD_MAX_INCR     = 5,
    IRDMA_MR_MAX_INCR     = 6,
    IRDMA_Q1_MAX_INCR     = 7,
    IRDMA_AH_MAX_INCR     = 8,
    IRDMA_SRQ_MAX_INCR    = 9,
    IRDMA_TIMER_MAX_INCR  = 10,
    IRDMA_XF_MAX_INCR     = 11,
    IRDMA_RRF_MAX_INCR    = 12,
    IRDMA_PBLE_MAX_INCR   = 13,
    IRDMA_OBJ_1           = 22,
    IRDMA_OBJ_2           = 23,
    IRDMA_ENDPT_TRK       = 24,
    IRDMA_FTN_INLINE_MAX  = 25,
    IRDMA_QSETS_MAX       = 26,
    IRDMA_ASO	      = 27,
    IRDMA_FTN_FLAGS	      = 32,
    IRDMA_FTN_NOP         = 33,
    IRDMA_MAX_FEATURES, /* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_sched_prio_type {
    IRDMA_PRIO_WEIGHTED_RR     = 1,
    IRDMA_PRIO_STRICT	   = 2,
    IRDMA_PRIO_WEIGHTED_STRICT = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_vm_vf_type {
    IRDMA_VF_TYPE = 0,
    IRDMA_VM_TYPE,
    IRDMA_PF_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_cqp_hmc_profile {
    IRDMA_HMC_PROFILE_DEFAULT  = 1,
    IRDMA_HMC_PROFILE_FAVOR_VF = 2,
    IRDMA_HMC_PROFILE_EQUAL    = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_quad_entry_type {
    IRDMA_QHASH_TYPE_TCP_ESTABLISHED = 1,
    IRDMA_QHASH_TYPE_TCP_SYN,
    IRDMA_QHASH_TYPE_UDP_UNICAST,
    IRDMA_QHASH_TYPE_UDP_MCAST,
    IRDMA_QHASH_TYPE_ROCE_MCAST,
    IRDMA_QHASH_TYPE_ROCEV2_HW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_quad_hash_manage_type {
    IRDMA_QHASH_MANAGE_TYPE_DELETE = 0,
    IRDMA_QHASH_MANAGE_TYPE_ADD,
    IRDMA_QHASH_MANAGE_TYPE_MODIFY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_syn_rst_handling {
    IRDMA_SYN_RST_HANDLING_HW_TCP_SECURE = 0,
    IRDMA_SYN_RST_HANDLING_HW_TCP,
    IRDMA_SYN_RST_HANDLING_FW_TCP_SECURE,
    IRDMA_SYN_RST_HANDLING_FW_TCP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_queue_type {
    IRDMA_QUEUE_TYPE_SQ_RQ = 0,
    IRDMA_QUEUE_TYPE_CQP,
    IRDMA_QUEUE_TYPE_SRQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_rsvd_cq_id {
    IRDMA_RSVD_CQ_ID_CQP,
    IRDMA_RSVD_CQ_ID_ILQ,
    IRDMA_RSVD_CQ_ID_IEQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_rsvd_qp_id {
    IRDMA_RSVD_QP_ID_0,
    IRDMA_RSVD_QP_ID_GSI_ILQ,
    IRDMA_RSVD_QP_ID_IEQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dcqcn_cc_params {
    pub cc_cfg_valid: u8,
    pub min_dec_factor: u8,
    pub min_rate: u8,
    pub dcqcn_f: u8,
    pub rai_factor: u16,
    pub hai_factor: u16,
    pub dcqcn_t: u16,
    pub dcqcn_b: u32,
    pub rreduce_mperiod: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_init_info {
    pub cqp_compl_ctx: u64,
    pub host_ctx_pa: u64,
    pub sq_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub sq: *mut irdma_cqp_quanta,
    pub dcqcn_params: irdma_dcqcn_cc_params,
    pub host_ctx: *mut __le64,
    pub scratch_array: *mut u64,
    pub sq_size: u32,
    pub ooo_op_array: *mut irdma_ooo_cqp_op,
    pub pe_en_vf_cnt: u32,
    pub hw_maj_ver: u16,
    pub hw_min_ver: u16,
    pub struct_ver: u8,
    pub hmc_profile: u8,
    pub ena_vf_count: u8,
    pub ceqs_per_vf: u8,
    pub ooisc_blksize: u8,
    pub rrsp_blksize: u8,
    pub q1_blksize: u8,
    pub xmit_blksize: u8,
    pub ts_override: u8,
    pub ts_shift: u8,
    pub en_fine_grained_timers: u8,
    pub blksizes_valid: u8,
    pub en_datacenter_tcp:1: bool,
    pub disable_packed:1: bool,
    pub rocev2_rto_policy:1: bool,
    pub protocol_used: irdma_protocol_used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_terminate_hdr {
    pub layer_etype: u8,
    pub error_code: u8,
    pub hdrct: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_sq_wqe {
    pub buf: [__le64; IRDMA_CQP_WQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_aeqe {
    pub buf: [__le64; IRDMA_AEQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ceqe {
    pub buf: [__le64; IRDMA_CEQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_ctx {
    pub buf: [__le64; IRDMA_CQP_CTX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_shadow_area {
    pub buf: [__le64; IRDMA_SHADOW_AREA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dev_hw_stats_offsets {
    pub stats_offset: [u32; IRDMA_HW_STAT_INDEX_MAX_GEN_1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dev_hw_stats {
    pub sizeof(u64)]: u64 stats_val[IRDMA_GATHER_STATS_BUF_SIZE /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_gather_stats {
    pub sizeof(u64)]: u64 val[IRDMA_GATHER_STATS_BUF_SIZE /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hw_stat_map {
    pub byteoff: u16,
    pub bitoff: u8,
    pub bitmask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_stats_gather_info {
    pub use_hmc_fcn_index:1: bool,
    pub use_stats_inst:1: bool,
    pub hmc_fcn_index: u8,
    pub stats_inst_index: u8,
    pub stats_buff_mem: irdma_dma_mem,
    pub gather_stats_va: *mut c_void,
    pub last_gather_stats_va: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vsi_pestat {
    pub hw: *mut irdma_hw,
    pub hw_stats: irdma_dev_hw_stats,
    pub gather_info: irdma_stats_gather_info,
    pub stats_timer: timer_list,
    pub vsi: *mut irdma_sc_vsi,
    pub last_hw_stats: irdma_dev_hw_stats,
    pub /: *mut *mut spinlock_t lock; / rdma stats lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mmio_region {
    pub addr: *mut u8 __iomem,
    pub len: resource_size_t,
    pub offset: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hw {
    pub hw_addr: *mut u8 __iomem,
    pub /: *mut *mut irdma_mmio_region rdma_reg; / RDMA region,
    pub /: *mut *mut *mut irdma_mmio_region io_regs; / Non-RDMA MMIO regions,
    pub /: *mut *mut u16 num_io_regions; / Number of Non-RDMA MMIO regions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_pfpdu {
    pub rxlist: list_head,
    pub rcv_nxt: u32,
    pub fps: u32,
    pub max_fpdu_data: u32,
    pub nextseqnum: u32,
    pub rcv_start_seq: u32,
    pub mode:1: bool,
    pub mpa_crc_err:1: bool,
    pub marker_len: u8,
    pub total_ieq_bufs: u64,
    pub fpdu_processed: u64,
    pub bad_seq_num: u64,
    pub crc_err: u64,
    pub no_tx_bufs: u64,
    pub tx_err: u64,
    pub out_of_order: u64,
    pub pmode_count: u64,
    pub ah: *mut irdma_sc_ah,
    pub ah_buf: *mut irdma_puda_buf,
    pub /: *mut *mut spinlock_t lock; / fpdu processing lock,
    pub lastrcv_buf: *mut irdma_puda_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_pd {
    pub dev: *mut irdma_sc_dev,
    pub pd_id: u32,
    pub abi_ver: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_quanta {
    pub elem: [__le64; IRDMA_CQP_WQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ooo_cqp_op {
    pub list_entry: list_head,
    pub scratch: u64,
    pub def_info: u32,
    pub sw_def_info: u32,
    pub wqe_idx: u32,
    pub deferred:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_cqp {
    pub /: *mut *mut spinlock_t ooo_list_lock; / protects list of pending completions,
    pub ooo_avail: list_head,
    pub ooo_pnd: list_head,
    pub last_def_cmpl_ticket: u32,
    pub sw_def_cmpl_ticket: u32,
    pub size: u32,
    pub sq_pa: u64,
    pub host_ctx_pa: u64,
    pub back_cqp: *mut c_void,
    pub dev: *mut irdma_sc_dev,
    pub info): *mut irdma_update_sds_info,
    pub sdbuf: irdma_dma_mem,
    pub sq_ring: irdma_ring,
    pub sq_base: *mut irdma_cqp_quanta,
    pub dcqcn_params: irdma_dcqcn_cc_params,
    pub host_ctx: *mut __le64,
    pub scratch_array: *mut u64,
    pub requested_ops: u64,
    pub completed_ops: core::sync::atomic::AtomicI64,
    pub ooo_op_array: *mut irdma_ooo_cqp_op,
    pub cqp_id: u32,
    pub sq_size: u32,
    pub pe_en_vf_cnt: u32,
    pub hw_sq_size: u32,
    pub hw_maj_ver: u16,
    pub hw_min_ver: u16,
    pub struct_ver: u8,
    pub polarity: u8,
    pub hmc_profile: u8,
    pub ena_vf_count: u8,
    pub timeout_count: u8,
    pub ceqs_per_vf: u8,
    pub ooisc_blksize: u8,
    pub rrsp_blksize: u8,
    pub q1_blksize: u8,
    pub xmit_blksize: u8,
    pub ts_override: u8,
    pub ts_shift: u8,
    pub en_fine_grained_timers: u8,
    pub blksizes_valid: u8,
    pub en_datacenter_tcp:1: bool,
    pub disable_packed:1: bool,
    pub rocev2_rto_policy:1: bool,
    pub protocol_used: irdma_protocol_used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_aeq {
    pub size: u32,
    pub aeq_elem_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub aeqe_base: *mut irdma_sc_aeqe,
    pub pbl_list: *mut c_void,
    pub elem_cnt: u32,
    pub aeq_ring: irdma_ring,
    pub pbl_chunk_size: u8,
    pub first_pm_pbl_idx: u32,
    pub msix_idx: u32,
    pub polarity: u8,
    pub virtual_map:1: bool,
    pub pasid_valid:1: bool,
    pub pasid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_ceq {
    pub size: u32,
    pub ceq_elem_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub ceqe_base: *mut irdma_ceqe,
    pub pbl_list: *mut c_void,
    pub ceq_id: u32,
    pub elem_cnt: u32,
    pub ceq_ring: irdma_ring,
    pub pbl_chunk_size: u8,
    pub tph_val: u8,
    pub first_pm_pbl_idx: u32,
    pub polarity: u8,
    pub vsi_idx: u16,
    pub virtual_map:1: bool,
    pub tph_en:1: bool,
    pub itr_no_expire:1: bool,
    pub pasid_valid:1: bool,
    pub pasid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_cq {
    pub cq_uk: irdma_cq_uk,
    pub cq_pa: u64,
    pub shadow_area_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub vsi_idx: u16,
    pub vsi: *mut irdma_sc_vsi,
    pub pbl_list: *mut c_void,
    pub back_cq: *mut c_void,
    pub ceq_id: u32,
    pub shadow_read_threshold: u32,
    pub pbl_chunk_size: u8,
    pub cq_type: u8,
    pub tph_val: u8,
    pub first_pm_pbl_idx: u32,
    pub ceqe_mask:1: bool,
    pub virtual_map:1: bool,
    pub check_overflow:1: bool,
    pub ceq_id_valid:1: bool,
    pub tph_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_qp {
    pub qp_uk: irdma_qp_uk,
    pub sq_pa: u64,
    pub rq_pa: u64,
    pub hw_host_ctx_pa: u64,
    pub shadow_area_pa: u64,
    pub q2_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub vsi: *mut irdma_sc_vsi,
    pub pd: *mut irdma_sc_pd,
    pub hw_host_ctx: *mut __le64,
    pub llp_stream_handle: *mut c_void,
    pub pfpdu: irdma_pfpdu,
    pub ieq_qp: u32,
    pub q2_buf: *mut u8,
    pub qp_compl_ctx: u64,
    pub push_idx: u32,
    pub qs_handle: u16,
    pub push_offset: u16,
    pub flush_wqes_count: u8,
    pub sq_tph_val: u8,
    pub rq_tph_val: u8,
    pub qp_state: u8,
    pub hw_sq_size: u8,
    pub hw_rq_size: u8,
    pub src_mac_addr_idx: u8,
    pub on_qoslist:1: bool,
    pub ieq_pass_thru:1: bool,
    pub sq_tph_en:1: bool,
    pub rq_tph_en:1: bool,
    pub rcv_tph_en:1: bool,
    pub xmit_tph_en:1: bool,
    pub virtual_map:1: bool,
    pub flush_sq:1: bool,
    pub flush_rq:1: bool,
    pub err_sq_idx_valid:1: bool,
    pub err_rq_idx_valid:1: bool,
    pub err_sq_idx: u32,
    pub err_rq_idx: u32,
    pub sq_flush_code:1: bool,
    pub rq_flush_code:1: bool,
    pub pkt_limit: u32,
    pub flush_code: irdma_flush_opcode,
    pub event_type: irdma_qp_event_type,
    pub term_flags: u8,
    pub user_pri: u8,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_stats_inst_info {
    pub use_hmc_fcn_index: bool,
    pub hmc_fn_id: u8,
    pub stats_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_up_info {
    pub map: [u8; 8],
    pub cnp_up_override: u8,
    pub hmc_fcn_idx: u16,
    pub use_vlan:1: bool,
    pub use_cnp_up_override:1: bool,
}

pub const IRDMA_MAX_WS_NODES: c_uint = 0x3FF;
pub const IRDMA_WS_NODE_INVALID: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ws_node_info {
    pub id: u16,
    pub vsi: u16,
    pub parent_id: u16,
    pub qs_handle: u16,
    pub type_leaf:1: bool,
    pub enable:1: bool,
    pub prio_type: u8,
    pub tc: u8,
    pub weight: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hmc_fpm_misc {
    pub max_ceqs: u32,
    pub max_sds: u32,
    pub loc_mem_pages: u32,
    pub ird: u8,
    pub xf_block_size: u32,
    pub q1_block_size: u32,
    pub ht_multiplier: u32,
    pub timer_bucket: u32,
    pub rrf_block_size: u32,
    pub ooiscf_block_size: u32,
    pub fw_scratch_buf0: irdma_dma_mem,
    pub fw_scratch_buf1: irdma_dma_mem,
}

pub const IRDMA_VCHNL_MAX_MSG_SIZE: c_int = 512;
pub const IRDMA_LEAF_DEFAULT_REL_BW: c_int = 64;
pub const IRDMA_PARENT_DEFAULT_REL_BW: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qos {
    pub qplist: list_head,
    pub /: *mut *mut mutex qos_mutex; / protect QoS attributes per QoS level,
    pub lan_qos_handle: u64,
    pub l2_sched_node_id: u32,
    pub qs_handle: u16,
    pub traffic_class: u8,
    pub rel_bw: u8,
    pub prio_type: u8,
    pub valid: bool,
}

pub const IRDMA_INVALID_STATS_IDX: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_vsi {
    pub vsi_idx: u16,
    pub dev: *mut irdma_sc_dev,
    pub back_vsi: *mut c_void,
    pub ilq_count: u32,
    pub ilq_mem: irdma_virt_mem,
    pub ilq: *mut irdma_puda_rsrc,
    pub ieq_count: u32,
    pub ieq_mem: irdma_virt_mem,
    pub ieq: *mut irdma_puda_rsrc,
    pub exception_lan_q: u32,
    pub mtu: u16,
    pub vm_id: u16,
    pub vm_vf_type: irdma_vm_vf_type,
    pub stats_inst_alloc:1: bool,
    pub tc_change_pending:1: bool,
    pub pestat: *mut irdma_vsi_pestat,
    pub qp_suspend_reqs: core::sync::atomic::AtomicI32,
    pub tc_node): *mut irdma_ws_node,
    pub tc_node): *mut irdma_ws_node,
    pub qos_rel_bw: u8,
    pub qos_prio_type: u8,
    pub stats_idx: u8,
    pub dscp_map: [u8; DSCP_MAX],
    pub qos: [irdma_qos; IRDMA_MAX_USER_PRIORITY],
    pub hw_stats_regs: [u64; IRDMA_HW_STAT_INDEX_MAX_GEN_1],
    pub dscp_mode:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_dev {
    pub /: *mut *mut list_head cqp_cmd_head; / head of the CQP command list,
    pub /: *mut *mut spinlock_t cqp_lock; / protect CQP list access,
    pub stats_idx_array: [bool; IRDMA_MAX_STATS_COUNT_GEN_1],
    pub vf_fpm_query_buf: [irdma_dma_mem; IRDMA_MAX_PE_ENA_VF_COUNT],
    pub fpm_query_buf_pa: u64,
    pub fpm_commit_buf_pa: u64,
    pub fpm_query_buf: *mut __le64,
    pub fpm_commit_buf: *mut __le64,
    pub hw: *mut irdma_hw,
    pub db_addr: *mut u8 __iomem,
    pub wqe_alloc_db: *mut u32 __iomem,
    pub cq_arm_db: *mut u32 __iomem,
    pub aeq_alloc_db: *mut u32 __iomem,
    pub cqp_db: *mut u32 __iomem,
    pub cq_ack_db: *mut u32 __iomem,
    pub ceq_itr_mask_db: *mut u32 __iomem,
    pub aeq_itr_mask_db: *mut u32 __iomem,
    pub hw_regs: [*mut u32 __iomem; IRDMA_MAX_REGS],
    pub /: *mut *mut u32 ceq_itr; / Interrupt throttle, usecs between interrupts: 0 disabled. 2 - 8160,
    pub hw_masks: [u64; IRDMA_MAX_MASKS],
    pub hw_shifts: [u64; IRDMA_MAX_SHIFTS],
    pub hw_stats_map: *const irdma_hw_stat_map,
    pub hw_stats_regs: [u64; IRDMA_HW_STAT_INDEX_MAX_GEN_1],
    pub feature_info: [u64; IRDMA_MAX_FEATURES],
    pub cqp_cmd_stats: [u64; IRDMA_MAX_CQP_OPS],
    pub hw_attrs: irdma_hw_attrs,
    pub hmc_info: *mut irdma_hmc_info,
    pub vc_caps: irdma_vchnl_rdma_caps,
    pub vc_recv_buf: [u8; IRDMA_VCHNL_MAX_MSG_SIZE],
    pub vc_recv_len: u16,
    pub cqp: *mut irdma_sc_cqp,
    pub aeq: *mut irdma_sc_aeq,
    pub ceq: [*mut irdma_sc_ceq; IRDMA_CEQ_MAX_COUNT],
    pub ccq: *mut irdma_sc_cq,
    pub puda_cq_lock: spinlock_t,
    pub ilq_cq: *mut irdma_sc_cq,
    pub ieq_cq: *mut irdma_sc_cq,
    pub irq_ops: *const irdma_irq_ops,
    pub qos: [irdma_qos; IRDMA_MAX_USER_PRIORITY],
    pub hmc_fpm_misc: irdma_hmc_fpm_misc,
    pub ws_tree_root: *mut irdma_ws_node,
    pub /: *mut *mut mutex ws_mutex; / ws tree mutex,
    pub vchnl_ver: u32,
    pub num_vfs: u16,
    pub hmc_fn_id: u16,
    pub vf_id: u16,
    pub privileged:1: bool,
    pub vchnl_up:1: bool,
    pub ceq_valid:1: bool,
    pub is_pf:1: bool,
    pub protocol_used: u8,
    pub /: *mut *mut mutex vchnl_mutex; / mutex to synchronize RDMA virtual channel messages,
    pub pci_rev: u8,
    pub user_pri): *mut *mut *mut int (ws_add)(struct irdma_sc_vsi vsi, u8,
    pub user_pri): *mut *mut *mut void (ws_remove)(struct irdma_sc_vsi vsi, u8,
    pub vsi): *mut *mut void (ws_reset)(struct irdma_sc_vsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_modify_cq_info {
    pub cq_pa: u64,
    pub cq_base: *mut irdma_cqe,
    pub cq_size: u32,
    pub shadow_read_threshold: u32,
    pub pbl_chunk_size: u8,
    pub first_pm_pbl_idx: u32,
    pub virtual_map:1: bool,
    pub check_overflow: bool,
    pub cq_resize:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_srq_init_info {
    pub pd: *mut irdma_sc_pd,
    pub vsi: *mut irdma_sc_vsi,
    pub srq_pa: u64,
    pub shadow_area_pa: u64,
    pub first_pm_pbl_idx: u32,
    pub pasid: u32,
    pub srq_size: u32,
    pub srq_limit: u16,
    pub pasid_valid: u8,
    pub wqe_size: u8,
    pub leaf_pbl_size: u8,
    pub virtual_map: u8,
    pub tph_en: u8,
    pub arm_limit_event: u8,
    pub tph_value: u8,
    pub pbl_chunk_size: u8,
    pub srq_uk_init_info: irdma_srq_uk_init_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_sc_srq {
    pub dev: *mut irdma_sc_dev,
    pub vsi: *mut irdma_sc_vsi,
    pub pd: *mut irdma_sc_pd,
    pub srq_uk: irdma_srq_uk,
    pub back_srq: *mut c_void,
    pub srq_pa: u64,
    pub shadow_area_pa: u64,
    pub first_pm_pbl_idx: u32,
    pub pasid: u32,
    pub hw_srq_size: u32,
    pub srq_limit: u16,
    pub pasid_valid: u8,
    pub leaf_pbl_size: u8,
    pub virtual_map: u8,
    pub tph_en: u8,
    pub arm_limit_event: u8,
    pub tph_val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_modify_srq_info {
    pub srq_limit: u16,
    pub arm_limit_event: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_create_qp_info {
    pub ord_valid:1: bool,
    pub tcp_ctx_valid:1: bool,
    pub cq_num_valid:1: bool,
    pub arp_cache_idx_valid:1: bool,
    pub mac_valid:1: bool,
    pub force_lpb: bool,
    pub next_iwarp_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_modify_qp_info {
    pub rx_win0: u64,
    pub rx_win1: u64,
    pub new_mss: u16,
    pub next_iwarp_state: u8,
    pub curr_iwarp_state: u8,
    pub termlen: u8,
    pub ord_valid:1: bool,
    pub tcp_ctx_valid:1: bool,
    pub udp_ctx_valid:1: bool,
    pub cq_num_valid:1: bool,
    pub arp_cache_idx_valid:1: bool,
    pub reset_tcp_conn:1: bool,
    pub remove_hash_idx:1: bool,
    pub dont_send_term:1: bool,
    pub dont_send_fin:1: bool,
    pub cached_var_valid:1: bool,
    pub mss_change:1: bool,
    pub force_lpb:1: bool,
    pub mac_valid:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ccq_cqe_info {
    pub cqp: *mut irdma_sc_cqp,
    pub scratch: u64,
    pub op_ret_val: u32,
    pub maj_err_code: u16,
    pub min_err_code: u16,
    pub op_code: u8,
    pub error:1: bool,
    pub pending:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dcb_app_info {
    pub priority: u8,
    pub selector: u8,
    pub prot_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qos_tc_info {
    pub tc_ctx: u64,
    pub rel_bw: u8,
    pub prio_type: u8,
    pub egress_virt_up: u8,
    pub ingress_virt_up: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_l2params {
    pub tc_info: [irdma_qos_tc_info; IRDMA_MAX_USER_PRIORITY],
    pub apps: [irdma_dcb_app_info; IRDMA_MAX_APPS],
    pub num_apps: u32,
    pub qs_handle_list: [u16; IRDMA_MAX_USER_PRIORITY],
    pub mtu: u16,
    pub up2tc: [u8; IRDMA_MAX_USER_PRIORITY],
    pub dscp_map: [u8; DSCP_MAX],
    pub num_tc: u8,
    pub vsi_rel_bw: u8,
    pub vsi_prio_type: u8,
    pub mtu_changed:1: bool,
    pub tc_changed:1: bool,
    pub dscp_mode:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vsi_init_info {
    pub dev: *mut irdma_sc_dev,
    pub back_vsi: *mut c_void,
    pub params: *mut irdma_l2params,
    pub exception_lan_q: u16,
    pub pf_data_vsi_num: u16,
    pub vm_vf_type: irdma_vm_vf_type,
    pub vm_id: u16,
    pub tc_node): *mut irdma_ws_node,
    pub tc_node): *mut irdma_ws_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_vsi_stats_info {
    pub pestat: *mut irdma_vsi_pestat,
    pub fcn_id: u16,
    pub alloc_stats_inst: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_device_init_info {
    pub fpm_query_buf_pa: u64,
    pub fpm_commit_buf_pa: u64,
    pub fpm_query_buf: *mut __le64,
    pub fpm_commit_buf: *mut __le64,
    pub hw: *mut irdma_hw,
    pub bar0: *mut void __iomem,
    pub protocol_used: irdma_protocol_used,
    pub hmc_fn_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ceq_init_info {
    pub ceqe_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub ceqe_base: *mut u64,
    pub pbl_list: *mut c_void,
    pub elem_cnt: u32,
    pub ceq_id: u32,
    pub virtual_map:1: bool,
    pub tph_en:1: bool,
    pub itr_no_expire:1: bool,
    pub pbl_chunk_size: u8,
    pub tph_val: u8,
    pub vsi_idx: u16,
    pub first_pm_pbl_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_aeq_init_info {
    pub aeq_elem_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub aeqe_base: *mut u32,
    pub pbl_list: *mut c_void,
    pub elem_cnt: u32,
    pub virtual_map: bool,
    pub pbl_chunk_size: u8,
    pub first_pm_pbl_idx: u32,
    pub msix_idx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_ccq_init_info {
    pub cq_pa: u64,
    pub shadow_area_pa: u64,
    pub dev: *mut irdma_sc_dev,
    pub cq_base: *mut irdma_cqe,
    pub shadow_area: *mut __le64,
    pub pbl_list: *mut c_void,
    pub num_elem: u32,
    pub ceq_id: u32,
    pub shadow_read_threshold: u32,
    pub ceqe_mask:1: bool,
    pub ceq_id_valid:1: bool,
    pub avoid_mem_cflct:1: bool,
    pub virtual_map:1: bool,
    pub tph_en:1: bool,
    pub tph_val: u8,
    pub pbl_chunk_size: u8,
    pub first_pm_pbl_idx: u32,
    pub vsi: *mut irdma_sc_vsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_udp_offload_info {
    pub ipv4:1: bool,
    pub insert_vlan_tag:1: bool,
    pub ttl: u8,
    pub tos: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub dest_ip_addr: [u32; 4],
    pub snd_mss: u32,
    pub vlan_tag: u16,
    pub arp_idx: u16,
    pub flow_label: u32,
    pub udp_state: u8,
    pub psn_nxt: u32,
    pub lsn: u32,
    pub epsn: u32,
    pub psn_max: u32,
    pub psn_una: u32,
    pub local_ipaddr: [u32; 4],
    pub cwnd: u32,
    pub rexmit_thresh: u8,
    pub rnr_nak_thresh: u8,
    pub rnr_nak_tmr: u8,
    pub min_rnr_timer: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_roce_offload_info {
    pub p_key: u16,
    pub err_rq_idx: u16,
    pub qkey: u32,
    pub dest_qp: u32,
    pub roce_tver: u8,
    pub ack_credits: u8,
    pub err_rq_idx_valid: u8,
    pub pd_id: u32,
    pub ord_size: u16,
    pub ird_size: u16,
    pub is_qp1:1: bool,
    pub udprivcq_en:1: bool,
    pub dcqcn_en:1: bool,
    pub rcv_no_icrc:1: bool,
    pub wr_rdresp_en:1: bool,
    pub bind_en:1: bool,
    pub fast_reg_en:1: bool,
    pub priv_mode_en:1: bool,
    pub rd_en:1: bool,
    pub timely_en:1: bool,
    pub dctcp_en:1: bool,
    pub fw_cc_enable:1: bool,
    pub use_stats_inst:1: bool,
    pub local_ack_timeout: u8,
    pub t_high: u16,
    pub t_low: u16,
    pub last_byte_sent: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub rtomin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_iwarp_offload_info {
    pub rcv_mark_offset: u16,
    pub snd_mark_offset: u16,
    pub ddp_ver: u8,
    pub rdmap_ver: u8,
    pub iwarp_mode: u8,
    pub err_rq_idx: u16,
    pub pd_id: u32,
    pub ord_size: u16,
    pub ird_size: u16,
    pub ib_rd_en:1: bool,
    pub align_hdrs:1: bool,
    pub rcv_no_mpa_crc:1: bool,
    pub err_rq_idx_valid:1: bool,
    pub snd_mark_en:1: bool,
    pub rcv_mark_en:1: bool,
    pub wr_rdresp_en:1: bool,
    pub bind_en:1: bool,
    pub fast_reg_en:1: bool,
    pub priv_mode_en:1: bool,
    pub rd_en:1: bool,
    pub timely_en:1: bool,
    pub use_stats_inst:1: bool,
    pub ecn_en:1: bool,
    pub dctcp_en:1: bool,
    pub t_high: u16,
    pub t_low: u16,
    pub last_byte_sent: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub rtomin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_tcp_offload_info {
    pub ipv4:1: bool,
    pub no_nagle:1: bool,
    pub insert_vlan_tag:1: bool,
    pub time_stamp:1: bool,
    pub drop_ooo_seg:1: bool,
    pub avoid_stretch_ack:1: bool,
    pub wscale:1: bool,
    pub ignore_tcp_opt:1: bool,
    pub ignore_tcp_uns_opt:1: bool,
    pub cwnd_inc_limit: u8,
    pub dup_ack_thresh: u8,
    pub ttl: u8,
    pub src_mac_addr_idx: u8,
    pub tos: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub dest_ip_addr: [u32; 4],
// u32 dest_ip_addr0;
// u32 dest_ip_addr1;
// u32 dest_ip_addr2;
// u32 dest_ip_addr3;
    pub snd_mss: u32,
    pub syn_rst_handling: u16,
    pub vlan_tag: u16,
    pub arp_idx: u16,
    pub flow_label: u32,
    pub tcp_state: u8,
    pub snd_wscale: u8,
    pub rcv_wscale: u8,
    pub time_stamp_recent: u32,
    pub time_stamp_age: u32,
    pub snd_nxt: u32,
    pub snd_wnd: u32,
    pub rcv_nxt: u32,
    pub rcv_wnd: u32,
    pub snd_max: u32,
    pub snd_una: u32,
    pub srtt: u32,
    pub rtt_var: u32,
    pub ss_thresh: u32,
    pub cwnd: u32,
    pub snd_wl1: u32,
    pub snd_wl2: u32,
    pub max_snd_window: u32,
    pub rexmit_thresh: u8,
    pub local_ipaddr: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_host_ctx_info {
    pub qp_compl_ctx: u64,
    pub tcp_info: *mut irdma_tcp_offload_info,
    pub udp_info: *mut irdma_udp_offload_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_aeqe_info {
    pub compl_ctx: u64,
    pub qp_cq_id: u32,
    pub /: *mut *mut u32 def_info; / only valid for DEF_CMPL,
    pub ae_id: u16,
    pub wqe_idx: u16,
    pub tcp_state: u8,
    pub iwarp_state: u8,
    pub qp:1: bool,
    pub cq:1: bool,
    pub sq:1: bool,
    pub rq:1: bool,
    pub srq:1: bool,
    pub in_rdrsp_wr:1: bool,
    pub out_rdrsp:1: bool,
    pub aeqe_overflow:1: bool,
    pub err_rq_idx_valid:1: bool,
    pub q2_data_written: u8,
    pub ae_src: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_allocate_stag_info {
    pub total_len: u64,
    pub first_pm_pbl_idx: u64,
    pub chunk_size: u32,
    pub stag_idx: u32,
    pub page_size: u32,
    pub pd_id: u32,
    pub access_rights: u16,
    pub remote_access:1: bool,
    pub use_hmc_fcn_index:1: bool,
    pub use_pf_rid:1: bool,
    pub all_memory:1: bool,
    pub remote_atomics_en:1: bool,
    pub hmc_fcn_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mw_alloc_info {
    pub mw_stag_index: u32,
    pub page_size: u32,
    pub pd_id: u32,
    pub remote_access:1: bool,
    pub mw_wide:1: bool,
    pub mw1_bind_dont_vldt_key:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_reg_ns_stag_info {
    pub reg_addr_pa: u64,
    pub va: u64,
    pub total_len: u64,
    pub page_size: u32,
    pub chunk_size: u32,
    pub first_pm_pbl_index: u32,
    pub addr_type: irdma_addressing_type,
    pub stag_idx: irdma_stag_index,
    pub access_rights: u16,
    pub pd_id: u32,
    pub stag_key: irdma_stag_key,
    pub use_hmc_fcn_index:1: bool,
    pub hmc_fcn_index: u8,
    pub use_pf_rid:1: bool,
    pub all_memory:1: bool,
    pub remote_atomics_en:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_fast_reg_stag_info {
    pub wr_id: u64,
    pub reg_addr_pa: u64,
    pub fbo: u64,
    pub va: *mut c_void,
    pub total_len: u64,
    pub page_size: u32,
    pub chunk_size: u32,
    pub first_pm_pbl_index: u32,
    pub addr_type: irdma_addressing_type,
    pub stag_idx: irdma_stag_index,
    pub access_rights: u16,
    pub pd_id: u32,
    pub stag_key: irdma_stag_key,
    pub local_fence:1: bool,
    pub read_fence:1: bool,
    pub signaled:1: bool,
    pub use_hmc_fcn_index:1: bool,
    pub hmc_fcn_index: u8,
    pub use_pf_rid:1: bool,
    pub defer_flag:1: bool,
    pub remote_atomics_en:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_dealloc_stag_info {
    pub stag_idx: u32,
    pub pd_id: u32,
    pub mr:1: bool,
    pub dealloc_pbl:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_register_shared_stag {
    pub va: u64,
    pub addr_type: irdma_addressing_type,
    pub new_stag_idx: irdma_stag_index,
    pub parent_stag_idx: irdma_stag_index,
    pub access_rights: u32,
    pub pd_id: u32,
    pub page_size: u32,
    pub new_stag_key: irdma_stag_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_init_info {
    pub qp_uk_init_info: irdma_qp_uk_init_info,
    pub pd: *mut irdma_sc_pd,
    pub vsi: *mut irdma_sc_vsi,
    pub host_ctx: *mut __le64,
    pub q2: *mut u8,
    pub sq_pa: u64,
    pub rq_pa: u64,
    pub host_ctx_pa: u64,
    pub q2_pa: u64,
    pub shadow_area_pa: u64,
    pub sq_tph_val: u8,
    pub rq_tph_val: u8,
    pub sq_tph_en:1: bool,
    pub rq_tph_en:1: bool,
    pub rcv_tph_en:1: bool,
    pub xmit_tph_en:1: bool,
    pub virtual_map:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cq_init_info {
    pub dev: *mut irdma_sc_dev,
    pub cq_base_pa: u64,
    pub shadow_area_pa: u64,
    pub ceq_id: u32,
    pub shadow_read_threshold: u32,
    pub pbl_chunk_size: u8,
    pub first_pm_pbl_idx: u32,
    pub virtual_map:1: bool,
    pub ceqe_mask:1: bool,
    pub ceq_id_valid:1: bool,
    pub tph_en:1: bool,
    pub tph_val: u8,
    pub type: u8,
    pub cq_uk_init_info: irdma_cq_uk_init_info,
    pub vsi: *mut irdma_sc_vsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_upload_context_info {
    pub buf_pa: u64,
    pub qp_id: u32,
    pub qp_type: u8,
    pub freeze_qp:1: bool,
    pub raw_format:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_local_mac_entry_info {
    pub mac_addr: [u8; 6],
    pub entry_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_add_arp_cache_entry_info {
    pub mac_addr: [u8; ETH_ALEN],
    pub reach_max: u32,
    pub arp_index: u16,
    pub permanent: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_apbvt_info {
    pub port: u16,
    pub add: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qhash_table_info {
    pub vsi: *mut irdma_sc_vsi,
    pub manage: irdma_quad_hash_manage_type,
    pub entry_type: irdma_quad_entry_type,
    pub vlan_valid:1: bool,
    pub ipv4_valid:1: bool,
    pub mac_addr: [u8; ETH_ALEN],
    pub vlan_id: u16,
    pub user_pri: u8,
    pub qp_num: u32,
    pub dest_ip: [u32; 4],
    pub src_ip: [u32; 4],
    pub dest_port: u16,
    pub src_port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_manage_push_page_info {
    pub push_idx: u32,
    pub qs_handle: u16,
    pub free_page: u8,
    pub push_page_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_qp_flush_info {
    pub err_sq_idx: u32,
    pub err_rq_idx: u32,
    pub sq_minor_code: u16,
    pub sq_major_code: u16,
    pub rq_minor_code: u16,
    pub rq_major_code: u16,
    pub ae_code: u16,
    pub ae_src: u8,
    pub sq:1: bool,
    pub rq:1: bool,
    pub userflushcode:1: bool,
    pub generate_ae:1: bool,
    pub err_sq_idx_valid:1: bool,
    pub err_rq_idx_valid:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_gen_ae_info {
    pub ae_code: u16,
    pub ae_src: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_cqp_timeout {
    pub compl_cqp_cmds: u64,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_irq_ops {
    pub enable): *mut *mut *mut void (irdma_cfg_aeq)(struct irdma_sc_dev dev, u32 idx, bool,
    pub enable): bool,
    pub idx): *mut *mut *mut void (irdma_dis_irq)(struct irdma_sc_dev dev, u32,
    pub idx): *mut *mut *mut void (irdma_en_irq)(struct irdma_sc_dev dev, u32,
}

extern "C" {
    pub fn irdma_sc_ccq_arm(ccq: *mut irdma_sc_cq);
}
extern "C" {
    pub fn irdma_sc_ccq_destroy(ccq: *mut irdma_sc_cq, scratch: u64, post_sq: bool) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cceq_create(ceq: *mut irdma_sc_ceq, scratch: u64) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cceq_destroy_done(ceq: *mut irdma_sc_ceq) -> c_int;
}
extern "C" {
    pub fn irdma_sc_ceq_destroy(ceq: *mut irdma_sc_ceq, scratch: u64, post_sq: bool) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cleanup_ceqes(cq: *mut irdma_sc_cq, ceq: *mut irdma_sc_ceq);
}
extern "C" {
    pub fn irdma_sc_repost_aeq_entries(dev: *mut irdma_sc_dev, count: u32);
}
extern "C" {
    pub fn irdma_cfg_aeq(dev: *mut irdma_sc_dev, idx: u32, enable: bool);
}
extern "C" {
    pub fn irdma_sc_cqp_cleanup_handler(dev: *mut irdma_sc_dev) -> u64;
}
extern "C" {
    pub fn irdma_sc_cqp_create(cqp: *mut irdma_sc_cqp, maj_err: *mut u16, min_err: *mut u16) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cqp_destroy(cqp: *mut irdma_sc_cqp) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cqp_post_sq(cqp: *mut irdma_sc_cqp);
}
extern "C" {
    pub fn irdma_sc_qp_init(qp: *mut irdma_sc_qp, info: *mut irdma_qp_init_info) -> c_int;
}
extern "C" {
    pub fn irdma_sc_send_rtt(qp: *mut irdma_sc_qp, read: bool);
}
extern "C" {
    pub fn irdma_sc_cq_destroy(cq: *mut irdma_sc_cq, scratch: u64, post_sq: bool) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cq_init(cq: *mut irdma_sc_cq, info: *mut irdma_cq_init_info) -> c_int;
}
extern "C" {
    pub fn irdma_sc_cq_resize(cq: *mut irdma_sc_cq, info: *mut irdma_modify_cq_info);
}
extern "C" {
    pub fn sc_vsi_update_stats(vsi: *mut irdma_sc_vsi);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqp_info {
    pub qp: *mut irdma_sc_qp,
    pub info: irdma_create_qp_info,
    pub scratch: u64,
    pub qp_create: },
    pub qp: *mut irdma_sc_qp,
    pub info: irdma_modify_qp_info,
    pub scratch: u64,
    pub qp_modify: },
    pub qp: *mut irdma_sc_qp,
    pub scratch: u64,
    pub remove_hash_idx: bool,
    pub ignore_mw_bnd: bool,
    pub qp_destroy: },
    pub cq: *mut irdma_sc_cq,
    pub scratch: u64,
    pub check_overflow: bool,
    pub cq_create: },
    pub cq: *mut irdma_sc_cq,
    pub info: irdma_modify_cq_info,
    pub scratch: u64,
    pub cq_modify: },
    pub cq: *mut irdma_sc_cq,
    pub scratch: u64,
    pub cq_destroy: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_allocate_stag_info,
    pub scratch: u64,
    pub alloc_stag: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_mw_alloc_info,
    pub scratch: u64,
    pub mw_alloc: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_reg_ns_stag_info,
    pub scratch: u64,
    pub mr_reg_non_shared: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_dealloc_stag_info,
    pub scratch: u64,
    pub dealloc_stag: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_add_arp_cache_entry_info,
    pub scratch: u64,
    pub add_arp_cache_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub scratch: u64,
    pub arp_index: u16,
    pub del_arp_cache_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_local_mac_entry_info,
    pub scratch: u64,
    pub add_local_mac_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub scratch: u64,
    pub entry_idx: u8,
    pub ignore_ref_count: u8,
    pub del_local_mac_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub scratch: u64,
    pub alloc_local_mac_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_cqp_manage_push_page_info,
    pub scratch: u64,
    pub manage_push_page: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_upload_context_info,
    pub scratch: u64,
    pub qp_upload_context: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_hmc_fcn_info,
    pub scratch: u64,
    pub manage_hmc_pm: },
    pub ceq: *mut irdma_sc_ceq,
    pub scratch: u64,
    pub ceq_create: },
    pub ceq: *mut irdma_sc_ceq,
    pub scratch: u64,
    pub ceq_destroy: },
    pub aeq: *mut irdma_sc_aeq,
    pub scratch: u64,
    pub aeq_create: },
    pub aeq: *mut irdma_sc_aeq,
    pub scratch: u64,
    pub aeq_destroy: },
    pub qp: *mut irdma_sc_qp,
    pub info: irdma_qp_flush_info,
    pub scratch: u64,
    pub qp_flush_wqes: },
    pub qp: *mut irdma_sc_qp,
    pub info: irdma_gen_ae_info,
    pub scratch: u64,
    pub gen_ae: },
    pub cqp: *mut irdma_sc_cqp,
    pub fpm_val_va: *mut c_void,
    pub fpm_val_pa: u64,
    pub hmc_fn_id: u8,
    pub scratch: u64,
    pub query_fpm_val: },
    pub cqp: *mut irdma_sc_cqp,
    pub fpm_val_va: *mut c_void,
    pub fpm_val_pa: u64,
    pub hmc_fn_id: u8,
    pub scratch: u64,
    pub commit_fpm_val: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_apbvt_info,
    pub scratch: u64,
    pub manage_apbvt_entry: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_qhash_table_info,
    pub scratch: u64,
    pub manage_qhash_table_entry: },
    pub dev: *mut irdma_sc_dev,
    pub info: irdma_update_sds_info,
    pub scratch: u64,
    pub update_pe_sds: },
    pub cqp: *mut irdma_sc_cqp,
    pub qp: *mut irdma_sc_qp,
    pub scratch: u64,
    pub suspend_resume: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_ah_info,
    pub scratch: u64,
    pub ah_create: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_ah_info,
    pub scratch: u64,
    pub ah_destroy: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_mcast_grp_info,
    pub scratch: u64,
    pub mc_create: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_mcast_grp_info,
    pub scratch: u64,
    pub mc_destroy: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_mcast_grp_info,
    pub scratch: u64,
    pub mc_modify: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_stats_inst_info,
    pub scratch: u64,
    pub stats_manage: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_stats_gather_info,
    pub scratch: u64,
    pub stats_gather: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_ws_node_info,
    pub scratch: u64,
    pub ws_node: },
    pub cqp: *mut irdma_sc_cqp,
    pub info: irdma_up_info,
    pub scratch: u64,
    pub up_map: },
    pub cqp: *mut irdma_sc_cqp,
    pub query_buff_mem: irdma_dma_mem,
    pub scratch: u64,
    pub query_rdma: },
    pub srq: *mut irdma_sc_srq,
    pub scratch: u64,
    pub srq_create: },
    pub srq: *mut irdma_sc_srq,
    pub info: irdma_modify_srq_info,
    pub scratch: u64,
    pub srq_modify: },
    pub srq: *mut irdma_sc_srq,
    pub scratch: u64,
    pub srq_destroy: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqp_cmds_info {
    pub cqp_cmd_entry: list_head,
    pub cqp_cmd: u8,
    pub post_sq: u8,
    pub in: cqp_info,
}

//
// irdma_sc_cqp_get_next_send_wqe - get next wqe on cqp sq
// @cqp: struct for cqp hw
// @scratch: private data for CQP WQE
//
extern "C" {
    pub fn irdma_sc_cqp_get_next_send_wqe_idx(_arg: cqp, _arg: scratch, _arg: &wqe_idx) -> return;
}
