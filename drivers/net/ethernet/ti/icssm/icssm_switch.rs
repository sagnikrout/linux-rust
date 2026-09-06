//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssm/icssm_switch.h
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
// Copyright (C) 2015-2021 Texas Instruments Incorporated - https://www.ti.com
//
// Basic Switch Parameters
// Used to auto compute offset addresses on L3 OCMC RAM. Do not modify these
// without changing firmware accordingly
//

pub const PORT_LINK_MASK: c_uint = 0x1;
pub const PORT_IS_HD_MASK: c_uint = 0x2;
// Physical Port queue size (number of BDs). Same for both ports

// Host queue size (number of BDs). Each BD points to data buffer of 32 bytes.
// HOST PORT QUEUES can buffer up to 4 full sized frames per queue
//

pub const COL_QUEUE_SIZE: c_int = 0;
// NRT Buffer descriptor definition
// Each buffer descriptor points to a max 32 byte block and has 32 bit in size
// to have atomic operation.
// PRU can address bytewise into memory.
// Definition of 32 bit descriptor is as follows
//
// Bits		Name			Meaning
// =============================================================================
// 0..7		Index		points to index in buffer queue, max 256 x 32
// byte blocks can be addressed
// 6		LookupSuccess	For switch, FDB lookup was successful (source
// MAC address found in FDB).
// For RED, NodeTable lookup was successful.
// 7		Flood		Packet should be flooded (destination MAC
// address found in FDB). For switch only.
// 8..12	Block_length	number of valid bytes in this specific block.
// Will be <=32 bytes on last block of packet
// 13		More		"More" bit indicating that there are more blocks
// 14		Shadow		indicates that "index" is pointing into shadow
// buffer
// 15		TimeStamp	indicates that this packet has time stamp in
// separate buffer - only needed if PTP runs on
// host
// 16..17	Port		different meaning for ingress and egress,
// Ingress: Port = 0 indicates phy port 1 and
// Port = 1 indicates phy port 2.
// Egress: 0 sends on phy port 1 and 1 sends on
// phy port 2. Port = 2 goes over MAC table
// look-up
// 18..28	Length		11 bit of total packet length which is put into
// first BD only so that host access only one BD
// 29		VlanTag		indicates that packet has Length/Type field of
// 0x08100 with VLAN tag in following byte
// 30		Broadcast	indicates that packet goes out on both physical
// ports,	there will be two bd but only one buffer
// 31		Error		indicates there was an error in the packet
//

pub const PRUETH_BD_START_FLAG_SHIFT: c_int = 0;

pub const PRUETH_BD_HSR_FRAME_SHIFT: c_int = 4;

pub const PRUETH_BD_SUP_HSR_FRAME_SHIFT: c_int = 5;

pub const PRUETH_BD_LOOKUP_SUCCESS_SHIFT: c_int = 6;

pub const PRUETH_BD_SW_FLOOD_SHIFT: c_int = 7;

pub const PRUETH_BD_SHADOW_SHIFT: c_int = 14;

pub const PRUETH_BD_TIMESTAMP_SHIFT: c_int = 15;

pub const PRUETH_BD_PORT_SHIFT: c_int = 16;

pub const PRUETH_BD_LENGTH_SHIFT: c_int = 18;

pub const PRUETH_BD_BROADCAST_SHIFT: c_int = 30;

pub const PRUETH_BD_ERROR_SHIFT: c_int = 31;
// The following offsets indicate which sections of the memory are used
// for EMAC internal tasks
//
pub const DRAM_START_OFFSET: c_uint = 0x1E98;
pub const SRAM_START_OFFSET: c_uint = 0x400;
// General Purpose Statistics
// These are present on both PRU0 and PRU1 DRAM
//
// base statistics offset
pub const STATISTICS_OFFSET: c_uint = 0x1F00;
pub const STAT_SIZE: c_uint = 0x98;
// The following offsets indicate which sections of the memory are used
// for switch internal tasks
//
pub const SWITCH_SPECIFIC_DRAM0_START_SIZE: c_uint = 0x100;
pub const SWITCH_SPECIFIC_DRAM0_START_OFFSET: c_uint = 0x1F00;
pub const SWITCH_SPECIFIC_DRAM1_START_SIZE: c_uint = 0x300;
pub const SWITCH_SPECIFIC_DRAM1_START_OFFSET: c_uint = 0x1D00;
// Offset for storing
// 1. Storm Prevention Params
// 2. PHY Speed Offset
// 3. Port Status Offset
// These are present on both PRU0 and PRU1
//
// 4 bytes

// 4 bytes

// 1 byte

// 1 byte

// 4 bytes

// 4 bytes

// 6 bytes

// 1 byte

// 4 bytes

// 4 bytes

// 4 bytes ?

// DRAM1 Offsets for Switch
// 4 queue descriptors for port 0 (host receive)
pub const P0_QUEUE_DESC_OFFSET: c_uint = 0x1E7C;
pub const P1_QUEUE_DESC_OFFSET: c_uint = 0x1E9C;
pub const P2_QUEUE_DESC_OFFSET: c_uint = 0x1EBC;
// collision descriptor of port 0
pub const P0_COL_QUEUE_DESC_OFFSET: c_uint = 0x1E64;
pub const P1_COL_QUEUE_DESC_OFFSET: c_uint = 0x1E6C;
pub const P2_COL_QUEUE_DESC_OFFSET: c_uint = 0x1E74;
// Collision Status Register
// P0: bit 0 is pending flag, bit 1..2 indicates which queue,
// P1: bit 8 is pending flag, 9..10 is queue number
// P2: bit 16 is pending flag, 17..18 is queue number, remaining bits are 0.
//
pub const COLLISION_STATUS_ADDR: c_uint = 0x1E60;
pub const INTERFACE_MAC_ADDR: c_uint = 0x1E58;
pub const P2_MAC_ADDR: c_uint = 0x1E50;
pub const P1_MAC_ADDR: c_uint = 0x1E48;
pub const QUEUE_SIZE_ADDR: c_uint = 0x1E30;
pub const QUEUE_OFFSET_ADDR: c_uint = 0x1E18;
pub const QUEUE_DESCRIPTOR_OFFSET_ADDR: c_uint = 0x1E00;

// Port 2 Rx Context

// Port 1 Rx Context

// Host Port Rx Context

// Port 2 Tx Collision Context

// Port 1 Tx Collision Context

// Port 2

// Port 1

// DRAM Offsets for EMAC
// Present on Both DRAM0 and DRAM1
//
// 4 queue descriptors for port tx = 32 bytes

// EMAC Time Triggered Send Offsets

// Shared RAM offsets for EMAC
// Queue Descriptors
// 4 queue descriptors for port 0 (host receive). 32 bytes

// table offset for queue size:
// 3 ports * 4 Queues * 1 byte offset = 12 bytes
//

// table offset for queue:
// 4 Queues * 2 byte offset = 8 bytes
//

// table offset for Host queue descriptors:
// 1 ports * 4 Queues * 2 byte offset = 8 bytes
//

// Host Port Rx Context

// Promiscuous mode control

// allow for max 48k buffer which spans the descriptors up to 0x1800 6kB

// Memory Usage of L3 OCMC RAM
// L3 64KB Memory - mainly buffer Pool

pub const P0_COL_BUFFER_OFFSET: c_uint = 0xEE00;
pub const P0_Q1_BUFFER_OFFSET: c_uint = 0x0000;

pub const V2_1_FDB_TBL_OFFSET: c_uint = 0x2000;
pub const FDB_INDEX_TBL_MAX_ENTRIES: c_int = 256;
pub const FDB_MAC_TBL_MAX_ENTRIES: c_int = 256;

