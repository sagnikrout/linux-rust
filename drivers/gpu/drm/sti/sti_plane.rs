//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sti/sti_plane.h
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
// Author: Benjamin Gaignard <benjamin.gaignard@st.com> for STMicroelectronics.
//

pub const STI_PLANE_TYPE_SHIFT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_plane_type {
    STI_GDP = 1 << STI_PLANE_TYPE_SHIFT,
    STI_VDP = 2 << STI_PLANE_TYPE_SHIFT,
    STI_CUR = 3 << STI_PLANE_TYPE_SHIFT,
    STI_BCK = 4 << STI_PLANE_TYPE_SHIFT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_plane_id_of_type {
    STI_ID_0 = 0,
    STI_ID_1 = 1,
    STI_ID_2 = 2,
    STI_ID_3 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_plane_desc {
    STI_GDP_0       = STI_GDP | STI_ID_0,
    STI_GDP_1       = STI_GDP | STI_ID_1,
    STI_GDP_2       = STI_GDP | STI_ID_2,
    STI_GDP_3       = STI_GDP | STI_ID_3,
    STI_HQVDP_0     = STI_VDP | STI_ID_0,
    STI_CURSOR      = STI_CUR,
    STI_BACK        = STI_BCK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_plane_status {
    STI_PLANE_READY,
    STI_PLANE_UPDATED,
    STI_PLANE_DISABLING,
    STI_PLANE_FLUSHING,
    STI_PLANE_DISABLED,
}

pub const FPS_LENGTH: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_fps_info {
    pub output: bool,
    pub curr_frame_counter: c_uint,
    pub last_frame_counter: c_uint,
    pub curr_field_counter: c_uint,
    pub last_field_counter: c_uint,
    pub last_timestamp: ktime_t,
    pub fps_str: [c_char; FPS_LENGTH],
    pub fips_str: [c_char; FPS_LENGTH],
}

//
// STI plane structure
//
// @plane:              drm plane it is bound to (if any)
// @desc:               plane type & id
// @status:             to know the status of the plane
// @fps_info:           frame per second info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_plane {
    pub drm_plane: drm_plane,
    pub desc: sti_plane_desc,
    pub status: sti_plane_status,
    pub fps_info: sti_fps_info,
}
