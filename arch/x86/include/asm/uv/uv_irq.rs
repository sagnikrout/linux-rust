//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/uv_irq.h
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
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// SGI UV IRQ definitions
//
// Copyright (C) 2008 Silicon Graphics, Inc. All rights reserved.
//
// If a generic version of this structure gets defined, eliminate this one.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_IO_APIC_route_entry {
    pub 32: dest :,
}

extern "C" {
    pub fn uv_setup_irq(: *mut c_char, _arg: c_int, _arg: c_int, long: unsigned, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn uv_teardown_irq(int: unsigned);
}
