//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/cik.h
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


// cik.h -- Private header for radeon driver -*- linux-c -*-
// Copyright 2012 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
extern "C" {
    pub fn cik_enter_rlc_safe_mode(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cik_exit_rlc_safe_mode(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_mc_load_microcode(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_update_cg(rdev: *mut radeon_device, block: u32, enable: bool);
}
extern "C" {
    pub fn cik_gpu_check_soft_reset(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn cik_init_cp_pg_table(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cik_get_csb_size(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn cik_get_csb_buffer(rdev: *mut radeon_device, buffer: *mut volatile u32);
}
extern "C" {
    pub fn cik_sdma_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_sdma_enable(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn cik_sdma_fini(rdev: *mut radeon_device);
}
