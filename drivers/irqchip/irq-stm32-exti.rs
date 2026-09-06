//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-stm32-exti.c
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
// Copyright (C) Maxime Coquelin 2015
// Copyright (C) STMicroelectronics 2017-2024
// Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
//

pub const IRQS_PER_BANK: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_exti_bank {
    pub imr_ofst: u32,
    pub emr_ofst: u32,
    pub rtsr_ofst: u32,
    pub ftsr_ofst: u32,
    pub swier_ofst: u32,
    pub rpr_ofst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_exti_drv_data {
    pub exti_banks: *const stm32_exti_bank,
    pub desc_irqs: *const u8,
    pub bank_nr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_exti_chip_data {
    pub host_data: *mut stm32_exti_host_data,
    pub reg_bank: *const stm32_exti_bank,
    pub wake_active: u32,
    pub mask_cache: u32,
    pub rtsr_cache: u32,
    pub ftsr_cache: u32,
    pub event_reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_exti_host_data {
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub chips_data: *mut stm32_exti_chip_data,
    pub drv_data: *const stm32_exti_drv_data,
}

    static const struct stm32_exti_bank stm32f4xx_exti_b1 = {
    .imr_ofst	= 0x00,
    .emr_ofst	= 0x04,
    .rtsr_ofst	= 0x08,
    .ftsr_ofst	= 0x0C,
    .swier_ofst	= 0x10,
    .rpr_ofst	= 0x14,
    };
    static const struct stm32_exti_bank *stm32f4xx_exti_banks[] = {
    &stm32f4xx_exti_b1,
    };
    static const struct stm32_exti_drv_data stm32f4xx_drv_data = {
    .exti_banks = stm32f4xx_exti_banks,
    .bank_nr = ARRAY_SIZE(stm32f4xx_exti_banks),
    };
    static const struct stm32_exti_bank stm32h7xx_exti_b1 = {
    .imr_ofst	= 0x80,
    .emr_ofst	= 0x84,
    .rtsr_ofst	= 0x00,
    .ftsr_ofst	= 0x04,
    .swier_ofst	= 0x08,
    .rpr_ofst	= 0x88,
    };
    static const struct stm32_exti_bank stm32h7xx_exti_b2 = {
    .imr_ofst	= 0x90,
    .emr_ofst	= 0x94,
    .rtsr_ofst	= 0x20,
    .ftsr_ofst	= 0x24,
    .swier_ofst	= 0x28,
    .rpr_ofst	= 0x98,
    };
    static const struct stm32_exti_bank stm32h7xx_exti_b3 = {
    .imr_ofst	= 0xA0,
    .emr_ofst	= 0xA4,
    .rtsr_ofst	= 0x40,
    .ftsr_ofst	= 0x44,
    .swier_ofst	= 0x48,
    .rpr_ofst	= 0xA8,
    };
    static const struct stm32_exti_bank *stm32h7xx_exti_banks[] = {
    &stm32h7xx_exti_b1,
    &stm32h7xx_exti_b2,
    &stm32h7xx_exti_b3,
    };
    static const struct stm32_exti_drv_data stm32h7xx_drv_data = {
    .exti_banks = stm32h7xx_exti_banks,
    .bank_nr = ARRAY_SIZE(stm32h7xx_exti_banks),
    };
#[no_mangle]
unsafe extern "C" fn stm32_exti_pending(gc: *mut irq_chip_generic) -> c_ulong {
    static unsigned long stm32_exti_pending(struct irq_chip_generic *gc)
    {
    struct stm32_exti_chip_data *chip_data = gc.private;
    const struct stm32_exti_bank *stm32_bank = chip_data.reg_bank;
    return irq_reg_readl(gc, stm32_bank.rpr_ofst);
    }
#[no_mangle]
unsafe extern "C" fn stm32_irq_handler(desc: *mut irq_desc) {
    static void stm32_irq_handler(struct irq_desc *desc)
    {
    struct irq_domain *domain = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    let mut nbanks: c_uint = domain.gc.num_chips;
    struct irq_chip_generic *gc;
    unsigned long pending;
    int n, i, irq_base = 0;
    chained_irq_enter(chip, desc);
    for (i = 0; i < nbanks; i++, irq_base += IRQS_PER_BANK) {
    gc = irq_get_domain_generic_chip(domain, irq_base);
    while ((pending = stm32_exti_pending(gc))) {
    for_each_set_bit(n, &pending, IRQS_PER_BANK)
    generic_handle_domain_irq(domain, irq_base + n);
    }
    }
    chained_irq_exit(chip, desc);
    }
    static int stm32_exti_set_type(struct irq_data *d,
    unsigned int type, u32 *rtsr, u32 *ftsr)
    {
    let mut mask: u32 = BIT(d.hwirq % IRQS_PER_BANK);
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
// rtsr |= mask;
// ftsr &= ~mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
// rtsr &= ~mask;
// ftsr |= mask;
    break;
    case IRQ_TYPE_EDGE_BOTH:
// rtsr |= mask;
// ftsr |= mask;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int stm32_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct stm32_exti_chip_data *chip_data = gc.private;
    const struct stm32_exti_bank *stm32_bank = chip_data.reg_bank;
    u32 rtsr, ftsr;
    int err;
    guard(raw_spinlock)(&gc.lock);
    rtsr = irq_reg_readl(gc, stm32_bank.rtsr_ofst);
    ftsr = irq_reg_readl(gc, stm32_bank.ftsr_ofst);
    err = stm32_exti_set_type(d, type, &rtsr, &ftsr);
    if (err)
    return err;
    irq_reg_writel(gc, rtsr, stm32_bank.rtsr_ofst);
    irq_reg_writel(gc, ftsr, stm32_bank.ftsr_ofst);
    return 0;
    }
    static void stm32_chip_suspend(struct stm32_exti_chip_data *chip_data,
    u32 wake_active)
    {
    const struct stm32_exti_bank *stm32_bank = chip_data.reg_bank;
    void __iomem *base = chip_data.host_data.base;
// save rtsr, ftsr registers
    chip_data.rtsr_cache = readl_relaxed(base + stm32_bank.rtsr_ofst);
    chip_data.ftsr_cache = readl_relaxed(base + stm32_bank.ftsr_ofst);
    writel_relaxed(wake_active, base + stm32_bank.imr_ofst);
    }
    static void stm32_chip_resume(struct stm32_exti_chip_data *chip_data,
    u32 mask_cache)
    {
    const struct stm32_exti_bank *stm32_bank = chip_data.reg_bank;
    void __iomem *base = chip_data.host_data.base;
// restore rtsr, ftsr, registers
    writel_relaxed(chip_data.rtsr_cache, base + stm32_bank.rtsr_ofst);
    writel_relaxed(chip_data.ftsr_cache, base + stm32_bank.ftsr_ofst);
    writel_relaxed(mask_cache, base + stm32_bank.imr_ofst);
    }
#[no_mangle]
unsafe extern "C" fn stm32_irq_suspend(gc: *mut irq_chip_generic) {
    static void stm32_irq_suspend(struct irq_chip_generic *gc)
    {
    struct stm32_exti_chip_data *chip_data = gc.private;
    guard(raw_spinlock)(&gc.lock);
    stm32_chip_suspend(chip_data, gc.wake_active);
    }
#[no_mangle]
unsafe extern "C" fn stm32_irq_resume(gc: *mut irq_chip_generic) {
    static void stm32_irq_resume(struct irq_chip_generic *gc)
    {
    struct stm32_exti_chip_data *chip_data = gc.private;
    guard(raw_spinlock)(&gc.lock);
    stm32_chip_resume(chip_data, gc.mask_cache);
    }
    static int stm32_exti_alloc(struct irq_domain *d, unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct irq_fwspec *fwspec = data;
    irq_hw_number_t hwirq;
    hwirq = fwspec.param[0];
    irq_map_generic_chip(d, virq, hwirq);
    return 0;
    }
    static void stm32_exti_free(struct irq_domain *d, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *data = irq_domain_get_irq_data(d, virq);
    irq_domain_reset_irq_data(data);
    }
    static const struct irq_domain_ops irq_exti_domain_ops = {
    .map	= irq_map_generic_chip,
    .alloc  = stm32_exti_alloc,
    .free	= stm32_exti_free,
    .xlate	= irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn stm32_irq_ack(d: *mut irq_data) {
    static void stm32_irq_ack(struct irq_data *d)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct stm32_exti_chip_data *chip_data = gc.private;
    const struct stm32_exti_bank *stm32_bank = chip_data.reg_bank;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, d.mask, stm32_bank.rpr_ofst);
    }
    static struct
    stm32_exti_host_data *stm32_exti_host_init(const struct stm32_exti_drv_data *dd,
    struct device_node *node)
    {
    struct stm32_exti_host_data *host_data;
    host_data = kzalloc_obj(*host_data);
    if (!host_data)
    return core::ptr::null_mut();
    host_data.drv_data = dd;
    host_data.chips_data = kzalloc_objs(struct stm32_exti_chip_data,
    dd.bank_nr);
    if (!host_data.chips_data)
    goto free_host_data;
    host_data.base = of_iomap(node, 0);
    if (!host_data.base) {
    pr_err("%pOF: Unable to map registers\n", node);
    goto free_chips_data;
    }
    return host_data;
    free_chips_data:
    kfree(host_data.chips_data);
    free_host_data:
    kfree(host_data);
    return core::ptr::null_mut();
    }
    static struct
    stm32_exti_chip_data *stm32_exti_chip_init(struct stm32_exti_host_data *h_data,
    u32 bank_idx,
    struct device_node *node)
    {
    const struct stm32_exti_bank *stm32_bank;
    struct stm32_exti_chip_data *chip_data;
    void __iomem *base = h_data.base;
    stm32_bank = h_data.drv_data.exti_banks[bank_idx];
    chip_data = &h_data.chips_data[bank_idx];
    chip_data.host_data = h_data;
    chip_data.reg_bank = stm32_bank;
//
// This IP has no reset, so after hot reboot we should
// clear registers to avoid residue
//
    writel_relaxed(0, base + stm32_bank.imr_ofst);
    writel_relaxed(0, base + stm32_bank.emr_ofst);
    pr_info("%pOF: bank%d\n", node, bank_idx);
    return chip_data;
    }
    static int __init stm32_exti_init(const struct stm32_exti_drv_data *drv_data,
    struct device_node *node)
    {
    struct stm32_exti_host_data *host_data;
    let mut clr: c_uint = IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN;
    int nr_irqs, ret, i;
    struct irq_chip_generic *gc;
    struct irq_domain *domain;
    host_data = stm32_exti_host_init(drv_data, node);
    if (!host_data)
    return -ENOMEM;
    domain = irq_domain_create_linear(of_fwnode_handle(node), drv_data.bank_nr * IRQS_PER_BANK,
    &irq_exti_domain_ops, core::ptr::null_mut());
    if (!domain) {
    pr_err("%pOFn: Could not register interrupt domain.\n",
    node);
    ret = -ENOMEM;
    goto out_unmap;
    }
    ret = irq_alloc_domain_generic_chips(domain, IRQS_PER_BANK, 1, "exti",
    handle_edge_irq, clr, 0, 0);
    if (ret) {
    pr_err("%pOF: Could not allocate generic interrupt chip.\n",
    node);
    goto out_free_domain;
    }
    for (i = 0; i < drv_data.bank_nr; i++) {
    const struct stm32_exti_bank *stm32_bank;
    struct stm32_exti_chip_data *chip_data;
    stm32_bank = drv_data.exti_banks[i];
    chip_data = stm32_exti_chip_init(host_data, i, node);
    gc = irq_get_domain_generic_chip(domain, i * IRQS_PER_BANK);
    gc.reg_base = host_data.base;
    gc.chip_types.type = IRQ_TYPE_EDGE_BOTH;
    gc.chip_types.chip.irq_ack = stm32_irq_ack;
    gc.chip_types.chip.irq_mask = irq_gc_mask_clr_bit;
    gc.chip_types.chip.irq_unmask = irq_gc_mask_set_bit;
    gc.chip_types.chip.irq_set_type = stm32_irq_set_type;
    gc.chip_types.chip.irq_set_wake = irq_gc_set_wake;
    gc.suspend = stm32_irq_suspend;
    gc.resume = stm32_irq_resume;
    gc.wake_enabled = IRQ_MSK(IRQS_PER_BANK);
    gc.chip_types.regs.mask = stm32_bank.imr_ofst;
    gc.private = (void *)chip_data;
    }
    nr_irqs = of_irq_count(node);
    for (i = 0; i < nr_irqs; i++) {
    let mut irq: c_uint = irq_of_parse_and_map(node, i);
    irq_set_handler_data(irq, domain);
    irq_set_chained_handler(irq, stm32_irq_handler);
    }
    return 0;
    out_free_domain:
    irq_domain_remove(domain);
    out_unmap:
    iounmap(host_data.base);
    kfree(host_data.chips_data);
    kfree(host_data);
    return ret;
    }
    static int __init stm32f4_exti_of_init(struct device_node *np,
    struct device_node *parent)
    {
    return stm32_exti_init(&stm32f4xx_drv_data, np);
    }
    IRQCHIP_DECLARE(stm32f4_exti, "st,stm32-exti", stm32f4_exti_of_init);
    static int __init stm32h7_exti_of_init(struct device_node *np,
    struct device_node *parent)
    {
    return stm32_exti_init(&stm32h7xx_drv_data, np);
    }
    IRQCHIP_DECLARE(stm32h7_exti, "st,stm32h7-exti", stm32h7_exti_of_init);
