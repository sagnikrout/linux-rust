//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/treeboot-akebono.c
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
// Copyright © 2013 Tony Breeds IBM Corporation
// Copyright © 2013 Alistair Popple IBM Corporation
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
pub const SPRN_PIR: c_uint = 0x11E	/* Processor Identification Register */;

pub const MAX_RANKS: c_uint = 0x4;
pub const DDR3_MR0CF: c_uint = 0x80010011U;
pub const CCTL0_MCO2: c_uint = 0x8000080FU;
pub const CCTL0_MCO3: c_uint = 0x80000810U;
pub const CCTL0_MCO4: c_uint = 0x80000811U;
pub const CCTL0_MCO5: c_uint = 0x80000812U;
pub const CCTL0_MCO6: c_uint = 0x80000813U;
    static unsigned long long ibm_akebono_memsize;
    static long long unsigned mac_addr;
#[no_mangle]
unsafe extern "C" fn ibm_akebono_detect_memsize() -> c_ulonglong {
    static unsigned long long ibm_akebono_detect_memsize(void)
    {
    u32 reg;
    unsigned i;
    let mut memsize: c_ulonglong = 0;
    for (i = 0; i < MAX_RANKS; i++) {
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
unsafe extern "C" fn ibm_akebono_fixups() {
    static void ibm_akebono_fixups(void)
    {
    void *emac;
    u32 reg;
    dt_fixup_memory(0x0ULL,  ibm_akebono_memsize);
// Fixup the SD timeout frequency
    mtdcrx(CCTL0_MCO4, 0x1);
// Disable SD high-speed mode (which seems to be broken)
    reg = mfdcrx(CCTL0_MCO2) & ~0x2;
    mtdcrx(CCTL0_MCO2, reg);
// Set the MAC address
    emac = finddevice("/plb/opb/ethernet");
    if (emac > 0) {
    if (mac_addr)
    setprop(emac, "local-mac-address",
    ((u8 *) &mac_addr) + 2 , 6);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn platform_init(userdata: *mut c_char) {
    void platform_init(char *userdata)
    {
    unsigned long end_of_ram, avail_ram;
    u32 pir_reg;
    int node, size;
    const u32 *timebase;
    int len, i, userdata_len;
    char *end;
    userdata[USERDATA_LEN - 1] = '\0';
    userdata_len = strlen(userdata);
    for (i = 0; i < userdata_len - 15; i++) {
    if (strncmp(&userdata[i], "local-mac-addr=", 15) == 0) {
    if (i > 0 && userdata[i - 1] != ' ') {
// We've only found a substring ending
// with local-mac-addr so this isn't
// our mac address.
    continue;
    }
    mac_addr = strtoull(&userdata[i + 15], &end, 16);
// Remove the "local-mac-addr=<...>" from the kernel
// command line, including the tailing space if
// present.
    if (*end == ' ')
    end++;
    len = ((int) end) - ((int) &userdata[i]);
    memmove(&userdata[i], end,
    userdata_len - (len + i) + 1);
    break;
    }
    }
    loader_info.cmdline = userdata;
    loader_info.cmdline_len = 256;
    ibm_akebono_memsize = ibm_akebono_detect_memsize();
    if (ibm_akebono_memsize >> 32)
    end_of_ram = ~0UL;
    else
    end_of_ram = ibm_akebono_memsize;
    avail_ram = end_of_ram - (unsigned long)_end;
    simple_alloc_init(_end, avail_ram, 128, 64);
    platform_ops.fixups = ibm_akebono_fixups;
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
