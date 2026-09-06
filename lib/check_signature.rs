//! Automatically rewritten from C to Rust
//! Source: lib/check_signature.c
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
// check_signature		-	find BIOS signatures
// @io_addr: mmio address to check
// @signature:  signature block
// @length: length of signature
//
// Perform a signature comparison with the mmio address io_addr. This
// address should have been obtained by ioremap.
// Returns 1 on a match.
//
    int check_signature(const volatile void __iomem *io_addr,
    const unsigned char *signature, int length)
    {
    while (length--) {
    if (readb(io_addr) != *signature)
    return 0;
    io_addr++;
    signature++;
    }
    return 1;
    }
    EXPORT_SYMBOL(check_signature);
