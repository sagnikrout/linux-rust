//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ftintc010.c
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


// SPDX-License-Identifier: GPL-2.0
//
// irqchip for the Faraday Technology FTINTC010 Copyright (C) 2017 Linus
// Walleij <linus.walleij@linaro.org>
//
// Based on arch/arm/mach-gemini/irq.c
// Copyright (C) 2001-2006 Storlink, Corp.
// Copyright (C) 2008-2009 Paulius Zaleckas <paulius.zaleckas@gmail.com>
//

pub const FT010_NUM_IRQS: c_int = 32;

// Selects level- or edge-triggered

// Selects active low/high or falling/rising edge

//
// struct ft010_irq_data - irq data container for the Faraday IRQ controller
// @base: memory offset in virtual memory
// @chip: chip container for this instance
// @domain: IRQ domain for this instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ft010_irq_data {
    pub base: *mut void __iomem,
    pub chip: irq_chip,
    pub domain: *mut irq_domain,
}

#[no_mangle]
unsafe extern "C" fn ft010_irq_mask(d: *mut irq_data) {
    static void ft010_irq_mask(struct irq_data *d)
    {
    struct ft010_irq_data *f = irq_data_get_irq_chip_data(d);
    unsigned int mask;
    mask = readl(FT010_IRQ_MASK(f.base));
    mask &= ~BIT(irqd_to_hwirq(d));
    writel(mask, FT010_IRQ_MASK(f.base));
    }
#[no_mangle]
unsafe extern "C" fn ft010_irq_unmask(d: *mut irq_data) {
    static void ft010_irq_unmask(struct irq_data *d)
    {
    struct ft010_irq_data *f = irq_data_get_irq_chip_data(d);
    unsigned int mask;
    mask = readl(FT010_IRQ_MASK(f.base));
    mask |= BIT(irqd_to_hwirq(d));
    writel(mask, FT010_IRQ_MASK(f.base));
    }
#[no_mangle]
unsafe extern "C" fn ft010_irq_ack(d: *mut irq_data) {
    static void ft010_irq_ack(struct irq_data *d)
    {
    struct ft010_irq_data *f = irq_data_get_irq_chip_data(d);
    writel(BIT(irqd_to_hwirq(d)), FT010_IRQ_CLEAR(f.base));
    }
#[no_mangle]
unsafe extern "C" fn ft010_irq_set_type(d: *mut irq_data, trigger: c_uint) -> c_int {
    static int ft010_irq_set_type(struct irq_data *d, unsigned int trigger)
    {
    struct ft010_irq_data *f = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = irqd_to_hwirq(d);
    u32 mode, polarity;
    mode = readl(FT010_IRQ_MODE(f.base));
    polarity = readl(FT010_IRQ_POLARITY(f.base));
    if (trigger & (IRQ_TYPE_LEVEL_LOW)) {
    irq_set_handler_locked(d, handle_level_irq);
    mode &= ~BIT(offset);
    polarity |= BIT(offset);
    } else if (trigger & (IRQ_TYPE_LEVEL_HIGH)) {
    irq_set_handler_locked(d, handle_level_irq);
    mode &= ~BIT(offset);
    polarity &= ~BIT(offset);
    } else if (trigger & IRQ_TYPE_EDGE_FALLING) {
    irq_set_handler_locked(d, handle_edge_irq);
    mode |= BIT(offset);
    polarity |= BIT(offset);
    } else if (trigger & IRQ_TYPE_EDGE_RISING) {
    irq_set_handler_locked(d, handle_edge_irq);
    mode |= BIT(offset);
    polarity &= ~BIT(offset);
    } else {
    irq_set_handler_locked(d, handle_bad_irq);
    pr_warn("Faraday IRQ: no supported trigger selected for line %d\n",
    offset);
    }
    writel(mode, FT010_IRQ_MODE(f.base));
    writel(polarity, FT010_IRQ_POLARITY(f.base));
    return 0;
    }
    static struct irq_chip ft010_irq_chip = {
    .name		= "FTINTC010",
    .irq_ack	= ft010_irq_ack,
    .irq_mask	= ft010_irq_mask,
    .irq_unmask	= ft010_irq_unmask,
    .irq_set_type	= ft010_irq_set_type,
    };
// Local static for the IRQ entry call
    static struct ft010_irq_data firq;
#[no_mangle]
unsafe extern "C" fn ft010_irqchip_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry ft010_irqchip_handle_irq(struct pt_regs *regs)
    {
    struct ft010_irq_data *f = &firq;
    int irq;
    u32 status;
    while ((status = readl(FT010_IRQ_STATUS(f.base)))) {
    irq = ffs(status) - 1;
    generic_handle_domain_irq(f.domain, irq);
    }
    }
    static int ft010_irqdomain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct ft010_irq_data *f = d.host_data;
    irq_set_chip_data(irq, f);
// All IRQs should set up their type, flags as bad by default
    irq_set_chip_and_handler(irq, &ft010_irq_chip, handle_bad_irq);
    irq_set_probe(irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ft010_irqdomain_unmap(d: *mut irq_domain, irq: c_uint) {
    static void ft010_irqdomain_unmap(struct irq_domain *d, unsigned int irq)
    {
    irq_set_chip_and_handler(irq, core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_chip_data(irq, core::ptr::null_mut());
    }
    static const struct irq_domain_ops ft010_irqdomain_ops = {
    .map = ft010_irqdomain_map,
    .unmap = ft010_irqdomain_unmap,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static int __init ft010_of_init_irq(struct device_node *node,
    struct device_node *parent)
    {
    struct ft010_irq_data *f = &firq;
//
// Disable the idle handler by default since it is buggy
// For more info see arch/arm/mach-gemini/idle.c
//
    cpu_idle_poll_ctrl(true);
    f.base = of_iomap(node, 0);
    WARN(!f.base, "unable to map gemini irq registers\n");
// Disable all interrupts
    writel(0, FT010_IRQ_MASK(f.base));
    writel(0, FT010_FIQ_MASK(f.base));
    f.domain = irq_domain_create_simple(of_fwnode_handle(node),
    FT010_NUM_IRQS, 0,
    &ft010_irqdomain_ops, f);
    set_handle_irq(ft010_irqchip_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(faraday, "faraday,ftintc010",
    ft010_of_init_irq);
    IRQCHIP_DECLARE(gemini, "cortina,gemini-interrupt-controller",
    ft010_of_init_irq);
    IRQCHIP_DECLARE(moxa, "moxa,moxart-ic",
    ft010_of_init_irq);
