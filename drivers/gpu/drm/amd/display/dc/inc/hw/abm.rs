//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/abm.h
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


// Copyright 2012-15 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm {
    pub ctx: *mut dc_context,
    pub funcs: *const abm_funcs,
    pub dmcu_is_running: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm_funcs {
    pub user_level): *mut *mut *mut void (abm_init)(struct abm abm, uint32_t back_light, uint32_t,
    pub abm_level): *mut *mut *mut bool (set_abm_level)(struct abm abm, unsigned int,
    pub panel_inst): *mut *mut *mut bool (set_abm_immediate_disable)(struct abm abm, unsigned int,
    pub panel_inst): *mut *mut *mut bool (set_pipe)(struct abm abm, unsigned int controller_id, unsigned int,
// backlight_pwm_u16_16 is unsigned 32 bit,
// 16 bit integer + 16 fractional, where 1.0 is max backlight value.
//
    pub panel_inst): c_uint,
    pub abm): *mut *mut unsigned int (get_current_backlight)(struct abm,
    pub abm): *mut *mut unsigned int (get_target_backlight)(struct abm,
    pub inst): c_uint,
    pub otg_inst): *mut *mut *mut bool (set_abm_pause)(struct abm abm, bool pause, unsigned int panel_inst, unsigned int,
    pub pData): *mut abm_save_restore,
    pub pwrseq_inst): c_uint,
    pub panel_inst): c_uint,
}
