//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/fsl/bman.h
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


// Copyright 2008 - 2016 Freescale Semiconductor, Inc.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// wrapper for 48-bit buffers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_buffer {
    pub /: *mut *mut __be16 bpid; / hi 8-bits reserved,
    pub /: *mut *mut __be16 hi; / High 16-bits of 48-bit address,
    pub /: *mut *mut __be32 lo; / Low 32-bits of 48-bit address,
}

//
// Restore the 48 bit address previously stored in BMan
// hardware pools as a dma_addr_t
//
// Managed portal, high-level i/face
// Portal and Buffer Pools

//
// bman_new_pool - Allocates a Buffer Pool object
//
// Creates a pool object, and returns a reference to it or NULL on error.
//
// bman_free_pool - Deallocates a Buffer Pool object
// @pool: the pool object to release
//
extern "C" {
    pub fn bman_free_pool(pool: *mut bman_pool);
}
//
// bman_get_bpid - Returns a pool object's BPID.
// @pool: the pool object
//
// The returned value is the index of the encapsulated buffer pool,
// in the range of [0, @BM_POOL_MAX-1].
//
extern "C" {
    pub fn bman_get_bpid(pool: *const bman_pool) -> c_int;
}
//
// bman_release - Release buffer(s) to the buffer pool
// @pool: the buffer pool object to release to
// @bufs: an array of buffers to release
// @num: the number of buffers in @bufs (1-8)
//
// Adds the given buffers to RCR entries. If the RCR ring is unresponsive,
// the function will return -ETIMEDOUT. Otherwise, it returns zero.
//
extern "C" {
    pub fn bman_release(pool: *mut bman_pool, bufs: *const bm_buffer, num: u8) -> c_int;
}
//
// bman_acquire - Acquire buffer(s) from a buffer pool
// @pool: the buffer pool object to acquire from
// @bufs: array for storing the acquired buffers
// @num: the number of buffers desired (@bufs is at least this big)
//
// Issues an "Acquire" command via the portal's management command interface.
// The return value will be the number of buffers obtained from the pool, or a
// negative error code if a h/w error or pool starvation was encountered. In
// the latter case, the content of @bufs is undefined.
//
extern "C" {
    pub fn bman_acquire(pool: *mut bman_pool, bufs: *mut bm_buffer, num: u8) -> c_int;
}
//
// bman_is_probed - Check if bman is probed
//
// Returns 1 if the bman driver successfully probed, -1 if the bman driver
// failed to probe or 0 if the bman driver did not probed yet.
//
extern "C" {
    pub fn bman_is_probed() -> c_int;
}
//
// bman_portals_probed - Check if all cpu bound bman portals are probed
//
// Returns 1 if all the required cpu bound bman portals successfully probed,
// -1 if probe errors appeared or 0 if the bman portals did not yet finished
// probing.
//
extern "C" {
    pub fn bman_portals_probed() -> c_int;
}
