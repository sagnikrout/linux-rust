//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dmub_hw_lock_mgr.h
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
// Copyright 2012-16 Advanced Micro Devices, Inc.
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

//
// should_use_dmub_inbox1_lock() - Checks if the DMCUB hardware lock via inbox1 should be used.
//
// @dc: pointer to DC object
// @link: optional pointer to the link object to check for enabled link features
//
// Return: true if the inbox1 lock should be used, false otherwise
//
extern "C" {
    pub fn should_use_dmub_inbox1_lock(dc: *const dc, link: *const dc_link) -> bool;
}
//
// dmub_hw_lock_mgr_does_link_require_lock() - Returns true if the link has a feature that needs the HW lock.
//
// @dc: Pointer to DC object
// @link: The link to check
//
// Return: true if the link has a feature that needs the HW lock, false otherwise
//
extern "C" {
    pub fn dmub_hw_lock_mgr_does_link_require_lock(dc: *const dc, link: *const dc_link) -> bool;
}
//
// dmub_hw_lock_mgr_does_context_require_lock() - Returns true if the context has any stream that needs the HW lock.
//
// @dc: Pointer to DC object
// @context: The context to check
//
// Return: true if the context has any stream that needs the HW lock, false otherwise
//
extern "C" {
    pub fn dmub_hw_lock_mgr_does_context_require_lock(dc: *const dc, context: *const dc_state) -> bool;
}
//
// should_use_dmub_inbox0_lock_for_link() - Checks if the inbox0 interlock with DMU should be used.
//
// Is not functionally equivalent to inbox1 as DMUB will not own programming of the relevant locking
// registers.
//
// @dc: pointer to DC object
// @link: optional pointer to the link object to check for enabled link features
//
// Return: true if the inbox0 lock should be used, false otherwise
//
extern "C" {
    pub fn should_use_dmub_inbox0_lock_for_link(dc: *const dc, link: *const dc_link) -> bool;
}
