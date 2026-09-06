//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/cl9039.h
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
// Copyright (c) 2003-2004, NVIDIA CORPORATION. All rights reserved.
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
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define _cl_fermi_memory_to_memory_format_a_h_
pub const NV9039_SET_OBJECT: c_uint = 0x0000;

pub const NV9039_OFFSET_OUT_UPPER: c_uint = 0x0238;

pub const NV9039_OFFSET_OUT: c_uint = 0x023c;

pub const NV9039_LAUNCH_DMA: c_uint = 0x0300;

pub const NV9039_LAUNCH_DMA_SRC_INLINE_FALSE: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_SRC_INLINE_TRUE: c_uint = 0x00000001;

pub const NV9039_LAUNCH_DMA_SRC_MEMORY_LAYOUT_BLOCKLINEAR: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_SRC_MEMORY_LAYOUT_PITCH: c_uint = 0x00000001;

pub const NV9039_LAUNCH_DMA_DST_MEMORY_LAYOUT_BLOCKLINEAR: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_DST_MEMORY_LAYOUT_PITCH: c_uint = 0x00000001;

pub const NV9039_LAUNCH_DMA_COMPLETION_TYPE_FLUSH_DISABLE: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_COMPLETION_TYPE_FLUSH_ONLY: c_uint = 0x00000001;
pub const NV9039_LAUNCH_DMA_COMPLETION_TYPE_RELEASE_SEMAPHORE: c_uint = 0x00000002;

pub const NV9039_LAUNCH_DMA_INTERRUPT_TYPE_NONE: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_INTERRUPT_TYPE_INTERRUPT: c_uint = 0x00000001;

pub const NV9039_LAUNCH_DMA_SEMAPHORE_STRUCT_SIZE_FOUR_WORDS: c_uint = 0x00000000;
pub const NV9039_LAUNCH_DMA_SEMAPHORE_STRUCT_SIZE_ONE_WORD: c_uint = 0x00000001;
pub const NV9039_OFFSET_IN_UPPER: c_uint = 0x030c;

pub const NV9039_OFFSET_IN: c_uint = 0x0310;

pub const NV9039_PITCH_IN: c_uint = 0x0314;

pub const NV9039_PITCH_OUT: c_uint = 0x0318;

pub const NV9039_LINE_LENGTH_IN: c_uint = 0x031c;

pub const NV9039_LINE_COUNT: c_uint = 0x0320;

