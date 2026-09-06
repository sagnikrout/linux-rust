//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/udl/udl_drv.h
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
// Copyright (C) 2012 Red Hat
//
// based in parts on udlfb.c:
// Copyright (C) 2009 Roberto De Ioris <roberto@unbit.it>
// Copyright (C) 2009 Jaya Kumar <jayakumar.lkml@gmail.com>
// Copyright (C) 2009 Bernie Thompson <bernie@plugable.com>
//

pub const DRIVER_MAJOR: c_int = 0;
pub const DRIVER_MINOR: c_int = 0;
pub const DRIVER_PATCHLEVEL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_node {
    pub entry: list_head,
    pub dev: *mut udl_device,
    pub urb: *mut urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_list {
    pub list: list_head,
    pub lock: spinlock_t,
    pub sleep: wait_queue_head_t,
    pub available: c_int,
    pub count: c_int,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udl_device {
    pub drm: drm_device,
    pub sku_pixel_limit: c_ulong,
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub urbs: urb_list,
}

extern "C" {
    pub fn interface_to_usbdev(_arg: to_usb_interface(udl->drm.dev)) -> return;
}
// modeset
extern "C" {
    pub fn udl_modeset_init(udl: *mut udl_device) -> c_int;
}
extern "C" {
    pub fn udl_submit_urb(udl: *mut udl_device, urb: *mut urb, len: usize) -> c_int;
}
extern "C" {
    pub fn udl_sync_pending_urbs(udl: *mut udl_device);
}
extern "C" {
    pub fn udl_urb_completion(urb: *mut urb);
}
extern "C" {
    pub fn udl_init(udl: *mut udl_device) -> c_int;
}
extern "C" {
    pub fn udl_drop_usb(udl: *mut udl_device) -> c_int;
}
extern "C" {
    pub fn udl_select_std_channel(udl: *mut udl_device) -> c_int;
}
