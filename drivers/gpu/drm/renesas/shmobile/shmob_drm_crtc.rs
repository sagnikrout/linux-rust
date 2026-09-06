//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/shmobile/shmob_drm_crtc.h
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
// shmob_drm_crtc.h  --  SH Mobile DRM CRTCs
//
// Copyright (C) 2012 Renesas Electronics Corporation
//
// Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_crtc {
    pub base: drm_crtc,
    pub event: *mut drm_pending_vblank_event,
    pub flip_wait: wait_queue_head_t,
}

// Legacy connector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmob_drm_connector {
    pub base: drm_connector,
    pub encoder: *mut drm_encoder,
    pub mode: *const videomode,
}

extern "C" {
    pub fn shmob_drm_crtc_create(sdev: *mut shmob_drm_device) -> c_int;
}
extern "C" {
    pub fn shmob_drm_crtc_finish_page_flip(scrtc: *mut shmob_drm_crtc);
}
extern "C" {
    pub fn shmob_drm_encoder_create(sdev: *mut shmob_drm_device) -> c_int;
}
