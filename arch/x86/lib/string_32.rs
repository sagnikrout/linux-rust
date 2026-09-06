//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/string_32.c
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
// Most of the string-functions are rather heavily hand-optimized,
// see especially strsep,strstr,str[c]spn. They should work, but are not
// very easy to understand. Everything is done entirely within the register
// set, making the functions fast and clean. String instructions have been
// used through-out, making for "slightly" unclear code :-)
//
// AK: On P4 and K7 using non string instruction implementations might be faster
// for large memory blocks. But most of them are unlikely to be used on large
// strings.
//
// Macro flag: #define __NO_FORTIFY

    char *strcpy(char *dest, const char *src)
    {
    int d0, d1, d2;
    asm volatile("1:\tlodsb\n\t"
    "stosb\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b"
    : "=&S" (d0), "=&D" (d1), "=&a" (d2)
    : "0" (src), "1" (dest) : "memory");
    return dest;
    }
    EXPORT_SYMBOL(strcpy);

    char *strcat(char *dest, const char *src)
    {
    int d0, d1, d2, d3;
    asm volatile("repne scasb\n\t"
    "decl %1\n"
    "1:\tlodsb\n\t"
    "stosb\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b"
    : "=&S" (d0), "=&D" (d1), "=&a" (d2), "=&c" (d3)
    : "0" (src), "1" (dest), "2" (0), "3" (0xffffffffu) : "memory");
    return dest;
    }
    EXPORT_SYMBOL(strcat);

    char *strncat(char *dest, const char *src, size_t count)
    {
    int d0, d1, d2, d3;
    asm volatile("repne scasb\n\t"
    "decl %1\n\t"
    "movl %8,%3\n"
    "1:\tdecl %3\n\t"
    "js 2f\n\t"
    "lodsb\n\t"
    "stosb\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b\n"
    "2:\txorl %2,%2\n\t"
    "stosb"
    : "=&S" (d0), "=&D" (d1), "=&a" (d2), "=&c" (d3)
    : "0" (src), "1" (dest), "2" (0), "3" (0xffffffffu), "g" (count)
    : "memory");
    return dest;
    }
    EXPORT_SYMBOL(strncat);

#[no_mangle]
pub unsafe extern "C" fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int {
    int strcmp(const char *cs, const char *ct)
    {
    int d0, d1;
    int res;
    asm volatile("1:\tlodsb\n\t"
    "scasb\n\t"
    "jne 2f\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b\n\t"
    "xorl %%eax,%%eax\n\t"
    "jmp 3f\n"
    "2:\tsbbl %%eax,%%eax\n\t"
    "orb $1,%%al\n"
    "3:"
    : "=a" (res), "=&S" (d0), "=&D" (d1)
    : "1" (cs), "2" (ct)
    : "memory");
    return res;
    }
    EXPORT_SYMBOL(strcmp);

#[no_mangle]
pub unsafe extern "C" fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int {
    int strncmp(const char *cs, const char *ct, size_t count)
    {
    int res;
    int d0, d1, d2;
    asm volatile("1:\tdecl %3\n\t"
    "js 2f\n\t"
    "lodsb\n\t"
    "scasb\n\t"
    "jne 3f\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b\n"
    "2:\txorl %%eax,%%eax\n\t"
    "jmp 4f\n"
    "3:\tsbbl %%eax,%%eax\n\t"
    "orb $1,%%al\n"
    "4:"
    : "=a" (res), "=&S" (d0), "=&D" (d1), "=&c" (d2)
    : "1" (cs), "2" (ct), "3" (count)
    : "memory");
    return res;
    }
    EXPORT_SYMBOL(strncmp);

    char *strchr(const char *s, int c)
    {
    int d0;
    char *res;
    asm volatile("movb %%al,%%ah\n"
    "1:\tlodsb\n\t"
    "cmpb %%ah,%%al\n\t"
    "je 2f\n\t"
    "testb %%al,%%al\n\t"
    "jne 1b\n\t"
    "movl $1,%1\n"
    "2:\tmovl %1,%0\n\t"
    "decl %0"
    : "=a" (res), "=&S" (d0)
    : "1" (s), "0" (c)
    : "memory");
    return res;
    }
    EXPORT_SYMBOL(strchr);

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> usize {
    size_t strlen(const char *s)
    {
    int d0;
    size_t res;
    asm volatile("repne scasb"
    : "=c" (res), "=&D" (d0)
    : "1" (s), "a" (0), "0" (0xffffffffu)
    : "memory");
    return ~res - 1;
    }
    EXPORT_SYMBOL(strlen);

    void *memchr(const void *cs, int c, size_t count)
    {
    int d0;
    void *res;
    if (!count)
    return core::ptr::null_mut();
    asm volatile("repne scasb\n\t"
    "je 1f\n\t"
    "movl $1,%0\n"
    "1:\tdecl %0"
    : "=D" (res), "=&c" (d0)
    : "a" (c), "0" (cs), "1" (count)
    : "memory");
    return res;
    }
    EXPORT_SYMBOL(memchr);

    void *memscan(void *addr, int c, size_t size)
    {
    if (!size)
    return addr;
    asm volatile("repnz scasb\n\t"
    "jnz 1f\n\t"
    "dec %%edi\n"
    "1:"
    : "=D" (addr), "=c" (size)
    : "0" (addr), "1" (size), "a" (c)
    : "memory");
    return addr;
    }
    EXPORT_SYMBOL(memscan);

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const c_char, count: usize) -> usize {
    size_t strnlen(const char *s, size_t count)
    {
    int d0;
    int res;
    asm volatile("movl %2,%0\n\t"
    "jmp 2f\n"
    "1:\tcmpb $0,(%0)\n\t"
    "je 3f\n\t"
    "incl %0\n"
    "2:\tdecl %1\n\t"
    "cmpl $-1,%1\n\t"
    "jne 1b\n"
    "3:\tsubl %2,%0"
    : "=a" (res), "=&d" (d0)
    : "c" (s), "1" (count)
    : "memory");
    return res;
    }
    EXPORT_SYMBOL(strnlen);
