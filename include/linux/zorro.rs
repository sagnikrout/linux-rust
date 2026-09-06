//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zorro.h
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
// linux/zorro.h -- Amiga AutoConfig (Zorro) Bus Definitions
//
// Copyright (C) 1995--2003 Geert Uytterhoeven
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

//
// Zorro devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zorro_dev {
    pub rom: ExpansionRom,
    pub id: zorro_id,
    pub /: *mut *mut device dev; / Generic device interface,
    pub slotaddr: u16,
    pub slotsize: u16,
    pub name: [c_char; 64],
    pub resource: resource,
}

//
// Zorro device drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zorro_driver {
    pub node: list_head,
    pub name: *mut c_char,
    pub /: *const *const *const zorro_device_id id_table; / NULL if wants all devices,
    pub /: *const *const *const *const *const int (probe)(struct zorro_dev z, struct zorro_device_id id); / New device inserted,
    pub /: *mut *mut *mut *mut void (remove)(struct zorro_dev z); / Device removed (NULL if not a hot-plug capable driver),
    pub driver: device_driver,
}

// New-style probing
extern "C" {
    pub fn zorro_register_driver(: *mut zorro_driver) -> c_int;
}
extern "C" {
    pub fn zorro_unregister_driver(: *mut zorro_driver);
}
//
// Minimal information about a Zorro device, passed from bootinfo
// Only available temporarily, i.e. until initmem has been freed!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zorro_dev_init {
    pub rom: ExpansionRom,
    pub slotaddr: u16,
    pub slotsize: u16,
    pub boardaddr: u32,
    pub boardsize: u32,
}

//
// Zorro Functions
//

// Similar to the helpers above, these manipulate per-zorro_dev
// driver-specific data.  They are really just a wrapper around
// the generic device structure functions of these calls.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &z->dev) -> return;
}
//
// Bitmask indicating portions of available Zorro II RAM that are unused
// by the system. Every bit represents a 64K chunk, for a maximum of 8MB
// (128 chunks, physical 0x00200000-0x009fffff).
//
// If you want to use (= allocate) portions of this RAM, you should clear
// the corresponding bits.
//
extern "C" {
    pub fn DECLARE_BITMAP(_arg: zorro_unused_z2ram, _arg: 128) -> extern;
}

