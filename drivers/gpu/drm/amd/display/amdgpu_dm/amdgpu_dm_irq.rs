//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_irq.h
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

//
// Display Manager IRQ-related interfaces (for use by DAL).
//
// amdgpu_dm_irq_init - Initialize internal structures of 'amdgpu_dm_irq'.
//
// This function should be called exactly once - during DM initialization.
//
// Returns:
// 0 - success
// non-zero - error
//
extern "C" {
    pub fn amdgpu_dm_irq_init(adev: *mut amdgpu_device) -> c_int;
}
//
// amdgpu_dm_irq_fini - deallocate internal structures of 'amdgpu_dm_irq'.
//
// This function should be called exactly once - during DM destruction.
//
extern "C" {
    pub fn amdgpu_dm_irq_fini(adev: *mut amdgpu_device);
}
//
// amdgpu_dm_irq_register_interrupt - register irq handler for Display block.
//
// @adev: AMD DRM device
// @int_params: parameters for the irq
// @ih: pointer to the irq hander function
// @handler_args: arguments which will be passed to ih
//
// Returns:
// IRQ Handler Index on success.
// NULL on failure.
//
// Cannot be called from an interrupt handler.
//
// amdgpu_dm_irq_unregister_interrupt - unregister handler which was registered
// by amdgpu_dm_irq_register_interrupt().
//
// @adev: AMD DRM device.
// @ih_index: irq handler index which was returned by
// amdgpu_dm_irq_register_interrupt
//
extern "C" {
    pub fn amdgpu_dm_set_irq_funcs(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dm_outbox_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dm_hpd_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dm_hpd_fini(adev: *mut amdgpu_device);
}
//
// amdgpu_dm_irq_suspend - disable ASIC interrupt during suspend.
//
extern "C" {
    pub fn amdgpu_dm_irq_suspend(adev: *mut amdgpu_device);
}
//
// amdgpu_dm_irq_resume_early - enable HPDRX ASIC interrupts during resume.
// amdgpu_dm_irq_resume - enable ASIC interrupt during resume.
//
extern "C" {
    pub fn amdgpu_dm_irq_resume_early(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_dm_irq_resume_late(adev: *mut amdgpu_device);
}
// HPD handling
extern "C" {
    pub fn amdgpu_dm_hpd_rx_irq_work_suspend(dm: *mut amdgpu_display_manager);
}
extern "C" {
    pub fn amdgpu_dm_register_hpd_handlers(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_hdmi_hpd_debounce_work(work: *mut work_struct);
}
// IRQ handlers
extern "C" {
    pub fn amdgpu_dm_dce110_register_irq_handlers(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_dcn10_register_irq_handlers(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_register_outbox_irq_handlers(adev: *mut amdgpu_device) -> c_int;
}

extern "C" {
    pub fn amdgpu_dm_hpd_to_dal_irq_source(type: c_uint) -> dc_irq_source;
}
extern "C" {
    pub fn are_sinks_equal(sink1: *const dc_sink, sink2: *const dc_sink) -> bool;
}
extern "C" {
    pub fn dm_handle_hpd_rx_offload_work(work: *mut work_struct);
}
extern "C" {
    pub fn handle_hpd_irq(param: *mut c_void);
}
extern "C" {
    pub fn handle_hpd_rx_irq(param: *mut c_void);
}
extern "C" {
    pub fn dm_pflip_high_irq(interrupt_params: *mut c_void);
}
extern "C" {
    pub fn dm_vupdate_high_irq(interrupt_params: *mut c_void);
}
extern "C" {
    pub fn dm_crtc_high_irq(interrupt_params: *mut c_void);
}
extern "C" {
    pub fn dm_handle_hpd_work(work: *mut work_struct);
}
extern "C" {
    pub fn dm_dmub_outbox1_low_irq(interrupt_params: *mut c_void);
}
extern "C" {
    pub fn dm_handle_vmin_vmax_update(offload_work: *mut work_struct);
}

