//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/nwbutton.h
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
// NetWinder Button Driver-
// Copyright (C) Alex Holden <alex@linuxhacker.org> 1998, 1999.
//

// Various defines:

// Structure definitions:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct button_callback {
    pub (void): *mut *mut void (callback),
    pub count: c_int,
}

// Function prototypes:
extern "C" {
    pub fn button_sequence_finished(unused: *mut timer_list) -> static void;
}
extern "C" {
    pub fn button_handler(irq: c_int, dev_id: *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn button_init() -> c_int;
}
extern "C" {
    pub fn button_add_callback((void): *mut *mut void (callback), count: c_int) -> c_int;
}
extern "C" {
    pub fn button_del_callback((void): *mut *mut void (callback)) -> c_int;
}
extern "C" {
    pub fn button_consume_callbacks(bpcount: c_int) -> static void;
}

extern "C" {
    pub fn button_add_callback((void): *mut *mut void (callback), count: c_int) -> c_int;
}
extern "C" {
    pub fn button_del_callback((void): *mut *mut void (callback)) -> c_int;
}

