//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_rect.h
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


//
// Copyright (C) 2011-2013 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// DOC: rect utils
//
// Utility functions to help manage rectangular areas for
// clipping, scaling, etc. calculations.
//
// struct drm_rect - two dimensional rectangle
// @x1: horizontal starting coordinate (inclusive)
// @x2: horizontal ending coordinate (exclusive)
// @y1: vertical starting coordinate (inclusive)
// @y2: vertical ending coordinate (exclusive)
//
// Note that this must match the layout of struct drm_mode_rect or the damage
// helpers like drm_atomic_helper_damage_iter_init() break.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_rect {
    pub y2: int x1, y1, x2,,
}

//
// DRM_RECT_INIT - initialize a rectangle from x/y/w/h
// @x: x coordinate
// @y: y coordinate
// @w: width
// @h: height
//
// RETURNS:
// A new rectangle of the specified size.
//

//
// DRM_RECT_FMT - printf string for &struct drm_rect
//

//
// DRM_RECT_ARG - printf arguments for &struct drm_rect
// @r: rectangle struct
//

//
// DRM_RECT_FP_FMT - printf string for &struct drm_rect in 16.16 fixed point
//

//
// DRM_RECT_FP_ARG - printf arguments for &struct drm_rect in 16.16 fixed point
// @r: rectangle struct
//
// This is useful for e.g. printing plane source rectangles, which are in 16.16
// fixed point.
//

//
// drm_rect_init - initialize the rectangle from x/y/w/h
// @r: rectangle
// @x: x coordinate
// @y: y coordinate
// @width: width
// @height: height
//
// drm_rect_adjust_size - adjust the size of the rectangle
// @r: rectangle to be adjusted
// @dw: horizontal adjustment
// @dh: vertical adjustment
//
// Change the size of rectangle @r by @dw in the horizontal direction,
// and by @dh in the vertical direction, while keeping the center
// of @r stationary.
//
// Positive @dw and @dh increase the size, negative values decrease it.
//
// drm_rect_translate - translate the rectangle
// @r: rectangle to be translated
// @dx: horizontal translation
// @dy: vertical translation
//
// Move rectangle @r by @dx in the horizontal direction,
// and by @dy in the vertical direction.
//
// drm_rect_translate_to - translate the rectangle to an absolute position
// @r: rectangle to be translated
// @x: horizontal position
// @y: vertical position
//
// Move rectangle @r to @x in the horizontal direction,
// and to @y in the vertical direction.
//
// drm_rect_downscale - downscale a rectangle
// @r: rectangle to be downscaled
// @horz: horizontal downscale factor
// @vert: vertical downscale factor
//
// Divide the coordinates of rectangle @r by @horz and @vert.
//
// drm_rect_width - determine the rectangle width
// @r: rectangle whose width is returned
//
// RETURNS:
// The width of the rectangle.
//
// drm_rect_height - determine the rectangle height
// @r: rectangle whose height is returned
//
// RETURNS:
// The height of the rectangle.
//
// drm_rect_visible - determine if the rectangle is visible
// @r: rectangle whose visibility is returned
//
// RETURNS:
// %true if the rectangle is visible, %false otherwise.
//
// drm_rect_equals - determine if two rectangles are equal
// @r1: first rectangle
// @r2: second rectangle
//
// RETURNS:
// %true if the rectangles are equal, %false otherwise.
//
// drm_rect_fp_to_int - Convert a rect in 16.16 fixed point form to int form.
// @dst: rect to be stored the converted value
// @src: rect in 16.16 fixed point form
//
// drm_rect_overlap - Check if two rectangles overlap
// @a: first rectangle
// @b: second rectangle
//
// RETURNS:
// %true if the rectangles overlap, %false otherwise.
//
extern "C" {
    pub fn drm_rect_intersect(r: *mut drm_rect, clip: *const drm_rect) -> bool;
}
