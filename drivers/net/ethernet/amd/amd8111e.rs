//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/amd8111e.h
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
// Advanced  Micro Devices Inc. AMD8111E Linux Network Driver
// Copyright (C) 2003 Advanced Micro Devices
//
// Command style register access
//
// Offset for Memory Mapped Registers.
// 32 bit registers
pub const ASF_STAT: c_uint = 0x00	/* ASF status register */;
pub const CHIPID: c_uint = 0x04	/* Chip ID register */;
pub const MIB_DATA: c_uint = 0x10	/* MIB data register */;
pub const MIB_ADDR: c_uint = 0x14	/* MIB address register */;
pub const STAT0: c_uint = 0x30	/* Status0 register */;
pub const INT0: c_uint = 0x38	/* Interrupt0 register */;
pub const INTEN0: c_uint = 0x40	/* Interrupt0  enable register*/;
pub const CMD0: c_uint = 0x48	/* Command0 register */;
pub const CMD2: c_uint = 0x50	/* Command2 register */;
pub const CMD3: c_uint = 0x54	/* Command3 resiter */;
pub const CMD7: c_uint = 0x64	/* Command7 register */;
pub const CTRL1: c_uint = 0x6C	/* Control1 register */;
pub const CTRL2: c_uint = 0x70	/* Control2 register */;
pub const XMT_RING_LIMIT: c_uint = 0x7C	/* Transmit ring limit register */;
pub const AUTOPOLL0: c_uint = 0x88	/* Auto-poll0 register */;
pub const AUTOPOLL1: c_uint = 0x8A	/* Auto-poll1 register */;
pub const AUTOPOLL2: c_uint = 0x8C	/* Auto-poll2 register */;
pub const AUTOPOLL3: c_uint = 0x8E	/* Auto-poll3 register */;
pub const AUTOPOLL4: c_uint = 0x90	/* Auto-poll4 register */;
pub const AUTOPOLL5: c_uint = 0x92	/* Auto-poll5 register */;
pub const AP_VALUE: c_uint = 0x98	/* Auto-poll value register */;
pub const DLY_INT_A: c_uint = 0xA8	/* Group A delayed interrupt register */;
pub const DLY_INT_B: c_uint = 0xAC	/* Group B delayed interrupt register */;
pub const FLOW_CONTROL: c_uint = 0xC8	/* Flow control register */;
pub const PHY_ACCESS: c_uint = 0xD0	/* PHY access register */;
pub const STVAL: c_uint = 0xD8	/* Software timer value register */;
pub const XMT_RING_BASE_ADDR0: c_uint = 0x100	/* Transmit ring0 base addr register */;
pub const XMT_RING_BASE_ADDR1: c_uint = 0x108	/* Transmit ring1 base addr register */;
pub const XMT_RING_BASE_ADDR2: c_uint = 0x110	/* Transmit ring2 base addr register */;
pub const XMT_RING_BASE_ADDR3: c_uint = 0x118	/* Transmit ring2 base addr register */;
pub const RCV_RING_BASE_ADDR0: c_uint = 0x120	/* Transmit ring0 base addr register */;
pub const PMAT0: c_uint = 0x190	/* OnNow pattern register0 */;
pub const PMAT1: c_uint = 0x194	/* OnNow pattern register1 */;
// 16bit registers
pub const XMT_RING_LEN0: c_uint = 0x140	/* Transmit Ring0 length register */;
pub const XMT_RING_LEN1: c_uint = 0x144	/* Transmit Ring1 length register */;
pub const XMT_RING_LEN2: c_uint = 0x148 	/* Transmit Ring2 length register */;
pub const XMT_RING_LEN3: c_uint = 0x14C	/* Transmit Ring3 length register */;
pub const RCV_RING_LEN0: c_uint = 0x150	/* Receive Ring0 length register */;
pub const SRAM_SIZE: c_uint = 0x178	/* SRAM size register */;
pub const SRAM_BOUNDARY: c_uint = 0x17A	/* SRAM boundary register */;
// 48bit register
pub const PADR: c_uint = 0x160	/* Physical address register */;
pub const IFS1: c_uint = 0x18C	/* Inter-frame spacing Part1 register */;
pub const IFS: c_uint = 0x18D	/* Inter-frame spacing register */;
pub const IPG: c_uint = 0x18E	/* Inter-frame gap register */;
// 64bit register
pub const LADRF: c_uint = 0x168	/* Logical address filter register */;
// Register Bit Definitions
pub const PHY_SPEED_10: c_uint = 0x2;
pub const PHY_SPEED_100: c_uint = 0x3;
// INT0				0x38, 32bit register
// VAL3
// VAL2
// VAL1
// VAL0
// VAL2
// VAL1
// VAL0
// VAL3
// VAL2
// VAL1
// VAL0
// VAL3
// VAL2
// VAL1
// VAL0
// XMT_RING_LIMIT		0x7C, 32bit register
// AUTOPOLL1			0x8A, 16bit register
// AP_VALUE 			0x98, 32bit ragister
// FLOW_CONTROL 		0xC8, 32bit register
// PHY_ ACCESS			0xD0, 32bit register
// PMAT0			0x190,	 32bit register
// PMAT1			0x194,	 32bit register
//
// MIB counter definitions
//
pub const rcv_miss_pkts: c_uint = 0x00;
pub const rcv_octets: c_uint = 0x01;
pub const rcv_broadcast_pkts: c_uint = 0x02;
pub const rcv_multicast_pkts: c_uint = 0x03;
pub const rcv_undersize_pkts: c_uint = 0x04;
pub const rcv_oversize_pkts: c_uint = 0x05;
pub const rcv_fragments: c_uint = 0x06;
pub const rcv_jabbers: c_uint = 0x07;
pub const rcv_unicast_pkts: c_uint = 0x08;
pub const rcv_alignment_errors: c_uint = 0x09;
pub const rcv_fcs_errors: c_uint = 0x0A;
pub const rcv_good_octets: c_uint = 0x0B;
pub const rcv_mac_ctrl: c_uint = 0x0C;
pub const rcv_flow_ctrl: c_uint = 0x0D;
pub const rcv_pkts_64_octets: c_uint = 0x0E;
pub const rcv_pkts_65to127_octets: c_uint = 0x0F;
pub const rcv_pkts_128to255_octets: c_uint = 0x10;
pub const rcv_pkts_256to511_octets: c_uint = 0x11;
pub const rcv_pkts_512to1023_octets: c_uint = 0x12;
pub const rcv_pkts_1024to1518_octets: c_uint = 0x13;
pub const rcv_unsupported_opcode: c_uint = 0x14;
pub const rcv_symbol_errors: c_uint = 0x15;
pub const rcv_drop_pkts_ring1: c_uint = 0x16;
pub const rcv_drop_pkts_ring2: c_uint = 0x17;
pub const rcv_drop_pkts_ring3: c_uint = 0x18;
pub const rcv_drop_pkts_ring4: c_uint = 0x19;
pub const rcv_jumbo_pkts: c_uint = 0x1A;
pub const xmt_underrun_pkts: c_uint = 0x20;
pub const xmt_octets: c_uint = 0x21;
pub const xmt_packets: c_uint = 0x22;
pub const xmt_broadcast_pkts: c_uint = 0x23;
pub const xmt_multicast_pkts: c_uint = 0x24;
pub const xmt_collisions: c_uint = 0x25;
pub const xmt_unicast_pkts: c_uint = 0x26;
pub const xmt_one_collision: c_uint = 0x27;
pub const xmt_multiple_collision: c_uint = 0x28;
pub const xmt_deferred_transmit: c_uint = 0x29;
pub const xmt_late_collision: c_uint = 0x2A;
pub const xmt_excessive_defer: c_uint = 0x2B;
pub const xmt_loss_carrier: c_uint = 0x2C;
pub const xmt_excessive_collision: c_uint = 0x2D;
pub const xmt_back_pressure: c_uint = 0x2E;
pub const xmt_flow_ctrl: c_uint = 0x2F;
pub const xmt_pkts_64_octets: c_uint = 0x30;
pub const xmt_pkts_65to127_octets: c_uint = 0x31;
pub const xmt_pkts_128to255_octets: c_uint = 0x32;
pub const xmt_pkts_256to511_octets: c_uint = 0x33;
pub const xmt_pkts_512to1023_octets: c_uint = 0x34;
pub const xmt_pkts_1024to1518_octet: c_uint = 0x35;
pub const xmt_oversize_pkts: c_uint = 0x36;
pub const xmt_jumbo_pkts: c_uint = 0x37;
// Driver definitions
pub const PCI_DEVICE_ID_AMD8111E_7462: c_uint = 0x7462;

pub const NUM_TX_RING_DR: c_int = 32;
pub const NUM_RX_RING_DR: c_int = 32;

pub const AMD8111E_MIN_MTU: c_int = 60;
pub const AMD8111E_MAX_MTU: c_int = 9000;
pub const PKT_BUFF_SZ: c_int = 1536;
pub const MIN_PKT_LEN: c_int = 60;

pub const SOFT_TIMER_FREQ: c_uint = 0xBEBC  /* 0.5 sec */;

pub const OPTION_VLAN_ENABLE: c_uint = 0x0001;
pub const OPTION_JUMBO_ENABLE: c_uint = 0x0002;
pub const OPTION_MULTICAST_ENABLE: c_uint = 0x0004;
pub const OPTION_WOL_ENABLE: c_uint = 0x0008;
pub const OPTION_WAKE_MAGIC_ENABLE: c_uint = 0x0010;
pub const OPTION_WAKE_PHY_ENABLE: c_uint = 0x0020;
pub const OPTION_INTR_COAL_ENABLE: c_uint = 0x0040;
pub const OPTION_DYN_IPG_ENABLE: c_uint = 0x0080;
pub const PHY_REG_ADDR_MASK: c_uint = 0x1f;
// ipg parameters
pub const DEFAULT_IPG: c_uint = 0x60;
pub const IFS1_DELTA: c_int = 36;

pub const IPG_STABLE_TIME: c_int = 5;
pub const MIN_IPG: c_int = 96;
pub const MAX_IPG: c_int = 255;
pub const IPG_STEP: c_int = 16;
pub const CSTATE: c_int = 1;
pub const SSTATE: c_int = 2;
// Assume controller gets data 10 times the maximum processing time
pub const REPEAT_CNT: c_int = 10;
// amd8111e descriptor flag definitions
pub const RESET_RX_FLAGS: c_uint = 0x0000;
pub const TT_MASK: c_uint = 0x000c;
pub const TCC_MASK: c_uint = 0x0003;
// driver ioctl parameters

// amd8111e descriptor format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd8111e_tx_dr {
    pub /: *mut *mut __le16 buff_count; / Size of the buffer pointed by this descriptor,
    pub tx_flags: __le16,
    pub tag_ctrl_info: __le16,
    pub tag_ctrl_cmd: __le16,
    pub buff_phy_addr: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd8111e_rx_dr {
    pub reserved: __le32,
    pub /: *mut *mut __le16 msg_count; / Received message len,
    pub tag_ctrl_info: __le16,
    pub /: *mut *mut __le16 buff_count; / Len of the buffer pointed by descriptor.,
    pub rx_flags: __le16,
    pub buff_phy_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd8111e_link_config {
pub const SPEED_INVALID: c_uint = 0xffff;
pub const DUPLEX_INVALID: c_uint = 0xff;
pub const AUTONEG_INVALID: c_uint = 0xff;
    pub orig_phy_option: c_ulong,
    pub speed: u16,
    pub duplex: u8,
    pub autoneg: u8,
    pub /: *mut *mut u8 reserved; / 32bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coal_type {

    NO_COALESCE,
    LOW_COALESCE,
    MEDIUM_COALESCE,
    HIGH_COALESCE,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coal_mode {
    RX_INTR_COAL,
    TX_INTR_COAL,
    DISABLE_COAL,
    ENABLE_COAL,

}

pub const MAX_TIMEOUT: c_int = 40;
pub const MAX_EVENT_COUNT: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd8111e_coalesce_conf {
    pub rx_timeout: c_uint,
    pub rx_event_count: c_uint,
    pub rx_packets: c_ulong,
    pub rx_prev_packets: c_ulong,
    pub rx_bytes: c_ulong,
    pub rx_prev_bytes: c_ulong,
    pub rx_coal_type: c_uint,
    pub tx_timeout: c_uint,
    pub tx_event_count: c_uint,
    pub tx_packets: c_ulong,
    pub tx_prev_packets: c_ulong,
    pub tx_bytes: c_ulong,
    pub tx_prev_bytes: c_ulong,
    pub tx_coal_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipg_info {
    pub ipg_state: c_uint,
    pub ipg: c_uint,
    pub current_ipg: c_uint,
    pub col_cnt: c_uint,
    pub diff_col_cnt: c_uint,
    pub timer_tick: c_uint,
    pub prev_ipg: c_uint,
    pub ipg_timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd8111e_priv {
    pub tx_ring: *mut *mut amd8111e_tx_dr,
    pub rx_ring: *mut *mut amd8111e_rx_dr,
    pub /: *mut *mut dma_addr_t tx_ring_dma_addr; / tx descriptor ring base address,
    pub /: *mut *mut dma_addr_t rx_ring_dma_addr; / rx descriptor ring base address,
    pub name: *const c_char,
    pub /: *mut *mut *mut pci_dev pci_dev; / Ptr to the associated pci_dev,
    pub /: *mut *mut *mut net_device amd8111e_net_dev; / ptr to associated net_device,
// Transmit and receive skbs
    pub tx_skbuff: [*mut sk_buff; NUM_TX_BUFFERS],
    pub rx_skbuff: [*mut sk_buff; NUM_RX_BUFFERS],
// Transmit and receive dma mapped addr
    pub tx_dma_addr: [dma_addr_t; NUM_TX_BUFFERS],
    pub rx_dma_addr: [dma_addr_t; NUM_RX_BUFFERS],
// Reg memory mapped address
    pub mmio: *mut void __iomem,
    pub napi: napi_struct,
    pub /: *mut *mut spinlock_t lock; / Guard lock,
    pub /: *mut *mut unsigned long rx_idx, tx_idx; / The next free ring entry,
    pub tx_complete_idx: c_ulong,
    pub tx_ring_complete_idx: c_ulong,
    pub tx_ring_idx: c_ulong,
    pub /: *mut *mut unsigned int rx_buff_len; / Buffer length of rx buffers,
    pub /: *mut *mut int options; / Options enabled/disabled for the device,
    pub ext_phy_option: c_ulong,
    pub ext_phy_addr: c_int,
    pub ext_phy_id: u32,
    pub link_config: amd8111e_link_config,
    pub next: *mut net_device,
    pub mii: c_int,
    pub mii_if: mii_if_info,
    pub opened: c_char,
    pub drv_rx_errors: c_uint,
    pub coal_conf: amd8111e_coalesce_conf,
    pub ipg_data: ipg_info,
}

// kernel provided writeq does not write 64 bits into the amd8111e device register instead writes only higher 32bits data into lower 32bits of the register.

// maps the external speed options to internal value
