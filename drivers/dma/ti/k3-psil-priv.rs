//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ti/k3-psil-priv.h
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
// Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psil_ep {
    pub thread_id: u32,
    pub ep_config: psil_endpoint_config,
}

//
// struct psil_ep_map - PSI-L thread ID configuration maps
// @name:	Name of the map, set it to the name of the SoC
// @src:	Array of source PSI-L thread configurations
// @src_count:	Number of entries in the src array
// @dst:	Array of destination PSI-L thread configurations
// @dst_count:	Number of entries in the dst array
//
// In case of symmetric configuration for a matching src/dst thread (for example
// 0x4400 and 0xc400) only the src configuration can be present. If no dst
// configuration found the code will look for (dst_thread_id & ~0x8000) to find
// the symmetric match.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psil_ep_map {
    pub name: *mut c_char,
    pub src: *mut psil_ep,
    pub src_count: c_int,
    pub dst: *mut psil_ep,
    pub dst_count: c_int,
}

// SoC PSI-L endpoint maps
