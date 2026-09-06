//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_rss.h
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
// aQuantia Corporation Network Driver
// Copyright (C) 2014-2017 aQuantia Corporation. All rights reserved
//
// File aq_rss.h: Receive Side Scaling definitions.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_rss_parameters {
    pub base_cpu_number: u16,
    pub indirection_table_size: u16,
    pub hash_secret_key_size: u16,
    pub sizeof(u32)]: u32 hash_secret_key[AQ_CFG_RSS_HASHKEY_SIZE /,
    pub indirection_table: [u8; AQ_CFG_RSS_INDIRECTION_TABLE_MAX],
}
