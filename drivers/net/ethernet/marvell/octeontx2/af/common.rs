//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/common.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2018 Marvell.
//

// Admin queue info
// Since we intend to add only one instruction at a time,
// keep queue size to it's minimum.
//

// HW head & tail pointer mask
pub const AQ_PTR_MASK: c_uint = 0xFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmem {
    pub base: *mut c_void,
    pub iova: dma_addr_t,
    pub alloc_sz: c_int,
    pub entry_sz: u32,
    pub align: u8,
    pub qsize: u32,
}

// q = devm_kzalloc(dev, sizeof(*qmem), GFP_KERNEL);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct admin_queue {
    pub inst: *mut qmem,
    pub res: *mut qmem,
    pub /: *mut *mut spinlock_t lock; / Serialize inst enqueue from PFs,
}

// NPA aura count
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_aura_sz {
    NPA_AURA_SZ_0,
    NPA_AURA_SZ_128,
    NPA_AURA_SZ_256,
    NPA_AURA_SZ_512,
    NPA_AURA_SZ_1K,
    NPA_AURA_SZ_2K,
    NPA_AURA_SZ_4K,
    NPA_AURA_SZ_8K,
    NPA_AURA_SZ_16K,
    NPA_AURA_SZ_32K,
    NPA_AURA_SZ_64K,
    NPA_AURA_SZ_128K,
    NPA_AURA_SZ_256K,
    NPA_AURA_SZ_512K,
    NPA_AURA_SZ_1M,
    NPA_AURA_SZ_MAX,
}

// NPA AQ result structure for init/read/write of aura HW contexts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_aura_res {
    pub res: npa_aq_res_s,
    pub aura_ctx: npa_aura_s,
    pub ctx_mask: npa_aura_s,
}

// NPA AQ result structure for init/read/write of pool HW contexts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_pool_res {
    pub res: npa_aq_res_s,
    pub pool_ctx: npa_pool_s,
    pub ctx_mask: npa_pool_s,
}

// NIX Transmit schedulers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_scheduler {
    NIX_TXSCH_LVL_SMQ = 0x0,
    NIX_TXSCH_LVL_MDQ = 0x0,
    NIX_TXSCH_LVL_TL4 = 0x1,
    NIX_TXSCH_LVL_TL3 = 0x2,
    NIX_TXSCH_LVL_TL2 = 0x3,
    NIX_TXSCH_LVL_TL1 = 0x4,
    NIX_TXSCH_LVL_CNT = 0x5,
}

// Don't change the order as on CN10K (except CN10KB)
// SMQX_CFG[SDP] value should be 1 for SDP flows.
//
pub const SMQ_LINK_TYPE_RPM: c_int = 0;
pub const SMQ_LINK_TYPE_SDP: c_int = 1;
pub const SMQ_LINK_TYPE_LBK: c_int = 2;
// Min/Max packet sizes, excluding FCS
pub const NIC_HW_MIN_FRS: c_int = 40;
pub const NIC_HW_MAX_FRS: c_int = 9212;
pub const SDP_HW_MAX_FRS: c_int = 65535;
pub const SDP_HW_MIN_FRS: c_int = 16;

pub const SDP_LINK_CREDIT: c_uint = 0x320202;
// NIX RX action operation

// Use the RX action set in the default unicast entry

// NIX TX action operation

// Default interfaces are NIX0_RX and NIX0_TX

pub const NIX_INTF_TYPE_CGX: c_int = 0;
pub const NIX_INTF_TYPE_LBK: c_int = 1;
pub const NIX_INTF_TYPE_SDP: c_int = 2;
pub const MAX_LMAC_PKIND: c_int = 12;

pub const NIX_CHAN_SDP_NUM_CHANS: c_int = 256;

// The mask is to extract lower 10-bits of channel number
// which CPT will pass to X2P.
//

// NIX LSO format indices.
// As of now TSO is the only one using, so statically assigning indices.
//
pub const NIX_LSO_FORMAT_IDX_TSOV4: c_int = 0;
pub const NIX_LSO_FORMAT_IDX_TSOV6: c_int = 1;
// RSS info
pub const MAX_RSS_GROUPS: c_int = 8;
// Group 0 has to be used in default pkt forwarding MCAM entries
// reserved for NIXLFs. Groups 1-7 can be used for RSS for ntuple
// filters.
//
pub const DEFAULT_RSS_CONTEXT_GROUP: c_int = 0;

// NDC info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ndc_idx_e {
    NIX0_RX = 0x0,
    NIX0_TX = 0x1,
    NPA0_U  = 0x2,
    NIX1_RX = 0x4,
    NIX1_TX = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ndc_ctype_e {
    CACHING = 0x0,
    BYPASS = 0x1,
}

pub const NDC_MAX_PORT: c_int = 6;
pub const NDC_READ_TRANS: c_int = 0;
pub const NDC_WRITE_TRANS: c_int = 1;
