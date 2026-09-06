//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_utils.h
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
// Copyright © 2016 Intel Corporation
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

// (ptr) = (typeof(*ptr))0;					\
//
// check_user_mbz: Check that a user value exists and is zero
//
// Frequently in our uABI we reserve space for future extensions, and
// two ensure that userspace is prepared we enforce that space must
// be zero. (Then any future extension can safely assume a default value
// of 0.)
//
// check_user_mbz() combines checking that the user pointer is accessible
// and that the contained value is zero.
//
// Returns: -EFAULT if not accessible, -EINVAL if !zero, or 0 on success.
//

extern "C" {
    pub fn add_taint_for_CI(i915: *mut drm_i915_private, taint: c_uint);
}
//
// The system is "ok", just about surviving for the user, but
// CI results are now unreliable as the HW is very suspect.
// CI checks the taint state after every test and will reboot
// the machine if the kernel is tainted.
//

// Not supported yet

extern "C" {
    pub fn i915_vtd_active(i915: *mut drm_i915_private) -> bool;
}
extern "C" {
    pub fn i915_direct_stolen_access(i915: *mut drm_i915_private) -> bool;
}
