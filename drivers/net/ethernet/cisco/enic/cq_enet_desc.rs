//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/cq_enet_desc.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

// Ethernet completion queue descriptor: 16B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_enet_wq_desc {
    pub completed_index: __le16,
    pub q_number: __le16,
    pub reserved: [u8; 11],
    pub type_color: u8,
}

//
// Defines and Capabilities for CMD_CQ_ENTRY_SIZE_SET
//

pub const VNIC_RQ_CQ_ENTRY_SIZE_16: c_int = 0;
pub const VNIC_RQ_CQ_ENTRY_SIZE_32: c_int = 1;
pub const VNIC_RQ_CQ_ENTRY_SIZE_64: c_int = 2;

// Completion queue descriptor: Ethernet receive queue, 16B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_enet_rq_desc {
    pub completed_index_flags: __le16,
    pub q_number_rss_type_flags: __le16,
    pub rss_hash: __le32,
    pub bytes_written_flags: __le16,
    pub vlan: __le16,
    pub checksum_fcoe: __le16,
    pub flags: u8,
    pub type_color: u8,
}

// Completion queue descriptor: Ethernet receive queue, 32B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_enet_rq_desc_32 {
    pub completed_index_flags: __le16,
    pub q_number_rss_type_flags: __le16,
    pub rss_hash: __le32,
    pub bytes_written_flags: __le16,
    pub vlan: __le16,
    pub checksum_fcoe: __le16,
    pub flags: u8,
    pub fetch_index_flags: u8,
    pub time_stamp: __le32,
    pub time_stamp2: __le16,
    pub pie_info: __le16,
    pub pie_info2: __le32,
    pub pie_info3: __le16,
    pub pie_info4: u8,
    pub type_color: u8,
}

// Completion queue descriptor: Ethernet receive queue, 64B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_enet_rq_desc_64 {
    pub completed_index_flags: __le16,
    pub q_number_rss_type_flags: __le16,
    pub rss_hash: __le32,
    pub bytes_written_flags: __le16,
    pub vlan: __le16,
    pub checksum_fcoe: __le16,
    pub flags: u8,
    pub fetch_index_flags: u8,
    pub time_stamp: __le32,
    pub time_stamp2: __le16,
    pub pie_info: __le16,
    pub pie_info2: __le32,
    pub pie_info3: __le16,
    pub pie_info4: u8,
    pub reserved: [u8; 32],
    pub type_color: u8,
}

pub const CQ_ENET_RQ_DESC_RSS_TYPE_BITS: c_int = 4;

pub const CQ_ENET_RQ_DESC_RSS_TYPE_NONE: c_int = 0;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv4: c_int = 1;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv4: c_int = 2;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv6: c_int = 3;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv6: c_int = 4;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv6_EX: c_int = 5;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv6_EX: c_int = 6;

pub const CQ_ENET_RQ_DESC_BYTES_WRITTEN_BITS: c_int = 14;

pub const CQ_ENET_RQ_DESC_VLAN_TCI_VLAN_BITS: c_int = 12;

pub const CQ_ENET_RQ_DESC_VLAN_TCI_USER_PRIO_BITS: c_int = 3;

pub const CQ_ENET_RQ_DESC_VLAN_TCI_USER_PRIO_SHIFT: c_int = 13;
pub const CQ_ENET_RQ_DESC_FCOE_SOF_BITS: c_int = 8;

pub const CQ_ENET_RQ_DESC_FCOE_EOF_BITS: c_int = 8;

pub const CQ_ENET_RQ_DESC_FCOE_EOF_SHIFT: c_int = 8;

