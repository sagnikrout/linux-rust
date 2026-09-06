//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/k3-udma-glue.h
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
//
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_udma_glue_tx_channel_cfg {
    pub tx_cfg: k3_ring_cfg,
    pub txcq_cfg: k3_ring_cfg,
    pub tx_pause_on_err: bool,
    pub tx_filt_einfo: bool,
    pub tx_filt_pswords: bool,
    pub tx_supr_tdpkt: bool,
    pub swdata_size: u32,
}

extern "C" {
    pub fn k3_udma_glue_release_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
}
extern "C" {
    pub fn k3_udma_glue_enable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel) -> c_int;
}
extern "C" {
    pub fn k3_udma_glue_disable_tx_chn(tx_chn: *mut k3_udma_glue_tx_channel);
}
extern "C" {
    pub fn k3_udma_glue_tx_get_hdesc_size(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
}
extern "C" {
    pub fn k3_udma_glue_tx_get_txcq_id(tx_chn: *mut k3_udma_glue_tx_channel) -> u32;
}
extern "C" {
    pub fn k3_udma_glue_tx_get_irq(tx_chn: *mut k3_udma_glue_tx_channel) -> c_int;
}
//
// k3_udma_glue_rx_flow_cfg - UDMA RX flow cfg
//
// @rx_cfg:		RX ring configuration
// @rxfdq_cfg:		RX free Host PD ring configuration
// @ring_rxq_id:	RX ring id (or -1 for any)
// @ring_rxfdq0_id:	RX free Host PD ring (FDQ) if (or -1 for any)
// @rx_error_handling:	Rx Error Handling Mode (0 - drop, 1 - re-try)
// @src_tag_lo_sel:	Rx Source Tag Low Byte Selector in Host PD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_udma_glue_rx_flow_cfg {
    pub rx_cfg: k3_ring_cfg,
    pub rxfdq_cfg: k3_ring_cfg,
    pub ring_rxq_id: c_int,
    pub ring_rxfdq0_id: c_int,
    pub rx_error_handling: bool,
    pub src_tag_lo_sel: c_int,
}

//
// k3_udma_glue_rx_channel_cfg - UDMA RX channel cfg
//
// @psdata_size:	SW Data is present in Host PD of @swdata_size bytes
// @flow_id_base:	first flow_id used by channel.
// if @flow_id_base = -1 - range of GP rflows will be
// allocated dynamically.
// @flow_id_num:	number of RX flows used by channel
// @flow_id_use_rxchan_id:	use RX channel id as flow id,
// used only if @flow_id_num = 1
// @remote		indication that RX channel is remote - some remote CPU
// core owns and control the RX channel. Linux Host only
// allowed to attach and configure RX Flow within RX
// channel. if set - not RX channel operation will be
// performed by K3 NAVSS DMA glue interface.
// @def_flow_cfg	default RX flow configuration,
// used only if @flow_id_num = 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k3_udma_glue_rx_channel_cfg {
    pub swdata_size: u32,
    pub flow_id_base: c_int,
    pub flow_id_num: c_int,
    pub flow_id_use_rxchan_id: bool,
    pub remote: bool,
    pub def_flow_cfg: *mut k3_udma_glue_rx_flow_cfg,
}

extern "C" {
    pub fn k3_udma_glue_release_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
}
extern "C" {
    pub fn k3_udma_glue_enable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel) -> c_int;
}
extern "C" {
    pub fn k3_udma_glue_disable_rx_chn(rx_chn: *mut k3_udma_glue_rx_channel);
}
extern "C" {
    pub fn k3_udma_glue_rx_get_flow_id_base(rx_chn: *mut k3_udma_glue_rx_channel) -> u32;
}
