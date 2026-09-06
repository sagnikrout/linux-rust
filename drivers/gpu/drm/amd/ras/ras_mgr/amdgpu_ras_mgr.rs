//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/ras_mgr/amdgpu_ras_mgr.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_ih_type {
    RAS_IH_NONE,
    RAS_IH_FROM_BLOCK_CONTROLLER,
    RAS_IH_FROM_CONSUMER_CLIENT,
    RAS_IH_FROM_FATAL_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_ih_info {
    pub block: u32,
    pub iv_entry: amdgpu_iv_entry,
    pub pasid: u16,
    pub reset: u32,
    pub pasid_fn: pasid_notify,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_mgr {
    pub adev: *mut amdgpu_device,
    pub ras_core: *mut ras_core_context,
    pub retire_page_dwork: delayed_work,
    pub ras_event_mgr: ras_event_manager,
    pub virt_ras_cmd: *mut c_void,
    pub last_poison_consumption_seqno: u64,
    pub ras_is_ready: bool,
    pub is_debug_mode: bool,
    pub is_paused: bool,
    pub ras_event_done: completion,
}

extern "C" {
    pub fn amdgpu_enable_uniras(adev: *mut amdgpu_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_uniras_enabled(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_mgr_handle_fatal_interrupt(adev: *mut amdgpu_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_handle_controller_interrupt(adev: *mut amdgpu_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_handle_consumer_interrupt(adev: *mut amdgpu_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_dispatch_interrupt(adev: *mut amdgpu_device, ih_info: *mut ras_ih_info) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_update_ras_ecc(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_reset_gpu(adev: *mut amdgpu_device, flags: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_check_eeprom_safety_watermark(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_mgr_get_curr_nps_mode(adev: *mut amdgpu_device, nps_mode: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_is_rma(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_mgr_pre_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_post_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_resume_after_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_set_debug_mode(adev: *mut amdgpu_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_mgr_get_debug_mode(adev: *mut amdgpu_device) -> bool;
}
