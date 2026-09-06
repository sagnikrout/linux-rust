//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_rps.h
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

pub const GT_FREQUENCY_MULTIPLIER: c_int = 50;
pub const GEN9_FREQ_SCALER: c_int = 3;
extern "C" {
    pub fn intel_rps_init_early(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_init(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_sanitize(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_driver_register(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_driver_unregister(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_enable(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_disable(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_park(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_unpark(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_boost(rq: *mut i915_request);
}
extern "C" {
    pub fn intel_rps_dec_waiters(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_get_boost_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_set_boost_frequency(rps: *mut intel_rps, freq: u32) -> c_int;
}
extern "C" {
    pub fn intel_rps_set(rps: *mut intel_rps, val: u8) -> c_int;
}
extern "C" {
    pub fn intel_rps_mark_interactive(rps: *mut intel_rps, interactive: bool);
}
extern "C" {
    pub fn intel_gpu_freq(rps: *mut intel_rps, val: c_int) -> c_int;
}
extern "C" {
    pub fn intel_freq_opcode(rps: *mut intel_rps, val: c_int) -> c_int;
}
extern "C" {
    pub fn intel_rps_get_up_threshold(rps: *mut intel_rps) -> u8;
}
extern "C" {
    pub fn intel_rps_set_up_threshold(rps: *mut intel_rps, threshold: u8) -> c_int;
}
extern "C" {
    pub fn intel_rps_get_down_threshold(rps: *mut intel_rps) -> u8;
}
extern "C" {
    pub fn intel_rps_set_down_threshold(rps: *mut intel_rps, threshold: u8) -> c_int;
}
extern "C" {
    pub fn intel_rps_read_actual_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_read_actual_frequency_fw(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_requested_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_min_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_min_raw_freq(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_set_min_frequency(rps: *mut intel_rps, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_rps_get_max_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_max_raw_freq(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_set_max_frequency(rps: *mut intel_rps, val: u32) -> c_int;
}
extern "C" {
    pub fn intel_rps_get_rp0_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_rp1_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_get_rpn_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_read_punit_req_frequency(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn intel_rps_read_rpstat(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn gen6_rps_get_freq_caps(rps: *mut intel_rps, caps: *mut intel_rps_freq_caps);
}
extern "C" {
    pub fn intel_rps_raise_unslice(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_lower_unslice(rps: *mut intel_rps);
}
extern "C" {
    pub fn intel_rps_read_throttle_reason(rps: *mut intel_rps) -> u32;
}
extern "C" {
    pub fn rps_read_mask_mmio(rps: *mut intel_rps, reg32: i915_reg_t, mask: u32) -> bool;
}
extern "C" {
    pub fn gen6_rps_frequency_dump(rps: *mut intel_rps, p: *mut drm_printer);
}
extern "C" {
    pub fn gen5_rps_irq_handler(rps: *mut intel_rps);
}
extern "C" {
    pub fn gen6_rps_irq_handler(rps: *mut intel_rps, pm_iir: u32);
}
extern "C" {
    pub fn gen11_rps_irq_handler(rps: *mut intel_rps, pm_iir: u32);
}
extern "C" {
    pub fn test_bit(_arg: INTEL_RPS_ENABLED, _arg: &rps->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: INTEL_RPS_ACTIVE, _arg: &rps->flags) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: INTEL_RPS_ACTIVE, _arg: &rps->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: INTEL_RPS_INTERRUPTS, _arg: &rps->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: INTEL_RPS_TIMER, _arg: &rps->flags) -> return;
}
