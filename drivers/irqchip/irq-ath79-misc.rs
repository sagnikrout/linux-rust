//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-ath79-misc.c
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
// Atheros AR71xx/AR724x/AR913x MISC interrupt controller
//
// Copyright (C) 2015 Alban Bedel <albeu@free.fr>
// Copyright (C) 2010-2011 Jaiganesh Narayanan <jnarayanan@atheros.com>
// Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
// Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
//
// Parts of this file are based on Atheros' 2.6.15/2.6.31 BSP
//

pub const AR71XX_RESET_REG_MISC_INT_STATUS: c_int = 0;
pub const AR71XX_RESET_REG_MISC_INT_ENABLE: c_int = 4;
pub const ATH79_MISC_IRQ_COUNT: c_int = 32;
pub const ATH79_MISC_PERF_IRQ: c_int = 5;
    static int ath79_perfcount_irq;
#[no_mangle]
pub unsafe extern "C" fn get_c0_perfcount_int() -> c_int {
    int get_c0_perfcount_int(void)
    {
    return ath79_perfcount_irq;
    }
    EXPORT_SYMBOL_GPL(get_c0_perfcount_int);
#[no_mangle]
unsafe extern "C" fn ath79_misc_irq_handler(desc: *mut irq_desc) {
    static void ath79_misc_irq_handler(struct irq_desc *desc)
    {
    struct irq_domain *domain = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    void __iomem *base = domain.host_data;
    u32 pending;
    chained_irq_enter(chip, desc);
    pending = __raw_readl(base + AR71XX_RESET_REG_MISC_INT_STATUS) &
    __raw_readl(base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    if (!pending) {
    spurious_interrupt();
    chained_irq_exit(chip, desc);
    return;
    }
    while (pending) {
    let mut bit: c_int = __ffs(pending);
    generic_handle_domain_irq(domain, bit);
    pending &= ~BIT(bit);
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn ar71xx_misc_irq_unmask(d: *mut irq_data) {
    static void ar71xx_misc_irq_unmask(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    u32 t;
    t = __raw_readl(base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    __raw_writel(t | BIT(irq), base + AR71XX_RESET_REG_MISC_INT_ENABLE);
// flush write
    __raw_readl(base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn ar71xx_misc_irq_mask(d: *mut irq_data) {
    static void ar71xx_misc_irq_mask(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    u32 t;
    t = __raw_readl(base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    __raw_writel(t & ~BIT(irq), base + AR71XX_RESET_REG_MISC_INT_ENABLE);
// flush write
    __raw_readl(base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn ar724x_misc_irq_ack(d: *mut irq_data) {
    static void ar724x_misc_irq_ack(struct irq_data *d)
    {
    void __iomem *base = irq_data_get_irq_chip_data(d);
    let mut irq: c_uint = d.hwirq;
    u32 t;
    t = __raw_readl(base + AR71XX_RESET_REG_MISC_INT_STATUS);
    __raw_writel(t & ~BIT(irq), base + AR71XX_RESET_REG_MISC_INT_STATUS);
// flush write
    __raw_readl(base + AR71XX_RESET_REG_MISC_INT_STATUS);
    }
    static struct irq_chip ath79_misc_irq_chip = {
    .name		= "MISC",
    .irq_unmask	= ar71xx_misc_irq_unmask,
    .irq_mask	= ar71xx_misc_irq_mask,
    };
#[no_mangle]
unsafe extern "C" fn misc_map(d: *mut irq_domain, irq: c_uint, hw: irq_hw_number_t) -> c_int {
    static int misc_map(struct irq_domain *d, unsigned int irq, irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(irq, &ath79_misc_irq_chip, handle_level_irq);
    irq_set_chip_data(irq, d.host_data);
    return 0;
    }
    static const struct irq_domain_ops misc_irq_domain_ops = {
    .xlate = irq_domain_xlate_onecell,
    .map = misc_map,
    };
    static void __init ath79_misc_intc_domain_init(
    struct irq_domain *domain, int irq)
    {
    void __iomem *base = domain.host_data;
    ath79_perfcount_irq = irq_create_mapping(domain, ATH79_MISC_PERF_IRQ);
// Disable and clear all interrupts
    __raw_writel(0, base + AR71XX_RESET_REG_MISC_INT_ENABLE);
    __raw_writel(0, base + AR71XX_RESET_REG_MISC_INT_STATUS);
    irq_set_chained_handler_and_data(irq, ath79_misc_irq_handler, domain);
    }
    static int __init ath79_misc_intc_of_init(
    struct device_node *node, struct device_node *parent)
    {
    struct irq_domain *domain;
    void __iomem *base;
    int irq;
    irq = irq_of_parse_and_map(node, 0);
    if (!irq) {
    pr_err("Failed to get MISC IRQ\n");
    return -EINVAL;
    }
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("Failed to get MISC IRQ registers\n");
    return -ENOMEM;
    }
    domain = irq_domain_create_linear(of_fwnode_handle(node), ATH79_MISC_IRQ_COUNT,
    &misc_irq_domain_ops, base);
    if (!domain) {
    pr_err("Failed to add MISC irqdomain\n");
    return -EINVAL;
    }
    ath79_misc_intc_domain_init(domain, irq);
    return 0;
    }
    static int __init ar7100_misc_intc_of_init(
    struct device_node *node, struct device_node *parent)
    {
    ath79_misc_irq_chip.irq_mask_ack = ar71xx_misc_irq_mask;
    return ath79_misc_intc_of_init(node, parent);
    }
    IRQCHIP_DECLARE(ar7100_misc_intc, "qca,ar7100-misc-intc",
    ar7100_misc_intc_of_init);
    static int __init ar7240_misc_intc_of_init(
    struct device_node *node, struct device_node *parent)
    {
    ath79_misc_irq_chip.irq_ack = ar724x_misc_irq_ack;
    return ath79_misc_intc_of_init(node, parent);
    }
    IRQCHIP_DECLARE(ar7240_misc_intc, "qca,ar7240-misc-intc",
    ar7240_misc_intc_of_init);
