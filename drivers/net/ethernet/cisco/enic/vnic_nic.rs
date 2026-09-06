//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_nic.h
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
pub const NIC_CFG_RSS_DEFAULT_CPU_MASK_FIELD: c_uint = 0xffUL;
pub const NIC_CFG_RSS_DEFAULT_CPU_SHIFT: c_int = 0;

pub const NIC_CFG_RSS_HASH_TYPE_MASK_FIELD: c_uint = 0xffUL;
pub const NIC_CFG_RSS_HASH_TYPE_SHIFT: c_int = 8;

pub const NIC_CFG_RSS_HASH_BITS_SHIFT: c_int = 16;

pub const NIC_CFG_RSS_BASE_CPU_SHIFT: c_int = 19;

pub const NIC_CFG_RSS_ENABLE_SHIFT: c_int = 22;

pub const NIC_CFG_TSO_IPID_SPLIT_EN_SHIFT: c_int = 23;

pub const NIC_CFG_IG_VLAN_STRIP_EN_SHIFT: c_int = 24;

// nic_cfg = (rss_default_cpu & NIC_CFG_RSS_DEFAULT_CPU_MASK_FIELD) |
