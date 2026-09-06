//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/evergreen.h
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


// radeon_evergreen.h -- Private header for radeon driver -*- linux-c -*-
//
// Copyright 2010 Advanced Micro Devices, Inc.
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
// PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
extern "C" {
    pub fn evergreen_is_display_hung(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn evergreen_print_gpu_status_regs(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_mc_stop(rdev: *mut radeon_device, save: *mut evergreen_mc_save);
}
extern "C" {
    pub fn evergreen_mc_resume(rdev: *mut radeon_device, save: *mut evergreen_mc_save);
}
extern "C" {
    pub fn evergreen_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_mc_program(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_irq_suspend(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_mc_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_fix_pci_max_read_req_size(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_pcie_gen2_enable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_program_aspm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_rlc_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_rlc_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_gpu_pci_config_reset(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_get_number_of_dram_channels(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn evergreen_gpu_check_soft_reset(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn evergreen_rlc_resume(rdev: *mut radeon_device) -> c_int;
}
