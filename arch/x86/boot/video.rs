//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/video.h
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
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
//
// -----------------------------------------------------------------------
//
// Header file for the real-mode video probing code
//

//
// This code uses an extended set of video mode numbers. These include:
// Aliases for standard modes
// NORMAL_VGA (-1)
// EXTENDED_VGA (-2)
// ASK_VGA (-3)
// Video modes numbered by menu position -- NOT RECOMMENDED because of lack
// of compatibility when extending the table. These are between 0x00 and 0xff.
//
pub const VIDEO_FIRST_MENU: c_uint = 0x0000;
// Standard BIOS video modes (BIOS number + 0x0100)
pub const VIDEO_FIRST_BIOS: c_uint = 0x0100;
// VESA BIOS video modes (VESA number + 0x0200)
pub const VIDEO_FIRST_VESA: c_uint = 0x0200;
// Video7 special modes (BIOS number + 0x0900)
pub const VIDEO_FIRST_V7: c_uint = 0x0900;
// Special video modes
pub const VIDEO_FIRST_SPECIAL: c_uint = 0x0f00;
pub const VIDEO_80x25: c_uint = 0x0f00;
pub const VIDEO_8POINT: c_uint = 0x0f01;
pub const VIDEO_80x43: c_uint = 0x0f02;
pub const VIDEO_80x28: c_uint = 0x0f03;
pub const VIDEO_CURRENT_MODE: c_uint = 0x0f04;
pub const VIDEO_80x30: c_uint = 0x0f05;
pub const VIDEO_80x34: c_uint = 0x0f06;
pub const VIDEO_80x60: c_uint = 0x0f07;
pub const VIDEO_GFX_HACK: c_uint = 0x0f08;
pub const VIDEO_LAST_SPECIAL: c_uint = 0x0f09;
// Video modes given by resolution
pub const VIDEO_FIRST_RESOLUTION: c_uint = 0x1000;
// The "recalculate timings" flag
pub const VIDEO_RECALC: c_uint = 0x8000;
extern "C" {
    pub fn store_screen();
}

//
// Mode table structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_info {
    pub /: *mut *mut u16 mode; / Mode number (vga= style),
    pub /: *mut *mut u16 x, y; / Width, height,
    pub /: *mut *mut u16 depth; / Bits per pixel, 0 for text mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_info {
    pub card_name: *const c_char,
    pub mode): *mut *mut int (set_mode)(struct mode_info,
    pub (*probe)(void): *mut c_int,
    pub modes: *mut mode_info,
    pub /: *mut *mut int nmodes; / Number of probed modes so far,
    pub /: *mut *mut int unsafe; / Probing is unsafe, only do after "scan",
    pub /: *mut *mut u16 xmode_first; / Unprobed modes to try to call anyway,
    pub /: *mut *mut u16 xmode_n; / Size of unprobed mode range,
}

// Basic video information

pub const ADAPTER_EGA: c_int = 1;
pub const ADAPTER_VGA: c_int = 2;
// Accessing VGA indexed registers
extern "C" {
    pub fn inb(_arg: port+1) -> return;
}
// Writes a value to an indexed port and then reads the port again
extern "C" {
    pub fn in_idx(_arg: port, _arg: index) -> return;
}
// Get the I/O port of the VGA CRTC
