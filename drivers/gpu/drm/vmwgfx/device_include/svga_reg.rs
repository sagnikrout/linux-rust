//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/svga_reg.h
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
// Copyright 1998-2021 VMware, Inc.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// svga_reg.h --
//
// Virtual hardware definitions for the VMware SVGA II device.
//

pub type SVGAMobId = uint32;
pub const SVGA_MAX_WIDTH: c_int = 2560;
pub const SVGA_MAX_HEIGHT: c_int = 1600;
pub const SVGA_MAX_BITS_PER_PIXEL: c_int = 32;
pub const SVGA_MAX_DEPTH: c_int = 24;
pub const SVGA_MAX_DISPLAYS: c_int = 10;
pub const SVGA_MAX_SCREEN_SIZE: c_int = 8192;

pub const SVGA_CURSOR_ON_HIDE: c_uint = 0x0;
pub const SVGA_CURSOR_ON_SHOW: c_uint = 0x1;
pub const SVGA_CURSOR_ON_REMOVE_FROM_FB: c_uint = 0x2;
pub const SVGA_CURSOR_ON_RESTORE_TO_FB: c_uint = 0x3;
pub const SVGA_FB_MAX_TRACEABLE_SIZE: c_uint = 0x1000000;
pub const SVGA_MAX_PSEUDOCOLOR_DEPTH: c_int = 8;

pub const SVGA_MAGIC: c_uint = 0x900000UL;

pub const SVGA_VERSION_3: c_int = 3;

pub const SVGA_VERSION_2: c_int = 2;

pub const SVGA_VERSION_1: c_int = 1;

pub const SVGA_VERSION_0: c_int = 0;

pub const SVGA_ID_INVALID: c_uint = 0xFFFFFFFF;
pub const SVGA_INDEX_PORT: c_uint = 0x0;
pub const SVGA_VALUE_PORT: c_uint = 0x1;
pub const SVGA_BIOS_PORT: c_uint = 0x2;
pub const SVGA_IRQSTATUS_PORT: c_uint = 0x8;

pub const SVGA_MAX_CURSOR_CMD_DIMENSION: c_int = 1024;

pub const SVGA_CB_MAX_QUEUED_PER_CONTEXT: c_int = 32;

pub const SVGA_CB_CONTEXT_MASK: c_uint = 0x3f;

pub const SVGA_CAP_NONE: c_uint = 0x00000000;
pub const SVGA_CAP_RECT_COPY: c_uint = 0x00000002;
pub const SVGA_CAP_CURSOR: c_uint = 0x00000020;
pub const SVGA_CAP_CURSOR_BYPASS: c_uint = 0x00000040;
pub const SVGA_CAP_CURSOR_BYPASS_2: c_uint = 0x00000080;
pub const SVGA_CAP_8BIT_EMULATION: c_uint = 0x00000100;
pub const SVGA_CAP_ALPHA_CURSOR: c_uint = 0x00000200;
pub const SVGA_CAP_3D: c_uint = 0x00004000;
pub const SVGA_CAP_EXTENDED_FIFO: c_uint = 0x00008000;
pub const SVGA_CAP_MULTIMON: c_uint = 0x00010000;
pub const SVGA_CAP_PITCHLOCK: c_uint = 0x00020000;
pub const SVGA_CAP_IRQMASK: c_uint = 0x00040000;
pub const SVGA_CAP_DISPLAY_TOPOLOGY: c_uint = 0x00080000;
pub const SVGA_CAP_GMR: c_uint = 0x00100000;
pub const SVGA_CAP_TRACES: c_uint = 0x00200000;
pub const SVGA_CAP_GMR2: c_uint = 0x00400000;
pub const SVGA_CAP_SCREEN_OBJECT_2: c_uint = 0x00800000;
pub const SVGA_CAP_COMMAND_BUFFERS: c_uint = 0x01000000;
pub const SVGA_CAP_DEAD1: c_uint = 0x02000000;
pub const SVGA_CAP_CMD_BUFFERS_2: c_uint = 0x04000000;
pub const SVGA_CAP_GBOBJECTS: c_uint = 0x08000000;
pub const SVGA_CAP_DX: c_uint = 0x10000000;
pub const SVGA_CAP_HP_CMD_QUEUE: c_uint = 0x20000000;
pub const SVGA_CAP_NO_BB_RESTRICTION: c_uint = 0x40000000;
pub const SVGA_CAP_CAP2_REGISTER: c_uint = 0x80000000;
pub const SVGA_CAP2_NONE: c_uint = 0x00000000;
pub const SVGA_CAP2_GROW_OTABLE: c_uint = 0x00000001;
pub const SVGA_CAP2_INTRA_SURFACE_COPY: c_uint = 0x00000002;
pub const SVGA_CAP2_DX2: c_uint = 0x00000004;
pub const SVGA_CAP2_GB_MEMSIZE_2: c_uint = 0x00000008;
pub const SVGA_CAP2_SCREENDMA_REG: c_uint = 0x00000010;
pub const SVGA_CAP2_OTABLE_PTDEPTH_2: c_uint = 0x00000020;
pub const SVGA_CAP2_NON_MS_TO_MS_STRETCHBLT: c_uint = 0x00000040;
pub const SVGA_CAP2_CURSOR_MOB: c_uint = 0x00000080;
pub const SVGA_CAP2_MSHINT: c_uint = 0x00000100;
pub const SVGA_CAP2_CB_MAX_SIZE_4MB: c_uint = 0x00000200;
pub const SVGA_CAP2_DX3: c_uint = 0x00000400;
pub const SVGA_CAP2_FRAME_TYPE: c_uint = 0x00000800;
pub const SVGA_CAP2_COTABLE_COPY: c_uint = 0x00001000;
pub const SVGA_CAP2_TRACE_FULL_FB: c_uint = 0x00002000;
pub const SVGA_CAP2_EXTRA_REGS: c_uint = 0x00004000;
pub const SVGA_CAP2_LO_STAGING: c_uint = 0x00008000;
pub const SVGA_CAP2_VIDEO_BLT: c_uint = 0x00010000;
pub const SVGA_CAP2_RESERVED: c_uint = 0x80000000;

pub const SVGA3D_FIFO_CAPS_RECORD_DEVCAPS: c_uint = 0x100;
pub type SVGA3dFifoCapsRecordType = uint32;

pub const SVGA_FIFO_CAP_NONE: c_int = 0;

pub const SVGA_FIFO_FLAG_NONE: c_int = 0;

pub const SVGA_FIFO_RESERVED_UNKNOWN: c_uint = 0xffffffff;
pub const SVGA_SCREENDMA_REG_UNDEFINED: c_int = 0;
pub const SVGA_SCREENDMA_REG_NOT_PRESENT: c_int = 1;
pub const SVGA_SCREENDMA_REG_PRESENT: c_int = 2;
pub const SVGA_SCREENDMA_REG_MAX: c_int = 3;
pub const SVGA_NUM_OVERLAY_UNITS: c_int = 32;
pub const SVGA_VIDEO_FLAG_COLORKEY: c_uint = 0x0001;

pub const SVGA_CMD_MAX_ARGS: c_int = 64;

