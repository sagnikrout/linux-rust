//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_rx.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//
// struct octep_oq_desc_hw - Octeon Hardware OQ descriptor format.
//
// The descriptor ring is made of descriptors which have 2 64-bit values:
//
// @buffer_ptr: DMA address of the skb->data
// @info_ptr:  DMA address of host memory, used to update pkt count by hw.
// This is currently unused to save pci writes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq_desc_hw {
    pub buffer_ptr: dma_addr_t,
    pub info_ptr: u64,
}

// Rx offload flags

// bit 0 is vlan strip

// Extended Response Header in packet data received from Hardware.
// Includes metadata like checksum status.
// this is valid only if hardware/firmware published support for this.
// This is at offset 0 of packet data (skb->data).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq_resp_hw_ext {
// Reserved.
    pub rsvd:48: u64,
// offload flags
    pub rx_ol_flags: u16,
}

// Length of Rx packet DMA'ed by Octeon to Host.
// this is in bigendian; so need to be converted to cpu endian.
// Octeon writes this at the beginning of Rx buffer (skb->data).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq_resp_hw {
// The Length of the packet.
    pub length: __be64,
}

// Pointer to data buffer.
// Driver keeps a pointer to the data buffer that it made available to
// the Octeon device. Since the descriptor ring keeps physical (bus)
// addresses, this field is required for the driver to keep track of
// the virtual address pointers. The fields are operated by
// OS-dependent routines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_rx_buffer {
    pub page: *mut page,
// length from rx hardware descriptor after converting to cpu endian
    pub len: u64,
}

// Output Queue statistics. Each output queue has four stats fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq_stats {
// Number of packets received from the Device.
    pub packets: u64,
// Number of bytes received from the Device.
    pub bytes: u64,
// Number of times failed to allocate buffers.
    pub alloc_failures: u64,
}

// Hardware interface Rx statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iface_rx_stats {
// Received packets
    pub pkts: u64,
// Octets of received packets
    pub octets: u64,
// Received PAUSE and Control packets
    pub pause_pkts: u64,
// Received PAUSE and Control octets
    pub pause_octets: u64,
// Filtered DMAC0 packets
    pub dmac0_pkts: u64,
// Filtered DMAC0 octets
    pub dmac0_octets: u64,
// Packets dropped due to RX FIFO full
    pub dropped_pkts_fifo_full: u64,
// Octets dropped due to RX FIFO full
    pub dropped_octets_fifo_full: u64,
// Error packets
    pub err_pkts: u64,
// Filtered DMAC1 packets
    pub dmac1_pkts: u64,
// Filtered DMAC1 octets
    pub dmac1_octets: u64,
// NCSI-bound packets dropped
    pub ncsi_dropped_pkts: u64,
// NCSI-bound octets dropped
    pub ncsi_dropped_octets: u64,
// Multicast packets received.
    pub mcast_pkts: u64,
// Broadcast packets received.
    pub bcast_pkts: u64,
}

// The Descriptor Ring Output Queue structure.
// This structure has all the information required to implement a
// Octeon OQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_oq {
    pub q_no: u32,
    pub octep_dev: *mut octep_device,
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub napi: *mut napi_struct,
// The receive buffer list. This list has the virtual addresses
// of the buffers.
//
    pub buff_info: *mut octep_rx_buffer,
// Pointer to the mapped packet credit register.
// Host writes number of info/buffer ptrs available to this register
//
    pub pkts_credit_reg: *mut u8 __iomem,
// Pointer to the mapped packet sent register.
// Octeon writes the number of packets DMA'ed to host memory
// in this register.
//
    pub pkts_sent_reg: *mut u8 __iomem,
// Pointer to statistics for this OQ.
    pub stats: *mut octep_oq_stats,
// Packets pending to be processed
    pub pkts_pending: u32,
    pub last_pkt_count: u32,
// Index in the ring where the driver should read the next packet
    pub host_read_idx: u32,
// Number of  descriptors in this ring.
    pub max_count: u32,
    pub ring_size_mask: u32,
// The number of descriptors pending refill.
    pub refill_count: u32,
// Index in the ring where the driver will refill the
// descriptor's buffer
//
    pub host_refill_idx: u32,
    pub refill_threshold: u32,
// The size of each buffer pointed by the buffer pointer.
    pub buffer_size: u32,
    pub max_single_buffer_size: u32,
// The 8B aligned descriptor ring starts at this address.
    pub desc_ring: *mut octep_oq_desc_hw,
// DMA mapped address of the OQ descriptor ring.
    pub desc_ring_dma: dma_addr_t,
}

