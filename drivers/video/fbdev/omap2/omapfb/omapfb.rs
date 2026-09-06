//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/omapfb.h
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
// linux/drivers/video/omap2/omapfb.h
//
// Copyright (C) 2008 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
// Some code and ideas taken from drivers/video/omap/ driver
// by Imre Deak.
//

// Macro flag: #define DEBUG

// max number of overlays to which a framebuffer data can be direct
pub const OMAPFB_MAX_OVL_PER_FB: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb2_mem_region {
    pub id: c_int,
    pub attrs: c_ulong,
    pub token: *mut c_void,
    pub dma_handle: dma_addr_t,
    pub paddr: u32,
    pub vaddr: *mut void __iomem,
    pub vrfb: vrfb,
    pub size: c_ulong,
    pub /: *mut *mut *mut u8 type; / OMAPFB_PLANE_MEM_,
    pub /: *mut *mut bool alloc; / allocated by the driver,
    pub /: *mut *mut bool map; / kernel mapped by the driver,
    pub map_count: core::sync::atomic::AtomicI32,
    pub lock: rw_semaphore,
    pub lock_count: core::sync::atomic::AtomicI32,
}

// appended to fb_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_info {
    pub id: c_int,
    pub region: *mut omapfb2_mem_region,
    pub num_overlays: c_int,
    pub overlays: [*mut omap_overlay; OMAPFB_MAX_OVL_PER_FB],
    pub fbdev: *mut omapfb2_device,
    pub rotation_type: omap_dss_rotation_type,
    pub rotation: [u8; OMAPFB_MAX_OVL_PER_FB],
    pub mirror: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_display_data {
    pub fbdev: *mut omapfb2_device,
    pub dssdev: *mut omap_dss_device,
    pub bpp_override: u8,
    pub update_mode: omapfb_update_mode,
    pub auto_update_work_enabled: bool,
    pub auto_update_work: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb2_device {
    pub dev: *mut device,
    pub mtx: mutex,
    pub pseudo_palette: [u32; 17],
    pub state: c_int,
    pub num_fbs: unsigned,
    pub fbs: [*mut fb_info; 10],
    pub regions: [omapfb2_mem_region; 10],
    pub num_displays: unsigned,
    pub displays: [omapfb_display_data; 10],
    pub num_overlays: unsigned,
    pub overlays: [*mut omap_overlay; 10],
    pub num_managers: unsigned,
    pub managers: [*mut omap_overlay_manager; 10],
    pub auto_update_wq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapfb_colormode {
    pub dssmode: omap_color_mode,
    pub bits_per_pixel: u32,
    pub nonstd: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
}

extern "C" {
    pub fn set_fb_fix(fbi: *mut fb_info);
}
extern "C" {
    pub fn check_fb_var(fbi: *mut fb_info, var: *mut fb_var_screeninfo) -> c_int;
}
extern "C" {
    pub fn omapfb_realloc_fbmem(fbi: *mut fb_info, size: c_ulong, type: c_int) -> c_int;
}
extern "C" {
    pub fn omapfb_apply_changes(fbi: *mut fb_info, init: c_int) -> c_int;
}
extern "C" {
    pub fn omapfb_create_sysfs(fbdev: *mut omapfb2_device) -> c_int;
}
extern "C" {
    pub fn omapfb_remove_sysfs(fbdev: *mut omapfb2_device);
}
extern "C" {
    pub fn omapfb_ioctl(fbi: *mut fb_info, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn omapfb_get_update_mode(fbi: *mut fb_info, mode: *mut omapfb_update_mode) -> c_int;
}
extern "C" {
    pub fn omapfb_set_update_mode(fbi: *mut fb_info, mode: omapfb_update_mode) -> c_int;
}
// find the display connected to this fb, if any
// XXX: returns the display connected to first attached overlay
// This should never happen
