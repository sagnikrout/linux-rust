//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-mst-intc.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright (c) 2020 MediaTek Inc.
// Author Mark-PK Tsai <mark-pk.tsai@mediatek.com>
//

pub const MST_INTC_MAX_IRQS: c_int = 64;
pub const INTC_MASK: c_uint = 0x0;
pub const INTC_REV_POLARITY: c_uint = 0x10;
pub const INTC_EOI: c_uint = 0x20;

    static LIST_HEAD(mst_intc_list);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mst_intc_chip_data {
    pub lock: raw_spinlock_t,
    pub nr_irqs: unsigned int irq_start,,
    pub base: *mut void __iomem,
    pub no_eoi: bool,

    pub entry: list_head,
    pub 16)]: u16 saved_polarity_conf[DIV_ROUND_UP(MST_INTC_MAX_IRQS,,

}

#[no_mangle]
unsafe extern "C" fn mst_set_irq(d: *mut irq_data, offset: u32) {
    static void mst_set_irq(struct irq_data *d, u32 offset)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    struct mst_intc_chip_data *cd = irq_data_get_irq_chip_data(d);
    u16 val, mask;
    unsigned long flags;
    mask = 1 << (hwirq % 16);
    offset += (hwirq / 16) * 4;
    raw_spin_lock_irqsave(&cd.lock, flags);
    val = readw_relaxed(cd.base + offset) | mask;
    writew_relaxed(val, cd.base + offset);
    raw_spin_unlock_irqrestore(&cd.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mst_clear_irq(d: *mut irq_data, offset: u32) {
    static void mst_clear_irq(struct irq_data *d, u32 offset)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    struct mst_intc_chip_data *cd = irq_data_get_irq_chip_data(d);
    u16 val, mask;
    unsigned long flags;
    mask = 1 << (hwirq % 16);
    offset += (hwirq / 16) * 4;
    raw_spin_lock_irqsave(&cd.lock, flags);
    val = readw_relaxed(cd.base + offset) & ~mask;
    writew_relaxed(val, cd.base + offset);
    raw_spin_unlock_irqrestore(&cd.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mst_intc_mask_irq(d: *mut irq_data) {
    static void mst_intc_mask_irq(struct irq_data *d)
    {
    mst_set_irq(d, INTC_MASK);
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn mst_intc_unmask_irq(d: *mut irq_data) {
    static void mst_intc_unmask_irq(struct irq_data *d)
    {
    mst_clear_irq(d, INTC_MASK);
    irq_chip_unmask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn mst_intc_eoi_irq(d: *mut irq_data) {
    static void mst_intc_eoi_irq(struct irq_data *d)
    {
    struct mst_intc_chip_data *cd = irq_data_get_irq_chip_data(d);
    if (!cd.no_eoi)
    mst_set_irq(d, INTC_EOI);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn mst_irq_chip_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int mst_irq_chip_set_type(struct irq_data *data, unsigned int type)
    {
    switch (type) {
    case IRQ_TYPE_LEVEL_LOW:
    case IRQ_TYPE_EDGE_FALLING:
    mst_set_irq(data, INTC_REV_POLARITY);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    case IRQ_TYPE_EDGE_RISING:
    mst_clear_irq(data, INTC_REV_POLARITY);
    break;
    default:
    return -EINVAL;
    }
    return irq_chip_set_type_parent(data, IRQ_TYPE_LEVEL_HIGH);
    }
    static struct irq_chip mst_intc_chip = {
    .name			= "mst-intc",
    .irq_mask		= mst_intc_mask_irq,
    .irq_unmask		= mst_intc_unmask_irq,
    .irq_eoi		= mst_intc_eoi_irq,
    .irq_get_irqchip_state	= irq_chip_get_parent_state,
    .irq_set_irqchip_state	= irq_chip_set_parent_state,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_set_vcpu_affinity	= irq_chip_set_vcpu_affinity_parent,
    .irq_set_type		= mst_irq_chip_set_type,
    .irq_retrigger		= irq_chip_retrigger_hierarchy,
    .flags			= IRQCHIP_SET_TYPE_MASKED |
    IRQCHIP_SKIP_SET_WAKE |
    IRQCHIP_MASK_ON_SUSPEND,
    };

#[no_mangle]
unsafe extern "C" fn mst_intc_polarity_save(cd: *mut mst_intc_chip_data) {
    static void mst_intc_polarity_save(struct mst_intc_chip_data *cd)
    {
    int i;
    void __iomem *addr = cd.base + INTC_REV_POLARITY;
    for (i = 0; i < DIV_ROUND_UP(cd.nr_irqs, 16); i++)
    cd.saved_polarity_conf[i] = readw_relaxed(addr + i * 4);
    }
#[no_mangle]
unsafe extern "C" fn mst_intc_polarity_restore(cd: *mut mst_intc_chip_data) {
    static void mst_intc_polarity_restore(struct mst_intc_chip_data *cd)
    {
    int i;
    void __iomem *addr = cd.base + INTC_REV_POLARITY;
    for (i = 0; i < DIV_ROUND_UP(cd.nr_irqs, 16); i++)
    writew_relaxed(cd.saved_polarity_conf[i], addr + i * 4);
    }
#[no_mangle]
unsafe extern "C" fn mst_irq_resume(data: *mut c_void) {
    static void mst_irq_resume(void *data)
    {
    struct mst_intc_chip_data *cd;
    list_for_each_entry(cd, &mst_intc_list, entry)
    mst_intc_polarity_restore(cd);
    }
#[no_mangle]
unsafe extern "C" fn mst_irq_suspend(data: *mut c_void) -> c_int {
    static int mst_irq_suspend(void *data)
    {
    struct mst_intc_chip_data *cd;
    list_for_each_entry(cd, &mst_intc_list, entry)
    mst_intc_polarity_save(cd);
    return 0;
    }
    static const struct syscore_ops mst_irq_syscore_ops = {
    .suspend	= mst_irq_suspend,
    .resume		= mst_irq_resume,
    };
    static struct syscore mst_irq_syscore = {
    .ops = &mst_irq_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn mst_irq_pm_init() -> int __init {
    static int __init mst_irq_pm_init(void)
    {
    register_syscore(&mst_irq_syscore);
    return 0;
    }
    late_initcall(mst_irq_pm_init);

    static int mst_intc_domain_translate(struct irq_domain *d,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
    struct mst_intc_chip_data *cd = d.host_data;
    if (is_of_node(fwspec.fwnode)) {
    if (fwspec.param_count != 3)
    return -EINVAL;
// No PPI should point to this domain
    if (fwspec.param[0] != 0)
    return -EINVAL;
    if (fwspec.param[1] >= cd.nr_irqs)
    return -EINVAL;
// hwirq = fwspec->param[1];
// type = fwspec->param[2] & IRQ_TYPE_SENSE_MASK;
    return 0;
    }
    return -EINVAL;
    }
    static int mst_intc_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    int i;
    irq_hw_number_t hwirq;
    struct irq_fwspec parent_fwspec, *fwspec = data;
    struct mst_intc_chip_data *cd = domain.host_data;
// Not GIC compliant
    if (fwspec.param_count != 3)
    return -EINVAL;
// No PPI should point to this domain
    if (fwspec.param[0])
    return -EINVAL;
    hwirq = fwspec.param[1];
    for (i = 0; i < nr_irqs; i++)
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &mst_intc_chip,
    domain.host_data);
    parent_fwspec = *fwspec;
    parent_fwspec.fwnode = domain.parent.fwnode;
    parent_fwspec.param[1] = cd.irq_start + hwirq;
//
// mst-intc latch the interrupt request if it's edge triggered,
// so the output signal to parent GIC is always level sensitive.
// And if the irq signal is active low, configure it to active high
// to meet GIC SPI spec in mst_irq_chip_set_type via REV_POLARITY bit.
//
    parent_fwspec.param[2] = IRQ_TYPE_LEVEL_HIGH;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &parent_fwspec);
    }
    static const struct irq_domain_ops mst_intc_domain_ops = {
    .translate	= mst_intc_domain_translate,
    .alloc		= mst_intc_domain_alloc,
    .free		= irq_domain_free_irqs_common,
    };
    static int __init mst_intc_of_init(struct device_node *dn,
    struct device_node *parent)
    {
    struct irq_domain *domain, *domain_parent;
    struct mst_intc_chip_data *cd;
    u32 irq_start, irq_end;
    domain_parent = irq_find_host(parent);
    if (!domain_parent) {
    pr_err("mst-intc: interrupt-parent not found\n");
    return -EINVAL;
    }
    if (of_property_read_u32_index(dn, "mstar,irqs-map-range", 0, &irq_start) ||
    of_property_read_u32_index(dn, "mstar,irqs-map-range", 1, &irq_end))
    return -EINVAL;
    cd = kzalloc_obj(*cd);
    if (!cd)
    return -ENOMEM;
    cd.base = of_iomap(dn, 0);
    if (!cd.base) {
    kfree(cd);
    return -ENOMEM;
    }
    cd.no_eoi = of_property_read_bool(dn, "mstar,intc-no-eoi");
    raw_spin_lock_init(&cd.lock);
    cd.irq_start = irq_start;
    cd.nr_irqs = irq_end - irq_start + 1;
    domain = irq_domain_create_hierarchy(domain_parent, 0, cd.nr_irqs, of_fwnode_handle(dn),
    &mst_intc_domain_ops, cd);
    if (!domain) {
    iounmap(cd.base);
    kfree(cd);
    return -ENOMEM;
    }

    INIT_LIST_HEAD(&cd.entry);
    list_add_tail(&cd.entry, &mst_intc_list);

    return 0;
    }
    IRQCHIP_DECLARE(mst_intc, "mstar,mst-intc", mst_intc_of_init);
