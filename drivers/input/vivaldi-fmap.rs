//! Automatically rewritten from C to Rust
//! Source: drivers/input/vivaldi-fmap.c
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
// Helpers for ChromeOS Vivaldi keyboard function row mapping
//
// Copyright (C) 2022 Google, Inc
//

//
// vivaldi_function_row_physmap_show - Print vivaldi function row physmap attribute
// @data: The vivaldi function row map
// @buf: Buffer to print the function row phsymap to
//
    ssize_t vivaldi_function_row_physmap_show(const struct vivaldi_data *data,
    char *buf)
    {
    let mut size: isize = 0;
    int i;
    const u32 *physmap = data.function_row_physmap;
    if (!data.num_function_row_keys)
    return 0;
    for (i = 0; i < data.num_function_row_keys; i++)
    size += sysfs_emit_at(buf, size,
    "%s%02X", size ? " " : "", physmap[i]);
    if (size)
    size += sysfs_emit_at(buf, size, "\n");
    return size;
    }
    EXPORT_SYMBOL_GPL(vivaldi_function_row_physmap_show);
    MODULE_DESCRIPTION("Helpers for ChromeOS Vivaldi keyboard function row mapping");
    MODULE_LICENSE("GPL");
