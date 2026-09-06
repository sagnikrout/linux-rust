//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/engleder/tsnep.h
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
// Copyright (C) 2021 Gerhard Engleder <gerhard@engleder-embedded.com>

pub const TSNEP_RING_SIZE: c_int = 256;

pub const TSNEP_RING_RX_REFILL: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_gcl {
    pub addr: *mut void __iomem,
    pub base_time: u64,
    pub cycle_time: u64,
    pub cycle_time_extension: u64,
    pub operation: [tsnep_gcl_operation; TSNEP_GCL_COUNT],
    pub count: c_int,
    pub change_limit: u64,
    pub start_time: u64,
    pub change: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsnep_rxnfc_filter_type {
    TSNEP_RXNFC_ETHER_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rxnfc_filter {
    pub type: tsnep_rxnfc_filter_type,
    pub ether_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rxnfc_rule {
    pub list: list_head,
    pub filter: tsnep_rxnfc_filter,
    pub queue_index: c_int,
    pub location: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_tx_entry {
    pub desc: *mut tsnep_tx_desc,
    pub desc_wb: *mut tsnep_tx_desc_wb,
    pub desc_dma: dma_addr_t,
    pub owner_user_flag: bool,
    pub properties: u32,
    pub type: u32,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
    pub zc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_tx {
    pub adapter: *mut tsnep_adapter,
    pub addr: *mut void __iomem,
    pub queue_index: c_int,
    pub page: [*mut c_void; TSNEP_RING_PAGE_COUNT],
    pub page_dma: [dma_addr_t; TSNEP_RING_PAGE_COUNT],
    pub entry: [tsnep_tx_entry; TSNEP_RING_SIZE],
    pub write: c_int,
    pub read: c_int,
    pub owner_counter: u32,
    pub increment_owner_counter: c_int,
    pub xsk_pool: *mut xsk_buff_pool,
    pub packets: u32,
    pub bytes: u32,
    pub dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rx_entry {
    pub desc: *mut tsnep_rx_desc,
    pub desc_wb: *mut tsnep_rx_desc_wb,
    pub desc_dma: dma_addr_t,
    pub properties: u32,
    pub page: *mut page,
    pub xdp: *mut xdp_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rx {
    pub adapter: *mut tsnep_adapter,
    pub addr: *mut void __iomem,
    pub queue_index: c_int,
    pub tx_queue_index: c_int,
    pub page: [*mut c_void; TSNEP_RING_PAGE_COUNT],
    pub page_dma: [dma_addr_t; TSNEP_RING_PAGE_COUNT],
    pub entry: [tsnep_rx_entry; TSNEP_RING_SIZE],
    pub write: c_int,
    pub read: c_int,
    pub owner_counter: u32,
    pub increment_owner_counter: c_int,
    pub page_pool: *mut page_pool,
    pub page_buffer: *mut page,
    pub xsk_pool: *mut xsk_buff_pool,
    pub xdp_batch: *mut xdp_buff,
    pub packets: u32,
    pub bytes: u32,
    pub dropped: u32,
    pub multicast: u32,
    pub alloc_failed: u32,
    pub xdp_rxq: xdp_rxq_info,
    pub xdp_rxq_zc: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_queue {
    pub adapter: *mut tsnep_adapter,
    pub 16]: char name[IFNAMSIZ +,
    pub tx: *mut tsnep_tx,
    pub rx: *mut tsnep_rx,
    pub napi: napi_struct,
    pub irq: c_int,
    pub irq_mask: u32,
    pub irq_delay_addr: *mut void __iomem,
    pub irq_delay: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_adapter {
    pub netdev: *mut net_device,
    pub mac_address: [u8; ETH_ALEN],
    pub mdiobus: *mut mii_bus,
    pub suppress_preamble: bool,
    pub phy_mode: phy_interface_t,
    pub phydev: *mut phy_device,
    pub msg_enable: c_int,
    pub pdev: *mut platform_device,
    pub dmadev: *mut device,
    pub addr: *mut void __iomem,
    pub gate_control: bool,
// gate control lock
    pub gate_control_lock: mutex,
    pub gate_control_active: bool,
    pub gcl: [tsnep_gcl; 2],
    pub next_gcl: c_int,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_info: ptp_clock_info,
// ptp clock lock
    pub ptp_lock: spinlock_t,
// RX flow classification rules lock
    pub rxnfc_lock: mutex,
    pub rxnfc_rules: list_head,
    pub rxnfc_count: c_int,
    pub rxnfc_max: c_int,
    pub xdp_prog: *mut bpf_prog,
    pub num_tx_queues: c_int,
    pub tx: [tsnep_tx; TSNEP_MAX_QUEUES],
    pub num_rx_queues: c_int,
    pub rx: [tsnep_rx; TSNEP_MAX_QUEUES],
    pub num_queues: c_int,
    pub queue: [tsnep_queue; TSNEP_MAX_QUEUES],
}

extern "C" {
    pub fn tsnep_ptp_init(adapter: *mut tsnep_adapter) -> c_int;
}
extern "C" {
    pub fn tsnep_ptp_cleanup(adapter: *mut tsnep_adapter);
}
extern "C" {
    pub fn tsnep_tc_init(adapter: *mut tsnep_adapter) -> c_int;
}
extern "C" {
    pub fn tsnep_tc_cleanup(adapter: *mut tsnep_adapter);
}
extern "C" {
    pub fn tsnep_rxnfc_init(adapter: *mut tsnep_adapter) -> c_int;
}
extern "C" {
    pub fn tsnep_rxnfc_cleanup(adapter: *mut tsnep_adapter);
}

extern "C" {
    pub fn tsnep_ethtool_get_test_count() -> c_int;
}
extern "C" {
    pub fn tsnep_ethtool_get_test_strings(data: *mut u8);
}

// not enabled

extern "C" {
    pub fn tsnep_get_system_time(adapter: *mut tsnep_adapter, time: *mut u64);
}
extern "C" {
    pub fn tsnep_set_irq_coalesce(queue: *mut tsnep_queue, usecs: u32) -> c_int;
}
extern "C" {
    pub fn tsnep_get_irq_coalesce(queue: *mut tsnep_queue) -> u32;
}
extern "C" {
    pub fn tsnep_enable_xsk(queue: *mut tsnep_queue, pool: *mut xsk_buff_pool) -> c_int;
}
extern "C" {
    pub fn tsnep_disable_xsk(queue: *mut tsnep_queue);
}
