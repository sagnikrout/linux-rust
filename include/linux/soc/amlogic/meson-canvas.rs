//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/amlogic/meson-canvas.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2018 BayLibre, SAS
//

pub const MESON_CANVAS_WRAP_NONE: c_uint = 0x00;
pub const MESON_CANVAS_WRAP_X: c_uint = 0x01;
pub const MESON_CANVAS_WRAP_Y: c_uint = 0x02;
pub const MESON_CANVAS_BLKMODE_LINEAR: c_uint = 0x00;
pub const MESON_CANVAS_BLKMODE_32x32: c_uint = 0x01;
pub const MESON_CANVAS_BLKMODE_64x64: c_uint = 0x02;
pub const MESON_CANVAS_ENDIAN_SWAP16: c_uint = 0x1;
pub const MESON_CANVAS_ENDIAN_SWAP32: c_uint = 0x3;
pub const MESON_CANVAS_ENDIAN_SWAP64: c_uint = 0x7;
pub const MESON_CANVAS_ENDIAN_SWAP128: c_uint = 0xf;
//
// meson_canvas_get() - get a canvas provider instance
//
// @dev: consumer device pointer
//
// meson_canvas_alloc() - take ownership of a canvas
//
// @canvas: canvas provider instance retrieved from meson_canvas_get()
// @canvas_index: will be filled with the canvas ID
//
extern "C" {
    pub fn meson_canvas_alloc(canvas: *mut meson_canvas, canvas_index: *mut u8) -> c_int;
}
//
// meson_canvas_free() - remove ownership from a canvas
//
// @canvas: canvas provider instance retrieved from meson_canvas_get()
// @canvas_index: canvas ID that was obtained via meson_canvas_alloc()
//
extern "C" {
    pub fn meson_canvas_free(canvas: *mut meson_canvas, canvas_index: u8) -> c_int;
}
//
// meson_canvas_config() - configure a canvas
//
// @canvas: canvas provider instance retrieved from meson_canvas_get()
// @canvas_index: canvas ID that was obtained via meson_canvas_alloc()
// @addr: physical address to the pixel buffer
// @stride: width of the buffer
// @height: height of the buffer
// @wrap: undocumented
// @blkmode: block mode (linear, 32x32, 64x64)
// @endian: byte swapping (swap16, swap32, swap64, swap128)
//
