//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/actions/owl-emac.h
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
// Actions Semi Owl SoCs Ethernet MAC driver
//
// Copyright (c) 2012 Actions Semi Inc.
// Copyright (c) 2021 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
//

pub const OWL_EMAC_POLL_DELAY_USEC: c_int = 5;
pub const OWL_EMAC_MDIO_POLL_TIMEOUT_USEC: c_int = 1000;
pub const OWL_EMAC_RESET_POLL_TIMEOUT_USEC: c_int = 2000;

pub const OWL_EMAC_SKB_ALIGN: c_int = 4;
pub const OWL_EMAC_SKB_RESERVE: c_int = 18;
pub const OWL_EMAC_MAX_MULTICAST_ADDRS: c_int = 14;
pub const OWL_EMAC_SETUP_FRAME_LEN: c_int = 192;
pub const OWL_EMAC_RX_RING_SIZE: c_int = 64;
pub const OWL_EMAC_TX_RING_SIZE: c_int = 32;
// Bus mode register
pub const OWL_EMAC_REG_MAC_CSR0: c_uint = 0x0000;

// Transmit/receive poll demand registers
pub const OWL_EMAC_REG_MAC_CSR1: c_uint = 0x0008;
pub const OWL_EMAC_VAL_MAC_CSR1_TPD: c_uint = 0x01;
pub const OWL_EMAC_REG_MAC_CSR2: c_uint = 0x0010;
pub const OWL_EMAC_VAL_MAC_CSR2_RPD: c_uint = 0x01;
// Receive/transmit descriptor list base address registers
pub const OWL_EMAC_REG_MAC_CSR3: c_uint = 0x0018;
pub const OWL_EMAC_REG_MAC_CSR4: c_uint = 0x0020;
// Status register
pub const OWL_EMAC_REG_MAC_CSR5: c_uint = 0x0028;

pub const OWL_EMAC_OFF_MAC_CSR5_TS: c_int = 20;
pub const OWL_EMAC_VAL_MAC_CSR5_TS_DATA: c_uint = 0x03	/* Transferring data HOST -> FIFO */;
pub const OWL_EMAC_VAL_MAC_CSR5_TS_CDES: c_uint = 0x07	/* Closing transmit descriptor */;

pub const OWL_EMAC_OFF_MAC_CSR5_RS: c_int = 17;
pub const OWL_EMAC_VAL_MAC_CSR5_RS_FDES: c_uint = 0x01	/* Fetching receive descriptor */;
pub const OWL_EMAC_VAL_MAC_CSR5_RS_CDES: c_uint = 0x05	/* Closing receive descriptor */;
pub const OWL_EMAC_VAL_MAC_CSR5_RS_DATA: c_uint = 0x07	/* Transferring data FIFO -> HOST */;

// Operation mode register
pub const OWL_EMAC_REG_MAC_CSR6: c_uint = 0x0030;

pub const OWL_EMAC_OFF_MAC_CSR6_SPEED: c_int = 16;
pub const OWL_EMAC_VAL_MAC_CSR6_SPEED_100M: c_uint = 0x00;
pub const OWL_EMAC_VAL_MAC_CSR6_SPEED_10M: c_uint = 0x02;

// Interrupt enable register
pub const OWL_EMAC_REG_MAC_CSR7: c_uint = 0x0038;

// Missed frames and overflow counter register
pub const OWL_EMAC_REG_MAC_CSR8: c_uint = 0x0040;
// MII management and serial ROM register
pub const OWL_EMAC_REG_MAC_CSR9: c_uint = 0x0048;
// MII serial management register
pub const OWL_EMAC_REG_MAC_CSR10: c_uint = 0x0050;

pub const OWL_EMAC_OFF_MAC_CSR10_CLKDIV: c_int = 28;
pub const OWL_EMAC_VAL_MAC_CSR10_CLKDIV_128: c_uint = 0x04;
pub const OWL_EMAC_VAL_MAC_CSR10_OPCODE_WR: c_uint = 0x01	/* Register write command */;

pub const OWL_EMAC_VAL_MAC_CSR10_OPCODE_DCG: c_uint = 0x00	/* Disable clock generation */;
pub const OWL_EMAC_VAL_MAC_CSR10_OPCODE_WR: c_uint = 0x01	/* Register write command */;
pub const OWL_EMAC_VAL_MAC_CSR10_OPCODE_RD: c_uint = 0x02	/* Register read command */;
pub const OWL_EMAC_VAL_MAC_CSR10_OPCODE_CDS: c_uint = 0x03	/* Clock divider set */;

pub const OWL_EMAC_OFF_MAC_CSR10_PHYADD: c_int = 21;

pub const OWL_EMAC_OFF_MAC_CSR10_REGADD: c_int = 16;

// General-purpose timer and interrupt mitigation control register
pub const OWL_EMAC_REG_MAC_CSR11: c_uint = 0x0058;

// MAC address low/high registers
pub const OWL_EMAC_REG_MAC_CSR16: c_uint = 0x0080;
pub const OWL_EMAC_REG_MAC_CSR17: c_uint = 0x0088;
// Pause time & cache thresholds register
pub const OWL_EMAC_REG_MAC_CSR18: c_uint = 0x0090;

// FIFO pause & restart threshold register
pub const OWL_EMAC_REG_MAC_CSR19: c_uint = 0x0098;

// Flow control setup & status register
pub const OWL_EMAC_REG_MAC_CSR20: c_uint = 0x00A0;

// MII control register
pub const OWL_EMAC_REG_MAC_CTRL: c_uint = 0x00B0;

// Receive descriptor status field

pub const OWL_EMAC_OFF_RDES0_FL: c_int = 16;

// Receive descriptor control and count field

// Transmit descriptor status field

// Transmit descriptor control and count field

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum owl_emac_clk_map {
    OWL_EMAC_CLK_ETH = 0,
    OWL_EMAC_CLK_RMII
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_emac_addr_list {
    pub addrs: [u8; OWL_EMAC_MAX_MULTICAST_ADDRS][ETH_ALEN],
    pub count: c_int,
}

// TX/RX descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_emac_ring_desc {
    pub status: u32,
    pub control: u32,
    pub buf_addr: u32,
    pub /: *mut *mut u32 reserved; / 2nd buffer address is not used,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_emac_ring {
    pub descs: *mut owl_emac_ring_desc,
    pub descs_dma: dma_addr_t,
    pub skbs: *mut sk_buff,
    pub skbs_dma: *mut dma_addr_t,
    pub size: c_uint,
    pub head: c_uint,
    pub tail: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_emac_priv {
    pub netdev: *mut net_device,
    pub base: *mut void __iomem,
    pub clks: [clk_bulk_data; OWL_EMAC_NCLKS],
    pub reset: *mut reset_control,
    pub rx_ring: owl_emac_ring,
    pub tx_ring: owl_emac_ring,
    pub mii: *mut mii_bus,
    pub napi: napi_struct,
    pub phy_mode: phy_interface_t,
    pub link: c_uint,
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,
    pub mcaddr_list: owl_emac_addr_list,
    pub mac_reset_task: work_struct,
    pub /: *mut *mut u32 msg_enable; / Debug message level,
    pub /: *mut *mut spinlock_t lock; / Sync concurrent ring access,
}
