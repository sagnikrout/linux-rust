//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cgx.h
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
// Marvell OcteonTx2 CGX driver
//
// Copyright (C) 2018 Marvell.
//

// PCI device IDs
pub const PCI_DEVID_OCTEONTX2_CGX: c_uint = 0xA059;
// PCI BAR nos
pub const PCI_CFG_REG_BAR_NUM: c_int = 0;
pub const CGX_ID_MASK: c_uint = 0xF;
// Registers
pub const CGXX_CMRX_CFG: c_uint = 0x00;

pub const CGX_LMAC_TYPE_SHIFT: c_int = 40;
pub const CGX_LMAC_TYPE_MASK: c_uint = 0xF;
pub const CGXX_CMRX_INT: c_uint = 0x040;

pub const CGXX_CMR_GLOBAL_CONFIG: c_uint = 0x08;

pub const CGXX_CMRX_INT_ENA_W1S: c_uint = 0x058;
pub const CGXX_CMRX_RX_ID_MAP: c_uint = 0x060;
pub const CGXX_CMRX_RX_STAT0: c_uint = 0x070;
pub const CGXX_CMRX_RX_LOGL_XON: c_uint = 0x100;
pub const CGXX_CMRX_RX_LMACS: c_uint = 0x128;

pub const CGXX_CMRX_RX_DMAC_CAM1: c_uint = 0x400;

pub const CGXX_CMRX_TX_STAT0: c_uint = 0x700;
pub const CGXX_SCRATCH0_REG: c_uint = 0x1050;
pub const CGXX_SCRATCH1_REG: c_uint = 0x1058;
pub const CGX_CONST: c_uint = 0x2000;

pub const CGXX_SPUX_CONTROL1: c_uint = 0x10000;
pub const CGXX_SPUX_LNX_FEC_CORR_BLOCKS: c_uint = 0x10700;
pub const CGXX_SPUX_LNX_FEC_UNCORR_BLOCKS: c_uint = 0x10800;
pub const CGXX_SPUX_RSFEC_CORR: c_uint = 0x10088;
pub const CGXX_SPUX_RSFEC_UNCORR: c_uint = 0x10090;

pub const CGXX_GMP_PCS_MRX_CTL: c_uint = 0x30000;

pub const CGXX_SMUX_RX_FRM_CTL: c_uint = 0x20020;

pub const CGXX_GMP_GMI_RXX_FRM_CTL: c_uint = 0x38028;

pub const CGXX_SMUX_TX_CTL: c_uint = 0x20178;
pub const CGXX_SMUX_TX_PAUSE_PKT_TIME: c_uint = 0x20110;
pub const CGXX_SMUX_TX_PAUSE_PKT_INTERVAL: c_uint = 0x20120;
pub const CGXX_SMUX_SMAC: c_uint = 0x20108;
pub const CGXX_SMUX_CBFC_CTL: c_uint = 0x20218;

pub const CGXX_GMP_GMI_TX_PAUSE_PKT_TIME: c_uint = 0x38230;
pub const CGXX_GMP_GMI_TX_PAUSE_PKT_INTERVAL: c_uint = 0x38248;

pub const CGXX_CMR_RX_OVR_BP: c_uint = 0x130;

pub const DEFAULT_PAUSE_TIME: c_uint = 0x7FF;
pub const CGX_LMAC_FWI: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgx_nix_stat_type {
    NIX_STATS_RX,
    NIX_STATS_TX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LMAC_TYPE {
    LMAC_MODE_SGMII		= 0,
    LMAC_MODE_XAUI		= 1,
    LMAC_MODE_RXAUI		= 2,
    LMAC_MODE_10G_R		= 3,
    LMAC_MODE_40G_R		= 4,
    LMAC_MODE_QSGMII	= 6,
    LMAC_MODE_25G_R		= 7,
    LMAC_MODE_50G_R		= 8,
    LMAC_MODE_100G_R	= 9,
    LMAC_MODE_USXGMII	= 10,
    LMAC_MODE_USGMII	= 11,
    LMAC_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_link_event {
    pub link_uinfo: cgx_link_user_info,
    pub cgx_id: u8,
    pub lmac_id: u8,
}

//
// struct cgx_event_cb
// @notify_link_chg:	callback for link change notification
// @data:	data passed to callback function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx_event_cb {
    pub data): *mut *mut *mut int (notify_link_chg)(struct cgx_link_event event, void,
    pub data: *mut c_void,
}

extern "C" {
    pub fn cgx_get_cgxcnt_max() -> c_int;
}
extern "C" {
    pub fn cgx_get_cgxid(cgxd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cgx_get_lmac_cnt(cgxd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cgx_set_pkind(cgxd: *mut c_void, lmac_id: u8, pkind: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_get_pkind(cgxd: *mut c_void, lmac_id: u8, pkind: *mut c_int) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_evh_register(cb: *mut cgx_event_cb, cgxd: *mut c_void, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_evh_unregister(cgxd: *mut c_void, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_get_tx_stats(cgxd: *mut c_void, lmac_id: c_int, idx: c_int, tx_stat: *mut u64) -> c_int;
}
extern "C" {
    pub fn cgx_get_rx_stats(cgxd: *mut c_void, lmac_id: c_int, idx: c_int, rx_stat: *mut u64) -> c_int;
}
extern "C" {
    pub fn cgx_stats_reset(cgxd: *mut c_void, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_rx_tx_enable(cgxd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_tx_enable(cgxd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_addr_set(cgx_id: u8, lmac_id: u8, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_addr_reset(cgx_id: u8, lmac_id: u8) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_addr_get(cgx_id: u8, lmac_id: u8) -> u64;
}
extern "C" {
    pub fn cgx_lmac_addr_add(cgx_id: u8, lmac_id: u8, mac_addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_addr_del(cgx_id: u8, lmac_id: u8, index: u8) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_addr_max_entries_get(cgx_id: u8, lmac_id: u8) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_promisc_config(cgx_id: c_int, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn cgx_lmac_enadis_rx_pause_fwding(cgxd: *mut c_void, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn cgx_lmac_internal_loopback(cgxd: *mut c_void, lmac_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_linkup_start(cgxd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cgx_get_fwdata_base(base: *mut u64) -> c_int;
}
extern "C" {
    pub fn cgx_lmac_ptp_config(cgxd: *mut c_void, lmac_id: c_int, enable: bool);
}
extern "C" {
    pub fn cgx_lmac_get_p2x(cgx_id: c_int, lmac_id: c_int) -> u8;
}
extern "C" {
    pub fn cgx_set_fec(fec: u64, cgx_id: c_int, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_get_fec_stats(cgxd: *mut c_void, lmac_id: c_int, rsp: *mut cgx_fec_stats_rsp) -> c_int;
}
extern "C" {
    pub fn cgx_get_phy_fec_stats(cgxd: *mut c_void, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn cgx_features_get(cgxd: *mut c_void) -> u64;
}
extern "C" {
    pub fn cgx_get_nr_lmacs(cgxd: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cgx_get_lmacid(cgxd: *mut c_void, lmac_index: u8) -> u8;
}
extern "C" {
    pub fn cgx_get_lmac_bmap(cgxd: *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn cgx_lmac_write(cgx_id: c_int, lmac_id: c_int, offset: u64, val: u64);
}
extern "C" {
    pub fn cgx_lmac_read(cgx_id: c_int, lmac_id: c_int, offset: u64) -> u64;
}
extern "C" {
    pub fn cgx_lmac_addr_update(cgx_id: u8, lmac_id: u8, mac_addr: *mut u8, index: u8) -> c_int;
}
extern "C" {
    pub fn cgx_read_dmac_ctrl(cgxd: *mut c_void, lmac_id: c_int) -> u64;
}
extern "C" {
    pub fn cgx_read_dmac_entry(cgxd: *mut c_void, index: c_int) -> u64;
}
extern "C" {
    pub fn cgx_lmac_reset(cgxd: *mut c_void, lmac_id: c_int, pf_req_flr: u8) -> c_int;
}
extern "C" {
    pub fn cgx_get_fifo_len(cgxd: *mut c_void) -> u32;
}
