//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/vesa.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// -----------------------------------------------------------------------
//
// Copyright 1999-2007 H. Peter Anvin - All Rights Reserved
//
// -----------------------------------------------------------------------
// VESA General Information table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vesa_general_info {
    pub /: *mut *mut u32 signature; / 0 Magic number = "VESA",
    pub /: *mut *mut u16 version; / 4,
    pub /: *mut *mut far_ptr vendor_string; / 6,
    pub /: *mut *mut u32 capabilities; / 10,
    pub /: *mut *mut far_ptr video_mode_ptr; / 14,
    pub /: *mut *mut u16 total_memory; / 18,
    pub /: *mut *mut u8 reserved[236]; / 20,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vesa_mode_info {
    pub /: *mut *mut u16 mode_attr; / 0,
    pub /: *mut *mut u8 win_attr[2]; / 2,
    pub /: *mut *mut u16 win_grain; / 4,
    pub /: *mut *mut u16 win_size; / 6,
    pub /: *mut *mut u16 win_seg[2]; / 8,
    pub /: *mut *mut far_ptr win_scheme; / 12,
    pub /: *mut *mut u16 logical_scan; / 16,
    pub /: *mut *mut u16 h_res; / 18,
    pub /: *mut *mut u16 v_res; / 20,
    pub /: *mut *mut u8 char_width; / 22,
    pub /: *mut *mut u8 char_height; / 23,
    pub /: *mut *mut u8 memory_planes; / 24,
    pub /: *mut *mut u8 bpp; / 25,
    pub /: *mut *mut u8 banks; / 26,
    pub /: *mut *mut u8 memory_layout; / 27,
    pub /: *mut *mut u8 bank_size; / 28,
    pub /: *mut *mut u8 image_planes; / 29,
    pub /: *mut *mut u8 page_function; / 30,
    pub /: *mut *mut u8 rmask; / 31,
    pub /: *mut *mut u8 rpos; / 32,
    pub /: *mut *mut u8 gmask; / 33,
    pub /: *mut *mut u8 gpos; / 34,
    pub /: *mut *mut u8 bmask; / 35,
    pub /: *mut *mut u8 bpos; / 36,
    pub /: *mut *mut u8 resv_mask; / 37,
    pub /: *mut *mut u8 resv_pos; / 38,
    pub /: *mut *mut u8 dcm_info; / 39,
    pub /: *mut *mut u32 lfb_ptr; / 40 Linear frame buffer address,
    pub /: *mut *mut u32 offscreen_ptr; / 44 Offscreen memory address,
    pub /: *mut *mut u16 offscreen_size; / 48,
    pub /: *mut *mut u8 reserved[206]; / 50,
// C attribute field omitted
