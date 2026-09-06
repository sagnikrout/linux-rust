//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/sja1105/sja1105_dynamic_config.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
//

// Special index that can be used for sja1105_dynamic_config_read

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_dynamic_table_ops {
// This returns size_t just to keep same prototype as the
// static config ops, of which we are reusing some functions.
//
    pub op): *mut *mut *mut *mut size_t (entry_packing)(void buf, void entry_ptr, enum packing_op,
    pub op): packing_op,
    pub max_entry_count: usize,
    pub packed_size: usize,
    pub addr: u64,
    pub access: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_mgmt_entry {
    pub tsreg: u64,
    pub takets: u64,
    pub macaddr: u64,
    pub destports: u64,
    pub enfport: u64,
    pub index: u64,
}
