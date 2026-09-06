//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_ipp.h
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

// Macro flag: #define TO_DCE_IPP(ipp)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_ipp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_ipp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_ipp_registers {
    pub CUR_UPDATE: u32,
    pub CUR_CONTROL: u32,
    pub CUR_POSITION: u32,
    pub CUR_HOT_SPOT: u32,
    pub CUR_COLOR1: u32,
    pub CUR_COLOR2: u32,
    pub CUR_SIZE: u32,
    pub CUR_SURFACE_ADDRESS_HIGH: u32,
    pub CUR_SURFACE_ADDRESS: u32,
    pub PRESCALE_GRPH_CONTROL: u32,
    pub PRESCALE_VALUES_GRPH_R: u32,
    pub PRESCALE_VALUES_GRPH_G: u32,
    pub PRESCALE_VALUES_GRPH_B: u32,
    pub INPUT_GAMMA_CONTROL: u32,
    pub DCFE_MEM_PWR_CTRL: u32,
    pub DC_LUT_WRITE_EN_MASK: u32,
    pub DC_LUT_RW_MODE: u32,
    pub DC_LUT_CONTROL: u32,
    pub DC_LUT_RW_INDEX: u32,
    pub DC_LUT_SEQ_COLOR: u32,
    pub DEGAMMA_CONTROL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_ipp {
    pub base: input_pixel_processor,
    pub regs: *const dce_ipp_registers,
    pub ipp_shift: *const dce_ipp_shift,
    pub ipp_mask: *const dce_ipp_mask,
}

extern "C" {
    pub fn dce_ipp_destroy(ipp: *mut input_pixel_processor);
}
