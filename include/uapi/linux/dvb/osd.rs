//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/osd.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// osd.h - DEPRECATED On Screen Display API
//
// NOTE: should not be used on future drivers
//
// Copyright (C) 2001 Ralph  Metzler <ralph@convergence.de>
// & Marcus Metzler <marcus@convergence.de>
// for convergence integrated media GmbH
//

// All functions return -2 on "not open"
//
// Disables OSD and releases the buffers
// returns 0 on success
//
// Opens OSD with this size and bit depth
// returns 0 on success, -1 on DRAM allocation error, -2 on "already open"
//
// enables OSD mode
// returns 0 on success
//
// disables OSD mode
// returns 0 on success
//
// Sets all pixel to color 0
// returns 0 on success
//
// Sets all pixel to color <col>
// returns 0 on success
//
// set palette entry <num> to <r,g,b>, <mix> and <trans> apply
// R,G,B: 0..255
// R=Red, G=Green, B=Blue
// opacity=0:      pixel opacity 0% (only video pixel shows)
// opacity=1..254: pixel opacity as specified in header
// opacity=255:    pixel opacity 100% (only OSD pixel shows)
// returns 0 on success, -1 on error
//
// Set a number of entries in the palette
// sets the entries "firstcolor" through "lastcolor" from the array "data"
// data has 4 byte for each color:
// R,G,B, and a opacity value: 0->transparent, 1..254->mix, 255->pixel
//
// Sets transparency of mixed pixel (0..15)
// returns 0 on success
//
// sets pixel <x>,<y> to color number <col>
// returns 0 on success, -1 on error
//
// returns color number of pixel <x>,<y>,  or -1
//
// fills pixels x0,y through  x1,y with the content of data[]
// returns 0 on success, -1 on clipping all pixel (no pixel drawn)
//
// fills pixels x0,y0 through  x1,y1 with the content of data[]
// inc contains the width of one line in the data block,
// inc<=0 uses blockwidth as linewidth
// returns 0 on success, -1 on clipping all pixel
//
// fills pixels x0,y through  x1,y with the color <col>
// returns 0 on success, -1 on clipping all pixel
//
// fills pixels x0,y0 through  x1,y1 with the color <col>
// returns 0 on success, -1 on clipping all pixel
//
// draw a line from x0,y0 to x1,y1 with the color <col>
// returns 0 on success
//
// fills parameters with the picture dimensions and the pixel aspect ratio
// returns 0 on success
//
// draws a test picture. for debugging purposes only
// returns 0 on success
// TODO: remove "test" in final version
//
// OSD_OpenRaw: set 'color' to desired window type

