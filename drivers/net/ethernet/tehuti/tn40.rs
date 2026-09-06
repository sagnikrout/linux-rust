//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/tehuti/tn40.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) Tehuti Networks Ltd.

pub const PCI_DEVICE_ID_TEHUTI_TN9510: c_uint = 0x4025;

// netdev tx queue len for Luxor. The default value is 1000.
// ifconfig eth1 txqueuelen 3000 - to change it at runtime.
//
pub const TN40_NDEV_TXQ_LEN: c_int = 1000;
pub const TN40_FIFO_SIZE: c_int = 4096;
pub const TN40_FIFO_EXTRA_SPACE: c_int = 1024;
pub const TN40_TXF_DESC_SZ: c_int = 16;

pub const TN40_MIN_TX_LEVEL: c_int = 256;
pub const TN40_NO_UPD_PACKETS: c_int = 40;

pub const TN40_PCK_TH_MULT: c_int = 128;
pub const TN40_INT_COAL_MULT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_fifo {
    pub /: *mut *mut dma_addr_t da; / Physical address of fifo (used by HW),
    pub /: *mut *mut *mut char va; / Virtual address of fifo (used by SW),
    pub wptr: u32 rptr,,
// Cached values of RPTR and WPTR registers,
// they're 32 bits on both 32 and 64 archs.
//
    pub reg_cfg0: u16,
    pub reg_cfg1: u16,
    pub reg_rptr: u16,
    pub reg_wptr: u16,
    pub /: *mut *mut u16 memsz; / Memory size allocated for fifo,
    pub size_mask: u16,
    pub /: *mut *mut u16 pktsz; / Skb packet size to allocate,
    pub /: *mut *mut u16 rcvno; / Number of buffers that come from this RXF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_txf_fifo {
    pub /: *mut *mut tn40_fifo m; / The minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_txd_fifo {
    pub /: *mut *mut tn40_fifo m; / The minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rxf_fifo {
    pub /: *mut *mut tn40_fifo m; / The minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rxd_fifo {
    pub /: *mut *mut tn40_fifo m; / The minimal set of variables used by all fifos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rx_map {
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rxdb {
    pub stack: *mut c_uint,
    pub elems: *mut tn40_rx_map,
    pub nelem: c_uint,
    pub top: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tn40_tx_dma_addr {
    pub dma: dma_addr_t,
    pub skb: *mut sk_buff,
}

// Entry in the db.
// if len == 0 addr is dma
// if len != 0 addr is skb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_tx_map {
    pub addr: tn40_tx_dma_addr,
    pub len: c_int,
}

// tx database - implemented as circular fifo buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_txdb {
    pub /: *mut *mut *mut tn40_tx_map start; / Points to the first element,
    pub /: *mut *mut *mut tn40_tx_map end; / Points just AFTER the last element,
    pub /: *mut *mut *mut tn40_tx_map rptr; / Points to the next element to read,
    pub /: *mut *mut *mut tn40_tx_map wptr; / Points to the next element to write,
    pub /: *mut *mut int size; / Number of elements in the db,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tn40_swnodes {
    SWNODE_MDIO,
    SWNODE_PHY,
    SWNODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_nodes {
    pub phy_name: [c_char; 32],
    pub mdio_name: [c_char; 32],
    pub phy_props: [property_entry; 3],
    pub swnodes: [software_node; SWNODE_MAX],
    pub 1]: *const *const software_node group[SWNODE_MAX +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_priv {
    pub ndev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub nodes: tn40_nodes,
    pub napi: napi_struct,
// RX FIFOs: 1 for data (full) descs, and 2 for free descs
    pub rxd_fifo0: tn40_rxd_fifo,
    pub rxf_fifo0: tn40_rxf_fifo,
    pub /: *mut *mut *mut tn40_rxdb rxdb0; / Rx dbs to store skb pointers,
    pub page_pool: *mut page_pool,
// Tx FIFOs: 1 for data desc, 1 for empty (acks) desc
    pub txd_fifo0: tn40_txd_fifo,
    pub txf_fifo0: tn40_txf_fifo,
    pub txdb: tn40_txdb,
    pub tx_level: c_int,
    pub tx_update_mark: c_int,
    pub tx_noupd: c_int,
    pub stats_flag: c_int,
    pub stats: rtnl_link_stats64,
    pub alloc_fail: u64,
    pub syncp: u64_stats_sync,
    pub txd_size: u8,
    pub txf_size: u8,
    pub rxd_size: u8,
    pub rxf_size: u8,
    pub rdintcm: u32,
    pub tdintcm: u32,
    pub isr_mask: u32,
    pub regs: *mut void __iomem,
// SHORT_PKT_FIX
    pub b0_len: u32,
    pub /: *mut *mut dma_addr_t b0_dma; / Physical address of buffer,
    pub /: *mut *mut *mut char b0_va; / Virtual address of buffer,
    pub mdio: *mut mii_bus,
    pub phydev: *mut phy_device,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
}

// RX FREE descriptor - 64bit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rxf_desc {
    pub /: *mut *mut __le32 info; / Buffer Count + Info - described below,
    pub /: *mut *mut __le32 va_lo; / VAdr[31:0],
    pub /: *mut *mut __le32 va_hi; / VAdr[63:32],
    pub /: *mut *mut __le32 pa_lo; / PAdr[31:0],
    pub /: *mut *mut __le32 pa_hi; / PAdr[63:32],
    pub /: *mut *mut __le32 len; / Buffer Length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_rxd_desc {
    pub rxd_val1: __le32,
    pub len: __le16,
    pub rxd_vlan: __le16,
    pub va_lo: __le32,
    pub va_hi: __le32,
    pub rss_lo: __le32,
    pub rss_hash: __le32,
}

// PBL describes each virtual buffer to be transmitted from the host.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_pbl {
    pub pa_lo: __le32,
    pub pa_hi: __le32,
    pub len: __le32,
}

// First word for TXD descriptor. It means: type = 3 for regular Tx packet,
// hw_csum = 7 for IP+UDP+TCP HW checksums.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_txd_desc {
    pub txd_val1: __le32,
    pub mss: __le16,
    pub length: __le16,
    pub va_lo: __le32,
    pub va_hi: __le32,
    pub /: *mut *mut tn40_pbl pbl[]; / Fragments,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tn40_txf_desc {
    pub status: u32,
    pub /: *mut *mut u32 va_lo; / VAdr[31:0],
    pub /: *mut *mut u32 va_hi; / VAdr[63:32],
    pub pad: u32,
}

extern "C" {
    pub fn readl(reg: priv->regs +) -> return;
}
extern "C" {
    pub fn tn40_set_link_speed(priv: *mut tn40_priv, speed: u32) -> c_int;
}
extern "C" {
    pub fn tn40_swnodes_cleanup(priv: *mut tn40_priv);
}
extern "C" {
    pub fn tn40_mdiobus_init(priv: *mut tn40_priv) -> c_int;
}
extern "C" {
    pub fn tn40_phy_register(priv: *mut tn40_priv) -> c_int;
}
extern "C" {
    pub fn tn40_phy_unregister(priv: *mut tn40_priv);
}
