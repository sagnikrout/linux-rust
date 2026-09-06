//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/eeh_event.h
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
//
// Copyright (c) 2005 Linas Vepstas <linas@linas.org>
//

//
// structure holding pci controller data that describes a
// change in the isolation status of a PCI slot.  A pointer
// to this struct is passed as the data pointer in a notify
// callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeh_event {
    pub /: *mut *mut list_head list; / to form event queue,
    pub /: *mut *mut *mut eeh_pe pe; / EEH PE,
}

extern "C" {
    pub fn eeh_event_init() -> c_int;
}
extern "C" {
    pub fn eeh_send_failure_event(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn __eeh_send_failure_event(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_remove_event(pe: *mut eeh_pe, force: bool);
}
extern "C" {
    pub fn eeh_handle_normal_event(pe: *mut eeh_pe);
}
extern "C" {
    pub fn eeh_handle_special_event();
}

