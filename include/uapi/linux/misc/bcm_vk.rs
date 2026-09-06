//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/misc/bcm_vk.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright 2018-2020 Broadcom.
//

pub const BCM_VK_MAX_FILENAME: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vk_image {
    pub /: *mut *mut __u32 type; / Type of image,

    pub /: *mut *mut __u8 filename[BCM_VK_MAX_FILENAME]; / Filename of image,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vk_reset {
    pub arg1: __u32,
    pub arg2: __u32,
}

pub const VK_MAGIC: c_uint = 0x5e;
// Load image to Valkyrie

// Send Reset to Valkyrie

//
// Firmware Status accessed directly via BAR space
//
pub const VK_BAR_FWSTS: c_uint = 0x41c;
pub const VK_BAR_COP_FWSTS: c_uint = 0x428;
// VK_FWSTS definitions

pub const VK_FWSTS_MASK: c_uint = 0xffffffff;

// Deinit

// Last nibble for reboot reason
pub const VK_FWSTS_RESET_REASON_SHIFT: c_int = 28;

