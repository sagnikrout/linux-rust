//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ibm/emac/core.h
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
// drivers/net/ethernet/ibm/emac/core.h
//
// Driver for PowerPC 4xx on-chip ethernet controller.
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//
// Based on the arch/ppc version of the driver:
//
// Copyright (c) 2004, 2005 Zultys Technologies.
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
//
// Based on original work by
// Armin Kuster <akuster@mvista.com>
// Johnnie Peters <jpeters@mvista.com>
// Copyright 2000, 2001 MontaVista Softare Inc.
//

// Simple sanity check

pub const EMAC_MIN_MTU: c_int = 46;
// Maximum L2 header length (VLAN tagged, no FCS)

// RX BD size for the given MTU
extern "C" {
    pub fn mal_rx_size(EMAC_MTU_OVERHEAD: ETH_DATA_LEN +) -> return;
}
// Size of RX skb for the given MTU
// RX DMA sync size
extern "C" {
    pub fn SKB_DATA_ALIGN(NET_IP_ALIGN: emac_rx_size(mtu) +) -> return;
}
// Driver statistcs is split into two parts to make it more cache friendly:
// - normal statistics (packet count, etc)
// - error statistics
//
// When statistics is requested by ethtool, these parts are concatenated,
// normal one goes first.
//
// Please, keep these structures in sync with emac_stats_keys.
//
// Normal TX/RX Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub rx_packets_csum: u64,
    pub tx_packets_csum: u64,
}

// Error statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_error_stats {
    pub tx_undo: u64,
// Software RX Errors
    pub rx_dropped_stack: u64,
    pub rx_dropped_oom: u64,
    pub rx_dropped_error: u64,
    pub rx_dropped_resize: u64,
    pub rx_dropped_mtu: u64,
    pub rx_stopped: u64,
// BD reported RX errors
    pub rx_bd_errors: u64,
    pub rx_bd_overrun: u64,
    pub rx_bd_bad_packet: u64,
    pub rx_bd_runt_packet: u64,
    pub rx_bd_short_event: u64,
    pub rx_bd_alignment_error: u64,
    pub rx_bd_bad_fcs: u64,
    pub rx_bd_packet_too_long: u64,
    pub rx_bd_out_of_range: u64,
    pub rx_bd_in_range: u64,
// EMAC IRQ reported RX errors
    pub rx_parity: u64,
    pub rx_fifo_overrun: u64,
    pub rx_overrun: u64,
    pub rx_bad_packet: u64,
    pub rx_runt_packet: u64,
    pub rx_short_event: u64,
    pub rx_alignment_error: u64,
    pub rx_bad_fcs: u64,
    pub rx_packet_too_long: u64,
    pub rx_out_of_range: u64,
    pub rx_in_range: u64,
// Software TX Errors
    pub tx_dropped: u64,
// BD reported TX errors
    pub tx_bd_errors: u64,
    pub tx_bd_bad_fcs: u64,
    pub tx_bd_carrier_loss: u64,
    pub tx_bd_excessive_deferral: u64,
    pub tx_bd_excessive_collisions: u64,
    pub tx_bd_late_collision: u64,
    pub tx_bd_multple_collisions: u64,
    pub tx_bd_single_collision: u64,
    pub tx_bd_underrun: u64,
    pub tx_bd_sqe: u64,
// EMAC IRQ reported TX errors
    pub tx_parity: u64,
    pub tx_underrun: u64,
    pub tx_sqe: u64,
    pub tx_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_instance {
    pub ndev: *mut net_device,
    pub emacp: *mut emac_regs __iomem,
    pub ofdev: *mut platform_device,
    pub /: *mut *mut *mut *mut device_node blist; / bootlist entry,
// MAL linkage
    pub mal_ph: u32,
    pub mal_dev: *mut platform_device,
    pub mal_rx_chan: u32,
    pub mal_tx_chan: u32,
    pub mal: *mut mal_instance,
    pub commac: mal_commac,
// PHY infos
    pub phy_mode: phy_interface_t,
    pub phy_map: u32,
    pub phy_address: u32,
    pub phy_feat_exc: u32,
    pub phy: mii_phy,
    pub link_lock: mutex,
    pub link_work: delayed_work,
    pub link_polling: c_int,
// GPCS PHY infos
    pub gpcs_address: u32,
// Shared MDIO if any
    pub mdio_ph: u32,
    pub mdio_dev: *mut platform_device,
    pub mdio_instance: *mut emac_instance,
    pub mdio_lock: mutex,
// ZMII infos if any
    pub zmii_ph: u32,
    pub zmii_port: u32,
    pub zmii_dev: *mut platform_device,
// RGMII infos if any
    pub rgmii_ph: u32,
    pub rgmii_port: u32,
    pub rgmii_dev: *mut platform_device,
// TAH infos if any
    pub tah_ph: u32,
    pub tah_port: u32,
    pub tah_dev: *mut platform_device,
// IRQs
    pub wol_irq: c_int,
    pub emac_irq: c_int,
// OPB bus frequency in Mhz
    pub opb_bus_freq: u32,
// Cell index within an ASIC (for clk mgmnt)
    pub cell_index: u32,
// Max supported MTU
    pub max_mtu: u32,
// Feature bits (from probe table)
    pub features: c_uint,
// Tx and Rx fifo sizes & other infos in bytes
    pub tx_fifo_size: u32,
    pub tx_fifo_size_gige: u32,
    pub rx_fifo_size: u32,
    pub rx_fifo_size_gige: u32,
    pub fifo_entry_size: u32,
    pub /: *mut *mut u32 mal_burst_size; / move to MAL ?,
// IAHT and GAHT filter parameterization
    pub xaht_slots_shift: u32,
    pub xaht_width_shift: u32,
// Descriptor management
//
    pub tx_desc: *mut mal_descriptor,
    pub tx_cnt: c_int,
    pub tx_slot: c_int,
    pub ack_slot: c_int,
    pub rx_desc: *mut mal_descriptor,
    pub rx_slot: c_int,
    pub /: *mut *mut *mut sk_buff rx_sg_skb; / 1,
    pub rx_skb_size: c_int,
    pub rx_sync_size: c_int,
    pub tx_skb: [*mut sk_buff; NUM_TX_BUFF],
    pub rx_skb: [*mut sk_buff; NUM_RX_BUFF],
// Stats
//
    pub estats: emac_error_stats,
    pub stats: emac_stats,
// Misc
//
    pub reset_failed: c_int,
    pub /: *mut *mut int stop_timeout; / in us,
    pub no_mcast: c_int,
    pub mcast_pending: c_int,
    pub opened: c_int,
    pub reset_work: work_struct,
    pub lock: spinlock_t,
}

//
// Features of various EMAC implementations
//
// No flow control on 40x according to the original driver
//
pub const EMAC_FTR_NO_FLOW_CONTROL_40x: c_uint = 0x00000001;
//
// Cell is an EMAC4
//
pub const EMAC_FTR_EMAC4: c_uint = 0x00000002;
//
// For the 440SPe, AMCC inexplicably changed the polarity of
// the "operation complete" bit in the MII control register.
//
pub const EMAC_FTR_STACR_OC_INVERT: c_uint = 0x00000004;
//
// Set if we have a TAH.
//
pub const EMAC_FTR_HAS_TAH: c_uint = 0x00000008;
//
// Set if we have a ZMII.
//
pub const EMAC_FTR_HAS_ZMII: c_uint = 0x00000010;
//
// Set if we have a RGMII.
//
pub const EMAC_FTR_HAS_RGMII: c_uint = 0x00000020;
//
// Set if we have new type STACR with STAOPC
//
pub const EMAC_FTR_HAS_NEW_STACR: c_uint = 0x00000040;
//
// Set if we need phy clock workaround for 440gx
//
pub const EMAC_FTR_440GX_PHY_CLK_FIX: c_uint = 0x00000080;
//
// Set if we need phy clock workaround for 440ep or 440gr
//
pub const EMAC_FTR_440EP_PHY_CLK_FIX: c_uint = 0x00000100;
//
// The 405EX and 460EX contain the EMAC4SYNC core
//
pub const EMAC_FTR_EMAC4SYNC: c_uint = 0x00000200;
//
// Set if we need phy clock workaround for 460ex or 460gt
//
pub const EMAC_FTR_460EX_PHY_CLK_FIX: c_uint = 0x00000400;
//
// APM821xx requires Jumbo frame size set explicitly
//
pub const EMAC_APM821XX_REQ_JUMBO_FRAME_SIZE: c_uint = 0x00000800;
//
// APM821xx does not support Half Duplex mode
//
pub const EMAC_FTR_APM821XX_NO_HALF_DUPLEX: c_uint = 0x00001000;
// Right now, we don't quite handle the always/possible masks on the
// most optimal way as we don't have a way to say something like
// always EMAC4. Patches welcome.
//

//
// Various instances of the EMAC core have varying 1) number of
// address match slots, 2) width of the registers for handling address
// match slots, 3) number of registers for handling address match
// slots and 4) base offset for those registers.
//
// These macros and inlines handle these differences based on
// parameters supplied by the device structure which are, in turn,
// initialized based on the "compatible" entry in the device tree.
//
pub const EMAC4_XAHT_SLOTS_SHIFT: c_int = 6;
pub const EMAC4_XAHT_WIDTH_SHIFT: c_int = 4;
pub const EMAC4SYNC_XAHT_SLOTS_SHIFT: c_int = 8;
pub const EMAC4SYNC_XAHT_WIDTH_SHIFT: c_int = 5;
// The largest span between slots and widths above is 3

// The first IAHT entry always is the base of the block of
// IAHT and GAHT registers.
//
// GAHT registers always come after an identical number of
// IAHT registers.
//
extern "C" {
    pub fn emac_xaht_base(EMAC_XAHT_REGS(dev: dev) +) -> return;
}
// IAHT registers always come before an identical number of
// GAHT registers.
//
extern "C" {
    pub fn emac_xaht_base(_arg: dev) -> return;
}
// Ethtool get_regs complex data.
// We want to get not just EMAC registers, but also MAL, ZMII, RGMII, TAH
// when available.
//
// Returned BLOB consists of the ibm_emac_ethtool_regs_hdr,
// MAL registers, EMAC registers and optional ZMII, RGMII, TAH registers.
// Each register component is preceded with emac_ethtool_regs_subhdr.
// Order of the optional headers follows their relative bit posititions
// in emac_ethtool_regs_hdr.components
//
pub const EMAC_ETHTOOL_REGS_ZMII: c_uint = 0x00000001;
pub const EMAC_ETHTOOL_REGS_RGMII: c_uint = 0x00000002;
pub const EMAC_ETHTOOL_REGS_TAH: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_ethtool_regs_hdr {
    pub components: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_ethtool_regs_subhdr {
    pub version: u32,
    pub index: u32,
}

pub const EMAC_ETHTOOL_REGS_VER: c_int = 3;
pub const EMAC4_ETHTOOL_REGS_VER: c_int = 4;
pub const EMAC4SYNC_ETHTOOL_REGS_VER: c_int = 5;
