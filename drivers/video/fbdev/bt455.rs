//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/bt455.h
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
// linux/drivers/video/bt455.h
//
// Copyright 2003  Thiemo Seufer <seufer@csv.ica.uni-stuttgart.de>
// Copyright 2016  Maciej W. Rozycki <macro@linux-mips.org>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file COPYING in the main directory of this
// archive for more details.
//

//
// Bt455 byte-wide registers, 32-bit aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt455_regs {
    pub addr_cmap: volatile u8,
    pub pad0: [u8; 3],
    pub addr_cmap_data: volatile u8,
    pub pad1: [u8; 3],
    pub addr_clr: volatile u8,
    pub pad2: [u8; 3],
    pub addr_ovly: volatile u8,
    pub pad3: [u8; 3],
}

//
// Read/write to a Bt455 color map register.
//
// grey = regs->addr_cmap_data & 0xf;
