//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/inc/dmub_trace_buffer.h
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


//
// Copyright 2019 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

pub const LOAD_DMCU_FW: c_int = 1;
pub const LOAD_PHY_FW: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmucb_trace_code {
    DMCUB__UNKNOWN,
    DMCUB__MAIN_BEGIN,
    DMCUB__PHY_INIT_BEGIN,
    DMCUB__PHY_FW_SRAM_LOAD_BEGIN,
    DMCUB__PHY_FW_SRAM_LOAD_END,
    DMCUB__PHY_INIT_POLL_DONE,
    DMCUB__PHY_INIT_END,
    DMCUB__DMCU_ERAM_LOAD_BEGIN,
    DMCUB__DMCU_ERAM_LOAD_END,
    DMCUB__DMCU_ISR_LOAD_BEGIN,
    DMCUB__DMCU_ISR_LOAD_END,
    DMCUB__MAIN_IDLE,
    DMCUB__PERF_TRACE,
    DMCUB__PG_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcub_trace_buf_entry {
    pub trace_code: dmucb_trace_code,
    pub tick_count: u32,
    pub param0: u32,
    pub param1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcub_trace_buf {
    pub entry_count: u32,
    pub clk_freq: u32,
    pub entries: [dmcub_trace_buf_entry; PERF_TRACE_MAX_ENTRY],
}
