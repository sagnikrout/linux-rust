//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/hwmgr/smu7_powertune.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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
pub const DIDT_SQ_CTRL0__UNUSED_0_MASK: c_uint = 0xfffc0000;
pub const DIDT_SQ_CTRL0__UNUSED_0__SHIFT: c_uint = 0x12;
pub const DIDT_TD_CTRL0__UNUSED_0_MASK: c_uint = 0xfffc0000;
pub const DIDT_TD_CTRL0__UNUSED_0__SHIFT: c_uint = 0x12;
pub const DIDT_TCP_CTRL0__UNUSED_0_MASK: c_uint = 0xfffc0000;
pub const DIDT_TCP_CTRL0__UNUSED_0__SHIFT: c_uint = 0x12;
pub const DIDT_SQ_TUNING_CTRL__UNUSED_0_MASK: c_uint = 0xc0000000;
pub const DIDT_SQ_TUNING_CTRL__UNUSED_0__SHIFT: c_uint = 0x0000001e;
pub const DIDT_TD_TUNING_CTRL__UNUSED_0_MASK: c_uint = 0xc0000000;
pub const DIDT_TD_TUNING_CTRL__UNUSED_0__SHIFT: c_uint = 0x0000001e;
pub const DIDT_TCP_TUNING_CTRL__UNUSED_0_MASK: c_uint = 0xc0000000;
pub const DIDT_TCP_TUNING_CTRL__UNUSED_0__SHIFT: c_uint = 0x0000001e;
// PowerContainment Features
pub const POWERCONTAINMENT_FEATURE_DTE: c_uint = 0x00000001;
pub const POWERCONTAINMENT_FEATURE_TDCLimit: c_uint = 0x00000002;
pub const POWERCONTAINMENT_FEATURE_PkgPwrLimit: c_uint = 0x00000004;
pub const ixGC_CAC_CNTL: c_uint = 0x0000;
pub const ixDIDT_SQ_STALL_CTRL: c_uint = 0x0004;
pub const ixDIDT_SQ_TUNING_CTRL: c_uint = 0x0005;
pub const ixDIDT_TD_STALL_CTRL: c_uint = 0x0044;
pub const ixDIDT_TD_TUNING_CTRL: c_uint = 0x0045;
pub const ixDIDT_TCP_STALL_CTRL: c_uint = 0x0064;
pub const ixDIDT_TCP_TUNING_CTRL: c_uint = 0x0065;
extern "C" {
    pub fn smu7_enable_smc_cac(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_disable_smc_cac(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_enable_power_containment(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_disable_power_containment(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_set_power_limit(hwmgr: *mut pp_hwmgr, n: u32) -> c_int;
}
extern "C" {
    pub fn smu7_power_control_set_level(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_enable_didt_config(hwmgr: *mut pp_hwmgr) -> c_int;
}
extern "C" {
    pub fn smu7_disable_didt_config(hwmgr: *mut pp_hwmgr) -> c_int;
}
