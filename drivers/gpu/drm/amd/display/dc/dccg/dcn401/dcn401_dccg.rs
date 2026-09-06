//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dccg/dcn401/dcn401_dccg.h
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

extern "C" {
    pub fn dccg401_init(dccg: *mut dccg);
}
extern "C" {
    pub fn dccg401_update_dpp_dto(dccg: *mut dccg, dpp_inst: c_int, req_dppclk: c_int);
}
extern "C" {
    pub fn dccg401_disable_dpstreamclk(dccg: *mut dccg, dp_hpo_inst: c_int);
}
extern "C" {
    pub fn dccg401_set_dto_dscclk(dccg: *mut dccg, inst: u32, num_slices_h: u32);
}
extern "C" {
    pub fn dccg401_enable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
}
extern "C" {
    pub fn dccg401_disable_symclk_se(dccg: *mut dccg, stream_enc_inst: u32, link_enc_inst: u32);
}
extern "C" {
    pub fn dccg401_set_hdmistreamclk(dccg: *mut dccg, src: streamclk_source, otg_inst: u32);
}
extern "C" {
    pub fn dccg401_enable_hdmicharclk(dccg: *mut dccg, hpo_inst: c_int, phypll_inst: c_int);
}
extern "C" {
    pub fn dccg401_disable_hdmicharclk(dccg: *mut dccg, hpo_inst: c_int);
}
