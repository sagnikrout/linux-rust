//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/posix-clock.h
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
// posix-clock.h - support for dynamic clock devices
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

//
// struct posix_clock_operations - functional interface to the clock
//
// Every posix clock is represented by a character device. Drivers may
// optionally offer extended capabilities by implementing the
// character device methods. The character device file operations are
// first handled by the clock device layer, then passed on to the
// driver by calling these functions.
//
// @owner:          The clock driver should set to THIS_MODULE
// @clock_adjtime:  Adjust the clock
// @clock_gettime:  Read the current time
// @clock_getres:   Get the clock resolution
// @clock_settime:  Set the current time value
// @open:           Optional character device open method
// @release:        Optional character device release method
// @ioctl:          Optional character device ioctl method
// @read:           Optional character device read method
// @poll:           Optional character device poll method
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_clock_operations {
    pub owner: *mut module,
    pub tx): *mut *mut *mut int (clock_adjtime)(struct posix_clock pc, struct __kernel_timex,
    pub ts): *mut *mut *mut int (clock_gettime)(struct posix_clock pc, struct timespec64,
    pub ts): *mut *mut *mut int (clock_getres) (struct posix_clock pc, struct timespec64,
    pub ts): *const timespec64,
//
// Optional character device methods:
//
    pub arg): c_ulong,
    pub f_mode): *mut *mut *mut int (open)(struct posix_clock_context pccontext, fmode_t,
    pub wait): *mut poll_table,
    pub pccontext): *mut *mut int (release)(struct posix_clock_context,
    pub cnt): *mut *mut char __user buf, size_t,
}

//
// struct posix_clock - represents a dynamic posix clock
//
// @ops:     Functional interface to the clock
// @cdev:    Character device instance for this clock
// @dev:     Pointer to the clock's device.
// @rwsem:   Protects the 'zombie' field from concurrent access.
// @zombie:  If 'zombie' is true, then the hardware has disappeared.
//
// Drivers should embed their struct posix_clock within a private
// structure, obtaining a reference to it during callbacks using
// container_of().
//
// Drivers should supply an initialized but not exposed struct device
// to posix_clock_register(). It is used to manage lifetime of the
// driver's private structure. It's 'release' field should be set to
// a release function for this private structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_clock {
    pub ops: posix_clock_operations,
    pub cdev: cdev,
    pub dev: *mut device,
    pub rwsem: rw_semaphore,
    pub zombie: bool,
}

//
// struct posix_clock_context - represents clock file operations context
//
// @clk:              Pointer to the clock
// @fp:               Pointer to the file used to open the clock
// @private_clkdata:  Pointer to user data
//
// Drivers should use struct posix_clock_context during specific character
// device file operation methods to access the posix clock. In particular,
// the file pointer can be used to verify correct access mode for ioctl()
// calls.
//
// Drivers can store a private data structure during the open operation
// if they have specific information that is required in other file
// operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_clock_context {
    pub clk: *mut posix_clock,
    pub fp: *mut file,
    pub private_clkdata: *mut c_void,
}

//
// posix_clock_register() - register a new clock
// @clk:   Pointer to the clock. Caller must provide 'ops' field
// @dev:   Pointer to the initialized device. Caller must provide
// 'release' field
//
// A clock driver calls this function to register itself with the
// clock device subsystem. If 'clk' points to dynamically allocated
// memory, then the caller must provide a 'release' function to free
// that memory.
//
// Returns zero on success, non-zero otherwise.
//
extern "C" {
    pub fn posix_clock_register(clk: *mut posix_clock, dev: *mut device) -> c_int;
}
//
// posix_clock_unregister() - unregister a clock
// @clk: Clock instance previously registered via posix_clock_register()
//
// A clock driver calls this function to remove itself from the clock
// device subsystem. The posix_clock itself will remain (in an
// inactive state) until its reference count drops to zero, at which
// point it will be deallocated with its 'release' method.
//
extern "C" {
    pub fn posix_clock_unregister(clk: *mut posix_clock);
}
