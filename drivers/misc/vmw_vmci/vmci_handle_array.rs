//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_handle_array.h
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
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_handle_arr {
    pub capacity: u32,
    pub max_capacity: u32,
    pub size: u32,
    pub pad: u32,
    pub __counted_by(capacity): vmci_handle entries[],
}

// Select a default capacity that results in a 64 byte sized array
pub const VMCI_HANDLE_ARRAY_DEFAULT_CAPACITY: c_int = 6;
extern "C" {
    pub fn vmci_handle_arr_destroy(array: *mut vmci_handle_arr);
}
extern "C" {
    pub fn vmci_handle_arr_remove_tail(array: *mut vmci_handle_arr) -> vmci_handle;
}
