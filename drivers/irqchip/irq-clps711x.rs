//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-clps711x.c
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
// CLPS711X IRQ driver
//
// Copyright (C) 2013 Alexander Shiyan <shc_work@mail.ru>
//

    static const struct {

    unsigned int	flags;
    phys_addr_t	eoi;
    } clps711x_irqs[] = {
    [1]	= { CLPS711X_FLAG_FIQ, CLPS711X_BLEOI, },
    [3]	= { CLPS711X_FLAG_FIQ, CLPS711X_MCEOI, },
    [4]	= { CLPS711X_FLAG_EN, CLPS711X_COEOI, },
    [5]	= { CLPS711X_FLAG_EN, },
    [6]	= { CLPS711X_FLAG_EN, },
    [7]	= { CLPS711X_FLAG_EN, },
    [8]	= { CLPS711X_FLAG_EN, CLPS711X_TC1EOI, },
    [9]	= { CLPS711X_FLAG_EN, CLPS711X_TC2EOI, },
    [10]	= { CLPS711X_FLAG_EN, CLPS711X_RTCEOI, },
    [11]	= { CLPS711X_FLAG_EN, CLPS711X_TEOI, },
    [12]	= { CLPS711X_FLAG_EN, },
    [13]	= { CLPS711X_FLAG_EN, },
    [14]	= { CLPS711X_FLAG_EN, CLPS711X_UMSEOI, },
    [15]	= { CLPS711X_FLAG_EN, CLPS711X_SRXEOF, },
    [16]	= { CLPS711X_FLAG_EN, CLPS711X_KBDEOI, },
    [17]	= { CLPS711X_FLAG_EN, },
    [18]	= { CLPS711X_FLAG_EN, },
    [28]	= { CLPS711X_FLAG_EN, },
    [29]	= { CLPS711X_FLAG_EN, },
    [32]	= { CLPS711X_FLAG_FIQ, },
    };
    static struct {
    void __iomem		*base;
    void __iomem		*intmr[3];
    void __iomem		*intsr[3];
    struct irq_domain	*domain;
    struct irq_domain_ops	ops;
    } *clps711x_intc;
#[no_mangle]
unsafe extern "C" fn clps711x_irqh(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry clps711x_irqh(struct pt_regs *regs)
    {
    u32 irqstat;
    do {
    irqstat = readw_relaxed(clps711x_intc.intmr[0]) &
    readw_relaxed(clps711x_intc.intsr[0]);
    if (irqstat)
    generic_handle_domain_irq(clps711x_intc.domain,
    fls(irqstat) - 1);
    irqstat = readw_relaxed(clps711x_intc.intmr[1]) &
    readw_relaxed(clps711x_intc.intsr[1]);
    if (irqstat)
    generic_handle_domain_irq(clps711x_intc.domain,
    fls(irqstat) - 1 + 16);
    } while (irqstat);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_intc_eoi(d: *mut irq_data) {
    static void clps711x_intc_eoi(struct irq_data *d)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    writel_relaxed(0, clps711x_intc.base + clps711x_irqs[hwirq].eoi);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_intc_mask(d: *mut irq_data) {
    static void clps711x_intc_mask(struct irq_data *d)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *intmr = clps711x_intc.intmr[hwirq / 16];
    u32 tmp;
    tmp = readl_relaxed(intmr);
    tmp &= ~(1 << (hwirq % 16));
    writel_relaxed(tmp, intmr);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_intc_unmask(d: *mut irq_data) {
    static void clps711x_intc_unmask(struct irq_data *d)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *intmr = clps711x_intc.intmr[hwirq / 16];
    u32 tmp;
    tmp = readl_relaxed(intmr);
    tmp |= 1 << (hwirq % 16);
    writel_relaxed(tmp, intmr);
    }
    static struct irq_chip clps711x_intc_chip = {
    .name		= "clps711x-intc",
    .irq_eoi	= clps711x_intc_eoi,
    .irq_mask	= clps711x_intc_mask,
    .irq_unmask	= clps711x_intc_unmask,
    };
    static int __init clps711x_intc_irq_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    let mut handler: irq_flow_handler_t = handle_level_irq;
    let mut flags: c_uint = 0;
    if (!clps711x_irqs[hw].flags)
    return 0;
    if (clps711x_irqs[hw].flags & CLPS711X_FLAG_FIQ) {
    handler = handle_bad_irq;
    flags |= IRQ_NOAUTOEN;
    } else if (clps711x_irqs[hw].eoi) {
    handler = handle_fasteoi_irq;
    }
// Clear down pending interrupt
    if (clps711x_irqs[hw].eoi)
    writel_relaxed(0, clps711x_intc.base + clps711x_irqs[hw].eoi);
    irq_set_chip_and_handler(virq, &clps711x_intc_chip, handler);
    irq_modify_status(virq, IRQ_NOPROBE, flags);
    return 0;
    }
    static int __init _clps711x_intc_init(struct device_node *np,
    phys_addr_t base, resource_size_t size)
    {
    int err;
    clps711x_intc = kzalloc_obj(*clps711x_intc);
    if (!clps711x_intc)
    return -ENOMEM;
    clps711x_intc.base = ioremap(base, size);
    if (!clps711x_intc.base) {
    err = -ENOMEM;
    goto out_kfree;
    }
    clps711x_intc.intsr[0] = clps711x_intc.base + CLPS711X_INTSR1;
    clps711x_intc.intmr[0] = clps711x_intc.base + CLPS711X_INTMR1;
    clps711x_intc.intsr[1] = clps711x_intc.base + CLPS711X_INTSR2;
    clps711x_intc.intmr[1] = clps711x_intc.base + CLPS711X_INTMR2;
    clps711x_intc.intsr[2] = clps711x_intc.base + CLPS711X_INTSR3;
    clps711x_intc.intmr[2] = clps711x_intc.base + CLPS711X_INTMR3;
// Mask all interrupts
    writel_relaxed(0, clps711x_intc.intmr[0]);
    writel_relaxed(0, clps711x_intc.intmr[1]);
    writel_relaxed(0, clps711x_intc.intmr[2]);
    err = irq_alloc_descs(-1, 0, ARRAY_SIZE(clps711x_irqs), numa_node_id());
    if (err < 0)
    goto out_iounmap;
    clps711x_intc.ops.map = clps711x_intc_irq_map;
    clps711x_intc.ops.xlate = irq_domain_xlate_onecell;
    clps711x_intc.domain =
    irq_domain_create_legacy(of_fwnode_handle(np), ARRAY_SIZE(clps711x_irqs), 0, 0,
    &clps711x_intc.ops, core::ptr::null_mut());
    if (!clps711x_intc.domain) {
    err = -ENOMEM;
    goto out_irqfree;
    }
    irq_set_default_domain(clps711x_intc.domain);
    set_handle_irq(clps711x_irqh);

    init_FIQ(0);

    return 0;
    out_irqfree:
    irq_free_descs(0, ARRAY_SIZE(clps711x_irqs));
    out_iounmap:
    iounmap(clps711x_intc.base);
    out_kfree:
    kfree(clps711x_intc);
    return err;
    }
    static int __init clps711x_intc_init_dt(struct device_node *np,
    struct device_node *parent)
    {
    struct resource res;
    int err;
    err = of_address_to_resource(np, 0, &res);
    if (err)
    return err;
    return _clps711x_intc_init(np, res.start, resource_size(&res));
    }
    IRQCHIP_DECLARE(clps711x, "cirrus,ep7209-intc", clps711x_intc_init_dt);
