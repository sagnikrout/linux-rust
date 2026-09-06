//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dccg/dcn20/dcn20_dccg.h
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
// Copyright 2018-2026 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_registers {
    pub OTG_ADD_DROP_PIXEL_CNTL: u32,
    pub DSCCLK_SRC_SEL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dccg {
    pub base: dccg,
    pub regs: *const dccg_registers,
    pub dccg_shift: *const dccg_shift,
    pub dccg_mask: *const dccg_mask,
}

extern "C" {
    pub fn dccg2_update_dpp_dto(dccg: *mut dccg, dpp_inst: c_int, req_dppclk: c_int);
}
extern "C" {
    pub fn dccg2_init(dccg: *mut dccg);
}
extern "C" {
    pub fn dccg2_refclk_setup(dccg: *mut dccg);
}
extern "C" {
    pub fn dccg2_allow_clock_gating(dccg: *mut dccg, allow: bool);
}
extern "C" {
    pub fn dccg2_enable_memory_low_power(dccg: *mut dccg, enable: bool);
}
extern "C" {
    pub fn dccg2_is_s0i3_golden_init_wa_done(dccg: *mut dccg) -> bool;
}
extern "C" {
    pub fn dcn_dccg_destroy(dccg: *mut dccg);
}
