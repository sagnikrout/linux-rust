//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/au1000_eth.h
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
// Alchemy Au1x00 ethernet driver include file
//
// Author: Pete Popov <ppopov@mvista.com>
//
// Copyright 2001 MontaVista Software Inc.
//
pub const MAC_IOSIZE: c_uint = 0x10000;

pub const NUM_RX_BUFFS: c_int = 4;
pub const NUM_TX_BUFFS: c_int = 4;
pub const MAX_BUF_SIZE: c_int = 2048;

pub const MAC_MIN_PKT_SIZE: c_int = 64;
pub const MULTICAST_FILTER_LIMIT: c_int = 64;
//
// Data Buffer Descriptor. Data buffers must be aligned on 32 byte
// boundary for both, receive and transmit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_dest {
    pub pnext: *mut db_dest,
    pub vaddr: *mut u32,
    pub dma_addr: dma_addr_t,
}

//
// The transmit and receive descriptors are memory
// mapped registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_dma {
    pub status: u32,
    pub buff_stat: u32,
    pub len: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_dma {
    pub status: u32,
    pub buff_stat: u32,
    pub pad: [u32; 2],
}

//
// MAC control registers, memory mapped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_reg {
    pub control: u32,
    pub mac_addr_high: u32,
    pub mac_addr_low: u32,
    pub multi_hash_high: u32,
    pub multi_hash_low: u32,
    pub mii_control: u32,
    pub mii_data: u32,
    pub flow_control: u32,
    pub vlan1_tag: u32,
    pub vlan2_tag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au1000_private {
    pub pDBfree: *mut db_dest,
    pub db: [db_dest; NUM_RX_BUFFS+NUM_TX_BUFFS],
    pub rx_dma_ring: [*mut rx_dma; NUM_RX_DMA],
    pub tx_dma_ring: [*mut tx_dma; NUM_TX_DMA],
    pub rx_db_inuse: [*mut db_dest; NUM_RX_DMA],
    pub tx_db_inuse: [*mut db_dest; NUM_TX_DMA],
    pub rx_head: u32,
    pub tx_head: u32,
    pub tx_tail: u32,
    pub tx_full: u32,
    pub mac_id: c_int,
    pub running: *mut *mut int mac_enabled; / whether MAC is currently enabled and,
// (req. for mdio)
//
    pub /: *mut *mut int old_link; / used by au1000_adjust_link,
    pub old_speed: c_int,
    pub old_duplex: c_int,
    pub mii_bus: *mut mii_bus,
// PHY configuration
    pub phy_static_config: c_int,
    pub phy_search_highest_addr: c_int,
    pub phy1_search_mac0: c_int,
    pub phy_addr: c_int,
    pub phy_busid: c_int,
    pub phy_irq: c_int,
// These variables are just for quick access
// to certain regs addresses.
//
    pub /: *mut *mut *mut mac_reg mac; / mac registers,
    pub /: *mut *mut *mut u32 enable; / address of MAC Enable Register,
    pub /: *mut *mut *mut void __iomem macdma; / base of MAC DMA port,
    pub /: *mut *mut *mut void vaddr; / virtual address of rx/tx buffers,
    pub /: *mut *mut dma_addr_t dma_addr; / dma address of rx/tx buffers,
    pub /: *mut *mut spinlock_t lock; / Serialise access to device,
    pub msg_enable: u32,
}
