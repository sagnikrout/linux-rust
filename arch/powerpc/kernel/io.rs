//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/io.c
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
// I/O string operations
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
// Copyright (C) 2006 IBM Corporation
//
// Largely rewritten by Cort Dougan (cort@cs.nmt.edu)
// and Paul Mackerras.
//
// Adapted for iSeries by Mike Corrigan (mikejc@us.ibm.com)
// PPC64 updates by Dave Engebretsen (engebret@us.ibm.com)
//
// Rewritten in C by Stephen Rothwell.
//

// See definition in io.h
    bool isa_io_special;
#[no_mangle]
pub unsafe extern "C" fn _insb(port: *const volatile u8 __iomem, buf: *mut c_void, count: c_long) {
    void _insb(const volatile u8 __iomem *port, void *buf, long count)
    {
    u8 *tbuf = buf;
    u8 tmp;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
    tmp = *(const volatile u8  *)port;
    eieio();
// tbuf++ = tmp;
    } while (--count != 0);
    data_barrier(tmp);
    }
    EXPORT_SYMBOL(_insb);
#[no_mangle]
pub unsafe extern "C" fn _outsb(port: *mut volatile u8 __iomem, buf: *const c_void, count: c_long) {
    void _outsb(volatile u8 __iomem *port, const void *buf, long count)
    {
    const u8 *tbuf = buf;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
// (volatile u8  *)port = *tbuf++;
    } while (--count != 0);
    mb();
    }
    EXPORT_SYMBOL(_outsb);
#[no_mangle]
pub unsafe extern "C" fn _insw(port: *const volatile u16 __iomem, buf: *mut c_void, count: c_long) {
    void _insw(const volatile u16 __iomem *port, void *buf, long count)
    {
    u16 *tbuf = buf;
    u16 tmp;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
    tmp = *(const volatile u16  *)port;
    eieio();
// tbuf++ = tmp;
    } while (--count != 0);
    data_barrier(tmp);
    }
    EXPORT_SYMBOL(_insw);
#[no_mangle]
pub unsafe extern "C" fn _outsw(port: *mut volatile u16 __iomem, buf: *const c_void, count: c_long) {
    void _outsw(volatile u16 __iomem *port, const void *buf, long count)
    {
    const u16 *tbuf = buf;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
// (volatile u16  *)port = *tbuf++;
    } while (--count != 0);
    mb();
    }
    EXPORT_SYMBOL(_outsw);
#[no_mangle]
pub unsafe extern "C" fn _insl(port: *const volatile u32 __iomem, buf: *mut c_void, count: c_long) {
    void _insl(const volatile u32 __iomem *port, void *buf, long count)
    {
    u32 *tbuf = buf;
    u32 tmp;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
    tmp = *(const volatile u32  *)port;
    eieio();
// tbuf++ = tmp;
    } while (--count != 0);
    data_barrier(tmp);
    }
    EXPORT_SYMBOL(_insl);
#[no_mangle]
pub unsafe extern "C" fn _outsl(port: *mut volatile u32 __iomem, buf: *const c_void, count: c_long) {
    void _outsl(volatile u32 __iomem *port, const void *buf, long count)
    {
    const u32 *tbuf = buf;
    if (unlikely(count <= 0))
    return;
    mb();
    do {
// (volatile u32  *)port = *tbuf++;
    } while (--count != 0);
    mb();
    }
    EXPORT_SYMBOL(_outsl);

    notrace void
    _memset_io(volatile void __iomem *addr, int c, unsigned long n)
    {
    void *p = (void  *)addr;
    let mut lc: u32 = c;
    lc |= lc << 8;
    lc |= lc << 16;
    mb();
    while(n && !IO_CHECK_ALIGN(p, 4)) {
// ((volatile u8 *)p) = c;
    p++;
    n--;
    }
    while(n >= 4) {
// ((volatile u32 *)p) = lc;
    p += 4;
    n -= 4;
    }
    while(n) {
// ((volatile u8 *)p) = c;
    p++;
    n--;
    }
    mb();
    }
    EXPORT_SYMBOL(_memset_io);
    void _memcpy_fromio(void *dest, const volatile void __iomem *src,
    unsigned long n)
    {
    void *vsrc = (void  *) src;
    mb();
    while(n && (!IO_CHECK_ALIGN(vsrc, 4) || !IO_CHECK_ALIGN(dest, 4))) {
// ((u8 *)dest) = *((volatile u8 *)vsrc);
    eieio();
    vsrc++;
    dest++;
    n--;
    }
    while(n >= 4) {
// ((u32 *)dest) = *((volatile u32 *)vsrc);
    eieio();
    vsrc += 4;
    dest += 4;
    n -= 4;
    }
    while(n) {
// ((u8 *)dest) = *((volatile u8 *)vsrc);
    eieio();
    vsrc++;
    dest++;
    n--;
    }
    mb();
    }
    EXPORT_SYMBOL(_memcpy_fromio);
#[no_mangle]
pub unsafe extern "C" fn _memcpy_toio(dest: *mut volatile void __iomem, src: *const c_void, n: c_ulong) {
    void _memcpy_toio(volatile void __iomem *dest, const void *src, unsigned long n)
    {
    void *vdest = (void  *) dest;
    mb();
    while(n && (!IO_CHECK_ALIGN(vdest, 4) || !IO_CHECK_ALIGN(src, 4))) {
// ((volatile u8 *)vdest) = *((u8 *)src);
    src++;
    vdest++;
    n--;
    }
    while(n >= 4) {
// ((volatile u32 *)vdest) = *((volatile u32 *)src);
    src += 4;
    vdest += 4;
    n-=4;
    }
    while(n) {
// ((volatile u8 *)vdest) = *((u8 *)src);
    src++;
    vdest++;
    n--;
    }
    mb();
    }
    EXPORT_SYMBOL(_memcpy_toio);
