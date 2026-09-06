//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/vbox_drv.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2013-2017 Oracle Corporation
// This file is based on ast_drv.h
// Copyright 2012 Red Hat Inc.
// Authors: Dave Airlie <airlied@redhat.com>
// Michael Thayer <michael.thayer@oracle.com,
// Hans de Goede <hdegoede@redhat.com>
//

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 0;
pub const DRIVER_PATCHLEVEL: c_int = 0;
pub const VBOX_MAX_CURSOR_WIDTH: c_int = 64;
pub const VBOX_MAX_CURSOR_HEIGHT: c_int = 64;

pub const VBOX_MAX_SCREENS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbox_private {
// Must be first; or we must define our own release callback
    pub ddev: drm_device,
    pub guest_heap: *mut u8 __iomem,
    pub vbva_buffers: *mut u8 __iomem,
    pub guest_pool: *mut gen_pool,
    pub vbva_info: *mut vbva_buf_ctx,
    pub any_pitch: bool,
    pub num_crtcs: u32,
// Amount of available VRAM, including space used for buffers.
    pub full_vram_size: u32,
// Amount of available VRAM, not including space used for buffers.
    pub available_vram_size: u32,
// Array of structures for receiving mode hints.
    pub last_mode_hints: *mut vbva_modehint,
    pub fb_mtrr: c_int,
    pub /: *mut *mut mutex hw_mutex; / protects modeset and accel/vbva accesses,
    pub hotplug_work: work_struct,
    pub input_mapping_width: u32,
    pub input_mapping_height: u32,
//
// Is user-space using an X.Org-style layout of one large frame-buffer
// encompassing all screen ones or is the fbdev console active?
//
    pub single_framebuffer: bool,
    pub cursor_data: [u8; CURSOR_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbox_connector {
    pub base: drm_connector,
    pub name: [c_char; 32],
    pub vbox_crtc: *mut vbox_crtc,
    pub width: u32,
    pub height: u32,
    pub disconnected: bool,
    pub mode_hint: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbox_crtc {
    pub base: drm_crtc,
    pub disconnected: bool,
    pub crtc_id: c_uint,
    pub fb_offset: u32,
    pub cursor_enabled: bool,
    pub x_hint: u32,
    pub y_hint: u32,
//
// When setting a mode we not only pass the mode to the hypervisor,
// but also information on how to map / translate input coordinates
// for the emulated USB tablet.  This input-mapping may change when
// the mode on *another* crtc changes.
//
// This means that sometimes we must do a modeset on other crtc-s then
// the one being changed to update the input-mapping. Including crtc-s
// which may be disabled inside the guest (shown as a black window
// on the host unless closed by the user).
//
// With atomic modesetting the mode-info of disabled crtcs gets zeroed
// yet we need it when updating the input-map to avoid resizing the
// window as a side effect of a mode_set on another crtc. Therefor we
// cache the info of the last mode below.
//
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbox_encoder {
    pub base: drm_encoder,
}

extern "C" {
    pub fn vbox_check_supported(id: u16) -> bool;
}
extern "C" {
    pub fn vbox_hw_init(vbox: *mut vbox_private) -> c_int;
}
extern "C" {
    pub fn vbox_hw_fini(vbox: *mut vbox_private);
}
extern "C" {
    pub fn vbox_mode_init(vbox: *mut vbox_private) -> c_int;
}
extern "C" {
    pub fn vbox_mode_fini(vbox: *mut vbox_private);
}
extern "C" {
    pub fn vbox_report_caps(vbox: *mut vbox_private);
}
extern "C" {
    pub fn vbox_mm_init(vbox: *mut vbox_private) -> c_int;
}
// vbox_irq.c
extern "C" {
    pub fn vbox_irq_init(vbox: *mut vbox_private) -> c_int;
}
extern "C" {
    pub fn vbox_irq_fini(vbox: *mut vbox_private);
}
extern "C" {
    pub fn vbox_report_hotplug(vbox: *mut vbox_private);
}
// vbox_hgsmi.c
extern "C" {
    pub fn hgsmi_buffer_free(guest_pool: *mut gen_pool, buf: *mut c_void);
}
extern "C" {
    pub fn hgsmi_buffer_submit(guest_pool: *mut gen_pool, buf: *mut c_void) -> c_int;
}
