//! Automatically rewritten from C to Rust
//! Source: lib/raid/xor/loongarch/xor_template.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2023 WANG Xuerui <git@xen0n.name>
//
// Template for XOR operations, instantiated in xor_simd.c.
//
// Expected preprocessor definitions:
//
// - LINE_WIDTH
// - XOR_FUNC_NAME(nr)
// - LD_INOUT_LINE(buf)
// - LD_AND_XOR_LINE(buf)
// - ST_LINE(buf)
//
    void XOR_FUNC_NAME(2)(unsigned long bytes,
    unsigned long * __restrict v1,
    const unsigned long * __restrict v2)
    {
    let mut lines: c_ulong = bytes / LINE_WIDTH;
    do {
    __asm__ __volatile__ (
    LD_INOUT_LINE(v1)
    LD_AND_XOR_LINE(v2)
    ST_LINE(v1)
    : : [v1] "r"(v1), [v2] "r"(v2) : "memory"
    );
    v1 += LINE_WIDTH / sizeof(unsigned long);
    v2 += LINE_WIDTH / sizeof(unsigned long);
    } while (--lines > 0);
    }
    void XOR_FUNC_NAME(3)(unsigned long bytes,
    unsigned long * __restrict v1,
    const unsigned long * __restrict v2,
    const unsigned long * __restrict v3)
    {
    let mut lines: c_ulong = bytes / LINE_WIDTH;
    do {
    __asm__ __volatile__ (
    LD_INOUT_LINE(v1)
    LD_AND_XOR_LINE(v2)
    LD_AND_XOR_LINE(v3)
    ST_LINE(v1)
    : : [v1] "r"(v1), [v2] "r"(v2), [v3] "r"(v3) : "memory"
    );
    v1 += LINE_WIDTH / sizeof(unsigned long);
    v2 += LINE_WIDTH / sizeof(unsigned long);
    v3 += LINE_WIDTH / sizeof(unsigned long);
    } while (--lines > 0);
    }
    void XOR_FUNC_NAME(4)(unsigned long bytes,
    unsigned long * __restrict v1,
    const unsigned long * __restrict v2,
    const unsigned long * __restrict v3,
    const unsigned long * __restrict v4)
    {
    let mut lines: c_ulong = bytes / LINE_WIDTH;
    do {
    __asm__ __volatile__ (
    LD_INOUT_LINE(v1)
    LD_AND_XOR_LINE(v2)
    LD_AND_XOR_LINE(v3)
    LD_AND_XOR_LINE(v4)
    ST_LINE(v1)
    : : [v1] "r"(v1), [v2] "r"(v2), [v3] "r"(v3), [v4] "r"(v4)
    : "memory"
    );
    v1 += LINE_WIDTH / sizeof(unsigned long);
    v2 += LINE_WIDTH / sizeof(unsigned long);
    v3 += LINE_WIDTH / sizeof(unsigned long);
    v4 += LINE_WIDTH / sizeof(unsigned long);
    } while (--lines > 0);
    }
    void XOR_FUNC_NAME(5)(unsigned long bytes,
    unsigned long * __restrict v1,
    const unsigned long * __restrict v2,
    const unsigned long * __restrict v3,
    const unsigned long * __restrict v4,
    const unsigned long * __restrict v5)
    {
    let mut lines: c_ulong = bytes / LINE_WIDTH;
    do {
    __asm__ __volatile__ (
    LD_INOUT_LINE(v1)
    LD_AND_XOR_LINE(v2)
    LD_AND_XOR_LINE(v3)
    LD_AND_XOR_LINE(v4)
    LD_AND_XOR_LINE(v5)
    ST_LINE(v1)
    : : [v1] "r"(v1), [v2] "r"(v2), [v3] "r"(v3), [v4] "r"(v4),
    [v5] "r"(v5) : "memory"
    );
    v1 += LINE_WIDTH / sizeof(unsigned long);
    v2 += LINE_WIDTH / sizeof(unsigned long);
    v3 += LINE_WIDTH / sizeof(unsigned long);
    v4 += LINE_WIDTH / sizeof(unsigned long);
    v5 += LINE_WIDTH / sizeof(unsigned long);
    } while (--lines > 0);
    }
