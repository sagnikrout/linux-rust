//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_res_cursor.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright 2020 Advanced Micro Devices, Inc.
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
// struct xe_res_cursor - state for walking over dma mapping, vram_mgr,
// stolen_mgr, and gtt_mgr allocations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_res_cursor {
// @start: Start of cursor
    pub start: u64,
// @size: Size of the current segment.
    pub size: u64,
// @remaining: Remaining bytes in cursor
    pub remaining: u64,
// @node: Opaque point current node cursor
    pub node: *mut c_void,
// @mem_type: Memory type
    pub mem_type: u32,
// @sgl: Scatterlist for cursor
    pub sgl: *mut scatterlist,
// @dma_addr: Current element in a struct drm_pagemap_addr array
    pub dma_addr: *const drm_pagemap_addr,
// @mm: Buddy allocator for VRAM cursor
    pub mm: *mut gpu_buddy,
//
// @dma_start: DMA start address for the current segment.
// This may be different to @dma_addr.addr since elements in
// the array may be coalesced to a single segment.
//
    pub dma_start: u64,
// @dma_seg_size: Size of the current DMA segment.
    pub dma_seg_size: u64,
}

//
// xe_res_first - initialize a xe_res_cursor
//
// @res: TTM resource object to walk
// @start: Start of the range
// @size: Size of the range
// @cur: cursor object to initialize
//
// Start walking over the range of allocations between @start and @size.
//
// res->start is in pages (ttm_range_manager).
//
// __xe_res_dma_next() - Advance the cursor when end-of-segment is reached
// @cur: The cursor
//
// Coalesce array_elements
//
// xe_res_first_sg - initialize a xe_res_cursor with a scatter gather table
//
// @sg: scatter gather table to walk
// @start: Start of the range
// @size: Size of the range
// @cur: cursor object to initialize
//
// Start walking over the range of allocations between @start and @size.
//
// xe_res_first_dma - initialize a xe_res_cursor with dma_addr array
//
// @dma_addr: struct drm_pagemap_addr array to walk
// @start: Start of the range
// @size: Size of the range
// @cur: cursor object to initialize
//
// Start walking over the range of allocations between @start and @size.
//
// xe_res_next - advance the cursor
//
// @cur: the cursor to advance
// @size: number of bytes to move forward
//
// Move the cursor @size bytes forwrad, walking to the next node if necessary.
//
// Just advance within the contiguous region.
//
// xe_res_dma - return dma address of cursor at current position
//
// @cur: the cursor to return the dma address from
//
// xe_res_is_vram() - Whether the cursor current dma address points to
// same-device VRAM
// @cur: The cursor.
//
// Return: true iff the address returned by xe_res_dma() points to internal vram.
//
