//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/link/protocols/link_dp_capability.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn detect_dp_sink_caps(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn detect_edp_sink_caps(link: *mut dc_link);
}
extern "C" {
    pub fn dp_get_max_link_cap(link: *mut dc_link) -> dc_link_settings;
}
extern "C" {
    pub fn dp_retrieve_lttpr_cap(link: *mut dc_link) -> dc_status;
}
// Convert PHY repeater count read from DPCD uint8_t.
extern "C" {
    pub fn dp_parse_lttpr_repeater_count(lttpr_repeater_count: u8) -> u8;
}
// Calculate embedded LTTPR address offset for vendor-specific behaviour
extern "C" {
    pub fn dp_get_closest_lttpr_offset(lttpr_count: u8) -> u32;
}
extern "C" {
    pub fn dp_is_sink_present(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn dp_is_lttpr_present(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn dp_is_fec_supported(link: *const dc_link) -> bool;
}
extern "C" {
    pub fn is_dp_active_dongle(link: *const dc_link) -> bool;
}
extern "C" {
    pub fn is_dp_branch_device(link: *const dc_link) -> bool;
}
extern "C" {
    pub fn dpcd_write_cable_id_to_dprx(link: *mut dc_link);
}
extern "C" {
    pub fn dp_should_enable_fec(link: *const dc_link) -> bool;
}
extern "C" {
    pub fn dp_is_128b_132b_signal(pipe_ctx: *mut pipe_ctx) -> bool;
}
// Initialize output parameter lt_settings.
extern "C" {
    pub fn mst_decide_link_encoding_format(link: *const dc_link) -> dp_link_encoding;
}
extern "C" {
    pub fn dpcd_set_source_specific_data(link: *mut dc_link);
}
// query dpcd for version and mst cap addresses
extern "C" {
    pub fn read_is_mst_supported(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn link_bw_kbps_from_raw_frl_link_rate_data(bw: u8) -> u32;
}
extern "C" {
    pub fn dp_overwrite_extended_receiver_cap(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn dp_get_lttpr_count(link: *mut dc_link) -> u8;
}
