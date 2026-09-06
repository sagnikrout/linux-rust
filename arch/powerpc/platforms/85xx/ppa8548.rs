//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/ppa8548.c
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
// ppa8548 setup and early boot code.
//
// Copyright 2009 Prodrive B.V..
//
// By Stef van Os (see MAINTAINERS for contact information)
//
// Based on the SBC8548 support - Copyright 2007 Wind River Systems Inc.
// Based on the MPC8548CDS support - Copyright 2005 Freescale Inc.
//

#[no_mangle]
unsafe extern "C" fn ppa8548_pic_init() -> void __init {
    static void __init ppa8548_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn ppa8548_setup_arch() -> void __init {
    static void __init ppa8548_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("ppa8548_setup_arch()", 0);
    }
#[no_mangle]
unsafe extern "C" fn ppa8548_show_cpuinfo(m: *mut seq_file) {
    static void ppa8548_show_cpuinfo(struct seq_file *m)
    {
    uint32_t svid, phid1;
    svid = mfspr(SPRN_SVR);
    seq_printf(m, "Vendor\t\t: Prodrive B.V.\n");
    seq_printf(m, "SVR\t\t: 0x%x\n", svid);
// Display cpu Pll setting
    phid1 = mfspr(SPRN_HID1);
    seq_printf(m, "PLL setting\t: 0x%x\n", ((phid1 >> 24) & 0x3f));
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .name = "soc", },
    { .type = "soc", },
    { .compatible = "simple-bus", },
    { .compatible = "gianfar", },
    { .compatible = "fsl,srio", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(ppa8548, declare_of_platform_devices);
    define_machine(ppa8548) {
    .name		= "ppa8548",
    .compatible	= "ppa8548",
    .setup_arch	= ppa8548_setup_arch,
    .init_IRQ	= ppa8548_pic_init,
    .show_cpuinfo	= ppa8548_show_cpuinfo,
    .get_irq	= mpic_get_irq,
    .progress	= udbg_progress,
    };
