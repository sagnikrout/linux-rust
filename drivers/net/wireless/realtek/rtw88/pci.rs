//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw88/pci.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2018-2019  Realtek Corporation
//

pub const RTK_DEFAULT_TX_DESC_NUM: c_int = 128;
pub const RTK_BEQ_TX_DESC_NUM: c_int = 256;
pub const RTK_MAX_RX_DESC_NUM: c_int = 512;
// 11K + rx desc size

pub const RTK_PCI_CTRL: c_uint = 0x300;

pub const REG_DBI_WDATA_V1: c_uint = 0x03E8;
pub const REG_DBI_RDATA_V1: c_uint = 0x03EC;
pub const REG_DBI_FLAG_V1: c_uint = 0x03F0;

pub const REG_MDIO_V1: c_uint = 0x03F4;
pub const REG_PCIE_MIX_CFG: c_uint = 0x03F8;

pub const RTW_PCI_MDIO_PG_OFFS_G1: c_int = 0;
pub const RTW_PCI_MDIO_PG_OFFS_G2: c_int = 2;
pub const RTW_PCI_WR_RETRY_CNT: c_int = 20;
pub const RTK_PCIE_LINK_CFG: c_uint = 0x0719;

pub const RTK_PCIE_CLKDLY_CTRL: c_uint = 0x0725;

pub const RTK_PCI_TXBD_DESA_BCNQ: c_uint = 0x308;
pub const RTK_PCI_TXBD_DESA_H2CQ: c_uint = 0x1320;
pub const RTK_PCI_TXBD_DESA_MGMTQ: c_uint = 0x310;
pub const RTK_PCI_TXBD_DESA_BKQ: c_uint = 0x330;
pub const RTK_PCI_TXBD_DESA_BEQ: c_uint = 0x328;
pub const RTK_PCI_TXBD_DESA_VIQ: c_uint = 0x320;
pub const RTK_PCI_TXBD_DESA_VOQ: c_uint = 0x318;
pub const RTK_PCI_TXBD_DESA_HI0Q: c_uint = 0x340;
pub const RTK_PCI_RXBD_DESA_MPDUQ: c_uint = 0x338;

// BCNQ is specialized for rsvd page, does not need to specify a number
pub const RTK_PCI_TXBD_NUM_H2CQ: c_uint = 0x1328;
pub const RTK_PCI_TXBD_NUM_MGMTQ: c_uint = 0x380;
pub const RTK_PCI_TXBD_NUM_BKQ: c_uint = 0x38A;
pub const RTK_PCI_TXBD_NUM_BEQ: c_uint = 0x388;
pub const RTK_PCI_TXBD_NUM_VIQ: c_uint = 0x386;
pub const RTK_PCI_TXBD_NUM_VOQ: c_uint = 0x384;
pub const RTK_PCI_TXBD_NUM_HI0Q: c_uint = 0x38C;
pub const RTK_PCI_RXBD_NUM_MPDUQ: c_uint = 0x382;
pub const RTK_PCI_TXBD_IDX_H2CQ: c_uint = 0x132C;
pub const RTK_PCI_TXBD_IDX_MGMTQ: c_uint = 0x3B0;
pub const RTK_PCI_TXBD_IDX_BKQ: c_uint = 0x3AC;
pub const RTK_PCI_TXBD_IDX_BEQ: c_uint = 0x3A8;
pub const RTK_PCI_TXBD_IDX_VIQ: c_uint = 0x3A4;
pub const RTK_PCI_TXBD_IDX_VOQ: c_uint = 0x3A0;
pub const RTK_PCI_TXBD_IDX_HI0Q: c_uint = 0x3B8;
pub const RTK_PCI_RXBD_IDX_MPDUQ: c_uint = 0x3B4;
pub const RTK_PCI_TXBD_RWPTR_CLR: c_uint = 0x39C;
pub const RTK_PCI_TXBD_H2CQ_CSR: c_uint = 0x1330;

pub const RTK_PCI_HIMR0: c_uint = 0x0B0;
pub const RTK_PCI_HISR0: c_uint = 0x0B4;
pub const RTK_PCI_HIMR1: c_uint = 0x0B8;
pub const RTK_PCI_HISR1: c_uint = 0x0BC;
pub const RTK_PCI_HIMR2: c_uint = 0x10B0;
pub const RTK_PCI_HISR2: c_uint = 0x10B4;
pub const RTK_PCI_HIMR3: c_uint = 0x10B8;
pub const RTK_PCI_HISR3: c_uint = 0x10BC;
// IMR 0

// IMR 1

// IMR 3

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw_pci_flags {
    RTW_PCI_FLAG_NAPI_RUNNING,

    NUM_OF_RTW_PCI_FLAGS,
}

// one element is reserved to know if the ring is closed
pub const RTK_PCI_TXBD_OWN_OFFSET: c_int = 15;
pub const RTK_PCI_TXBD_BCN_WORK: c_uint = 0x383;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_tx_buffer_desc {
    pub buf_size: __le16,
    pub psb_len: __le16,
    pub dma: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_tx_data {
    pub dma: dma_addr_t,
    pub sn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_ring {
    pub head: *mut u8,
    pub dma: dma_addr_t,
    pub desc_size: u8,
    pub len: u32,
    pub wp: u32,
    pub rp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_tx_ring {
    pub r: rtw_pci_ring,
    pub queue: sk_buff_head,
    pub queue_stopped: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_rx_buffer_desc {
    pub buf_size: __le16,
    pub total_pkt_size: __le16,
    pub dma: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci_rx_ring {
    pub r: rtw_pci_ring,
    pub buf: [*mut sk_buff; RTK_MAX_RX_DESC_NUM],
}

pub const RX_TAG_MAX: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw_pci {
    pub pdev: *mut pci_dev,
// Used for PCI interrupt.
    pub hwirq_lock: spinlock_t,
// Used for PCI TX ring/queueing, and enable INT.
    pub irq_lock: spinlock_t,
    pub irq_mask: [u32; 4],
    pub irq_enabled: bool,
    pub running: bool,
// napi structure
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub rx_tag: u16,
    pub RTK_MAX_TX_QUEUE_NUM): DECLARE_BITMAP(tx_queued,,
    pub tx_rings: [rtw_pci_tx_ring; RTK_MAX_TX_QUEUE_NUM],
    pub rx_rings: [rtw_pci_rx_ring; RTK_MAX_RX_QUEUE_NUM],
    pub link_ctrl: u16,
    pub link_usage: core::sync::atomic::AtomicI32,
    pub rx_no_aspm: bool,
    pub NUM_OF_RTW_PCI_FLAGS): DECLARE_BITMAP(flags,,
    pub mmap: *mut void __iomem,
}

extern "C" {
    pub fn rtw_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn rtw_pci_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn rtw_pci_shutdown(pdev: *mut pci_dev);
}
