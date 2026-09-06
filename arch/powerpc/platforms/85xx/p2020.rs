//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/p2020.c
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
// Freescale P2020 board Setup
//
// Copyright 2007,2009,2012-2013 Freescale Semiconductor Inc.
// Copyright 2022-2023 Pali Rohár <pali@kernel.org>
//

#[no_mangle]
unsafe extern "C" fn p2020_pic_init() -> void __init {
    static void __init p2020_pic_init(void)
    {
    struct mpic *mpic;
    let mut flags: c_int = MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU;
    mpic = mpic_alloc(core::ptr::null_mut(), 0, flags, 0, 256, " OpenPIC  ");
    if (WARN_ON(!mpic))
    return;
    mpic_init(mpic);
    mpc85xx_8259_init();
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn p2020_setup_arch() -> void __init {
    static void __init p2020_setup_arch(void)
    {
    swiotlb_detect_4g();
    fsl_pci_assign_primary();
    uli_init();
    mpc85xx_smp_init();
    mpc85xx_qe_par_io_init();
    }
//
// Called very early, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn p2020_probe() -> int __init {
    static int __init p2020_probe(void)
    {
    struct device_node *p2020_cpu;
//
// There is no common compatible string for all P2020 boards.
// The only common thing is "PowerPC,P2020@0" cpu node.
// So check for P2020 board via this cpu node.
//
    p2020_cpu = of_find_node_by_path("/cpus/PowerPC,P2020@0");
    of_node_put(p2020_cpu);
    return !!p2020_cpu;
    }
    machine_arch_initcall(p2020, mpc85xx_common_publish_devices);
    define_machine(p2020) {
    .name			= "Freescale P2020",
    .probe			= p2020_probe,
    .setup_arch		= p2020_setup_arch,
    .init_IRQ		= p2020_pic_init,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb	= fsl_pcibios_fixup_phb,

    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
