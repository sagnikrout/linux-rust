//! Automatically rewritten from C to Rust
//! Source: mm/kasan/report_sw_tags.c
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
// This file contains software tag-based KASAN specific error reporting code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

    const void *kasan_find_first_bad_addr(const void *addr, size_t size)
    {
    let mut tag: u8 = get_tag(addr);
    void *p = kasan_reset_tag(addr);
    void *end = p + size;
    if (!addr_has_metadata(p))
    return p;
    while (p < end && tag == *(u8 *)kasan_mem_to_shadow(p))
    p += KASAN_GRANULE_SIZE;
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize {
    size_t kasan_get_alloc_size(void *object, struct kmem_cache *cache)
    {
    let mut size: usize = 0;
    u8 *shadow;
//
// Skip the addr_has_metadata check, as this function only operates on
// slab memory, which must have metadata.
//
// The loop below returns 0 for freed objects, for which KASAN cannot
// calculate the allocation size based on the metadata.
//
    shadow = (u8 *)kasan_mem_to_shadow(object);
    while (size < cache.object_size) {
    if (*shadow != KASAN_TAG_INVALID)
    size += KASAN_GRANULE_SIZE;
    else
    return size;
    shadow++;
    }
    return cache.object_size;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void) {
    void kasan_metadata_fetch_row(char *buffer, void *row)
    {
    memcpy(buffer, kasan_mem_to_shadow(row), META_BYTES_PER_ROW);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_print_tags(addr_tag: u8, addr: *const c_void) {
    void kasan_print_tags(u8 addr_tag, const void *addr)
    {
    u8 *shadow = (u8 *)kasan_mem_to_shadow(addr);
    pr_err("Pointer tag: [%02x], memory tag: [%02x]\n", addr_tag, *shadow);
    }

#[no_mangle]
pub unsafe extern "C" fn kasan_print_address_stack_frame(addr: *const c_void) {
    void kasan_print_address_stack_frame(const void *addr)
    {
    if (WARN_ON(!object_is_on_stack(addr)))
    return;
    pr_err("The buggy address belongs to stack of task %s/%d\n",
    current.comm, task_pid_nr(current));
    }
