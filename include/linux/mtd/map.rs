//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/map.h
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
// Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org> et al.
//
// Overhauled routines for dealing with different mmap regions of flash

pub const map_bankwidth(map): c_int = 1;

pub const MAX_MAP_BANKWIDTH: c_int = 1;

pub const MAX_MAP_BANKWIDTH: c_int = 2;

pub const MAX_MAP_BANKWIDTH: c_int = 4;

// ensure we never evaluate anything shorted than an unsigned long
// to zero, and ensure we'll never miss the end of an comparison (bjd)

pub const MAX_MAP_BANKWIDTH: c_int = 8;

pub const MAX_MAP_BANKWIDTH: c_int = 16;

// always use indirect access for 256-bit to preserve kernel stack

pub const MAX_MAP_BANKWIDTH: c_int = 32;

pub const MAX_MAP_BANKWIDTH: c_int = 1;

// The map stuff is very simple. You fill in your struct map_info with
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_info {
    pub name: *const c_char,
    pub size: c_ulong,
    pub phys: resource_size_t,

    pub virt: *mut void __iomem,
    pub cached: *mut c_void,
    pub /: *mut *mut int swap; / this mapping's byte-swapping requirement,
    pub width: *mut *mut int bankwidth; / in octets. This isn't necessarily the,
//

    pub long): *mut *mut *mut map_word (read)(struct map_info , unsigned,
    pub ssize_t): *mut *mut *mut *mut void (copy_from)(struct map_info , void , unsigned long,,
    pub long): *const *const *const void (write)(struct map_info , map_word, unsigned,
    pub ssize_t): *const *const *const *const void (copy_to)(struct map_info , unsigned long, void ,,
// We can perhaps put in 'point' and 'unpoint' methods, if we really

// It's possible for the map driver to use cached memory in its
    pub ssize_t): *mut *mut *mut void (inval_cache)(struct map_info , unsigned long,,
// This will be called with 1 as parameter when the first map user
// needs VPP, and called with 0 when the last user exits. The map
// core maintains a reference counter, and assumes that VPP is a
// global resource applying to all mapped flash chips on the system.
//
    pub int): *mut *mut *mut void (set_vpp)(struct map_info ,,
    pub pfow_base: c_ulong,
    pub map_priv_1: c_ulong,
    pub map_priv_2: c_ulong,
    pub device_node: *mut device_node,
    pub fldrv_priv: *mut c_void,
    pub fldrv: *mut mtd_chip_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_chip_driver {
    pub map): *mut *mut *mut mtd_info (probe)(map_info,
    pub ): *mut *mut void (destroy)(struct mtd_info,
    pub module: *mut module,
    pub name: *mut c_char,
    pub list: list_head,
}

extern "C" {
    pub fn register_mtd_chip_driver(: *mut mtd_chip_driver);
}
extern "C" {
    pub fn unregister_mtd_chip_driver(: *mut mtd_chip_driver);
}
extern "C" {
    pub fn map_destroy(mtd: *mut mtd_info);
}

pub const MAP_FF_LIMIT: c_int = 4;

pub const MAP_FF_LIMIT: c_int = 8;

extern "C" {
    pub fn simple_map_init(: *mut map_info);
}

