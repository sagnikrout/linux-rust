//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/iomem.c
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


#[no_mangle]
pub unsafe extern "C" fn volatile((to): "movs" type:"=&D", (to): "=&S" (from):"0", (from):"memory": "1") -> asm {
    asm volatile("movs" type:"=&D" (to), "=&S" (from):"0" (to), "1" (from):"memory")
// Originally from i386/string.h
#[no_mangle]
unsafe extern "C" fn rep_movs(to: *mut c_void, from: *const c_void, n: usize) -> __always_inline void {
    static __always_inline void rep_movs(void *to, const void *from, size_t n)
    {
    unsigned long d0, d1, d2;
    asm volatile("rep movsl\n\t"
    "testb $2,%b4\n\t"
    "je 1f\n\t"
    "movsw\n"
    "1:\ttestb $1,%b4\n\t"
    "je 2f\n\t"
    "movsb\n"
    "2:"
    : "=&c" (d0), "=&D" (d1), "=&S" (d2)
    : "0" (n / 4), "q" (n), "1" ((long)to), "2" ((long)from)
    : "memory");
    }
#[no_mangle]
unsafe extern "C" fn string_memcpy_fromio(to: *mut c_void, from: *const volatile void __iomem, n: usize) {
    static void string_memcpy_fromio(void *to, const volatile void __iomem *from, size_t n)
    {
    const void *orig_to = to;
    let mut orig_n: usize = n;
    if (unlikely(!n))
    return;
// Align any unaligned source IO
    if (unlikely(1 & (unsigned long)from)) {
    movs("b", to, from);
    n--;
    }
    if (n > 1 && unlikely(2 & (unsigned long)from)) {
    movs("w", to, from);
    n-=2;
    }
    rep_movs(to, (const void *)from, n);
// KMSAN must treat values read from devices as initialized.
    kmsan_unpoison_memory(orig_to, orig_n);
    }
#[no_mangle]
unsafe extern "C" fn string_memcpy_toio(to: *mut volatile void __iomem, from: *const c_void, n: usize) {
    static void string_memcpy_toio(volatile void __iomem *to, const void *from, size_t n)
    {
    if (unlikely(!n))
    return;
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(from, n);
// Align any unaligned destination IO
    if (unlikely(1 & (unsigned long)to)) {
    movs("b", to, from);
    n--;
    }
    if (n > 1 && unlikely(2 & (unsigned long)to)) {
    movs("w", to, from);
    n-=2;
    }
    rep_movs((void *)to, (const void *) from, n);
    }
#[no_mangle]
unsafe extern "C" fn unrolled_memcpy_fromio(to: *mut c_void, from: *const volatile void __iomem, n: usize) {
    static void unrolled_memcpy_fromio(void *to, const volatile void __iomem *from, size_t n)
    {
    const volatile char __iomem *in = from;
    char *out = to;
    int i;
    for (i = 0; i < n; ++i)
    out[i] = readb(&in[i]);
    }
#[no_mangle]
unsafe extern "C" fn unrolled_memcpy_toio(to: *mut volatile void __iomem, from: *const c_void, n: usize) {
    static void unrolled_memcpy_toio(volatile void __iomem *to, const void *from, size_t n)
    {
    volatile char __iomem *out = to;
    const char *in = from;
    int i;
    for (i = 0; i < n; ++i)
    writeb(in[i], &out[i]);
    }
#[no_mangle]
unsafe extern "C" fn unrolled_memset_io(a: *mut volatile void __iomem, b: c_int, c: usize) {
    static void unrolled_memset_io(volatile void __iomem *a, int b, size_t c)
    {
    volatile char __iomem *mem = a;
    int i;
    for (i = 0; i < c; ++i)
    writeb(b, &mem[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn memcpy_fromio(to: *mut c_void, from: *const volatile void __iomem, n: usize) {
    void memcpy_fromio(void *to, const volatile void __iomem *from, size_t n)
    {
    if (cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO))
    unrolled_memcpy_fromio(to, from, n);
    else
    string_memcpy_fromio(to, from, n);
    }
    EXPORT_SYMBOL(memcpy_fromio);
#[no_mangle]
pub unsafe extern "C" fn memcpy_toio(to: *mut volatile void __iomem, from: *const c_void, n: usize) {
    void memcpy_toio(volatile void __iomem *to, const void *from, size_t n)
    {
    if (cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO))
    unrolled_memcpy_toio(to, from, n);
    else
    string_memcpy_toio(to, from, n);
    }
    EXPORT_SYMBOL(memcpy_toio);
#[no_mangle]
pub unsafe extern "C" fn memset_io(a: *mut volatile void __iomem, b: c_int, c: usize) {
    void memset_io(volatile void __iomem *a, int b, size_t c)
    {
    if (cc_platform_has(CC_ATTR_GUEST_UNROLL_STRING_IO)) {
    unrolled_memset_io(a, b, c);
    } else {
//
// TODO: memset can mangle the IO patterns quite a bit.
// perhaps it would be better to use a dumb one:
//
    memset((void *)a, b, c);
    }
    }
    EXPORT_SYMBOL(memset_io);
