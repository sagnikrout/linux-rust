//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/log_ring.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const MAX_RECORD_PER_BATCH: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_log_event {
    RAS_LOG_EVENT_NONE,
    RAS_LOG_EVENT_UE,
    RAS_LOG_EVENT_DE,
    RAS_LOG_EVENT_CE,
    RAS_LOG_EVENT_POISON_CREATION,
    RAS_LOG_EVENT_POISON_CONSUMPTION,
    RAS_LOG_EVENT_RMA,
    RAS_LOG_EVENT_COUNT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_aca_reg {
    pub regs: [u64; ACA_REG_MAX_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_log_info {
    pub seqno: u64,
    pub timestamp: u64,
    pub event: ras_log_event,
    pub aca_reg: ras_aca_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_log_batch_tag {
    pub batch_id: u64,
    pub timestamp: u64,
    pub sub_seqno: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_log_ring {
    pub ras_log_mempool: *mut c_void,
    pub ras_log_root: radix_tree_root,
    pub spin_lock: spinlock_t,
    pub mono_upward_batch_id: u64,
    pub last_del_batch_id: u64,
    pub logged_ecc_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_log_batch_overview {
    pub first_batch_id: u64,
    pub last_batch_id: u64,
    pub logged_batch_count: u32,
}

extern "C" {
    pub fn ras_log_ring_sw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_log_ring_sw_fini(ras_core: *mut ras_core_context) -> c_int;
}
