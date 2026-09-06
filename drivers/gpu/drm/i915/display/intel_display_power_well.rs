//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_power_well.h
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
// Copyright © 2022 Intel Corporation
//

//
// i915_power_well_id:
//
// IDs used to look up power wells. Power wells accessed directly bypassing
// the power domains framework must be assigned a unique ID. The rest of power
// wells must be assigned DISP_PW_ID_NONE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i915_power_well_id {
    DISP_PW_ID_NONE = 0,		/* must be kept zero */

    VLV_DISP_PW_DISP2D,
    BXT_DISP_PW_DPIO_CMN_A,
    VLV_DISP_PW_DPIO_CMN_BC,
    GLK_DISP_PW_DPIO_CMN_C,
    CHV_DISP_PW_DPIO_CMN_D,
    HSW_DISP_PW_GLOBAL,
    SKL_DISP_PW_MISC_IO,
    SKL_DISP_PW_1,
    SKL_DISP_PW_2,
    ICL_DISP_PW_3,
    SKL_DISP_DC_OFF,
    TGL_DISP_PW_TC_COLD_OFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_power_well_instance {
    pub name: *const c_char,
    pub list: *const intel_display_power_domain,
    pub count: u8,
    pub domain_list: *mut },
// unique identifier for this power well
    pub id: i915_power_well_id,
//
// Arbitrary data associated with this power well. Platform and power
// well specific.
//
// request/status flag index in the PUNIT power well
// control/status registers.
//
    pub idx: u8,
    pub vlv: },
    pub phy: dpio_phy,
    pub bxt: },
//
// request/status flag index in the power well
// control/status registers.
//
    pub idx: u8,
    pub hsw: },
    pub aux_ch: u8,
    pub xelpdp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_power_well_desc {
    pub ops: *const i915_power_well_ops,
    pub list: *const i915_power_well_instance,
    pub count: u8,
    pub instances: *mut },
// Mask of pipes whose IRQ logic is backed by the pw
    pub irq_pipe_mask:4: u16,
    pub always_on:1: u16,
//
// Instead of waiting for the status bit to ack enables,
// just wait a specific amount of time and then consider
// the well enabled.
//
    pub fixed_enable_delay:1: u16,
    pub has_fuses:1: u16,
//
// The pw is for an ICL+ TypeC PHY port in
// Thunderbolt mode.
//
    pub is_tc_tbt:1: u16,
// Enable timeout if greater than the default 1ms
    pub enable_timeout: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_power_well {
    pub desc: *const i915_power_well_desc,
    pub domains: intel_power_domain_mask,
// power well enable/disable usage count
    pub count: c_int,
// cached hw enabled state
    pub hw_enabled: bool,
// index into desc->instances->list
    pub instance_idx: u8,
}

extern "C" {
    pub fn intel_power_well_is_enabled_cached(power_well: *mut i915_power_well) -> bool;
}
extern "C" {
    pub fn intel_power_well_is_always_on(power_well: *mut i915_power_well) -> bool;
}
extern "C" {
    pub fn intel_power_well_refcount(power_well: *mut i915_power_well) -> c_int;
}
extern "C" {
    pub fn gen9_enable_dc5(display: *mut intel_display);
}
extern "C" {
    pub fn skl_enable_dc6(display: *mut intel_display);
}
extern "C" {
    pub fn gen9_sanitize_dc_state(display: *mut intel_display);
}
extern "C" {
    pub fn gen9_set_dc_state(display: *mut intel_display, state: u32);
}
extern "C" {
    pub fn gen9_disable_dc_states(display: *mut intel_display);
}
extern "C" {
    pub fn bxt_enable_dc9(display: *mut intel_display);
}
extern "C" {
    pub fn bxt_disable_dc9(display: *mut intel_display);
}
extern "C" {
    pub fn xe3lpd_enable_dc_count(display: *mut intel_display);
}
