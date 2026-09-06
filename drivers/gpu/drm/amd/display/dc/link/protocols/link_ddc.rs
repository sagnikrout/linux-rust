//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/link/protocols/link_ddc.h
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

pub const AUX_POWER_UP_WA_DELAY: c_int = 500;
pub const I2C_OVER_AUX_DEFER_WA_DELAY: c_int = 70;
pub const DPVGA_DONGLE_AUX_DEFER_WA_DELAY: c_int = 40;
pub const I2C_OVER_AUX_DEFER_WA_DELAY_1MS: c_int = 1;

pub const EDID_SEGMENT_SIZE: c_int = 256;
extern "C" {
    pub fn link_destroy_ddc_service(ddc: *mut ddc_service);
}
extern "C" {
    pub fn link_get_ddc_aux_inst(link: *const dc_link) -> u8;
}
extern "C" {
    pub fn link_get_aux_defer_delay(ddc: *mut ddc_service) -> u32;
}
extern "C" {
    pub fn link_is_in_aux_transaction_mode(ddc: *mut ddc_service) -> bool;
}
// Attempt to submit an aux payload, retrying on timeouts, defers, and busy
// states as outlined in the DP spec.  Returns true if the request was
// successful.
//
// NOTE: The function requires explicit mutex on DM side in order to prevent
// potential race condition. DC components should call the dpcd read/write
// function in dm_helpers in order to access dpcd safely
//
extern "C" {
    pub fn link_get_fixed_vs_pe_retimer_read_address(link: *mut dc_link) -> u32;
}
extern "C" {
    pub fn link_get_fixed_vs_pe_retimer_write_address(link: *mut dc_link) -> u32;
}
