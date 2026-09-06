//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/iss4xx.c
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
// PPC476 board specific routines
//
// Copyright 2010 Torez Smith, IBM Corporation.
//
// Based on earlier code:
// Matt Porter <mporter@kernel.crashing.org>
// Copyright 2002-2005 MontaVista Software Inc.
//
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
// Copyright (c) 2003-2005 Zultys Technologies
//
// Rewritten and ported to the merged powerpc tree:
// Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
//

    static const struct of_device_id iss4xx_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,plb6", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn iss4xx_device_probe() -> int __init {
    static int __init iss4xx_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), iss4xx_of_bus, core::ptr::null_mut());
    of_instantiate_rtc();
    return 0;
    }
    machine_device_initcall(iss4xx, iss4xx_device_probe);
// We can have either UICs or MPICs
#[no_mangle]
unsafe extern "C" fn iss4xx_init_irq() -> void __init {
    static void __init iss4xx_init_irq(void)
    {
    struct device_node *np;
// Find top level interrupt controller
    for_each_node_with_property(np, "interrupt-controller") {
    if (!of_property_present(np, "interrupts"))
    break;
    }
    if (np == core::ptr::null_mut())
    panic("Can't find top level interrupt controller");
// Check type and do appropriate initialization
    if (of_device_is_compatible(np, "ibm,uic")) {
    uic_init_tree();
    ppc_md.get_irq = uic_get_irq;

    } else if (of_device_is_compatible(np, "chrp,open-pic")) {
// The MPIC driver will get everything it needs from the
// device-tree, just pass 0 to all arguments
//
    struct mpic *mpic = mpic_alloc(np, 0, MPIC_NO_RESET, 0, 0, " MPIC     ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    ppc_md.get_irq = mpic_get_irq;

    } else
    panic("Unrecognized top level interrupt controller");
    }

#[no_mangle]
unsafe extern "C" fn smp_iss4xx_setup_cpu(cpu: c_int) {
    static void smp_iss4xx_setup_cpu(int cpu)
    {
    mpic_setup_this_cpu();
    }
#[no_mangle]
unsafe extern "C" fn smp_iss4xx_kick_cpu(cpu: c_int) -> c_int {
    static int smp_iss4xx_kick_cpu(int cpu)
    {
    struct device_node *cpunode = of_get_cpu_node(cpu, core::ptr::null_mut());
    const u64 *spin_table_addr_prop;
    u32 *spin_table;
    extern void start_secondary_47x(void);
    BUG_ON(cpunode == core::ptr::null_mut());
// Assume spin table. We could test for the enable-method in
// the device-tree but currently there's little point as it's
// our only supported method
//
    spin_table_addr_prop = of_get_property(cpunode, "cpu-release-addr",
    core::ptr::null_mut());
    if (spin_table_addr_prop == core::ptr::null_mut()) {
    pr_err("CPU%d: Can't start, missing cpu-release-addr !\n", cpu);
    return -ENOENT;
    }
// Assume it's mapped as part of the linear mapping. This is a bit
// fishy but will work fine for now
//
    spin_table = (u32 *)__va(*spin_table_addr_prop);
    pr_debug("CPU%d: Spin table mapped at %p\n", cpu, spin_table);
    spin_table[3] = cpu;
    smp_wmb();
    spin_table[1] = __pa(start_secondary_47x);
    mb();
    return 0;
    }
    static struct smp_ops_t iss_smp_ops = {
    .probe		= smp_mpic_probe,
    .message_pass	= smp_mpic_message_pass,
    .setup_cpu	= smp_iss4xx_setup_cpu,
    .kick_cpu	= smp_iss4xx_kick_cpu,
    .give_timebase	= smp_generic_give_timebase,
    .take_timebase	= smp_generic_take_timebase,
    };
#[no_mangle]
unsafe extern "C" fn iss4xx_smp_init() -> void __init {
    static void __init iss4xx_smp_init(void)
    {
    if (mmu_has_feature(MMU_FTR_TYPE_47x))
    smp_ops = &iss_smp_ops;
    }

    static void __init iss4xx_smp_init(void) { }

#[no_mangle]
unsafe extern "C" fn iss4xx_setup_arch() -> void __init {
    static void __init iss4xx_setup_arch(void)
    {
    iss4xx_smp_init();
    }
    define_machine(iss4xx) {
    .name			= "ISS-4xx",
    .compatible		= "ibm,iss-4xx",
    .progress		= udbg_progress,
    .init_IRQ		= iss4xx_init_irq,
    .setup_arch		= iss4xx_setup_arch,
    .restart		= ppc4xx_reset_system,
    };
