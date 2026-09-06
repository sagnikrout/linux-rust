//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-streams.h
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
    pub fn ivtv_streams_setup(itv: *mut ivtv) -> c_int;
}
extern "C" {
    pub fn ivtv_streams_register(itv: *mut ivtv) -> c_int;
}
extern "C" {
    pub fn ivtv_streams_cleanup(itv: *mut ivtv);
}
// Capture related
extern "C" {
    pub fn ivtv_start_v4l2_encode_stream(s: *mut ivtv_stream) -> c_int;
}
extern "C" {
    pub fn ivtv_stop_v4l2_encode_stream(s: *mut ivtv_stream, gop_end: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_start_v4l2_decode_stream(s: *mut ivtv_stream, gop_offset: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_stop_v4l2_decode_stream(s: *mut ivtv_stream, flags: c_int, pts: u64) -> c_int;
}
extern "C" {
    pub fn ivtv_stop_all_captures(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_passthrough_mode(itv: *mut ivtv, enable: c_int) -> c_int;
}
