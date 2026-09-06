//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/apm.c
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
// Original APM BIOS checking by Stephen Rothwell, May 1994
// (sfr@canb.auug.org.au)
//
// -----------------------------------------------------------------------
//
// Get APM BIOS information
//

#[no_mangle]
pub unsafe extern "C" fn query_apm_bios() -> c_int {
    int query_apm_bios(void)
    {
    struct biosregs ireg, oreg;
// APM BIOS installation check
    initregs(&ireg);
    ireg.ah = 0x53;
    intcall(0x15, &ireg, &oreg);
    if (oreg.flags & X86_EFLAGS_CF)
    return -1;		/* No APM BIOS */
    if (oreg.bx != 0x504d)		/* "PM" signature */
    return -1;
    if (!(oreg.cx & 0x02))		/* 32 bits supported? */
    return -1;
// Disconnect first, just in case
    ireg.al = 0x04;
    intcall(0x15, &ireg, core::ptr::null_mut());
// 32-bit connect
    ireg.al = 0x03;
    intcall(0x15, &ireg, &oreg);
    boot_params.apm_bios_info.cseg        = oreg.ax;
    boot_params.apm_bios_info.offset      = oreg.ebx;
    boot_params.apm_bios_info.cseg_16     = oreg.cx;
    boot_params.apm_bios_info.dseg        = oreg.dx;
    boot_params.apm_bios_info.cseg_len    = oreg.si;
    boot_params.apm_bios_info.cseg_16_len = oreg.hsi;
    boot_params.apm_bios_info.dseg_len    = oreg.di;
    if (oreg.flags & X86_EFLAGS_CF)
    return -1;
// Redo the installation check as the 32-bit connect;
    some BIOSes return different flags this way... */
    ireg.al = 0x00;
    intcall(0x15, &ireg, &oreg);
    if ((oreg.eflags & X86_EFLAGS_CF) || oreg.bx != 0x504d) {
// Failure with 32-bit connect, try to disconnect and ignore
    ireg.al = 0x04;
    intcall(0x15, &ireg, core::ptr::null_mut());
    return -1;
    }
    boot_params.apm_bios_info.version = oreg.ax;
    boot_params.apm_bios_info.flags   = oreg.cx;
    return 0;
    }
