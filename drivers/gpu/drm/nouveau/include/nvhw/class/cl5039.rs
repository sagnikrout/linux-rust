//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/cl5039.h
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

// Macro flag: #define _cl_nv50_memory_to_memory_format_h_
pub const NV5039_SET_OBJECT: c_uint = 0x0000;

pub const NV5039_NO_OPERATION: c_uint = 0x0100;

pub const NV5039_SET_CONTEXT_DMA_NOTIFY: c_uint = 0x0180;

pub const NV5039_SET_CONTEXT_DMA_BUFFER_IN: c_uint = 0x0184;

pub const NV5039_SET_CONTEXT_DMA_BUFFER_OUT: c_uint = 0x0188;

pub const NV5039_SET_SRC_MEMORY_LAYOUT: c_uint = 0x0200;

pub const NV5039_SET_SRC_MEMORY_LAYOUT_V_BLOCKLINEAR: c_uint = 0x00000000;
pub const NV5039_SET_SRC_MEMORY_LAYOUT_V_PITCH: c_uint = 0x00000001;
pub const NV5039_SET_SRC_BLOCK_SIZE: c_uint = 0x0204;

pub const NV5039_SET_SRC_BLOCK_SIZE_WIDTH_ONE_GOB: c_uint = 0x00000000;

pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_ONE_GOB: c_uint = 0x00000000;
pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_TWO_GOBS: c_uint = 0x00000001;
pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_FOUR_GOBS: c_uint = 0x00000002;
pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_EIGHT_GOBS: c_uint = 0x00000003;
pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_SIXTEEN_GOBS: c_uint = 0x00000004;
pub const NV5039_SET_SRC_BLOCK_SIZE_HEIGHT_THIRTYTWO_GOBS: c_uint = 0x00000005;

pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_ONE_GOB: c_uint = 0x00000000;
pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_TWO_GOBS: c_uint = 0x00000001;
pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_FOUR_GOBS: c_uint = 0x00000002;
pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_EIGHT_GOBS: c_uint = 0x00000003;
pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_SIXTEEN_GOBS: c_uint = 0x00000004;
pub const NV5039_SET_SRC_BLOCK_SIZE_DEPTH_THIRTYTWO_GOBS: c_uint = 0x00000005;
pub const NV5039_SET_SRC_WIDTH: c_uint = 0x0208;

pub const NV5039_SET_SRC_HEIGHT: c_uint = 0x020c;

pub const NV5039_SET_SRC_DEPTH: c_uint = 0x0210;

pub const NV5039_SET_SRC_LAYER: c_uint = 0x0214;

pub const NV5039_SET_SRC_ORIGIN: c_uint = 0x0218;

pub const NV5039_SET_DST_MEMORY_LAYOUT: c_uint = 0x021c;

pub const NV5039_SET_DST_MEMORY_LAYOUT_V_BLOCKLINEAR: c_uint = 0x00000000;
pub const NV5039_SET_DST_MEMORY_LAYOUT_V_PITCH: c_uint = 0x00000001;
pub const NV5039_SET_DST_BLOCK_SIZE: c_uint = 0x0220;

pub const NV5039_SET_DST_BLOCK_SIZE_WIDTH_ONE_GOB: c_uint = 0x00000000;

pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_ONE_GOB: c_uint = 0x00000000;
pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_TWO_GOBS: c_uint = 0x00000001;
pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_FOUR_GOBS: c_uint = 0x00000002;
pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_EIGHT_GOBS: c_uint = 0x00000003;
pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_SIXTEEN_GOBS: c_uint = 0x00000004;
pub const NV5039_SET_DST_BLOCK_SIZE_HEIGHT_THIRTYTWO_GOBS: c_uint = 0x00000005;

pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_ONE_GOB: c_uint = 0x00000000;
pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_TWO_GOBS: c_uint = 0x00000001;
pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_FOUR_GOBS: c_uint = 0x00000002;
pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_EIGHT_GOBS: c_uint = 0x00000003;
pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_SIXTEEN_GOBS: c_uint = 0x00000004;
pub const NV5039_SET_DST_BLOCK_SIZE_DEPTH_THIRTYTWO_GOBS: c_uint = 0x00000005;
pub const NV5039_SET_DST_WIDTH: c_uint = 0x0224;

pub const NV5039_SET_DST_HEIGHT: c_uint = 0x0228;

pub const NV5039_SET_DST_DEPTH: c_uint = 0x022c;

pub const NV5039_SET_DST_LAYER: c_uint = 0x0230;

pub const NV5039_SET_DST_ORIGIN: c_uint = 0x0234;

pub const NV5039_OFFSET_IN_UPPER: c_uint = 0x0238;

pub const NV5039_OFFSET_OUT_UPPER: c_uint = 0x023c;

pub const NV5039_OFFSET_IN: c_uint = 0x030c;

pub const NV5039_OFFSET_OUT: c_uint = 0x0310;

pub const NV5039_PITCH_IN: c_uint = 0x0314;

pub const NV5039_PITCH_OUT: c_uint = 0x0318;

pub const NV5039_LINE_LENGTH_IN: c_uint = 0x031c;

pub const NV5039_LINE_COUNT: c_uint = 0x0320;

pub const NV5039_FORMAT: c_uint = 0x0324;

pub const NV5039_FORMAT_IN_ONE: c_uint = 0x00000001;

pub const NV5039_FORMAT_OUT_ONE: c_uint = 0x00000001;
pub const NV5039_BUFFER_NOTIFY: c_uint = 0x0328;

pub const NV5039_BUFFER_NOTIFY_TYPE_WRITE_ONLY: c_uint = 0x00000000;
pub const NV5039_BUFFER_NOTIFY_TYPE_WRITE_THEN_AWAKEN: c_uint = 0x00000001;
