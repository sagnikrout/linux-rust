//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mscc/ocelot_fdma.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi SoCs FDMA driver
//
// Copyright (c) 2021 Microchip
//

pub const MSCC_FDMA_CH_SAFE: c_uint = 0xcc;
pub const MSCC_FDMA_CH_ACTIVATE: c_uint = 0xd0;
pub const MSCC_FDMA_CH_DISABLE: c_uint = 0xd4;
pub const MSCC_FDMA_CH_FORCEDIS: c_uint = 0xd8;
pub const MSCC_FDMA_EVT_ERR: c_uint = 0x164;
pub const MSCC_FDMA_EVT_ERR_CODE: c_uint = 0x168;
pub const MSCC_FDMA_INTR_LLP: c_uint = 0x16c;
pub const MSCC_FDMA_INTR_LLP_ENA: c_uint = 0x170;
pub const MSCC_FDMA_INTR_FRM: c_uint = 0x174;
pub const MSCC_FDMA_INTR_FRM_ENA: c_uint = 0x178;
pub const MSCC_FDMA_INTR_ENA: c_uint = 0x184;
pub const MSCC_FDMA_INTR_IDENT: c_uint = 0x188;
pub const MSCC_FDMA_INJ_CHAN: c_int = 2;
pub const MSCC_FDMA_XTR_CHAN: c_int = 0;
pub const OCELOT_FDMA_WEIGHT: c_int = 32;
pub const OCELOT_FDMA_CH_SAFE_TIMEOUT_US: c_int = 10;
pub const OCELOT_FDMA_RX_RING_SIZE: c_int = 512;
pub const OCELOT_FDMA_TX_RING_SIZE: c_int = 128;

// +4 allows for word alignment after allocation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma_dcb {
    pub llp: u32,
    pub datap: u32,
    pub datal: u32,
    pub stat: u32,
    pub __packed: },
//
// struct ocelot_fdma_tx_buf - TX buffer structure
// @skb: SKB currently used in the corresponding DCB.
// @dma_addr: SKB DMA mapped address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma_tx_buf {
    pub skb: *mut sk_buff,
}

//
// struct ocelot_fdma_tx_ring - TX ring description of DCBs
//
// @dcbs: DCBs allocated for the ring
// @dcbs_dma: DMA base address of the DCBs
// @bufs: List of TX buffer associated to the DCBs
// @xmit_lock: lock for concurrent xmit access
// @next_to_clean: Next DCB to be cleaned in tx_cleanup
// @next_to_use: Next available DCB to send SKB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma_tx_ring {
    pub dcbs: *mut ocelot_fdma_dcb,
    pub dcbs_dma: dma_addr_t,
    pub bufs: [ocelot_fdma_tx_buf; OCELOT_FDMA_TX_RING_SIZE],
// Protect concurrent xmit calls
    pub xmit_lock: spinlock_t,
    pub next_to_clean: u16,
    pub next_to_use: u16,
}

//
// struct ocelot_fdma_rx_buf - RX buffer structure
// @page: Struct page used in this buffer
// @page_offset: Current page offset (either 0 or PAGE_SIZE/2)
// @dma_addr: DMA address of the page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma_rx_buf {
    pub page: *mut page,
    pub page_offset: u32,
    pub dma_addr: dma_addr_t,
}

//
// struct ocelot_fdma_rx_ring - TX ring description of DCBs
//
// @dcbs: DCBs allocated for the ring
// @dcbs_dma: DMA base address of the DCBs
// @bufs: List of RX buffer associated to the DCBs
// @skb: SKB currently received by the netdev
// @next_to_clean: Next DCB to be cleaned NAPI polling
// @next_to_use: Next available DCB to send SKB
// @next_to_alloc: Next buffer that needs to be allocated (page reuse or alloc)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma_rx_ring {
    pub dcbs: *mut ocelot_fdma_dcb,
    pub dcbs_dma: dma_addr_t,
    pub bufs: [ocelot_fdma_rx_buf; OCELOT_FDMA_RX_RING_SIZE],
    pub skb: *mut sk_buff,
    pub next_to_clean: u16,
    pub next_to_use: u16,
    pub next_to_alloc: u16,
}

//
// struct ocelot_fdma - FDMA context
//
// @irq: FDMA interrupt
// @ndev: Net device used to initialize NAPI
// @dcbs_base: Memory coherent DCBs
// @dcbs_dma_base: DMA base address of memory coherent DCBs
// @tx_ring: Injection ring
// @rx_ring: Extraction ring
// @napi: NAPI context
// @ocelot: Back-pointer to ocelot struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_fdma {
    pub irq: c_int,
    pub ndev: *mut net_device,
    pub dcbs_base: *mut ocelot_fdma_dcb,
    pub dcbs_dma_base: dma_addr_t,
    pub tx_ring: ocelot_fdma_tx_ring,
    pub rx_ring: ocelot_fdma_rx_ring,
    pub napi: napi_struct,
    pub ocelot: *mut ocelot,
}

extern "C" {
    pub fn ocelot_fdma_init(pdev: *mut platform_device, ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_fdma_start(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_fdma_deinit(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_fdma_netdev_init(ocelot: *mut ocelot, dev: *mut net_device);
}
