//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/coreboot.h
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
// coreboot.h
//
// Coreboot device and driver interfaces.
//
// Copyright 2014 Gerd Hoffmann <kraxel@redhat.com>
// Copyright 2017 Google Inc.
// Copyright 2017 Samuel Holland <samuel@sholland.org>
//

// List of coreboot entry structures that is used
pub const CB_TAG_FRAMEBUFFER: c_uint = 0x12;
pub const LB_TAG_CBMEM_ENTRY: c_uint = 0x31;
// Generic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coreboot_table_entry {
    pub tag: u32,
    pub size: u32,
}

// Points to a CBMEM entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_cbmem_ref {
    pub tag: u32,
    pub size: u32,
    pub cbmem_addr: cb_u64,
}

// Corresponds to LB_TAG_CBMEM_ENTRY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_cbmem_entry {
    pub tag: u32,
    pub size: u32,
    pub address: cb_u64,
    pub entry_size: u32,
    pub id: u32,
}

pub const LB_FRAMEBUFFER_ORIENTATION_NORMAL: c_int = 0;
pub const LB_FRAMEBUFFER_ORIENTATION_BOTTOM_UP: c_int = 1;
pub const LB_FRAMEBUFFER_ORIENTATION_LEFT_UP: c_int = 2;
pub const LB_FRAMEBUFFER_ORIENTATION_RIGHT_UP: c_int = 3;
// Describes framebuffer setup by coreboot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_framebuffer {
    pub tag: u32,
    pub size: u32,
    pub physical_address: cb_u64,
    pub x_resolution: u32,
    pub y_resolution: u32,
    pub bytes_per_line: u32,
    pub bits_per_pixel: u8,
    pub red_mask_pos: u8,
    pub red_mask_size: u8,
    pub green_mask_pos: u8,
    pub green_mask_size: u8,
    pub blue_mask_pos: u8,
    pub blue_mask_size: u8,
    pub reserved_mask_pos: u8,
    pub reserved_mask_size: u8,
    pub orientation: u8,
}

//
// True if the coreboot-provided data is large enough to hold information
// on the linear framebuffer. False otherwise.
//

//
// True if the coreboot-provided data is large enough to hold information
// on the display orientation. False otherwise.
//

