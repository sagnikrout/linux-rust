//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/power.h
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
// Copyright (c) 2009-2011, Intel Corporation.
// All Rights Reserved.
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
// Benjamin Defnet <benjamin.r.defnet@intel.com>
// Rajesh Poornachandran <rajesh.poornachandran@intel.com>
// Massively reworked
// Alan Cox <alan@linux.intel.com>
//

extern "C" {
    pub fn gma_power_init(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_power_uninit(dev: *mut drm_device);
}
//
// The kernel bus power management  will call these functions
//
extern "C" {
    pub fn gma_power_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn gma_power_resume(dev: *mut device) -> c_int;
}
//
// These are the functions the driver should use to wrap all hw access
// (i.e. register reads and writes)
//
extern "C" {
    pub fn gma_power_begin(dev: *mut drm_device, force: bool) -> bool;
}
extern "C" {
    pub fn gma_power_end(dev: *mut drm_device);
}
