//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/dmabuf.h
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
// Copyright(c) 2017 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Zhiyuan Lv <zhiyuan.lv@intel.com>
//
// Contributors:
// Xiaoguang Chen
// Tina Zhang <tina.zhang@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_fb_info {
    pub start: __u64,
    pub start_gpa: __u64,
    pub drm_format_mod: __u64,
    pub /: *mut *mut __u32 drm_format; / drm format of plane,
    pub /: *mut *mut __u32 width; / width of plane,
    pub /: *mut *mut __u32 height; / height of plane,
    pub /: *mut *mut __u32 stride; / stride of plane,
    pub /: *mut *mut __u32 size; / size of plane in bytes, align on page,
    pub /: *mut *mut __u32 x_pos; / horizontal position of cursor plane,
    pub /: *mut *mut __u32 y_pos; / vertical position of cursor plane,
    pub /: *mut *mut __u32 x_hot; / horizontal position of cursor hotspot,
    pub /: *mut *mut __u32 y_hot; / vertical position of cursor hotspot,
    pub obj: *mut intel_vgpu_dmabuf_obj,
}

//
// struct intel_vgpu_dmabuf_obj- Intel vGPU device buffer object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_dmabuf_obj {
    pub vgpu: *mut intel_vgpu,
    pub info: *mut intel_vgpu_fb_info,
    pub dmabuf_id: __u32,
    pub kref: kref,
    pub initref: bool,
    pub list: list_head,
}

extern "C" {
    pub fn intel_vgpu_query_plane(vgpu: *mut intel_vgpu, args: *mut c_void) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_get_dmabuf(vgpu: *mut intel_vgpu, dmabuf_id: c_uint) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_dmabuf_cleanup(vgpu: *mut intel_vgpu);
}
