//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display.h
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
// Copyright © 2006-2019 Intel Corporation
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
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// Ports identifier referenced from other drivers.
// Expected to remain stable over time
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_port {
    TC_PORT_NONE = -1,

    TC_PORT_1 = 0,
    TC_PORT_2,
    TC_PORT_3,
    TC_PORT_4,
    TC_PORT_5,
    TC_PORT_6,

    I915_MAX_TC_PORTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy {
    PHY_NONE = -1,

    PHY_A = 0,
    PHY_B,
    PHY_C,
    PHY_D,
    PHY_E,
    PHY_F,
    PHY_G,
    PHY_H,
    PHY_I,

    I915_MAX_PHYS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_fia {
    FIA1,
    FIA2,
    FIA3,
}

extern "C" {
    pub fn intel_atomic_check(dev: *mut drm_device, state: *mut drm_atomic_commit) -> c_int;
}
extern "C" {
    pub fn intel_port_to_phy(display: *mut intel_display, port: port) -> phy;
}
extern "C" {
    pub fn is_trans_port_sync_mode(state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn is_trans_port_sync_master(state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_joined_pipe_mask(crtc_state: *const intel_crtc_state) -> u8;
}
extern "C" {
    pub fn intel_crtc_is_joiner_secondary(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_is_joiner_primary(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_is_bigjoiner_primary(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_is_bigjoiner_secondary(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_is_ultrajoiner(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_is_ultrajoiner_primary(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_ultrajoiner_enable_needed(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_crtc_joiner_secondary_pipes(crtc_state: *const intel_crtc_state) -> u8;
}
extern "C" {
    pub fn _intel_modeset_primary_pipes(crtc_state: *const intel_crtc_state) -> u8;
}
extern "C" {
    pub fn _intel_modeset_secondary_pipes(crtc_state: *const intel_crtc_state) -> u8;
}
extern "C" {
    pub fn intel_crtc_get_pipe_config(crtc_state: *mut intel_crtc_state) -> bool;
}
extern "C" {
    pub fn i9xx_set_pipeconf(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn ilk_set_pipeconf(crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_enable_transcoder(new_crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn intel_disable_transcoder(old_crtc_state: *const intel_crtc_state);
}
extern "C" {
    pub fn i830_enable_pipe(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn i830_disable_pipe(display: *mut intel_display, pipe: pipe);
}
extern "C" {
    pub fn intel_has_pending_fb_unpin(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_flush_cleanup_work(display: *mut intel_display);
}
extern "C" {
    pub fn intel_encoder_destroy(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn intel_phy_is_combo(display: *mut intel_display, phy: phy) -> bool;
}
extern "C" {
    pub fn intel_phy_is_tc(display: *mut intel_display, phy: phy) -> bool;
}
extern "C" {
    pub fn intel_phy_is_snps(display: *mut intel_display, phy: phy) -> bool;
}
extern "C" {
    pub fn intel_port_to_tc(display: *mut intel_display, port: port) -> tc_port;
}
extern "C" {
    pub fn intel_tc_phy_port_to_tc(display: *mut intel_display, port: port) -> tc_port;
}
extern "C" {
    pub fn intel_encoder_to_phy(encoder: *mut intel_encoder) -> phy;
}
extern "C" {
    pub fn intel_encoder_is_combo(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_encoder_is_snps(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_encoder_is_tc(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_encoder_to_tc(encoder: *mut intel_encoder) -> tc_port;
}
extern "C" {
    pub fn ilk_get_lanes_required(target_clock: c_int, link_bw: c_int, bpp: c_int) -> c_int;
}
extern "C" {
    pub fn intel_fuzzy_clock_check(clock1: c_int, clock2: c_int) -> bool;
}
extern "C" {
    pub fn intel_zero_m_n(m_n: *mut intel_link_m_n);
}
extern "C" {
    pub fn intel_dotclock_calculate(link_freq: c_int, m_n: *const intel_link_m_n) -> c_int;
}
extern "C" {
    pub fn intel_crtc_dotclock(pipe_config: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_max_uncompressed_dotclock(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_port_to_power_domain(dig_port: *mut intel_digital_port) -> intel_display_power_domain;
}
extern "C" {
    pub fn bdw_get_pipe_misc_bpp(crtc: *mut intel_crtc) -> c_int;
}
extern "C" {
    pub fn intel_plane_fence_y_offset(plane_state: *const intel_plane_state) -> c_uint;
}
extern "C" {
    pub fn intel_plane_fixup_bitmasks(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_display_min_pipe_bpp() -> c_int;
}
extern "C" {
    pub fn intel_display_max_pipe_bpp(display: *mut intel_display) -> c_int;
}
// modesetting
// interface for intel_display_driver.c
extern "C" {
    pub fn intel_init_display_hooks(display: *mut intel_display);
}
extern "C" {
    pub fn intel_setup_outputs(display: *mut intel_display);
}
extern "C" {
    pub fn intel_initial_commit(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_panel_sanitize_ssc(display: *mut intel_display);
}
// modesetting asserts

extern "C" {
    pub fn assert_port_valid(display: *mut intel_display, port: port) -> bool;
}
//
// Use INTEL_DISPLAY_STATE_WARN(x) (rather than WARN() and WARN_ON()) for hw
// state sanity checks to check for unexpected conditions which may not
// necessarily be a user visible problem. This will either drm_WARN() or
// drm_err() depending on the verbose_state_checks module param, to enable
// distros and users to tailor their preferred amount of i915 abrt spam.
//

extern "C" {
    pub fn intel_scanout_needs_vtd_wa(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_crtc_num_joined_pipes(crtc_state: *const intel_crtc_state) -> c_int;
}
