//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/mvme7100.c
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
// Motload compatibility for the Emerson/Artesyn MVME7100
//
// Copyright 2016 Elettra-Sincrotrone Trieste S.C.p.A.
//
// Author: Alessio Igor Bogani <alessio.bogani@elettra.eu>
//

// Macro flag: #define TARGET_86xx
// Macro flag: #define TARGET_HAS_ETH1
// Macro flag: #define TARGET_HAS_ETH2
// Macro flag: #define TARGET_HAS_ETH3

    static bd_t bd;
    BSS_STACK(16384);
#[no_mangle]
unsafe extern "C" fn mvme7100_fixups() {
    static void mvme7100_fixups(void)
    {
    void *devp;
    let mut busfreq: c_ulong = bd.bi_busfreq * 1000000;
    dt_fixup_cpu_clocks(bd.bi_intfreq * 1000000, busfreq / 4, busfreq);
    devp = finddevice("/soc@f1000000");
    if (devp)
    setprop(devp, "bus-frequency", &busfreq, sizeof(busfreq));
    devp = finddevice("/soc/serial@4500");
    if (devp)
    setprop(devp, "clock-frequency", &busfreq, sizeof(busfreq));
    dt_fixup_memory(bd.bi_memstart, bd.bi_memsize);
    dt_fixup_mac_address_by_alias("ethernet0", bd.bi_enetaddr);
    dt_fixup_mac_address_by_alias("ethernet1", bd.bi_enet1addr);
    dt_fixup_mac_address_by_alias("ethernet2", bd.bi_enet2addr);
    dt_fixup_mac_address_by_alias("ethernet3", bd.bi_enet3addr);
    }
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5,
    unsigned long r6, unsigned long r7)
    {
    CUBOOT_INIT();
    fdt_init(_dtb_start);
    serial_console_init();
    platform_ops.fixups = mvme7100_fixups;
    }
