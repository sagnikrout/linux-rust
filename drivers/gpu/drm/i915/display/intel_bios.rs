//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_bios.h
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
// Copyright © 2016-2019 Intel Corporation
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
// Please use intel_vbt_defs.h for VBT private data, to hide and abstract away
// the VBT from the rest of the driver. Add the parsed, clean data to struct
// intel_vbt_data within struct intel_display.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_backlight_type {
    INTEL_BACKLIGHT_PMIC,
    INTEL_BACKLIGHT_LPSS,
    INTEL_BACKLIGHT_DISPLAY_DDI,
    INTEL_BACKLIGHT_DSI_DCS,
    INTEL_BACKLIGHT_PANEL_DRIVER_INTERFACE,
    INTEL_BACKLIGHT_VESA_EDP_AUX_INTERFACE,
}

extern "C" {
    pub fn intel_bios_init(display: *mut intel_display);
}
extern "C" {
    pub fn intel_bios_fini_panel(panel: *mut intel_panel);
}
extern "C" {
    pub fn intel_bios_driver_remove(display: *mut intel_display);
}
extern "C" {
    pub fn intel_bios_is_tv_present(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_bios_is_lvds_present(display: *mut intel_display, i2c_pin: *mut u8) -> bool;
}
extern "C" {
    pub fn intel_bios_is_port_present(display: *mut intel_display, port: port) -> bool;
}
extern "C" {
    pub fn intel_bios_is_dsi_present(display: *mut intel_display, port: *mut port) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_dvi(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_hdmi(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_dp(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_edp(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_typec_usb(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_tbt(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_is_dedicated_external(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_dyn_port_over_tc(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_dsi(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_supports_dp_dual_mode(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_is_lspcon(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_lane_reversal(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_hpd_invert(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_encoder_port(devdata: *const intel_bios_encoder_data) -> port;
}
extern "C" {
    pub fn intel_bios_dp_aux_ch(devdata: *const intel_bios_encoder_data) -> aux_ch;
}
extern "C" {
    pub fn intel_bios_dp_boost_level(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_dp_max_lane_count(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_dp_max_link_rate(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_dp_has_shared_aux_ch(devdata: *const intel_bios_encoder_data) -> bool;
}
extern "C" {
    pub fn intel_bios_hdmi_boost_level(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_hdmi_ddc_pin(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_hdmi_level_shift(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_hdmi_max_tmds_clock(devdata: *const intel_bios_encoder_data) -> c_int;
}
extern "C" {
    pub fn intel_bios_debugfs_register(display: *mut intel_display);
}
