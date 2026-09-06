//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/gpibP.h
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
// copyright		   : (C) 2002,2003 by Frank Mori Hess
//

extern "C" {
    pub fn gpib_register_driver(interface: *mut gpib_interface, mod: *mut module) -> c_int;
}
extern "C" {
    pub fn gpib_unregister_driver(interface: *mut gpib_interface);
}
extern "C" {
    pub fn num_gpib_events(queue: *const gpib_event_queue) -> c_uint;
}
extern "C" {
    pub fn push_gpib_event(board: *mut gpib_board, event_type: c_short) -> c_int;
}
extern "C" {
    pub fn pop_gpib_event(board: *mut gpib_board, queue: *mut gpib_event_queue, event_type: *mut c_short) -> c_int;
}
extern "C" {
    pub fn gpib_request_pseudo_irq(board: *mut gpib_board, (*handler)(int: *mut irqreturn_t, ): *mut c_void) -> c_int;
}
extern "C" {
    pub fn gpib_free_pseudo_irq(board: *mut gpib_board);
}
extern "C" {
    pub fn gpib_match_device_path(dev: *mut device, device_path_in: *const c_char) -> c_int;
}
