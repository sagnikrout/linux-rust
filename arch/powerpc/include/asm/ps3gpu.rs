//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ps3gpu.h
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
// PS3 GPU declarations.
//
// Copyright 2009 Sony Corporation
//

pub const L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_SYNC: c_uint = 0x101;
pub const L1GPU_CONTEXT_ATTRIBUTE_DISPLAY_FLIP: c_uint = 0x102;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_SETUP: c_uint = 0x600;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_BLIT: c_uint = 0x601;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_BLIT_SYNC: c_uint = 0x602;
pub const L1GPU_CONTEXT_ATTRIBUTE_FB_CLOSE: c_uint = 0x603;

pub const L1GPU_DISPLAY_SYNC_HSYNC: c_int = 1;
pub const L1GPU_DISPLAY_SYNC_VSYNC: c_int = 2;
// mutex synchronizing GPU accesses and video mode changes
