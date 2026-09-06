//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bna.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

// Macros and constants

//
// input : _addr-> os dma addr in host endian format,
// output : _bna_dma_addr-> pointer to hw dma addr
//

//
// input : _bna_dma_addr-> pointer to hw dma addr
// output : _addr-> os dma addr in host endian format
//

pub const BNA_LARGE_PKT_SIZE: c_int = 1000;

// Inline functions

// Function prototypes
// BNA
// FW response handlers
extern "C" {
    pub fn bna_bfi_stats_clr_rsp(bna: *mut bna, msghdr: *mut bfi_msgq_mhdr);
}
// APIs for BNAD
extern "C" {
    pub fn bna_res_req(res_info: *mut bna_res_info);
}
extern "C" {
    pub fn bna_mod_res_req(bna: *mut bna, res_info: *mut bna_res_info);
}
extern "C" {
    pub fn bna_mod_init(bna: *mut bna, res_info: *mut bna_res_info);
}
extern "C" {
    pub fn bna_uninit(bna: *mut bna);
}
extern "C" {
    pub fn bna_num_txq_set(bna: *mut bna, num_txq: c_int) -> c_int;
}
extern "C" {
    pub fn bna_num_rxp_set(bna: *mut bna, num_rxp: c_int) -> c_int;
}
extern "C" {
    pub fn bna_hw_stats_get(bna: *mut bna);
}
// APIs for RxF
// MBOX
// API for BNAD
extern "C" {
    pub fn bna_mbox_handler(bna: *mut bna, intr_status: u32);
}
// ETHPORT
// Callbacks for RX
extern "C" {
    pub fn bna_ethport_cb_rx_started(ethport: *mut bna_ethport);
}
extern "C" {
    pub fn bna_ethport_cb_rx_stopped(ethport: *mut bna_ethport);
}
// TX MODULE AND TX
// FW response handelrs
extern "C" {
    pub fn bna_bfi_bw_update_aen(tx_mod: *mut bna_tx_mod);
}
// APIs for BNA
extern "C" {
    pub fn bna_tx_mod_uninit(tx_mod: *mut bna_tx_mod);
}
// APIs for ENET
extern "C" {
    pub fn bna_tx_mod_start(tx_mod: *mut bna_tx_mod, type: bna_tx_type);
}
extern "C" {
    pub fn bna_tx_mod_stop(tx_mod: *mut bna_tx_mod, type: bna_tx_type);
}
extern "C" {
    pub fn bna_tx_mod_fail(tx_mod: *mut bna_tx_mod);
}
// APIs for BNAD
extern "C" {
    pub fn bna_tx_destroy(tx: *mut bna_tx);
}
extern "C" {
    pub fn bna_tx_enable(tx: *mut bna_tx);
}
extern "C" {
    pub fn bna_tx_cleanup_complete(tx: *mut bna_tx);
}
extern "C" {
    pub fn bna_tx_coalescing_timeo_set(tx: *mut bna_tx, coalescing_timeo: c_int);
}
// RX MODULE, RX, RXF
// FW response handlers
extern "C" {
    pub fn bna_bfi_rxf_cfg_rsp(rxf: *mut bna_rxf, msghdr: *mut bfi_msgq_mhdr);
}
// APIs for BNA
extern "C" {
    pub fn bna_rx_mod_uninit(rx_mod: *mut bna_rx_mod);
}
// APIs for ENET
extern "C" {
    pub fn bna_rx_mod_start(rx_mod: *mut bna_rx_mod, type: bna_rx_type);
}
extern "C" {
    pub fn bna_rx_mod_stop(rx_mod: *mut bna_rx_mod, type: bna_rx_type);
}
extern "C" {
    pub fn bna_rx_mod_fail(rx_mod: *mut bna_rx_mod);
}
// APIs for BNAD
extern "C" {
    pub fn bna_rx_destroy(rx: *mut bna_rx);
}
extern "C" {
    pub fn bna_rx_enable(rx: *mut bna_rx);
}
extern "C" {
    pub fn bna_rx_cleanup_complete(rx: *mut bna_rx);
}
extern "C" {
    pub fn bna_rx_coalescing_timeo_set(rx: *mut bna_rx, coalescing_timeo: c_int);
}
extern "C" {
    pub fn bna_rx_dim_reconfig(bna: *mut bna, vector[][BNA_BIAS_T_MAX]: u32);
}
extern "C" {
    pub fn bna_rx_dim_update(ccb: *mut bna_ccb);
}
extern "C" {
    pub fn bna_rx_ucast_set(rx: *mut bna_rx, ucmac: *const u8) -> bna_cb_status;
}
extern "C" {
    pub fn bna_rx_vlan_add(rx: *mut bna_rx, vlan_id: c_int);
}
extern "C" {
    pub fn bna_rx_vlan_del(rx: *mut bna_rx, vlan_id: c_int);
}
extern "C" {
    pub fn bna_rx_vlanfilter_enable(rx: *mut bna_rx);
}
extern "C" {
    pub fn bna_rx_vlan_strip_enable(rx: *mut bna_rx);
}
extern "C" {
    pub fn bna_rx_vlan_strip_disable(rx: *mut bna_rx);
}
// ENET
// API for RX
extern "C" {
    pub fn bna_enet_mtu_get(enet: *mut bna_enet) -> c_int;
}
// Callbacks for TX, RX
extern "C" {
    pub fn bna_enet_cb_tx_stopped(enet: *mut bna_enet);
}
extern "C" {
    pub fn bna_enet_cb_rx_stopped(enet: *mut bna_enet);
}
// API for BNAD
extern "C" {
    pub fn bna_enet_enable(enet: *mut bna_enet);
}
extern "C" {
    pub fn bna_enet_perm_mac_get(enet: *mut bna_enet, mac: *mut u8);
}
// IOCETH
// APIs for BNAD
extern "C" {
    pub fn bna_ioceth_enable(ioceth: *mut bna_ioceth);
}
// BNAD
// Callbacks for ENET
// Callbacks for IOCETH
extern "C" {
    pub fn bnad_cb_ioceth_ready(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_cb_ioceth_failed(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_cb_ioceth_disabled(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_cb_mbox_intr_enable(bnad: *mut bnad);
}
extern "C" {
    pub fn bnad_cb_mbox_intr_disable(bnad: *mut bnad);
}
// Callbacks for BNA
