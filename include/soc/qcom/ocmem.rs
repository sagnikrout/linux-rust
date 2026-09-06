//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/qcom/ocmem.h
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
// The On Chip Memory (OCMEM) allocator allows various clients to allocate
// memory from OCMEM based on performance, latency and power requirements.
// This is typically used by the GPU, camera/video, and audio components on
// some Snapdragon SoCs.
//
// Copyright (C) 2019 Brian Masney <masneyb@onstation.org>
// Copyright (C) 2015 Red Hat. Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocmem_client {
// GMEM clients
    OCMEM_GRAPHICS = 0x0,
//
// TODO add more once ocmem_allocate() is clever enough to
// deal with multiple clients.
//
    OCMEM_CLIENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocmem_buf {
    pub offset: c_ulong,
    pub addr: c_ulong,
    pub len: c_ulong,
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

