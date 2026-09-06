//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge.h
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

pub const DRV_VER_MAJ: c_int = 1;
pub const DRV_VER_MIN: c_int = 15;
pub const DRV_VER_UPD: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_idx {
    BCM57708,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_auxr_priv {
    pub aux_dev: auxiliary_device,
    pub auxr_dev: *mut bnge_auxr_dev,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_pf_info {
    pub fw_fid: u16,
    pub port_id: u16,
    pub mac_addr: [u8; ETH_ALEN],
}

pub const BNGE_MAX_QUEUE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_queue_info {
    pub queue_id: u8,
    pub queue_profile: u8,
}

pub const BNGE_PHY_FLAGS2_SHIFT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_dev {
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub dsn: u64,
pub const BNGE_VPD_FLD_LEN: c_int = 32;
    pub board_partno: [c_char; BNGE_VPD_FLD_LEN],
    pub board_serialno: [c_char; BNGE_VPD_FLD_LEN],
    pub bar0: *mut void __iomem,
    pub bar1: *mut void __iomem,
    pub chip_num: u16,
    pub chip_rev: u8,

// ensure atomic 64-bit doorbell writes on 32-bit systems.
    pub db_lock: spinlock_t,

    pub /: *mut *mut int db_offset; / db_offset within db_size,
    pub db_size: c_int,
// HWRM members
    pub hwrm_cmd_seq: u16,
    pub hwrm_cmd_kong_seq: u16,
    pub hwrm_dma_pool: *mut dma_pool,
    pub hwrm_pending_list: hlist_head,
    pub hwrm_max_req_len: u16,
    pub hwrm_max_ext_req_len: u16,
    pub hwrm_cmd_timeout: c_uint,
    pub hwrm_cmd_max_timeout: c_uint,
    pub /: *mut *mut mutex hwrm_cmd_lock; / serialize hwrm messages,
    pub ver_resp: hwrm_ver_get_output,
pub const FW_VER_STR_LEN: c_int = 32;
    pub fw_ver_str: [c_char; FW_VER_STR_LEN],
    pub hwrm_ver_supp: [c_char; FW_VER_STR_LEN],
    pub nvm_cfg_ver: [c_char; FW_VER_STR_LEN],
    pub fw_ver_code: u64,

    pub pf: bnge_pf_info,
    pub state: c_ulong,
pub const BNGE_STATE_DRV_REGISTERED: c_int = 0;
pub const BNGE_STATE_OPEN: c_int = 1;
    pub fw_cap: u64,
// Backing stores
    pub ctx: *mut bnge_ctx_mem_info,
    pub flags: u64,
    pub hw_resc: bnge_hw_resc,
    pub tso_max_segs: u16,
    pub max_fltr: c_int,
pub const BNGE_L2_FLTR_MAX_FLTR: c_int = 1024;
    pub rss_indir_tbl: *mut u32,
pub const BNGE_RSS_TABLE_ENTRIES: c_int = 64;

pub const BNGE_RSS_TABLE_MAX_TBL: c_int = 8;

    pub rss_indir_tbl_entries: u16,
    pub rss_cap: u32,
    pub rss_hash_cfg: u32,
    pub rx_nr_rings: u16,
    pub tx_nr_rings: u16,
    pub tx_nr_rings_per_tc: u16,
// Number of NQs
    pub nq_nr_rings: u16,
// Aux device resources
    pub aux_num_msix: u16,
    pub aux_num_stat_ctxs: u16,
    pub max_mtu: u16,
pub const BNGE_MAX_MTU: c_int = 9500;
    pub hw_ring_stats_size: u16,
pub const BNGE_NUM_RX_RING_STATS: c_int = 8;
pub const BNGE_NUM_TX_RING_STATS: c_int = 8;
pub const BNGE_NUM_TPA_RING_STATS: c_int = 6;

    pub max_tpa_v2: u16,

    pub num_tc: u8,
    pub max_tc: u8,
    pub /: *mut *mut u8 max_lltc; / lossless TCs,
    pub q_info: [bnge_queue_info; BNGE_MAX_QUEUE],
    pub tc_to_qidx: [u8; BNGE_MAX_QUEUE],
    pub q_ids: [u8; BNGE_MAX_QUEUE],
    pub max_q: u8,
    pub port_count: u8,
    pub irq_tbl: *mut bnge_irq,
    pub irqs_acquired: u16,
    pub aux_priv: *mut bnge_auxr_priv,
    pub auxr_dev: *mut bnge_auxr_dev,
    pub link_info: bnge_link_info,
// Copied from flags and flags2 in hwrm_port_phy_qcaps_output
    pub phy_flags: u32,
}

// For TX and RX ring doorbells
extern "C" {
    pub fn bnge_aux_registered(bd: *mut bnge_dev) -> bool;
}
extern "C" {
    pub fn bnge_aux_get_msix(bd: *mut bnge_dev) -> u16;
}
