//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssm/icssm_prueth.h
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
// Texas Instruments ICSSM Ethernet driver
//
// Copyright (C) 2018-2022 Texas Instruments Incorporated - https://www.ti.com
//

// ICSSM size of redundancy tag
pub const ICSSM_LRE_TAG_SIZE: c_int = 6;
// PRUSS local memory map
pub const ICSS_LOCAL_SHARED_RAM: c_uint = 0x00010000;

// Below macro is for 1528 Byte Frame support, to Allow even with
// Redundancy tag
//

// PRU Ethernet Type - Ethernet functionality (protocol
// implemented) provided by the PRU firmware being loaded.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_ethtype {
    PRUSS_ETHTYPE_EMAC = 0,
    PRUSS_ETHTYPE_HSR,
    PRUSS_ETHTYPE_PRP,
    PRUSS_ETHTYPE_SWITCH,
    PRUSS_ETHTYPE_MAX,
}

//
// struct prueth_queue_desc - Queue descriptor
// @rd_ptr:	Read pointer, points to a buffer descriptor in Shared PRU RAM.
// @wr_ptr:	Write pointer, points to a buffer descriptor in Shared PRU RAM.
// @busy_s:	Slave queue busy flag, set by slave(us) to request access from
// master(PRU).
// @status:	Bit field status register, Bits:
// 0: Master queue busy flag.
// 1: Packet has been placed in collision queue.
// 2: Packet has been discarded due to overflow.
// @max_fill_level:	Maximum queue usage seen.
// @overflow_cnt:	Count of queue overflows.
//
// Each port has up to 4 queues with variable length. The queue is processed
// as ring buffer with read and write pointers. Both pointers are address
// pointers and increment by 4 for each buffer descriptor position. Queue has
// a length defined in constants and a status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_queue_desc {
    pub rd_ptr: u16,
    pub wr_ptr: u16,
    pub busy_s: u8,
    pub status: u8,
    pub max_fill_level: u8,
    pub overflow_cnt: u8,
}

//
// struct prueth_queue_info - Information about a queue in memory
// @buffer_offset: buffer offset in OCMC RAM
// @queue_desc_offset: queue descriptor offset in Shared RAM
// @buffer_desc_offset: buffer descriptors offset in Shared RAM
// @buffer_desc_end: end address of buffer descriptors in Shared RAM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_queue_info {
    pub buffer_offset: u16,
    pub queue_desc_offset: u16,
    pub buffer_desc_offset: u16,
    pub buffer_desc_end: u16,
}

//
// struct prueth_packet_info - Info about a packet in buffer
// @shadow: this packet is stored in the collision queue
// @port: port packet is on
// @length: length of packet
// @broadcast: this packet is a broadcast packet
// @error: this packet has an error
// @lookup_success: src mac found in FDB
// @flood: packet is to be flooded
// @timestamp: Specifies if timestamp is appended to the packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_packet_info {
    pub shadow: bool,
    pub port: c_uint,
    pub length: c_uint,
    pub broadcast: bool,
    pub error: bool,
    pub lookup_success: bool,
    pub flood: bool,
    pub timestamp: bool,
}

// In switch mode there are 3 real ports i.e. 3 mac addrs.
// however Linux sees only the host side port. The other 2 ports
// are the switch ports.
// In emac mode there are 2 real ports i.e. 2 mac addrs.
// Linux sees both the ports.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_port {
    PRUETH_PORT_HOST = 0,	/* host side port */
    PRUETH_PORT_MII0,	/* physical port MII 0 */
    PRUETH_PORT_MII1,	/* physical port MII 1 */
    PRUETH_PORT_INVALID,	/* Invalid prueth port */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_mac {
    PRUETH_MAC0 = 0,
    PRUETH_MAC1,
    PRUETH_NUM_MACS,
    PRUETH_MAC_INVALID,
}

// In both switch & emac modes there are 3 port queues
// EMAC mode:
// RX packets for both MII0 & MII1 ports come on
// QUEUE_HOST.
// TX packets for MII0 go on QUEUE_MII0, TX packets
// for MII1 go on QUEUE_MII1.
// Switch mode:
// Host port RX packets come on QUEUE_HOST
// TX packets might have to go on MII0 or MII1 or both.
// MII0 TX queue is QUEUE_MII0 and MII1 TX queue is
// QUEUE_MII1.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_port_queue_id {
    PRUETH_PORT_QUEUE_HOST = 0,
    PRUETH_PORT_QUEUE_MII0,
    PRUETH_PORT_QUEUE_MII1,
    PRUETH_PORT_QUEUE_MAX,
}

// Each port queue has 4 queues and 1 collision queue
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_queue_id {
    PRUETH_QUEUE1 = 0,
    PRUETH_QUEUE2,
    PRUETH_QUEUE3,
    PRUETH_QUEUE4,
    PRUETH_COLQUEUE,        /* collision queue */
}

//
// struct prueth_firmware - PRU Ethernet FW data
// @fw_name: firmware names of firmware to run on PRU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_firmware {
    pub fw_name: [*const c_char; PRUSS_ETHTYPE_MAX],
}

// PRUeth memory range identifiers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_mem {
    PRUETH_MEM_DRAM0 = 0,
    PRUETH_MEM_DRAM1,
    PRUETH_MEM_SHARED_RAM,
    PRUETH_MEM_OCMC,
    PRUETH_MEM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_device {
    PRUSS_AM57XX = 0,
    PRUSS_AM43XX,
    PRUSS_AM33XX,
    PRUSS_K2G
}

//
// struct prueth_private_data - PRU Ethernet private data
// @driver_data: PRU Ethernet device name
// @fw_pru: firmware names to be used for PRUSS ethernet usecases
// @support_switch: boolean to indicate if switch is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_private_data {
    pub driver_data: pruss_device,
    pub fw_pru: [prueth_firmware; PRUSS_NUM_PRUS],
    pub support_switch: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_emac_stats {
    pub tx_packets: u64,
    pub tx_dropped: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_length_errors: u64,
    pub rx_over_errors: u64,
}

// data for each emac port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_emac {
    pub prueth: *mut prueth,
    pub ndev: *mut net_device,
    pub napi: napi_struct,
    pub pru: *mut rproc,
    pub phydev: *mut phy_device,
    pub rx_queue_descs: *mut prueth_queue_desc __iomem,
    pub tx_queue_descs: *mut prueth_queue_desc __iomem,
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub rx_irq: c_int,
    pub tx_port_queue: prueth_port_queue_id,
    pub rx_queue_start: prueth_queue_id,
    pub rx_queue_end: prueth_queue_id,
    pub port_id: prueth_port,
    pub dram: prueth_mem,
    pub phy_id: *const c_char,
    pub msg_enable: u32,
    pub mac_addr: [u8; 6],
    pub /: *mut *mut unsigned char mc_filter_mask[ETH_ALEN]; / for multicast filtering,
    pub phy_if: phy_interface_t,
// spin lock used to protect
// during link configuration
//
    pub lock: spinlock_t,
    pub /: *mut *mut spinlock_t addr_lock; / serialize access to VLAN/MC filter table,
    pub tx_hrtimer: hrtimer,
    pub stats: prueth_emac_stats,
    pub offload_fwd_mark: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth {
    pub dev: *mut device,
    pub pruss: *mut pruss,
    pub pru1: *mut *mut rproc pru0,,
    pub mem: [pruss_mem_region; PRUETH_MEM_MAX],
    pub sram_pool: *mut gen_pool,
    pub mii_rt: *mut regmap,
    pub iep: *mut icss_iep,
    pub fw_data: *const prueth_private_data,
    pub fw_offsets: *mut prueth_fw_offsets,
    pub eth_node: [*mut device_node; PRUETH_NUM_MACS],
    pub emac: [*mut prueth_emac; PRUETH_NUM_MACS],
    pub registered_netdevs: [*mut net_device; PRUETH_NUM_MACS],
    pub hw_bridge_dev: *mut net_device,
    pub fdb_tbl: *mut fdb_tbl,
    pub prueth_netdevice_nb: notifier_block,
    pub prueth_switchdev_nb: notifier_block,
    pub prueth_switchdev_bl_nb: notifier_block,
    pub eth_type: c_uint,
    pub ocmc_ram_size: usize,
    pub emac_configured: u8,
    pub br_members: u8,
}

extern "C" {
    pub fn icssm_emac_mc_filter_bin_allow(emac: *mut prueth_emac, hash: u8);
}
extern "C" {
    pub fn icssm_emac_mc_filter_bin_disallow(emac: *mut prueth_emac, hash: u8);
}
extern "C" {
    pub fn icssm_emac_get_mc_hash(mac: *mut u8, mask: *mut u8) -> u8;
}
