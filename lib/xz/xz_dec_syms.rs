//! Automatically rewritten from C to Rust
//! Source: lib/xz/xz_dec_syms.c
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


// SPDX-License-Identifier: 0BSD
//
// XZ decoder module information
//
// Author: Lasse Collin <lasse.collin@tukaani.org>
//

    EXPORT_SYMBOL(xz_dec_init);
    EXPORT_SYMBOL(xz_dec_reset);
    EXPORT_SYMBOL(xz_dec_run);
    EXPORT_SYMBOL(xz_dec_end);

    EXPORT_SYMBOL(xz_dec_microlzma_alloc);
    EXPORT_SYMBOL(xz_dec_microlzma_reset);
    EXPORT_SYMBOL(xz_dec_microlzma_run);
    EXPORT_SYMBOL(xz_dec_microlzma_end);

    MODULE_DESCRIPTION("XZ decompressor");
    MODULE_VERSION("1.2");
    MODULE_AUTHOR("Lasse Collin <lasse.collin@tukaani.org> and Igor Pavlov");
    MODULE_LICENSE("Dual BSD/GPL");
