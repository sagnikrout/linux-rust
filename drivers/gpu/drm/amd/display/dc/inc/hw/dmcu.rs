//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/dmcu.h
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


// Copyright 2012-15 Advanced Micro Devices, Inc.
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

// If HW itself ever powered down it will be 0.
// fwDmcuInit will write to 1.
// Driver will only call MCP init if current state is 1,
// and the MCP command will transition this to 2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmcu_state {
    DMCU_UNLOADED = 0,
    DMCU_LOADED_UNINITIALIZED = 1,
    DMCU_RUNNING = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_version {
    pub interface_version: c_uint,
    pub abm_version: c_uint,
    pub psr_version: c_uint,
    pub build_version: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu {
    pub ctx: *mut dc_context,
    pub funcs: *const dmcu_funcs,
    pub dmcu_state: dmcu_state,
    pub dmcu_version: dmcu_version,
    pub cached_wait_loop_number: c_uint,
    pub psp_version: u32,
    pub auto_load_dmcu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_funcs {
    pub dmcu): *mut *mut bool (dmcu_init)(struct dmcu,
    pub bytes): c_uint,
    pub wait): *mut *mut *mut void (set_psr_enable)(struct dmcu dmcu, bool enable, bool,
    pub psr_context): *mut psr_context,
    pub dc_psr_state): *mut *mut *mut void (get_psr_state)(struct dmcu dmcu, enum dc_psr_state,
    pub wait_loop_number): c_uint,
    pub psr_wait_loop_number): *mut c_uint,
    pub dmcu): *mut *mut bool (is_dmcu_initialized)(struct dmcu,
    pub dmcu): *mut *mut bool (lock_phy)(struct dmcu,
    pub dmcu): *mut *mut bool (unlock_phy)(struct dmcu,
    pub length): c_int,
    pub max_frame_rate): *mut c_int,
    pub offset): *mut *mut *mut bool (recv_edid_cea_ack)(struct dmcu dmcu, int,

    pub mux_mapping): *mut otg_phy_mux,
    pub mux_mapping): *mut otg_phy_mux,

}
