//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_net.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//

pub const EEA_VER_MAJOR: c_int = 1;
pub const EEA_VER_MINOR: c_int = 0;
pub const EEA_VER_SUB_MINOR: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net_tx {
    pub enet: *mut eea_net,
    pub ering: *mut eea_ring,
    pub meta: *mut eea_tx_meta,
    pub free: *mut eea_tx_meta,
    pub dma_dev: *mut device,
    pub index: u32,
    pub name: [c_char; 16],
    pub stats: eea_tx_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_meta {
    pub next: *mut eea_rx_meta,
    pub page: *mut page,
    pub dma: dma_addr_t,
    pub offset: u32,
    pub sync_for_cpu: u32,
    pub frags: u32,
    pub hdr_page: *mut page,
    pub hdr_addr: *mut c_void,
    pub hdr_dma: dma_addr_t,
    pub id: u32,
    pub truesize: u32,
    pub headroom: u32,
    pub tailroom: u32,
    pub len: u32,
    pub in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net_rx_pkt_ctx {
    pub idx: u16,
    pub data_valid: bool,
    pub do_drop: bool,
    pub recv_len: u32,
    pub head_skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net_rx {
    pub enet: *mut eea_net,
    pub ering: *mut eea_ring,
    pub meta: *mut eea_rx_meta,
    pub free: *mut eea_rx_meta,
    pub dma_dev: *mut device,
    pub index: u32,
    pub flags: u32,
    pub headroom: u32,
    pub napi: *mut napi_struct,
    pub stats: eea_rx_stats,
    pub name: [c_char; 16],
    pub pkt: eea_net_rx_pkt_ctx,
    pub pp: *mut page_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net_cfg {
    pub rx_ring_depth: u32,
    pub tx_ring_depth: u32,
    pub rx_ring_num: u32,
    pub tx_ring_num: u32,
    pub rx_sq_desc_size: u8,
    pub rx_cq_desc_size: u8,
    pub tx_sq_desc_size: u8,
    pub tx_cq_desc_size: u8,
    pub split_hdr: u32,
    pub ts_cfg: hwtstamp_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net_init_ctx {
    pub cfg: eea_net_cfg,
    pub tx: *mut eea_net_tx,
    pub rx: *mut eea_net_rx,
    pub netdev: *mut net_device,
    pub edev: *mut eea_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_irq_blk {
    pub napi: napi_struct,
    pub msix_vec: u16,
    pub ready: bool,
    pub rx: *mut eea_net_rx,
    pub irq_name: [c_char; 32],
    pub irq: c_int,
    pub idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_net {
    pub edev: *mut eea_device,
    pub netdev: *mut net_device,
    pub adminq: eea_aq,
    pub tx: *mut eea_net_tx,
    pub rx: *mut eea_net_rx,
    pub cfg: eea_net_cfg,
    pub cfg_hw: eea_net_cfg,
    pub irq_blks: *mut eea_irq_blk,
    pub link_err: u32,
    pub started: bool,
    pub wait_pci_ready: bool,
    pub duplex: u8,
    pub speed: u32,
    pub hw_ts_offset: u64,
// Protect the tx and rx of struct eea_net, when eea_stats accesses the
// stats from rx and tx queues.
//
    pub stats_lock: spinlock_t,
}

extern "C" {
    pub fn eea_net_probe(edev: *mut eea_device) -> c_int;
}
extern "C" {
    pub fn eea_net_remove(edev: *mut eea_device, ha: bool);
}
extern "C" {
    pub fn eea_net_shutdown(edev: *mut eea_device);
}
extern "C" {
    pub fn eea_reset_hw_resources(enet: *mut eea_net, ctx: *mut eea_net_init_ctx) -> c_int;
}
extern "C" {
    pub fn eea_init_ctx(enet: *mut eea_net, ctx: *mut eea_net_init_ctx);
}
extern "C" {
    pub fn eea_queues_check_and_reset(edev: *mut eea_device) -> c_int;
}
// rx apis
extern "C" {
    pub fn enet_rx_stop(rx: *mut eea_net_rx);
}
extern "C" {
    pub fn enet_rx_start(rx: *mut eea_net_rx);
}
extern "C" {
    pub fn eea_free_rx(rx: *mut eea_net_rx, cfg: *mut eea_net_cfg);
}
// tx apis
extern "C" {
    pub fn eea_poll_tx(tx: *mut eea_net_tx, budget: c_int) -> c_int;
}
extern "C" {
    pub fn eea_tx_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn eea_free_tx(tx: *mut eea_net_tx, cfg: *mut eea_net_cfg);
}
extern "C" {
    pub fn eea_alloc_tx(ctx: *mut eea_net_init_ctx, tx: *mut eea_net_tx, idx: u32) -> c_int;
}
