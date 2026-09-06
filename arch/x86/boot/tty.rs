//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/tty.c
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
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Very simple screen and serial I/O
//

    int early_serial_base;
pub const XMTRDY: c_uint = 0x20;

//
// These functions are in .inittext so they can be used to signal
// error during initialization.
//
#[no_mangle]
unsafe extern "C" fn __section(ch: ".inittext") serial_putchar(int) {
    static void __section(".inittext") serial_putchar(int ch)
    {
    let mut timeout: unsigned = 0xffff;
    while ((inb(early_serial_base + LSR) & XMTRDY) == 0 && --timeout)
    cpu_relax();
    outb(ch, early_serial_base + TXR);
    }
#[no_mangle]
unsafe extern "C" fn __section(ch: ".inittext") bios_putchar(int) {
    static void __section(".inittext") bios_putchar(int ch)
    {
    struct biosregs ireg;
    initregs(&ireg);
    ireg.bx = 0x0007;
    ireg.cx = 0x0001;
    ireg.ah = 0x0e;
    ireg.al = ch;
    intcall(0x10, &ireg, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn __section(ch: ".inittext") putchar(int) {
    void __section(".inittext") putchar(int ch)
    {
    if (ch == '\n')
    putchar('\r');	/* \n . \r\n */
    bios_putchar(ch);
    if (early_serial_base != 0)
    serial_putchar(ch);
    }
#[no_mangle]
pub unsafe extern "C" fn __section(str: *const ".inittext") puts(char) {
    void __section(".inittext") puts(const char *str)
    {
    while (*str)
    putchar(*str++);
    }
//
// Read the CMOS clock through the BIOS, and return the
// seconds in BCD.
//
#[no_mangle]
unsafe extern "C" fn gettime() -> u8 {
    static u8 gettime(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ah = 0x02;
    intcall(0x1a, &ireg, &oreg);
    return oreg.dh;
    }
//
// Read from the keyboard
//
#[no_mangle]
pub unsafe extern "C" fn getchar() -> c_int {
    int getchar(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
// ireg.ah = 0x00;
    intcall(0x16, &ireg, &oreg);
    return oreg.al;
    }
#[no_mangle]
unsafe extern "C" fn kbd_pending() -> c_int {
    static int kbd_pending(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ah = 0x01;
    intcall(0x16, &ireg, &oreg);
    return !(oreg.eflags & X86_EFLAGS_ZF);
    }
#[no_mangle]
pub unsafe extern "C" fn kbd_flush() {
    void kbd_flush(void)
    {
    for (;;) {
    if (!kbd_pending())
    break;
    getchar();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn getchar_timeout() -> c_int {
    int getchar_timeout(void)
    {
    let mut cnt: c_int = 30;
    int t0, t1;
    t0 = gettime();
    while (cnt) {
    if (kbd_pending())
    return getchar();
    t1 = gettime();
    if (t0 != t1) {
    cnt--;
    t0 = t1;
    }
    }
    return 0;		/* Timeout! */
    }
