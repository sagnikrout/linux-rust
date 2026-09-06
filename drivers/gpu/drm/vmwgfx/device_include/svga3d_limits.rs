//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/svga3d_limits.h
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
// svga3d_limits.h --
//
// SVGA 3d hardware limits
//
pub const SVGA3D_HB_MAX_CONTEXT_IDS: c_int = 256;

pub const SVGA3D_DX_MAX_RENDER_TARGETS: c_int = 8;
pub const SVGA3D_DX11_MAX_UAVIEWS: c_int = 8;
pub const SVGA3D_DX11_1_MAX_UAVIEWS: c_int = 64;

pub const SVGA3D_MAX_SHADERIDS: c_int = 5000;
pub const SVGA3D_MAX_SIMULTANEOUS_SHADERS: c_int = 20000;
pub const SVGA3D_NUM_TEXTURE_UNITS: c_int = 32;
pub const SVGA3D_NUM_LIGHTS: c_int = 8;
pub const SVGA3D_MAX_VIDEOPROCESSOR_SAMPLERS: c_int = 32;

pub const SVGA3D_MAX_SHADER_THREAD_GROUPS: c_int = 65535;
pub const SVGA3D_MAX_CLIP_PLANES: c_int = 6;
pub const SVGA3D_MAX_TEXTURE_COORDS: c_int = 8;
pub const SVGA3D_MAX_SURFACE_FACES: c_int = 6;
pub const SVGA3D_SM4_MAX_SURFACE_ARRAYSIZE: c_int = 512;
pub const SVGA3D_SM5_MAX_SURFACE_ARRAYSIZE: c_int = 2048;

pub const SVGA3D_MAX_VERTEX_ARRAYS: c_int = 32;
pub const SVGA3D_MAX_DRAW_PRIMITIVE_RANGES: c_int = 32;
pub const SVGA3D_MAX_SAMPLES: c_int = 8;

