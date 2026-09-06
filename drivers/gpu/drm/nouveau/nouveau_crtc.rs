//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_crtc.h
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


//
// Copyright (C) 2008 Maarten Maathuis.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_crtc {
    pub base: drm_crtc,
    pub head: nvif_head,
    pub index: c_int,
    pub vblank: nvif_event,
    pub dpms_saved_fp_control: u32,
    pub fp_users: u32,
    pub saturation: c_int,
    pub sharpness: c_int,
    pub last_dpms: c_int,
    pub cursor_saved_y: int cursor_saved_x,,
    pub cpp: c_int,
    pub blanked: bool,
    pub offset: u32,
    pub handle: u32,
    pub fb: },
    pub nvbo: *mut nouveau_bo,
    pub offset: u32,
    pub offset): *mut *mut *mut void (set_offset)(struct nouveau_crtc , uint32_t,
    pub y): *mut *mut *mut void (set_pos)(struct nouveau_crtc , int x, int,
    pub update): *mut *mut *mut void (hide)(struct nouveau_crtc , bool,
    pub update): *mut *mut *mut void (show)(struct nouveau_crtc , bool,
    pub cursor: },
    pub depth: c_int,
    pub lut: },
    pub crtc): *mut *mut void (save)(struct drm_crtc,
    pub crtc): *mut *mut void (restore)(struct drm_crtc,
}

extern "C" {
    pub fn nv04_cursor_init(: *mut nouveau_crtc) -> c_int;
}
