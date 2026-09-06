//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/panel_cntl.h
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
// panel_cntl.h
//
// Created on: Oct 6, 2015
// Author: yonsun
//

pub const MAX_BACKLIGHT_LEVEL: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_cntl_backlight_registers {
    pub BL_PWM_CNTL: c_uint,
    pub BL_PWM_CNTL2: c_uint,
    pub BL_PWM_PERIOD_CNTL: c_uint,
    pub LVTMA_PWRSEQ_REF_DIV_BL_PWM_REF_DIV: c_uint,
    pub PANEL_PWRSEQ_REF_DIV2: c_uint,
    pub USER_LEVEL: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_cntl_funcs {
    pub panel_cntl): *mut *mut void (destroy)(struct panel_cntl,
    pub panel_cntl): *mut *mut uint32_t (hw_init)(struct panel_cntl,
    pub panel_cntl): *mut *mut bool (is_panel_backlight_on)(struct panel_cntl,
    pub panel_cntl): *mut *mut bool (is_panel_powered_on)(struct panel_cntl,
    pub panel_cntl): *mut *mut void (store_backlight_level)(struct panel_cntl,
    pub backlight_pwm_u16_16): u32,
    pub panel_cntl): *mut *mut uint32_t (get_current_backlight)(struct panel_cntl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_cntl_init_data {
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub eng_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_cntl {
    pub funcs: *const panel_cntl_funcs,
    pub ctx: *mut dc_context,
    pub inst: u32,
    pub pwrseq_inst: u32,
// registers setting needs to be saved and restored at InitBacklight
    pub stored_backlight_registers: panel_cntl_backlight_registers,
}
