//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/mkcpustr.c
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
// -----------------------------------------------------------------------
//
// Copyright 2008 rPath, Inc. - All Rights Reserved
//
// -----------------------------------------------------------------------
//
// This is a host program to preprocess the CPU strings into a
// compact format suitable for the setup code.
//

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    int i, j;
    const char *str;
    printf("#include <asm/cpufeaturemasks.h>\n\n");
    printf("static const char x86_cap_strs[] =\n");
    for (i = 0; i < NCAPINTS; i++) {
    for (j = 0; j < 32; j++) {
    str = x86_cap_flags[i*32+j];
    if (i == NCAPINTS-1 && j == 31) {
// The last entry must be unconditional; this
    also consumes the compiler-added null
    character */
    if (!str)
    str = "";
    printf("\t\"\\x%02x\\x%02x\"\"%s\"\n",
    i, j, str);
    } else if (str) {
    printf("#if REQUIRED_MASK%d & (1 << %d)\n"
    "\t\"\\x%02x\\x%02x\"\"%s\\0\"\n"
    "#endif\n",
    i, j, i, j, str);
    }
    }
    }
    printf("\t;\n");
    return 0;
    }
