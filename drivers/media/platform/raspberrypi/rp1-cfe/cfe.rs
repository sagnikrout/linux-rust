//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/raspberrypi/rp1-cfe/cfe.h
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
// RP1 CFE Driver
//
// Copyright (c) 2021-2024 Raspberry Pi Ltd.
// Copyright (c) 2023-2024 Ideas on Board Oy
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cfe_remap_types {
    CFE_REMAP_16BIT,
    CFE_REMAP_COMPRESSED,
    CFE_NUM_REMAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfe_fmt {
    pub fourcc: u32,
    pub code: u32,
    pub depth: u8,
    pub csi_dt: u8,
    pub remap: [u32; CFE_NUM_REMAP],
    pub flags: u32,
}

extern "C" {
    pub fn cfe_find_16bit_code(code: u32) -> u32;
}
extern "C" {
    pub fn cfe_find_compressed_code(code: u32) -> u32;
}
