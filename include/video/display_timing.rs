//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/display_timing.h
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
// Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
//
// description of display timings
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_flags {
    DISPLAY_FLAGS_HSYNC_LOW		= BIT(0),
    DISPLAY_FLAGS_HSYNC_HIGH	= BIT(1),
    DISPLAY_FLAGS_VSYNC_LOW		= BIT(2),
    DISPLAY_FLAGS_VSYNC_HIGH	= BIT(3),

// data enable flag
    DISPLAY_FLAGS_DE_LOW		= BIT(4),
    DISPLAY_FLAGS_DE_HIGH		= BIT(5),
// drive data on pos. edge
    DISPLAY_FLAGS_PIXDATA_POSEDGE	= BIT(6),
// drive data on neg. edge
    DISPLAY_FLAGS_PIXDATA_NEGEDGE	= BIT(7),
    DISPLAY_FLAGS_INTERLACED	= BIT(8),
    DISPLAY_FLAGS_DOUBLESCAN	= BIT(9),
    DISPLAY_FLAGS_DOUBLECLK		= BIT(10),
// drive sync on pos. edge
    DISPLAY_FLAGS_SYNC_POSEDGE	= BIT(11),
// drive sync on neg. edge
    DISPLAY_FLAGS_SYNC_NEGEDGE	= BIT(12),
}

//
// A single signal can be specified via a range of minimal and maximal values
// with a typical value, that lies somewhere inbetween.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_entry {
    pub min: u32,
    pub typ: u32,
    pub max: u32,
}

//
// Single "mode" entry. This describes one set of signal timings a display can
// have in one setting. This struct can later be converted to struct videomode
// (see include/video/videomode.h). As each timing_entry can be defined as a
// range, one struct display_timing may become multiple struct videomodes.
//
// Example: hsync active high, vsync active low
//
// Active Video
// Video  ______________________XXXXXXXXXXXXXXXXXXXXXX_____________________
// |<- sync ->|<- back ->|<----- active ----->|<- front ->|<- sync..
// |	     |	 porch  |		     |	 porch	 |
//
// HSync _|¯¯¯¯¯¯¯¯¯¯|___________________________________________|¯¯¯¯¯¯¯¯¯
//
// VSync ¯|__________|¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯|_________
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_timing {
    pub pixelclock: timing_entry,
    pub /: *mut *mut timing_entry hactive; / hor. active video,
    pub /: *mut *mut timing_entry hfront_porch; / hor. front porch,
    pub /: *mut *mut timing_entry hback_porch; / hor. back porch,
    pub /: *mut *mut timing_entry hsync_len; / hor. sync len,
    pub /: *mut *mut timing_entry vactive; / ver. active video,
    pub /: *mut *mut timing_entry vfront_porch; / ver. front porch,
    pub /: *mut *mut timing_entry vback_porch; / ver. back porch,
    pub /: *mut *mut timing_entry vsync_len; / ver. sync len,
    pub /: *mut *mut display_flags flags; / display flags,
}

//
// This describes all timing settings a display provides.
// The native_mode is the default setting for this display.
// Drivers that can handle multiple videomodes should work with this struct and
// convert each entry to the desired end result.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_timings {
    pub num_timings: c_uint,
    pub native_mode: c_uint,
    pub timings: *mut display_timing,
}

// get one entry from struct display_timings
extern "C" {
    pub fn display_timings_release(disp: *mut display_timings);
}
