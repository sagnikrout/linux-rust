//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_power.h
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

// -ENOENT means we got the ref, but there's no tracking

//
// Keep the pipe, transcoder, port (DDI_LANES,DDI_IO,AUX) domain instances
// consecutive, so that the pipe,transcoder,port -> power domain macros
// work correctly.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_display_power_domain {
    POWER_DOMAIN_DISPLAY_CORE,
    POWER_DOMAIN_PIPE_A,
    POWER_DOMAIN_PIPE_B,
    POWER_DOMAIN_PIPE_C,
    POWER_DOMAIN_PIPE_D,
    POWER_DOMAIN_PIPE_PANEL_FITTER_A,
    POWER_DOMAIN_PIPE_PANEL_FITTER_B,
    POWER_DOMAIN_PIPE_PANEL_FITTER_C,
    POWER_DOMAIN_PIPE_PANEL_FITTER_D,
    POWER_DOMAIN_TRANSCODER_A,
    POWER_DOMAIN_TRANSCODER_B,
    POWER_DOMAIN_TRANSCODER_C,
    POWER_DOMAIN_TRANSCODER_D,
    POWER_DOMAIN_TRANSCODER_EDP,
    POWER_DOMAIN_TRANSCODER_DSI_A,
    POWER_DOMAIN_TRANSCODER_DSI_C,

// VDSC/joining for eDP/DSI transcoder (ICL) or pipe A (TGL)
    POWER_DOMAIN_TRANSCODER_VDSC_PW2,

    POWER_DOMAIN_PORT_DDI_LANES_A,
    POWER_DOMAIN_PORT_DDI_LANES_B,
    POWER_DOMAIN_PORT_DDI_LANES_C,
    POWER_DOMAIN_PORT_DDI_LANES_D,
    POWER_DOMAIN_PORT_DDI_LANES_E,
    POWER_DOMAIN_PORT_DDI_LANES_F,

    POWER_DOMAIN_PORT_DDI_LANES_TC1,
    POWER_DOMAIN_PORT_DDI_LANES_TC2,
    POWER_DOMAIN_PORT_DDI_LANES_TC3,
    POWER_DOMAIN_PORT_DDI_LANES_TC4,
    POWER_DOMAIN_PORT_DDI_LANES_TC5,
    POWER_DOMAIN_PORT_DDI_LANES_TC6,

    POWER_DOMAIN_PORT_DDI_IO_A,
    POWER_DOMAIN_PORT_DDI_IO_B,
    POWER_DOMAIN_PORT_DDI_IO_C,
    POWER_DOMAIN_PORT_DDI_IO_D,
    POWER_DOMAIN_PORT_DDI_IO_E,
    POWER_DOMAIN_PORT_DDI_IO_F,

    POWER_DOMAIN_PORT_DDI_IO_TC1,
    POWER_DOMAIN_PORT_DDI_IO_TC2,
    POWER_DOMAIN_PORT_DDI_IO_TC3,
    POWER_DOMAIN_PORT_DDI_IO_TC4,
    POWER_DOMAIN_PORT_DDI_IO_TC5,
    POWER_DOMAIN_PORT_DDI_IO_TC6,

    POWER_DOMAIN_PORT_DSI,
    POWER_DOMAIN_PORT_CRT,
    POWER_DOMAIN_PORT_OTHER,
    POWER_DOMAIN_VGA,
    POWER_DOMAIN_AUDIO_MMIO,
    POWER_DOMAIN_AUDIO_PLAYBACK,

    POWER_DOMAIN_AUX_IO_A,
    POWER_DOMAIN_AUX_IO_B,
    POWER_DOMAIN_AUX_IO_C,
    POWER_DOMAIN_AUX_IO_D,
    POWER_DOMAIN_AUX_IO_E,
    POWER_DOMAIN_AUX_IO_F,

    POWER_DOMAIN_AUX_A,
    POWER_DOMAIN_AUX_B,
    POWER_DOMAIN_AUX_C,
    POWER_DOMAIN_AUX_D,
    POWER_DOMAIN_AUX_E,
    POWER_DOMAIN_AUX_F,

    POWER_DOMAIN_AUX_USBC1,
    POWER_DOMAIN_AUX_USBC2,
    POWER_DOMAIN_AUX_USBC3,
    POWER_DOMAIN_AUX_USBC4,
    POWER_DOMAIN_AUX_USBC5,
    POWER_DOMAIN_AUX_USBC6,

    POWER_DOMAIN_AUX_TBT1,
    POWER_DOMAIN_AUX_TBT2,
    POWER_DOMAIN_AUX_TBT3,
    POWER_DOMAIN_AUX_TBT4,
    POWER_DOMAIN_AUX_TBT5,
    POWER_DOMAIN_AUX_TBT6,

    POWER_DOMAIN_GMBUS,
    POWER_DOMAIN_GT_IRQ,
    POWER_DOMAIN_DC_OFF,
    POWER_DOMAIN_TC_COLD_OFF,
    POWER_DOMAIN_INIT,

    POWER_DOMAIN_NUM,
    POWER_DOMAIN_INVALID = POWER_DOMAIN_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_power_domain_mask {
    pub POWER_DOMAIN_NUM): DECLARE_BITMAP(bits,,
}

//
// DC3CO enabling triggers (bitmask).
// DC3CO may be enabled when at least one of these triggers is active.
// Additional constraints may still apply.
//

//
// Delay to re-enable DC5/DC6 states by 17 ms to avoid the off->on->off
// toggling overhead at and above 60 FPS.
//
pub const DC6_PUT_ASYNC_DELAY_MS: c_int = 17;
//
// Use minimal re-enable delay to allow DC3CO entry on
// the next idle frame.
//
pub const DC3CO_PUT_ASYNC_DELAY_MS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dc3co_state {
    pub /: *mut *mut mutex lock; / protects allowed and trigger fields,
    pub /: *mut *mut bool allowed; / DC3CO compute result,
    pub /: *mut *mut u32 trigger; / Bitmask of active DC3CO triggers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_power_domains {
//
// Power wells needed for initialization at driver init and suspend
// time are on. They are kept on until after the first modeset.
//
    pub initializing: bool,
    pub display_core_suspended: bool,
    pub dc3co_to_dc6: bool,
    pub power_well_count: c_int,
    pub dc_state: u32,
    pub target_dc_state: u32,
    pub allowed_dc_mask: u32,
    pub init_wakeref: *mut ref_tracker,
    pub disable_wakeref: *mut ref_tracker,
    pub lock: mutex,
    pub domain_use_count: [c_int; POWER_DOMAIN_NUM],
    pub async_put_work: delayed_work,
    pub async_put_wakeref: *mut ref_tracker,
    pub async_put_domains: [intel_power_domain_mask; 2],
    pub async_put_next_delay: c_int,
    pub power_wells: *mut i915_power_well,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_power_domain_set {
    pub mask: intel_power_domain_mask,
    pub wakerefs: [*mut ref_tracker; POWER_DOMAIN_NUM],
}

extern "C" {
    pub fn intel_display_power_init(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_display_power_cleanup(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_init_hw(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_driver_remove(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_enable(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_disable(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_sanitize_state(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_suspend_late(display: *mut intel_display, s2idle: bool);
}
extern "C" {
    pub fn intel_display_power_resume_early(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_get_and_reset_dc3co_to_dc6(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_power_get_current_dc_state(display: *mut intel_display) -> u32;
}
extern "C" {
    pub fn intel_display_power_dc3co_supported(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_power_dc3co_update(display: *mut intel_display, trigger: u32);
}
extern "C" {
    pub fn intel_display_power_dc3co_allowed(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_display_power_dc3co_compute(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_display_power_select_target_dc_state(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_display_power_runtime_suspend(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_runtime_resume(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_power_flush_work(display: *mut intel_display);
}

extern "C" {
    pub fn intel_display_power_debug(display: *mut intel_display, m: *mut seq_file);
}
//
// FIXME: We should probably switch this to a 0-based scheme to be consistent
// with how we now name/number DBUF_CTL instances.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbuf_slice {
    DBUF_S1,
    DBUF_S2,
    DBUF_S3,
    DBUF_S4,
    I915_MAX_DBUF_SLICES
}

