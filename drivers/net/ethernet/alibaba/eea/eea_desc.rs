//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_desc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq_desc {
    pub flags: __le16,
    pub id: __le16,
    pub reserved: __le16,
    pub classid: u8,
    pub command: u8,
    pub data_addr: __le64,
    pub reply_addr: __le64,
    pub data_len: __le32,
    pub reply_len: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq_cdesc {
    pub flags: __le16,
    pub id: __le16,
pub const EEA_OK: c_int = 0;
pub const EEA_ERR: c_uint = 0xffffffff;
    pub status: __le32,
    pub reply_len: __le32,
    pub reserved1: __le32,
    pub reserved2: __le64,
    pub reserved3: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_desc_no_hdr {
    pub flags: __le16,
    pub id: __le16,
    pub len: __le16,
    pub reserved1: __le16,
    pub addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_desc {
    pub flags: __le16,
    pub id: __le16,
    pub len: __le16,
    pub reserved1: __le16,
    pub addr: __le64,
    pub hdr_addr: __le64,
    pub reserved2: __le32,
    pub reserved3: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_cdesc {

    pub flags: __le16,
    pub id: __le16,
    pub len: __le16,
pub const EEA_NET_PT_NONE: c_int = 0;
pub const EEA_NET_PT_IPv4: c_int = 1;
pub const EEA_NET_PT_TCPv4: c_int = 2;
pub const EEA_NET_PT_UDPv4: c_int = 3;
pub const EEA_NET_PT_IPv6: c_int = 4;
pub const EEA_NET_PT_TCPv6: c_int = 5;
pub const EEA_NET_PT_UDPv6: c_int = 6;
pub const EEA_NET_PT_IPv6_EX: c_int = 7;
pub const EEA_NET_PT_TCPv6_EX: c_int = 8;
pub const EEA_NET_PT_UDPv6_EX: c_int = 9;
// [9:0] is packet type.
    pub type: __le16,
// hw timestamp [0:47]: ts
    pub ts: __le64,
    pub hash: __le32,
// 0-9: hdr_len  split header
// 10-15: reserved1
//
    pub len_ex: __le16,
    pub reserved2: __le16,
    pub reserved3: __le32,
    pub reserved4: __le32,
}

pub const EEA_TX_GSO_NONE: c_int = 0;
pub const EEA_TX_GSO_TCPV4: c_int = 1;
pub const EEA_TX_GSO_TCPV6: c_int = 4;
pub const EEA_TX_GSO_UDP_L4: c_int = 5;
pub const EEA_TX_GSO_ECN: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_tx_desc {

    pub flags: __le16,
    pub id: __le16,
    pub len: __le16,
    pub reserved1: __le16,
    pub addr: __le64,
    pub csum_start: __le16,
    pub csum_offset: __le16,
    pub gso_type: u8,
    pub reserved2: u8,
    pub gso_size: __le16,
    pub reserved3: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_tx_cdesc {
    pub flags: __le16,
    pub id: __le16,
    pub len: __le16,
    pub reserved1: __le16,
// hw timestamp [0:47]: ts
    pub ts: __le64,
}

pub const EEA_DB_FLAGS_OFF: c_int = 0;

