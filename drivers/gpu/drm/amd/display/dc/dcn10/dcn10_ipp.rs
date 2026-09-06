//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dcn10/dcn10_ipp.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// Macro flag: #define TO_DCN10_IPP(ipp)\

pub const CURSOR0_CURSOR_CONTROL__CURSOR_2X_MAGNIFY__SHIFT: c_uint = 0x4;
pub const CURSOR0_CURSOR_CONTROL__CURSOR_2X_MAGNIFY_MASK: c_uint = 0x00000010L;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_ipp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_ipp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_ipp_registers {
    pub CURSOR_SETTINS: u32,
    pub CURSOR_SETTINGS: u32,
    pub CNVC_SURFACE_PIXEL_FORMAT: u32,
    pub CURSOR0_CONTROL: u32,
    pub CURSOR0_COLOR0: u32,
    pub CURSOR0_COLOR1: u32,
    pub FORMAT_CONTROL: u32,
    pub CURSOR_SURFACE_ADDRESS_HIGH: u32,
    pub CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR_SIZE: u32,
    pub CURSOR_CONTROL: u32,
    pub CURSOR_POSITION: u32,
    pub CURSOR_HOT_SPOT: u32,
    pub CURSOR_DST_OFFSET: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_ipp {
    pub base: input_pixel_processor,
    pub regs: *const dcn10_ipp_registers,
    pub ipp_shift: *const dcn10_ipp_shift,
    pub ipp_mask: *const dcn10_ipp_mask,
    pub curs_attr: dc_cursor_attributes,
}
