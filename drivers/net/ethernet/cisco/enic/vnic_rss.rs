//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_rss.h
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
// RSS key array
pub const ENIC_RSS_BYTES_PER_KEY: c_int = 10;
pub const ENIC_RSS_KEYS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub union vnic_rss_key {
    pub b: [u8; ENIC_RSS_BYTES_PER_KEY],
    pub b_pad: [u8; 6],
    pub key: [}; ENIC_RSS_KEYS],
    pub raw: [u64; 8],
}

// RSS cpu array
#[repr(C)]
#[derive(Copy, Clone)]
pub union vnic_rss_cpu {
    pub b: [u8; 4],
    pub b_pad: [u8; 4],
    pub cpu: [}; 32],
    pub raw: [u64; 32],
}
