//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/mediatek/pinctrl-mtmips.h
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
// Copyright (C) 2012 John Crispin <john@phrozen.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtmips_pmx_func {
    pub name: *const c_char,
    pub value: c_char,
    pub pin_first: c_int,
    pub pin_count: c_int,
    pub pins: *mut c_int,
    pub groups: *mut c_int,
    pub group_count: c_int,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtmips_pmx_group {
    pub name: *const c_char,
    pub enabled: c_int,
    pub shift: u32,
    pub mask: c_char,
    pub gpio: c_char,
    pub func: *mut mtmips_pmx_func,
    pub func_count: c_int,
}
