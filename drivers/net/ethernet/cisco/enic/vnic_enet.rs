//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_enet.h
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
// Device-specific region: enet configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_enet_config {
    pub flags: u32,
    pub wq_desc_count: u32,
    pub rq_desc_count: u32,
    pub mtu: u16,
    pub intr_timer_deprecated: u16,
    pub intr_timer_type: u8,
    pub intr_mode: u8,
    pub devname: [c_char; 16],
    pub intr_timer_usec: u32,
    pub loop_tag: u16,
    pub vf_rq_count: u16,
    pub num_arfs: u16,
    pub reserved1: [u8; 32],
    pub mq_subvnic_count: u16,
    pub reserved2: [u8; 32],
    pub size: u32 max_rq_ring; // MAX RQ ring,
    pub size: u32 max_wq_ring; // MAX WQ ring,
    pub size: u32 max_cq_ring; // MAX CQ ring,
    pub LKey: u32 rdma_rsvd_lkey; // Reserved (privileged),
}

pub const VENETF_TSO: c_uint = 0x1	/* TSO enabled */;
pub const VENETF_LRO: c_uint = 0x2	/* LRO enabled */;
pub const VENETF_RXCSUM: c_uint = 0x4	/* RX csum enabled */;
pub const VENETF_TXCSUM: c_uint = 0x8	/* TX csum enabled */;
pub const VENETF_RSS: c_uint = 0x10	/* RSS enabled */;
pub const VENETF_RSSHASH_IPV4: c_uint = 0x20	/* Hash on IPv4 fields */;
pub const VENETF_RSSHASH_TCPIPV4: c_uint = 0x40	/* Hash on TCP + IPv4 fields */;
pub const VENETF_RSSHASH_IPV6: c_uint = 0x80	/* Hash on IPv6 fields */;
pub const VENETF_RSSHASH_TCPIPV6: c_uint = 0x100	/* Hash on TCP + IPv6 fields */;
pub const VENETF_RSSHASH_IPV6_EX: c_uint = 0x200	/* Hash on IPv6 extended fields */;
pub const VENETF_RSSHASH_TCPIPV6_EX: c_uint = 0x400	/* Hash on TCP + IPv6 ext. fields */;
pub const VENETF_LOOP: c_uint = 0x800	/* Loopback enabled */;
pub const VENETF_VXLAN: c_uint = 0x10000	/* VxLAN offload */;

