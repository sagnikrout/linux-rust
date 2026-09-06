//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/bundle.h
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
// Greybus bundles
//
// Copyright 2014 Google Inc.
// Copyright 2014 Linaro Ltd.
//

// Greybus "public" definitions"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bundle {
    pub dev: device,
    pub intf: *mut gb_interface,
    pub id: u8,
    pub class: u8,
    pub class_major: u8,
    pub class_minor: u8,
    pub num_cports: usize,
    pub cport_desc: *mut greybus_descriptor_cport,
    pub connections: list_head,
    pub state: *mut u8,
    pub /: *mut *mut list_head links; / interface->bundles,
}

// Greybus "private" definitions"
extern "C" {
    pub fn gb_bundle_add(bundle: *mut gb_bundle) -> c_int;
}
extern "C" {
    pub fn gb_bundle_destroy(bundle: *mut gb_bundle);
}
// Bundle Runtime PM wrappers

