//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-atmel-aic.c
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


//
// Atmel AT91 AIC (Advanced Interrupt Controller) driver
//
// Copyright (C) 2004 SAN People
// Copyright (C) 2004 ATMEL
// Copyright (C) Rick Bronson
// Copyright (C) 2014 Free Electrons
//
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

// Number of irq lines managed by AIC
pub const NR_AIC_IRQS: c_int = 32;

pub const AT91_AIC_IVR: c_uint = 0x100;
pub const AT91_AIC_FVR: c_uint = 0x104;
pub const AT91_AIC_ISR: c_uint = 0x108;
pub const AT91_AIC_IPR: c_uint = 0x10c;
pub const AT91_AIC_IMR: c_uint = 0x110;
pub const AT91_AIC_CISR: c_uint = 0x114;
pub const AT91_AIC_IECR: c_uint = 0x120;
pub const AT91_AIC_IDCR: c_uint = 0x124;
pub const AT91_AIC_ICCR: c_uint = 0x128;
pub const AT91_AIC_ISCR: c_uint = 0x12c;
pub const AT91_AIC_EOICR: c_uint = 0x130;
pub const AT91_AIC_SPU: c_uint = 0x134;
pub const AT91_AIC_DCR: c_uint = 0x138;
    static struct irq_domain *aic_domain;
#[no_mangle]
unsafe extern "C" fn aic_handle(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry aic_handle(struct pt_regs *regs)
    {
    struct irq_domain_chip_generic *dgc = aic_domain.gc;
    struct irq_chip_generic *gc = dgc.gc[0];
    u32 irqnr;
    u32 irqstat;
    irqnr = irq_reg_readl(gc, AT91_AIC_IVR);
    irqstat = irq_reg_readl(gc, AT91_AIC_ISR);
    if (!irqstat)
    irq_reg_writel(gc, 0, AT91_AIC_EOICR);
    else
    generic_handle_domain_irq(aic_domain, irqnr);
    }
#[no_mangle]
unsafe extern "C" fn aic_retrigger(d: *mut irq_data) -> c_int {
    static int aic_retrigger(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
// Enable interrupt on AIC5
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, d.mask, AT91_AIC_ISCR);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn aic_set_type(d: *mut irq_data, type: unsigned) -> c_int {
    static int aic_set_type(struct irq_data *d, unsigned type)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    unsigned int smr;
    int ret;
    smr = irq_reg_readl(gc, AT91_AIC_SMR(d.hwirq));
    ret = aic_common_set_type(d, type, &smr);
    if (ret)
    return ret;
    irq_reg_writel(gc, smr, AT91_AIC_SMR(d.hwirq));
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn aic_suspend(d: *mut irq_data) {
    static void aic_suspend(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, gc.mask_cache, AT91_AIC_IDCR);
    irq_reg_writel(gc, gc.wake_active, AT91_AIC_IECR);
    }
#[no_mangle]
unsafe extern "C" fn aic_resume(d: *mut irq_data) {
    static void aic_resume(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, gc.wake_active, AT91_AIC_IDCR);
    irq_reg_writel(gc, gc.mask_cache, AT91_AIC_IECR);
    }
#[no_mangle]
unsafe extern "C" fn aic_pm_shutdown(d: *mut irq_data) {
    static void aic_pm_shutdown(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, 0xffffffff, AT91_AIC_IDCR);
    irq_reg_writel(gc, 0xffffffff, AT91_AIC_ICCR);
    }

#[no_mangle]
unsafe extern "C" fn aic_hw_init(domain: *mut irq_domain) -> void __init {
    static void __init aic_hw_init(struct irq_domain *domain)
    {
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(domain, 0);
    int i;
//
// Perform 8 End Of Interrupt Command to make sure AIC
// will not Lock out nIRQ
//
    for (i = 0; i < 8; i++)
    irq_reg_writel(gc, 0, AT91_AIC_EOICR);
//
// Spurious Interrupt ID in Spurious Vector Register.
// When there is no current interrupt, the IRQ Vector Register
// reads the value stored in AIC_SPU
//
    irq_reg_writel(gc, 0xffffffff, AT91_AIC_SPU);
// No debugging in AIC: Debug (Protect) Control Register
    irq_reg_writel(gc, 0, AT91_AIC_DCR);
// Disable and clear all interrupts initially
    irq_reg_writel(gc, 0xffffffff, AT91_AIC_IDCR);
    irq_reg_writel(gc, 0xffffffff, AT91_AIC_ICCR);
    for (i = 0; i < 32; i++)
    irq_reg_writel(gc, i, AT91_AIC_SVR(i));
    }
    static int aic_irq_domain_xlate(struct irq_domain *d,
    struct device_node *ctrlr,
    const u32 *intspec, unsigned int intsize,
    irq_hw_number_t *out_hwirq,
    unsigned int *out_type)
    {
    struct irq_domain_chip_generic *dgc = d.gc;
    struct irq_chip_generic *gc;
    unsigned smr;
    int idx, ret;
    if (!dgc)
    return -EINVAL;
    ret = aic_common_irq_domain_xlate(d, ctrlr, intspec, intsize,
    out_hwirq, out_type);
    if (ret)
    return ret;
    idx = intspec[0] / dgc.irqs_per_chip;
    if (idx >= dgc.num_chips)
    return -EINVAL;
    gc = dgc.gc[idx];
    guard(raw_spinlock_irqsave)(&gc.lock);
    smr = irq_reg_readl(gc, AT91_AIC_SMR(*out_hwirq));
    aic_common_set_priority(intspec[2], &smr);
    irq_reg_writel(gc, smr, AT91_AIC_SMR(*out_hwirq));
    return ret;
    }
    static const struct irq_domain_ops aic_irq_ops = {
    .map	= irq_map_generic_chip,
    .xlate	= aic_irq_domain_xlate,
    };
#[no_mangle]
unsafe extern "C" fn at91rm9200_aic_irq_fixup() -> void __init {
    static void __init at91rm9200_aic_irq_fixup(void)
    {
    aic_common_rtc_irq_fixup();
    }
#[no_mangle]
unsafe extern "C" fn at91sam9260_aic_irq_fixup() -> void __init {
    static void __init at91sam9260_aic_irq_fixup(void)
    {
    aic_common_rtt_irq_fixup();
    }
#[no_mangle]
unsafe extern "C" fn at91sam9g45_aic_irq_fixup() -> void __init {
    static void __init at91sam9g45_aic_irq_fixup(void)
    {
    aic_common_rtc_irq_fixup();
    aic_common_rtt_irq_fixup();
    }
    static const struct of_device_id aic_irq_fixups[] __initconst = {
    { .compatible = "atmel,at91rm9200", .data = at91rm9200_aic_irq_fixup },
    { .compatible = "atmel,at91sam9g45", .data = at91sam9g45_aic_irq_fixup },
    { .compatible = "atmel,at91sam9n12", .data = at91rm9200_aic_irq_fixup },
    { .compatible = "atmel,at91sam9rl", .data = at91sam9g45_aic_irq_fixup },
    { .compatible = "atmel,at91sam9x5", .data = at91rm9200_aic_irq_fixup },
    { .compatible = "atmel,at91sam9260", .data = at91sam9260_aic_irq_fixup },
    { .compatible = "atmel,at91sam9261", .data = at91sam9260_aic_irq_fixup },
    { .compatible = "atmel,at91sam9263", .data = at91sam9260_aic_irq_fixup },
    { .compatible = "atmel,at91sam9g20", .data = at91sam9260_aic_irq_fixup },
    { /* sentinel */ },
    };
    static int __init aic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    struct irq_chip_generic *gc;
    struct irq_domain *domain;
    if (aic_domain)
    return -EEXIST;
    domain = aic_common_of_init(node, &aic_irq_ops, "atmel-aic",
    NR_AIC_IRQS, aic_irq_fixups);
    if (IS_ERR(domain))
    return PTR_ERR(domain);
    aic_domain = domain;
    gc = irq_get_domain_generic_chip(domain, 0);
    gc.chip_types[0].regs.eoi = AT91_AIC_EOICR;
    gc.chip_types[0].regs.enable = AT91_AIC_IECR;
    gc.chip_types[0].regs.disable = AT91_AIC_IDCR;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_disable_reg;
    gc.chip_types[0].chip.irq_unmask = irq_gc_unmask_enable_reg;
    gc.chip_types[0].chip.irq_retrigger = aic_retrigger;
    gc.chip_types[0].chip.irq_set_type = aic_set_type;
    gc.chip_types[0].chip.irq_suspend = aic_suspend;
    gc.chip_types[0].chip.irq_resume = aic_resume;
    gc.chip_types[0].chip.irq_pm_shutdown = aic_pm_shutdown;
    aic_hw_init(domain);
    set_handle_irq(aic_handle);
    return 0;
    }
    IRQCHIP_DECLARE(at91rm9200_aic, "atmel,at91rm9200-aic", aic_of_init);
