//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/i915_drm.h
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
// Copyright 2003 Tungsten Graphics, Inc., Cedar Park, Texas.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
// IN NO EVENT SHALL TUNGSTEN GRAPHICS AND/OR ITS SUPPLIERS BE LIABLE FOR
// ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

// For use by IPS driver
extern "C" {
    pub fn i915_read_mch_val() -> c_ulong;
}
extern "C" {
    pub fn i915_gpu_raise() -> bool;
}
extern "C" {
    pub fn i915_gpu_lower() -> bool;
}
extern "C" {
    pub fn i915_gpu_busy() -> bool;
}
extern "C" {
    pub fn i915_gpu_turbo_disable() -> bool;
}
// Exported from arch/x86/kernel/early-quirks.c
//
// The bridge device's (device 0) PCI config space has information
// about the fb aperture size and the amount of pre-reserved memory.
//
// device 2 has a read-only mirror
pub const SNB_GMCH_CTRL: c_uint = 0x50;

pub const SNB_GMCH_GGMS_MASK: c_uint = 0x3;

pub const SNB_GMCH_GMS_MASK: c_uint = 0x1f;
pub const BDW_GMCH_GGMS_SHIFT: c_int = 6;
pub const BDW_GMCH_GGMS_MASK: c_uint = 0x3;
pub const BDW_GMCH_GMS_SHIFT: c_int = 8;
pub const BDW_GMCH_GMS_MASK: c_uint = 0xff;
// device 2 has a read-only mirror from i85x/i865 onwards
pub const I830_GMCH_CTRL: c_uint = 0x52;

// valid for both I830_GMCH_CTRL and SNB_GMCH_CTRL

pub const I830_DRB3: c_uint = 0x63;
pub const I85X_DRB3: c_uint = 0x43;
pub const I865_TOUD: c_uint = 0xc4;
pub const I830_ESMRAMC: c_uint = 0x91;
pub const I845_ESMRAMC: c_uint = 0x9e;
pub const I85X_ESMRAMC: c_uint = 0x61;

pub const INTEL_BSM: c_uint = 0x5c;
pub const INTEL_GEN11_BSM_DW0: c_uint = 0xc0;
pub const INTEL_GEN11_BSM_DW1: c_uint = 0xc4;

