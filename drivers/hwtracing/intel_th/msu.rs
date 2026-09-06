//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/intel_th/msu.h
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
//
// Intel(R) Trace Hub Memory Storage Unit (MSU) data structures
//
// Copyright (C) 2014-2015 Intel Corporation.
//
// MSUSTS bits

// MSCnCTL bits

// MINTCTL bits

// MSCnSTS bits

//
// Multiblock/multiwindow block descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc_block_desc {
    pub sw_tag: u32,
    pub block_sz: u32,
    pub next_blk: u32,
    pub next_win: u32,
    pub res0: [u32; 4],
    pub hw_tag: u32,
    pub valid_dw: u32,
    pub ts_low: u32,
    pub ts_high: u32,
    pub res1: [u32; 4],
    pub __packed: },

// MSC multiblock sw tag bits

// MSC multiblock hw tag bits

    pub 0: return,
    pub MSC_BDESC: *mut *mut return bdesc->valid_dw  4 -,
    pub 4: *mut *mut return bdesc->valid_dw,
    pub MSC_BDESC: *mut *mut return bdesc->block_sz  64 -,
    pub true: return,
    pub false: return,
    pub true: return,
    pub false: return,
// waiting for Pipeline Empty bit(s) to assert for MSC
pub const MSC_PLE_WAITLOOP_DEPTH: c_int = 10000;
