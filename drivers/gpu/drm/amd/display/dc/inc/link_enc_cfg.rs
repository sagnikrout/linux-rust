//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/link_enc_cfg.h
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
// This module implements functionality for dynamically assigning DIG link
// encoder resources to display endpoints (links).
//

//
// Initialise link encoder resource tracking.
//
// Copies a link encoder assignment from another state.
//
extern "C" {
    pub fn link_enc_cfg_copy(src_ctx: *const dc_state, dst_ctx: *mut dc_state);
}
//
// Algorithm for assigning available DIG link encoders to streams.
//
// Update link_enc_assignments table and link_enc_avail list accordingly in
// struct resource_context.
//
// Loop over all streams twice:
// a) First assign encoders to unmappable endpoints.
// b) Then assign encoders to mappable endpoints.
//
// Unassign a link encoder from a stream.
//
// Update link_enc_assignments table and link_enc_avail list accordingly in
// struct resource_context.
//
// Check whether the transmitter driven by a link encoder is a mappable
// endpoint.
//
// Return stream using DIG link encoder resource. NULL if unused.
// Return link using DIG link encoder resource. NULL if unused.
// Return DIG link encoder used by link. NULL if unused.
// Return next available DIG link encoder. NULL if none available.
// Return DIG link encoder. NULL if unused.
// Return DIG link encoder used by stream in current/previous state. NULL if unused.
// Return true if encoder available to use.
extern "C" {
    pub fn link_enc_cfg_is_link_enc_avail(dc: *mut dc, eng_id: engine_id, link: *mut dc_link) -> bool;
}
// Returns true if encoder assignments in supplied state pass validity checks.
extern "C" {
    pub fn link_enc_cfg_validate(dc: *mut dc, state: *mut dc_state) -> bool;
}
// Set the link encoder assignment mode for the current_state to LINK_ENC_CFG_TRANSIENT mode.
// This indicates that a new_state is in the process of being applied to hardware.
// During this transition, old and new encoder assignments should be accessible from the old_state.
// Only allow transition into transient mode if new encoder assignments are valid.
//
extern "C" {
    pub fn link_enc_cfg_set_transient_mode(dc: *mut dc, current_state: *mut dc_state, new_state: *mut dc_state);
}
