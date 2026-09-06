//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/wq_enet_desc.h
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
// Ethernet work queue descriptor: 16B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wq_enet_desc {
    pub address: __le64,
    pub length: __le16,
    pub mss_loopback: __le16,
    pub header_length_flags: __le16,
    pub vlan_tag: __le16,
}

pub const WQ_ENET_ADDR_BITS: c_int = 64;
pub const WQ_ENET_LEN_BITS: c_int = 14;

pub const WQ_ENET_MSS_BITS: c_int = 14;

pub const WQ_ENET_MSS_SHIFT: c_int = 2;
pub const WQ_ENET_LOOPBACK_SHIFT: c_int = 1;
pub const WQ_ENET_HDRLEN_BITS: c_int = 10;

pub const WQ_ENET_FLAGS_OM_BITS: c_int = 2;

pub const WQ_ENET_FLAGS_EOP_SHIFT: c_int = 12;
pub const WQ_ENET_FLAGS_CQ_ENTRY_SHIFT: c_int = 13;
pub const WQ_ENET_FLAGS_FCOE_ENCAP_SHIFT: c_int = 14;
pub const WQ_ENET_FLAGS_VLAN_TAG_INSERT_SHIFT: c_int = 15;
pub const WQ_ENET_OFFLOAD_MODE_CSUM: c_int = 0;
pub const WQ_ENET_OFFLOAD_MODE_RESERVED: c_int = 1;
pub const WQ_ENET_OFFLOAD_MODE_CSUM_L4: c_int = 2;
pub const WQ_ENET_OFFLOAD_MODE_TSO: c_int = 3;
// address = le64_to_cpu(desc->address);
// length = le16_to_cpu(desc->length) & WQ_ENET_LEN_MASK;
// mss = (le16_to_cpu(desc->mss_loopback) >> WQ_ENET_MSS_SHIFT) &
// loopback = (u8)((le16_to_cpu(desc->mss_loopback) >>
// header_length = le16_to_cpu(desc->header_length_flags) &
// offload_mode = (u8)((le16_to_cpu(desc->header_length_flags) >>
// eop = (u8)((le16_to_cpu(desc->header_length_flags) >>
// cq_entry = (u8)((le16_to_cpu(desc->header_length_flags) >>
// fcoe_encap = (u8)((le16_to_cpu(desc->header_length_flags) >>
// vlan_tag_insert = (u8)((le16_to_cpu(desc->header_length_flags) >>
// vlan_tag = le16_to_cpu(desc->vlan_tag);
