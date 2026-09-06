//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/acpi.h
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
// acpi.h
// acpi file for domain 0 kernel
//
// Copyright (c) 2011 Konrad Rzeszutek Wilk <konrad.wilk@oracle.com>
// Copyright (c) 2011 Yu Ke <ke.yu@intel.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

extern "C" {
    pub fn int(sbdf: *mut *mut get_gsi_from_sbdf_t)(u32) -> typedef;
}

//
// Xen will save and restore CPU context, so
// we can skip that and just go straight to
// the suspend.
//
extern "C" {
    pub fn xen_pvh_setup_gsi(gsi: c_int, trigger: c_int, polarity: c_int) -> c_int;
}
extern "C" {
    pub fn xen_acpi_register_get_gsi_func(func: get_gsi_from_sbdf_t);
}
extern "C" {
    pub fn xen_acpi_get_gsi_from_sbdf(sbdf: u32) -> c_int;
}

