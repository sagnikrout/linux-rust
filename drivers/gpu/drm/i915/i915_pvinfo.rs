//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_pvinfo.h
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

// The MMIO offset of the shared info between guest and host emulator
pub const VGT_PVINFO_PAGE: c_uint = 0x78000;
pub const VGT_PVINFO_SIZE: c_uint = 0x1000;
//
// The following structure pages are defined in GEN MMIO space
// for virtualization. (One page for now)
//
pub const VGT_MAGIC: c_uint = 0x4776544776544776ULL	/* 'vGTvGTvG' */;
pub const VGT_VERSION_MAJOR: c_int = 1;
pub const VGT_VERSION_MINOR: c_int = 0;
//
// notifications from guest to vgpu device model
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vgt_g2v_type {
    VGT_G2V_PPGTT_L3_PAGE_TABLE_CREATE = 2,
    VGT_G2V_PPGTT_L3_PAGE_TABLE_DESTROY,
    VGT_G2V_PPGTT_L4_PAGE_TABLE_CREATE,
    VGT_G2V_PPGTT_L4_PAGE_TABLE_DESTROY,
    VGT_G2V_EXECLIST_CONTEXT_CREATE,
    VGT_G2V_EXECLIST_CONTEXT_DESTROY,
    VGT_G2V_MAX,
}

//
// VGT capabilities type
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgt_if {
    pub /: *mut *mut u64 magic; / VGT_MAGIC,
    pub version_major: u16,
    pub version_minor: u16,
    pub /: *mut *mut u32 vgt_id; / ID of vGT instance,
    pub /: *mut *mut u32 vgt_caps; / VGT capabilities,
    pub /: *mut *mut u32 rsv1[11]; / pad to offset 0x40,
//
// Data structure to describe the balooning info of resources.
// Each VM can only have one portion of continuous area for now.
// (May support scattered resource in future)
// (starting from offset 0x40)
//
// Aperture register balooning
    pub base: u32,
    pub size: u32,
    pub /: *mut *mut } mappable_gmadr; / aperture,
// GMADR register balooning
    pub base: u32,
    pub size: u32,
    pub /: *mut *mut } nonmappable_gmadr; / non aperture,
// allowed fence registers
    pub fence_num: u32,
    pub rsv2: [u32; 3],
    pub /: *mut *mut } avail_rs; / available/assigned resource,
    pub /: *mut *mut u32 rsv3[0x200 - 24]; / pad to half page,
//
// The bottom half page is for response from Gfx driver to hypervisor.
//
    pub rsv4: u32,
    pub /: *mut *mut u32 display_ready; / ready for display owner switch,
    pub rsv5: [u32; 4],
    pub g2v_notify: u32,
    pub rsv6: [u32; 5],
    pub cursor_x_hot: u32,
    pub cursor_y_hot: u32,
    pub lo: u32,
    pub hi: u32,
    pub pdp: [}; 4],
    pub execlist_context_descriptor_lo: u32,
    pub execlist_context_descriptor_hi: u32,
    pub /: *mut *mut u32 rsv7[0x200 - 24]; / pad to one page,
    pub __packed: },

// vGPU display status to be used by the host side
pub const VGT_DRV_DISPLAY_NOT_READY: c_int = 0;

