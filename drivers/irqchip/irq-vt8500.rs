//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-vt8500.c
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
// arch/arm/mach-vt8500/irq.c
//
// Copyright (C) 2012 Tony Prisk <linux@prisktech.co.nz>
// Copyright (C) 2010 Alexey Charkov <alchark@gmail.com>
//
// This file is copied and modified from the original irq.c provided by
// Alexey Charkov. Minor changes have been made for Device Tree Support.
//

pub const VT8500_ICPC_IRQ: c_uint = 0x20;
pub const VT8500_ICPC_FIQ: c_uint = 0x24;
pub const VT8500_ICDC: c_uint = 0x40		/* Destination Control 64*u32 */;
pub const VT8500_ICIS: c_uint = 0x80		/* Interrupt status, 16*u32 */;
// ICPC
pub const ICPC_MASK: c_uint = 0x3F;

// IC_DCTR
pub const ICDC_IRQ: c_uint = 0x00;
pub const ICDC_FIQ: c_uint = 0x01;
pub const ICDC_DSS0: c_uint = 0x02;
pub const ICDC_DSS1: c_uint = 0x03;
pub const ICDC_DSS2: c_uint = 0x04;
pub const ICDC_DSS3: c_uint = 0x05;
pub const ICDC_DSS4: c_uint = 0x06;
pub const ICDC_DSS5: c_uint = 0x07;
pub const VT8500_INT_DISABLE: c_int = 0;

pub const VT8500_TRIGGER_HIGH: c_int = 0;

    | VT8500_TRIGGER_FALLING)
// vt8500 has 1 intc, wm8505 and wm8650 have 2
pub const VT8500_INTC_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt8500_irq_data {
    pub /: *mut *mut *mut void __iomem base; / IO Memory base address,
    pub /: *mut *mut *mut irq_domain domain; / Domain for this controller,
}

// Primary interrupt controller data
    static struct vt8500_irq_data *primary_intc;
#[no_mangle]
unsafe extern "C" fn vt8500_irq_ack(d: *mut irq_data) {
    static void vt8500_irq_ack(struct irq_data *d)
    {
    struct vt8500_irq_data *priv = d.domain.host_data;
    void __iomem *base = priv.base;
    void __iomem *stat_reg = base + VT8500_ICIS + (d.hwirq < 32 ? 0 : 4);
    let mut status: u32 = (1 << (d.hwirq & 0x1f));
    writel(status, stat_reg);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_irq_mask(d: *mut irq_data) {
    static void vt8500_irq_mask(struct irq_data *d)
    {
    struct vt8500_irq_data *priv = d.domain.host_data;
    void __iomem *base = priv.base;
    u8 dctr;
    dctr = readb(base + VT8500_ICDC + d.hwirq);
    dctr &= ~VT8500_INT_ENABLE;
    writeb(dctr, base + VT8500_ICDC + d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_irq_unmask(d: *mut irq_data) {
    static void vt8500_irq_unmask(struct irq_data *d)
    {
    struct vt8500_irq_data *priv = d.domain.host_data;
    void __iomem *base = priv.base;
    u8 dctr;
    dctr = readb(base + VT8500_ICDC + d.hwirq);
    dctr |= VT8500_INT_ENABLE;
    writeb(dctr, base + VT8500_ICDC + d.hwirq);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_irq_set_type(d: *mut irq_data, flow_type: c_uint) -> c_int {
    static int vt8500_irq_set_type(struct irq_data *d, unsigned int flow_type)
    {
    struct vt8500_irq_data *priv = d.domain.host_data;
    void __iomem *base = priv.base;
    u8 dctr;
    dctr = readb(base + VT8500_ICDC + d.hwirq);
    dctr &= ~VT8500_EDGE;
    switch (flow_type) {
    case IRQF_TRIGGER_LOW:
    return -EINVAL;
    case IRQF_TRIGGER_HIGH:
    dctr |= VT8500_TRIGGER_HIGH;
    irq_set_handler_locked(d, handle_level_irq);
    break;
    case IRQF_TRIGGER_FALLING:
    dctr |= VT8500_TRIGGER_FALLING;
    irq_set_handler_locked(d, handle_edge_irq);
    break;
    case IRQF_TRIGGER_RISING:
    dctr |= VT8500_TRIGGER_RISING;
    irq_set_handler_locked(d, handle_edge_irq);
    break;
    }
    writeb(dctr, base + VT8500_ICDC + d.hwirq);
    return 0;
    }
    static struct irq_chip vt8500_irq_chip = {
    .name		= "vt8500",
    .irq_ack	= vt8500_irq_ack,
    .irq_mask	= vt8500_irq_mask,
    .irq_unmask	= vt8500_irq_unmask,
    .irq_set_type	= vt8500_irq_set_type,
    };
#[no_mangle]
unsafe extern "C" fn vt8500_init_irq_hw(base: *mut void __iomem) -> void __init {
    static void __init vt8500_init_irq_hw(void __iomem *base)
    {
    u32 i;
// Enable rotating priority for IRQ
    writel(ICPC_ROTATE, base + VT8500_ICPC_IRQ);
    writel(0x00, base + VT8500_ICPC_FIQ);
// Disable all interrupts and route them to IRQ
    for (i = 0; i < 64; i++)
    writeb(VT8500_INT_DISABLE | ICDC_IRQ, base + VT8500_ICDC + i);
    }
    static int vt8500_irq_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(virq, &vt8500_irq_chip, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops vt8500_irq_domain_ops = {
    .map = vt8500_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
pub unsafe extern "C" fn vt8500_handle_irq_common(intc: *mut vt8500_irq_data) {
    static inline void vt8500_handle_irq_common(struct vt8500_irq_data *intc)
    {
    let mut irqnr: c_ulong = readl_relaxed(intc.base) & 0x3F;
    unsigned long stat;
//
// Highest Priority register default = 63, so check that this
// is a real interrupt by checking the status register
//
    if (irqnr == 63) {
    stat = readl_relaxed(intc.base + VT8500_ICIS + 4);
    if (!(stat & BIT(31)))
    return;
    }
    generic_handle_domain_irq(intc.domain, irqnr);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry vt8500_handle_irq(struct pt_regs *regs)
    {
    vt8500_handle_irq_common(primary_intc);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_handle_irq_chained(desc: *mut irq_desc) {
    static void vt8500_handle_irq_chained(struct irq_desc *desc)
    {
    struct irq_domain *d = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct vt8500_irq_data *intc = d.host_data;
    chained_irq_enter(chip, desc);
    vt8500_handle_irq_common(intc);
    chained_irq_exit(chip, desc);
    }
    static int __init vt8500_irq_init(struct device_node *node,
    struct device_node *parent)
    {
    struct vt8500_irq_data *intc;
    int irq, i, ret = 0;
    intc = kzalloc_obj(*intc);
    if (!intc)
    return -ENOMEM;
    intc.base = of_iomap(node, 0);
    if (!intc.base) {
    pr_err("%s: Unable to map IO memory\n", __func__);
    ret = -ENOMEM;
    goto err_free;
    }
    intc.domain = irq_domain_create_linear(of_fwnode_handle(node), 64,
    &vt8500_irq_domain_ops, intc);
    if (!intc.domain) {
    pr_err("%s: Unable to add irq domain!\n", __func__);
    ret = -ENOMEM;
    goto err_unmap;
    }
    vt8500_init_irq_hw(intc.base);
    pr_info("vt8500-irq: Added interrupt controller\n");
// check if this is a chained controller
    if (of_irq_count(node) != 0) {
    for (i = 0; i < of_irq_count(node); i++) {
    irq = irq_of_parse_and_map(node, i);
    irq_set_chained_handler_and_data(irq, vt8500_handle_irq_chained,
    intc);
    }
    pr_info("vt8500-irq: Enabled slave.parent interrupts\n");
    } else {
    primary_intc = intc;
    set_handle_irq(vt8500_handle_irq);
    }
    return 0;
    err_unmap:
    iounmap(intc.base);
    err_free:
    kfree(intc);
    return ret;
    }
    IRQCHIP_DECLARE(vt8500_irq, "via,vt8500-intc", vt8500_irq_init);
