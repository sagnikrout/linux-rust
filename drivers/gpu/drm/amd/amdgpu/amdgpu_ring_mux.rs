//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ring_mux.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

//
// struct amdgpu_mux_entry - the entry recording software rings copying information.
// @ring: the pointer to the software ring.
// @start_ptr_in_hw_ring: last start location copied to in the hardware ring.
// @end_ptr_in_hw_ring: last end location copied to in the hardware ring.
// @sw_cptr: the position of the copy pointer in the sw ring.
// @sw_rptr: the read pointer in software ring.
// @sw_wptr: the write pointer in software ring.
// @list: list head for amdgpu_mux_chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mux_entry {
    pub ring: *mut amdgpu_ring,
    pub start_ptr_in_hw_ring: u64,
    pub end_ptr_in_hw_ring: u64,
    pub sw_cptr: u64,
    pub sw_rptr: u64,
    pub sw_wptr: u64,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ring_mux_offset_type {
    AMDGPU_MUX_OFFSET_TYPE_CONTROL,
    AMDGPU_MUX_OFFSET_TYPE_DE,
    AMDGPU_MUX_OFFSET_TYPE_CE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_complete_status {
// IB not started/reset value, default value.
    IB_COMPLETION_STATUS_DEFAULT = 0,
// IB preempted, started but not completed.
    IB_COMPLETION_STATUS_PREEMPTED = 1,
// IB completed.
    IB_COMPLETION_STATUS_COMPLETED = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ring_mux {
    pub real_ring: *mut amdgpu_ring,
    pub ring_entry: *mut amdgpu_mux_entry,
    pub num_ring_entries: c_uint,
    pub ring_entry_size: c_uint,
// the lock for copy data from different software rings
    pub lock: spinlock_t,
    pub s_resubmit: bool,
    pub seqno_to_resubmit: u32,
    pub wptr_resubmit: u64,
    pub resubmit_timer: timer_list,
    pub pending_trailing_fence_signaled: bool,
}

//
// struct amdgpu_mux_chunk - save the location of indirect buffer's package on softare rings.
// @entry: the list entry.
// @sync_seq: the fence seqno related with the saved IB.
// @start:- start location on the software ring.
// @end:- end location on the software ring.
// @control_offset:- the PRE_RESUME bit position used for resubmission.
// @de_offset:- the anchor in write_data for de meta of resubmission.
// @ce_offset:- the anchor in write_data for ce meta of resubmission.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mux_chunk {
    pub entry: list_head,
    pub sync_seq: u32,
    pub start: u64,
    pub end: u64,
    pub cntl_offset: u64,
    pub de_offset: u64,
    pub ce_offset: u64,
}

extern "C" {
    pub fn amdgpu_ring_mux_fini(mux: *mut amdgpu_ring_mux);
}
extern "C" {
    pub fn amdgpu_ring_mux_add_sw_ring(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_ring_mux_set_wptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring, wptr: u64);
}
extern "C" {
    pub fn amdgpu_ring_mux_get_wptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> u64;
}
extern "C" {
    pub fn amdgpu_ring_mux_get_rptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> u64;
}
extern "C" {
    pub fn amdgpu_ring_mux_start_ib(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_ring_mux_end_ib(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_mcbp_handle_trailing_fence_irq(mux: *mut amdgpu_ring_mux) -> bool;
}
extern "C" {
    pub fn amdgpu_sw_ring_get_rptr_gfx(ring: *mut amdgpu_ring) -> u64;
}
extern "C" {
    pub fn amdgpu_sw_ring_get_wptr_gfx(ring: *mut amdgpu_ring) -> u64;
}
extern "C" {
    pub fn amdgpu_sw_ring_set_wptr_gfx(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_sw_ring_insert_nop(ring: *mut amdgpu_ring, count: u32);
}
extern "C" {
    pub fn amdgpu_sw_ring_ib_begin(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_sw_ring_ib_end(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_sw_ring_ib_mark_offset(ring: *mut amdgpu_ring, type: amdgpu_ring_mux_offset_type);
}
extern "C" {
    pub fn amdgpu_sw_ring_priority(idx: c_int) -> c_uint;
}
