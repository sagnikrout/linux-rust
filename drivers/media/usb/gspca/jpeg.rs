//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/jpeg.h
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
pub const JPEG_H: c_int = 1;
//
// Insert a JPEG header at start of frame
//
// This module is used by the gspca subdrivers.
// A special case is done for Conexant webcams.
//
// Copyright (C) Jean-Francois Moine (http://moinejf.free.fr)
//
// generation options
// CONEX_CAM	Conexant if present
//
// JPEG header
// quantization table quality 50%
pub const JPEG_QT0_OFFSET: c_int = 7;
pub const JPEG_QT1_OFFSET: c_int = 72;
// huffman table

// the Conexant frames start with SOF0
pub const JPEG_HDR_SZ: c_int = 556;

pub const JPEG_HEIGHT_OFFSET: c_int = 561;
pub const JPEG_HDR_SZ: c_int = 589;

// define the JPEG header

// set the JPEG quality
