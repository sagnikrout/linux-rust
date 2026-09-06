//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-fileops.h
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
// Testing/Debugging
extern "C" {
    pub fn ivtv_v4l2_open(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn ivtv_v4l2_close(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn ivtv_v4l2_enc_poll(filp: *mut file, wait: *mut *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn ivtv_v4l2_dec_poll(filp: *mut file, wait: *mut *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn ivtv_start_capture(id: *mut ivtv_open_id) -> c_int;
}
extern "C" {
    pub fn ivtv_stop_capture(id: *mut ivtv_open_id, gop_end: c_int);
}
extern "C" {
    pub fn ivtv_start_decoding(id: *mut ivtv_open_id, speed: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_mute(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_unmute(itv: *mut ivtv);
}
// Utilities
// Shared with ivtv-alsa module
extern "C" {
    pub fn ivtv_claim_stream(id: *mut ivtv_open_id, type: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_release_stream(s: *mut ivtv_stream);
}
