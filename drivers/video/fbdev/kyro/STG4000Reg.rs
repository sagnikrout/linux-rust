//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/kyro/STG4000Reg.h
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


//
// linux/drivers/video/kyro/STG4000Reg.h
//
// Copyright (C) 2002 STMicroelectronics
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Macros that access memory mapped card registers in PCI space
// Add an appropriate section for your OS or processor architecture.
//

// LUT select
// Primary surface pixel format select
// Overlay blending mode select
// Overlay Pixel format select
// Register Table
// 0h
//
// DWFILL; //GAP 0x0CE0
//
// DWFILL; //GAP 0x3000          ALUT 256MB offset
