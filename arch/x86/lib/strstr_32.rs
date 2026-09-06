//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/strstr_32.c
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

    char *strstr(const char *cs, const char *ct)
    {
    int	d0, d1;
    register char *__res;
    __asm__ __volatile__(
    "movl %6,%%edi\n\t"
    "repne scasb\n\t"
    "notl %%ecx\n\t"
    "decl %%ecx\n\t"	/* NOTE! This also sets Z if searchstring='' */
    "movl %%ecx,%%edx\n"
    "1:\tmovl %6,%%edi\n\t"
    "movl %%esi,%%eax\n\t"
    "movl %%edx,%%ecx\n\t"
    "repe cmpsb\n\t"
    "je 2f\n\t"		/* also works for empty string, see above */
    "xchgl %%eax,%%esi\n\t"
    "incl %%esi\n\t"
    "cmpb $0,-1(%%eax)\n\t"
    "jne 1b\n\t"
    "xorl %%eax,%%eax\n\t"
    "2:"
    : "=a" (__res), "=&c" (d0), "=&S" (d1)
    : "0" (0), "1" (0xffffffff), "2" (cs), "g" (ct)
    : "dx", "di");
    return __res;
    }
    EXPORT_SYMBOL(strstr);
