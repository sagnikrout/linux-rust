//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/keypad-omap.h
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
// Copyright (C) 2006 Komal Shah <komal_shah802003@yahoo.com>
//

pub const omap_readw(reg): c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_kp_platform_data {
    pub rows: c_int,
    pub cols: c_int,
    pub keymap_data: *const matrix_keymap_data,
    pub rep: bool,
    pub delay: c_ulong,
    pub dbounce: bool,
}

// Group (0..3) -- when multiple keys are pressed, only the
// keys pressed in the same group are considered as pressed. This is
// in order to workaround certain crappy HW designs that produce ghost
// keypresses. Two free bits, not used by neither row/col nor keynum,
// must be available for use as group bits. The below GROUP_SHIFT
// macro definition is based on some prior knowledge of the
// matrix_keypad defined KEY() macro internals.
//
pub const GROUP_SHIFT: c_int = 14;

