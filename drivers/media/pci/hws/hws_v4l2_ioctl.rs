//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/hws/hws_v4l2_ioctl.h
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

extern "C" {
    pub fn hws_vidioc_querycap(file: *mut file, priv: *mut c_void, cap: *mut v4l2_capability) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_enum_fmt_vid_cap(file: *mut file, priv_fh: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_g_fmt_vid_cap(file: *mut file, fh: *mut c_void, fmt: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_try_fmt_vid_cap(file: *mut file, fh: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_g_std(file: *mut file, priv: *mut c_void, tvnorms: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_s_std(file: *mut file, priv: *mut c_void, tvnorms: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_g_parm(file: *mut file, fh: *mut c_void, setfps: *mut v4l2_streamparm) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_enum_input(file: *mut file, priv: *mut c_void, i: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_g_input(file: *mut file, priv: *mut c_void, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_s_input(file: *mut file, priv: *mut c_void, i: c_uint) -> c_int;
}
extern "C" {
    pub fn hws_vidioc_s_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
