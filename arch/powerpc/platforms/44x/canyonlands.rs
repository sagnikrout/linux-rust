//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/canyonlands.c
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
// This contain platform specific code for APM PPC460EX based Canyonlands
// board.
//
// Copyright (c) 2010, Applied Micro Circuits Corporation
// Author: Rupjyoti Sarmah <rsarmah@apm.com>
//

pub const BCSR_USB_EN: c_uint = 0x11;
    static const struct of_device_id ppc460ex_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    { .compatible = "simple-bus", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn ppc460ex_device_probe() -> int __init {
    static int __init ppc460ex_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), ppc460ex_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(canyonlands, ppc460ex_device_probe);
// Using this code only for the Canyonlands board.
#[no_mangle]
unsafe extern "C" fn ppc460ex_probe() -> int __init {
    static int __init ppc460ex_probe(void)
    {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);
    return 1;
    }
// USB PHY fixup code on Canyonlands kit.
#[no_mangle]
unsafe extern "C" fn ppc460ex_canyonlands_fixup() -> int __init {
    static int __init ppc460ex_canyonlands_fixup(void)
    {
    u8 __iomem *bcsr ;
    void __iomem *vaddr;
    struct device_node *np;
    let mut ret: c_int = 0;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "amcc,ppc460ex-bcsr");
    if (!np) {
    printk(KERN_ERR "failed did not find amcc, ppc460ex bcsr node\n");
    return -ENODEV;
    }
    bcsr = of_iomap(np, 0);
    of_node_put(np);
    if (!bcsr) {
    printk(KERN_CRIT "Could not remap bcsr\n");
    ret = -ENODEV;
    goto err_bcsr;
    }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,ppc4xx-gpio");
    if (!np) {
    printk(KERN_ERR "failed did not find ibm,ppc4xx-gpio node\n");
    return -ENODEV;
    }
    vaddr = of_iomap(np, 0);
    of_node_put(np);
    if (!vaddr) {
    printk(KERN_CRIT "Could not get gpio node address\n");
    ret = -ENODEV;
    goto err_gpio;
    }
// Disable USB, through the BCSR7 bits
    setbits8(&bcsr[7], BCSR_USB_EN);
// Wait for a while after reset
    msleep(100);
// Enable USB here
    clrbits8(&bcsr[7], BCSR_USB_EN);
//
// Configure multiplexed gpio16 and gpio19 as alternate1 output
// source after USB reset. In this configuration gpio16 will be
// USB2HStop and gpio19 will be USB2DStop. For more details refer to
// table 34-7 of PPC460EX user manual.
//
    setbits32((vaddr + GPIO0_OSRH), 0x42000000);
    setbits32((vaddr + GPIO0_TSRH), 0x42000000);
    err_gpio:
    iounmap(vaddr);
    err_bcsr:
    iounmap(bcsr);
    return ret;
    }
    machine_device_initcall(canyonlands, ppc460ex_canyonlands_fixup);
    define_machine(canyonlands) {
    .name = "Canyonlands",
    .compatible = "amcc,canyonlands",
    .probe = ppc460ex_probe,
    .progress = udbg_progress,
    .init_IRQ = uic_init_tree,
    .get_irq = uic_get_irq,
    .restart = ppc4xx_reset_system,
    };
