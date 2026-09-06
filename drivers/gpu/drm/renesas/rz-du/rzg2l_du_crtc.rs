//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rz-du/rzg2l_du_crtc.h
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
// RZ/G2L Display Unit CRTCs
//
// Copyright (C) 2023 Renesas Electronics Corporation
//
// Based on rcar_du_crtc.h
//

//
// struct rzg2l_du_crtc - the CRTC, representing a DU superposition processor
// @crtc: base DRM CRTC
// @dev: the DU device
// @initialized: whether the CRTC has been initialized and clocks enabled
// @vblank_enable: whether vblank events are enabled on this CRTC
// @event: event to post when the pending page flip completes
// @flip_wait: wait queue used to signal page flip completion
// @vsp: VSP feeding video to this CRTC
// @vsp_pipe: index of the VSP pipeline feeding video to this CRTC
// @rstc: reset controller
// @rzg2l_clocks: the bus, main and video clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_crtc {
    pub crtc: drm_crtc,
    pub dev: *mut rzg2l_du_device,
    pub initialized: bool,
    pub vblank_enable: bool,
    pub event: *mut drm_pending_vblank_event,
    pub flip_wait: wait_queue_head_t,
    pub vsp: *mut rzg2l_du_vsp,
    pub vsp_pipe: c_uint,
    pub sources: *const *const c_char,
    pub sources_count: c_uint,
    pub rstc: *mut reset_control,
    pub aclk: *mut clk,
    pub pclk: *mut clk,
    pub dclk: *mut clk,
    pub rzg2l_clocks: },
}

extern "C" {
    pub fn container_of(_arg: c, rzg2l_du_crtc: struct, _arg: crtc) -> return;
}
//
// struct rzg2l_du_crtc_state - Driver-specific CRTC state
// @state: base DRM CRTC state
// @outputs: bitmask of the outputs (enum rzg2l_du_output) driven by this CRTC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_crtc_state {
    pub state: drm_crtc_state,
    pub outputs: c_uint,
}

extern "C" {
    pub fn container_of(_arg: s, rzg2l_du_crtc_state: struct, _arg: state) -> return;
}
extern "C" {
    pub fn rzg2l_du_crtc_create(rcdu: *mut rzg2l_du_device) -> c_int;
}
extern "C" {
    pub fn rzg2l_du_crtc_finish_page_flip(rcrtc: *mut rzg2l_du_crtc);
}
