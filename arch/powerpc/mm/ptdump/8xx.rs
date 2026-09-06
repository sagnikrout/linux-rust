//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/ptdump/8xx.c
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
// From split of dump_linuxpagetables.c
// Copyright 2016, Rashmica Gupta, IBM Corp.
//

    static const struct flag_info flag_array[] = {
    {

    .mask	= _PAGE_HUGE,
    .val	= _PAGE_HUGE,

    .mask	= _PAGE_SPS,
    .val	= _PAGE_SPS,

    .set	= "huge",
    .clear	= "    ",
    }, {
    .mask	= _PAGE_RO | _PAGE_NA,
    .val	= 0,
    .set	= "rw",
    }, {
    .mask	= _PAGE_RO | _PAGE_NA,
    .val	= _PAGE_RO,
    .set	= "r ",
    }, {
    .mask	= _PAGE_RO | _PAGE_NA,
    .val	= _PAGE_NA,
    .set	= "  ",
    }, {
    .mask	= _PAGE_EXEC,
    .val	= _PAGE_EXEC,
    .set	= " X ",
    .clear	= "   ",
    }, {
    .mask	= _PAGE_PRESENT,
    .val	= _PAGE_PRESENT,
    .set	= "present",
    .clear	= "       ",
    }, {
    .mask	= _PAGE_GUARDED,
    .val	= _PAGE_GUARDED,
    .set	= "guarded",
    .clear	= "       ",
    }, {
    .mask	= _PAGE_DIRTY,
    .val	= _PAGE_DIRTY,
    .set	= "dirty",
    .clear	= "     ",
    }, {
    .mask	= _PAGE_ACCESSED,
    .val	= _PAGE_ACCESSED,
    .set	= "accessed",
    .clear	= "        ",
    }, {
    .mask	= _PAGE_NO_CACHE,
    .val	= _PAGE_NO_CACHE,
    .set	= "no cache",
    .clear	= "        ",
    }, {
    .mask	= _PAGE_SPECIAL,
    .val	= _PAGE_SPECIAL,
    .set	= "special",
    }
    };
    struct ptdump_pg_level pg_level[5] = {
    { /* pgd */
    .name	= "PGD",
    .flag	= flag_array,
    .num	= ARRAY_SIZE(flag_array),
    }, { /* p4d */
    .name	= "P4D",
    .flag	= flag_array,
    .num	= ARRAY_SIZE(flag_array),
    }, { /* pud */
    .name	= "PUD",
    .flag	= flag_array,
    .num	= ARRAY_SIZE(flag_array),
    }, { /* pmd */
    .name	= "PMD",
    .flag	= flag_array,
    .num	= ARRAY_SIZE(flag_array),
    }, { /* pte */
    .name	= "PTE",
    .flag	= flag_array,
    .num	= ARRAY_SIZE(flag_array),
    },
    };
