//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_crtc.h
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
// R-Car Display Unit CRTCs
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

//
// struct rcar_du_crtc - the CRTC, representing a DU superposition processor
// @crtc: base DRM CRTC
// @dev: the DU device
// @clock: the CRTC functional clock
// @extclock: external pixel dot clock (optional)
// @mmio_offset: offset of the CRTC registers in the DU MMIO block
// @index: CRTC hardware index
// @initialized: whether the CRTC has been initialized and clocks enabled
// @dsysr: cached value of the DSYSR register
// @vblank_enable: whether vblank events are enabled on this CRTC
// @event: event to post when the pending page flip completes
// @flip_wait: wait queue used to signal page flip completion
// @vblank_lock: protects vblank_wait and vblank_count
// @vblank_wait: wait queue used to signal vertical blanking
// @vblank_count: number of vertical blanking interrupts to wait for
// @group: CRTC group this CRTC belongs to
// @cmm: CMM associated with this CRTC
// @vsp: VSP feeding video to this CRTC
// @vsp_pipe: index of the VSP pipeline feeding video to this CRTC
// @writeback: the writeback connector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_crtc {
    pub crtc: drm_crtc,
    pub dev: *mut rcar_du_device,
    pub clock: *mut clk,
    pub extclock: *mut clk,
    pub mmio_offset: c_uint,
    pub index: c_uint,
    pub initialized: bool,
    pub dsysr: u32,
    pub vblank_enable: bool,
    pub event: *mut drm_pending_vblank_event,
    pub flip_wait: wait_queue_head_t,
    pub vblank_lock: spinlock_t,
    pub vblank_wait: wait_queue_head_t,
    pub vblank_count: c_uint,
    pub group: *mut rcar_du_group,
    pub cmm: *mut rcar_du_cmm,
    pub vsp: *mut rcar_du_vsp,
    pub vsp_pipe: c_uint,
    pub sources: *const *const c_char,
    pub sources_count: c_uint,
    pub writeback: drm_writeback_connector,
}

//
// struct rcar_du_crtc_state - Driver-specific CRTC state
// @state: base DRM CRTC state
// @crc: CRC computation configuration
// @outputs: bitmask of the outputs (enum rcar_du_output) driven by this CRTC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_crtc_state {
    pub state: drm_crtc_state,
    pub crc: vsp1_du_crc_config,
    pub outputs: c_uint,
}

extern "C" {
    pub fn rcar_du_crtc_finish_page_flip(rcrtc: *mut rcar_du_crtc);
}
extern "C" {
    pub fn rcar_du_crtc_dsysr_clr_set(rcrtc: *mut rcar_du_crtc, clr: u32, set: u32);
}
