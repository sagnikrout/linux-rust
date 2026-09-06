//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gud/gud_internal.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gud_device {
    pub drm: drm_device,
    pub plane: drm_plane,
    pub crtc: drm_crtc,
    pub work: work_struct,
    pub flags: u32,
    pub xrgb8888_emulation_format: *const drm_format_info,
    pub properties: *mut u16,
    pub num_properties: c_uint,
    pub bulk_pipe: c_uint,
    pub bulk_buf: *mut c_void,
    pub bulk_len: usize,
    pub bulk_sgt: sg_table,
    pub compression: u8,
    pub lz4_comp_mem: *mut c_void,
    pub compress_buf: *mut c_void,
    pub stats_length: u64,
    pub stats_actual_length: u64,
    pub stats_num_errors: c_uint,
    pub /: *mut *mut mutex ctrl_lock; / Serialize get/set and status transfers,
    pub /: *mut *mut mutex damage_lock; / Protects the following members:,
    pub fb: *mut drm_framebuffer,
    pub damage: drm_rect,
    pub prev_flush_failed: bool,
    pub shadow_buf: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: drm, gud_device: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn interface_to_usbdev(_arg: to_usb_interface(gdrm->drm.dev)) -> return;
}
extern "C" {
    pub fn gud_usb_get(gdrm: *mut gud_device, request: u8, index: u16, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn gud_usb_set(gdrm: *mut gud_device, request: u8, index: u16, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn gud_usb_get_u8(gdrm: *mut gud_device, request: u8, index: u16, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn gud_usb_set_u8(gdrm: *mut gud_device, request: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn gud_clear_damage(gdrm: *mut gud_device);
}
extern "C" {
    pub fn gud_flush_work(work: *mut work_struct);
}
extern "C" {
    pub fn gud_get_connectors(gdrm: *mut gud_device) -> c_int;
}
// Driver internal fourcc transfer formats
pub const GUD_DRM_FORMAT_R1: c_uint = 0x00000122;
pub const GUD_DRM_FORMAT_XRGB1111: c_uint = 0x03121722;
