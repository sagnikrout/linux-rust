//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/gsc_mkhi_commands_abi.h
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
// Copyright © 2023 Intel Corporation
//

// Heci client ID for MKHI commands
pub const HECI_MEADDRESS_MKHI: c_int = 7;
// Generic MKHI header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_mkhi_header {
    pub group_id: u8,
    pub command: u8,
    pub reserved: u8,
    pub result: u8,
    pub __packed: },
// GFX_SRV commands
pub const MKHI_GROUP_ID_GFX_SRV: c_uint = 0x30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_get_compatibility_version_in {
    pub header: gsc_mkhi_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_get_compatibility_version_out {
    pub header: gsc_mkhi_header,
    pub proj_major: u16,
    pub compat_major: u16,
    pub compat_minor: u16,
    pub reserved: [u16; 5],
    pub __packed: },
