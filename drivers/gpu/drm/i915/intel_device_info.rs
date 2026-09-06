//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/intel_device_info.h
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
// Copyright © 2014-2017 Intel Corporation
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

// Keep in gen based order, and chronological order within a gen
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_platform {
    INTEL_PLATFORM_UNINITIALIZED = 0,
// gen2
    INTEL_I830,
    INTEL_I845G,
    INTEL_I85X,
    INTEL_I865G,
// gen3
    INTEL_I915G,
    INTEL_I915GM,
    INTEL_I945G,
    INTEL_I945GM,
    INTEL_G33,
    INTEL_PINEVIEW,
// gen4
    INTEL_I965G,
    INTEL_I965GM,
    INTEL_G45,
    INTEL_GM45,
// gen5
    INTEL_IRONLAKE,
// gen6
    INTEL_SANDYBRIDGE,
// gen7
    INTEL_IVYBRIDGE,
    INTEL_VALLEYVIEW,
    INTEL_HASWELL,
// gen8
    INTEL_BROADWELL,
    INTEL_CHERRYVIEW,
// gen9
    INTEL_SKYLAKE,
    INTEL_BROXTON,
    INTEL_KABYLAKE,
    INTEL_GEMINILAKE,
    INTEL_COFFEELAKE,
    INTEL_COMETLAKE,
// gen11
    INTEL_ICELAKE,
    INTEL_ELKHARTLAKE,
    INTEL_JASPERLAKE,
// gen12
    INTEL_TIGERLAKE,
    INTEL_ROCKETLAKE,
    INTEL_DG1,
    INTEL_ALDERLAKE_S,
    INTEL_ALDERLAKE_P,
    INTEL_DG2,
    INTEL_METEORLAKE,
    INTEL_MAX_PLATFORMS
}

//
// Subplatform bits share the same namespace per parent platform. In other words
// it is fine for the same bit to be used on multiple parent platforms.
// Devices can belong to multiple subplatforms if needed, so it's possible to set
// multiple bits for same device.
//

// HSW/BDW/SKL/KBL/CFL

// ICL

// TGL

// DG2
pub const INTEL_SUBPLATFORM_G10: c_int = 0;
pub const INTEL_SUBPLATFORM_G11: c_int = 1;
pub const INTEL_SUBPLATFORM_G12: c_int = 2;
pub const INTEL_SUBPLATFORM_D: c_int = 3;
// ADL
pub const INTEL_SUBPLATFORM_RPL: c_int = 0;
// ADL-P
//
// As #define INTEL_SUBPLATFORM_RPL 0 will apply
// here too, SUBPLATFORM_N will have different
// bit set
//
pub const INTEL_SUBPLATFORM_N: c_int = 1;
pub const INTEL_SUBPLATFORM_RPLU: c_int = 2;
// MTL
pub const INTEL_SUBPLATFORM_ARL_H: c_int = 0;
pub const INTEL_SUBPLATFORM_ARL_U: c_int = 1;
pub const INTEL_SUBPLATFORM_ARL_S: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_ppgtt_type {
    INTEL_PPGTT_NONE = I915_GEM_PPGTT_NONE,
    INTEL_PPGTT_ALIASING = I915_GEM_PPGTT_ALIASING,
    INTEL_PPGTT_FULL = I915_GEM_PPGTT_FULL,
}

// Keep has_* in alphabetical order */ \
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ip_version {
    pub ver: u8,
    pub rel: u8,
    pub step: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_runtime_info {
//
// Single "graphics" IP version that represents
// render, compute and copy behavior.
//
    pub ip: intel_ip_version,
    pub graphics: },
    pub ip: intel_ip_version,
    pub media: },
//
// Platform mask is used for optimizing or-ed IS_PLATFORM calls into
// single runtime conditionals, and also to provide groundwork for
// future per platform, or per SKU build optimizations.
//
// Array can be extended when necessary if the corresponding
// BUILD_BUG_ON is hit.
//
    pub platform_mask: [u32; 2],
    pub device_id: u16,
    pub step: intel_step_info,
    pub /: *mut *mut unsigned int page_sizes; / page sizes supported by the HW,
    pub ppgtt_type: intel_ppgtt_type,
    pub /: *mut *mut unsigned int ppgtt_size; / log2, e.g. 31/32/48 bits,
    pub has_pooled_eu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_device_info {
    pub platform: intel_platform,
    pub /: *mut *mut unsigned int dma_mask_size; / available DMA address bits,
    pub extra_gt_list: *const intel_gt_definition,
    pub /: *mut *mut u8 gt; / GT number, 0 if undefined,
    pub /: *mut *mut intel_engine_mask_t platform_engine_mask; / Engines supported by the HW,
    pub /: *mut *mut u32 memory_regions; / regions supported by the HW,

//
// Initial runtime info. Do not access outside of i915_driver_create().
//
    pub __runtime: intel_runtime_info,
    pub cachelevel_to_pat: [u32; I915_MAX_CACHE_LEVEL],
    pub max_pat_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_driver_caps {
    pub scheduler: c_uint,
    pub has_logical_contexts:1: bool,
}

extern "C" {
    pub fn intel_device_info_runtime_init_early(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_device_info_runtime_init(dev_priv: *mut drm_i915_private);
}
