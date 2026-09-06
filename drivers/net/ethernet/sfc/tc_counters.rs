//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/tc_counters.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2022 Advanced Micro Devices, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_tc_counter_type {
    EFX_TC_COUNTER_TYPE_AR = MAE_COUNTER_TYPE_AR,
    EFX_TC_COUNTER_TYPE_CT = MAE_COUNTER_TYPE_CT,
    EFX_TC_COUNTER_TYPE_OR = MAE_COUNTER_TYPE_OR,
    EFX_TC_COUNTER_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_counter {
    pub /: *mut *mut u32 fw_id; / index in firmware counter table,
    pub type: efx_tc_counter_type,
    pub /: *mut *mut rhash_head linkage; / efx->tc->counter_ht,
    pub /: *mut *mut spinlock_t lock; / Serialises updates to counter values,
    pub /: *mut *mut u32 gen; / Generation count at which this counter is current,
    pub bytes: u64 packets,,
    pub /: *mut *mut u64 old_packets, old_bytes; / Values last time passed to userspace,
// jiffies of the last time we saw packets increase
    pub touched: c_ulong,
    pub /: *mut *mut work_work; / For notifying encap actions,
// owners of corresponding count actions
    pub users: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_counter_index {
    pub cookie: c_ulong,
    pub /: *mut *mut rhash_head linkage; / efx->tc->counter_id_ht,
    pub ref: refcount_t,
    pub cnt: *mut efx_tc_counter,
}

// create/uncreate/teardown hashtables
extern "C" {
    pub fn efx_tc_init_counters(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_tc_destroy_counters(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_tc_fini_counters(efx: *mut efx_nic);
}
