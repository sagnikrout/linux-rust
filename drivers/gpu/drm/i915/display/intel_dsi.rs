//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dsi.h
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
// Copyright © 2013 Intel Corporation
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

pub const INTEL_DSI_VIDEO_MODE: c_int = 0;
pub const INTEL_DSI_COMMAND_MODE: c_int = 1;
// Dual Link support
pub const DSI_DUAL_LINK_NONE: c_int = 0;
pub const DSI_DUAL_LINK_FRONT_BACK: c_int = 1;
pub const DSI_DUAL_LINK_PIXEL_ALT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dsi {
    pub base: intel_encoder,
    pub dsi_hosts: [*mut intel_dsi_host; I915_MAX_PORTS],
    pub io_wakeref: [*mut ref_tracker; I915_MAX_PORTS],
// GPIO Desc for panel and backlight control
    pub gpio_panel: *mut gpio_desc,
    pub gpio_backlight: *mut gpio_desc,
    pub attached_connector: *mut intel_connector,
// bit mask of ports (vlv dsi) or phys (icl dsi) being driven
    pub /: *mut *mut u16 ports; / VLV DSI,
    pub /: *mut *mut u16 phys; / ICL DSI,
}

// virtual channel
// Video mode or command mode
// number of DSI lanes
// i2c bus associated with the target device
//
// video mode pixel format
//
// XXX: consolidate on .format in struct mipi_dsi_device.
//
// NON_BURST_SYNC_PULSE, NON_BURST_SYNC_EVENTS, or BURST_MODE
// RGB or BGR
// data lanes dphy timing
// timeouts in byte clocks
// all delays in ms
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dsi_host {
    pub base: mipi_dsi_host,
    pub intel_dsi: *mut intel_dsi,
    pub port: port,
// our little hack
    pub device: *mut mipi_dsi_device,
}

extern "C" {
    pub fn container_of(_arg: h, intel_dsi_host: struct, _arg: base) -> return;
}

extern "C" {
    pub fn container_of(_arg: &encoder->base, intel_dsi: struct, _arg: base.base) -> return;
}
extern "C" {
    pub fn intel_dsi_bitrate(intel_dsi: *const intel_dsi) -> c_int;
}
extern "C" {
    pub fn intel_dsi_tlpx_ns(intel_dsi: *const intel_dsi) -> c_int;
}
extern "C" {
    pub fn intel_dsi_get_modes(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn intel_dsi_wait_panel_power_cycle(intel_dsi: *mut intel_dsi);
}
extern "C" {
    pub fn intel_dsi_shutdown(encoder: *mut intel_encoder);
}
