//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mmzone.h
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
// Written by Kanoj Sarcar (kanoj@sgi.com) Aug 99
//
// PowerPC64 port:
// Copyright (C) 2002 Anton Blanchard, IBM Corp.
//

//
// generic non-linear memory support:
//
// 1) we will not split memory into more chunks than will fit into the
// flags field of the struct page
//

//
// Following are specific to this numa platform.
//

extern "C" {
    pub fn memory_hotplug_max() -> u64;
}
extern "C" {
    pub fn hot_add_drconf_memory_max() -> u64;
}

