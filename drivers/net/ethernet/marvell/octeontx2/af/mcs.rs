//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/mcs.h
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
// Marvell CN10K MCS driver
//
// Copyright (C) 2022 Marvell.
//

pub const PCI_DEVID_CN10K_MCS: c_uint = 0xA096;

pub const MCS_ID_MASK: c_uint = 0x7;
pub const MCS_MAX_PFS: c_int = 128;
pub const MCS_PORT_MODE_MASK: c_uint = 0x3;
pub const MCS_PORT_FIFO_SKID_MASK: c_uint = 0x3F;
pub const MCS_MAX_CUSTOM_TAGS: c_uint = 0x8;
pub const MCS_CTRLPKT_ETYPE_RULE_MAX: c_int = 8;
pub const MCS_CTRLPKT_DA_RULE_MAX: c_int = 8;
pub const MCS_CTRLPKT_DA_RANGE_RULE_MAX: c_int = 4;
pub const MCS_CTRLPKT_COMBO_RULE_MAX: c_int = 4;
pub const MCS_CTRLPKT_MAC_RULE_MAX: c_int = 1;

pub const MCS_CTRLPKT_ETYPE_RULE_OFFSET: c_int = 0;
pub const MCS_CTRLPKT_DA_RULE_OFFSET: c_int = 8;
pub const MCS_CTRLPKT_DA_RANGE_RULE_OFFSET: c_int = 16;
pub const MCS_CTRLPKT_COMBO_RULE_OFFSET: c_int = 20;
pub const MCS_CTRLPKT_MAC_EN_RULE_OFFSET: c_int = 24;
// Reserved resources for default bypass entry
pub const MCS_RSRC_RSVD_CNT: c_int = 1;
// MCS Interrupt Vector
pub const MCS_CNF10KB_INT_VEC_IP: c_uint = 0x13;
pub const MCS_CN10KB_INT_VEC_IP: c_uint = 0x53;

pub const MCS_BBE_INT_MASK: c_uint = 0xFFULL;

pub const MCS_PAB_INT_MASK: c_uint = 0xFULL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_pfvf {
    pub /: *mut *mut u64 intr_mask; / Enabled Interrupt mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_intr_event {
    pub pcifunc: u16,
    pub intr_mask: u64,
    pub sa_id: u64,
    pub mcs_id: u8,
    pub lmac_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_intrq_entry {
    pub node: list_head,
    pub intr_event: mcs_intr_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secy_mem_map {
    pub flow_id: u8,
    pub secy: u8,
    pub ctrl_pkt: u8,
    pub sc: u8,
    pub sci: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_rsrc_map {
    pub flowid2pf_map: *mut u16,
    pub secy2pf_map: *mut u16,
    pub sc2pf_map: *mut u16,
    pub sa2pf_map: *mut u16,
    pub secy*/: *mut *mut *mut u16 flowid2secy_map; / bitmap flowid mapped to,
    pub ctrlpktrule2pf_map: *mut u16,
    pub flow_ids: rsrc_bmap,
    pub secy: rsrc_bmap,
    pub sc: rsrc_bmap,
    pub sa: rsrc_bmap,
    pub ctrlpktrule: rsrc_bmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwinfo {
    pub tcam_entries: u8,
    pub secy_entries: u8,
    pub sc_entries: u8,
    pub sa_entries: u16,
    pub mcs_x2p_intf: u8,
    pub lmac_cnt: u8,
    pub mcs_blks: u8,
    pub /: *mut *mut unsigned long lmac_bmap; / bitmap of enabled mcs lmac,
    pub ip_vec: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs {
    pub reg_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub hw: *mut hwinfo,
    pub tx: mcs_rsrc_map,
    pub rx: mcs_rsrc_map,
    pub /: *mut *mut u16 pf_map[MCS_MAX_PFS]; / List of PCIFUNC mapped to MCS,
    pub mcs_id: u8,
    pub mcs_ops: *mut mcs_ops,
    pub mcs_list: list_head,
// Lock for mcs stats
    pub stats_lock: mutex,
    pub pf: *mut mcs_pfvf,
    pub vf: *mut mcs_pfvf,
    pub num_vec: u16,
    pub rvu: *mut c_void,
    pub tx_sa_active: *mut u16,
    pub bypass: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_ops {
    pub mcs): *mut *mut void (mcs_set_hw_capabilities)(struct mcs,
    pub mcs): *mut *mut void (mcs_parser_cfg)(struct mcs,
    pub map): *mut *mut *mut void (mcs_tx_sa_mem_map_write)(struct mcs mcs, struct mcs_tx_sc_sa_map,
    pub map): *mut *mut *mut void (mcs_rx_sa_mem_map_write)(struct mcs mcs, struct mcs_rx_sc_sa_map,
    pub dir): *mut *mut *mut *mut void (mcs_flowid_secy_map)(struct mcs mcs, struct secy_mem_map map, int,
    pub dir): *mut *mut *mut void (mcs_bbe_intr_handler)(struct mcs mcs, u64 intr, enum mcs_direction,
    pub dir): *mut *mut *mut void (mcs_pab_intr_handler)(struct mcs mcs, u64 intr, enum mcs_direction,
}

extern "C" {
    pub fn readq(offset: mcs->reg_base +) -> return;
}
// MCS APIs
extern "C" {
    pub fn mcs_get_blkcnt() -> c_int;
}
extern "C" {
    pub fn mcs_set_lmac_channels(mcs_id: c_int, base: u16) -> c_int;
}
extern "C" {
    pub fn mcs_alloc_rsrc(rsrc: *mut rsrc_bmap, pf_map: *mut u16, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn mcs_free_rsrc(rsrc: *mut rsrc_bmap, pf_map: *mut u16, rsrc_id: c_int, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn mcs_free_all_rsrc(mcs: *mut mcs, dir: c_int, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn mcs_clear_secy_plcy(mcs: *mut mcs, secy_id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_ena_dis_flowid_entry(mcs: *mut mcs, id: c_int, dir: c_int, ena: c_int);
}
extern "C" {
    pub fn mcs_ena_dis_sc_cam_entry(mcs: *mut mcs, id: c_int, ena: c_int);
}
extern "C" {
    pub fn mcs_flowid_entry_write(mcs: *mut mcs, data: *mut u64, mask: *mut u64, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_secy_plcy_write(mcs: *mut mcs, plcy: u64, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_rx_sc_cam_write(mcs: *mut mcs, sci: u64, secy: u64, sc_id: c_int);
}
extern "C" {
    pub fn mcs_sa_plcy_write(mcs: *mut mcs, plcy: *mut u64, sa: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_map_sc_to_sa(mcs: *mut mcs, sa_map: *mut u64, sc: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_pn_table_write(mcs: *mut mcs, pn_id: u8, next_pn: u64, dir: u8);
}
extern "C" {
    pub fn mcs_tx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_tx_sc_sa_map);
}
extern "C" {
    pub fn mcs_flowid_secy_map(mcs: *mut mcs, map: *mut secy_mem_map, dir: c_int);
}
extern "C" {
    pub fn mcs_rx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_rx_sc_sa_map);
}
extern "C" {
    pub fn mcs_pn_threshold_set(mcs: *mut mcs, pn: *mut mcs_set_pn_threshold);
}
extern "C" {
    pub fn mcs_install_flowid_bypass_entry(mcs: *mut mcs) -> c_int;
}
extern "C" {
    pub fn mcs_set_lmac_mode(mcs: *mut mcs, lmac_id: c_int, mode: u8);
}
extern "C" {
    pub fn mcs_reset_port(mcs: *mut mcs, port_id: u8, reset: u8);
}
extern "C" {
    pub fn mcs_set_port_cfg(mcs: *mut mcs, req: *mut mcs_port_cfg_set_req);
}
extern "C" {
    pub fn mcs_alloc_ctrlpktrule(rsrc: *mut rsrc_bmap, pf_map: *mut u16, offset: u16, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn mcs_free_ctrlpktrule(mcs: *mut mcs, req: *mut mcs_free_ctrl_pkt_rule_req) -> c_int;
}
extern "C" {
    pub fn mcs_ctrlpktrule_write(mcs: *mut mcs, req: *mut mcs_ctrl_pkt_rule_write_req) -> c_int;
}
extern "C" {
    pub fn is_mcs_bypass(mcs_id: c_int) -> bool;
}
// CN10K-B APIs
extern "C" {
    pub fn cn10kb_mcs_set_hw_capabilities(mcs: *mut mcs);
}
extern "C" {
    pub fn cn10kb_mcs_tx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_tx_sc_sa_map);
}
extern "C" {
    pub fn cn10kb_mcs_flowid_secy_map(mcs: *mut mcs, map: *mut secy_mem_map, dir: c_int);
}
extern "C" {
    pub fn cn10kb_mcs_rx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_rx_sc_sa_map);
}
extern "C" {
    pub fn cn10kb_mcs_parser_cfg(mcs: *mut mcs);
}
extern "C" {
    pub fn cn10kb_mcs_pab_intr_handler(mcs: *mut mcs, intr: u64, dir: mcs_direction);
}
extern "C" {
    pub fn cn10kb_mcs_bbe_intr_handler(mcs: *mut mcs, intr: u64, dir: mcs_direction);
}
// CNF10K-B APIs
extern "C" {
    pub fn cnf10kb_mcs_set_hw_capabilities(mcs: *mut mcs);
}
extern "C" {
    pub fn cnf10kb_mcs_tx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_tx_sc_sa_map);
}
extern "C" {
    pub fn cnf10kb_mcs_flowid_secy_map(mcs: *mut mcs, map: *mut secy_mem_map, dir: c_int);
}
extern "C" {
    pub fn cnf10kb_mcs_rx_sa_mem_map_write(mcs: *mut mcs, map: *mut mcs_rx_sc_sa_map);
}
extern "C" {
    pub fn cnf10kb_mcs_parser_cfg(mcs: *mut mcs);
}
extern "C" {
    pub fn cnf10kb_mcs_tx_pn_thresh_reached_handler(mcs: *mut mcs);
}
extern "C" {
    pub fn cnf10kb_mcs_tx_pn_wrapped_handler(mcs: *mut mcs);
}
extern "C" {
    pub fn cnf10kb_mcs_bbe_intr_handler(mcs: *mut mcs, intr: u64, dir: mcs_direction);
}
extern "C" {
    pub fn cnf10kb_mcs_pab_intr_handler(mcs: *mut mcs, intr: u64, dir: mcs_direction);
}
// Stats APIs
extern "C" {
    pub fn mcs_get_sc_stats(mcs: *mut mcs, stats: *mut mcs_sc_stats, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_get_sa_stats(mcs: *mut mcs, stats: *mut mcs_sa_stats, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_get_port_stats(mcs: *mut mcs, stats: *mut mcs_port_stats, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_get_flowid_stats(mcs: *mut mcs, stats: *mut mcs_flowid_stats, id: c_int, dir: c_int);
}
extern "C" {
    pub fn mcs_get_rx_secy_stats(mcs: *mut mcs, stats: *mut mcs_secy_stats, id: c_int);
}
extern "C" {
    pub fn mcs_get_tx_secy_stats(mcs: *mut mcs, stats: *mut mcs_secy_stats, id: c_int);
}
extern "C" {
    pub fn mcs_clear_stats(mcs: *mut mcs, type: u8, id: u8, dir: c_int);
}
extern "C" {
    pub fn mcs_clear_all_stats(mcs: *mut mcs, pcifunc: u16, dir: c_int) -> c_int;
}
extern "C" {
    pub fn mcs_set_force_clk_en(mcs: *mut mcs, set: bool) -> c_int;
}
extern "C" {
    pub fn mcs_add_intr_wq_entry(mcs: *mut mcs, event: *mut mcs_intr_event) -> c_int;
}
