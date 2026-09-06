//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/pg_cntl.h
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


// Copyright 2023 Advanced Micro Devices, Inc.
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
pub struct pg_cntl {
    pub ctx: *mut dc_context,
    pub funcs: *const pg_cntl_funcs,
    pub pg_pipe_res_enable: [bool; PG_HW_PIPE_RESOURCES_NUM_ELEMENT][MAX_PIPES],
    pub pg_res_enable: [bool; PG_HW_RESOURCES_NUM_ELEMENT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_cntl_funcs {
    pub power_on): *mut *mut *mut void (dsc_pg_control)(struct pg_cntl pg_cntl, unsigned int dsc_inst, bool,
    pub power_on): *mut *mut *mut void (hubp_dpp_pg_control)(struct pg_cntl pg_cntl, unsigned int hubp_dpp_inst, bool,
    pub power_on): *mut *mut *mut void (hpo_pg_control)(struct pg_cntl pg_cntl, bool,
    pub power_on): *mut *mut *mut void (io_clk_pg_control)(struct pg_cntl pg_cntl, bool,
    pub power_on): *mut *mut *mut void (plane_otg_pg_control)(struct pg_cntl pg_cntl, bool,
    pub power_on): *mut *mut *mut void (mpcc_pg_control)(struct pg_cntl pg_cntl, unsigned int mpcc_inst, bool,
    pub power_on): *mut *mut *mut void (opp_pg_control)(struct pg_cntl pg_cntl, unsigned int opp_inst, bool,
    pub power_on): *mut *mut *mut void (optc_pg_control)(struct pg_cntl pg_cntl, unsigned int optc_inst, bool,
    pub power_on): *mut *mut *mut void (dwb_pg_control)(struct pg_cntl pg_cntl, bool,
    pub power_on): *mut *mut *mut void (mem_pg_control)(struct pg_cntl pg_cntl, bool,
    pub power_on): *mut *mut *mut void (dio_pg_control)(struct pg_cntl pg_cntl, bool,
    pub pg_cntl): *mut *mut void (init_pg_status)(struct pg_cntl,
    pub debug_log): *const *const *const *const void (print_pg_status)(struct pg_cntl pg_cntl, char debug_func, char,
}
