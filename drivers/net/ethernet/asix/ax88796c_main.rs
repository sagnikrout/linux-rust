//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/asix/ax88796c_main.h
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
// Copyright (c) 2010 ASIX Electronics Corporation
// Copyright (c) 2020 Samsung Electronics
//
// ASIX AX88796C SPI Fast Ethernet Linux driver
//

// These identify the driver base version and may not be removed.

pub const AX88796C_REGDUMP_LEN: c_int = 256;
pub const AX88796C_PHY_REGDUMP_LEN: c_int = 14;
pub const AX88796C_PHY_ID: c_uint = 0x10;

pub const TX_EOP_SIZE: c_int = 4;
pub const AX_MCAST_FILTER_SIZE: c_int = 8;
pub const AX_MAX_MCAST: c_int = 64;
pub const AX_MAX_CLK: c_int = 80000000;
pub const TX_HDR_SOP_DICF: c_uint = 0x8000;
pub const TX_HDR_SOP_CPHI: c_uint = 0x4000;
pub const TX_HDR_SOP_INT: c_uint = 0x2000;
pub const TX_HDR_SOP_MDEQ: c_uint = 0x1000;
pub const TX_HDR_SOP_PKTLEN: c_uint = 0x07FF;
pub const TX_HDR_SOP_SEQNUM: c_uint = 0xF800;
pub const TX_HDR_SOP_PKTLENBAR: c_uint = 0x07FF;
pub const TX_HDR_SEG_FS: c_uint = 0x8000;
pub const TX_HDR_SEG_LS: c_uint = 0x4000;
pub const TX_HDR_SEG_SEGNUM: c_uint = 0x3800;
pub const TX_HDR_SEG_SEGLEN: c_uint = 0x0700;
pub const TX_HDR_SEG_EOFST: c_uint = 0xC000;
pub const TX_HDR_SEG_SOFST: c_uint = 0x3800;
pub const TX_HDR_SEG_SEGLENBAR: c_uint = 0x07FF;
pub const TX_HDR_EOP_SEQNUM: c_uint = 0xF800;
pub const TX_HDR_EOP_PKTLEN: c_uint = 0x07FF;
pub const TX_HDR_EOP_SEQNUMBAR: c_uint = 0xF800;
pub const TX_HDR_EOP_PKTLENBAR: c_uint = 0x07FF;
// Rx header fields mask
pub const RX_HDR1_MCBC: c_uint = 0x8000;
pub const RX_HDR1_STUFF_PKT: c_uint = 0x4000;
pub const RX_HDR1_MII_ERR: c_uint = 0x2000;
pub const RX_HDR1_CRC_ERR: c_uint = 0x1000;
pub const RX_HDR1_PKT_LEN: c_uint = 0x07FF;
pub const RX_HDR2_SEQ_NUM: c_uint = 0xF800;
pub const RX_HDR2_PKT_LEN_BAR: c_uint = 0x7FFF;
pub const RX_HDR3_PE: c_uint = 0x8000;
pub const RX_HDR3_L3_TYPE_IPV4V6: c_uint = 0x6000;
pub const RX_HDR3_L3_TYPE_IP: c_uint = 0x4000;
pub const RX_HDR3_L3_TYPE_IPV6: c_uint = 0x2000;
pub const RX_HDR3_L4_TYPE_ICMPV6: c_uint = 0x1400;
pub const RX_HDR3_L4_TYPE_TCP: c_uint = 0x1000;
pub const RX_HDR3_L4_TYPE_IGMP: c_uint = 0x0c00;
pub const RX_HDR3_L4_TYPE_ICMP: c_uint = 0x0800;
pub const RX_HDR3_L4_TYPE_UDP: c_uint = 0x0400;
pub const RX_HDR3_L3_ERR: c_uint = 0x0200;
pub const RX_HDR3_L4_ERR: c_uint = 0x0100;

pub const RX_HDR3_STRIP: c_uint = 0x0008;
pub const RX_HDR3_VLAN_ID: c_uint = 0x0007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax88796c_pcpu_stats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub rx_frame_errors: u32,
    pub rx_crc_errors: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax88796c_device {
    pub spi: *mut spi_device,
    pub ndev: *mut net_device,
    pub stats: *mut ax88796c_pcpu_stats __percpu,
    pub ax_work: work_struct,
    pub /: *mut *mut mutex spi_lock; / device access,
    pub tx_wait_q: sk_buff_head,
    pub ax_spi: axspi_data,
    pub mdiobus: *mut mii_bus,
    pub phydev: *mut phy_device,
    pub msg_enable: c_int,
    pub seq_num: u16,
    pub multi_filter: [u8; AX_MCAST_FILTER_SIZE],
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: c_int,
    pub asym_pause: c_int,
    pub flowctrl: c_int,
pub const AX_FC_NONE: c_int = 0;

    pub priv_flags: u32,

    pub flags: c_ulong,
pub const EVENT_INTR: c_int = 0;
pub const EVENT_TX: c_int = 1;
pub const EVENT_SET_MULTI: c_int = 2;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum skb_state {
    illegal = 0,
    tx_done,
    rx_done,
    rx_err,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_data {
    pub state: skb_state,
    pub len: usize,
}

// A88796C register definition
// Definition of PAGE0

// PS control

// WOL PS

// Active high

// Active low

pub const MACCR_RXFC_MASK: c_uint = 0xFFF7;

pub const MACCR_TXFC_MASK: c_uint = 0xFFEF;

// Power Saving Interrupt

pub const MACCR_PMM_BITS: c_int = 8;

pub const TFBFCR_SCHE_FREE_PAGE: c_uint = 0xE07F;
pub const TFBFCR_FREE_PAGE_BITS: c_uint = 0x07;

pub const TX_FREEBUF_MASK: c_uint = 0x003F;
pub const TX_DPTSTART: c_uint = 0x4000;

// Definition of PAGE1

// Definition of PAGE2

// Definition of PAGE3

pub const MACASR_LOWBYTE_MASK: c_uint = 0x00FF;
pub const MACASR_HIGH_BITS: c_uint = 0x08;

// Definition of PAGE4

// Definition of PAGE5

// Definition of PAGE6

// Definition of PAGE7

// Tx headers structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_sop_header {
// bit 15-11: flags, bit 10-0: packet length
    pub flags_len: u16,
// bit 15-11: sequence number, bit 11-0: packet length bar
    pub seq_lenbar: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_segment_header {
// bit 15-14: flags, bit 13-11: segment number
// bit 10-0: segment length
    pub flags_seqnum_seglen: u16,
// bit 15-14: end offset, bit 13-11: start offset
// bit 10-0: segment length bar
    pub eo_so_seglenbar: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_eop_header {
// bit 15-11: sequence number, bit 10-0: packet length
    pub seq_len: u16,
// bit 15-11: sequence number bar, bit 10-0: packet length bar
    pub seqbar_lenbar: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_pkt_info {
    pub sop: tx_sop_header,
    pub seg: tx_segment_header,
    pub eop: tx_eop_header,
    pub pkt_len: u16,
    pub seq_num: u16,
}

// Rx headers structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_header {
    pub flags_len: u16,
    pub seq_lenbar: u16,
    pub flags: u16,
}
