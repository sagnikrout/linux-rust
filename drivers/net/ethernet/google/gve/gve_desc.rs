//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_desc.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2019 Google, Inc.
//
// GVE Transmit Descriptor formats

// A note on seg_addrs
//
// Base addresses encoded in seg_addr are not assumed to be physical
// addresses. The ring format assumes these come from some linear address
// space. This could be physical memory, kernel virtual memory, user virtual
// memory.
// If raw dma addressing is not supported then gVNIC uses lists of registered
// pages. Each queue is assumed to be associated with a single such linear
// address space to ensure a consistent meaning for seg_addrs posted to its
// rings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_pkt_desc {
    pub /: *mut *mut u8 type_flags; / desc type is lower 4 bits, flags upper,
    pub /: *mut *mut u8 l4_csum_offset; / relative offset of L4 csum word,
    pub /: *mut *mut u8 l4_hdr_offset; / Offset of start of L4 headers in packet,
    pub /: *mut *mut u8 desc_cnt; / Total descriptors for this packet,
    pub /: *mut *mut __be16 len; / Total length of this packet (in bytes),
    pub /: *mut *mut __be16 seg_len; / Length of this descriptor's segment,
    pub /: *mut *mut __be64 seg_addr; / Base address (see note) of this segment,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_mtd_desc {
    pub /: *mut *mut u8 type_flags; / type is lower 4 bits, subtype upper,
    pub /: *mut *mut u8 path_state; / state is lower 4 bits, hash type upper,
    pub reserved0: __be16,
    pub path_hash: __be32,
    pub reserved1: __be64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_seg_desc {
    pub /: *mut *mut u8 type_flags; / type is lower 4 bits, flags upper,
    pub /: *mut *mut u8 l3_offset; / TSO: 2 byte units to start of IPH,
    pub reserved: __be16,
    pub /: *mut *mut __be16 mss; / TSO MSS,
    pub seg_len: __be16,
    pub seg_addr: __be64,
    pub __packed: },
// GVE Transmit Descriptor Types

// GVE Transmit Descriptor Flags for Std Pkts

// GVE Transmit Descriptor Flags for TSO Segs

// GVE Transmit Descriptor Options for MTD Segs
pub const GVE_MTD_SUBTYPE_PATH: c_int = 0;
pub const GVE_MTD_PATH_STATE_DEFAULT: c_int = 0;
pub const GVE_MTD_PATH_STATE_TIMEOUT: c_int = 1;
pub const GVE_MTD_PATH_STATE_CONGESTION: c_int = 2;
pub const GVE_MTD_PATH_STATE_RETRANSMIT: c_int = 3;

// GVE Receive Packet Descriptor
// The start of an ethernet packet comes 2 bytes into the rx buffer.
// gVNIC adds this padding so that both the DMA and the L3/4 protocol header
// access is aligned.
//
pub const GVE_RX_PAD: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_desc {
    pub padding: [u8; 48],
    pub /: *mut *mut __be32 rss_hash; / Receive-side scaling hash (Toeplitz for gVNIC),
    pub mss: __be16,
    pub /: *mut *mut __be16 reserved; / Reserved to zero,
    pub /: *mut *mut u8 hdr_len; / Header length (L2-L4) including padding,
    pub /: *mut *mut u8 hdr_off; / 64-byte-scaled offset into RX_DATA entry,
    pub /: *mut *mut __sum16 csum; / 1's-complement partial checksum of L3+ bytes,
    pub /: *mut *mut __be16 len; / Length of the received packet,
    pub /: *mut *mut __be16 flags_seq; / Flags [15:3] and sequence number [2:0] (1-7),
    pub __packed: },
    pub 64): static_assert(sizeof(struct gve_rx_desc) ==,
// If the device supports raw dma addressing then the addr in data slot is
// the dma address of the buffer.
// If the device only supports registered segments then the addr is a byte
// offset into the registered segment (an ordered list of pages) where the
// buffer is.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union gve_rx_data_slot {
    pub qpl_offset: __be64,
    pub addr: __be64,
}

// GVE Receive Packet Descriptor Seq No

// GVE Receive Packet Descriptor Flags

// GVE IRQ

