//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/cuboot-52xx.c
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
// Old U-boot compatibility for MPC5200
//
// Author: Grant Likely <grant.likely@secretlab.ca>
//
// Copyright (c) 2007 Secret Lab Technologies Ltd.
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

// Macro flag: #define TARGET_PPC_MPC52xx

    static bd_t bd;
#[no_mangle]
unsafe extern "C" fn platform_fixups() {
    static void platform_fixups(void)
    {
    void *soc, *reg;
    int div;
    u32 sysfreq;
    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_addresses(bd.bi_enetaddr);
    dt_fixup_cpu_clocks(bd.bi_intfreq, bd.bi_busfreq / 4, bd.bi_busfreq);
// Unfortunately, the specific model number is encoded in the
// soc node name in existing dts files -- once that is fixed,
// this can do a simple path lookup.
//
    soc = find_node_by_devtype(core::ptr::null_mut(), "soc");
    if (!soc)
    soc = find_node_by_compatible(core::ptr::null_mut(), "fsl,mpc5200-immr");
    if (!soc)
    soc = find_node_by_compatible(core::ptr::null_mut(), "fsl,mpc5200b-immr");
    if (soc) {
    setprop(soc, "bus-frequency", &bd.bi_ipbfreq,
    sizeof(bd.bi_ipbfreq));
    if (!dt_xlate_reg(soc, 0, (void*)&reg, core::ptr::null_mut()))
    return;
    div = in_8(reg + 0x204) & 0x0020 ? 8 : 4;
    sysfreq = bd.bi_busfreq * div;
    setprop(soc, "system-frequency", &sysfreq, sizeof(sysfreq));
    }
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = platform_fixups;
    }
