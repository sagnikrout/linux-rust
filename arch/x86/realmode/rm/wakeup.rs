//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/realmode/rm/wakeup.h
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
// Definitions for the wakeup data structure at the head of the
// wakeup code.
//

// This must match data at wakeup.S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakeup_header {
    pub /: *mut *mut u16 video_mode; / Video mode number,
    pub /: *mut *mut u32 pmode_entry; / Protected mode resume point, 32-bit only,
    pub pmode_cs: u16,
    pub /: *mut *mut u32 pmode_cr0; / Protected mode cr0,
    pub /: *mut *mut u32 pmode_cr3; / Protected mode cr3,
    pub /: *mut *mut u32 pmode_cr4; / Protected mode cr4,
    pub /: *mut *mut u32 pmode_efer_low; / Protected mode EFER,
    pub pmode_efer_high: u32,
    pub pmode_gdt: u64,
    pub /: *mut *mut u32 pmode_misc_en_low; / Protected mode MISC_ENABLE,
    pub pmode_misc_en_high: u32,
    pub /: *mut *mut u32 pmode_behavior; / Wakeup routine behavior flags,
    pub realmode_flags: u32,
    pub real_magic: u32,
    pub /: *mut *mut u32 signature; / To check we have correct structure,
    pub __attribute__((__packed__)): },
    pub wakeup_header: extern struct wakeup_header,

pub const WAKEUP_HEADER_OFFSET: c_int = 8;
pub const WAKEUP_HEADER_SIGNATURE: c_uint = 0x51ee1111;
// Wakeup behavior bits
pub const WAKEUP_BEHAVIOR_RESTORE_MISC_ENABLE: c_int = 0;
pub const WAKEUP_BEHAVIOR_RESTORE_CR4: c_int = 1;
pub const WAKEUP_BEHAVIOR_RESTORE_EFER: c_int = 2;
