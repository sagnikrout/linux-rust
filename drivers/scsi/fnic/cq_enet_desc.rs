//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/cq_enet_desc.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
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

pub const CQ_ENET_RQ_DESC_RSS_TYPE_BITS: c_int = 4;

pub const CQ_ENET_RQ_DESC_RSS_TYPE_NONE: c_int = 0;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv4: c_int = 1;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv4: c_int = 2;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv6: c_int = 3;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv6: c_int = 4;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_IPv6_EX: c_int = 5;
pub const CQ_ENET_RQ_DESC_RSS_TYPE_TCP_IPv6_EX: c_int = 6;

pub const CQ_ENET_RQ_DESC_BYTES_WRITTEN_BITS: c_int = 14;

pub const CQ_ENET_RQ_DESC_FCOE_SOF_BITS: c_int = 4;

pub const CQ_ENET_RQ_DESC_FCOE_EOF_BITS: c_int = 8;

pub const CQ_ENET_RQ_DESC_FCOE_EOF_SHIFT: c_int = 8;

// ingress_port = (completed_index_flags &
// fcoe = (completed_index_flags & CQ_ENET_RQ_DESC_FLAGS_FCOE) ?
// eop = (completed_index_flags & CQ_ENET_RQ_DESC_FLAGS_EOP) ?
// sop = (completed_index_flags & CQ_ENET_RQ_DESC_FLAGS_SOP) ?
// rss_type = (u8)((q_number_rss_type_flags >> CQ_DESC_Q_NUM_BITS) &
// csum_not_calc = (q_number_rss_type_flags &
// rss_hash = le32_to_cpu(desc->rss_hash);
// bytes_written = bytes_written_flags &
// packet_error = (bytes_written_flags &
// vlan_stripped = (bytes_written_flags &
// vlan = le16_to_cpu(desc->vlan);
// fcoe_sof = (u8)(le16_to_cpu(desc->checksum_fcoe) &
// fcoe_fc_crc_ok = (desc->flags &
// fcoe_enc_error = (desc->flags &
// fcoe_eof = (u8)((desc->checksum_fcoe >>
// checksum = 0;
// fcoe_sof = 0;
// fcoe_fc_crc_ok = 0;
// fcoe_enc_error = 0;
// fcoe_eof = 0;
// checksum = le16_to_cpu(desc->checksum_fcoe);
// tcp_udp_csum_ok =
// udp = (desc->flags & CQ_ENET_RQ_DESC_FLAGS_UDP) ? 1 : 0;
// tcp = (desc->flags & CQ_ENET_RQ_DESC_FLAGS_TCP) ? 1 : 0;
// ipv4_csum_ok =
// ipv6 = (desc->flags & CQ_ENET_RQ_DESC_FLAGS_IPV6) ? 1 : 0;
// ipv4 = (desc->flags & CQ_ENET_RQ_DESC_FLAGS_IPV4) ? 1 : 0;
// ipv4_fragment =
// fcs_ok = (desc->flags & CQ_ENET_RQ_DESC_FLAGS_FCS_OK) ? 1 : 0;
