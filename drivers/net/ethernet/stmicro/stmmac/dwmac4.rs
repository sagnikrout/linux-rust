//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac4.h
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
// DWMAC4 Header file.
//
// Copyright (C) 2015  STMicroelectronics Ltd
//
// Author: Alexandre Torgue <alexandre.torgue@st.com>
//

// MAC registers
pub const GMAC_CONFIG: c_uint = 0x00000000;
pub const GMAC_EXT_CONFIG: c_uint = 0x00000004;
pub const GMAC_PACKET_FILTER: c_uint = 0x00000008;

pub const GMAC_RX_FLOW_CTRL: c_uint = 0x00000090;

pub const GMAC_TXQ_PRTY_MAP0: c_uint = 0x98;
pub const GMAC_TXQ_PRTY_MAP1: c_uint = 0x9C;
pub const GMAC_RXQ_CTRL0: c_uint = 0x000000a0;
pub const GMAC_RXQ_CTRL1: c_uint = 0x000000a4;
pub const GMAC_RXQ_CTRL2: c_uint = 0x000000a8;
pub const GMAC_RXQ_CTRL3: c_uint = 0x000000ac;
pub const GMAC_INT_STATUS: c_uint = 0x000000b0;
pub const GMAC_INT_EN: c_uint = 0x000000b4;
pub const GMAC_PCS_BASE: c_uint = 0x000000e0;
pub const GMAC_PHYIF_CONTROL_STATUS: c_uint = 0x000000f8;
pub const GMAC_PMT: c_uint = 0x000000c0;
pub const GMAC_DEBUG: c_uint = 0x00000114;
pub const GMAC_HW_FEATURE0: c_uint = 0x0000011c;
pub const GMAC_HW_FEATURE1: c_uint = 0x00000120;
pub const GMAC_HW_FEATURE2: c_uint = 0x00000124;
pub const GMAC_HW_FEATURE3: c_uint = 0x00000128;
pub const GMAC_MDIO_ADDR: c_uint = 0x00000200;
pub const GMAC_MDIO_DATA: c_uint = 0x00000204;
pub const GMAC_GPIO_STATUS: c_uint = 0x0000020C;
pub const GMAC_ARP_ADDR: c_uint = 0x00000210;
pub const GMAC_EXT_CFG1: c_uint = 0x00000238;

pub const GMAC_TIMESTAMP_STATUS: c_uint = 0x00000b20;
// RX Queues Routing

pub const GMAC_RXQCTRL_AVCPQ_SHIFT: c_int = 0;

pub const GMAC_RXQCTRL_PTPQ_SHIFT: c_int = 4;

pub const GMAC_RXQCTRL_DCBCPQ_SHIFT: c_int = 8;

pub const GMAC_RXQCTRL_UPQ_SHIFT: c_int = 12;

pub const GMAC_RXQCTRL_MCBCQ_SHIFT: c_int = 16;

pub const GMAC_RXQCTRL_MCBCQEN_SHIFT: c_int = 20;

pub const GMAC_RXQCTRL_TACPQE_SHIFT: c_int = 21;

// MAC Packet Filtering

pub const GMAC_MAX_PERFECT_ADDRESSES: c_int = 128;
// MAC RX Queue Enable

// MAC Flow Control RX

// RX Queues Priorities

// TX Queues Priorities

// MAC Flow Control TX

// MAC Interrupt bitmap

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwmac4_irq_status {
    time_stamp_irq = 0x00001000,
    mmc_rx_csum_offload_irq = 0x00000800,
    mmc_tx_irq = 0x00000400,
    mmc_rx_irq = 0x00000200,
    mmc_irq = 0x00000100,
    lpi_irq = 0x00000020,
    pmt_irq = 0x00000010,
}

// MAC PMT bitmap
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_event {
    pointer_reset =	0x80000000,
    global_unicast = 0x00000200,
    wake_up_rx_frame = 0x00000040,
    magic_frame = 0x00000020,
    wake_up_frame_en = 0x00000004,
    magic_pkt_en = 0x00000002,
    power_down = 0x00000001,
}

// Energy Efficient Ethernet (EEE) for GMAC4
//
// LPI status, timer and control register offset
// For LPI control and status bit definitions, see common.h.
//
pub const GMAC4_LPI_CTRL_STATUS: c_uint = 0xd0;
pub const GMAC4_LPI_TIMER_CTRL: c_uint = 0xd4;
pub const GMAC4_LPI_ENTRY_TIMER: c_uint = 0xd8;
pub const GMAC4_MAC_ONEUS_TIC_COUNTER: c_uint = 0xdc;
// MAC Debug bitmap

pub const GMAC_DEBUG_TFCSTS_IDLE: c_int = 0;
pub const GMAC_DEBUG_TFCSTS_WAIT: c_int = 1;
pub const GMAC_DEBUG_TFCSTS_GEN_PAUSE: c_int = 2;
pub const GMAC_DEBUG_TFCSTS_XFER: c_int = 3;

// MAC config

// MAC extended config

// MAC HW features0 bitmap

// MAC HW features1 bitmap

// MAC HW features2 bitmap

// MAC HW features3 bitmap

// MAC extended config 1

// GMAC GPIO Status reg

// MAC HW ADDR regs

// L3/L4 Filters regs

// MAC Timestamp Status

pub const GMAC_TIMESTAMP_ATSNS_SHIFT: c_int = 25;
// MTL registers
pub const MTL_OPERATION_MODE: c_uint = 0x00000c00;

pub const MTL_INT_STATUS: c_uint = 0x00000c20;

pub const MTL_RXQ_DMA_MAP0: c_uint = 0x00000c30 /* queue 0 to 3 */;
pub const MTL_RXQ_DMA_MAP1: c_uint = 0x00000c34 /* queue 4 to 7 */;

pub const MTL_CHAN_BASE_ADDR: c_uint = 0x00000d00;
pub const MTL_CHAN_BASE_OFFSET: c_uint = 0x40;

// MTL ETS Control register
pub const MTL_ETS_CTRL_BASE_ADDR: c_uint = 0x00000d10;
pub const MTL_ETS_CTRL_BASE_OFFSET: c_uint = 0x40;

// MTL Queue Quantum Weight
pub const MTL_TXQ_WEIGHT_BASE_ADDR: c_uint = 0x00000d18;
pub const MTL_TXQ_WEIGHT_BASE_OFFSET: c_uint = 0x40;

// MTL sendSlopeCredit register
pub const MTL_SEND_SLP_CRED_BASE_ADDR: c_uint = 0x00000d1c;
pub const MTL_SEND_SLP_CRED_OFFSET: c_uint = 0x40;

// MTL hiCredit register
pub const MTL_HIGH_CRED_BASE_ADDR: c_uint = 0x00000d20;
pub const MTL_HIGH_CRED_OFFSET: c_uint = 0x40;

// MTL loCredit register
pub const MTL_LOW_CRED_BASE_ADDR: c_uint = 0x00000d24;
pub const MTL_LOW_CRED_OFFSET: c_uint = 0x40;

// MTL debug

// MTL debug: Tx FIFO Read Controller Status

pub const MTL_DEBUG_TRCSTS_IDLE: c_int = 0;
pub const MTL_DEBUG_TRCSTS_READ: c_int = 1;
pub const MTL_DEBUG_TRCSTS_TXW: c_int = 2;
pub const MTL_DEBUG_TRCSTS_WRITE: c_int = 3;

// MAC debug: GMII or MII Transmit Protocol Engine Status

pub const MTL_DEBUG_RXFSTS_EMPTY: c_int = 0;
pub const MTL_DEBUG_RXFSTS_BT: c_int = 1;
pub const MTL_DEBUG_RXFSTS_AT: c_int = 2;
pub const MTL_DEBUG_RXFSTS_FULL: c_int = 3;

pub const MTL_DEBUG_RRCSTS_IDLE: c_int = 0;
pub const MTL_DEBUG_RRCSTS_RDATA: c_int = 1;
pub const MTL_DEBUG_RRCSTS_RSTAT: c_int = 2;
pub const MTL_DEBUG_RRCSTS_FLUSH: c_int = 3;

// MTL interrupt

// Default operating mode of the MAC

// To dump the core regs excluding  the Address Registers
pub const GMAC_REG_NUM: c_int = 132;
// SGMII/RGMII status register

