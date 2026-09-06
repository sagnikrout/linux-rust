//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_tx.h
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
pub const IQ_SEND_OK: c_int = 0;
pub const IQ_SEND_STOP: c_int = 1;

pub const TX_BUFTYPE_NONE: c_int = 0;
pub const TX_BUFTYPE_NET: c_int = 1;
pub const TX_BUFTYPE_NET_SG: c_int = 2;
pub const NUM_TX_BUFTYPES: c_int = 3;
// Hardware format for Scatter/Gather list
//
// 63      48|47     32|31     16|15       0
// -----------------------------------------
// |  Len 0  |  Len 1  |  Len 2  |  Len 3  |
// -----------------------------------------
// |                Ptr 0                  |
// -----------------------------------------
// |                Ptr 1                  |
// -----------------------------------------
// |                Ptr 2                  |
// -----------------------------------------
// |                Ptr 3                  |
// -----------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_tx_sglist_desc {
    pub len: [u16; 4],
    pub dma_ptr: [dma_addr_t; 4],
}

// Each Scatter/Gather entry sent to hardwar hold four pointers.
// So, number of entries required is (MAX_SKB_FRAGS + 1)/4, where '+1'
// is for main skb which also goes as a gather buffer to Octeon hardware.
// To allocate sufficient SGLIST entries for a packet with max fragments,
// align by adding 3 before calcuating max SGLIST entries per packet.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_tx_buffer {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub sglist: *mut octep_tx_sglist_desc,
    pub sglist_dma: dma_addr_t,
    pub gather: u8,
}

// Hardware interface Tx statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iface_tx_stats {
// Total frames sent on the interface
    pub pkts: u64,
// Total octets sent on the interface
    pub octs: u64,
// Packets sent to a broadcast DMAC
    pub bcst: u64,
// Packets sent to the multicast DMAC
    pub mcst: u64,
// Packets dropped due to excessive collisions
    pub xscol: u64,
// Packets dropped due to excessive deferral
    pub xsdef: u64,
// Packets sent that experienced multiple collisions before successful
// transmission
//
    pub mcol: u64,
// Packets sent that experienced a single collision before successful
// transmission
//
    pub scol: u64,
// Packets sent with an octet count < 64
    pub hist_lt64: u64,
// Packets sent with an octet count == 64
    pub hist_eq64: u64,
// Packets sent with an octet count of 65–127
    pub hist_65to127: u64,
// Packets sent with an octet count of 128–255
    pub hist_128to255: u64,
// Packets sent with an octet count of 256–511
    pub hist_256to511: u64,
// Packets sent with an octet count of 512–1023
    pub hist_512to1023: u64,
// Packets sent with an octet count of 1024-1518
    pub hist_1024to1518: u64,
// Packets sent with an octet count of > 1518
    pub hist_gt1518: u64,
// Packets sent that experienced a transmit underflow and were
// truncated
//
    pub undflw: u64,
// Control/PAUSE packets sent
    pub ctl: u64,
}

// Input Queue statistics. Each input queue has four stats fields.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iq_stats {
// Instructions posted to this queue.
    pub instr_posted: u64,
// Instructions copied by hardware for processing.
    pub instr_completed: u64,
// Instructions that could not be processed.
    pub instr_dropped: u64,
// Bytes sent through this queue.
    pub bytes_sent: u64,
// Gather entries sent through this queue.
    pub sgentry_sent: u64,
// Number of transmit failures due to TX_BUSY
    pub tx_busy: u64,
// Number of times the queue is restarted
    pub restart_cnt: u64,
}

// The instruction (input) queue.
// The input queue is used to post raw (instruction) mode data or packet
// data to Octeon device from the host. Each input queue (up to 4) for
// a Octeon device has one such structure to represent it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iq {
    pub q_no: u32,
    pub octep_dev: *mut octep_device,
    pub netdev: *mut net_device,
    pub dev: *mut device,
    pub netdev_q: *mut netdev_queue,
// Index in input ring where driver should write the next packet
    pub host_write_index: u16,
// Index in input ring where Octeon is expected to read next packet
    pub octep_read_index: u16,
// This index aids in finding the window in the queue where Octeon
// has read the commands.
//
    pub flush_index: u16,
// Pointer to statistics for this input queue.
    pub stats: *mut octep_iq_stats,
// Pointer to the Virtual Base addr of the input ring.
    pub desc_ring: *mut octep_tx_desc_hw,
// DMA mapped base address of the input descriptor ring.
    pub desc_ring_dma: dma_addr_t,
// Info of Tx buffers pending completion.
    pub buff_info: *mut octep_tx_buffer,
// Base pointer to Scatter/Gather lists for all ring descriptors.
    pub sglist: *mut octep_tx_sglist_desc,
// DMA mapped addr of Scatter Gather Lists
    pub sglist_dma: dma_addr_t,
// Octeon doorbell register for the ring.
    pub doorbell_reg: *mut u8 __iomem,
// Octeon instruction count register for this ring.
    pub inst_cnt_reg: *mut u8 __iomem,
// interrupt level register for this ring
    pub intr_lvl_reg: *mut u8 __iomem,
// Maximum no. of instructions in this queue.
    pub max_count: u32,
    pub ring_size_mask: u32,
    pub pkt_in_done: u32,
    pub pkts_processed: u32,
    pub status: u32,
// Number of instructions pending to be posted to Octeon.
    pub fill_cnt: u32,
// The max. number of instructions that can be held pending by the
// driver before ringing doorbell.
//
    pub fill_threshold: u32,
}

// Hardware Tx Instruction Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_instr_hdr {
// Data Len
    pub tlen:16: u64,
// Reserved
    pub rsvd:20: u64,
// PKIND for SDP
    pub pkind:6: u64,
// Front Data size
    pub fsz:6: u64,
// No. of entries in gather list
    pub gsz:14: u64,
// Gather indicator 1=gather
    pub gather:1: u64,
// Reserved3
    pub reserved3:1: u64,
}

// Tx offload flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_mdata {
// offload flags
    pub ol_flags: u16,
// gso size
    pub gso_size: u16,
// gso flags
    pub gso_segs: u16,
// reserved
    pub rsvd1: u16,
// reserved
    pub rsvd2: u64,
}

// 64-byte Tx instruction format.
// Format of instruction for a 64-byte mode input queue.
//
// only first 16-bytes (dptr and ih) are mandatory; rest are optional
// and filled by the driver based on firmware/hardware capabilities.
// These optional headers together called Front Data and its size is
// described by ih->fsz.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_tx_desc_hw {
// Pointer where the input data is available.
    pub dptr: u64,
// Instruction Header.
    pub ih: octep_instr_hdr,
    pub ih64: u64,
}

// Additional headers available in a 64-byte instruction.

