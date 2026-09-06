//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/idxd/perfmon.h
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
// Copyright(c) 2020 Intel Corporation. All rights rsvd.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_perf_events {
    DSA_PERF_EVENT_WQ = 0,
    DSA_PERF_EVENT_ENGINE,
    DSA_PERF_EVENT_ADDR_TRANS,
    DSA_PERF_EVENT_OP,
    DSA_PERF_EVENT_COMPL,
    DSA_PERF_EVENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum filter_enc {
    FLT_WQ = 0,
    FLT_TC,
    FLT_PG_SZ,
    FLT_XFER_SZ,
    FLT_ENG,
    FLT_MAX,
}

pub const CONFIG_RESET: c_uint = 0x0000000000000001;
pub const CNTR_RESET: c_uint = 0x0000000000000002;
pub const CNTR_ENABLE: c_uint = 0x0000000000000001;
pub const INTR_OVFL: c_uint = 0x0000000000000002;
pub const COUNTER_FREEZE: c_uint = 0x00000000FFFFFFFF;
pub const COUNTER_UNFREEZE: c_uint = 0x0000000000000000;
pub const OVERFLOW_SIZE: c_int = 32;

pub const CNTRCFG_CATEGORY_SHIFT: c_int = 8;
pub const CNTRCFG_EVENT_SHIFT: c_int = 32;

