//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/irdma.h
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
// Copyright (c) 2017 - 2021 Intel Corporation

pub const IRDMA_INVALID_CQ_IDX: c_uint = 0xffffffff;
pub const IRDMA_Q_INVALID_IDX: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_dyn_idx_t {
    IRDMA_IDX_ITR0 = 0,
    IRDMA_IDX_ITR1 = 1,
    IRDMA_IDX_ITR2 = 2,
    IRDMA_IDX_NOITR = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_registers {
    IRDMA_CQPTAIL,
    IRDMA_CQPDB,
    IRDMA_CCQPSTATUS,
    IRDMA_CCQPHIGH,
    IRDMA_CCQPLOW,
    IRDMA_CQARM,
    IRDMA_CQACK,
    IRDMA_AEQALLOC,
    IRDMA_CQPERRCODES,
    IRDMA_WQEALLOC,
    IRDMA_GLINT_DYN_CTL,
    IRDMA_DB_ADDR_OFFSET,
    IRDMA_GLPCI_LBARCTRL,
    IRDMA_GLPE_CPUSTATUS0,
    IRDMA_GLPE_CPUSTATUS1,
    IRDMA_GLPE_CPUSTATUS2,
    IRDMA_PFINT_AEQCTL,
    IRDMA_GLINT_CEQCTL,
    IRDMA_VSIQF_PE_CTL1,
    IRDMA_PFHMC_PDINV,
    IRDMA_GLHMC_VFPDINV,
    IRDMA_GLPE_CRITERR,
    IRDMA_GLINT_RATE,
    IRDMA_PFHMC_ERRORINFO,
    IRDMA_PFHMC_ERRORDATA,
    IRDMA_MAX_REGS, /* Must be last entry */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_shifts {
    IRDMA_CCQPSTATUS_CCQP_DONE_S,
    IRDMA_CCQPSTATUS_CCQP_ERR_S,
    IRDMA_CQPSQ_STAG_PDID_S,
    IRDMA_CQPSQ_CQ_CEQID_S,
    IRDMA_CQPSQ_CQ_CQID_S,
    IRDMA_COMMIT_FPM_CQCNT_S,
    IRDMA_CQPSQ_UPESD_HMCFNID_S,
    IRDMA_MAX_SHIFTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_masks {
    IRDMA_CCQPSTATUS_CCQP_DONE_M,
    IRDMA_CCQPSTATUS_CCQP_ERR_M,
    IRDMA_CQPSQ_STAG_PDID_M,
    IRDMA_CQPSQ_CQ_CEQID_M,
    IRDMA_CQPSQ_CQ_CQID_M,
    IRDMA_COMMIT_FPM_CQCNT_M,
    IRDMA_CQPSQ_UPESD_HMCFNID_M,
    IRDMA_MAX_MASKS, /* Must be last entry */
}

pub const IRDMA_MAX_MGS_PER_CTX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mcast_grp_ctx_entry_info {
    pub qp_id: u32,
    pub valid_entry: bool,
    pub dest_port: u16,
    pub use_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_mcast_grp_info {
    pub dest_mac_addr: [u8; ETH_ALEN],
    pub vlan_id: u16,
    pub hmc_fcn_id: u16,
    pub ipv4_valid:1: bool,
    pub vlan_valid:1: bool,
    pub mg_id: u16,
    pub no_of_mgs: u32,
    pub dest_ip_addr: [u32; 4],
    pub qs_handle: u16,
    pub dma_mem_mc: irdma_dma_mem,
    pub mg_ctx_info: [irdma_mcast_grp_ctx_entry_info; IRDMA_MAX_MGS_PER_CTX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irdma_vers {
    IRDMA_GEN_RSVD,
    IRDMA_GEN_1,
    IRDMA_GEN_2,
    IRDMA_GEN_3,
    IRDMA_GEN_4,
    IRDMA_GEN_NEXT,
    IRDMA_GEN_MAX = IRDMA_GEN_NEXT-1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_uk_attrs {
    pub feature_flags: u64,
    pub max_hw_wq_frags: u32,
    pub max_hw_read_sges: u32,
    pub max_hw_inline: u32,
    pub max_hw_rq_quanta: u32,
    pub max_hw_wq_quanta: u32,
    pub min_hw_cq_size: u32,
    pub max_hw_cq_size: u32,
    pub max_hw_srq_quanta: u32,
    pub max_hw_sq_chunk: u16,
    pub min_hw_wq_size: u16,
    pub hw_rev: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irdma_hw_attrs {
    pub uk_attrs: irdma_uk_attrs,
    pub max_hw_outbound_msg_size: u64,
    pub max_hw_inbound_msg_size: u64,
    pub max_mr_size: u64,
    pub page_size_cap: u64,
    pub min_hw_qp_id: u32,
    pub min_hw_aeq_size: u32,
    pub max_hw_aeq_size: u32,
    pub min_hw_ceq_size: u32,
    pub max_hw_ceq_size: u32,
    pub max_hw_device_pages: u32,
    pub max_hw_vf_fpm_id: u32,
    pub first_hw_vf_fpm_id: u32,
    pub max_hw_ird: u32,
    pub max_hw_ord: u32,
    pub max_hw_wqes: u32,
    pub max_hw_pds: u32,
    pub max_hw_ena_vf_count: u32,
    pub max_qp_wr: u32,
    pub max_pe_ready_count: u32,
    pub max_done_count: u32,
    pub max_sleep_count: u32,
    pub max_cqp_compl_wait_time_ms: u32,
    pub min_hw_srq_id: u32,
    pub max_stat_inst: u16,
    pub max_stat_idx: u16,
}

extern "C" {
    pub fn i40iw_init_hw(dev: *mut irdma_sc_dev);
}
extern "C" {
    pub fn icrdma_init_hw(dev: *mut irdma_sc_dev);
}
extern "C" {
    pub fn ig3rdma_init_hw(dev: *mut irdma_sc_dev);
}
