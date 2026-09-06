//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/via_modesetting.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
// Copyright 2010 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
//
// basic modesetting functions
//

pub const VIA_PITCH_MAX: c_uint = 0x3FF8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_display_timing {
    pub hor_total: u16,
    pub hor_addr: u16,
    pub hor_blank_start: u16,
    pub hor_blank_end: u16,
    pub hor_sync_start: u16,
    pub hor_sync_end: u16,
    pub ver_total: u16,
    pub ver_addr: u16,
    pub ver_blank_start: u16,
    pub ver_blank_end: u16,
    pub ver_sync_start: u16,
    pub ver_sync_end: u16,
}

extern "C" {
    pub fn via_set_primary_timing(timing: *const via_display_timing);
}
extern "C" {
    pub fn via_set_secondary_timing(timing: *const via_display_timing);
}
extern "C" {
    pub fn via_set_primary_address(addr: u32);
}
extern "C" {
    pub fn via_set_secondary_address(addr: u32);
}
extern "C" {
    pub fn via_set_primary_pitch(pitch: u32);
}
extern "C" {
    pub fn via_set_secondary_pitch(pitch: u32);
}
extern "C" {
    pub fn via_set_primary_color_depth(depth: u8);
}
extern "C" {
    pub fn via_set_secondary_color_depth(depth: u8);
}
