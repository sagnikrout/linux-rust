//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac1000.h
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

pub const GMAC_CONTROL: c_uint = 0x00000000	/* Configuration */;
pub const GMAC_FRAME_FILTER: c_uint = 0x00000004	/* Frame Filter */;
pub const GMAC_HASH_HIGH: c_uint = 0x00000008	/* Multicast Hash Table High */;
pub const GMAC_HASH_LOW: c_uint = 0x0000000c	/* Multicast Hash Table Low */;
pub const GMAC_MII_ADDR: c_uint = 0x00000010	/* MII Address */;
pub const GMAC_MII_DATA: c_uint = 0x00000014	/* MII Data */;
pub const GMAC_FLOW_CTRL: c_uint = 0x00000018	/* Flow Control */;
pub const GMAC_VLAN_TAG: c_uint = 0x0000001c	/* VLAN Tag */;
pub const GMAC_DEBUG: c_uint = 0x00000024	/* GMAC debug register */;
pub const GMAC_INT_STATUS: c_uint = 0x00000038	/* interrupt status register */;

// interrupt mask register
pub const GMAC_INT_MASK: c_uint = 0x0000003c;

// PMT Control and Status
pub const GMAC_PMT: c_uint = 0x0000002c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_event {
    pointer_reset = 0x80000000,
    global_unicast = 0x00000200,
    wake_up_rx_frame = 0x00000040,
    magic_frame = 0x00000020,
    wake_up_frame_en = 0x00000004,
    magic_pkt_en = 0x00000002,
    power_down = 0x00000001,
}

// Energy Efficient Ethernet (EEE)
//
// LPI status, timer and control register offset
// For LPI control and status bit definitions, see common.h.
//
pub const LPI_CTRL_STATUS: c_uint = 0x0030;
pub const LPI_TIMER_CTRL: c_uint = 0x0034;
// GMAC HW ADDR regs

pub const GMAC_MAX_PERFECT_ADDRESSES: c_int = 1;
pub const GMAC_PCS_BASE: c_uint = 0x000000c0	/* PCS register base */;
pub const GMAC_RGSMIIIS: c_uint = 0x000000d8	/* RGMII/SMII status */;
// SGMII/RGMII status register

// GMAC Configuration defines
pub const GMAC_CONTROL_2K: c_uint = 0x08000000	/* IEEE 802.3as 2K packets */;
pub const GMAC_CONTROL_JD: c_uint = 0x00400000	/* Jabber disable */;
pub const GMAC_CONTROL_BE: c_uint = 0x00200000	/* Frame Burst Enable */;
pub const GMAC_CONTROL_JE: c_uint = 0x00100000	/* Jumbo frame */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inter_frame_gap {
    GMAC_CONTROL_IFG_88 = 0x00040000,
    GMAC_CONTROL_IFG_80 = 0x00020000,
    GMAC_CONTROL_IFG_40 = 0x000e0000,
}

pub const GMAC_CONTROL_DCRS: c_uint = 0x00010000	/* Disable carrier sense */;
pub const GMAC_CONTROL_PS: c_uint = 0x00008000	/* Port Select 0:GMI 1:MII */;
pub const GMAC_CONTROL_FES: c_uint = 0x00004000	/* Speed 0:10 1:100 */;
pub const GMAC_CONTROL_LM: c_uint = 0x00001000	/* Loop-back mode */;
pub const GMAC_CONTROL_DM: c_uint = 0x00000800	/* Duplex Mode */;
pub const GMAC_CONTROL_IPC: c_uint = 0x00000400	/* Checksum Offload */;

// GMAC Frame Filter defines
pub const GMAC_FRAME_FILTER_PR: c_uint = 0x00000001	/* Promiscuous Mode */;
pub const GMAC_FRAME_FILTER_HMC: c_uint = 0x00000004	/* Hash Multicast */;
pub const GMAC_FRAME_FILTER_PM: c_uint = 0x00000010	/* Pass all multicast */;
pub const GMAC_FRAME_FILTER_PCF: c_uint = 0x00000080	/* Pass Control frames */;
pub const GMAC_FRAME_FILTER_HPF: c_uint = 0x00000400	/* Hash or perfect Filter */;
pub const GMAC_FRAME_FILTER_RA: c_uint = 0x80000000	/* Receive all mode */;
// GMAC FLOW CTRL defines

pub const GMAC_FLOW_CTRL_UP: c_uint = 0x00000008	/* Unicast pause frame enable */;
pub const GMAC_FLOW_CTRL_RFE: c_uint = 0x00000004	/* Rx Flow Control Enable */;
pub const GMAC_FLOW_CTRL_TFE: c_uint = 0x00000002	/* Tx Flow Control Enable */;
// DEBUG Register defines
// MTL TxStatus FIFO

// MTL Tx FIFO Read Controller Status

pub const GMAC_DEBUG_TRCSTS_READ: c_int = 1;
pub const GMAC_DEBUG_TRCSTS_TXW: c_int = 2;
pub const GMAC_DEBUG_TRCSTS_WRITE: c_int = 3;

// MAC Transmit Frame Controller Status

pub const GMAC_DEBUG_TFCSTS_WAIT: c_int = 1;
pub const GMAC_DEBUG_TFCSTS_GEN_PAUSE: c_int = 2;
pub const GMAC_DEBUG_TFCSTS_XFER: c_int = 3;
// MAC GMII or MII Transmit Protocol Engine Status

pub const GMAC_DEBUG_RXFSTS_EMPTY: c_int = 0;
pub const GMAC_DEBUG_RXFSTS_BT: c_int = 1;
pub const GMAC_DEBUG_RXFSTS_AT: c_int = 2;
pub const GMAC_DEBUG_RXFSTS_FULL: c_int = 3;

pub const GMAC_DEBUG_RRCSTS_IDLE: c_int = 0;
pub const GMAC_DEBUG_RRCSTS_RDATA: c_int = 1;
pub const GMAC_DEBUG_RRCSTS_RSTAT: c_int = 2;
pub const GMAC_DEBUG_RRCSTS_FLUSH: c_int = 3;

// MAC Receive Frame Controller FIFO Status

// MAC GMII or MII Receive Protocol Engine Status

// --- DMA BLOCK defines ---
// DMA Bus Mode register defines
// Programmable burst length (passed through platform)

pub const DMA_BUS_MODE_ATDS: c_uint = 0x00000080	/* Alternate Descriptor Size */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_tx_priority_ratio {
    double_ratio = 0x00004000,	/* 2:1 */
    triple_ratio = 0x00008000,	/* 3:1 */
    quadruple_ratio = 0x0000c000,	/* 4:1 */
}

pub const DMA_BUS_MODE_FB: c_uint = 0x00010000	/* Fixed burst */;
pub const DMA_BUS_MODE_MB: c_uint = 0x04000000	/* Mixed burst */;

pub const DMA_BUS_MODE_USP: c_uint = 0x00800000;
pub const DMA_BUS_MODE_MAXPBL: c_uint = 0x01000000;
pub const DMA_BUS_MODE_AAL: c_uint = 0x02000000;
// DMA CRS Control and Status Register Mapping
// DMA operation mode defines (start/stop tx/rx are placed in common header)
// Disable Drop TCP/IP csum error
pub const DMA_CONTROL_RSF: c_uint = 0x02000000	/* Receive Store and Forward */;
pub const DMA_CONTROL_DFF: c_uint = 0x01000000	/* Disaable flushing */;
// Threshold for Activating the FC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rfa {
    act_full_minus_1 = 0x00800000,
    act_full_minus_2 = 0x00800200,
    act_full_minus_3 = 0x00800400,
    act_full_minus_4 = 0x00800600,
}

// Threshold for Deactivating the FC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rfd {
    deac_full_minus_1 = 0x00400000,
    deac_full_minus_2 = 0x00400800,
    deac_full_minus_3 = 0x00401000,
    deac_full_minus_4 = 0x00401800,
}

pub const DMA_CONTROL_TSF: c_uint = 0x00200000	/* Transmit  Store and Forward */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ttc_control {
    DMA_CONTROL_TTC_64 = 0x00000000,
    DMA_CONTROL_TTC_128 = 0x00004000,
    DMA_CONTROL_TTC_192 = 0x00008000,
    DMA_CONTROL_TTC_256 = 0x0000c000,
    DMA_CONTROL_TTC_40 = 0x00010000,
    DMA_CONTROL_TTC_32 = 0x00014000,
    DMA_CONTROL_TTC_24 = 0x00018000,
    DMA_CONTROL_TTC_16 = 0x0001c000,
}

pub const DMA_CONTROL_TC_TX_MASK: c_uint = 0xfffe3fff;
pub const DMA_CONTROL_EFC: c_uint = 0x00000100;
// Receive flow control activation field
// RFA field in DMA control register, bits 23,10:9
//
pub const DMA_CONTROL_RFA_MASK: c_uint = 0x00800600;
// Receive flow control deactivation field
// RFD field in DMA control register, bits 22,12:11
//
pub const DMA_CONTROL_RFD_MASK: c_uint = 0x00401800;
// RFD and RFA fields are encoded as follows
//
// Bit Field
// 0,00 - Full minus 1KB (only valid when rxfifo >= 4KB and EFC enabled)
// 0,01 - Full minus 2KB (only valid when rxfifo >= 4KB and EFC enabled)
// 0,10 - Full minus 3KB (only valid when rxfifo >= 4KB and EFC enabled)
// 0,11 - Full minus 4KB (only valid when rxfifo > 4KB and EFC enabled)
// 1,00 - Full minus 5KB (only valid when rxfifo > 8KB and EFC enabled)
// 1,01 - Full minus 6KB (only valid when rxfifo > 8KB and EFC enabled)
// 1,10 - Full minus 7KB (only valid when rxfifo > 8KB and EFC enabled)
// 1,11 - Reserved
//
// RFD should always be > RFA for a given FIFO size. RFD == RFA may work,
// but packet throughput performance may not be as expected.
//
// Be sure that bit 3 in GMAC Register 6 is set for Unicast Pause frame
// detection (IEEE Specification Requirement, Annex 31B, 31B.1, Pause
// Description).
//
// Be sure that DZPA (bit 7 in Flow Control Register, GMAC Register 6),
// is set to 0. This allows pause frames with a quanta of 0 to be sent
// as an XOFF message to the link peer.
//
pub const RFA_FULL_MINUS_1K: c_uint = 0x00000000;
pub const RFD_FULL_MINUS_2K: c_uint = 0x00000800;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtc_control {
    DMA_CONTROL_RTC_64 = 0x00000000,
    DMA_CONTROL_RTC_32 = 0x00000008,
    DMA_CONTROL_RTC_96 = 0x00000010,
    DMA_CONTROL_RTC_128 = 0x00000018,
}

pub const DMA_CONTROL_TC_RX_MASK: c_uint = 0xffffffe7;
pub const DMA_CONTROL_OSF: c_uint = 0x00000004	/* Operate on second frame */;
// MMC registers offset
pub const GMAC_EXTHASH_BASE: c_uint = 0x500;
// PTP and timestamping registers

pub const GMAC3_X_TIMESTAMP_STATUS: c_uint = 0x28;
pub const GMAC_PTP_ATNR: c_uint = 0x30;
pub const GMAC_PTP_ATSR: c_uint = 0x34;
