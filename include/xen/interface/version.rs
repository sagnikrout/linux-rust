//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/version.h
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


// SPDX-License-Identifier: MIT
//
// version.h
//
// Xen version, type, and compile information.
//
// Copyright (c) 2005, Nguyen Anh Quynh <aquynh@gmail.com>
// Copyright (c) 2005, Keir Fraser <keir@xensource.com>
//
// NB. All ops return zero on success, except XENVER_version.
// arg == NULL; returns major:minor (16:16).
pub const XENVER_version: c_int = 0;
// arg == xen_extraversion_t.
pub const XENVER_extraversion: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_extraversion {
    pub extraversion: [c_char; 16],
}

// arg == xen_compile_info_t.
pub const XENVER_compile_info: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_compile_info {
    pub compiler: [c_char; 64],
    pub compile_by: [c_char; 16],
    pub compile_domain: [c_char; 32],
    pub compile_date: [c_char; 32],
}

pub const XENVER_capabilities: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_capabilities_info {
    pub info: [c_char; 1024],
}

pub const XENVER_changeset: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_changeset_info {
    pub info: [c_char; 64],
}

pub const XENVER_platform_parameters: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_platform_parameters {
    pub virt_start: xen_ulong_t,
}

pub const XENVER_get_features: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_feature_info {
    pub /: *mut *mut unsigned int submap_idx; / IN: which 32-bit submap to return,
    pub /: *mut *mut uint32_t submap; / OUT: 32-bit submap,
}

// Declares the features reported by XENVER_get_features.

// arg == NULL; returns host memory page size.
pub const XENVER_pagesize: c_int = 7;
// arg == xen_domain_handle_t.
pub const XENVER_guest_handle: c_int = 8;
pub const XENVER_commandline: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_commandline {
    pub buf: [c_char; 1024],
}

//
// Return value is the number of bytes written, or XEN_Exx on error.
// Calling with empty parameter returns the size of build_id.
//
pub const XENVER_build_id: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_build_id {
    pub /: *mut *mut uint32_t len; / IN: size of buf[].,
    pub buf: [c_uchar; ],
}
