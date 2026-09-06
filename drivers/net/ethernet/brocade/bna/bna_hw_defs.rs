//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bna_hw_defs.h
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
//
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//
// File for interrupt macros and functions

// SW imposed limits
pub const BFI_ENET_DEF_TXQ: c_int = 1;
pub const BFI_ENET_DEF_RXP: c_int = 1;
pub const BFI_ENET_DEF_UCAM: c_int = 1;
pub const BFI_ENET_DEF_RITSZ: c_int = 1;
pub const BFI_ENET_MAX_MCAM: c_int = 256;

pub const BFI_IBIDX_SIZE: c_int = 4;

pub const BFI_VLAN_WORD_MASK: c_uint = 0x1F;

pub const BFI_VLAN_BMASK_ALL: c_uint = 0xFF;

pub const BFI_MAX_COALESCING_TIMEO: c_uint = 0xFF	/* in 5us units */;
pub const BFI_MAX_INTERPKT_COUNT: c_uint = 0xFF;
pub const BFI_MAX_INTERPKT_TIMEO: c_uint = 0xF	/* in 0.5us units */;

pub const BFI_TX_MAX_WRR_QUOTA: c_uint = 0xFFF;
pub const BFI_TX_MAX_VECTORS_PER_WI: c_int = 4;
pub const BFI_TX_MAX_VECTORS_PER_PKT: c_uint = 0xFF;
pub const BFI_TX_MAX_DATA_PER_VECTOR: c_uint = 0xFFFF;
pub const BFI_TX_MAX_DATA_PER_PKT: c_uint = 0xFFFFFF;
// Small Q buffer size
pub const BFI_SMALL_RXBUF_SIZE: c_int = 128;
pub const BFI_TX_MAX_PRIO: c_int = 8;
pub const BFI_TX_PRIO_MAP_ALL: c_uint = 0xFF;
//
// Register definitions and macros
//

// Interrupt related bits, flags and macros
pub const IB_STATUS_BITS: c_uint = 0x0000ffff;

//
// MAX ACK EVENTS : No. of acks that can be accumulated in driver,
// before acking to h/w. The no. of bits is 16 in the doorbell register,
// however we keep this limited to 15 bits.
// This is because around the edge of 64K boundary (16 bits), one
// single poll can make the accumulated ACK counter cross the 64K boundary,
// causing problems, when we try to ack with a value greater than 64K.
// 15 bits (32K) should  be large enough to accumulate, anyways, and the max.
// acked events to h/w can be (32K + max poll weight) (currently 64).
//

// These macros build the data portion of the TxQ/RxQ doorbell

// These macros build the data portion of the IB doorbell

// Set the coalescing timer for the given ib

// Acks 'events' # of events for a given ib while disabling interrupts

// Acks 'events' # of events for a given ib

// TxQ, RxQ, CQ related bits, offsets, macros
// TxQ Entry Opcodes

// TxQ Entry Control Flags

//
// Completion Q defines
//
// CQ Entry Flags

// CAT2 ASIC does not use bit 21 as per the SPEC.
// Bit 31 is set in every end of frame completion
//

// Data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_reg_offset {
    pub fn_int_status: u32,
    pub fn_int_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_bit_defn {
    pub mbox_status_bits: u32,
    pub mbox_mask_bits: u32,
    pub error_status_bits: u32,
    pub error_mask_bits: u32,
    pub halt_status_bits: u32,
    pub halt_mask_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_reg {
    pub fn_int_status: *mut void __iomem,
    pub fn_int_mask: *mut void __iomem,
}

// TxQ Vector (a.k.a. Tx-Buffer Descriptor)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_dma_addr {
    pub msb: u32,
    pub lsb: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_txq_wi_vector {
    pub reserved: u16,
    pub /: *mut *mut u16 length; / Only 14 LSB are valid,
    pub /: *mut *mut bna_dma_addr host_addr; / Tx-Buf DMA addr,
}

// TxQ Entry Structure
//
// BEWARE:  Load values into this structure with correct endianness.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_txq_entry {
    pub reserved: u8,
    pub /: *mut *mut u8 num_vectors; / number of vectors present,
    pub /: *mut *mut u16 opcode; / Either,
// BNA_TXQ_WI_SEND or
// BNA_TXQ_WI_SEND_LSO
    pub /: *mut *mut u16 flags; / OR of all the flags,
    pub l4_hdr_size_n_offset: u16,
    pub vlan_tag: u16,
    pub /: *mut *mut u16 lso_mss; / Only 14 LSB are valid,
    pub /: *mut *mut u32 frame_length; / Only 24 LSB are valid,
    pub wi: },
    pub reserved: u16,
    pub /: *mut *mut u16 opcode; / Must be,
// BNA_TXQ_WI_EXTENSION
    pub /: *mut *mut u32 reserved2[3]; / Place holder for,
// removed vector (12 bytes)
    pub wi_ext: },
    pub hdr: },
    pub vector: [bna_txq_wi_vector; 4],
}

// RxQ Entry Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_rxq_entry {
    pub /: *mut *mut bna_dma_addr host_addr; / Rx-Buffer DMA address,
}

// CQ Entry Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bna_cq_entry {
    pub flags: u32,
    pub vlan_tag: u16,
    pub length: u16,
    pub rss_hash: u32,
    pub valid: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub rxq_id: u8,
}
