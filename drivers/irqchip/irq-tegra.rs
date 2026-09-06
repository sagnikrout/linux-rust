//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-tegra.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver code for Tegra's Legacy Interrupt Controller
//
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Heavily based on the original arch/arm/mach-tegra/irq.c code:
// Copyright (C) 2011 Google, Inc.
//
// Author:
// Colin Cross <ccross@android.com>
//
// Copyright (C) 2010,2013, NVIDIA Corporation
//

pub const ICTLR_CPU_IEP_VFIQ: c_uint = 0x08;
pub const ICTLR_CPU_IEP_FIR: c_uint = 0x14;
pub const ICTLR_CPU_IEP_FIR_SET: c_uint = 0x18;
pub const ICTLR_CPU_IEP_FIR_CLR: c_uint = 0x1c;
pub const ICTLR_CPU_IER: c_uint = 0x20;
pub const ICTLR_CPU_IER_SET: c_uint = 0x24;
pub const ICTLR_CPU_IER_CLR: c_uint = 0x28;
pub const ICTLR_CPU_IEP_CLASS: c_uint = 0x2C;
pub const ICTLR_COP_IER: c_uint = 0x30;
pub const ICTLR_COP_IER_SET: c_uint = 0x34;
pub const ICTLR_COP_IER_CLR: c_uint = 0x38;
pub const ICTLR_COP_IEP_CLASS: c_uint = 0x3c;
pub const TEGRA_MAX_NUM_ICTLRS: c_int = 6;
    static unsigned int num_ictlrs;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ictlr_soc {
    pub num_ictlrs: c_uint,
}

    static const struct tegra_ictlr_soc tegra20_ictlr_soc = {
    .num_ictlrs = 4,
    };
    static const struct tegra_ictlr_soc tegra30_ictlr_soc = {
    .num_ictlrs = 5,
    };
    static const struct tegra_ictlr_soc tegra210_ictlr_soc = {
    .num_ictlrs = 6,
    };
    static const struct of_device_id ictlr_matches[] = {
    { .compatible = "nvidia,tegra210-ictlr", .data = &tegra210_ictlr_soc },
    { .compatible = "nvidia,tegra30-ictlr", .data = &tegra30_ictlr_soc },
    { .compatible = "nvidia,tegra20-ictlr", .data = &tegra20_ictlr_soc },
    { }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_ictlr_info {
    pub base: [*mut void __iomem; TEGRA_MAX_NUM_ICTLRS],    pub cop_ier: [u32; TEGRA_MAX_NUM_ICTLRS],
    pub cop_iep: [u32; TEGRA_MAX_NUM_ICTLRS],
    pub cpu_ier: [u32; TEGRA_MAX_NUM_ICTLRS],
    pub cpu_iep: [u32; TEGRA_MAX_NUM_ICTLRS],
    pub ictlr_wake_mask: [u32; TEGRA_MAX_NUM_ICTLRS],
}

    static struct tegra_ictlr_info *lic;
#[no_mangle]
pub unsafe extern "C" fn tegra_ictlr_write_mask(d: *mut irq_data, reg: c_ulong) {
    static inline void tegra_ictlr_write_mask(struct irq_data *d, unsigned long reg)
    {
    void __iomem *base = (void __iomem  *)d.chip_data;
    u32 mask;
    mask = BIT(d.hwirq % 32);
    writel_relaxed(mask, base + reg);
    }
#[no_mangle]
unsafe extern "C" fn tegra_mask(d: *mut irq_data) {
    static void tegra_mask(struct irq_data *d)
    {
    tegra_ictlr_write_mask(d, ICTLR_CPU_IER_CLR);
    irq_chip_mask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn tegra_unmask(d: *mut irq_data) {
    static void tegra_unmask(struct irq_data *d)
    {
    tegra_ictlr_write_mask(d, ICTLR_CPU_IER_SET);
    irq_chip_unmask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn tegra_eoi(d: *mut irq_data) {
    static void tegra_eoi(struct irq_data *d)
    {
    tegra_ictlr_write_mask(d, ICTLR_CPU_IEP_FIR_CLR);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn tegra_retrigger(d: *mut irq_data) -> c_int {
    static int tegra_retrigger(struct irq_data *d)
    {
    tegra_ictlr_write_mask(d, ICTLR_CPU_IEP_FIR_SET);
    return irq_chip_retrigger_hierarchy(d);
    }

#[no_mangle]
unsafe extern "C" fn tegra_set_wake(d: *mut irq_data, enable: c_uint) -> c_int {
    static int tegra_set_wake(struct irq_data *d, unsigned int enable)
    {
    let mut irq: u32 = d.hwirq;
    u32 index, mask;
    index = (irq / 32);
    mask = BIT(irq % 32);
    if (enable)
    lic.ictlr_wake_mask[index] |= mask;
    else
    lic.ictlr_wake_mask[index] &= ~mask;
//
// Do *not* call into the parent, as the GIC doesn't have any
// wake-up facility...
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ictlr_suspend(data: *mut c_void) -> c_int {
    static int tegra_ictlr_suspend(void *data)
    {
    unsigned long flags;
    unsigned int i;
    local_irq_save(flags);
    for (i = 0; i < num_ictlrs; i++) {
    void __iomem *ictlr = lic.base[i];
// Save interrupt state
    lic.cpu_ier[i] = readl_relaxed(ictlr + ICTLR_CPU_IER);
    lic.cpu_iep[i] = readl_relaxed(ictlr + ICTLR_CPU_IEP_CLASS);
    lic.cop_ier[i] = readl_relaxed(ictlr + ICTLR_COP_IER);
    lic.cop_iep[i] = readl_relaxed(ictlr + ICTLR_COP_IEP_CLASS);
// Disable COP interrupts
    writel_relaxed(GENMASK(31, 0), ictlr + ICTLR_COP_IER_CLR);
// Disable CPU interrupts
    writel_relaxed(GENMASK(31, 0), ictlr + ICTLR_CPU_IER_CLR);
// Enable the wakeup sources of ictlr
    writel_relaxed(lic.ictlr_wake_mask[i], ictlr + ICTLR_CPU_IER_SET);
    }
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_ictlr_resume(data: *mut c_void) {
    static void tegra_ictlr_resume(void *data)
    {
    unsigned long flags;
    unsigned int i;
    local_irq_save(flags);
    for (i = 0; i < num_ictlrs; i++) {
    void __iomem *ictlr = lic.base[i];
    writel_relaxed(lic.cpu_iep[i],
    ictlr + ICTLR_CPU_IEP_CLASS);
    writel_relaxed(GENMASK(31, 0), ictlr + ICTLR_CPU_IER_CLR);
    writel_relaxed(lic.cpu_ier[i],
    ictlr + ICTLR_CPU_IER_SET);
    writel_relaxed(lic.cop_iep[i],
    ictlr + ICTLR_COP_IEP_CLASS);
    writel_relaxed(GENMASK(31, 0), ictlr + ICTLR_COP_IER_CLR);
    writel_relaxed(lic.cop_ier[i],
    ictlr + ICTLR_COP_IER_SET);
    }
    local_irq_restore(flags);
    }
    static const struct syscore_ops tegra_ictlr_syscore_ops = {
    .suspend	= tegra_ictlr_suspend,
    .resume		= tegra_ictlr_resume,
    };
    static struct syscore tegra_ictlr_syscore = {
    .ops = &tegra_ictlr_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn tegra_ictlr_syscore_init() {
    static void tegra_ictlr_syscore_init(void)
    {
    register_syscore(&tegra_ictlr_syscore);
    }

    static inline void tegra_ictlr_syscore_init(void) {}

    static struct irq_chip tegra_ictlr_chip = {
    .name			= "LIC",
    .irq_eoi		= tegra_eoi,
    .irq_mask		= tegra_mask,
    .irq_unmask		= tegra_unmask,
    .irq_retrigger		= tegra_retrigger,
    .irq_set_wake		= tegra_set_wake,
    .irq_set_type		= irq_chip_set_type_parent,
    .flags			= IRQCHIP_MASK_ON_SUSPEND,

    .irq_set_affinity	= irq_chip_set_affinity_parent,

    };
    static int tegra_ictlr_domain_translate(struct irq_domain *d,
    struct irq_fwspec *fwspec,
    unsigned long *hwirq,
    unsigned int *type)
    {
    if (is_of_node(fwspec.fwnode)) {
    if (fwspec.param_count != 3)
    return -EINVAL;
// No PPI should point to this domain
    if (fwspec.param[0] != 0)
    return -EINVAL;
// hwirq = fwspec->param[1];
// type = fwspec->param[2] & IRQ_TYPE_SENSE_MASK;
    return 0;
    }
    return -EINVAL;
    }
    static int tegra_ictlr_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct irq_fwspec *fwspec = data;
    struct irq_fwspec parent_fwspec;
    struct tegra_ictlr_info *info = domain.host_data;
    irq_hw_number_t hwirq;
    unsigned int i;
    if (fwspec.param_count != 3)
    return -EINVAL;	/* Not GIC compliant */
    if (fwspec.param[0] != GIC_SPI)
    return -EINVAL;	/* No PPI should point to this domain */
    hwirq = fwspec.param[1];
    if (hwirq >= (num_ictlrs * 32))
    return -EINVAL;
    for (i = 0; i < nr_irqs; i++) {
    let mut ictlr: c_int = (hwirq + i) / 32;
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &tegra_ictlr_chip,
    (void  *)info.base[ictlr]);
    }
    parent_fwspec = *fwspec;
    parent_fwspec.fwnode = domain.parent.fwnode;
    return irq_domain_alloc_irqs_parent(domain, virq, nr_irqs,
    &parent_fwspec);
    }
    static const struct irq_domain_ops tegra_ictlr_domain_ops = {
    .translate	= tegra_ictlr_domain_translate,
    .alloc		= tegra_ictlr_domain_alloc,
    .free		= irq_domain_free_irqs_common,
    };
    static int __init tegra_ictlr_init(struct device_node *node,
    struct device_node *parent)
    {
    struct irq_domain *parent_domain, *domain;
    const struct of_device_id *match;
    const struct tegra_ictlr_soc *soc;
    unsigned int i;
    int err;
    if (!parent) {
    pr_err("%pOF: no parent, giving up\n", node);
    return -ENODEV;
    }
    parent_domain = irq_find_host(parent);
    if (!parent_domain) {
    pr_err("%pOF: unable to obtain parent domain\n", node);
    return -ENXIO;
    }
    match = of_match_node(ictlr_matches, node);
    if (!match)		/* Should never happen... */
    return -ENODEV;
    soc = match.data;
    lic = kzalloc_obj(*lic);
    if (!lic)
    return -ENOMEM;
    for (i = 0; i < TEGRA_MAX_NUM_ICTLRS; i++) {
    void __iomem *base;
    base = of_iomap(node, i);
    if (!base)
    break;
    lic.base[i] = base;
// Disable all interrupts
    writel_relaxed(GENMASK(31, 0), base + ICTLR_CPU_IER_CLR);
// All interrupts target IRQ
    writel_relaxed(0, base + ICTLR_CPU_IEP_CLASS);
    num_ictlrs++;
    }
    if (!num_ictlrs) {
    pr_err("%pOF: no valid regions, giving up\n", node);
    err = -ENOMEM;
    goto out_free;
    }
    WARN(num_ictlrs != soc.num_ictlrs,
    "%pOF: Found %u interrupt controllers in DT; expected %u.\n",
    node, num_ictlrs, soc.num_ictlrs);
    domain = irq_domain_create_hierarchy(parent_domain, 0, num_ictlrs * 32,
    of_fwnode_handle(node), &tegra_ictlr_domain_ops, lic);
    if (!domain) {
    pr_err("%pOF: failed to allocated domain\n", node);
    err = -ENOMEM;
    goto out_unmap;
    }
    tegra_ictlr_syscore_init();
    pr_info("%pOF: %d interrupts forwarded to %pOF\n",
    node, num_ictlrs * 32, parent);
    return 0;
    out_unmap:
    for (i = 0; i < num_ictlrs; i++)
    iounmap(lic.base[i]);
    out_free:
    kfree(lic);
    return err;
    }
    IRQCHIP_DECLARE(tegra20_ictlr, "nvidia,tegra20-ictlr", tegra_ictlr_init);
    IRQCHIP_DECLARE(tegra30_ictlr, "nvidia,tegra30-ictlr", tegra_ictlr_init);
    IRQCHIP_DECLARE(tegra210_ictlr, "nvidia,tegra210-ictlr", tegra_ictlr_init);
