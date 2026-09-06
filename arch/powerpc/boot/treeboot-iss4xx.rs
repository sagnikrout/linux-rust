//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/treeboot-iss4xx.c
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
// Copyright 2010 Ben. Herrenschmidt, IBM Corporation.
//
// Based on earlier code:
// Copyright (C) Paul Mackerras 1997.
//
// Matt Porter <mporter@kernel.crashing.org>
// Copyright 2002-2005 MontaVista Software Inc.
//
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
// Copyright (c) 2003, 2004 Zultys Technologies
//
// Copyright 2007 David Gibson, IBM Corporation.
//

    BSS_STACK(4096);
    static u32 ibm4xx_memstart;
#[no_mangle]
unsafe extern "C" fn iss_4xx_fixups() {
    static void iss_4xx_fixups(void)
    {
    void *memory;
    u32 reg[3];
    memory = finddevice("/memory");
    if (!memory)
    fatal("Can't find memory node\n");
// This assumes #address-cells = 2, #size-cells =1 and that
    getprop(memory, "reg", reg, sizeof(reg));
    if (reg[2])
// If the device tree specifies the memory range, use it
    ibm4xx_memstart = reg[1];
    else
// othersize, read it from the SDRAM controller
    ibm4xx_sdram_fixup_memsize();
    }
    static void *iss_4xx_vmlinux_alloc(unsigned long size)
    {
    return (void *)ibm4xx_memstart;
    }
pub const SPRN_PIR: c_uint = 0x11E	/* Processor Identification Register */;
#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    void platform_init(void)
    {
    let mut end_of_ram: c_ulong = 0x08000000;
    let mut avail_ram: c_ulong = end_of_ram - (unsigned long)_end;
    u32 pir_reg;
    simple_alloc_init(_end, avail_ram, 128, 64);
    platform_ops.fixups = iss_4xx_fixups;
    platform_ops.vmlinux_alloc = iss_4xx_vmlinux_alloc;
    platform_ops.exit = ibm44x_dbcr_reset;
    pir_reg = mfspr(SPRN_PIR);
    fdt_set_boot_cpuid_phys(_dtb_start, pir_reg);
    fdt_init(_dtb_start);
    serial_console_init();
    }
