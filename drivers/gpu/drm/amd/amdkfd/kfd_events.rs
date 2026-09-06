//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_events.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
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

// Macro flag: #define KFD_EVENTS_H_INCLUDED

//
// IDR supports non-negative integer IDs. Small IDs are used for
// signal events to match their signal slot. Use the upper half of the
// ID space for non-signal events.
//

//
// Written into kfd_signal_slot_t to indicate that the event is not signaled.
// Since the event protocol may need to write the event ID into memory, this
// must not be a valid event ID.
// For the sake of easy memset-ing, this must be a byte pattern.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_event {
    pub event_id: u32,
    pub event_age: u64,
    pub signaled: bool,
    pub auto_reset: bool,
    pub type: c_int,
    pub lock: spinlock_t,
    pub /: *mut *mut wait_queue_head_t wq; / List of event waiters.,
// type specific data
    pub memory_exception_data: kfd_hsa_memory_exception_data,
    pub hw_exception_data: kfd_hsa_hw_exception_data,
}

pub const KFD_EVENT_TIMEOUT_IMMEDIATE: c_int = 0;
pub const KFD_EVENT_TIMEOUT_INFINITE: c_uint = 0xFFFFFFFFu;
// Matching HSA_EVENTTYPE
pub const KFD_EVENT_TYPE_SIGNAL: c_int = 0;
pub const KFD_EVENT_TYPE_HW_EXCEPTION: c_int = 3;
pub const KFD_EVENT_TYPE_DEBUG: c_int = 5;
pub const KFD_EVENT_TYPE_MEMORY: c_int = 8;
