//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/epapr.c
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
//
// Bootwrapper for ePAPR compliant firmwares
//
// Copyright 2010 David Gibson <david@gibson.dropbear.id.au>, IBM Corporation.
//
// Based on earlier bootwrappers by:
// (c) Benjamin Herrenschmidt <benh@kernel.crashing.org>, IBM Corp,\
// and
// Scott Wood <scottwood@freescale.com>
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

    BSS_STACK(4096);
pub const EPAPR_SMAGIC: c_uint = 0x65504150;
pub const EPAPR_EMAGIC: c_uint = 0x45504150;
    static unsigned epapr_magic;
    static unsigned long ima_size;
    static unsigned long fdt_addr;
#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    if ((epapr_magic != EPAPR_EMAGIC)
    && (epapr_magic != EPAPR_SMAGIC))
    fatal("r6 contained 0x%08x instead of ePAPR magic number\n",
    epapr_magic);
    if (ima_size < (unsigned long)_end)
    printf("WARNING: Image loaded outside IMA!"
    " (_end=%p, ima_size=0x%lx)\n", _end, ima_size);
    if (ima_size < fdt_addr)
    printf("WARNING: Device tree address is outside IMA!"
    "(fdt_addr=0x%lx, ima_size=0x%lx)\n", fdt_addr,
    ima_size);
    if (ima_size < fdt_addr + fdt_totalsize((void *)fdt_addr))
    printf("WARNING: Device tree extends outside IMA!"
    " (fdt_addr=0x%lx, size=0x%x, ima_size=0x%lx\n",
    fdt_addr, fdt_totalsize((void *)fdt_addr), ima_size);
    }
    void epapr_platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    epapr_magic = r6;
    ima_size = r7;
    fdt_addr = r3;
// FIXME: we should process reserve entries
    simple_alloc_init(_end, ima_size - (unsigned long)_end, 32, 64);
    fdt_init((void *)fdt_addr);
    serial_console_init();
    platform_ops.fixups = platform_fixups;
    }
