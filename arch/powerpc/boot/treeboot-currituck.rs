//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/treeboot-currituck.c
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
// Copyright © 2011 Tony Breeds IBM Corporation
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
// Copyright 2010 Ben. Herrenschmidt, IBM Corporation.
// Copyright © 2011 David Kleikamp IBM Corporation
//

    BSS_STACK(4096);
pub const MAX_RANKS: c_uint = 0x4;
pub const DDR3_MR0CF: c_uint = 0x80010011U;
    static unsigned long long ibm_currituck_memsize;
#[no_mangle]
unsafe extern "C" fn ibm_currituck_detect_memsize() -> c_ulonglong {
    static unsigned long long ibm_currituck_detect_memsize(void)
    {
    u32 reg;
    unsigned i;
    let mut memsize: c_ulonglong = 0;
    for(i = 0; i < MAX_RANKS; i++){
    reg = mfdcrx(DDR3_MR0CF + i);
    if (!(reg & 1))
    continue;
    reg &= 0x0000f000;
    reg >>= 12;
    memsize += (0x800000ULL << reg);
    }
    return memsize;
    }
#[no_mangle]
unsafe extern "C" fn ibm_currituck_fixups() {
    static void ibm_currituck_fixups(void)
    {
    void *devp = finddevice("/");
    u32 dma_ranges[7];
    dt_fixup_memory(0x0ULL,  ibm_currituck_memsize);
    while ((devp = find_node_by_devtype(devp, "pci"))) {
    if (getprop(devp, "dma-ranges", dma_ranges, sizeof(dma_ranges)) < 0) {
    printf("%s: Failed to get dma-ranges\r\n", __func__);
    continue;
    }
    dma_ranges[5] = ibm_currituck_memsize >> 32;
    dma_ranges[6] = ibm_currituck_memsize & 0xffffffffUL;
    setprop(devp, "dma-ranges", dma_ranges, sizeof(dma_ranges));
    }
    }
pub const SPRN_PIR: c_uint = 0x11E	/* Processor Identification Register */;
#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    void platform_init(void)
    {
    unsigned long end_of_ram, avail_ram;
    u32 pir_reg;
    int node, size;
    const u32 *timebase;
    ibm_currituck_memsize = ibm_currituck_detect_memsize();
    if (ibm_currituck_memsize >> 32)
    end_of_ram = ~0UL;
    else
    end_of_ram = ibm_currituck_memsize;
    avail_ram = end_of_ram - (unsigned long)_end;
    simple_alloc_init(_end, avail_ram, 128, 64);
    platform_ops.fixups = ibm_currituck_fixups;
    platform_ops.exit = ibm44x_dbcr_reset;
    pir_reg = mfspr(SPRN_PIR);
// Make sure FDT blob is sane
    if (fdt_check_header(_dtb_start) != 0)
    fatal("Invalid device tree blob\n");
    node = fdt_node_offset_by_prop_value(_dtb_start, -1, "device_type",
    "cpu", sizeof("cpu"));
    if (node < 0)
    fatal("Cannot find cpu node\n");
    timebase = fdt_getprop(_dtb_start, node, "timebase-frequency", &size);
    if (timebase && (size == 4))
    timebase_period_ns = 1000000000 / *timebase;
    fdt_set_boot_cpuid_phys(_dtb_start, pir_reg);
    fdt_init(_dtb_start);
    serial_console_init();
    }
