//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_pack.h
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
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_field {
    pub struct_offset_bytes: usize,
    pub struct_size_bytes: usize,
    pub offset_words: c_int,
    pub offset_bits: c_int,
    pub size_bits: c_int,
    pub field_name: *mut c_char,
}

//
// This macro cleans up the definitions of constants for BTH opcodes.
// It is used to define constants such as IB_OPCODE_UD_SEND_ONLY,
// which becomes IB_OPCODE_UD + IB_OPCODE_SEND_ONLY, and this gives
// the correct value.
//
// In short, user code should use the constants defined using the
// macro rather than worrying about adding together other constants.
//

// transport types -- just used to define real constants
// per IBTA 1.3 vol 1 Table 38, A10.3.2
// Manufacturer specific
// operations -- just used to define real constants
// opcode 0x15 is reserved
// real constants follow -- see comment about above IB_OPCODE()
// RC
// UC
// RD
// UD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_lrh {
    pub virtual_lane: u8,
    pub link_version: u8,
    pub service_level: u8,
    pub link_next_header: u8,
    pub destination_lid: __be16,
    pub packet_length: __be16,
    pub source_lid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_grh {
    pub ip_version: u8,
    pub traffic_class: u8,
    pub flow_label: __be32,
    pub payload_length: __be16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source_gid: ib_gid,
    pub destination_gid: ib_gid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_bth {
    pub opcode: u8,
    pub solicited_event: u8,
    pub mig_req: u8,
    pub pad_count: u8,
    pub transport_header_version: u8,
    pub pkey: __be16,
    pub destination_qpn: __be32,
    pub ack_req: u8,
    pub psn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_deth {
    pub qkey: __be32,
    pub source_qpn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_eth {
    pub dmac_h: [u8; 4],
    pub dmac_l: [u8; 2],
    pub smac_h: [u8; 2],
    pub smac_l: [u8; 4],
    pub type: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_ip4 {
    pub ver: u8,
    pub hdr_len: u8,
    pub tos: u8,
    pub tot_len: __be16,
    pub id: __be16,
    pub frag_off: __be16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: __sum16,
    pub saddr: __be32,
    pub daddr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_udp {
    pub sport: __be16,
    pub dport: __be16,
    pub length: __be16,
    pub csum: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_unpacked_vlan {
    pub tag: __be16,
    pub type: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ud_header {
    pub lrh_present: c_int,
    pub lrh: ib_unpacked_lrh,
    pub eth_present: c_int,
    pub eth: ib_unpacked_eth,
    pub vlan_present: c_int,
    pub vlan: ib_unpacked_vlan,
    pub grh_present: c_int,
    pub grh: ib_unpacked_grh,
    pub ipv4_present: c_int,
    pub ip4: ib_unpacked_ip4,
    pub udp_present: c_int,
    pub udp: ib_unpacked_udp,
    pub bth: ib_unpacked_bth,
    pub deth: ib_unpacked_deth,
    pub immediate_present: c_int,
    pub immediate_data: __be32,
}

extern "C" {
    pub fn ib_ud_ip4_csum(header: *mut ib_ud_header) -> __sum16;
}
