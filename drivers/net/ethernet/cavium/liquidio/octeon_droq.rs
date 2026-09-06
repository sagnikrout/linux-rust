//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_droq.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// !  \file  octeon_droq.h
// \brief Implementation of Octeon Output queues. "Output" is with
// respect to the Octeon device on the NIC. From this driver's point of
// view they are ingress queues.
//
// Default number of packets that will be processed in one iteration.
pub const MAX_PACKET_BUDGET: c_uint = 0xFFFFFFFF;
// Octeon descriptor format.
// The descriptor ring is made of descriptors which have 2 64-bit values:
// -# Physical (bus) address of the data buffer.
// -# Physical (bus) address of a octeon_droq_info structure.
// The Octeon device DMA's incoming packets and its information at the address
// given by these descriptor fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_droq_desc {
// The buffer pointer
    pub buffer_ptr: u64,
// The Info pointer
    pub info_ptr: u64,
}

// Information about packet DMA'ed by Octeon.
// The format of the information available at Info Pointer after Octeon
// has posted a packet. Not all descriptors have valid information. Only
// the Info field of the first descriptor for a packet has information
// about the packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_droq_info {
// The Length of the packet.
    pub length: u64,
// The Output Receive Header.
    pub rh: octeon_rh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_skb_page_info {
// DMA address for the page
    pub dma: dma_addr_t,
// Page for the rx dma
    pub page: *mut page,
// which offset into page
    pub page_offset: c_uint,
}

// Pointer to data buffer.
// Driver keeps a pointer to the data buffer that it made available to
// the Octeon device. Since the descriptor ring keeps physical (bus)
// addresses, this field is required for the driver to keep track of
// the virtual address pointers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_recv_buffer {
// Packet buffer, including metadata.
    pub buffer: *mut c_void,
// Data in the packet buffer.
    pub data: *mut u8,
// pg_info
    pub pg_info: octeon_skb_page_info,
}

// Output Queue statistics. Each output queue has four stats fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_droq_stats {
// Number of packets received in this queue.
    pub pkts_received: u64,
// Bytes received by this queue.
    pub bytes_received: u64,
// Packets dropped due to no dispatch function.
    pub dropped_nodispatch: u64,
// Packets dropped due to no memory available.
    pub dropped_nomem: u64,
// Packets dropped due to large number of pkts to process.
    pub dropped_toomany: u64,
// Number of packets  sent to stack from this queue.
    pub rx_pkts_received: u64,
// Number of Bytes sent to stack from this queue.
    pub rx_bytes_received: u64,
// Num of Packets dropped due to receive path failures.
    pub rx_dropped: u64,
    pub rx_vxlan: u64,
// Num of failures of recv_buffer_alloc()
    pub rx_alloc_failure: u64,
}

// The maximum number of buffers that can be dispatched from the
// output/dma queue. Set to 64 assuming 1K buffers in DROQ and the fact that
// max packet size from DROQ is 64K.
//
pub const MAX_RECV_BUFS: c_int = 64;
// Receive Packet format used when dispatching output queue packets
// with non-raw opcodes.
// The received packet will be sent to the upper layers using this
// structure which is passed as a parameter to the dispatch function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_recv_pkt {
// Number of buffers in this received packet
    pub buffer_count: u16,
// Id of the device that is sending the packet up
    pub octeon_id: u16,
// Length of data in the packet buffer
    pub length: u32,
// The receive header
    pub rh: octeon_rh,
// Pointer to the OS-specific packet buffer
    pub buffer_ptr: [*mut c_void; MAX_RECV_BUFS],
// Size of the buffers pointed to by ptr's in buffer_ptr
    pub buffer_size: [u32; MAX_RECV_BUFS],
}

// The first parameter of a dispatch function.
// For a raw mode opcode, the driver dispatches with the device
// pointer in this structure.
// For non-raw mode opcode, the driver dispatches the recv_pkt
// created to contain the buffers with data received from Octeon.
// ---------------------
// |     *recv_pkt ----|---
// |-------------------|   |
// | 0 or more bytes   |   |
// | reserved by driver|   |
// |-------------------|<-
// | octeon_recv_pkt   |
// |                   |
// |___________________|
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_recv_info {
    pub rsvd: *mut c_void,
    pub recv_pkt: *mut octeon_recv_pkt,
}

// Allocate a recv_info structure. The recv_pkt pointer in the recv_info
// structure is filled in before this call returns.
// @param extra_bytes - extra bytes to be allocated at the end of the recv info
// structure.
// @return - pointer to a newly allocated recv_info structure.
//
// Free a recv_info structure.
// @param recv_info - Pointer to receive_info to be freed
//
extern "C" {
    pub fn int(: *mut *mut octeon_dispatch_fn_t)(struct octeon_recv_info, : *mut c_void) -> typedef;
}
// Used by NIC module to register packet handler and to get device
// information for each octeon device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_droq_ops {
// This registered function will be called by the driver with
// the octeon id, pointer to buffer from droq and length of
// data in the buffer. The receive header gives the port
// number to the caller.  Function pointer is set by caller.
//
    pub ): *mut *mut *mut *mut *mut void (fptr)(u32, void , u32, union octeon_rh , void , void,
    pub farg: *mut c_void,
// This function will be called by the driver for all NAPI related
// events. The first param is the octeon id. The second param is the
// output queue number. The third is the NAPI event that occurred.
//
    pub ): *mut *mut void (napi_fn)(void,
    pub poll_mode: u32,
// Flag indicating if the DROQ handler should drop packets that
// it cannot handle in one iteration. Set by caller.
//
    pub drop_on_max: u32,
}

// The Descriptor Ring Output Queue structure.
// This structure has all the information required to implement a
// Octeon DROQ.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_droq {
    pub q_no: u32,
    pub pkt_count: u32,
    pub ops: octeon_droq_ops,
    pub oct_dev: *mut octeon_device,
// The 8B aligned descriptor ring starts at this address.
    pub desc_ring: *mut octeon_droq_desc,
// Index in the ring where the driver should read the next packet
    pub read_idx: u32,
// Index in the ring where Octeon will write the next packet
    pub write_idx: u32,
// Index in the ring where the driver will refill the descriptor's
// buffer
//
    pub refill_idx: u32,
// Packets pending to be processed
    pub pkts_pending: core::sync::atomic::AtomicI32,
// Number of  descriptors in this ring.
    pub max_count: u32,
// The number of descriptors pending refill.
    pub refill_count: u32,
    pub pkts_per_intr: u32,
    pub refill_threshold: u32,
// The max number of descriptors in DROQ without a buffer.
// This field is used to keep track of empty space threshold. If the
// refill_count reaches this value, the DROQ cannot accept a max-sized
// (64K) packet.
//
    pub max_empty_descs: u32,
// The receive buffer list. This list has the virtual addresses of the
// buffers.
//
    pub recv_buf_list: *mut octeon_recv_buffer,
// The size of each buffer pointed by the buffer pointer.
    pub buffer_size: u32,
// Pointer to the mapped packet credit register.
// Host writes number of info/buffer ptrs available to this register
//
    pub pkts_credit_reg: *mut void __iomem,
// Pointer to the mapped packet sent register.
// Octeon writes the number of packets DMA'ed to host memory
// in this register.
//
    pub pkts_sent_reg: *mut void __iomem,
    pub dispatch_list: list_head,
// Statistics for this DROQ.
    pub stats: oct_droq_stats,
// DMA mapped address of the DROQ descriptor ring.
    pub desc_ring_dma: usize,
// application context
    pub app_ctx: *mut c_void,
    pub napi: napi_struct,
    pub cpu_id: u32,
    pub csd: call_single_data_t,
}

//
// Allocates space for the descriptor ring for the droq and sets the
// base addr, num desc etc in Octeon registers.
//
// @param  oct_dev    - pointer to the octeon device structure
// @param  q_no       - droq no. ranges from 0 - 3.
// @param app_ctx     - pointer to application context
// @return Success: 0    Failure: 1
//
// Frees the space for descriptor ring for the droq.
//
// @param oct_dev - pointer to the octeon device structure
// @param q_no    - droq no. ranges from 0 - 3.
// @return:    Success: 0    Failure: 1
//
extern "C" {
    pub fn octeon_delete_droq(oct_dev: *mut octeon_device, q_no: u32) -> c_int;
}
// Register a change in droq operations. The ops field has a pointer to a
// function which will called by the DROQ handler for all packets arriving
// on output queues given by q_no irrespective of the type of packet.
// The ops field also has a flag which if set tells the DROQ handler to
// drop packets if it receives more than what it can process in one
// invocation of the handler.
// @param oct       - octeon device
// @param q_no      - octeon output queue number (0 <= q_no <= MAX_OCTEON_DROQ-1
// @param ops       - the droq_ops settings for this queue
// @return          - 0 on success, -ENODEV or -EINVAL on error.
//
// Resets the function pointer and flag settings made by
// octeon_register_droq_ops(). After this routine is called, the DROQ handler
// will lookup dispatch function for each arriving packet on the output queue
// given by q_no.
// @param oct       - octeon device
// @param q_no      - octeon output queue number (0 <= q_no <= MAX_OCTEON_DROQ-1
// @return          - 0 on success, -ENODEV or -EINVAL on error.
//
extern "C" {
    pub fn octeon_unregister_droq_ops(oct: *mut octeon_device, q_no: u32) -> c_int;
}
// Register a dispatch function for a opcode/subcode. The driver will call
// this dispatch function when it receives a packet with the given
// opcode/subcode in its output queues along with the user specified
// argument.
// @param  oct        - the octeon device to register with.
// @param  opcode     - the opcode for which the dispatch will be registered.
// @param  subcode    - the subcode for which the dispatch will be registered
// @param  fn         - the dispatch function.
// @param  fn_arg     - user specified that will be passed along with the
// dispatch function by the driver.
// @return Success: 0; Failure: 1
//
extern "C" {
    pub fn octeon_droq_check_hw_for_pkts(droq: *mut octeon_droq) -> u32;
}
extern "C" {
    pub fn octeon_enable_irq(oct: *mut octeon_device, q_no: u32) -> c_int;
}
extern "C" {
    pub fn octeon_retry_droq_refill(droq: *mut octeon_droq) -> c_int;
}
