//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_irq.h
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

extern "C" {
    pub fn intel_irq_init(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_irq_fini(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_irq_install(dev_priv: *mut drm_i915_private) -> c_int;
}
extern "C" {
    pub fn intel_irq_uninstall(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen5_enable_gt_irq(dev_priv: *mut drm_i915_private, mask: u32);
}
extern "C" {
    pub fn gen5_disable_gt_irq(dev_priv: *mut drm_i915_private, mask: u32);
}
extern "C" {
    pub fn gen11_reset_rps_interrupts(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen6_reset_rps_interrupts(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen6_enable_rps_interrupts(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen6_disable_rps_interrupts(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen6_rps_reset_ei(dev_priv: *mut drm_i915_private);
}
extern "C" {
    pub fn gen6_sanitize_rps_pm_mask(i915: *const drm_i915_private, mask: u32) -> u32;
}
extern "C" {
    pub fn intel_irq_suspend(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_irq_resume(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_irqs_enabled(dev_priv: *mut drm_i915_private) -> bool;
}
extern "C" {
    pub fn intel_synchronize_irq(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn intel_synchronize_hardirq(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn gen2_assert_iir_is_zero(uncore: *mut intel_uncore, reg: i915_reg_t);
}
extern "C" {
    pub fn gen2_irq_reset(uncore: *mut intel_uncore, regs: i915_irq_regs);
}
extern "C" {
    pub fn gen2_error_reset(uncore: *mut intel_uncore, regs: i915_error_regs);
}
