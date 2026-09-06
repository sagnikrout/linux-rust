//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_queue_common.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_queue_pages {
// Array of DMA-able pages that actually holds the queue entries.
    pub pages: *mut hinic3_dma_addr_align,
// Page size in bytes.
    pub page_size: u32,
// Number of pages, must be power of 2.
    pub num_pages: u16,
    pub elem_size_shift: u8,
    pub elem_per_pg_shift: u8,
}

// Get pointer to queue entry at the specified index. Index does not have to be
// masked to queue depth, only least significant bits will be used. Also
// provides remaining elements in same page (including the first one) in case
// caller needs multiple entries.
//
// remaining_in_page = elem_per_pg - elem_idx;
