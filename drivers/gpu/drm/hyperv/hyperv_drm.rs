//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hyperv/hyperv_drm.h
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
// Copyright 2021 Microsoft
//
pub const VMBUS_MAX_PACKET_SIZE: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_drm_device {
// drm
    pub dev: drm_device,
    pub plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
// mode
    pub screen_width_max: u32,
    pub screen_height_max: u32,
    pub preferred_width: u32,
    pub preferred_height: u32,
    pub screen_depth: u32,
// hw
    pub mem: *mut resource,
    pub vram: *mut void __iomem,
    pub fb_base: c_ulong,
    pub fb_size: c_ulong,
    pub wait: completion,
    pub synthvid_version: u32,
    pub mmio_megabytes: u32,
    pub dirt_needed: bool,
    pub init_buf: [u8; VMBUS_MAX_PACKET_SIZE],
    pub recv_buf: [u8; VMBUS_MAX_PACKET_SIZE],
    pub hdev: *mut hv_device,
}

// hyperv_drm_modeset
extern "C" {
    pub fn hv_drm_mode_config_init(hv: *mut hv_drm_device) -> c_int;
}
// hyperv_drm_proto
extern "C" {
    pub fn hv_drm_update_vram_location(hdev: *mut hv_device, vram_pp: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn hv_drm_hide_hw_ptr(hdev: *mut hv_device) -> c_int;
}
extern "C" {
    pub fn hv_drm_update_dirt(hdev: *mut hv_device, rect: *mut drm_rect) -> c_int;
}
extern "C" {
    pub fn hv_drm_connect_vsp(hdev: *mut hv_device) -> c_int;
}
