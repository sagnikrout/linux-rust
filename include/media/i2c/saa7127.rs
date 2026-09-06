//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/saa7127.h
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
// Enumeration for the supported input types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7127_input_type {
    SAA7127_INPUT_TYPE_NORMAL,
    SAA7127_INPUT_TYPE_TEST_IMAGE
}

// Enumeration for the supported output signal types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7127_output_type {
    SAA7127_OUTPUT_TYPE_BOTH,
    SAA7127_OUTPUT_TYPE_COMPOSITE,
    SAA7127_OUTPUT_TYPE_SVIDEO,
    SAA7127_OUTPUT_TYPE_RGB,
    SAA7127_OUTPUT_TYPE_YUV_C,
    SAA7127_OUTPUT_TYPE_YUV_V
}
