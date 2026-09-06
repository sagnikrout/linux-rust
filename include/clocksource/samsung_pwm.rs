//! Automatically rewritten from C Header to Rust Module
//! Source: include/clocksource/samsung_pwm.h
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
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//

pub const SAMSUNG_PWM_NUM: c_int = 5;
//
// Following declaration must be in an ifdef due to this symbol being static
// in pwm-samsung driver if the clocksource driver is not compiled in and the
// spinlock is not shared between both drivers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_pwm_variant {
    pub bits: u8,
    pub div_base: u8,
    pub tclk_mask: u8,
    pub output_mask: u8,
    pub has_tint_cstat: bool,
}
