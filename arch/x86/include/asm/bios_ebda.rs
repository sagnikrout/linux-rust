//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/bios_ebda.h
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
// Returns physical address of EBDA.  Returns 0 if there is no EBDA.
//
// There is a real-mode segmented pointer pointing to the
// 4K EBDA area at 0x40E.
//
extern "C" {
    pub fn reserve_bios_regions();
}

//
// This is obviously not a great place for this, but we want to be
// able to scatter it around anywhere in the kernel.
//
extern "C" {
    pub fn check_for_bios_corruption();
}
extern "C" {
    pub fn start_periodic_check_for_corruption();
}

