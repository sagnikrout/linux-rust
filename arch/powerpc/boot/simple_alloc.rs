//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/simple_alloc.c
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
// Implement primitive realloc(3) functionality.
//
// Author: Mark A. Greer <mgreer@mvista.com>
//
// 2006 (c) MontaVista, Software, Inc.
//

pub const ENTRY_BEEN_USED: c_uint = 0x01;
pub const ENTRY_IN_USE: c_uint = 0x02;
    static struct alloc_info {
    unsigned long	flags;
    unsigned long	base;
    unsigned long	size;
    } *alloc_tbl;
    static unsigned long tbl_entries;
    static unsigned long alloc_min;
    static unsigned long next_base;
    static unsigned long space_left;
//
// First time an entry is used, its base and size are set.
// An entry can be freed and re-malloc'd but its base & size don't change.
// Should be smart enough for needs of bootwrapper.
//
    static void *simple_malloc(unsigned long size)
    {
    unsigned long i;
    struct alloc_info *p = alloc_tbl;
    if (size == 0)
    goto err_out;
    size = _ALIGN_UP(size, alloc_min);
    for (i=0; i<tbl_entries; i++, p++)
    if (!(p.flags & ENTRY_BEEN_USED)) { /* never been used */
    if (size <= space_left) {
    p.base = next_base;
    p.size = size;
    p.flags = ENTRY_BEEN_USED | ENTRY_IN_USE;
    next_base += size;
    space_left -= size;
    return (void *)p.base;
    }
    goto err_out; /* not enough space left */
    }
// reuse an entry keeping same base & size
#[no_mangle]
pub unsafe extern "C" fn if(p->size): !(p->flags & ENTRY_IN_USE) && (size <=) -> else {
    p.flags |= ENTRY_IN_USE;
    return (void *)p.base;
    }
    err_out:
    return core::ptr::null_mut();
    }
    static struct alloc_info *simple_find_entry(void *ptr)
    {
    unsigned long i;
    struct alloc_info *p = alloc_tbl;
    for (i=0; i<tbl_entries; i++,p++) {
    if (!(p.flags & ENTRY_BEEN_USED))
    break;
    if ((p.flags & ENTRY_IN_USE) &&
    (p.base == (unsigned long)ptr))
    return p;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn simple_free(ptr: *mut c_void) {
    static void simple_free(void *ptr)
    {
    struct alloc_info *p = simple_find_entry(ptr);
    if (p != core::ptr::null_mut())
    p.flags &= ~ENTRY_IN_USE;
    }
//
// Change size of area pointed to by 'ptr' to 'size'.
// If 'ptr' is NULL, then its a malloc().  If 'size' is 0, then its a free().
// 'ptr' must be NULL or a pointer to a non-freed area previously returned by
// simple_realloc() or simple_malloc().
//
    static void *simple_realloc(void *ptr, unsigned long size)
    {
    struct alloc_info *p;
    void *new;
    if (size == 0) {
    simple_free(ptr);
    return core::ptr::null_mut();
    }
    if (ptr == core::ptr::null_mut())
    return simple_malloc(size);
    p = simple_find_entry(ptr);
    if (p == core::ptr::null_mut()) /* ptr not from simple_malloc/simple_realloc */
    return core::ptr::null_mut();
    if (size <= p.size) /* fits in current block */
    return ptr;
    new = simple_malloc(size);
    if (new) {
    memcpy(new, ptr, p.size);
    simple_free(ptr);
    }
    return new;
    }
//
// Returns addr of first byte after heap so caller can see if it took
// too much space.  If so, change args & try again.
//
    void *simple_alloc_init(char *base, unsigned long heap_size,
    unsigned long granularity, unsigned long max_allocs)
    {
    unsigned long heap_base, tbl_size;
    heap_size = _ALIGN_UP(heap_size, granularity);
    alloc_min = granularity;
    tbl_entries = max_allocs;
    tbl_size = tbl_entries * sizeof(struct alloc_info);
    alloc_tbl = (struct alloc_info *)_ALIGN_UP((unsigned long)base, 8);
    memset(alloc_tbl, 0, tbl_size);
    heap_base = _ALIGN_UP((unsigned long)alloc_tbl + tbl_size, alloc_min);
    next_base = heap_base;
    space_left = heap_size;
    platform_ops.malloc = simple_malloc;
    platform_ops.free = simple_free;
    platform_ops.realloc = simple_realloc;
    return (void *)(heap_base + heap_size);
    }
