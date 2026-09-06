//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/memory.c
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
// Memory detection code
//

pub const SMAP: c_uint = 0x534d4150	/* ASCII "SMAP" */;
#[no_mangle]
unsafe extern "C" fn detect_memory_e820() {
    static void detect_memory_e820(void)
    {
    let mut count: c_int = 0;
    struct biosregs ireg, oreg;
    struct boot_e820_entry *desc = boot_params.e820_table;
    static struct boot_e820_entry buf; /* static so it is zeroed */
    initregs(&ireg);
    ireg.ax  = 0xe820;
    ireg.cx  = sizeof(buf);
    ireg.edx = SMAP;
    ireg.di  = (size_t)&buf;
//
// Note: at least one BIOS is known which assumes that the
// buffer pointed to by one e820 call is the same one as
// the previous call, and only changes modified fields.  Therefore,
// we use a temporary buffer and copy the results entry by entry.
//
// This routine deliberately does not try to account for
// ACPI 3+ extended attributes.  This is because there are
// BIOSes in the field which report zero for the valid bit for
// all ranges, and we don't currently make any use of the
// other attribute bits.  Revisit this if we see the extended
// attribute bits deployed in a meaningful way in the future.
//
    do {
    intcall(0x15, &ireg, &oreg);
    ireg.ebx = oreg.ebx; /* for next iteration... */
// BIOSes which terminate the chain with CF = 1 as opposed
    to %ebx = 0 don't always report the SMAP signature on
    the final, failing, probe. */
    if (oreg.eflags & X86_EFLAGS_CF)
    break;
// Some BIOSes stop returning SMAP in the middle of
    the search loop.  We don't know exactly how the BIOS
    screwed up the map at that point, we might have a
    partial map, the full map, or complete garbage, so
    just return failure. */
    if (oreg.eax != SMAP) {
    count = 0;
    break;
    }
// desc++ = buf;
    count++;
    } while (ireg.ebx && count < ARRAY_SIZE(boot_params.e820_table));
    boot_params.e820_entries = count;
    }
#[no_mangle]
unsafe extern "C" fn detect_memory_e801() {
    static void detect_memory_e801(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ax = 0xe801;
    intcall(0x15, &ireg, &oreg);
    if (oreg.eflags & X86_EFLAGS_CF)
    return;
// Do we really need to do this?
    if (oreg.cx || oreg.dx) {
    oreg.ax = oreg.cx;
    oreg.bx = oreg.dx;
    }
    if (oreg.ax > 15*1024) {
    return;	/* Bogus! */
    } else if (oreg.ax == 15*1024) {
    boot_params.alt_mem_k = (oreg.bx << 6) + oreg.ax;
    } else {
//
// This ignores memory above 16MB if we have a memory
// hole there.  If someone actually finds a machine
// with a memory hole at 16MB and no support for
// 0E820h they should probably generate a fake e820
// map.
//
    boot_params.alt_mem_k = oreg.ax;
    }
    }
#[no_mangle]
unsafe extern "C" fn detect_memory_88() {
    static void detect_memory_88(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ah = 0x88;
    intcall(0x15, &ireg, &oreg);
    boot_params.screen_info.ext_mem_k = oreg.ax;
    }
#[no_mangle]
pub unsafe extern "C" fn detect_memory() {
    void detect_memory(void)
    {
    detect_memory_e820();
    detect_memory_e801();
    detect_memory_88();
    }
