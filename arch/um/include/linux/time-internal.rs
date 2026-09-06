//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/linux/time-internal.h
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
// Copyright (C) 2012 - 2014 Cisco Systems
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

pub const TIMER_MULTIPLIER: c_int = 256;
pub const TIMER_MIN_DELTA: c_int = 500;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_travel_event {
    pub time: c_ulonglong,
    pub d): *mut *mut void (fn)(struct time_travel_event,
    pub list: list_head,
    pub onstack: bool pending,,
}

extern "C" {
    pub fn time_travel_sleep();
}
extern "C" {
    pub fn __time_travel_propagate_time();
}
extern "C" {
    pub fn __time_travel_wait_readable(fd: c_int);
}
extern "C" {
    pub fn time_travel_add_irq_event(e: *mut time_travel_event);
}
extern "C" {
    pub fn time_travel_del_event(e: *mut time_travel_event) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_travel_event {
}

// this is a macro so the event/function need not exist

//
// not inlines so the data structure need not exist,
// cause linker failures
//
extern "C" {
    pub fn time_travel_not_configured();
}

//
// Without CONFIG_UML_TIME_TRAVEL_SUPPORT this is a linker error if used,
// which is intentional since we really shouldn't link it in that case.
//
extern "C" {
    pub fn time_travel_ndelay(nsec: c_ulong);
}
extern "C" {
    pub fn um_setup_timer() -> c_int;
}
