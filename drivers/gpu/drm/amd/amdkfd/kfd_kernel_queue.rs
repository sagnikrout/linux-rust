//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_kernel_queue.h
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

//
// kq_acquire_packet_buffer: Returns a pointer to the location in the kernel
// queue ring buffer where the calling function can write its packet. It is
// Guaranteed that there is enough space for that packet. It also updates the
// pending write pointer to that location so subsequent calls to
// acquire_packet_buffer will get a correct write pointer
//
// kq_submit_packet: Update the write pointer and doorbell of a kernel queue.
//
// kq_rollback_packet: This routine is called if we failed to build an acquired
// packet for some reason. It just overwrites the pending wptr with the current
// one
//
extern "C" {
    pub fn kq_submit_packet(kq: *mut kernel_queue) -> c_int;
}
extern "C" {
    pub fn kq_rollback_packet(kq: *mut kernel_queue);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_queue {
// data
    pub dev: *mut kfd_node,
    pub mqd_mgr: *mut mqd_manager,
    pub queue: *mut queue,
    pub pending_wptr64: u64,
    pub pending_wptr: u32,
    pub nop_packet: c_uint,
    pub rptr_mem: *mut kfd_mem_obj,
    pub rptr_kernel: *mut u32,
    pub rptr_gpu_addr: u64,
    pub wptr_mem: *mut kfd_mem_obj,
    pub wptr64_kernel: *mut u64,
    pub wptr_kernel: *mut u32,
}
