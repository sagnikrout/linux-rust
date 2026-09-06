//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-ioctl.h
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
extern "C" {
    pub fn ivtv_service2vbi(type: c_int) -> u16;
}
extern "C" {
    pub fn ivtv_expand_service_set(fmt: *mut v4l2_sliced_vbi_format, is_pal: c_int);
}
extern "C" {
    pub fn ivtv_get_service_set(fmt: *mut v4l2_sliced_vbi_format) -> u16;
}
extern "C" {
    pub fn ivtv_set_osd_alpha(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_set_speed(itv: *mut ivtv, speed: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_set_funcs(vdev: *mut video_device);
}
extern "C" {
    pub fn ivtv_s_std_enc(itv: *mut ivtv, std: v4l2_std_id);
}
extern "C" {
    pub fn ivtv_s_std_dec(itv: *mut ivtv, std: v4l2_std_id);
}
extern "C" {
    pub fn ivtv_do_s_frequency(s: *mut ivtv_stream, vf: *const v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn ivtv_do_s_input(itv: *mut ivtv, inp: c_uint) -> c_int;
}
