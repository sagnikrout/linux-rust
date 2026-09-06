//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/a20.c
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
// Copyright 2007-2008 rPath, Inc. - All Rights Reserved
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Enable A20 gate (return -1 on failure)
//

pub const MAX_8042_LOOPS: c_int = 100000;
pub const MAX_8042_FF: c_int = 32;
#[no_mangle]
unsafe extern "C" fn empty_8042() -> c_int {
    static int empty_8042(void)
    {
    u8 status;
    let mut loops: c_int = MAX_8042_LOOPS;
    let mut ffs: c_int = MAX_8042_FF;
    while (loops--) {
    io_delay();
    status = inb(0x64);
    if (status == 0xff) {
// FF is a plausible, but very unlikely status
    if (!--ffs)
    return -1; /* Assume no KBC present */
    }
    if (status & 1) {
// Read and discard input data
    io_delay();
    (void)inb(0x60);
    } else if (!(status & 2)) {
// Buffers empty, finished!
    return 0;
    }
    }
    return -1;
    }
// Returns nonzero if the A20 line is enabled.  The memory address
    used as a test is the int $0x80 vector, which should be safe. */

pub const A20_TEST_SHORT: c_int = 32;

#[no_mangle]
unsafe extern "C" fn a20_test(loops: c_int) -> c_int {
    static int a20_test(int loops)
    {
    let mut ok: c_int = 0;
    int saved, ctr;
    set_fs(0x0000);
    set_gs(0xffff);
    saved = ctr = rdfs32(A20_TEST_ADDR);
    while (loops--) {
    wrfs32(++ctr, A20_TEST_ADDR);
    io_delay();	/* Serialize and make delay constant */
    ok = rdgs32(A20_TEST_ADDR+0x10) ^ ctr;
    if (ok)
    break;
    }
    wrfs32(saved, A20_TEST_ADDR);
    return ok;
    }
// Quick test to see if A20 is already enabled
#[no_mangle]
unsafe extern "C" fn a20_test_short() -> c_int {
    static int a20_test_short(void)
    {
    return a20_test(A20_TEST_SHORT);
    }
// Longer test that actually waits for A20 to come on line; this
    is useful when dealing with the KBC or other slow external circuitry. */
#[no_mangle]
unsafe extern "C" fn a20_test_long() -> c_int {
    static int a20_test_long(void)
    {
    return a20_test(A20_TEST_LONG);
    }
#[no_mangle]
unsafe extern "C" fn enable_a20_bios() {
    static void enable_a20_bios(void)
    {
    struct biosregs ireg;
    initregs(&ireg);
    ireg.ax = 0x2401;
    intcall(0x15, &ireg, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn enable_a20_kbc() {
    static void enable_a20_kbc(void)
    {
    empty_8042();
    outb(0xd1, 0x64);	/* Command write */
    empty_8042();
    outb(0xdf, 0x60);	/* A20 on */
    empty_8042();
    outb(0xff, 0x64);	/* Null command, but UHCI wants it */
    empty_8042();
    }
#[no_mangle]
unsafe extern "C" fn enable_a20_fast() {
    static void enable_a20_fast(void)
    {
    u8 port_a;
    port_a = inb(0x92);	/* Configuration port A */
    port_a |=  0x02;	/* Enable A20 */
    port_a &= ~0x01;	/* Do not reset machine */
    outb(port_a, 0x92);
    }
//
// Actual routine to enable A20; return 0 on ok, -1 on failure
//

#[no_mangle]
pub unsafe extern "C" fn enable_a20() -> c_int {
    int enable_a20(void)
    {
    let mut loops: c_int = A20_ENABLE_LOOPS;
    int kbc_err;
    while (loops--) {
// First, check to see if A20 is already enabled
    (legacy free, etc.) */
    if (a20_test_short())
    return 0;
// Next, try the BIOS (INT 0x15, AX=0x2401)
    enable_a20_bios();
    if (a20_test_short())
    return 0;
// Try enabling A20 through the keyboard controller
    kbc_err = empty_8042();
    if (a20_test_short())
    return 0; /* BIOS worked, but with delayed reaction */
    if (!kbc_err) {
    enable_a20_kbc();
    if (a20_test_long())
    return 0;
    }
// Finally, try enabling the "fast A20 gate"
    enable_a20_fast();
    if (a20_test_long())
    return 0;
    }
    return -1;
    }
