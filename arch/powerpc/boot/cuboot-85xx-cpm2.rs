//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cuboot-85xx-cpm2.c
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
// Old U-boot compatibility for 85xx
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

// Macro flag: #define TARGET_85xx
// Macro flag: #define TARGET_CPM2

    static bd_t bd;
#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    void *devp;
    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_address_by_alias("ethernet0", bd.bi_enetaddr);
    dt_fixup_mac_address_by_alias("ethernet1", bd.bi_enet1addr);
    dt_fixup_mac_address_by_alias("ethernet2", bd.bi_enet2addr);
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 8, bd.bi_busfreq);
// Unfortunately, the specific model number is encoded in the
// soc node name in existing dts files -- once that is fixed,
// this can do a simple path lookup.
//
    devp = find_node_by_devtype(core::ptr::null_mut(), "soc");
    if (devp) {
    void *serial = core::ptr::null_mut();
    setprop(devp, "bus-frequency", &bd.bi_busfreq,
    sizeof(bd.bi_busfreq));
    while ((serial = find_node_by_devtype(serial, "serial"))) {
    if (get_parent(serial) != devp)
    continue;
    setprop(serial, "clock-frequency", &bd.bi_busfreq,
    sizeof(bd.bi_busfreq));
    }
    }
    devp = find_node_by_compatible(core::ptr::null_mut(), "fsl,cpm2-brg");
    if (devp)
    setprop(devp, "clock-frequency", &bd.bi_brgfreq,
    sizeof(bd.bi_brgfreq));
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = platform_fixups;
    }
