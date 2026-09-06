//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/ppc476.c
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
// PowerPC 476FPE board specific routines
//
// Copyright © 2013 Tony Breeds IBM Corporation
// Copyright © 2013 Alistair Popple IBM Corporation
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
// Copyright © 2011 David Kliekamp IBM Corporation
//

    static const struct of_device_id ppc47x_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,plb6", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    {},
    };
// The EEPROM is missing and the default values are bogus.  This forces USB in
// to EHCI mode
#[no_mangle]
unsafe extern "C" fn quirk_ppc_currituck_usb_fixup(dev: *mut pci_dev) {
    static void quirk_ppc_currituck_usb_fixup(struct pci_dev *dev)
    {
    if (of_machine_is_compatible("ibm,currituck")) {
    pci_write_config_dword(dev, 0xe0, 0x0114231f);
    pci_write_config_dword(dev, 0xe4, 0x00006c40);
    }
    }
    DECLARE_PCI_FIXUP_HEADER(0x1033, 0x0035, quirk_ppc_currituck_usb_fixup);
// Akebono has an AVR microcontroller attached to the I2C bus
// which is used to power off/reset the system.
// AVR I2C Commands

// Flags for the power control I2C commands

    static struct i2c_client *avr_i2c_client;
#[no_mangle]
unsafe extern "C" fn avr_halt_system(pwrctl_flags: c_int) -> void __noreturn {
    static void __noreturn avr_halt_system(int pwrctl_flags)
    {
// Request the AVR to reset the system
    i2c_smbus_write_byte_data(avr_i2c_client,
    AVR_PWRCTL_CMD, pwrctl_flags);
// Wait for system to be reset
    while (1)
    ;
    }
#[no_mangle]
unsafe extern "C" fn avr_power_off_system() {
    static void avr_power_off_system(void)
    {
    avr_halt_system(AVR_PWRCTL_PWROFF);
    }
#[no_mangle]
unsafe extern "C" fn avr_reset_system(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn avr_reset_system(char *cmd)
    {
    avr_halt_system(AVR_PWRCTL_RESET);
    }
#[no_mangle]
unsafe extern "C" fn avr_probe(client: *mut i2c_client) -> c_int {
    static int avr_probe(struct i2c_client *client)
    {
    avr_i2c_client = client;
    ppc_md.restart = avr_reset_system;
    pm_power_off = avr_power_off_system;
    return 0;
    }
    static const struct i2c_device_id avr_id[] = {
    { "akebono-avr" },
    { }
    };
    static struct i2c_driver avr_driver = {
    .driver = {
    .name = "akebono-avr",
    },
    .probe = avr_probe,
    .id_table = avr_id,
    };
#[no_mangle]
unsafe extern "C" fn ppc47x_device_probe() -> int __init {
    static int __init ppc47x_device_probe(void)
    {
    i2c_add_driver(&avr_driver);
    of_platform_bus_probe(core::ptr::null_mut(), ppc47x_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(ppc47x_akebono, ppc47x_device_probe);
    machine_device_initcall(ppc47x_currituck, ppc47x_device_probe);
#[no_mangle]
unsafe extern "C" fn ppc47x_init_irq() -> void __init {
    static void __init ppc47x_init_irq(void)
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
    if (of_device_is_compatible(np, "chrp,open-pic")) {
// The MPIC driver will get everything it needs from the
// device-tree, just pass 0 to all arguments
//
    struct mpic *mpic =
    mpic_alloc(np, 0, MPIC_NO_RESET, 0, 0, " MPIC     ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    ppc_md.get_irq = mpic_get_irq;
    } else
    panic("Unrecognized top level interrupt controller");
    of_node_put(np);
    }

#[no_mangle]
unsafe extern "C" fn smp_ppc47x_setup_cpu(cpu: c_int) {
    static void smp_ppc47x_setup_cpu(int cpu)
    {
    mpic_setup_this_cpu();
    }
#[no_mangle]
unsafe extern "C" fn smp_ppc47x_kick_cpu(cpu: c_int) -> c_int {
    static int smp_ppc47x_kick_cpu(int cpu)
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
    spin_table_addr_prop =
    of_get_property(cpunode, "cpu-release-addr", core::ptr::null_mut());
    if (spin_table_addr_prop == core::ptr::null_mut()) {
    pr_err("CPU%d: Can't start, missing cpu-release-addr !\n",
    cpu);
    return 1;
    }
// Assume it's mapped as part of the linear mapping. This is a bit
// fishy but will work fine for now
//
// XXX: Is there any reason to assume differently?
//
    spin_table = (u32 *)__va(*spin_table_addr_prop);
    pr_debug("CPU%d: Spin table mapped at %p\n", cpu, spin_table);
    spin_table[3] = cpu;
    smp_wmb();
    spin_table[1] = __pa(start_secondary_47x);
    mb();
    return 0;
    }
    static struct smp_ops_t ppc47x_smp_ops = {
    .probe		= smp_mpic_probe,
    .message_pass	= smp_mpic_message_pass,
    .setup_cpu	= smp_ppc47x_setup_cpu,
    .kick_cpu	= smp_ppc47x_kick_cpu,
    .give_timebase	= smp_generic_give_timebase,
    .take_timebase	= smp_generic_take_timebase,
    };
#[no_mangle]
unsafe extern "C" fn ppc47x_smp_init() -> void __init {
    static void __init ppc47x_smp_init(void)
    {
    if (mmu_has_feature(MMU_FTR_TYPE_47x))
    smp_ops = &ppc47x_smp_ops;
    }

    static void __init ppc47x_smp_init(void) { }

#[no_mangle]
unsafe extern "C" fn ppc47x_setup_arch() -> void __init {
    static void __init ppc47x_setup_arch(void)
    {
// No need to check the DMA config as we /know/ our windows are all of
// RAM.  Lets hope that doesn't change
    swiotlb_detect_4g();
    ppc47x_smp_init();
    }
    let mut board_rev: static int = -1;
#[no_mangle]
unsafe extern "C" fn ppc47x_get_board_rev() -> int __init {
    static int __init ppc47x_get_board_rev(void)
    {
    int reg;
    u8 __iomem *fpga;
    struct device_node *np = core::ptr::null_mut();
    if (of_machine_is_compatible("ibm,currituck")) {
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,currituck-fpga");
    reg = 0;
    } else if (of_machine_is_compatible("ibm,akebono")) {
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,akebono-fpga");
    reg = 2;
    }
    if (!np)
    goto fail;
    fpga = of_iomap(np, 0);
    of_node_put(np);
    if (!fpga)
    goto fail;
    board_rev = ioread8(fpga + reg) & 0x03;
    pr_info("%s: Found board revision %d\n", __func__, board_rev);
    iounmap(fpga);
    return 0;
    fail:
    pr_info("%s: Unable to find board revision\n", __func__);
    return 0;
    }
    machine_arch_initcall(ppc47x_akebono, ppc47x_get_board_rev);
    machine_arch_initcall(ppc47x_currituck, ppc47x_get_board_rev);
// Use USB controller should have been hardware swizzled but it wasn't :(
#[no_mangle]
unsafe extern "C" fn ppc47x_pci_irq_fixup(dev: *mut pci_dev) {
    static void ppc47x_pci_irq_fixup(struct pci_dev *dev)
    {
    if (dev.vendor == 0x1033 && (dev.device == 0x0035 ||
    dev.device == 0x00e0)) {
    if (board_rev == 0) {
    dev.irq = irq_create_mapping(core::ptr::null_mut(), 47);
    pr_info("%s: Mapping irq %d\n", __func__, dev.irq);
    } else if (board_rev == 2) {
    dev.irq = irq_create_mapping(core::ptr::null_mut(), 49);
    pr_info("%s: Mapping irq %d\n", __func__, dev.irq);
    } else {
    pr_alert("%s: Unknown board revision\n", __func__);
    }
    }
    }
    define_machine(ppc47x_akebono) {
    .name			= "PowerPC 47x (akebono)",
    .compatible		= "ibm,akebono",
    .progress		= udbg_progress,
    .init_IRQ		= ppc47x_init_irq,
    .setup_arch		= ppc47x_setup_arch,
    .restart		= ppc4xx_reset_system,
    };
    define_machine(ppc47x_currituck) {
    .name			= "PowerPC 47x (currituck)",
    .compatible		= "ibm,currituck",
    .progress		= udbg_progress,
    .init_IRQ		= ppc47x_init_irq,
    .pci_irq_fixup		= ppc47x_pci_irq_fixup,
    .setup_arch		= ppc47x_setup_arch,
    .restart		= ppc4xx_reset_system,
    };
