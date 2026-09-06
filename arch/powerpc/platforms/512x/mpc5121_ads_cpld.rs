//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/512x/mpc5121_ads_cpld.c
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
// Copyright (C) 2008 Freescale Semiconductor, Inc. All rights reserved.
//
// Author: John Rigby, <jrigby@freescale.com>
//
// Description:
// MPC5121ADS CPLD irq handling
//

    static struct device_node *cpld_pic_node;
    static struct irq_domain *cpld_pic_host;
//
// Bits to ignore in the misc_status register
// 0x10 touch screen pendown is hard routed to irq1
// 0x02 pci status is read from pci status register
//
pub const MISC_IGNORE: c_uint = 0x12;
//
// Nothing to ignore in pci status register
//
pub const PCI_IGNORE: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpld_pic {
    pub pci_mask: u8,
    pub pci_status: u8,
    pub route: u8,
    pub misc_mask: u8,
    pub misc_status: u8,
    pub misc_control: u8,
}

    static struct cpld_pic __iomem *cpld_regs;
    static void __iomem *
    irq_to_pic_mask(unsigned int irq)
    {
    return irq <= 7 ? &cpld_regs.pci_mask : &cpld_regs.misc_mask;
    }
    static unsigned int
    irq_to_pic_bit(unsigned int irq)
    {
    return 1 << (irq & 0x7);
    }
    static void
    cpld_mask_irq(struct irq_data *d)
    {
    let mut cpld_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    void __iomem *pic_mask = irq_to_pic_mask(cpld_irq);
    out_8(pic_mask,
    in_8(pic_mask) | irq_to_pic_bit(cpld_irq));
    }
    static void
    cpld_unmask_irq(struct irq_data *d)
    {
    let mut cpld_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    void __iomem *pic_mask = irq_to_pic_mask(cpld_irq);
    out_8(pic_mask,
    in_8(pic_mask) & ~irq_to_pic_bit(cpld_irq));
    }
    static struct irq_chip cpld_pic = {
    .name = "CPLD PIC",
    .irq_mask = cpld_mask_irq,
    .irq_ack = cpld_mask_irq,
    .irq_unmask = cpld_unmask_irq,
    };
    static unsigned int
    cpld_pic_get_irq(int offset, u8 ignore, u8 __iomem *statusp,
    u8 __iomem *maskp)
    {
    let mut status: u8 = in_8(statusp);
    let mut mask: u8 = in_8(maskp);
// ignore don't cares and masked irqs
    status |= (ignore | mask);
    if (status == 0xff)
    return ~0;
    return ffz(status) + offset;
    }
#[no_mangle]
unsafe extern "C" fn cpld_pic_cascade(desc: *mut irq_desc) {
    static void cpld_pic_cascade(struct irq_desc *desc)
    {
    unsigned int hwirq;
    hwirq = cpld_pic_get_irq(0, PCI_IGNORE, &cpld_regs.pci_status,
    &cpld_regs.pci_mask);
    if (hwirq != ~0) {
    generic_handle_domain_irq(cpld_pic_host, hwirq);
    return;
    }
    hwirq = cpld_pic_get_irq(8, MISC_IGNORE, &cpld_regs.misc_status,
    &cpld_regs.misc_mask);
    if (hwirq != ~0) {
    generic_handle_domain_irq(cpld_pic_host, hwirq);
    return;
    }
    }
    static int
    cpld_pic_host_match(struct irq_domain *h, struct device_node *node,
    enum irq_domain_bus_token bus_token)
    {
    let mut cpld_pic_node: return = = node;
    }
    static int
    cpld_pic_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &cpld_pic, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops cpld_pic_host_ops = {
    .match = cpld_pic_host_match,
    .map = cpld_pic_host_map,
    };
    void __init
    mpc5121_ads_cpld_map(void)
    {
    struct device_node *np = core::ptr::null_mut();
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,mpc5121ads-cpld-pic");
    if (!np) {
    printk(KERN_ERR "CPLD PIC init: can not find cpld-pic node\n");
    return;
    }
    cpld_regs = of_iomap(np, 0);
    of_node_put(np);
    }
    void __init
    mpc5121_ads_cpld_pic_init(void)
    {
    unsigned int cascade_irq;
    struct device_node *np = core::ptr::null_mut();
    pr_debug("cpld_ic_init\n");
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,mpc5121ads-cpld-pic");
    if (!np) {
    printk(KERN_ERR "CPLD PIC init: can not find cpld-pic node\n");
    return;
    }
    if (!cpld_regs)
    goto end;
    cascade_irq = irq_of_parse_and_map(np, 0);
    if (!cascade_irq)
    goto end;
//
// statically route touch screen pendown through 1
// and ignore it here
// route all others through our cascade irq
//
    out_8(&cpld_regs.route, 0xfd);
    out_8(&cpld_regs.pci_mask, 0xff);
// unmask pci ints in misc mask
    out_8(&cpld_regs.misc_mask, ~(MISC_IGNORE));
    cpld_pic_node = of_node_get(np);
    cpld_pic_host = irq_domain_create_linear(of_fwnode_handle(np), 16,
    &cpld_pic_host_ops, core::ptr::null_mut());
    if (!cpld_pic_host) {
    printk(KERN_ERR "CPLD PIC: failed to allocate irq host!\n");
    goto end;
    }
    irq_set_chained_handler(cascade_irq, cpld_pic_cascade);
    end:
    of_node_put(np);
    }
