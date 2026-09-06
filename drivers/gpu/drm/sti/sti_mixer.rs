//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sti/sti_mixer.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics SA 2014
// Authors: Benjamin Gaignard <benjamin.gaignard@st.com>
// Fabien Dessenne <fabien.dessenne@st.com>
// for STMicroelectronics.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_mixer_status {
    STI_MIXER_READY,
    STI_MIXER_DISABLING,
    STI_MIXER_DISABLED,
}

//
// STI Mixer subdevice structure
//
// @dev: driver device
// @regs: mixer registers
// @id: id of the mixer
// @drm_crtc: crtc object link to the mixer
// @status: to know the status of the mixer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_mixer {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub id: c_int,
    pub drm_crtc: drm_crtc,
    pub status: sti_mixer_status,
}

extern "C" {
    pub fn sti_mixer_set_plane_depth(mixer: *mut sti_mixer, plane: *mut sti_plane) -> c_int;
}
extern "C" {
    pub fn sti_mixer_set_background_status(mixer: *mut sti_mixer, enable: bool);
}
extern "C" {
    pub fn sti_mixer_debugfs_init(mixer: *mut sti_mixer, minor: *mut drm_minor);
}
// depth in Cross-bar control = z order
pub const GAM_MIXER_NB_DEPTH_LEVEL: c_int = 6;
pub const STI_MIXER_MAIN: c_int = 0;
pub const STI_MIXER_AUX: c_int = 1;
