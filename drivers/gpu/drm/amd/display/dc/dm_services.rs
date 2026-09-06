//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dm_services.h
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
// Authors: AMD
//
// This file defines external dependencies of Display Core.
//
// TODO: remove when DC is complete.

//
// GPU registers access
//
// enable for debugging new code, this adds 50k to the driver size.
// #define DM_CHECK_ADDR_0

extern "C" {
    pub fn cgs_read_ind_register(_arg: ctx->cgs_device, _arg: addr_space, _arg: index) -> return;
}

extern "C" {
    pub fn dc_dmub_srv_destroy(dmub_srv: *mut dc_dmub_srv);
}

//
// return number of poll before condition is met
// return 0 if condition is not meet after specified time out tries
//
extern "C" {
    pub fn snprintf_count(pBuf: *mut c_char, bufSize: c_uint, fmt: *const c_char, ...) -> c_uint;
}
// These macros need to be used with soc15 registers in order to retrieve
// the actual offset.
//

//
// Power Play (PP) interfaces
//
// Gets valid clocks levels from pplib
//
// input: clk_type - display clk / sclk / mem clk
//
// output: array of valid clock levels for given type in ascending order,
// with invalid levels filtered out
//
// DAL calls this function to notify PP about completion of Mode Set.
// For PP it means that current DCE clocks are those which were returned
// by dc_service_pp_pre_dce_clock_change(), in the 'output' parameter.
//
// If the clocks are higher than before, then PP does nothing.
//
// If the clocks are lower than before, then PP reduces the voltage.
//
// \returns	true - call is successful
// false - call failed
//
// end of PP interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct persistent_data_flag {
    pub save_per_link: bool,
    pub save_per_edid: bool,
}

extern "C" {
    pub fn dm_dmcu_set_pipe(ctx: *mut dc_context, controller_id: c_uint) -> bool;
}
//
// print-out services
//

extern "C" {
    pub fn ktime_get_raw_ns() -> return;
}
//
// performance tracing
//
extern "C" {
    pub fn dm_perf_trace_timestamp(func_name: *const c_char, line: c_uint, ctx: *mut dc_context);
}

//
// SMU message tracing
//
extern "C" {
    pub fn dm_trace_smu_enter(msg_id: u32, param_in: u32, delay: c_uint, ctx: *mut dc_context);
}
extern "C" {
    pub fn dm_trace_smu_exit(success: bool, response: u32, ctx: *mut dc_context);
}

//
// DMUB Interfaces
//
extern "C" {
    pub fn dm_execute_dmub_cmd(ctx: *const dc_context, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
}
extern "C" {
    pub fn dm_execute_dmub_cmd_list(ctx: *const dc_context, count: c_uint, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
}
//
// ACPI Interfaces
//
// Debug and verification hooks
//
extern "C" {
    pub fn dc_supports_vrr(v: dce_version) -> bool;
}
