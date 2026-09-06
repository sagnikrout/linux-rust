//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-atmel-aic5.c
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
// Atmel AT91 AIC5 (Advanced Interrupt Controller) driver
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
pub const NR_AIC5_IRQS: c_int = 128;
pub const AT91_AIC5_SSR: c_uint = 0x0;

pub const AT91_AIC5_SMR: c_uint = 0x4;
pub const AT91_AIC5_SVR: c_uint = 0x8;
pub const AT91_AIC5_IVR: c_uint = 0x10;
pub const AT91_AIC5_FVR: c_uint = 0x14;
pub const AT91_AIC5_ISR: c_uint = 0x18;
pub const AT91_AIC5_IPR0: c_uint = 0x20;
pub const AT91_AIC5_IPR1: c_uint = 0x24;
pub const AT91_AIC5_IPR2: c_uint = 0x28;
pub const AT91_AIC5_IPR3: c_uint = 0x2c;
pub const AT91_AIC5_IMR: c_uint = 0x30;
pub const AT91_AIC5_CISR: c_uint = 0x34;
pub const AT91_AIC5_IECR: c_uint = 0x40;
pub const AT91_AIC5_IDCR: c_uint = 0x44;
pub const AT91_AIC5_ICCR: c_uint = 0x48;
pub const AT91_AIC5_ISCR: c_uint = 0x4c;
pub const AT91_AIC5_EOICR: c_uint = 0x38;
pub const AT91_AIC5_SPU: c_uint = 0x3c;
pub const AT91_AIC5_DCR: c_uint = 0x6c;
pub const AT91_AIC5_FFER: c_uint = 0x50;
pub const AT91_AIC5_FFDR: c_uint = 0x54;
pub const AT91_AIC5_FFSR: c_uint = 0x58;
    static struct irq_domain *aic5_domain;
#[no_mangle]
unsafe extern "C" fn aic5_handle(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry aic5_handle(struct pt_regs *regs)
    {
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(aic5_domain, 0);
    u32 irqnr;
    u32 irqstat;
    irqnr = irq_reg_readl(bgc, AT91_AIC5_IVR);
    irqstat = irq_reg_readl(bgc, AT91_AIC5_ISR);
    if (!irqstat)
    irq_reg_writel(bgc, 0, AT91_AIC5_EOICR);
    else
    generic_handle_domain_irq(aic5_domain, irqnr);
    }
#[no_mangle]
unsafe extern "C" fn aic5_mask(d: *mut irq_data) {
    static void aic5_mask(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
//
// Disable interrupt on AIC5. We always take the lock of the
// first irq chip as all chips share the same registers.
//
    guard(raw_spinlock)(&bgc.lock);
    irq_reg_writel(gc, d.hwirq, AT91_AIC5_SSR);
    irq_reg_writel(gc, 1, AT91_AIC5_IDCR);
    gc.mask_cache &= ~d.mask;
    }
#[no_mangle]
unsafe extern "C" fn aic5_unmask(d: *mut irq_data) {
    static void aic5_unmask(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
//
// Enable interrupt on AIC5. We always take the lock of the
// first irq chip as all chips share the same registers.
//
    guard(raw_spinlock)(&bgc.lock);
    irq_reg_writel(gc, d.hwirq, AT91_AIC5_SSR);
    irq_reg_writel(gc, 1, AT91_AIC5_IECR);
    gc.mask_cache |= d.mask;
    }
#[no_mangle]
unsafe extern "C" fn aic5_retrigger(d: *mut irq_data) -> c_int {
    static int aic5_retrigger(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
// Enable interrupt on AIC5
    guard(raw_spinlock)(&bgc.lock);
    irq_reg_writel(bgc, d.hwirq, AT91_AIC5_SSR);
    irq_reg_writel(bgc, 1, AT91_AIC5_ISCR);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn aic5_set_type(d: *mut irq_data, type: unsigned) -> c_int {
    static int aic5_set_type(struct irq_data *d, unsigned type)
    {
    struct irq_domain *domain = d.domain;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    unsigned int smr;
    int ret;
    guard(raw_spinlock)(&bgc.lock);
    irq_reg_writel(bgc, d.hwirq, AT91_AIC5_SSR);
    smr = irq_reg_readl(bgc, AT91_AIC5_SMR);
    ret = aic_common_set_type(d, type, &smr);
    if (!ret)
    irq_reg_writel(bgc, smr, AT91_AIC5_SMR);
    return ret;
    }

    static u32 *smr_cache;
#[no_mangle]
unsafe extern "C" fn aic5_suspend(d: *mut irq_data) {
    static void aic5_suspend(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_domain_chip_generic *dgc = domain.gc;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    int i;
    u32 mask;
    if (smr_cache)
    for (i = 0; i < domain.revmap_size; i++) {
    irq_reg_writel(bgc, i, AT91_AIC5_SSR);
    smr_cache[i] = irq_reg_readl(bgc, AT91_AIC5_SMR);
    }
    guard(raw_spinlock)(&bgc.lock);
    for (i = 0; i < dgc.irqs_per_chip; i++) {
    mask = 1 << i;
    if ((mask & gc.mask_cache) == (mask & gc.wake_active))
    continue;
    irq_reg_writel(bgc, i + gc.irq_base, AT91_AIC5_SSR);
    if (mask & gc.wake_active)
    irq_reg_writel(bgc, 1, AT91_AIC5_IECR);
    else
    irq_reg_writel(bgc, 1, AT91_AIC5_IDCR);
    }
    }
#[no_mangle]
unsafe extern "C" fn aic5_resume(d: *mut irq_data) {
    static void aic5_resume(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_domain_chip_generic *dgc = domain.gc;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    int i;
    u32 mask;
    guard(raw_spinlock)(&bgc.lock);
    if (smr_cache) {
    irq_reg_writel(bgc, 0xffffffff, AT91_AIC5_SPU);
    for (i = 0; i < domain.revmap_size; i++) {
    irq_reg_writel(bgc, i, AT91_AIC5_SSR);
    irq_reg_writel(bgc, i, AT91_AIC5_SVR);
    irq_reg_writel(bgc, smr_cache[i], AT91_AIC5_SMR);
    }
    }
    for (i = 0; i < dgc.irqs_per_chip; i++) {
    mask = 1 << i;
    if (!smr_cache &&
    ((mask & gc.mask_cache) == (mask & gc.wake_active)))
    continue;
    irq_reg_writel(bgc, i + gc.irq_base, AT91_AIC5_SSR);
    if (mask & gc.mask_cache)
    irq_reg_writel(bgc, 1, AT91_AIC5_IECR);
    else
    irq_reg_writel(bgc, 1, AT91_AIC5_IDCR);
    }
    }
#[no_mangle]
unsafe extern "C" fn aic5_pm_shutdown(d: *mut irq_data) {
    static void aic5_pm_shutdown(struct irq_data *d)
    {
    struct irq_domain *domain = d.domain;
    struct irq_domain_chip_generic *dgc = domain.gc;
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(domain, 0);
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    int i;
    guard(raw_spinlock)(&bgc.lock);
    for (i = 0; i < dgc.irqs_per_chip; i++) {
    irq_reg_writel(bgc, i + gc.irq_base, AT91_AIC5_SSR);
    irq_reg_writel(bgc, 1, AT91_AIC5_IDCR);
    irq_reg_writel(bgc, 1, AT91_AIC5_ICCR);
    }
    }

#[no_mangle]
unsafe extern "C" fn aic5_hw_init(domain: *mut irq_domain) -> void __init {
    static void __init aic5_hw_init(struct irq_domain *domain)
    {
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(domain, 0);
    int i;
//
// Perform 8 End Of Interrupt Command to make sure AIC
// will not Lock out nIRQ
//
    for (i = 0; i < 8; i++)
    irq_reg_writel(gc, 0, AT91_AIC5_EOICR);
//
// Spurious Interrupt ID in Spurious Vector Register.
// When there is no current interrupt, the IRQ Vector Register
// reads the value stored in AIC_SPU
//
    irq_reg_writel(gc, 0xffffffff, AT91_AIC5_SPU);
// No debugging in AIC: Debug (Protect) Control Register
    irq_reg_writel(gc, 0, AT91_AIC5_DCR);
// Disable and clear all interrupts initially
    for (i = 0; i < domain.revmap_size; i++) {
    irq_reg_writel(gc, i, AT91_AIC5_SSR);
    irq_reg_writel(gc, i, AT91_AIC5_SVR);
    irq_reg_writel(gc, 1, AT91_AIC5_IDCR);
    irq_reg_writel(gc, 1, AT91_AIC5_ICCR);
    }
    }
    static int aic5_irq_domain_xlate(struct irq_domain *d,
    struct device_node *ctrlr,
    const u32 *intspec, unsigned int intsize,
    irq_hw_number_t *out_hwirq,
    unsigned int *out_type)
    {
    struct irq_chip_generic *bgc = irq_get_domain_generic_chip(d, 0);
    unsigned smr;
    int ret;
    if (!bgc)
    return -EINVAL;
    ret = aic_common_irq_domain_xlate(d, ctrlr, intspec, intsize,
    out_hwirq, out_type);
    if (ret)
    return ret;
    guard(raw_spinlock_irqsave)(&bgc.lock);
    irq_reg_writel(bgc, *out_hwirq, AT91_AIC5_SSR);
    smr = irq_reg_readl(bgc, AT91_AIC5_SMR);
    aic_common_set_priority(intspec[2], &smr);
    irq_reg_writel(bgc, smr, AT91_AIC5_SMR);
    return ret;
    }
    static const struct irq_domain_ops aic5_irq_ops = {
    .map	= irq_map_generic_chip,
    .xlate	= aic5_irq_domain_xlate,
    };
#[no_mangle]
unsafe extern "C" fn sama5d3_aic_irq_fixup() -> void __init {
    static void __init sama5d3_aic_irq_fixup(void)
    {
    aic_common_rtc_irq_fixup();
    }
#[no_mangle]
unsafe extern "C" fn sam9x60_aic_irq_fixup() -> void __init {
    static void __init sam9x60_aic_irq_fixup(void)
    {
    aic_common_rtc_irq_fixup();
    aic_common_rtt_irq_fixup();
    }
    static const struct of_device_id aic5_irq_fixups[] __initconst = {
    { .compatible = "atmel,sama5d3", .data = sama5d3_aic_irq_fixup },
    { .compatible = "atmel,sama5d4", .data = sama5d3_aic_irq_fixup },
    { .compatible = "microchip,sam9x60", .data = sam9x60_aic_irq_fixup },
    { .compatible = "microchip,sam9x7", .data = sam9x60_aic_irq_fixup },
    { /* sentinel */ },
    };
    static int __init aic5_of_init(struct device_node *node,
    struct device_node *parent,
    int nirqs)
    {
    struct irq_chip_generic *gc;
    struct irq_domain *domain;
    int nchips;
    int i;
    if (nirqs > NR_AIC5_IRQS)
    return -EINVAL;
    if (aic5_domain)
    return -EEXIST;
    domain = aic_common_of_init(node, &aic5_irq_ops, "atmel-aic5",
    nirqs, aic5_irq_fixups);
    if (IS_ERR(domain))
    return PTR_ERR(domain);
    aic5_domain = domain;
    nchips = aic5_domain.revmap_size / 32;
    for (i = 0; i < nchips; i++) {
    gc = irq_get_domain_generic_chip(domain, i * 32);
    gc.chip_types[0].regs.eoi = AT91_AIC5_EOICR;
    gc.chip_types[0].chip.irq_mask = aic5_mask;
    gc.chip_types[0].chip.irq_unmask = aic5_unmask;
    gc.chip_types[0].chip.irq_retrigger = aic5_retrigger;
    gc.chip_types[0].chip.irq_set_type = aic5_set_type;
    gc.chip_types[0].chip.irq_suspend = aic5_suspend;
    gc.chip_types[0].chip.irq_resume = aic5_resume;
    gc.chip_types[0].chip.irq_pm_shutdown = aic5_pm_shutdown;
    }
    aic5_hw_init(domain);
    set_handle_irq(aic5_handle);
    return 0;
    }
pub const NR_SAMA5D2_IRQS: c_int = 77;
    static int __init sama5d2_aic5_of_init(struct device_node *node,
    struct device_node *parent)
    {

    smr_cache = kcalloc(DIV_ROUND_UP(NR_SAMA5D2_IRQS, 32) * 32,
    sizeof(*smr_cache), GFP_KERNEL);
    if (!smr_cache)
    return -ENOMEM;

    return aic5_of_init(node, parent, NR_SAMA5D2_IRQS);
    }
    IRQCHIP_DECLARE(sama5d2_aic5, "atmel,sama5d2-aic", sama5d2_aic5_of_init);
pub const NR_SAMA5D3_IRQS: c_int = 48;
    static int __init sama5d3_aic5_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return aic5_of_init(node, parent, NR_SAMA5D3_IRQS);
    }
    IRQCHIP_DECLARE(sama5d3_aic5, "atmel,sama5d3-aic", sama5d3_aic5_of_init);
pub const NR_SAMA5D4_IRQS: c_int = 68;
    static int __init sama5d4_aic5_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return aic5_of_init(node, parent, NR_SAMA5D4_IRQS);
    }
    IRQCHIP_DECLARE(sama5d4_aic5, "atmel,sama5d4-aic", sama5d4_aic5_of_init);
pub const NR_SAM9X60_IRQS: c_int = 50;
    static int __init sam9x60_aic5_of_init(struct device_node *node,
    struct device_node *parent)
    {
    return aic5_of_init(node, parent, NR_SAM9X60_IRQS);
    }
    IRQCHIP_DECLARE(sam9x60_aic5, "microchip,sam9x60-aic", sam9x60_aic5_of_init);
pub const NR_SAM9X7_IRQS: c_int = 70;
#[no_mangle]
unsafe extern "C" fn sam9x7_aic5_of_init(node: *mut device_node, parent: *mut device_node) -> int __init {
    static int __init sam9x7_aic5_of_init(struct device_node *node, struct device_node *parent)
    {
    return aic5_of_init(node, parent, NR_SAM9X7_IRQS);
    }
    IRQCHIP_DECLARE(sam9x7_aic5, "microchip,sam9x7-aic", sam9x7_aic5_of_init);
