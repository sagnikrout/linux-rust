//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_mixer.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2017 The Linux Foundation. All rights reserved.
//
// represents a hw Layer Mixer, one (or more) is dynamically assigned to a crtc
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_hw_mixer {
    pub idx: c_int,
    pub name: *const c_char,
    pub /: *mut *mut int lm; / the LM instance #,
    pub caps: u32,
    pub pp: c_int,
    pub dspp: c_int,
    pub /: *mut *mut uint32_t flush_mask; / used to commit LM registers,
}

// global atomic state of assignment between CRTCs and Layer Mixers:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_hw_mixer_state {
    pub hwmixer_to_crtc: [*mut drm_crtc; 8],
}
