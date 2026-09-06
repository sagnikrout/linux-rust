//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnge/bnge_resc.h
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
pub struct bnge_hw_resc {
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_hw_rings {
    pub tx: u16,
    pub rx: u16,
    pub grp: u16,
    pub nq: u16,
    pub cmpl: u16,
    pub stat: u16,
    pub vnic: u16,
    pub rss_ctx: u16,
}

// "TXRX", 2 hypens, plus maximum integer
pub const BNGE_IRQ_NAME_EXTRA: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnge_irq {
    pub handler: irq_handler_t,
    pub vector: c_uint,
    pub requested:1: u8,
    pub have_cpumask:1: u8,
    pub BNGE_IRQ_NAME_EXTRA]: char name[IFNAMSIZ +,
    pub cpu_mask: cpumask_var_t,
}

extern "C" {
    pub fn bnge_reserve_rings(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_fix_rings_count(rx: *mut u16, tx: *mut u16, max: u16, shared: bool) -> c_int;
}
extern "C" {
    pub fn bnge_alloc_irqs(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_free_irqs(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_net_init_dflt_config(bd: *mut bnge_dev) -> c_int;
}
extern "C" {
    pub fn bnge_net_uninit_dflt_config(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_aux_init_dflt_config(bd: *mut bnge_dev);
}
extern "C" {
    pub fn bnge_get_rxfh_indir_size(bd: *mut bnge_dev) -> u32;
}
extern "C" {
    pub fn bnge_cal_nr_rss_ctxs(rx_rings: u16) -> c_int;
}
extern "C" {
    pub fn bnge_aux_has_enough_resources(bd: *mut bnge_dev) -> bool;
}
pub const BNGE_MAX_ROCE_MSIX: c_int = 64;
pub const BNGE_MIN_ROCE_CP_RINGS: c_int = 2;
pub const BNGE_MIN_ROCE_STAT_CTXS: c_int = 1;
