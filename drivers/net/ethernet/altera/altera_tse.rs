//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/altera/altera_tse.h
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
// Altera Triple-Speed Ethernet MAC driver
// Copyright (C) 2008-2014 Altera Corporation. All rights reserved
//
// Contributors:
// Dalon Westergreen
// Thomas Chou
// Ian Abbott
// Yuriy Kozlov
// Tobias Klauser
// Andriy Smolskyy
// Roman Bulgakov
// Dmytro Mytarchuk
// Matthew Gerlach
//
// Original driver contributed by SLS.
// Major updates contributed by GlobalLogic
//

pub const ALTERA_TSE_SW_RESET_WATCHDOG_CNTR: c_int = 10000;

// bytes
//
// Rx FIFO default settings
pub const ALTERA_TSE_RX_SECTION_EMPTY: c_int = 16;
pub const ALTERA_TSE_RX_SECTION_FULL: c_int = 0;
pub const ALTERA_TSE_RX_ALMOST_EMPTY: c_int = 8;
pub const ALTERA_TSE_RX_ALMOST_FULL: c_int = 8;
// Tx FIFO default settings
pub const ALTERA_TSE_TX_SECTION_EMPTY: c_int = 16;
pub const ALTERA_TSE_TX_SECTION_FULL: c_int = 0;
pub const ALTERA_TSE_TX_ALMOST_EMPTY: c_int = 8;
pub const ALTERA_TSE_TX_ALMOST_FULL: c_int = 3;
// MAC function configuration default settings
pub const ALTERA_TSE_TX_IPG_LENGTH: c_int = 12;
pub const ALTERA_TSE_PAUSE_QUANTA: c_uint = 0xffff;

// MAC Command_Config Register Bit Definitions
//

// MDIO registers within MAC register Space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_tse_mdio {
    pub /: *mut *mut u32 control; / PHY device operation control register,
    pub /: *mut *mut u32 status; / PHY device operation status register,
    pub /: *mut *mut u32 phy_id1; / Bits 31:16 of PHY identifier,
    pub /: *mut *mut u32 phy_id2; / Bits 15:0 of PHY identifier,
    pub Auto-negotiation: *mut *mut u32 auto_negotiation_advertisement; /,
// advertisement
// register
//
    pub remote_partner_base_page_ability: u32,
    pub reg6: u32,
    pub reg7: u32,
    pub reg8: u32,
    pub reg9: u32,
    pub rega: u32,
    pub regb: u32,
    pub regc: u32,
    pub regd: u32,
    pub rege: u32,
    pub regf: u32,
    pub reg10: u32,
    pub reg11: u32,
    pub reg12: u32,
    pub reg13: u32,
    pub reg14: u32,
    pub reg15: u32,
    pub reg16: u32,
    pub reg17: u32,
    pub reg18: u32,
    pub reg19: u32,
    pub reg1a: u32,
    pub reg1b: u32,
    pub reg1c: u32,
    pub reg1d: u32,
    pub reg1e: u32,
    pub reg1f: u32,
}

// MAC register Space. Note that some of these registers may or may not be
// present depending upon options chosen by the user when the core was
// configured and built. Please consult the Altera Triple Speed Ethernet User
// Guide for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_tse_mac {
// Bits 15:0: MegaCore function revision (0x0800). Bit 31:16: Customer
// specific revision
//
    pub megacore_revision: u32,
// Provides a memory location for user applications to test the device
// memory operation.
//
    pub scratch_pad: u32,
// The host processor uses this register to control and configure the
// MAC block
//
    pub command_config: u32,
// 32-bit primary MAC address word 0 bits 0 to 31 of the primary
// MAC address
//
    pub mac_addr_0: u32,
// 32-bit primary MAC address word 1 bits 32 to 47 of the primary
// MAC address
//
    pub mac_addr_1: u32,
// 14-bit maximum frame length. The MAC receive logic
    pub frm_length: u32,
// The pause quanta is used in each pause frame sent to a remote
// Ethernet device, in increments of 512 Ethernet bit times
//
    pub pause_quanta: u32,
// 12-bit receive FIFO section-empty threshold
    pub rx_section_empty: u32,
// 12-bit receive FIFO section-full threshold
    pub rx_section_full: u32,
// 12-bit transmit FIFO section-empty threshold
    pub tx_section_empty: u32,
// 12-bit transmit FIFO section-full threshold
    pub tx_section_full: u32,
// 12-bit receive FIFO almost-empty threshold
    pub rx_almost_empty: u32,
// 12-bit receive FIFO almost-full threshold
    pub rx_almost_full: u32,
// 12-bit transmit FIFO almost-empty threshold
    pub tx_almost_empty: u32,
// 12-bit transmit FIFO almost-full threshold
    pub tx_almost_full: u32,
// MDIO address of PHY Device 0. Bits 0 to 4 hold a 5-bit PHY address
    pub mdio_phy0_addr: u32,
// MDIO address of PHY Device 1. Bits 0 to 4 hold a 5-bit PHY address
    pub mdio_phy1_addr: u32,
// Bit[15:0]—16-bit holdoff quanta
    pub holdoff_quant: u32,
// only if 100/1000 BaseX PCS, reserved otherwise
    pub reserved1: [u32; 5],
// Minimum IPG between consecutive transmit frame in terms of bytes
    pub tx_ipg_length: u32,
// IEEE 802.3 oEntity Managed Object Support
// The MAC addresses
    pub mac_id_1: u32,
    pub mac_id_2: u32,
// Number of frames transmitted without error including pause frames
    pub frames_transmitted_ok: u32,
// Number of frames received without error including pause frames
    pub frames_received_ok: u32,
// Number of frames received with a CRC error
    pub frames_check_sequence_errors: u32,
// Frame received with an alignment error
    pub alignment_errors: u32,
// Sum of payload and padding octets of frames transmitted without
// error
//
    pub octets_transmitted_ok: u32,
// Sum of payload and padding octets of frames received without error
    pub octets_received_ok: u32,
// IEEE 802.3 oPausedEntity Managed Object Support
// Number of transmitted pause frames
    pub tx_pause_mac_ctrl_frames: u32,
// Number of Received pause frames
    pub rx_pause_mac_ctrl_frames: u32,
// IETF MIB (MIB-II) Object Support
// Number of frames received with error
    pub if_in_errors: u32,
// Number of frames transmitted with error
    pub if_out_errors: u32,
// Number of valid received unicast frames
    pub if_in_ucast_pkts: u32,
// Number of valid received multicasts frames (without pause)
    pub if_in_multicast_pkts: u32,
// Number of valid received broadcast frames
    pub if_in_broadcast_pkts: u32,
    pub if_out_discards: u32,
// The number of valid unicast frames transmitted
    pub if_out_ucast_pkts: u32,
// The number of valid multicast frames transmitted,
// excluding pause frames
//
    pub if_out_multicast_pkts: u32,
    pub if_out_broadcast_pkts: u32,
// IETF RMON MIB Object Support
// Counts the number of dropped packets due to internal errors
// of the MAC client.
//
    pub ether_stats_drop_events: u32,
// Total number of bytes received. Good and bad frames.
    pub ether_stats_octets: u32,
// Total number of packets received. Counts good and bad packets.
    pub ether_stats_pkts: u32,
// Number of packets received with less than 64 bytes.
    pub ether_stats_undersize_pkts: u32,
// The number of frames received that are longer than the
// value configured in the frm_length register
//
    pub ether_stats_oversize_pkts: u32,
// Number of received packet with 64 bytes
    pub ether_stats_pkts_64_octets: u32,
// Frames (good and bad) with 65 to 127 bytes
    pub ether_stats_pkts_65to127_octets: u32,
// Frames (good and bad) with 128 to 255 bytes
    pub ether_stats_pkts_128to255_octets: u32,
// Frames (good and bad) with 256 to 511 bytes
    pub ether_stats_pkts_256to511_octets: u32,
// Frames (good and bad) with 512 to 1023 bytes
    pub ether_stats_pkts_512to1023_octets: u32,
// Frames (good and bad) with 1024 to 1518 bytes
    pub ether_stats_pkts_1024to1518_octets: u32,
// Any frame length from 1519 to the maximum length configured in the
// frm_length register, if it is greater than 1518
//
    pub ether_stats_pkts_1519tox_octets: u32,
// Too long frames with CRC error
    pub ether_stats_jabbers: u32,
// Too short frames with CRC error
    pub ether_stats_fragments: u32,
    pub reserved2: u32,
// FIFO control register
    pub tx_cmd_stat: u32,
    pub rx_cmd_stat: u32,
// Extended Statistics Counters
    pub msb_octets_transmitted_ok: u32,
    pub msb_octets_received_ok: u32,
    pub msb_ether_stats_octets: u32,
    pub reserved3: u32,
// Multicast address resolution table, mapped in the controller address
// space
//
    pub hash_table: [u32; 64],
// Registers 0 to 31 within PHY device 0/1 connected to the MDIO PHY
// management interface
//
    pub mdio_phy0: altera_tse_mdio,
    pub mdio_phy1: altera_tse_mdio,
// 4 Supplemental MAC Addresses
    pub supp_mac_addr_0_0: u32,
    pub supp_mac_addr_0_1: u32,
    pub supp_mac_addr_1_0: u32,
    pub supp_mac_addr_1_1: u32,
    pub supp_mac_addr_2_0: u32,
    pub supp_mac_addr_2_1: u32,
    pub supp_mac_addr_3_0: u32,
    pub supp_mac_addr_3_1: u32,
    pub reserved4: [u32; 8],
// IEEE 1588v2 Feature
    pub tx_period: u32,
    pub tx_adjust_fns: u32,
    pub tx_adjust_ns: u32,
    pub rx_period: u32,
    pub rx_adjust_fns: u32,
    pub rx_adjust_ns: u32,
    pub reserved5: [u32; 42],
}

// Transmit and Receive Command Registers Bit Definitions
//

// Wrapper around a pointer to a socket buffer,
// so a DMA handle can be stored along with the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tse_buffer {
    pub lh: list_head,
    pub skb: *mut sk_buff,
    pub dma_addr: dma_addr_t,
    pub len: u32,
    pub mapped_as_page: c_int,
}

pub const ALTERA_DTYPE_SGDMA: c_int = 1;
pub const ALTERA_DTYPE_MSGDMA: c_int = 2;
// standard DMA interface for SGDMA and MSGDMA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_dmaops {
    pub altera_dtype: c_int,
    pub dmamask: c_int,
    pub ): *mut *mut void (reset_dma)(struct altera_tse_private,
    pub ): *mut *mut void (enable_txirq)(struct altera_tse_private,
    pub ): *mut *mut void (enable_rxirq)(struct altera_tse_private,
    pub ): *mut *mut void (disable_txirq)(struct altera_tse_private,
    pub ): *mut *mut void (disable_rxirq)(struct altera_tse_private,
    pub ): *mut *mut void (clear_txirq)(struct altera_tse_private,
    pub ): *mut *mut void (clear_rxirq)(struct altera_tse_private,
    pub ): *mut *mut *mut int (tx_buffer)(struct altera_tse_private , struct tse_buffer,
    pub ): *mut *mut u32 (tx_completions)(struct altera_tse_private,
    pub ): *mut *mut *mut void (add_rx_desc)(struct altera_tse_private , struct tse_buffer,
    pub ): *mut *mut u32 (get_rx_status)(struct altera_tse_private,
    pub ): *mut *mut int (init_dma)(struct altera_tse_private,
    pub ): *mut *mut void (uninit_dma)(struct altera_tse_private,
    pub ): *mut *mut void (start_rxdma)(struct altera_tse_private,
}

// This structure is private to each device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct altera_tse_private {
    pub dev: *mut net_device,
    pub device: *mut device,
    pub napi: napi_struct,
// MAC address space
    pub mac_dev: *mut altera_tse_mac __iomem,
// mSGDMA Rx Dispatcher address space
    pub rx_dma_csr: *mut void __iomem,
    pub rx_dma_desc: *mut void __iomem,
    pub rx_dma_resp: *mut void __iomem,
// mSGDMA Tx Dispatcher address space
    pub tx_dma_csr: *mut void __iomem,
    pub tx_dma_desc: *mut void __iomem,
// SGMII PCS address space
    pub pcs_base: *mut void __iomem,
// Rx buffers queue
    pub rx_ring: *mut tse_buffer,
    pub rx_cons: u32,
    pub rx_prod: u32,
    pub rx_ring_size: u32,
    pub rx_dma_buf_sz: u32,
// Tx ring buffer
    pub tx_ring: *mut tse_buffer,
    pub tx_prod: u32,
    pub tx_cons: u32,
    pub tx_ring_size: u32,
// Interrupts
    pub tx_irq: u32,
    pub rx_irq: u32,
// RX/TX MAC FIFO configs
    pub tx_fifo_depth: u32,
    pub rx_fifo_depth: u32,
// Hash filter settings
    pub hash_filter: u32,
    pub added_unicast: u32,
// Descriptor memory info for managing SGDMA
    pub txdescmem: u32,
    pub rxdescmem: u32,
    pub rxdescmem_busaddr: dma_addr_t,
    pub txdescmem_busaddr: dma_addr_t,
    pub txctrlreg: u32,
    pub rxctrlreg: u32,
    pub rxdescphys: dma_addr_t,
    pub txdescphys: dma_addr_t,
    pub txlisthd: list_head,
    pub rxlisthd: list_head,
// MAC command_config register protection
    pub mac_cfg_lock: spinlock_t,
// Tx path protection
    pub tx_lock: spinlock_t,
// Rx DMA & interrupt control protection
    pub rxdma_irq_lock: spinlock_t,
// PHY
    pub /: *mut *mut int phy_addr; / PHY's MDIO address, -1 for autodetection,
    pub phy_iface: phy_interface_t,
    pub mdio: *mut mii_bus,
    pub oldspeed: c_int,
    pub oldduplex: c_int,
    pub oldlink: c_int,
// ethtool msglvl option
    pub msg_enable: u32,
    pub dmaops: *const altera_dmaops,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub pcs: *mut phylink_pcs,
}

// Function prototypes
//
extern "C" {
    pub fn altera_tse_set_ethtool_ops(: *mut net_device);
}
extern "C" {
    pub fn readl(_arg: paddr) -> return;
}
extern "C" {
    pub fn readw(_arg: paddr) -> return;
}
extern "C" {
    pub fn readb(_arg: paddr) -> return;
}
