//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpic_timer.h
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
// arch/powerpc/include/asm/mpic_timer.h
//
// Header file for Mpic Global Timer
//
// Copyright 2013 Freescale Semiconductor, Inc.
//
// Author: Wang Dongsheng <Dongsheng.Wang@freescale.com>
// Li Yang <leoli@freescale.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpic_timer {
    pub dev: *mut c_void,
    pub cascade_handle: *mut cascade_priv,
    pub num: c_uint,
    pub irq: c_uint,
}

extern "C" {
    pub fn mpic_start_timer(handle: *mut mpic_timer);
}
extern "C" {
    pub fn mpic_stop_timer(handle: *mut mpic_timer);
}
extern "C" {
    pub fn mpic_get_remain_time(handle: *mut mpic_timer, time: *mut time64_t);
}
extern "C" {
    pub fn mpic_free_timer(handle: *mut mpic_timer);
}

