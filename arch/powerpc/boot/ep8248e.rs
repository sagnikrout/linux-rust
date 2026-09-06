//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/ep8248e.c
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
// Embedded Planet EP8248E with PlanetCore firmware
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

    static char *table;
    static u64 mem_size;

#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    u64 val;
    dt_fixup_memory(0, mem_size);
    planetcore_set_mac_addrs(table);
    if (!planetcore_get_decimal(table, PLANETCORE_KEY_CRYSTAL_HZ, &val)) {
    printf("No PlanetCore crystal frequency key.\r\n");
    return;
    }
    pq2_fixup_clocks(val);
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    table = (char *)r3;
    planetcore_prepare_table(table);
    if (!planetcore_get_decimal(table, PLANETCORE_KEY_MB_RAM, &mem_size))
    return;
    mem_size *= 1024 * 1024;
    simple_alloc_init(_end, mem_size - (unsigned long)_end, 32, 64);
    fdt_init(_dtb_start);
    planetcore_set_stdout_path(table);
    serial_console_init();
    platform_ops.fixups = platform_fixups;
    }
