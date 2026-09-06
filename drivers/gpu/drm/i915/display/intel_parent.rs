//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_parent.h
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
// Copyright © 2025 Intel Corporation

// dpt
extern "C" {
    pub fn intel_parent_dpt_destroy(display: *mut intel_display, dpt: *mut intel_dpt);
}
extern "C" {
    pub fn intel_parent_dpt_suspend(display: *mut intel_display, dpt: *mut intel_dpt);
}
extern "C" {
    pub fn intel_parent_dpt_resume(display: *mut intel_display, dpt: *mut intel_dpt);
}
// fb_pin
// frontbuffer
extern "C" {
    pub fn intel_parent_frontbuffer_ref(display: *mut intel_display, front: *mut intel_frontbuffer);
}
extern "C" {
    pub fn intel_parent_frontbuffer_put(display: *mut intel_display, front: *mut intel_frontbuffer);
}
extern "C" {
    pub fn intel_parent_frontbuffer_flush_for_display(display: *mut intel_display, front: *mut intel_frontbuffer);
}
// hdcp
extern "C" {
    pub fn intel_parent_hdcp_gsc_check_status(display: *mut intel_display) -> bool;
}
// irq
extern "C" {
    pub fn intel_parent_irq_enabled(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_irq_synchronize(display: *mut intel_display);
}
// overlay
extern "C" {
    pub fn intel_parent_overlay_is_active(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_overlay_off(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_parent_overlay_recover_from_interrupt(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_parent_overlay_release_old_vid(display: *mut intel_display) -> c_int;
}
extern "C" {
    pub fn intel_parent_overlay_reset(display: *mut intel_display);
}
extern "C" {
    pub fn intel_parent_overlay_cleanup(display: *mut intel_display);
}
// panic
extern "C" {
    pub fn int(x: *mut *mut tiling)(unsigned int, y: c_uint, width): c_uint) -> unsigned;
}
extern "C" {
    pub fn intel_parent_panic_finish(display: *mut intel_display, panic: *mut intel_panic);
}
// pc8
extern "C" {
    pub fn intel_parent_pc8_block(display: *mut intel_display);
}
extern "C" {
    pub fn intel_parent_pc8_unblock(display: *mut intel_display);
}
// pcode
extern "C" {
    pub fn intel_parent_pcode_read(display: *mut intel_display, mbox: u32, val: *mut u32, val1: *mut u32) -> c_int;
}
extern "C" {
    pub fn intel_parent_pcode_write_timeout(display: *mut intel_display, mbox: u32, val: u32, timeout_ms: c_int) -> c_int;
}
extern "C" {
    pub fn intel_parent_pcode_write(display: *mut intel_display, mbox: u32, val: u32) -> c_int;
}
// rps
extern "C" {
    pub fn intel_parent_rps_available(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_rps_boost_if_not_started(display: *mut intel_display, fence: *mut dma_fence);
}
extern "C" {
    pub fn intel_parent_rps_mark_interactive(display: *mut intel_display, interactive: bool);
}
extern "C" {
    pub fn intel_parent_rps_ilk_irq_handler(display: *mut intel_display);
}
// stolen
extern "C" {
    pub fn intel_parent_stolen_initialized(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_stolen_node_offset(display: *mut intel_display, node: *mut intel_stolen_node) -> u32;
}
extern "C" {
    pub fn intel_parent_stolen_area_address(display: *mut intel_display) -> u64;
}
extern "C" {
    pub fn intel_parent_stolen_area_size(display: *mut intel_display) -> u64;
}
extern "C" {
    pub fn intel_parent_stolen_node_address(display: *mut intel_display, node: *mut intel_stolen_node) -> u64;
}
extern "C" {
    pub fn intel_parent_stolen_node_size(display: *mut intel_display, node: *const intel_stolen_node) -> u64;
}
extern "C" {
    pub fn intel_parent_stolen_node_free(display: *mut intel_display, node: *const intel_stolen_node);
}
// vlv iosf
extern "C" {
    pub fn intel_parent_vlv_iosf_get(display: *mut intel_display, unit_mask: c_ulong);
}
extern "C" {
    pub fn intel_parent_vlv_iosf_put(display: *mut intel_display, unit_mask: c_ulong);
}
extern "C" {
    pub fn intel_parent_vlv_iosf_read(display: *mut intel_display, unit: vlv_iosf_sb_unit, addr: u32) -> u32;
}
extern "C" {
    pub fn intel_parent_vlv_iosf_write(display: *mut intel_display, unit: vlv_iosf_sb_unit, addr: u32, val: u32) -> c_int;
}
// generic
extern "C" {
    pub fn intel_parent_has_auxccs(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_has_fenced_regions(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_vgpu_active(display: *mut intel_display) -> bool;
}
extern "C" {
    pub fn intel_parent_fence_priority_display(display: *mut intel_display, fence: *mut dma_fence);
}
