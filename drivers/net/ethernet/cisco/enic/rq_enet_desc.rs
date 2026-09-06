//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/rq_enet_desc.h
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
// Ethernet receive queue descriptor: 16B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_enet_desc {
    pub address: __le64,
    pub length_type: __le16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rq_enet_type_types {
    RQ_ENET_TYPE_ONLY_SOP = 0,
    RQ_ENET_TYPE_NOT_SOP = 1,
    RQ_ENET_TYPE_RESV2 = 2,
    RQ_ENET_TYPE_RESV3 = 3,
}

pub const RQ_ENET_ADDR_BITS: c_int = 64;
pub const RQ_ENET_LEN_BITS: c_int = 14;

pub const RQ_ENET_TYPE_BITS: c_int = 2;

// address = le64_to_cpu(desc->address);
// length = le16_to_cpu(desc->length_type) & RQ_ENET_LEN_MASK;
// type = (u8)((le16_to_cpu(desc->length_type) >> RQ_ENET_LEN_BITS) &
