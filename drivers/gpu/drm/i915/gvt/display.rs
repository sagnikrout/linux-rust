//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/display.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Ke Yu
// Zhiyuan Lv <zhiyuan.lv@intel.com>
//
// Contributors:
// Terrence Xu <terrence.xu@intel.com>
// Changbin Du <changbin.du@intel.com>
// Bing Niu <bing.niu@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//

pub const SBI_REG_MAX: c_int = 20;
pub const DPCD_SIZE: c_uint = 0x700;

pub const INTEL_GVT_MAX_UEVENT_VARS: c_int = 3;

pub const AUX_BURST_SIZE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_sbi_register {
    pub offset: c_uint,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_sbi {
    pub number: c_int,
    pub registers: [intel_vgpu_sbi_register; SBI_REG_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gvt_plane_type {
    PRIMARY_PLANE = 0,
    CURSOR_PLANE,
    SPRITE_PLANE,
    MAX_PLANE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_dpcd_data {
    pub data_valid: bool,
    pub data: [u8; DPCD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_vgpu_port_type {
    GVT_CRT = 0,
    GVT_DP_A,
    GVT_DP_B,
    GVT_DP_C,
    GVT_DP_D,
    GVT_HDMI_B,
    GVT_HDMI_C,
    GVT_HDMI_D,
    GVT_PORT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_vgpu_edid {
    GVT_EDID_1024_768,
    GVT_EDID_1920_1200,
    GVT_EDID_NUM,
}

pub const GVT_DEFAULT_REFRESH_RATE: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_port {
// per display EDID information
    pub edid: *mut intel_vgpu_edid_data,
// per display DPCD information
    pub dpcd: *mut intel_vgpu_dpcd_data,
    pub type: c_int,
    pub id: intel_vgpu_edid,
// x1000 to get accurate 59.94, 24.976, 29.94, etc. in timing std.
    pub vrefresh_k: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_vblank_timer {
    pub timer: hrtimer,
    pub vrefresh_k: u32,
    pub period: u64,
}

extern "C" {
    pub fn intel_vgpu_emulate_vblank(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn vgpu_update_vblank_emulation(vgpu: *mut intel_vgpu, turnon: bool);
}
extern "C" {
    pub fn intel_vgpu_init_display(vgpu: *mut intel_vgpu, resolution: u64) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_reset_display(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_clean_display(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn pipe_is_enabled(vgpu: *mut intel_vgpu, pipe: c_int) -> c_int;
}
