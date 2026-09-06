//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/emac/emac-mac.h
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
// Copyright (c) 2013-2016, The Linux Foundation. All rights reserved.
//
// EMAC DMA HW engine uses three rings:
// Tx:
// TPD: Transmit Packet Descriptor ring.
// Rx:
// RFD: Receive Free Descriptor ring.
// Ring of descriptors with empty buffers to be filled by Rx HW.
// RRD: Receive Return Descriptor ring.
// Ring of descriptors with buffers filled with received data.
//
// EMAC_CSR register offsets
pub const EMAC_EMAC_WRAPPER_CSR1: c_uint = 0x000000;
pub const EMAC_EMAC_WRAPPER_CSR2: c_uint = 0x000004;
pub const EMAC_EMAC_WRAPPER_TX_TS_LO: c_uint = 0x000104;
pub const EMAC_EMAC_WRAPPER_TX_TS_HI: c_uint = 0x000108;
pub const EMAC_EMAC_WRAPPER_TX_TS_INX: c_uint = 0x00010c;
// DMA Order Settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emac_dma_order {
    emac_dma_ord_in = 1,
    emac_dma_ord_enh = 2,
    emac_dma_ord_out = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emac_dma_req_block {
    emac_dma_req_128 = 0,
    emac_dma_req_256 = 1,
    emac_dma_req_512 = 2,
    emac_dma_req_1024 = 3,
    emac_dma_req_2048 = 4,
    emac_dma_req_4096 = 5
}

// Returns the value of bits idx...idx+n_bits

// RRD (Receive Return Descriptor)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_rrd {
    pub word: [u32; 6],
// number of RFD

// start consumer index of rfd-ring

// vlan-tag (CVID, CFI and PRI)

// length of the packet

// L4(TCP/UDP) checksum failed

// vlan tagged

// When set, indicates that the descriptor is updated by the IP core.
// When cleared, indicates that the descriptor is invalid.
//

// timestamp low

// timestamp high

}

// TPD (Transmit Packet Descriptor)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_tpd {
    pub word: [u32; 4],
// Number of bytes of the transmit packet. (include 4-byte CRC)

// Custom Checksum Offload: When set, ask IP core to offload custom checksum

// TCP Large Send Offload: When set, ask IP core to do offload TCP Large Send

// Large Send Offload Version: When set, indicates this is an LSOv2
// (for both IPv4 and IPv6). When cleared, indicates this is an LSOv1
// (only for IPv4).
//

// IPv4 packet: When set, indicates this is an  IPv4 packet, this bit is only
// for LSOV2 format.
//

// 0: Ethernet   frame (DA+SA+TYPE+DATA+CRC)
// 1: IEEE 802.3 frame (DA+SA+LEN+DSAP+SSAP+CTL+ORG+TYPE+DATA+CRC)
//

// Low-32bit Buffer Address

// CVLAN Tag to be inserted if INS_VLAN_TAG is set, CVLAN TPID based on global
// register configuration.
//

// Insert CVlan Tag: When set, ask MAC to insert CVLAN TAG to outgoing packet
//

// High-14bit Buffer Address, So, the 64b-bit address is
// {DESC_CTRL_11_TX_DATA_HIADDR[17:0],(register) BUFFER_ADDR_H, BUFFER_ADDR_L}
// Extend TPD_BUFFER_ADDR_H to [31, 18], because we never enable timestamping.
//

// Format D. Word offset from the 1st byte of this packet to start to calculate
// the custom checksum.
//

// Format D. Word offset from the 1st byte of this packet to fill the custom
// checksum to
//

// Format C. TCP Header offset from the 1st byte of this packet. (byte unit)

// Format C. MSS (Maximum Segment Size) got from the protocol layer. (byte unit)
//

// packet length in ext tpd

}

// emac_ring_header represents a single, contiguous block of DMA space
// mapped for the three descriptor rings (tpd, rfd, rrd)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_ring_header {
    pub /: *mut *mut *mut void v_addr; / virtual address,
    pub /: *mut *mut dma_addr_t dma_addr; / dma address,
    pub /: *mut *mut size_t size; / length in bytes,
    pub used: usize,
}

// emac_buffer is wrapper around a pointer to a socket buffer
// so a DMA handle can be stored along with the skb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_buffer {
    pub /: *mut *mut *mut sk_buff skb; / socket buffer,
    pub /: *mut *mut u16 length; / rx buffer length,
    pub /: *mut *mut dma_addr_t dma_addr; / dma address,
}

// receive free descriptor (rfd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_rfd_ring {
    pub rfbuff: *mut emac_buffer,
    pub /: *mut *mut *mut u32 v_addr; / virtual address,
    pub /: *mut *mut dma_addr_t dma_addr; / dma address,
    pub /: *mut *mut size_t size; / length in bytes,
    pub /: *mut *mut unsigned int count; / number of desc in the ring,
    pub produce_idx: c_uint,
    pub process_idx: c_uint,
    pub /: *mut *mut unsigned int consume_idx; / unused,
}

// Receive Return Desciptor (RRD) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_rrd_ring {
    pub /: *mut *mut *mut u32 v_addr; / virtual address,
    pub /: *mut *mut dma_addr_t dma_addr; / physical address,
    pub /: *mut *mut size_t size; / length in bytes,
    pub /: *mut *mut unsigned int count; / number of desc in the ring,
    pub /: *mut *mut unsigned int produce_idx; / unused,
    pub consume_idx: c_uint,
}

// Rx queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_rx_queue {
    pub /: *mut *mut *mut net_device netdev; / netdev ring belongs to,
    pub rrd: emac_rrd_ring,
    pub rfd: emac_rfd_ring,
    pub napi: napi_struct,
    pub irq: *mut emac_irq,
    pub intr: u32,
    pub produce_mask: u32,
    pub process_mask: u32,
    pub consume_mask: u32,
    pub produce_reg: u16,
    pub process_reg: u16,
    pub consume_reg: u16,
    pub produce_shift: u8,
    pub process_shft: u8,
    pub consume_shift: u8,
}

// Transimit Packet Descriptor (tpd) ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_tpd_ring {
    pub tpbuff: *mut emac_buffer,
    pub /: *mut *mut *mut u32 v_addr; / virtual address,
    pub /: *mut *mut dma_addr_t dma_addr; / dma address,
    pub /: *mut *mut size_t size; / length in bytes,
    pub /: *mut *mut unsigned int count; / number of desc in the ring,
    pub produce_idx: c_uint,
    pub consume_idx: c_uint,
    pub last_produce_idx: c_uint,
}

// Tx queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_tx_queue {
    pub tpd: emac_tpd_ring,
    pub produce_mask: u32,
    pub consume_mask: u32,
    pub /: *mut *mut u16 max_packets; / max packets per interrupt,
    pub produce_reg: u16,
    pub consume_reg: u16,
    pub produce_shift: u8,
    pub consume_shift: u8,
}

extern "C" {
    pub fn emac_mac_up(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_mac_down(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_reset(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_stop(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_mode_config(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_tx_process(adpt: *mut emac_adapter, tx_q: *mut emac_tx_queue);
}
extern "C" {
    pub fn emac_mac_rx_tx_rings_alloc_all(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_mac_rx_tx_rings_free_all(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_multicast_addr_clear(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_mac_multicast_addr_set(adpt: *mut emac_adapter, addr: *mut u8);
}
