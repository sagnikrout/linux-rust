//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-queue.h
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

pub const SLICED_VBI_PIO: c_int = 0;
// ivtv_buffer utility functions
extern "C" {
    pub fn ivtv_buf_copy_from_user(s: *mut ivtv_stream, buf: *mut ivtv_buffer, src: *const char __user, copybytes: c_int) -> c_int;
}
extern "C" {
    pub fn ivtv_buf_swap(buf: *mut ivtv_buffer);
}
// ivtv_queue utility functions
extern "C" {
    pub fn ivtv_queue_init(q: *mut ivtv_queue);
}
extern "C" {
    pub fn ivtv_enqueue(s: *mut ivtv_stream, buf: *mut ivtv_buffer, q: *mut ivtv_queue);
}
extern "C" {
    pub fn ivtv_flush_queues(s: *mut ivtv_stream);
}
// ivtv_stream utility functions
extern "C" {
    pub fn ivtv_stream_alloc(s: *mut ivtv_stream) -> c_int;
}
extern "C" {
    pub fn ivtv_stream_free(s: *mut ivtv_stream);
}
