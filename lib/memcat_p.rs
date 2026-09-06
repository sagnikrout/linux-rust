//! Automatically rewritten from C to Rust
//! Source: lib/memcat_p.c
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
// Merge two NULL-terminated pointer arrays into a newly allocated
// array, which is also NULL-terminated. Nomenclature is inspired by
// memset_p() and memcat() found elsewhere in the kernel source tree.
//
    void **__memcat_p(void **a, void **b)
    {
    void **p = a, **new;
    int nr;
// count the elements in both arrays
    for (nr = 0, p = a; *p; nr++, p++)
    ;
    for (p = b; *p; nr++, p++)
    ;
// one for the NULL-terminator
    nr++;
    new = kmalloc_array(nr, sizeof(void *), GFP_KERNEL);
    if (!new)
    return core::ptr::null_mut();
// nr -> last index; p points to NULL in b[]
    for (nr--; nr >= 0; nr--, p = p == b ? &a[nr] : p - 1)
    new[nr] = *p;
    return new;
    }
    EXPORT_SYMBOL_GPL(__memcat_p);
