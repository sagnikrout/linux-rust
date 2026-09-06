//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nubus.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nubus_dir {
    pub base: *mut c_uchar,
    pub ptr: *mut c_uchar,
    pub done: c_int,
    pub mask: c_int,
    pub procdir: *mut proc_dir_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nubus_dirent {
    pub base: *mut c_uchar,
    pub type: c_uchar,
    pub /: *mut *mut __u32 data; / Actually 24 bits used,
    pub mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nubus_board {
    pub dev: device,
// Only 9-E actually exist, though 0-8 are also theoretically
    pub slot: c_int,
// For slot 0, this is bogus.
    pub name: [c_char; 64],
// Format block
    pub fblock: *mut c_uchar,
// Root directory (does *not* always equal fblock + doffset!)
    pub directory: *mut c_uchar,
    pub slot_addr: c_ulong,
// Offset to root directory (sometimes)
    pub doffset: c_ulong,
// Length over which to compute the crc
    pub rom_length: c_ulong,
// Completely useless most of the time
    pub crc: c_ulong,
    pub rev: c_uchar,
    pub format: c_uchar,
    pub lanes: c_uchar,
// Directory entry in /proc/bus/nubus
    pub procdir: *mut proc_dir_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nubus_rsrc {
    pub list: list_head,
// The functional resource ID
    pub resid: c_uchar,
// These are mostly here for convenience; we could always read
    pub category: c_ushort,
    pub type: c_ushort,
    pub dr_sw: c_ushort,
    pub dr_hw: c_ushort,
// Functional directory
    pub directory: *mut c_uchar,
// Much of our info comes from here
    pub board: *mut nubus_board,
}

// This is all NuBus functional resources (used to find devices later on)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nubus_driver {
    pub driver: device_driver,
    pub board): *mut *mut int (probe)(struct nubus_board,
    pub board): *mut *mut void (remove)(struct nubus_board,
}

// Generic NuBus interface functions, modelled after the PCI interface

extern "C" {
    pub fn nubus_proc_init();
}

// These are somewhat more NuBus-specific.  They all return 0 for
// The root directory which contains the board and functional
// The board directory
// The functional directory
extern "C" {
    pub fn nubus_get_func_dir(fres: *const nubus_rsrc, dir: *mut nubus_dir) -> c_int;
}
// These work on any directory gotten via the above
extern "C" {
    pub fn nubus_rewinddir(dir: *mut nubus_dir) -> c_int;
}
// Things to do with directory entries
// Declarations relating to driver model objects
extern "C" {
    pub fn nubus_device_register(parent: *mut device, board: *mut nubus_board) -> c_int;
}
extern "C" {
    pub fn nubus_driver_register(ndrv: *mut nubus_driver) -> c_int;
}
extern "C" {
    pub fn nubus_driver_unregister(ndrv: *mut nubus_driver);
}
extern "C" {
    pub fn nubus_proc_show(m: *mut seq_file, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &board->dev) -> return;
}
// Returns a pointer to the "standard" slot space.
