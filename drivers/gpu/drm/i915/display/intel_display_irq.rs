//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_irq.h
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
// Copyright © 2023 Intel Corporation
//

extern "C" {
    pub fn xelpdp_pica_aux_mask(display: *mut intel_display) -> u32;
}
extern "C" {
    pub fn valleyview_enable_display_irqs(display: *mut intel_display);
}
extern "C" {
    pub fn valleyview_disable_display_irqs(display: *mut intel_display);
}
extern "C" {
    pub fn ilk_enable_display_irq(display: *mut intel_display, bits: u32);
}
extern "C" {
    pub fn ilk_disable_display_irq(display: *mut intel_display, bits: u32);
}
extern "C" {
    pub fn bdw_update_port_irq(display: *mut intel_display, interrupt_mask: u32, enabled_irq_mask: u32);
}
extern "C" {
    pub fn bdw_enable_pipe_irq(display: *mut intel_display, pipe: pipe, bits: u32);
}
extern "C" {
    pub fn bdw_disable_pipe_irq(display: *mut intel_display, pipe: pipe, bits: u32);
}
extern "C" {
    pub fn ibx_enable_display_interrupt(display: *mut intel_display, bits: u32);
}
extern "C" {
    pub fn ibx_disable_display_interrupt(display: *mut intel_display, bits: u32);
}
extern "C" {
    pub fn gen8_irq_power_well_post_enable(display: *mut intel_display, pipe_mask: u8);
}
extern "C" {
    pub fn gen8_irq_power_well_pre_disable(display: *mut intel_display, pipe_mask: u8);
}
extern "C" {
    pub fn i8xx_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn i915gm_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn i965_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn ilk_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn bdw_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn i8xx_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn i915gm_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn i965_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn ilk_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn bdw_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn ilk_display_irq_master_disable(display: *mut intel_display, de_ier: *mut u32, sde_ier: *mut u32);
}
extern "C" {
    pub fn ilk_display_irq_master_enable(display: *mut intel_display, de_ier: u32, sde_ier: u32);
}
extern "C" {
    pub fn gen11_gu_misc_irq_ack(display: *mut intel_display, master_ctl: u32) -> u32;
}
extern "C" {
    pub fn gen11_gu_misc_irq_handler(display: *mut intel_display, iir: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_display_irq_state {
    pub master_ctl: u32,
    pub iir: u32,
    pub eir: u32,
    pub hotplug_status: u32,
    pub dpinvgtt: u32,
    pub pipe_stats: [u32; I915_MAX_PIPES],
}

extern "C" {
    pub fn intel_display_irq_reset(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_irq_postinstall(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_irq_ack(display: *mut intel_display, state: *mut intel_display_irq_state);
}
extern "C" {
    pub fn intel_display_irq_handler(display: *mut intel_display, state: *const intel_display_irq_state) -> bool;
}
extern "C" {
    pub fn i9xx_display_irq_enable_mask(display: *mut intel_display) -> u32;
}
extern "C" {
    pub fn i915_pipestat_enable_mask(display: *mut intel_display, pipe: pipe) -> u32;
}
extern "C" {
    pub fn i915_enable_pipestat(display: *mut intel_display, pipe: pipe, status_mask: u32);
}
extern "C" {
    pub fn i915_disable_pipestat(display: *mut intel_display, pipe: pipe, status_mask: u32);
}
extern "C" {
    pub fn intel_display_irq_init(display: *mut intel_display);
}
extern "C" {
    pub fn i915gm_irq_cstate_wa(display: *mut intel_display, enable: bool);
}
extern "C" {
    pub fn intel_display_irq_snapshot_print(snapshot: *const intel_display_irq_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_display_irq_port_interrupt_mask(display: *mut intel_display, bits: u32, mask: bool);
}
