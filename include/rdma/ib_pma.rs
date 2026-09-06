//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_pma.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2006, 2007, 2008, 2009, 2010 QLogic Corporation.
// All rights reserved.
// Copyright (c) 2005, 2006 PathScale, Inc. All rights reserved.
//

//
// PMA class portinfo capability mask bits
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_mad {
    pub mad_hdr: ib_mad_hdr,
    pub reserved: [u8; 40],
    pub data: [u8; 192],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portsamplescontrol {
    pub opcode: u8,
    pub port_select: u8,
    pub tick: u8,
    pub /: *mut *mut u8 counter_width; / resv: 7:3, counter width: 2:0,
    pub /: *mut *mut __be32 counter_mask0_9; / 2, 10 3-bit fields,
    pub /: *mut *mut __be16 counter_mask10_14; / 1, 5 3-bit fields,
    pub sample_mechanisms: u8,
    pub /: *mut *mut u8 sample_status; / only lower 2 bits,
    pub option_mask: __be64,
    pub vendor_mask: __be64,
    pub sample_start: __be32,
    pub sample_interval: __be32,
    pub tag: __be16,
    pub counter_select: [__be16; 15],
    pub reserved1: __be32,
    pub samples_only_option_mask: __be64,
    pub reserved2: [__be32; 28],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portsamplesresult {
    pub tag: __be16,
    pub /: *mut *mut __be16 sample_status; / only lower 2 bits,
    pub counter: [__be32; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portsamplesresult_ext {
    pub tag: __be16,
    pub /: *mut *mut __be16 sample_status; / only lower 2 bits,
    pub /: *mut *mut __be32 extended_width; / only upper 2 bits,
    pub counter: [__be64; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portcounters {
    pub reserved: u8,
    pub port_select: u8,
    pub counter_select: __be16,
    pub symbol_error_counter: __be16,
    pub link_error_recovery_counter: u8,
    pub link_downed_counter: u8,
    pub port_rcv_errors: __be16,
    pub port_rcv_remphys_errors: __be16,
    pub port_rcv_switch_relay_errors: __be16,
    pub port_xmit_discards: __be16,
    pub port_xmit_constraint_errors: u8,
    pub port_rcv_constraint_errors: u8,
    pub reserved1: u8,
    pub /: *mut *mut u8 link_overrun_errors; / LocalLink: 7:4, BufferOverrun: 3:0,
    pub reserved2: __be16,
    pub vl15_dropped: __be16,
    pub port_xmit_data: __be32,
    pub port_rcv_data: __be32,
    pub port_xmit_packets: __be32,
    pub port_rcv_packets: __be32,
    pub port_xmit_wait: __be32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pma_portcounters_ext {
    pub reserved: u8,
    pub port_select: u8,
    pub counter_select: __be16,
    pub reserved1: __be32,
    pub port_xmit_data: __be64,
    pub port_rcv_data: __be64,
    pub port_xmit_packets: __be64,
    pub port_rcv_packets: __be64,
    pub port_unicast_xmit_packets: __be64,
    pub port_unicast_rcv_packets: __be64,
    pub port_multicast_xmit_packets: __be64,
    pub port_multicast_rcv_packets: __be64,
    pub __packed: },

