//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn32/dcn32_clk_mgr_smu_msg.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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

pub const FCLK_PSTATE_NOTSUPPORTED: c_uint = 0x00;
pub const FCLK_PSTATE_SUPPORTED: c_uint = 0x01;
// TODO Remove this MSG ID define after it becomes available in dalsmc
pub const DALSMC_MSG_SetCabForUclkPstate: c_uint = 0x12;
pub const DALSMC_Result_OK: c_uint = 0x1;
extern "C" {
    pub fn dcn32_smu_send_fclk_pstate_message(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn dcn32_smu_send_cab_for_uclk_message(clk_mgr: *mut clk_mgr_internal, num_ways: c_uint);
}
extern "C" {
    pub fn dcn32_smu_transfer_wm_table_dram_2_smu(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn32_smu_set_pme_workaround(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn32_smu_set_hard_min_by_freq(clk_mgr: *mut clk_mgr_internal, clk: u32, freq_mhz: u16) -> c_uint;
}
extern "C" {
    pub fn dcn32_smu_wait_for_dmub_ack_mclk(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
