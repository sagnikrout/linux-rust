//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/csum-wrappers_64.c
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
// Copyright 2002, 2003 Andi Kleen, SuSE Labs.
//
// Wrappers of assembly checksum functions for x86-64.
//

//
// csum_and_copy_from_user - Copy and checksum from user space.
// @src: source address (user space)
// @dst: destination address
// @len: number of bytes to be copied.
//
// Returns an 32bit unfolded checksum of the buffer.
// src and dst are best aligned to 64bits.
//
    __wsum
    csum_and_copy_from_user(const void __user *src, void *dst, int len)
    {
    __wsum sum;
    might_sleep();
    if (!user_access_begin(src, len))
    return 0;
    sum = csum_partial_copy_generic(( const void *)src, dst, len);
    user_access_end();
    return sum;
    }
//
// csum_and_copy_to_user - Copy and checksum to user space.
// @src: source address
// @dst: destination address (user space)
// @len: number of bytes to be copied.
//
// Returns an 32bit unfolded checksum of the buffer.
// src and dst are best aligned to 64bits.
//
    __wsum
    csum_and_copy_to_user(const void *src, void __user *dst, int len)
    {
    __wsum sum;
    might_sleep();
    if (!user_access_begin(dst, len))
    return 0;
    sum = csum_partial_copy_generic(src, (void  *)dst, len);
    user_access_end();
    return sum;
    }
//
// csum_partial_copy_nocheck - Copy and checksum.
// @src: source address
// @dst: destination address
// @len: number of bytes to be copied.
//
// Returns an 32bit unfolded checksum of the buffer.
//
    __wsum
    csum_partial_copy_nocheck(const void *src, void *dst, int len)
    {
    return csum_partial_copy_generic(src, dst, len);
    }
    EXPORT_SYMBOL(csum_partial_copy_nocheck);
