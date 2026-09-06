//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_connector.h
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
pub struct nouveau_backlight {
    pub dev: *mut backlight_device,
    pub edp_info: drm_edp_backlight_info,
    pub 1: bool uses_dpcd :,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_conn_atom {
    pub state: drm_connector_state,
// The enum values specifically defined here match nv50/gf119
// hw values, and the code relies on this.
//
    pub mode: },
    pub depth: },
    pub dither: },
    pub /: *mut *mut *mut int mode; / DRM_MODE_SCALE_,
    pub mode: },
    pub hborder: u32,
    pub vborder: u32,
    pub underscan: },
    pub full: bool,
    pub scaler: },
    pub color_vibrance: c_int,
    pub vibrant_hue: c_int,
    pub procamp: },
    pub dither:1: bool,
    pub scaler:1: bool,
    pub procamp:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_connector {
    pub base: drm_connector,
    pub type: dcb_connector_type,
    pub index: u8,
    pub conn: nvif_conn,
    pub hpd_pending: u64,
    pub hpd: nvif_event,
    pub irq: nvif_event,
    pub irq_work: work_struct,
    pub aux: drm_dp_aux,
// The fixed DP encoder for this connector, if there is one
    pub dp_encoder: *mut nouveau_encoder,
    pub dithering_mode: c_int,
    pub scaling_mode: c_int,
    pub detected_encoder: *mut nouveau_encoder,
    pub edid: *mut edid,
    pub native_mode: *mut drm_display_mode,

    pub backlight: *mut nouveau_backlight,

//
// Our connector property code expects a nouveau_conn_atom struct
// even on pre-nv50 where we do not support atomic. This embedded
// version gets used in the non atomic modeset case.
//
    pub properties_state: nouveau_conn_atom,
}

extern "C" {
    pub fn container_of(_arg: con, nouveau_connector: struct, _arg: base) -> return;
}

extern "C" {
    pub fn nouveau_connector_hpd(: *mut nouveau_connector, bits: u64);
}
extern "C" {
    pub fn nouveau_conn_attach_properties(: *mut drm_connector);
}
extern "C" {
    pub fn nouveau_conn_reset(: *mut drm_connector);
}

extern "C" {
    pub fn nouveau_backlight_init(: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn nouveau_backlight_fini(: *mut drm_connector);
}
extern "C" {
    pub fn nouveau_backlight_ctor();
}
extern "C" {
    pub fn nouveau_backlight_dtor();
}

