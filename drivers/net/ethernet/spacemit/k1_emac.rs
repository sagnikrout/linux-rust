//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/spacemit/k1_emac.h
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
//
// SpacemiT K1 Ethernet hardware definitions
//
// Copyright (C) 2023-2025 SpacemiT (Hangzhou) Technology Co. Ltd
// Copyright (C) 2025 Vivian Wang <wangruikang@iscas.ac.cn>
//

// APMU syscon registers
pub const APMU_EMAC_CTRL_REG: c_uint = 0x0;

//
// Only valid for RMII mode
// 0: Ref clock from External PHY
// 1: Ref clock from SoC
//

//
// Function clock select
// 0: 208 MHz
// 1: 312 MHz
//

// Only valid for RMII, invert TX clk

// Only valid for RMII, invert RX clk

//
// Only valid for RGMII
// 0: TX clk from RX clk
// 1: TX clk from SoC
//

pub const APMU_EMAC_DLINE_REG: c_uint = 0x4;

// DMA register set
pub const DMA_CONFIGURATION: c_uint = 0x0000;
pub const DMA_CONTROL: c_uint = 0x0004;
pub const DMA_STATUS_IRQ: c_uint = 0x0008;
pub const DMA_INTERRUPT_ENABLE: c_uint = 0x000c;
pub const DMA_TRANSMIT_AUTO_POLL_COUNTER: c_uint = 0x0010;
pub const DMA_TRANSMIT_POLL_DEMAND: c_uint = 0x0014;
pub const DMA_RECEIVE_POLL_DEMAND: c_uint = 0x0018;
pub const DMA_TRANSMIT_BASE_ADDRESS: c_uint = 0x001c;
pub const DMA_RECEIVE_BASE_ADDRESS: c_uint = 0x0020;
pub const DMA_MISSED_FRAME_COUNTER: c_uint = 0x0024;
pub const DMA_STOP_FLUSH_COUNTER: c_uint = 0x0028;
pub const DMA_RECEIVE_IRQ_MITIGATION_CTRL: c_uint = 0x002c;
pub const DMA_CURRENT_TRANSMIT_DESCRIPTOR_POINTER: c_uint = 0x0030;
pub const DMA_CURRENT_TRANSMIT_BUFFER_POINTER: c_uint = 0x0034;
pub const DMA_CURRENT_RECEIVE_DESCRIPTOR_POINTER: c_uint = 0x0038;
pub const DMA_CURRENT_RECEIVE_BUFFER_POINTER: c_uint = 0x003c;
// MAC Register set
pub const MAC_GLOBAL_CONTROL: c_uint = 0x0100;
pub const MAC_TRANSMIT_CONTROL: c_uint = 0x0104;
pub const MAC_RECEIVE_CONTROL: c_uint = 0x0108;
pub const MAC_MAXIMUM_FRAME_SIZE: c_uint = 0x010c;
pub const MAC_TRANSMIT_JABBER_SIZE: c_uint = 0x0110;
pub const MAC_RECEIVE_JABBER_SIZE: c_uint = 0x0114;
pub const MAC_ADDRESS_CONTROL: c_uint = 0x0118;
pub const MAC_MDIO_CLK_DIV: c_uint = 0x011c;
pub const MAC_ADDRESS1_HIGH: c_uint = 0x0120;
pub const MAC_ADDRESS1_MED: c_uint = 0x0124;
pub const MAC_ADDRESS1_LOW: c_uint = 0x0128;
pub const MAC_ADDRESS2_HIGH: c_uint = 0x012c;
pub const MAC_ADDRESS2_MED: c_uint = 0x0130;
pub const MAC_ADDRESS2_LOW: c_uint = 0x0134;
pub const MAC_ADDRESS3_HIGH: c_uint = 0x0138;
pub const MAC_ADDRESS3_MED: c_uint = 0x013c;
pub const MAC_ADDRESS3_LOW: c_uint = 0x0140;
pub const MAC_ADDRESS4_HIGH: c_uint = 0x0144;
pub const MAC_ADDRESS4_MED: c_uint = 0x0148;
pub const MAC_ADDRESS4_LOW: c_uint = 0x014c;
pub const MAC_MULTICAST_HASH_TABLE1: c_uint = 0x0150;
pub const MAC_MULTICAST_HASH_TABLE2: c_uint = 0x0154;
pub const MAC_MULTICAST_HASH_TABLE3: c_uint = 0x0158;
pub const MAC_MULTICAST_HASH_TABLE4: c_uint = 0x015c;
pub const MAC_FC_CONTROL: c_uint = 0x0160;
pub const MAC_FC_PAUSE_FRAME_GENERATE: c_uint = 0x0164;
pub const MAC_FC_SOURCE_ADDRESS_HIGH: c_uint = 0x0168;
pub const MAC_FC_SOURCE_ADDRESS_MED: c_uint = 0x016c;
pub const MAC_FC_SOURCE_ADDRESS_LOW: c_uint = 0x0170;
pub const MAC_FC_DESTINATION_ADDRESS_HIGH: c_uint = 0x0174;
pub const MAC_FC_DESTINATION_ADDRESS_MED: c_uint = 0x0178;
pub const MAC_FC_DESTINATION_ADDRESS_LOW: c_uint = 0x017c;
pub const MAC_FC_PAUSE_TIME_VALUE: c_uint = 0x0180;
pub const MAC_FC_HIGH_PAUSE_TIME: c_uint = 0x0184;
pub const MAC_FC_LOW_PAUSE_TIME: c_uint = 0x0188;
pub const MAC_FC_PAUSE_HIGH_THRESHOLD: c_uint = 0x018c;
pub const MAC_FC_PAUSE_LOW_THRESHOLD: c_uint = 0x0190;
pub const MAC_MDIO_CONTROL: c_uint = 0x01a0;
pub const MAC_MDIO_DATA: c_uint = 0x01a4;
pub const MAC_RX_STATCTR_CONTROL: c_uint = 0x01a8;
pub const MAC_RX_STATCTR_DATA_HIGH: c_uint = 0x01ac;
pub const MAC_RX_STATCTR_DATA_LOW: c_uint = 0x01b0;
pub const MAC_TX_STATCTR_CONTROL: c_uint = 0x01b4;
pub const MAC_TX_STATCTR_DATA_HIGH: c_uint = 0x01b8;
pub const MAC_TX_STATCTR_DATA_LOW: c_uint = 0x01bc;
pub const MAC_TRANSMIT_FIFO_ALMOST_FULL: c_uint = 0x01c0;
pub const MAC_TRANSMIT_PACKET_START_THRESHOLD: c_uint = 0x01c4;
pub const MAC_RECEIVE_PACKET_START_THRESHOLD: c_uint = 0x01c8;
pub const MAC_STATUS_IRQ: c_uint = 0x01e0;
pub const MAC_INTERRUPT_ENABLE: c_uint = 0x01e4;
// Used for register dump
pub const EMAC_DMA_REG_CNT: c_int = 16;
pub const EMAC_MAC_REG_CNT: c_int = 124;
// DMA_CONFIGURATION (0x0000)
//
// 0-DMA controller in normal operation mode,
// 1-DMA controller reset to default state,
// clearing all internal state information
//

// For Receive and Transmit DMA operate in Big-Endian mode for Descriptors.

// DMA_CONTROL (0x0004)

// DMA_STATUS_IRQ (0x0008)

// DMA_INTERRUPT_ENABLE (0x000c)

// DMA_RECEIVE_IRQ_MITIGATION_CTRL (0x002c)

// MAC_GLOBAL_CONTROL (0x0100)

pub const MREGBIT_SPEED_10M: c_uint = 0x0;

// MAC_TRANSMIT_CONTROL (0x0104)

// MAC_RECEIVE_CONTROL (0x0108)

// MAC_MAXIMUM_FRAME_SIZE (0x010c)

// MAC_TRANSMIT_JABBER_SIZE (0x0110)

// MAC_RECEIVE_JABBER_SIZE (0x0114)

// MAC_ADDRESS_CONTROL (0x0118)

// MAC_FC_CONTROL (0x0160)

// MAC_FC_PAUSE_FRAME_GENERATE (0x0164)

// MAC_FC_PAUSE_TIME_VALUE (0x0180)

// MAC_MDIO_CONTROL (0x01a0)

// MAC_MDIO_DATA (0x01a4)

// MAC_RX_STATCTR_CONTROL (0x01a8)

// MAC_RX_STATCTR_DATA_HIGH (0x01ac)

// MAC_RX_STATCTR_DATA_LOW (0x01b0)

// MAC_TX_STATCTR_CONTROL (0x01b4)

// MAC_TX_STATCTR_DATA_HIGH (0x01b8)

// MAC_TX_STATCTR_DATA_LOW (0x01bc)

// MAC_TRANSMIT_FIFO_ALMOST_FULL (0x01c0)

// MAC_TRANSMIT_PACKET_START_THRESHOLD (0x01c4)

// MAC_RECEIVE_PACKET_START_THRESHOLD (0x01c8)

// MAC_STATUS_IRQ (0x01e0)

// MAC_INTERRUPT_ENABLE (0x01e4)

// RX DMA descriptor

// [24] reserved

// [29:27] reserved

// TX DMA descriptor
// [29:0] unused

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_desc {
    pub desc0: u32,
    pub desc1: u32,
    pub buffer_addr_1: u32,
    pub buffer_addr_2: u32,
}

// Keep stats in this order, index used for accessing hardware
#[repr(C)]
#[derive(Copy, Clone)]
pub union emac_hw_tx_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct individual_tx_stats {
    pub tx_ok_pkts: u64,
    pub tx_total_pkts: u64,
    pub tx_ok_bytes: u64,
    pub tx_err_pkts: u64,
    pub tx_singleclsn_pkts: u64,
    pub tx_multiclsn_pkts: u64,
    pub tx_lateclsn_pkts: u64,
    pub tx_excessclsn_pkts: u64,
    pub tx_unicast_pkts: u64,
    pub tx_multicast_pkts: u64,
    pub tx_broadcast_pkts: u64,
    pub tx_pause_pkts: u64,
    pub stats: },
    pub sizeof(u64)]: u64 array[sizeof(struct individual_tx_stats) /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union emac_hw_rx_stats {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct individual_rx_stats {
    pub rx_ok_pkts: u64,
    pub rx_total_pkts: u64,
    pub rx_crc_err_pkts: u64,
    pub rx_align_err_pkts: u64,
    pub rx_err_total_pkts: u64,
    pub rx_ok_bytes: u64,
    pub rx_total_bytes: u64,
    pub rx_unicast_pkts: u64,
    pub rx_multicast_pkts: u64,
    pub rx_broadcast_pkts: u64,
    pub rx_pause_pkts: u64,
    pub rx_len_err_pkts: u64,
    pub rx_len_undersize_pkts: u64,
    pub rx_len_oversize_pkts: u64,
    pub rx_len_fragment_pkts: u64,
    pub rx_len_jabber_pkts: u64,
    pub rx_64_pkts: u64,
    pub rx_65_127_pkts: u64,
    pub rx_128_255_pkts: u64,
    pub rx_256_511_pkts: u64,
    pub rx_512_1023_pkts: u64,
    pub rx_1024_1518_pkts: u64,
    pub rx_1519_plus_pkts: u64,
    pub rx_drp_fifo_full_pkts: u64,
    pub rx_truncate_fifo_full_pkts: u64,
    pub stats: },
    pub sizeof(u64)]: u64 array[sizeof(struct individual_rx_stats) /,
}
