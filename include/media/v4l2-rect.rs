//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-rect.h
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
// v4l2-rect.h - v4l2_rect helper functions
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// v4l2_rect_set_size_to() - copy the width/height values.
// @r: rect whose width and height fields will be set
// @size: rect containing the width and height fields you need.
//
// v4l2_rect_set_min_size() - width and height of r should be >= min_size.
// @r: rect whose width and height will be modified
// @min_size: rect containing the minimal width and height
//
// v4l2_rect_set_max_size() - width and height of r should be <= max_size
// @r: rect whose width and height will be modified
// @max_size: rect containing the maximum width and height
//
// v4l2_rect_map_inside()- r should be inside boundary.
// @r: rect that will be modified
// @boundary: rect containing the boundary for @r
//
// v4l2_rect_same_size() - return true if r1 has the same size as r2
// @r1: rectangle.
// @r2: rectangle.
//
// Return true if both rectangles have the same size.
//
// v4l2_rect_same_position() - return true if r1 has the same position as r2
// @r1: rectangle.
// @r2: rectangle.
//
// Return true if both rectangles have the same position
//
// v4l2_rect_equal() - return true if r1 equals r2
// @r1: rectangle.
// @r2: rectangle.
//
// Return true if both rectangles have the same size and position.
//
extern "C" {
    pub fn v4l2_rect_same_size(_arg: r1, v4l2_rect_same_position(r1: r2) &&, _arg: r2) -> return;
}
//
// v4l2_rect_intersect() - calculate the intersection of two rects.
// @r: intersection of @r1 and @r2.
// @r1: rectangle.
// @r2: rectangle.
//
// v4l2_rect_scale() - scale rect r by to/from
// @r: rect to be scaled.
// @from: from rectangle.
// @to: to rectangle.
//
// This scales rectangle @r horizontally by @to->width / @from->width and
// vertically by @to->height / @from->height.
//
// Typically @r is a rectangle inside @from and you want the rectangle as
// it would appear after scaling @from to @to. So the resulting @r will
// be the scaled rectangle inside @to.
//
// v4l2_rect_overlap() - do r1 and r2 overlap?
// @r1: rectangle.
// @r2: rectangle.
//
// Returns true if @r1 and @r2 overlap.
//
// IF the left side of r1 is to the right of the right side of r2 OR
// the left side of r2 is to the right of the right side of r1 THEN
// they do not overlap.
//
// IF the top side of r1 is below the bottom of r2 OR
// the top side of r2 is below the bottom of r1 THEN
// they do not overlap.
//
// v4l2_rect_enclosed() - is r1 enclosed in r2?
// @r1: rectangle.
// @r2: rectangle.
//
// Returns true if @r1 is enclosed in @r2.
//
