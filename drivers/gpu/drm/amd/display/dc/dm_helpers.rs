//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dm_helpers.h
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
// This file defines helper functions provided by the Display Manager to
// Display Core.
//

//
// Allocate memory accessible by the GPU
//
// frame buffer allocations must be aligned to a 4096-byte boundary
//
// Returns virtual address, sets addr to physical address
//
// Free the GPU-accessible memory at the virtual address pvMem
//
// Update DP branch info
//
// Writes payload allocation table in immediate downstream device.
//
// poll pending down reply
//
// Clear payload allocation table before enable MST DP link.
//
// Polls for ACT (allocation change trigger) handled and
//
// Sends ALLOCATE_PAYLOAD message.
//
// Update mst manager relevant variables
//
// OS specific aux read callback.
//
// OS specific aux write callback.
//
extern "C" {
    pub fn dm_helpers_mst_enable_stream_features(stream: *const dc_stream_state);
}

// Macro flag: #define STATIC_IFN_KUNIT

// Macro flag: #define INLINE_IFN_KUNIT
// Macro flag: #define EXPORT_IF_KUNIT(symbol)

extern "C" {
    pub fn dm_helpers_enable_periodic_detection(ctx: *mut dc_context, enable: bool);
}
extern "C" {
    pub fn dm_set_phyd32clk(ctx: *mut dc_context, freq_khz: c_int);
}
extern "C" {
    pub fn dm_helpers_dmub_outbox_interrupt_control(ctx: *mut dc_context, enable: bool) -> bool;
}
extern "C" {
    pub fn dm_helpers_dmu_timeout(ctx: *mut dc_context);
}
extern "C" {
    pub fn dm_helpers_smu_timeout(ctx: *mut dc_context, msg_id: c_uint, param: c_uint, timeout_us: c_uint);
}
// 0x1 = Result_OK, 0xFE = Result_UnkmownCmd, 0x0 = Status_Busy

extern "C" {
    pub fn dm_get_adaptive_sync_support_type(link: *mut dc_link) -> adaptive_sync_type;
}
extern "C" {
    pub fn dm_helpers_get_sbios_edid(link: *mut dc_link, edid: *mut dc_edid) -> dc_edid_status;
}
extern "C" {
    pub fn dm_helpers_is_fullscreen(ctx: *mut dc_context, stream: *mut dc_stream_state) -> bool;
}
extern "C" {
    pub fn dm_helpers_is_hdr_on(ctx: *mut dc_context, stream: *mut dc_stream_state) -> bool;
}
