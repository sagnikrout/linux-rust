//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dc/dc-kms.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2024 NXP
//

pub const DC_CRTC_IRQS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_crtc_irq {
    pub dc_crtc: *mut dc_crtc,
    pub irq: c_uint,
}

//
// struct dc_crtc - DC specific drm_crtc
//
// Each display controller contains one content stream and one safety stream.
// In general, the two streams have the same functionality. One stream is
// overlaid on the other by @fg. This driver chooses to generate black constant
// color from the content stream as background color, build plane(s) on the
// content stream by using layerblend(s) and always generate a constant color
// from the safety stream. Note that due to the decoupled timing, the safety
// stream still works to show the constant color properly even when the content
// stream has completely hung up due to mal-function of this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_crtc {
// @base: base drm_crtc structure
    pub base: drm_crtc,
// @de: display engine
    pub de: *mut dc_de,
// @cf_cont: content stream constframe
    pub cf_cont: *mut dc_cf,
// @cf_safe: safety stream constframe
    pub cf_safe: *mut dc_cf,
// @ed_cont: content stream extdst
    pub ed_cont: *mut dc_ed,
// @ed_safe: safety stream extdst
    pub ed_safe: *mut dc_ed,
// @fg: framegen
    pub fg: *mut dc_fg,
//
// @irq_dec_framecomplete:
//
// display engine configuration frame complete interrupt
//
    pub irq_dec_framecomplete: c_uint,
//
// @irq_dec_seqcomplete:
//
// display engine configuration sequence complete interrupt
//
    pub irq_dec_seqcomplete: c_uint,
//
// @irq_dec_shdload:
//
// display engine configuration shadow load interrupt
//
    pub irq_dec_shdload: c_uint,
//
// @irq_ed_cont_shdload:
//
// content stream extdst shadow load interrupt
//
    pub irq_ed_cont_shdload: c_uint,
//
// @irq_ed_safe_shdload:
//
// safety stream extdst shadow load interrupt
//
    pub irq_ed_safe_shdload: c_uint,
//
// @dec_seqcomplete_done:
//
// display engine configuration sequence completion
//
    pub dec_seqcomplete_done: completion,
//
// @dec_shdload_done:
//
// display engine configuration shadow load completion
//
    pub dec_shdload_done: completion,
//
// @ed_cont_shdload_done:
//
// content stream extdst shadow load completion
//
    pub ed_cont_shdload_done: completion,
//
// @ed_safe_shdload_done:
//
// safety stream extdst shadow load completion
//
    pub ed_safe_shdload_done: completion,
// @event: cached pending vblank event
    pub event: *mut drm_pending_vblank_event,
// @irqs: interrupt list
    pub irqs: [dc_crtc_irq; DC_CRTC_IRQS],
}

//
// struct dc_plane - DC specific drm_plane
//
// Build a plane on content stream with a fetchunit and a layerblend.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane {
// @base: base drm_plane structure
    pub base: drm_plane,
// @fu: fetchunit
    pub fu: *mut dc_fu,
// @cf: content stream constframe
    pub cf: *mut dc_cf,
// @lb: layerblend
    pub lb: *mut dc_lb,
// @ed: content stream extdst
    pub ed: *mut dc_ed,
}
