//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pasemi/pasemi_mac.h
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
// Copyright (C) 2006 PA Semi, Inc
//
// Driver for the PA6T-1682M onchip 1G/10G Ethernet MACs, soft state and
// hardware register layouts.
//

// Must be a power of two
pub const RX_RING_SIZE: c_int = 2048;
pub const TX_RING_SIZE: c_int = 4096;

pub const MAX_CS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_mac_txring {
    pub /: *mut *mut pasemi_dmachan chan; / Must be first,
    pub lock: spinlock_t,
    pub size: c_uint,
    pub next_to_fill: c_uint,
    pub next_to_clean: c_uint,
    pub ring_info: *mut pasemi_mac_buffer,
    pub /: *mut *mut *mut pasemi_mac mac; / Needed in intr handler,
    pub clean_timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_mac_rxring {
    pub /: *mut *mut pasemi_dmachan chan; / Must be first,
    pub lock: spinlock_t,
    pub /: *mut *mut *mut u64 buffers; / RX interface buffer ring,
    pub buf_dma: dma_addr_t,
    pub size: c_uint,
    pub next_to_fill: c_uint,
    pub next_to_clean: c_uint,
    pub ring_info: *mut pasemi_mac_buffer,
    pub /: *mut *mut *mut pasemi_mac mac; / Needed in intr handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_mac_csring {
    pub chan: pasemi_dmachan,
    pub size: c_uint,
    pub next_to_fill: c_uint,
    pub events: [c_int; 2],
    pub last_event: c_int,
    pub fun: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_mac {
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub dma_pdev: *mut pci_dev,
    pub iob_pdev: *mut pci_dev,
    pub napi: napi_struct,
    pub /: *mut *mut int bufsz; / RX ring buffer size,
    pub last_cs: c_int,
    pub num_cs: c_int,
    pub dma_if: u32,
    pub type: u8,
pub const MAC_TYPE_GMAC: c_int = 1;
pub const MAC_TYPE_XAUI: c_int = 2;
    pub mac_addr: [u8; ETH_ALEN],
    pub rxtimer: timer_list,
    pub tx: *mut pasemi_mac_txring,
    pub rx: *mut pasemi_mac_rxring,
    pub cs: [*mut pasemi_mac_csring; MAX_CS],
    pub /: *mut *mut char tx_irq_name[10]; / "eth%d tx",
    pub /: *mut *mut char rx_irq_name[10]; / "eth%d rx",
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub msg_enable: c_uint,
}

// Software status descriptor (ring_info)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasemi_mac_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
}

// PCI register offsets and formats
// MAC CFG register offsets
// MAC CFG register fields
pub const PAS_MAC_CFG_PCFG_PE: c_uint = 0x80000000;
pub const PAS_MAC_CFG_PCFG_CE: c_uint = 0x40000000;
pub const PAS_MAC_CFG_PCFG_BU: c_uint = 0x20000000;
pub const PAS_MAC_CFG_PCFG_TT: c_uint = 0x10000000;
pub const PAS_MAC_CFG_PCFG_TSR_M: c_uint = 0x0c000000;
pub const PAS_MAC_CFG_PCFG_TSR_10M: c_uint = 0x00000000;
pub const PAS_MAC_CFG_PCFG_TSR_100M: c_uint = 0x04000000;
pub const PAS_MAC_CFG_PCFG_TSR_1G: c_uint = 0x08000000;
pub const PAS_MAC_CFG_PCFG_TSR_10G: c_uint = 0x0c000000;
pub const PAS_MAC_CFG_PCFG_T24: c_uint = 0x02000000;
pub const PAS_MAC_CFG_PCFG_PR: c_uint = 0x01000000;
pub const PAS_MAC_CFG_PCFG_CRO_M: c_uint = 0x00ff0000;
pub const PAS_MAC_CFG_PCFG_CRO_S: c_int = 16;
pub const PAS_MAC_CFG_PCFG_IPO_M: c_uint = 0x0000ff00;
pub const PAS_MAC_CFG_PCFG_IPO_S: c_int = 8;
pub const PAS_MAC_CFG_PCFG_S1: c_uint = 0x00000080;
pub const PAS_MAC_CFG_PCFG_IO_M: c_uint = 0x00000060;
pub const PAS_MAC_CFG_PCFG_IO_MAC: c_uint = 0x00000000;
pub const PAS_MAC_CFG_PCFG_IO_OFF: c_uint = 0x00000020;
pub const PAS_MAC_CFG_PCFG_IO_IND_ETH: c_uint = 0x00000040;
pub const PAS_MAC_CFG_PCFG_IO_IND_IP: c_uint = 0x00000060;
pub const PAS_MAC_CFG_PCFG_LP: c_uint = 0x00000010;
pub const PAS_MAC_CFG_PCFG_TS: c_uint = 0x00000008;
pub const PAS_MAC_CFG_PCFG_HD: c_uint = 0x00000004;
pub const PAS_MAC_CFG_PCFG_SPD_M: c_uint = 0x00000003;
pub const PAS_MAC_CFG_PCFG_SPD_10M: c_uint = 0x00000000;
pub const PAS_MAC_CFG_PCFG_SPD_100M: c_uint = 0x00000001;
pub const PAS_MAC_CFG_PCFG_SPD_1G: c_uint = 0x00000002;
pub const PAS_MAC_CFG_PCFG_SPD_10G: c_uint = 0x00000003;
pub const PAS_MAC_CFG_MACCFG_TXT_M: c_uint = 0x70000000;
pub const PAS_MAC_CFG_MACCFG_TXT_S: c_int = 28;
pub const PAS_MAC_CFG_MACCFG_PRES_M: c_uint = 0x0f000000;
pub const PAS_MAC_CFG_MACCFG_PRES_S: c_int = 24;
pub const PAS_MAC_CFG_MACCFG_MAXF_M: c_uint = 0x00ffff00;
pub const PAS_MAC_CFG_MACCFG_MAXF_S: c_int = 8;

pub const PAS_MAC_CFG_MACCFG_MINF_M: c_uint = 0x000000ff;
pub const PAS_MAC_CFG_MACCFG_MINF_S: c_int = 0;
pub const PAS_MAC_CFG_TXP_FCF: c_uint = 0x01000000;
pub const PAS_MAC_CFG_TXP_FCE: c_uint = 0x00800000;
pub const PAS_MAC_CFG_TXP_FC: c_uint = 0x00400000;
pub const PAS_MAC_CFG_TXP_FPC_M: c_uint = 0x00300000;
pub const PAS_MAC_CFG_TXP_FPC_S: c_int = 20;

pub const PAS_MAC_CFG_TXP_RT: c_uint = 0x00080000;
pub const PAS_MAC_CFG_TXP_BL: c_uint = 0x00040000;
pub const PAS_MAC_CFG_TXP_SL_M: c_uint = 0x00030000;
pub const PAS_MAC_CFG_TXP_SL_S: c_int = 16;

pub const PAS_MAC_CFG_TXP_COB_M: c_uint = 0x0000f000;
pub const PAS_MAC_CFG_TXP_COB_S: c_int = 12;

pub const PAS_MAC_CFG_TXP_TIFT_M: c_uint = 0x00000f00;
pub const PAS_MAC_CFG_TXP_TIFT_S: c_int = 8;

pub const PAS_MAC_CFG_TXP_TIFG_M: c_uint = 0x000000ff;
pub const PAS_MAC_CFG_TXP_TIFG_S: c_int = 0;

pub const PAS_MAC_IPC_CHNL_DCHNO_M: c_uint = 0x003f0000;
pub const PAS_MAC_IPC_CHNL_DCHNO_S: c_int = 16;

pub const PAS_MAC_IPC_CHNL_BCH_M: c_uint = 0x0000003f;
pub const PAS_MAC_IPC_CHNL_BCH_S: c_int = 0;

