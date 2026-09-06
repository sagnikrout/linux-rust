//! Automatically rewritten from C to Rust
//! Source: arch/um/os-Linux/user_syms.c
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
// Macro flag: #define __NO_FORTIFY

//
// This file exports some critical string functions and compiler
// built-in functions (where calls are emitted by the compiler
// itself that we cannot avoid even in kernel code) to modules.
//
// "_user.c" code that previously used exports here such as hostfs
// really should be considered part of the 'hypervisor' and define
// its own API boundary like hostfs does now; don't add exports to
// this file for such cases.
//
// If it's not defined, the export is included in lib/string.c.

    EXPORT_SYMBOL(strstr);

    extern void *memcpy(void *, const void *, size_t);
    EXPORT_SYMBOL(memcpy);
    extern void *memmove(void *, const void *, size_t);
    EXPORT_SYMBOL(memmove);

    extern void *memset(void *, int, size_t);
    EXPORT_SYMBOL(memset);

    extern int __sprintf_chk(char *str, int flag, size_t len, const char *format);
    EXPORT_SYMBOL(__sprintf_chk);
