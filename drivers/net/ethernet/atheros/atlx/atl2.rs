//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/atheros/atlx/atl2.h
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
// atl2.h -- atl2 driver definitions
//
// Copyright(c) 2007 Atheros Corporation. All rights reserved.
// Copyright(c) 2006 xiong huang <xiong.huang@atheros.com>
// Copyright(c) 2007 Chris Snook <csnook@redhat.com>
//
// Derived from Intel e1000 driver
// Copyright(c) 1999 - 2005 Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn ethtool_ioctl(ifr: *mut ifreq) -> c_int;
}

// function prototype
extern "C" {
    pub fn atl2_reset_hw(hw: *mut atl2_hw) -> static s32;
}
extern "C" {
    pub fn atl2_read_mac_addr(hw: *mut atl2_hw) -> static s32;
}
extern "C" {
    pub fn atl2_init_hw(hw: *mut atl2_hw) -> static s32;
}
extern "C" {
    pub fn atl2_hash_mc_addr(hw: *mut atl2_hw, mc_addr: *mut u8) -> static u32;
}
extern "C" {
    pub fn atl2_hash_set(hw: *mut atl2_hw, hash_value: u32) -> static void;
}
extern "C" {
    pub fn atl2_read_phy_reg(hw: *mut atl2_hw, reg_addr: u16, phy_data: *mut u16) -> static s32;
}
extern "C" {
    pub fn atl2_write_phy_reg(hw: *mut atl2_hw, reg_addr: u32, phy_data: u16) -> static s32;
}
extern "C" {
    pub fn atl2_read_pci_cfg(hw: *mut atl2_hw, reg: u32, value: *mut u16) -> static void;
}
extern "C" {
    pub fn atl2_write_pci_cfg(hw: *mut atl2_hw, reg: u32, value: *mut u16) -> static void;
}
extern "C" {
    pub fn atl2_set_mac_addr(hw: *mut atl2_hw) -> static void;
}
extern "C" {
    pub fn atl2_read_eeprom(hw: *mut atl2_hw, Offset: u32, pValue: *mut u32) -> static bool;
}
extern "C" {
    pub fn atl2_write_eeprom(hw: *mut atl2_hw, offset: u32, value: u32) -> static bool;
}
extern "C" {
    pub fn atl2_phy_init(hw: *mut atl2_hw) -> static s32;
}
extern "C" {
    pub fn atl2_check_eeprom_exist(hw: *mut atl2_hw) -> static int;
}
extern "C" {
    pub fn atl2_force_ps(hw: *mut atl2_hw) -> static void;
}
// register definition
// Block IDLE Status Register

// MDIO Control Register
pub const MDIO_WAIT_TIMES: c_int = 10;
// MAC Control Register
pub const MAC_CTRL_DBG_TX_BKPRESURE: c_uint = 0x100000	/* 1: TX max backoff */;
pub const MAC_CTRL_MACLP_CLK_PHY: c_uint = 0x8000000	/* 1: 25MHz from phy */;
pub const MAC_CTRL_HALF_LEFT_BUF_SHIFT: c_int = 28;
pub const MAC_CTRL_HALF_LEFT_BUF_MASK: c_uint = 0xF		/* MAC retry buf x32B */;
// Internal SRAM Partition Register
pub const REG_SRAM_TXRAM_END: c_uint = 0x1500	/* Internal tail address of TXRAM;
// default: 2byte*1024
pub const REG_SRAM_RXRAM_END: c_uint = 0x1502	/* Internal tail address of RXRAM;
// default: 2byte*1024
// Descriptor Control register
pub const REG_TXD_BASE_ADDR_LO: c_uint = 0x1544	/* The base address of the Transmit;
// Data Mem low 32-bit(dword align)
pub const REG_TXD_MEM_SIZE: c_uint = 0x1548	/* Transmit Data Memory size(by;
// double word , max 256KB)
pub const REG_TXS_BASE_ADDR_LO: c_uint = 0x154C	/* The base address of the Transmit;
// Status Memory low 32-bit(dword word
// align)
pub const REG_TXS_MEM_SIZE: c_uint = 0x1550	/* double word unit, max 4*2047;
// bytes.
pub const REG_RXD_BASE_ADDR_LO: c_uint = 0x1554	/* The base address of the Transmit;
// Status Memory low 32-bit(unit 8
// bytes)
pub const REG_RXD_BUF_NUM: c_uint = 0x1558	/* Receive Data & Status Memory buffer;
// number (unit 1536bytes, max
// 1536*2047)
// DMAR Control Register
pub const REG_DMAR: c_uint = 0x1580;
pub const DMAR_EN: c_uint = 0x1	/* 1: Enable DMAR */;
// TX Cur-Through (early tx threshold) Control Register
pub const REG_TX_CUT_THRESH: c_uint = 0x1590	/* TxMac begin transmit packet;
// threshold(unit word)
// DMAW Control Register
pub const REG_DMAW: c_uint = 0x15A0;
pub const DMAW_EN: c_uint = 0x1;
// Flow control register
pub const REG_PAUSE_ON_TH: c_uint = 0x15A8	/* RXD high watermark of overflow;
// threshold configuration register
pub const REG_PAUSE_OFF_TH: c_uint = 0x15AA	/* RXD lower watermark of overflow;
// threshold configuration register
// Mailbox Register
pub const REG_MB_TXD_WR_IDX: c_uint = 0x15f0	/* double word align */;
pub const REG_MB_RXD_RD_IDX: c_uint = 0x15F4	/* RXD Read index (unit: 1536byets) */;
// Interrupt Status Register

// when SW_MAN_INT_EN is set in Table 51
// Selene Master Control Register
// (Offset 0x1400).

pub const ISR_TXS_OV: c_uint = 0x10	/* Internal transmit status buffer full;
// interrupt
pub const ISR_RXS_OV: c_uint = 0x20	/* Internal receive status buffer full;
// interrupt
pub const ISR_LINK_CHG: c_uint = 0x40	/* Link Status Change Interrupt */;
pub const ISR_HOST_TXD_UR: c_uint = 0x80;
pub const ISR_HOST_RXD_OV: c_uint = 0x100	/* Host rx data memory full , one pulse */;
pub const ISR_DMAR_TO_RST: c_uint = 0x200	/* DMAR op timeout interrupt. SW should;
// do Reset
pub const ISR_DMAW_TO_RST: c_uint = 0x400;
pub const ISR_PHY: c_uint = 0x800	/* phy interrupt */;
pub const ISR_TS_UPDATE: c_uint = 0x10000	/* interrupt after new tx pkt status written;
// to host
pub const ISR_RS_UPDATE: c_uint = 0x20000	/* interrupt ater new rx pkt status written;
// to host.
pub const ISR_TX_EARLY: c_uint = 0x40000	/* interrupt when txmac begin transmit one;
// packet

// ISR_LINK_CHG		|*/\
// Receive MAC Statistics Registers
pub const REG_STS_RX_PAUSE: c_uint = 0x1700	/* Num pause packets received */;
pub const REG_STS_RXD_OV: c_uint = 0x1704	/* Num frames dropped due to RX;
// FIFO overflow
pub const REG_STS_RXS_OV: c_uint = 0x1708	/* Num frames dropped due to RX;
// Status Buffer Overflow
pub const REG_STS_RX_FILTER: c_uint = 0x170C	/* Num packets dropped due to;
// address filtering
// MII definitions
// PHY Common Register
pub const MII_SMARTSPEED: c_uint = 0x14;
pub const MII_DBG_ADDR: c_uint = 0x1D;
pub const MII_DBG_DATA: c_uint = 0x1E;
// PCI Command Register Bit Definitions
pub const PCI_REG_COMMAND: c_uint = 0x04;
pub const CMD_IO_SPACE: c_uint = 0x0001;
pub const CMD_MEMORY_SPACE: c_uint = 0x0002;
pub const CMD_BUS_MASTER: c_uint = 0x0004;
pub const MEDIA_TYPE_100M_FULL: c_int = 1;
pub const MEDIA_TYPE_100M_HALF: c_int = 2;
pub const MEDIA_TYPE_10M_FULL: c_int = 3;
pub const MEDIA_TYPE_10M_HALF: c_int = 4;
pub const AUTONEG_ADVERTISE_SPEED_DEFAULT: c_uint = 0x000F	/* Everything */;
// The size (in bytes) of a ethernet packet

pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_pkt_header {
    pub pkt_size:11: unsigned,
    pub /: *mut *mut unsigned:4; / reserved,
    pub /: *mut *mut unsigned ins_vlan:1; / txmac should insert vlan,
    pub /: *mut *mut unsigned short vlan; / vlan tag,
}

// FIXME: replace above bitfields with MASK/SHIFT defines below
pub const TX_PKT_HEADER_SIZE_MASK: c_uint = 0x7FF;
pub const TX_PKT_HEADER_SIZE_SHIFT: c_int = 0;
pub const TX_PKT_HEADER_INS_VLAN_MASK: c_uint = 0x1;
pub const TX_PKT_HEADER_INS_VLAN_SHIFT: c_int = 15;
pub const TX_PKT_HEADER_VLAN_TAG_MASK: c_uint = 0xFFFF;
pub const TX_PKT_HEADER_VLAN_TAG_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_pkt_status {
    pub pkt_size:11: unsigned,
    pub /: *mut *mut unsigned:5; / reserved,
    pub /: *mut *mut unsigned ok:1; / current packet transmitted without error,
    pub /: *mut *mut unsigned bcast:1; / broadcast packet,
    pub /: *mut *mut unsigned mcast:1; / multicast packet,
    pub /: *mut *mut unsigned pause:1; / transmiited a pause frame,
    pub ctrl:1: unsigned,
    pub /: *mut *mut unsigned defer:1; / current packet is xmitted with defer,
    pub exc_defer:1: unsigned,
    pub single_col:1: unsigned,
    pub multi_col:1: unsigned,
    pub late_col:1: unsigned,
    pub abort_col:1: unsigned,
    pub aborted: *mut *mut unsigned underrun:1; / current packet is,
// due to txram underrun
    pub /: *mut *mut unsigned:3; / reserved,
    pub /: *mut *mut unsigned update:1; / always 1'b1 in tx_status_buf,
}

// FIXME: replace above bitfields with MASK/SHIFT defines below
pub const TX_PKT_STATUS_SIZE_MASK: c_uint = 0x7FF;
pub const TX_PKT_STATUS_SIZE_SHIFT: c_int = 0;
pub const TX_PKT_STATUS_OK_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_OK_SHIFT: c_int = 16;
pub const TX_PKT_STATUS_BCAST_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_BCAST_SHIFT: c_int = 17;
pub const TX_PKT_STATUS_MCAST_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_MCAST_SHIFT: c_int = 18;
pub const TX_PKT_STATUS_PAUSE_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_PAUSE_SHIFT: c_int = 19;
pub const TX_PKT_STATUS_CTRL_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_CTRL_SHIFT: c_int = 20;
pub const TX_PKT_STATUS_DEFER_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_DEFER_SHIFT: c_int = 21;
pub const TX_PKT_STATUS_EXC_DEFER_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_EXC_DEFER_SHIFT: c_int = 22;
pub const TX_PKT_STATUS_SINGLE_COL_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_SINGLE_COL_SHIFT: c_int = 23;
pub const TX_PKT_STATUS_MULTI_COL_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_MULTI_COL_SHIFT: c_int = 24;
pub const TX_PKT_STATUS_LATE_COL_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_LATE_COL_SHIFT: c_int = 25;
pub const TX_PKT_STATUS_ABORT_COL_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_ABORT_COL_SHIFT: c_int = 26;
pub const TX_PKT_STATUS_UNDERRUN_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_UNDERRUN_SHIFT: c_int = 27;
pub const TX_PKT_STATUS_UPDATE_MASK: c_uint = 0x1;
pub const TX_PKT_STATUS_UPDATE_SHIFT: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_pkt_status {
    pub /: *mut *mut unsigned pkt_size:11; / packet size, max 2047 bytes,
    pub /: *mut *mut unsigned:5; / reserved,
    pub /: *mut *mut unsigned ok:1; / current packet received ok without error,
    pub /: *mut *mut unsigned bcast:1; / current packet is broadcast,
    pub /: *mut *mut unsigned mcast:1; / current packet is multicast,
    pub pause:1: unsigned,
    pub ctrl:1: unsigned,
    pub /: *mut *mut unsigned crc:1; / received a packet with crc error,
    pub /: *mut *mut unsigned code:1; / received a packet with code error,
    pub bytes: *mut *mut unsigned runt:1; / received a packet less than 64,
// with good crc
    pub bytes: *mut *mut unsigned frag:1; / received a packet less than 64,
// with bad crc
    pub /: *mut *mut unsigned trunc:1; / current frame truncated due to rxram full,
    pub /: *mut *mut unsigned align:1; / this packet is alignment error,
    pub /: *mut *mut unsigned vlan:1; / this packet has vlan,
    pub /: *mut *mut unsigned:3; / reserved,
    pub update:1: unsigned,
    pub /: *mut *mut unsigned short vtag; / vlan tag,
}

// FIXME: replace above bitfields with MASK/SHIFT defines below
pub const RX_PKT_STATUS_SIZE_MASK: c_uint = 0x7FF;
pub const RX_PKT_STATUS_SIZE_SHIFT: c_int = 0;
pub const RX_PKT_STATUS_OK_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_OK_SHIFT: c_int = 16;
pub const RX_PKT_STATUS_BCAST_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_BCAST_SHIFT: c_int = 17;
pub const RX_PKT_STATUS_MCAST_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_MCAST_SHIFT: c_int = 18;
pub const RX_PKT_STATUS_PAUSE_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_PAUSE_SHIFT: c_int = 19;
pub const RX_PKT_STATUS_CTRL_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_CTRL_SHIFT: c_int = 20;
pub const RX_PKT_STATUS_CRC_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_CRC_SHIFT: c_int = 21;
pub const RX_PKT_STATUS_CODE_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_CODE_SHIFT: c_int = 22;
pub const RX_PKT_STATUS_RUNT_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_RUNT_SHIFT: c_int = 23;
pub const RX_PKT_STATUS_FRAG_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_FRAG_SHIFT: c_int = 24;
pub const RX_PKT_STATUS_TRUNK_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_TRUNK_SHIFT: c_int = 25;
pub const RX_PKT_STATUS_ALIGN_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_ALIGN_SHIFT: c_int = 26;
pub const RX_PKT_STATUS_VLAN_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_VLAN_SHIFT: c_int = 27;
pub const RX_PKT_STATUS_UPDATE_MASK: c_uint = 0x1;
pub const RX_PKT_STATUS_UPDATE_SHIFT: c_int = 31;
pub const RX_PKT_STATUS_VLAN_TAG_MASK: c_uint = 0xFFFF;
pub const RX_PKT_STATUS_VLAN_TAG_SHIFT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_desc {
    pub status: rx_pkt_status,
    pub rx_pkt_status)]: unsigned char packet[1536-sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl2_speed_duplex {
    atl2_10_half = 0,
    atl2_10_full = 1,
    atl2_100_half = 2,
    atl2_100_full = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl2_spi_flash_dev {
    pub /: *const *const *const char manu_name; / manufacturer id,
// op-code
    pub cmdWRSR: u8,
    pub cmdREAD: u8,
    pub cmdPROGRAM: u8,
    pub cmdWREN: u8,
    pub cmdWRDI: u8,
    pub cmdRDSR: u8,
    pub cmdRDID: u8,
    pub cmdSECTOR_ERASE: u8,
    pub cmdCHIP_ERASE: u8,
}

// Structure containing variables used by the shared code (atl2_hw.c)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl2_hw {
    pub hw_addr: *mut u8 __iomem,
    pub back: *mut c_void,
    pub preamble_len: u8,
    pub the: *mut *mut u8 max_retry; / Retransmission maximum, afterwards,
// packet will be discarded.
    pub flow: *mut *mut u8 jam_ipg; / IPG to start JAM for collision based,
// control in half-duplex mode. In unit of
// 8-bit time.
    pub The: *mut *mut u8 ipgt; / Desired back to back inter-packet gap.,
// default is 96-bit time.
    pub between: *mut *mut u8 min_ifg; / Minimum number of IFG to enforce in,
// RX frames. Frame gap below such IFP is
// dropped.
    pub /: *mut *mut u8 ipgr1; / 64bit Carrier-Sense window,
    pub /: *mut *mut u8 ipgr2; / 96-bit IPG window,
    pub some: *mut *mut u8 retry_buf; / When half-duplex mode, should hold,
// bytes for mac retry . (8*4bytes unit)
    pub fc_rxd_hi: u16,
    pub fc_rxd_lo: u16,
    pub /: *mut *mut u16 lcol; / Collision Window,
    pub max_frame_size: u16,
    pub MediaType: u16,
    pub autoneg_advertised: u16,
    pub pci_cmd_word: u16,
    pub mii_autoneg_adv_reg: u16,
    pub mem_rang: u32,
    pub txcw: u32,
    pub mc_filter_type: u32,
    pub num_mc_addrs: u32,
    pub collision_delta: u32,
    pub tx_packet_delta: u32,
    pub phy_spd_default: u16,
    pub device_id: u16,
    pub vendor_id: u16,
    pub subsystem_id: u16,
    pub subsystem_vendor_id: u16,
    pub revision_id: u8,
// spi flash
    pub flash_vendor: u8,
    pub dma_fairness: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub perm_mac_addr: [u8; ETH_ALEN],
// FIXME
// bool phy_preamble_sup;
    pub phy_configured: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl2_ring_header {
// pointer to the descriptor ring memory
    pub desc: *mut c_void,
// physical address of the descriptor ring
    pub dma: dma_addr_t,
// length of descriptor ring in bytes
    pub size: c_uint,
}

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl2_adapter {
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub wol: u32,
    pub link_speed: u16,
    pub link_duplex: u16,
    pub stats_lock: spinlock_t,
    pub reset_task: work_struct,
    pub link_chg_task: work_struct,
    pub watchdog_timer: timer_list,
    pub phy_config_timer: timer_list,
    pub cfg_phy: c_ulong,
    pub mac_disabled: bool,
// All Descriptor memory
    pub ring_dma: dma_addr_t,
    pub ring_vir_addr: *mut c_void,
    pub ring_size: c_int,
    pub txd_ring: *mut tx_pkt_header,
    pub txd_dma: dma_addr_t,
    pub txs_ring: *mut tx_pkt_status,
    pub txs_dma: dma_addr_t,
    pub rxd_ring: *mut rx_desc,
    pub rxd_dma: dma_addr_t,
    pub /: *mut *mut u32 txd_ring_size; / bytes per unit,
    pub /: *mut *mut u32 txs_ring_size; / dwords per unit,
    pub /: *mut *mut u32 rxd_ring_size; / 1536 bytes per unit,
// read /write ptr:
// host
    pub txd_write_ptr: u32,
    pub txs_next_clear: u32,
    pub rxd_read_ptr: u32,
// nic
    pub txd_read_ptr: core::sync::atomic::AtomicI32,
    pub txs_write_ptr: core::sync::atomic::AtomicI32,
    pub rxd_write_ptr: u32,
// Interrupt Moderator timer ( 2us resolution)
    pub imt: u16,
// Interrupt Clear timer (2us resolution)
    pub ict: u16,
    pub flags: c_ulong,
// structs defined in atl2_hw.h
    pub /: *mut *mut u32 bd_number; / board number,
    pub pci_using_64: bool,
    pub have_msi: bool,
    pub hw: atl2_hw,
    pub usr_cmd: u32,
// FIXME
// u32 regs_buff[ATL2_REGS_LEN];
    pub pci_state: [u32; 16],
    pub config_space: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atl2_state_t {
    __ATL2_TESTING,
    __ATL2_RESETTING,
    __ATL2_DOWN
}
