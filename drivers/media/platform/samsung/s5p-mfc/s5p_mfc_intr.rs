//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/s5p_mfc_intr.h
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
// drivers/media/platform/samsung/mfc5/s5p_mfc_intr.h
//
// Header file for Samsung MFC (Multi Function Codec - FIMV) driver
// It contains waiting functions declarations.
//
// Kamil Debski, Copyright (C) 2011 Samsung Electronics
// http://www.samsung.com
//

extern "C" {
    pub fn s5p_mfc_wait_for_done_dev(dev: *mut s5p_mfc_dev, command: c_int) -> c_int;
}
extern "C" {
    pub fn s5p_mfc_clean_ctx_int_flags(ctx: *mut s5p_mfc_ctx);
}
extern "C" {
    pub fn s5p_mfc_clean_dev_int_flags(dev: *mut s5p_mfc_dev);
}
