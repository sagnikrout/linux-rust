//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-loongson-pch-lpc.c
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
// Loongson LPC Interrupt Controller support
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

// Registers
pub const LPC_INT_CTL: c_uint = 0x00;
pub const LPC_INT_ENA: c_uint = 0x04;
pub const LPC_INT_STS: c_uint = 0x08;
pub const LPC_INT_CLR: c_uint = 0x0c;
pub const LPC_INT_POL: c_uint = 0x10;
pub const LPC_COUNT: c_int = 16;
// LPC_INT_CTL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pch_lpc {
    pub base: *mut void __iomem,
    pub lpc_domain: *mut irq_domain,
    pub lpc_lock: raw_spinlock_t,
    pub saved_reg_ctl: u32,
    pub saved_reg_ena: u32,
    pub saved_reg_pol: u32,
}

    static struct pch_lpc *pch_lpc_priv;
    struct fwnode_handle *pch_lpc_handle;
#[no_mangle]
unsafe extern "C" fn lpc_irq_ack(d: *mut irq_data) {
    static void lpc_irq_ack(struct irq_data *d)
    {
    unsigned long flags;
    struct pch_lpc *priv = d.domain.host_data;
    raw_spin_lock_irqsave(&priv.lpc_lock, flags);
    writel(0x1 << d.hwirq, priv.base + LPC_INT_CLR);
    raw_spin_unlock_irqrestore(&priv.lpc_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn lpc_irq_mask(d: *mut irq_data) {
    static void lpc_irq_mask(struct irq_data *d)
    {
    unsigned long flags;
    struct pch_lpc *priv = d.domain.host_data;
    raw_spin_lock_irqsave(&priv.lpc_lock, flags);
    writel(readl(priv.base + LPC_INT_ENA) & (~(0x1 << (d.hwirq))),
    priv.base + LPC_INT_ENA);
    raw_spin_unlock_irqrestore(&priv.lpc_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn lpc_irq_unmask(d: *mut irq_data) {
    static void lpc_irq_unmask(struct irq_data *d)
    {
    unsigned long flags;
    struct pch_lpc *priv = d.domain.host_data;
    raw_spin_lock_irqsave(&priv.lpc_lock, flags);
    writel(readl(priv.base + LPC_INT_ENA) | (0x1 << (d.hwirq)),
    priv.base + LPC_INT_ENA);
    raw_spin_unlock_irqrestore(&priv.lpc_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn lpc_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int lpc_irq_set_type(struct irq_data *d, unsigned int type)
    {
    u32 val;
    let mut mask: u32 = 0x1 << (d.hwirq);
    struct pch_lpc *priv = d.domain.host_data;
    if (!(type & IRQ_TYPE_LEVEL_MASK))
    return 0;
    val = readl(priv.base + LPC_INT_POL);
    if (type == IRQ_TYPE_LEVEL_HIGH)
    val |= mask;
    else
    val &= ~mask;
    writel(val, priv.base + LPC_INT_POL);
    return 0;
    }
    static const struct irq_chip pch_lpc_irq_chip = {
    .name			= "PCH LPC",
    .irq_mask		= lpc_irq_mask,
    .irq_unmask		= lpc_irq_unmask,
    .irq_ack		= lpc_irq_ack,
    .irq_set_type		= lpc_irq_set_type,
    .flags			= IRQCHIP_SKIP_SET_WAKE,
    };
#[no_mangle]
unsafe extern "C" fn lpc_irq_dispatch(desc: *mut irq_desc) {
    static void lpc_irq_dispatch(struct irq_desc *desc)
    {
    u32 pending, bit;
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct pch_lpc *priv = irq_desc_get_handler_data(desc);
    chained_irq_enter(chip, desc);
    pending = readl(priv.base + LPC_INT_ENA);
    pending &= readl(priv.base + LPC_INT_STS);
    if (!pending)
    spurious_interrupt();
    while (pending) {
    bit = __ffs(pending);
    generic_handle_domain_irq(priv.lpc_domain, bit);
    pending &= ~BIT(bit);
    }
    chained_irq_exit(chip, desc);
    }
    static int pch_lpc_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(irq, &pch_lpc_irq_chip, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops pch_lpc_domain_ops = {
    .map 		= pch_lpc_map,
    .translate	= irq_domain_translate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn pch_lpc_reset(priv: *mut pch_lpc) {
    static void pch_lpc_reset(struct pch_lpc *priv)
    {
// Enable the LPC interrupt, bit31: en  bit30: edge
    writel(LPC_INT_CTL_EN, priv.base + LPC_INT_CTL);
    writel(0, priv.base + LPC_INT_ENA);
// Clear all 18-bit interrpt bit
    writel(GENMASK(17, 0), priv.base + LPC_INT_CLR);
    }
#[no_mangle]
unsafe extern "C" fn pch_lpc_disabled(priv: *mut pch_lpc) -> c_int {
    static int pch_lpc_disabled(struct pch_lpc *priv)
    {
    return (readl(priv.base + LPC_INT_ENA) == 0xffffffff) &&
    (readl(priv.base + LPC_INT_STS) == 0xffffffff);
    }
#[no_mangle]
unsafe extern "C" fn pch_lpc_suspend(data: *mut c_void) -> c_int {
    static int pch_lpc_suspend(void *data)
    {
    pch_lpc_priv.saved_reg_ctl = readl(pch_lpc_priv.base + LPC_INT_CTL);
    pch_lpc_priv.saved_reg_ena = readl(pch_lpc_priv.base + LPC_INT_ENA);
    pch_lpc_priv.saved_reg_pol = readl(pch_lpc_priv.base + LPC_INT_POL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pch_lpc_resume(data: *mut c_void) {
    static void pch_lpc_resume(void *data)
    {
    writel(pch_lpc_priv.saved_reg_ctl, pch_lpc_priv.base + LPC_INT_CTL);
    writel(pch_lpc_priv.saved_reg_ena, pch_lpc_priv.base + LPC_INT_ENA);
    writel(pch_lpc_priv.saved_reg_pol, pch_lpc_priv.base + LPC_INT_POL);
    }
    static const struct syscore_ops pch_lpc_syscore_ops = {
    .suspend = pch_lpc_suspend,
    .resume = pch_lpc_resume,
    };
    static struct syscore pch_lpc_syscore = {
    .ops = &pch_lpc_syscore_ops,
    };
    static int __init pch_lpc_init(phys_addr_t addr, unsigned long size,
    struct fwnode_handle *irq_handle, int parent_irq)
    {
    struct pch_lpc *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.lpc_lock);
    priv.base = ioremap(addr, size);
    if (!priv.base)
    goto free_priv;
    if (pch_lpc_disabled(priv)) {
    pr_err("Failed to get LPC status\n");
    goto iounmap_base;
    }
//
// The LPC interrupt controller is a legacy i8259-compatible device,
// which requires a static 1:1 mapping for IRQs 0-15.
// Use irq_domain_create_legacy to establish this static mapping early.
//
    priv.lpc_domain = irq_domain_create_legacy(irq_handle, LPC_COUNT, 0, 0,
    &pch_lpc_domain_ops, priv);
    if (!priv.lpc_domain) {
    pr_err("Failed to create IRQ domain\n");
    goto iounmap_base;
    }
    pch_lpc_reset(priv);
    irq_set_chained_handler_and_data(parent_irq, lpc_irq_dispatch, priv);
    pch_lpc_priv = priv;
    pch_lpc_handle = irq_handle;
    register_syscore(&pch_lpc_syscore);
    return 0;
    iounmap_base:
    iounmap(priv.base);
    free_priv:
    kfree(priv);
    return -ENOMEM;
    }

#[no_mangle]
pub unsafe extern "C" fn pch_lpc_acpi_init(parent: *mut irq_domain, acpi_pchlpc: *mut acpi_madt_lpc_pic) -> int __init {
    int __init pch_lpc_acpi_init(struct irq_domain *parent, struct acpi_madt_lpc_pic *acpi_pchlpc)
    {
    struct fwnode_handle *irq_handle;
    struct irq_fwspec fwspec;
    int parent_irq, ret;
    irq_handle = irq_domain_alloc_named_fwnode("lpcintc");
    if (!irq_handle) {
    pr_err("Unable to allocate domain handle\n");
    return -ENOMEM;
    }
    fwspec.fwnode = parent.fwnode;
    fwspec.param[0] = acpi_pchlpc.cascade + GSI_MIN_PCH_IRQ;
    fwspec.param[1] = IRQ_TYPE_LEVEL_HIGH;
    fwspec.param_count = 2;
    parent_irq = irq_create_fwspec_mapping(&fwspec);
    if (parent_irq <= 0) {
    pr_err("Unable to map LPC parent interrupt\n");
    irq_domain_free_fwnode(irq_handle);
    return -ENOMEM;
    }
    ret = pch_lpc_init(acpi_pchlpc.address, acpi_pchlpc.size, irq_handle, parent_irq);
    if (ret) {
    irq_dispose_mapping(parent_irq);
    irq_domain_free_fwnode(irq_handle);
    return ret;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pch_lpc_of_init(node: *mut device_node, parent: *mut device_node) -> int __init {
    static int __init pch_lpc_of_init(struct device_node *node, struct device_node *parent)
    {
    struct fwnode_handle *irq_handle;
    struct resource res;
    int parent_irq, ret;
    if (of_address_to_resource(node, 0, &res))
    return -EINVAL;
    parent_irq = irq_of_parse_and_map(node, 0);
    if (!parent_irq) {
    pr_err("Failed to get the parent IRQ for LPC IRQs\n");
    return -EINVAL;
    }
    irq_handle = of_fwnode_handle(node);
    ret = pch_lpc_init(res.start, resource_size(&res), irq_handle,
    parent_irq);
    if (ret) {
    irq_dispose_mapping(parent_irq);
    return ret;
    }
    return 0;
    }
    IRQCHIP_DECLARE(pch_lpc, "loongson,ls7a-lpc", pch_lpc_of_init);
