//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/line6/midibuf.h
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
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
//
pub const LINE6_MIDIBUF_READ_TX: c_int = 0;
pub const LINE6_MIDIBUF_READ_RX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct midi_buffer {
    pub buf: *mut c_uchar,
    pub size: c_int,
    pub split: c_int,
    pub pos_write: int pos_read,,
    pub full: c_int,
    pub command_prev: c_int,
}

extern "C" {
    pub fn line6_midibuf_bytes_used(mb: *mut midi_buffer) -> c_int;
}
extern "C" {
    pub fn line6_midibuf_bytes_free(mb: *mut midi_buffer) -> c_int;
}
extern "C" {
    pub fn line6_midibuf_destroy(mb: *mut midi_buffer);
}
extern "C" {
    pub fn line6_midibuf_ignore(mb: *mut midi_buffer, length: c_int) -> c_int;
}
extern "C" {
    pub fn line6_midibuf_init(mb: *mut midi_buffer, size: c_int, split: c_int) -> c_int;
}
extern "C" {
    pub fn line6_midibuf_reset(mb: *mut midi_buffer);
}
