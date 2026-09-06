//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dp.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config_limits {
    pub link_config_filter: intel_dp_link_caps_filter,
// Uncompressed DSC input or link output bpp in 1 bpp units
    pub max_bpp: int min_bpp,,
    pub pipe: },
// Compressed or uncompressed link output bpp in 1/16 bpp units
    pub max_bpp_x16: int min_bpp_x16,,
    pub link: },
}

extern "C" {
    pub fn intel_edp_fixup_vbt_bpp(encoder: *mut intel_encoder, pipe_bpp: c_int);
}
extern "C" {
    pub fn intel_dp_min_bpp(output_format: intel_output_format) -> c_int;
}
extern "C" {
    pub fn intel_dp_init_modeset_retry_work(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_dp_flush_connector_commits(connector: *mut intel_connector);
}
extern "C" {
    pub fn intel_dp_set_power(intel_dp: *mut intel_dp, mode: u8);
}
extern "C" {
    pub fn intel_dp_encoder_suspend(intel_encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_dp_encoder_shutdown(intel_encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_dp_encoder_flush_work(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn intel_dp_dsc_reset_config(crtc_state: *mut intel_crtc_state);
}
extern "C" {
    pub fn intel_dp_has_hdmi_sink(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_is_edp(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_dp_is_uhbr(crtc_state: *const intel_crtc_state) -> bool;
}
extern "C" {
    pub fn intel_dp_has_dsc(connector: *const intel_connector) -> bool;
}
extern "C" {
    pub fn intel_dp_link_symbol_size(rate: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_link_symbol_clock(rate: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_is_port_edp(display: *mut intel_display, port: port) -> bool;
}
extern "C" {
    pub fn intel_edp_backlight_off(conn_state: *const drm_connector_state);
}
extern "C" {
    pub fn intel_edp_fixup_vbt_bpp(encoder: *mut intel_encoder, pipe_bpp: c_int);
}
extern "C" {
    pub fn intel_dp_mst_suspend(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dp_mst_resume(display: *mut intel_display);
}
extern "C" {
    pub fn intel_dp_rate_limit_len(rates: *const c_int, len: c_int, max_rate: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_max_source_lane_count(dig_port: *mut intel_digital_port) -> c_int;
}
extern "C" {
    pub fn intel_dp_config_required_rate(crtc_state: *const intel_crtc_state) -> c_int;
}
extern "C" {
    pub fn intel_dp_rate_select(intel_dp: *mut intel_dp, rate: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_rate_index(rates: *const c_int, len: c_int, rate: c_int) -> c_int;
}
extern "C" {
    pub fn intel_dp_update_sink_caps(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_reset_link_params(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_source_supports_tps3(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_dp_source_supports_tps4(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_dp_has_joiner(intel_dp: *mut intel_dp) -> bool;
}
extern "C" {
    pub fn intel_digital_port_lock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_digital_port_unlock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_digital_port_connected(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_digital_port_connected_locked(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_dp_dsc_valid_compressed_bpp(intel_dp: *mut intel_dp, bpp_x16: c_int) -> bool;
}
extern "C" {
    pub fn intel_dp_mode_to_fec_clock(mode_clock: u32) -> u32;
}
extern "C" {
    pub fn intel_dp_bw_fec_overhead(fec_enabled: bool) -> c_int;
}
extern "C" {
    pub fn intel_dp_check_frl_training(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_invalidate_source_oui(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_wait_source_oui(intel_dp: *mut intel_dp);
}
extern "C" {
    pub fn intel_dp_has_gamut_metadata_dip(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_dp_dsc_max_src_input_bpc(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_dp_dsc_min_src_input_bpc() -> c_int;
}
extern "C" {
    pub fn intel_dp_dsc_min_src_compressed_bpp() -> c_int;
}
extern "C" {
    pub fn intel_dp_dsc_bpp_step_x16(connector: *const intel_connector) -> c_int;
}
extern "C" {
    pub fn intel_dp_dpcd_set_probe(intel_dp: *mut intel_dp, force_on_external: bool);
}
extern "C" {
    pub fn intel_dp_in_hdr_mode(conn_state: *const drm_connector_state) -> bool;
}
extern "C" {
    pub fn intel_dp_max_hdisplay_per_pipe(display: *mut intel_display) -> c_int;
}

extern "C" {
    pub fn intel_dp_as_sdp_transmission_time() -> u8;
}
extern "C" {
    pub fn intel_dp_link_init(intel_dp: *mut intel_dp) -> c_int;
}
extern "C" {
    pub fn intel_dp_link_cleanup(intel_dp: *mut intel_dp);
}
