//! Automatically rewritten from C Header to Rust Module
//! Source: lib/fonts/font.h
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


// SPDX-License-Identifier: GPL-2.0

//
// Font data
//
pub const FONT_EXTRA_WORDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct font_data {
    pub extra: [c_uint; FONT_EXTRA_WORDS],
    pub data: [c_uchar; ],
    pub __packed: },
//
// Built-in fonts
//
pub const VGA8x8_IDX: c_int = 0;
pub const VGA8x16_IDX: c_int = 1;
pub const PEARL8x8_IDX: c_int = 2;
pub const VGA6x11_IDX: c_int = 3;
pub const FONT7x14_IDX: c_int = 4;
pub const FONT10x18_IDX: c_int = 5;
pub const SUN8x16_IDX: c_int = 6;
pub const SUN12x22_IDX: c_int = 7;
pub const ACORN8x8_IDX: c_int = 8;
pub const MINI4x6_IDX: c_int = 9;
pub const FONT6x10_IDX: c_int = 10;
pub const TER16x32_IDX: c_int = 11;
pub const FONT6x8_IDX: c_int = 12;
pub const TER10x18_IDX: c_int = 13;
