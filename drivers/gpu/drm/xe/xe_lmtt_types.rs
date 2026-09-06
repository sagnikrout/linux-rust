//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_lmtt_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// struct xe_lmtt - Local Memory Translation Table Manager
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_lmtt {
// @pd: root LMTT Directory
    pub pd: *mut xe_lmtt_pt,
// @ops: LMTT functions
    pub ops: *const xe_lmtt_ops,
}

//
// struct xe_lmtt_pt - Local Memory Translation Table Page Table
//
// Represents single level of the LMTT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_lmtt_pt {
// @level: page table level, 0 is leaf
    pub level: c_uint,
// @bo: buffer object with actual LMTT PTE values
    pub bo: *mut xe_bo,
// @entries: leaf page tables, exist only for root/non-leaf
    pub entries: [*mut xe_lmtt_pt; ],
}

//
// struct xe_lmtt_ops - Local Memory Translation Table Operations
//
// Provides abstraction of the LMTT variants.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_lmtt_ops {
// private:
    pub (*lmtt_root_pd_level)(void): *mut c_uint,
    pub level): *mut *mut unsigned int (lmtt_pte_num)(unsigned int,
    pub level): *mut *mut unsigned int (lmtt_pte_size)(unsigned int,
    pub level): *mut *mut unsigned int (lmtt_pte_shift)(unsigned int,
    pub level): *mut *mut unsigned int (lmtt_pte_index)(u64 addr, unsigned int,
    pub level): *mut *mut u64 (lmtt_pte_encode)(unsigned long offset, unsigned int,
}
