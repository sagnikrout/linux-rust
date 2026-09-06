//! Automatically rewritten from C to Rust
//! Source: lib/kasprintf.c
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
// linux/lib/kasprintf.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//

// Simplified asprintf.
    char *kvasprintf(gfp_t gfp, const char *fmt, va_list ap)
    {
    unsigned int first, second;
    char *p;
    va_list aq;
    va_copy(aq, ap);
    first = vsnprintf(core::ptr::null_mut(), 0, fmt, aq);
    va_end(aq);
    p = kmalloc_track_caller(first+1, gfp);
    if (!p)
    return core::ptr::null_mut();
    second = vsnprintf(p, first+1, fmt, ap);
    WARN(first != second, "different return values (%u and %u) from vsnprintf(\"%s\", ...)",
    first, second, fmt);
    return p;
    }
    EXPORT_SYMBOL(kvasprintf);
//
// If fmt contains no % (or is exactly %s), use kstrdup_const. If fmt
// (or the sole vararg) points to rodata, we will then save a memory
// allocation and string copy. In any case, the return value should be
// freed using kfree_const().
//
    const char *kvasprintf_const(gfp_t gfp, const char *fmt, va_list ap)
    {
    if (!strchr(fmt, '%'))
    return kstrdup_const(fmt, gfp);
    if (!strcmp(fmt, "%s"))
    return kstrdup_const(va_arg(ap, const char*), gfp);
    return kvasprintf(gfp, fmt, ap);
    }
    EXPORT_SYMBOL(kvasprintf_const);
    char *kasprintf(gfp_t gfp, const char *fmt, ...)
    {
    va_list ap;
    char *p;
    va_start(ap, fmt);
    p = kvasprintf(gfp, fmt, ap);
    va_end(ap);
    return p;
    }
    EXPORT_SYMBOL(kasprintf);
