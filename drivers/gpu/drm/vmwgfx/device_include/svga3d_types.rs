//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/svga3d_types.h
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
// Copyright 2012-2021 VMware, Inc.
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
// svga3d_types.h --
//
// SVGA 3d hardware definitions for basic types
//

pub const SVGA3D_RESOURCE_TYPE_MIN: c_int = 1;
pub const SVGA3D_RESOURCE_BUFFER: c_int = 1;
pub const SVGA3D_RESOURCE_TEXTURE1D: c_int = 2;
pub const SVGA3D_RESOURCE_TEXTURE2D: c_int = 3;
pub const SVGA3D_RESOURCE_TEXTURE3D: c_int = 4;
pub const SVGA3D_RESOURCE_TEXTURECUBE: c_int = 5;
pub const SVGA3D_RESOURCE_TYPE_DX10_MAX: c_int = 6;
pub const SVGA3D_RESOURCE_BUFFEREX: c_int = 6;
pub const SVGA3D_RESOURCE_TYPE_MAX: c_int = 7;
pub type SVGA3dResourceType = uint32;
pub type SVGABool8 = uint8;
pub type SVGA3dBool = uint32;
pub type SVGA3dColor = uint32;
pub type SVGA3dSurfaceId = uint32;

pub type SVGA3dSurface1Flags = uint32;
pub type SVGA3dSurface2Flags = uint32;
pub type SVGA3dSurfaceAllFlags = uint64;

pub const SVGA3D_BUFFER_STRUCTURED_STRIDE_MAX: c_int = 2048;

pub const SVGA3D_TM_MASK_LEN: c_int = 4;

pub const SVGA3D_CONSTREG_MAX: c_int = 256;
pub const SVGA3D_CONSTINTREG_MAX: c_int = 16;
pub const SVGA3D_CONSTBOOLREG_MAX: c_int = 16;
pub type SVGA3dQueryTypeUint8 = uint8;

pub const SVGA3D_MAX_QUERY: c_int = 64;

pub const SVGA3D_LOGICOP_INVALID: c_int = 0;
pub const SVGA3D_LOGICOP_COPY: c_int = 1;
pub const SVGA3D_LOGICOP_MIN: c_int = 1;
pub const SVGA3D_LOGICOP_NOT: c_int = 2;
pub const SVGA3D_LOGICOP_AND: c_int = 3;
pub const SVGA3D_LOGICOP_OR: c_int = 4;
pub const SVGA3D_LOGICOP_XOR: c_int = 5;
pub const SVGA3D_LOGICOP_NXOR: c_int = 6;
pub const SVGA3D_LOGICOP_ROP3: c_int = 7;
pub const SVGA3D_LOGICOP_MAX: c_int = 8;
pub type SVGA3dLogicOp = uint16;

pub const SVGA3D_LOGICOP_ROP3_MIN: c_int = 0;
pub const SVGA3D_LOGICOP_ROP3_MAX: c_int = 256;
pub type SVGA3dLogicOpRop3 = uint16;

pub const SVGA3D_MOB_EMPTY_BASE: c_int = 1;
