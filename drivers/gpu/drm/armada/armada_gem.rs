//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/armada/armada_gem.h
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
// Copyright (C) 2012 Russell King
//

// GEM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada_gem_object {
    pub obj: drm_gem_object,
    pub addr: *mut c_void,
    pub phys_addr: phys_addr_t,
    pub dev_addr: resource_size_t,
    pub mapped: bool,
    pub /: *mut *mut *mut drm_mm_node linear; / for linear backed,
    pub /: *mut *mut *mut page page; / for page backed,
    pub /: *mut *mut *mut sg_table sgt; / for imported,
    pub ): *mut *mut void (update)(void,
    pub update_data: *mut c_void,
}

extern "C" {
    pub fn armada_gem_free_object(: *mut drm_gem_object);
}
extern "C" {
    pub fn armada_gem_linear_back(: *mut drm_device, : *mut armada_gem_object) -> c_int;
}
extern "C" {
    pub fn armada_gem_map_import(: *mut armada_gem_object) -> c_int;
}
