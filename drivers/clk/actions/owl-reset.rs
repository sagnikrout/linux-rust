//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/actions/owl-reset.h
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
// Actions Semi Owl SoCs Reset Management Unit driver
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_reset_map {
    pub reg: u32,
    pub bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_reset {
    pub rcdev: reset_controller_dev,
    pub reset_map: *const owl_reset_map,
    pub regmap: *mut regmap,
}

extern "C" {
    pub fn container_of(_arg: rcdev, owl_reset: struct, _arg: rcdev) -> return;
}
