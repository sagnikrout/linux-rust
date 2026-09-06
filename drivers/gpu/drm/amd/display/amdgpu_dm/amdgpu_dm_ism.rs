//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_ism.h
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
// Copyright 2026 Advanced Micro Devices, Inc.
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

pub const AMDGPU_DM_IDLE_HIST_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_dm_ism_state {
    DM_ISM_STATE_FULL_POWER_RUNNING,
    DM_ISM_STATE_FULL_POWER_BUSY,
    DM_ISM_STATE_HYSTERESIS_WAITING,
    DM_ISM_STATE_HYSTERESIS_BUSY,
    DM_ISM_STATE_OPTIMIZED_IDLE,
    DM_ISM_STATE_OPTIMIZED_IDLE_SSO,
    DM_ISM_STATE_TIMER_ABORTED,
    DM_ISM_NUM_STATES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_dm_ism_event {
    DM_ISM_EVENT_IMMEDIATE,
    DM_ISM_EVENT_ENTER_IDLE_REQUESTED,
    DM_ISM_EVENT_EXIT_IDLE_REQUESTED,
    DM_ISM_EVENT_BEGIN_CURSOR_UPDATE,
    DM_ISM_EVENT_END_CURSOR_UPDATE,
    DM_ISM_EVENT_TIMER_ELAPSED,
    DM_ISM_EVENT_SSO_TIMER_ELAPSED,
    DM_ISM_NUM_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_ism_config {
//
// @filter_num_frames: Idle periods shorter than this number of frames
// will be considered a "short idle period" for filtering.
//
// 0 indicates no filtering (i.e. no idle allow delay will be applied)
//
    pub filter_num_frames: c_uint,
//
// @filter_history_size: Number of recent idle periods to consider when
// counting the number of short idle periods.
//
    pub filter_history_size: c_uint,
//
// @filter_entry_count: When the number of short idle periods within
// recent &filter_history_size reaches this count, the idle allow delay
// will be applied.
//
// 0 indicates no filtering (i.e. no idle allow delay will be applied)
//
    pub filter_entry_count: c_uint,
//
// @activation_num_delay_frames: Defines the number of frames to wait
// for the idle allow delay.
//
// 0 indicates no filtering (i.e. no idle allow delay will be applied)
//
    pub activation_num_delay_frames: c_uint,
//
// @filter_old_history_threshold: A time-based restriction on top of
// &filter_history_size. Idle periods older than this threshold (in
// number of frames) will be ignored when counting the number of short
// idle periods.
//
// 0 indicates no time-based restriction, i.e. history is limited only
// by &filter_history_size.
//
    pub filter_old_history_threshold: c_uint,
//
// @sso_num_frames: Number of frames to delay before enabling static
// screen optimizations, such as PSR1 and Replay low HZ idle mode.
//
// 0 indicates immediate SSO enable upon allowing idle.
//
    pub sso_num_frames: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_ism_record {
//
// @timestamp_ns: When idle was allowed
//
    pub timestamp_ns: c_ulonglong,
//
// @duration_ns: How long idle was allowed
//
    pub duration_ns: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_ism {
    pub config: amdgpu_dm_ism_config,
    pub last_idle_timestamp_ns: c_ulonglong,
    pub current_state: amdgpu_dm_ism_state,
    pub previous_state: amdgpu_dm_ism_state,
    pub records: [amdgpu_dm_ism_record; AMDGPU_DM_IDLE_HIST_LEN],
    pub next_record_idx: c_int,
    pub delayed_work: delayed_work,
    pub sso_delayed_work: delayed_work,
}

extern "C" {
    pub fn amdgpu_dm_ism_fini(ism: *mut amdgpu_dm_ism);
}
extern "C" {
    pub fn amdgpu_dm_ism_disable(dm: *mut amdgpu_display_manager);
}
extern "C" {
    pub fn amdgpu_dm_ism_force_full_power(dm: *mut amdgpu_display_manager);
}
extern "C" {
    pub fn amdgpu_dm_ism_enable(dm: *mut amdgpu_display_manager);
}

extern "C" {
    pub fn dm_ism_insert_record(ism: *mut amdgpu_dm_ism);
}
extern "C" {
    pub fn dm_ism_set_last_idle_ts(ism: *mut amdgpu_dm_ism);
}

