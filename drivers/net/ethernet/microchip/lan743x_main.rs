//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/lan743x_main.h
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
// Copyright (C) 2018 Microchip Technology Inc.

// Register Definitions

// Hearthstone top level & System Reg Addresses

// offset 0x400 - 0x500, x may range from 0 to 32, for a total of 33 entries

// offset 0x404 - 0x504, x may range from 0 to 32, for a total of 33 entries

// Vendor Specific SGMII MMD details
pub const SR_VSMMD_PCS_ID1: c_uint = 0x0004;
pub const SR_VSMMD_PCS_ID2: c_uint = 0x0005;
pub const SR_VSMMD_STS: c_uint = 0x0008;
pub const SR_VSMMD_CTRL: c_uint = 0x0009;
pub const VR_MII_DIG_CTRL1: c_uint = 0x8000;

pub const VR_MII_AN_CTRL: c_uint = 0x8001;

pub const VR_MII_AN_INTR_STS: c_uint = 0x8002;

pub const VR_MII_LINK_TIMER_CTRL: c_uint = 0x800A;
pub const VR_MII_DIG_STS: c_uint = 0x8010;

pub const VR_MII_GEN2_4_MPLL_CTRL0: c_uint = 0x8078;

pub const VR_MII_GEN2_4_MPLL_CTRL1: c_uint = 0x8079;

pub const VR_MII_GEN2_4_MISC_CTRL1: c_uint = 0x809A;

pub const RX_CFG_B_TS_NONE_: c_int = 0;

// Hearthstone OTP block registers

// MAC statistics registers

// End of Register definitions

// PCI
// SMSC acquired EFAR late 1990's, MCHP acquired SMSC 2012

// CSR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_csr {
    pub flags: u32,
    pub csr_address: *mut u8 __iomem,
    pub id_rev: u32,
    pub fpga_rev: u32,
}

// INTERRUPTS
extern "C" {
    pub fn void(context: *mut *mut lan743x_vector_handler)(void, int_sts: u32, flags: u32) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_vector {
    pub irq: c_int,
    pub flags: u32,
    pub adapter: *mut lan743x_adapter,
    pub vector_index: c_int,
    pub int_mask: u32,
    pub handler: lan743x_vector_handler,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_intr {
    pub flags: c_int,
    pub irq: c_uint,
    pub vector_list: [lan743x_vector; PCI11X1X_MAX_VECTOR_COUNT],
    pub number_of_vectors: c_int,
    pub using_vectors: bool,
    pub software_isr_flag: bool,
    pub software_isr_wq: wait_queue_head_t,
}

// PHY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_phy {
    pub fc_autoneg: bool,
    pub fc_request_control: u8,
}

// TX

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_tx {
    pub adapter: *mut lan743x_adapter,
    pub ts_flags: u32,
    pub vector_flags: u32,
    pub channel_number: c_int,
    pub ring_size: c_int,
    pub ring_allocation_size: usize,
    pub ring_cpu_ptr: *mut lan743x_tx_descriptor,
    pub ring_dma_ptr: dma_addr_t,
// ring_lock: used to prevent concurrent access to tx ring
    pub ring_lock: spinlock_t,
    pub frame_flags: u32,
    pub frame_first: u32,
    pub frame_data0: u32,
    pub frame_tail: u32,
    pub frame_last: u32,
    pub buffer_info: *mut lan743x_tx_buffer_info,
    pub head_cpu_ptr: *mut __le32,
    pub head_dma_ptr: dma_addr_t,
    pub last_head: c_int,
    pub last_tail: c_int,
    pub napi: napi_struct,
    pub frame_count: u32,
    pub rqd_descriptors: u32,
}

// RX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_rx {
    pub adapter: *mut lan743x_adapter,
    pub vector_flags: u32,
    pub channel_number: c_int,
    pub ring_size: c_int,
    pub ring_allocation_size: usize,
    pub ring_cpu_ptr: *mut lan743x_rx_descriptor,
    pub ring_dma_ptr: dma_addr_t,
    pub buffer_info: *mut lan743x_rx_buffer_info,
    pub head_cpu_ptr: *mut __le32,
    pub head_dma_ptr: dma_addr_t,
    pub last_head: u32,
    pub last_tail: u32,
    pub napi: napi_struct,
    pub frame_count: u32,
    pub skb_tail: *mut *mut sk_buff skb_head,,
}

// SGMII Link Speed Duplex status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lan743x_sgmii_lsd {
    POWER_DOWN = 0,
    LINK_DOWN,
    ANEG_BUSY,
    LINK_10HD,
    LINK_10FD,
    LINK_100HD,
    LINK_100FD,
    LINK_1000_MASTER,
    LINK_1000_SLAVE,
    LINK_2500_MASTER,
    LINK_2500_SLAVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_adapter {
    pub netdev: *mut net_device,
    pub mdiobus: *mut mii_bus,
    pub msg_enable: c_int,

    pub wolopts: u32,
    pub sopass: [u8; SOPASS_MAX],
    pub phy_wolopts: u32,
    pub phy_wol_supported: u32,

    pub pdev: *mut pci_dev,
    pub csr: lan743x_csr,
    pub intr: lan743x_intr,
    pub gpio: lan743x_gpio,
    pub ptp: lan743x_ptp,
    pub mac_address: [u8; ETH_ALEN],
    pub phy: lan743x_phy,
    pub tx: [lan743x_tx; PCI11X1X_USED_TX_CHANNELS],
    pub rx: [lan743x_rx; LAN743X_USED_RX_CHANNELS],
    pub is_pci11x1x: bool,
    pub is_sgmii_en: bool,
    pub is_rmii_en: bool,
// protect ethernet syslock
    pub eth_syslock_spinlock: spinlock_t,
    pub eth_syslock_en: bool,
    pub eth_syslock_acquire_cnt: u32,
    pub sgmii_rw_lock: mutex,
// SGMII Link Speed & Duplex status
    pub sgmii_lsd: lan743x_sgmii_lsd,
    pub max_tx_channels: u8,
    pub used_tx_channels: u8,
    pub max_vector_count: u8,

    pub flags: u32,
    pub hw_cfg: u32,
    pub phy_interface: phy_interface_t,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub rx_tstamp_filter: c_int,
}

pub const MAC_MII_READ: c_int = 1;
pub const MAC_MII_WRITE: c_int = 0;

// TX Descriptor bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_tx_descriptor {
    pub data0: __le32,
    pub data1: __le32,
    pub data2: __le32,
    pub data3: __le32,
    pub __aligned(DEFAULT_DMA_DESCRIPTOR_SPACING): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_tx_buffer_info {
    pub flags: c_int,
    pub skb: *mut sk_buff,
    pub dma_ptr: dma_addr_t,
    pub buffer_length: c_uint,
}

// OWN bit is set. ie, Descs are owned by RX DMAC

// OWN bit is clear. ie, Descs are owned by host

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_rx_descriptor {
    pub data0: __le32,
    pub data1: __le32,
    pub data2: __le32,
    pub data3: __le32,
    pub __aligned(DEFAULT_DMA_DESCRIPTOR_SPACING): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan743x_rx_buffer_info {
    pub flags: c_int,
    pub skb: *mut sk_buff,
    pub dma_ptr: dma_addr_t,
    pub buffer_length: c_uint,
}

extern "C" {
    pub fn lan743x_csr_read(adapter: *mut lan743x_adapter, offset: c_int) -> u32;
}
extern "C" {
    pub fn lan743x_csr_write(adapter: *mut lan743x_adapter, offset: c_int, data: u32);
}
extern "C" {
    pub fn lan743x_hs_syslock_acquire(adapter: *mut lan743x_adapter, timeout: u16) -> c_int;
}
extern "C" {
    pub fn lan743x_hs_syslock_release(adapter: *mut lan743x_adapter);
}
extern "C" {
    pub fn lan743x_sgmii_read(adapter: *mut lan743x_adapter, mmd: u8, addr: u16) -> c_int;
}
