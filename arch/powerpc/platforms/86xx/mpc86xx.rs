//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/86xx/mpc86xx.h
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
// Copyright 2006 Freescale Semiconductor Inc.
//
// Declaration for the various functions exported by the
// mpc86xx_* files. Mostly for use by mpc86xx_setup().
//
extern "C" {
    pub fn mpc86xx_smp_init();
}
extern "C" {
    pub fn mpc86xx_init_irq();
}
extern "C" {
    pub fn mpc86xx_time_init() -> c_long;
}
extern "C" {
    pub fn mpc86xx_common_publish_devices() -> c_int;
}
