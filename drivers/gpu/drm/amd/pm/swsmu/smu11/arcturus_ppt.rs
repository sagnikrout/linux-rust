//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/smu11/arcturus_ppt.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
pub const ARCTURUS_UMD_PSTATE_GFXCLK_LEVEL: c_uint = 0x3;
pub const ARCTURUS_UMD_PSTATE_SOCCLK_LEVEL: c_uint = 0x3;
pub const ARCTURUS_UMD_PSTATE_MCLK_LEVEL: c_uint = 0x2;
pub const MAX_DPM_NUMBER: c_int = 16;
pub const MAX_PCIE_CONF: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcturus_dpm_level {
    pub enabled: bool,
    pub value: u32,
    pub param1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcturus_dpm_state {
    pub soft_min_level: u32,
    pub soft_max_level: u32,
    pub hard_min_level: u32,
    pub hard_max_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcturus_single_dpm_table {
    pub count: u32,
    pub dpm_state: arcturus_dpm_state,
    pub dpm_levels: [arcturus_dpm_level; MAX_DPM_NUMBER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcturus_pcie_table {
    pub count: u16,
    pub pcie_gen: [u8; MAX_PCIE_CONF],
    pub pcie_lane: [u8; MAX_PCIE_CONF],
    pub lclk: [u32; MAX_PCIE_CONF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arcturus_dpm_table {
    pub soc_table: arcturus_single_dpm_table,
    pub gfx_table: arcturus_single_dpm_table,
    pub mem_table: arcturus_single_dpm_table,
    pub eclk_table: arcturus_single_dpm_table,
    pub vclk_table: arcturus_single_dpm_table,
    pub dclk_table: arcturus_single_dpm_table,
    pub fclk_table: arcturus_single_dpm_table,
    pub pcie_table: arcturus_pcie_table,
}

extern "C" {
    pub fn arcturus_set_ppt_funcs(smu: *mut smu_context);
}
