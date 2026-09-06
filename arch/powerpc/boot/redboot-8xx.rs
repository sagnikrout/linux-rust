//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/redboot-8xx.c
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
// RedBoot firmware support
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

    static bd_t bd;
    BSS_STACK(4096);

#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    void *node;
    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr);
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 16, bd.bi_busfreq);
    node = finddevice("/soc/cpm/brg");
    if (node) {
    printf("BRG clock-frequency <- 0x%x (%dMHz)\r\n",
    bd.bi_busfreq, MHZ(bd.bi_busfreq));
    setprop(node, "clock-frequency",  &bd.bi_busfreq, 4);
    }
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    memcpy(&bd, (char *)r3, sizeof(bd));
    if (bd.bi_tag != 0x42444944)
    return;
    simple_alloc_init(_end,
    bd.bi_memstart + bd.bi_memsize - (unsigned long)_end,
    32, 64);
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = platform_fixups;
    loader_info.cmdline = (char *)bd.bi_cmdline;
    loader_info.cmdline_len = strlen((char *)bd.bi_cmdline);
    }
