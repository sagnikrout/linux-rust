//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/stx_gp3.c
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
// Based on MPC8560 ADS and arch/ppc stx_gp3 ports
//
// Maintained by Kumar Gala (see MAINTAINERS for contact information)
//
// Copyright 2008 Freescale Semiconductor Inc.
//
// Dan Malek <dan@embeddededge.com>
// Copyright 2004 Embedded Edge, LLC
//
// Copied from mpc8560_ads.c
// Copyright 2002, 2003 Motorola Inc.
//
// Ported to 2.6, Matt Porter <mporter@kernel.crashing.org>
// Copyright 2004-2005 MontaVista Software, Inc.
//

#[no_mangle]
unsafe extern "C" fn stx_gp3_pic_init() -> void __init {
    static void __init stx_gp3_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    mpc85xx_cpm2_pic_init();
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn stx_gp3_setup_arch() -> void __init {
    static void __init stx_gp3_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("stx_gp3_setup_arch()", 0);
    fsl_pci_assign_primary();

    cpm2_reset();

    }
#[no_mangle]
unsafe extern "C" fn stx_gp3_show_cpuinfo(m: *mut seq_file) {
    static void stx_gp3_show_cpuinfo(struct seq_file *m)
    {
    uint pvid, svid, phid1;
    pvid = mfspr(SPRN_PVR);
    svid = mfspr(SPRN_SVR);
    seq_printf(m, "Vendor\t\t: RPC Electronics STx\n");
    seq_printf(m, "PVR\t\t: 0x%x\n", pvid);
    seq_printf(m, "SVR\t\t: 0x%x\n", svid);
// Display cpu Pll setting
    phid1 = mfspr(SPRN_HID1);
    seq_printf(m, "PLL setting\t: 0x%x\n", ((phid1 >> 24) & 0x3f));
    }
    machine_arch_initcall(stx_gp3, mpc85xx_common_publish_devices);
    define_machine(stx_gp3) {
    .name			= "STX GP3",
    .compatible		= "stx,gp3-8560",
    .setup_arch		= stx_gp3_setup_arch,
    .init_IRQ		= stx_gp3_pic_init,
    .show_cpuinfo		= stx_gp3_show_cpuinfo,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
