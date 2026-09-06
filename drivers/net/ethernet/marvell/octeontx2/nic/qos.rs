//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/qos.h
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
// Copyright (C) 2023 Marvell.
//

pub const OTX2_QOS_MAX_LVL: c_int = 4;
pub const OTX2_QOS_MAX_PRIO: c_int = 7;
pub const OTX2_QOS_MAX_LEAF_NODES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qos_smq_operations {
    QOS_CFG_SQ,
    QOS_SMQ_FLUSH,
}

extern "C" {
    pub fn otx2_get_txschq_rate_regval(nic: *mut otx2_nic, maxrate: u64, burst: u32) -> u64;
}
extern "C" {
    pub fn otx2_setup_tc_htb(ndev: *mut net_device, htb: *mut tc_htb_qopt_offload) -> c_int;
}
extern "C" {
    pub fn otx2_qos_get_qid(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn otx2_qos_free_qid(pfvf: *mut otx2_nic, qidx: c_int);
}
extern "C" {
    pub fn otx2_qos_enable_sq(pfvf: *mut otx2_nic, qidx: c_int) -> c_int;
}
extern "C" {
    pub fn otx2_qos_disable_sq(pfvf: *mut otx2_nic, qidx: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_qos_cfg {
    pub schq: [u16; NIX_TXSCH_LVL_CNT],
    pub schq_contig: [u16; NIX_TXSCH_LVL_CNT],
    pub static_node_pos: [c_int; NIX_TXSCH_LVL_CNT],
    pub dwrr_node_pos: [c_int; NIX_TXSCH_LVL_CNT],
    pub schq_contig_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub schq_list: [u16; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
    pub schq_index_used: [bool; NIX_TXSCH_LVL_CNT][MAX_TXSCHQ_PER_FUNC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_qos {
    pub order_base_2(OTX2_QOS_MAX_LEAF_NODES)): DECLARE_HASHTABLE(qos_hlist,,
    pub /: *mut *mut mutex qos_lock; / child list lock,
    pub qid_to_sqmap: [u16; OTX2_QOS_MAX_LEAF_NODES],
    pub qos_tree: list_head,
    pub OTX2_QOS_MAX_LEAF_NODES): DECLARE_BITMAP(qos_sq_bmap,,
    pub maj_id: u16,
    pub defcls: u16,
    pub /: *mut *mut u8 link_cfg_lvl; / LINKX_CFG CSRs mapped to TL3 or TL2's index ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_qos_node {
    pub /: *mut *mut list_head list; / list management,
    pub child_list: list_head,
    pub child_schq_list: list_head,
    pub hlist: hlist_node,
    pub 1): DECLARE_BITMAP(prio_bmap, OTX2_QOS_MAX_PRIO +,
    pub /: *mut *mut *mut otx2_qos_node parent; / parent qos node,
    pub /: *mut *mut u64 rate; / htb params,
    pub ceil: u64,
    pub classid: u32,
    pub prio: u32,
    pub quantum: u32,
// hw txschq
    pub schq: u16,
    pub qid: u16,
    pub prio_anchor: u16,
    pub max_static_prio: u16,
    pub child_dwrr_cnt: u16,
    pub child_static_cnt: u16,
    pub child_dwrr_prio: u16,
    pub /: *mut *mut u16 txschq_idx; / txschq allocation index,
    pub level: u8,
    pub is_static: bool,
}
