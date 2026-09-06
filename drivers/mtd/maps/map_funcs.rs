//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/map_funcs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Out-of-line map I/O functions for simple maps when CONFIG_MTD_COMPLEX_MAPPINGS
// is enabled.
//

#[no_mangle]
unsafe extern "C" fn simple_map_read(map: *mut map_info, ofs: c_ulong) -> map_word __xipram {
    static map_word __xipram simple_map_read(struct map_info *map, unsigned long ofs)
    {
    return inline_map_read(map, ofs);
    }
#[no_mangle]
unsafe extern "C" fn simple_map_write(map: *mut map_info, datum: map_word, ofs: c_ulong) -> void __xipram {
    static void __xipram simple_map_write(struct map_info *map, const map_word datum, unsigned long ofs)
    {
    inline_map_write(map, datum, ofs);
    }
#[no_mangle]
unsafe extern "C" fn simple_map_copy_from(map: *mut map_info, to: *mut c_void, from: c_ulong, len: isize) -> void __xipram {
    static void __xipram simple_map_copy_from(struct map_info *map, void *to, unsigned long from, ssize_t len)
    {
    inline_map_copy_from(map, to, from, len);
    }
#[no_mangle]
unsafe extern "C" fn simple_map_copy_to(map: *mut map_info, to: c_ulong, from: *const c_void, len: isize) -> void __xipram {
    static void __xipram simple_map_copy_to(struct map_info *map, unsigned long to, const void *from, ssize_t len)
    {
    inline_map_copy_to(map, to, from, len);
    }
#[no_mangle]
pub unsafe extern "C" fn simple_map_init(map: *mut map_info) {
    void simple_map_init(struct map_info *map)
    {
    BUG_ON(!map_bankwidth_supported(map.bankwidth));
    map.read = simple_map_read;
    map.write = simple_map_write;
    map.copy_from = simple_map_copy_from;
    map.copy_to = simple_map_copy_to;
    }
    EXPORT_SYMBOL(simple_map_init);
    MODULE_DESCRIPTION("Out-of-line map I/O");
    MODULE_LICENSE("GPL");
