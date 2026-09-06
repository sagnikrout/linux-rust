//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cfag12864b.h
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
// Filename: cfag12864b.h
// Version: 0.1.0
// Description: cfag12864b LCD driver header
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-12
//

//
// The driver will blit this buffer to the LCD
//
// Its size is CFAG12864B_SIZE.
//
// Enable refreshing
//
// Returns 0 if successful (anyone was using it),
// or != 0 if failed (someone is using it).
//
extern "C" {
    pub fn cfag12864b_enable() -> c_uchar;
}
//
// Disable refreshing
//
// You should call this only when you finish using the LCD.
//
extern "C" {
    pub fn cfag12864b_disable();
}
//
// Is the module inited?
//
extern "C" {
    pub fn cfag12864b_isinited() -> c_uchar;
}
