//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/regmap/internal.h
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
// Register map access API internal header
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_debugfs_off_cache {
    pub list: list_head,
    pub min: off_t,
    pub max: off_t,
    pub base_reg: c_uint,
    pub max_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_format {
    pub buf_size: usize,
    pub reg_bytes: usize,
    pub pad_bytes: usize,
    pub val_bytes: usize,
    pub reg_shift: i8,
    pub val): unsigned int reg, unsigned int,
    pub shift): *mut *mut *mut void (format_reg)(void buf, unsigned int reg, unsigned int,
    pub shift): *mut *mut *mut void (format_val)(void buf, unsigned int val, unsigned int,
    pub buf): *const *const unsigned int (parse_val)(void,
    pub buf): *mut *mut void (parse_inplace)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_async {
    pub list: list_head,
    pub map: *mut regmap,
    pub work_buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap {
    pub mutex: mutex,
    pub spinlock: spinlock_t,
    pub spinlock_flags: c_ulong,
}

// Bulk read/write
// number of bits to (left) shift the reg value when formatting
// If set, will always write field to HW.
// regcache specific members
// number of bytes in reg_defaults_raw
// number of bytes per word in reg_defaults_raw
// number of entries in reg_defaults
// number of entries in reg_defaults_raw
// if set, only the cache is modified not the HW
// if set, only the HW is modified not the cache
// if set, remember to free reg_defaults_raw
// if set, the cache contains newer data than the HW
// if set, the HW registers are known to match map->reg_defaults
// if set, the regmap core can sleep
// if set, converts bulk read to single read
// if set, converts bulk write to single write
// if set, the device supports multi write mode
// if set, raw reads/writes are limited to this size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regcache_ops {
    pub name: *const c_char,
    pub type: regcache_type,
    pub map): *mut *mut int (init)(struct regmap,
    pub map): *mut *mut void (exit)(struct regmap,
    pub map): *mut *mut int (populate)(struct regmap,

    pub map): *mut *mut void (debugfs_init)(struct regmap,

    pub value): *mut *mut *mut int (read)(struct regmap map, unsigned int reg, unsigned int,
    pub value): *mut *mut *mut int (write)(struct regmap map, unsigned int reg, unsigned int,
    pub max): *mut *mut *mut int (sync)(struct regmap map, unsigned int min, unsigned int,
    pub max): *mut *mut *mut int (drop)(struct regmap map, unsigned int min, unsigned int,
}

extern "C" {
    pub fn regmap_cached(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_writeable(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_readable(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_volatile(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_precious(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_writeable_noinc(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_readable_noinc(map: *mut regmap, reg: c_uint) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_range_node {
    pub node: rb_node,
    pub name: *const c_char,
    pub map: *mut regmap,
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_int,
    pub window_start: c_uint,
    pub window_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_field {
    pub regmap: *mut regmap,
    pub mask: c_uint,
// lsb
    pub shift: c_uint,
    pub reg: c_uint,
    pub id_size: c_uint,
    pub id_offset: c_uint,
}

extern "C" {
    pub fn regmap_debugfs_initcall();
}
extern "C" {
    pub fn regmap_debugfs_init(map: *mut regmap);
}
extern "C" {
    pub fn regmap_debugfs_exit(map: *mut regmap);
}

// regcache core declarations
extern "C" {
    pub fn regcache_init(map: *mut regmap, config: *const regmap_config) -> c_int;
}
extern "C" {
    pub fn regcache_exit(map: *mut regmap);
}
extern "C" {
    pub fn regcache_sync(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regcache_lookup_reg(map: *mut regmap, reg: c_uint) -> c_int;
}
extern "C" {
    pub fn regcache_sync_val(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_async_complete_cb(async: *mut regmap_async, ret: c_int);
}
extern "C" {
    pub fn dev_name(_arg: map->dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_ram_data {
    pub /: *mut *mut *mut unsigned int vals; / Allocatd by caller,
    pub read: *mut bool,
    pub written: *mut bool,
    pub reg_endian: regmap_endian,
    pub reg): *mut *mut *mut bool (noinc_reg)(struct regmap_ram_data data, unsigned int,
}

//
// Create a test register map with data stored in RAM, not intended
// for practical use.
//

