//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/resource_ext.h
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
// Copyright (C) 2015, Intel Corporation
// Author: Jiang Liu <jiang.liu@linux.intel.com>
//

// Represent resource window for bridge devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_win {
    pub /: *mut *mut resource res; / In master (CPU) address space,
    pub /: *mut *mut resource_size_t offset; / Translation offset for bridge,
}

//
// Common resource list management data structure and interfaces to support
// ACPI, PNP and PCI host bridge etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_entry {
    pub node: list_head,
    pub /: *mut *mut *mut resource res; / In master (CPU) address space,
    pub /: *mut *mut resource_size_t offset; / Translation offset for bridge,
    pub /: *mut *mut resource __res; / Default storage for res,
}

extern "C" {
    pub fn resource_list_free(head: *mut list_head);
}

