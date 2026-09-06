//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/mmio.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
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
// Ke Yu
// Kevin Tian <kevin.tian@intel.com>
// Dexuan Cui
//
// Contributors:
// Tina Zhang <tina.zhang@intel.com>
// Min He <min.he@intel.com>
// Niu Bing <bing.niu@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_mmio_info {
    pub offset: u32,
    pub ro_mask: u64,
    pub read: gvt_mmio_func,
    pub write: gvt_mmio_func,
    pub node: hlist_node,
}

extern "C" {
    pub fn intel_gvt_get_device_type(gvt: *mut intel_gvt) -> c_ulong;
}
extern "C" {
    pub fn intel_gvt_setup_mmio_info(gvt: *mut intel_gvt) -> c_int;
}
extern "C" {
    pub fn intel_gvt_clean_mmio_info(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_vgpu_init_mmio(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_reset_mmio(vgpu: *mut intel_vgpu, dmlr: bool);
}
extern "C" {
    pub fn intel_vgpu_clean_mmio(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_gpa_to_mmio_offset(vgpu: *mut intel_vgpu, gpa: u64) -> c_int;
}
extern "C" {
    pub fn intel_gvt_restore_fence(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_restore_mmio(gvt: *mut intel_gvt);
}
