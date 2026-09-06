//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/seamldr.h
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
// This is the "SEAMLDR_INFO" data structure defined in the
// "SEAM Loader (SEAMLDR) Interface Specification".
//
// Must be aligned to a 256-byte boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seamldr_info {
    pub version: u32,
    pub attributes: u32,
    pub vendor_id: u32,
    pub build_date: u32,
    pub build_num: u16,
    pub minor_version: u16,
    pub major_version: u16,
    pub update_version: u16,
    pub acm_x2apicid: u32,
    pub num_remaining_updates: u32,
    pub seam_info: [u8; 128],
    pub seam_ready: u8,
    pub seam_debug: u8,
    pub p_seam_ready: u8,
    pub reserved: [u8; 93],
    pub __aligned(256): } __packed,
    pub 256): static_assert(sizeof(struct seamldr_info) ==,
    pub seamldr_info): *mut int seamldr_get_info(struct seamldr_info,
    pub data_len): *const *const int seamldr_install_module(u8 data, u32,
    pub seamldr_lock_module_update(void): c_void,
    pub seamldr_unlock_module_update(void): c_void,
