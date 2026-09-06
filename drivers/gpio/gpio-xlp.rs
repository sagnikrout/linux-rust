//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-xlp.c
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
// Copyright (C) 2003-2015 Broadcom Corporation
// All Rights Reserved
//

//
// XLP GPIO has multiple 32 bit registers for each feature where each register
// controls 32 pins. So, pins up to 64 require 2 32-bit registers and up to 96
// require 3 32-bit registers for each feature.
// Here we only define offset of the first register for each feature. Offset of
// the registers for pins greater than 32 can be calculated as following(Use
// GPIO_INT_STAT as example):
//
// offset = (gpio / XLP_GPIO_REGSZ) * 4;
// reg_addr = addr + offset;
//
// where addr is base address of the that feature register and gpio is the pin.
//

pub const GPIO_9XX_OUTPUT_EN: c_uint = 0x14;
pub const GPIO_9XX_PADDRV: c_uint = 0x24;
//
// Only for 4 interrupt enable reg are defined for now,
// total reg available are 12.
//
pub const GPIO_9XX_INT_EN00: c_uint = 0x44;
pub const GPIO_9XX_INT_EN10: c_uint = 0x54;
pub const GPIO_9XX_INT_EN20: c_uint = 0x64;
pub const GPIO_9XX_INT_EN30: c_uint = 0x74;
pub const GPIO_9XX_INT_POL: c_uint = 0x104;
pub const GPIO_9XX_INT_TYPE: c_uint = 0x114;
pub const GPIO_9XX_INT_STAT: c_uint = 0x124;
// Interrupt type register mask
pub const XLP_GPIO_IRQ_TYPE_LVL: c_uint = 0x0;
pub const XLP_GPIO_IRQ_TYPE_EDGE: c_uint = 0x1;
// Interrupt polarity register mask
pub const XLP_GPIO_IRQ_POL_HIGH: c_uint = 0x0;
pub const XLP_GPIO_IRQ_POL_LOW: c_uint = 0x1;
pub const XLP_GPIO_REGSZ: c_int = 32;
pub const XLP_GPIO_IRQ_BASE: c_int = 768;
pub const XLP_MAX_NR_GPIO: c_int = 96;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlp_gpio_priv {
    pub chip: gpio_chip,
    pub XLP_MAX_NR_GPIO): DECLARE_BITMAP(gpio_enabled_mask,,
    pub /: *mut *mut *mut void __iomem gpio_intr_en; / pointer to first intr enable reg,
    pub /: *mut *mut *mut void __iomem gpio_intr_stat; / pointer to first intr status reg,
    pub /: *mut *mut *mut void __iomem gpio_intr_type; / pointer to first intr type reg,
    pub /: *mut *mut *mut void __iomem gpio_intr_pol; / pointer to first intr polarity reg,
    pub /: *mut *mut *mut void __iomem gpio_out_en; / pointer to first output enable reg,
    pub /: *mut *mut *mut void __iomem gpio_paddrv; / pointer to first pad drive reg,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn xlp_gpio_get_reg(addr: *mut void __iomem, gpio: unsigned) -> c_int {
    static int xlp_gpio_get_reg(void __iomem *addr, unsigned gpio)
    {
    u32 pos, regset;
    pos = gpio % XLP_GPIO_REGSZ;
    regset = (gpio / XLP_GPIO_REGSZ) * 4;
    return !!(readl(addr + regset) & BIT(pos));
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_set_reg(addr: *mut void __iomem, gpio: unsigned, state: c_int) {
    static void xlp_gpio_set_reg(void __iomem *addr, unsigned gpio, int state)
    {
    u32 value, pos, regset;
    pos = gpio % XLP_GPIO_REGSZ;
    regset = (gpio / XLP_GPIO_REGSZ) * 4;
    value = readl(addr + regset);
    if (state)
    value |= BIT(pos);
    else
    value &= ~BIT(pos);
    writel(value, addr + regset);
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_irq_enable(d: *mut irq_data) {
    static void xlp_gpio_irq_enable(struct irq_data *d)
    {
    struct gpio_chip *gc  = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_irq_disable(d: *mut irq_data) {
    static void xlp_gpio_irq_disable(struct irq_data *d)
    {
    struct gpio_chip *gc  = irq_data_get_irq_chip_data(d);
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    xlp_gpio_set_reg(priv.gpio_intr_en, d.hwirq, 0x0);
    __clear_bit(d.hwirq, priv.gpio_enabled_mask);
    spin_unlock_irqrestore(&priv.lock, flags);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_irq_mask_ack(d: *mut irq_data) {
    static void xlp_gpio_irq_mask_ack(struct irq_data *d)
    {
    struct gpio_chip *gc  = irq_data_get_irq_chip_data(d);
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    xlp_gpio_set_reg(priv.gpio_intr_en, d.hwirq, 0x0);
    xlp_gpio_set_reg(priv.gpio_intr_stat, d.hwirq, 0x1);
    __clear_bit(d.hwirq, priv.gpio_enabled_mask);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_irq_unmask(d: *mut irq_data) {
    static void xlp_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc  = irq_data_get_irq_chip_data(d);
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    xlp_gpio_set_reg(priv.gpio_intr_en, d.hwirq, 0x1);
    __set_bit(d.hwirq, priv.gpio_enabled_mask);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_set_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int xlp_gpio_set_irq_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc  = irq_data_get_irq_chip_data(d);
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    int pol, irq_type;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    irq_type = XLP_GPIO_IRQ_TYPE_EDGE;
    pol = XLP_GPIO_IRQ_POL_HIGH;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    irq_type = XLP_GPIO_IRQ_TYPE_EDGE;
    pol = XLP_GPIO_IRQ_POL_LOW;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    irq_type = XLP_GPIO_IRQ_TYPE_LVL;
    pol = XLP_GPIO_IRQ_POL_HIGH;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    irq_type = XLP_GPIO_IRQ_TYPE_LVL;
    pol = XLP_GPIO_IRQ_POL_LOW;
    break;
    default:
    return -EINVAL;
    }
    xlp_gpio_set_reg(priv.gpio_intr_type, d.hwirq, irq_type);
    xlp_gpio_set_reg(priv.gpio_intr_pol, d.hwirq, pol);
    return 0;
    }
    static struct irq_chip xlp_gpio_irq_chip = {
    .name		= "XLP-GPIO",
    .irq_mask_ack	= xlp_gpio_irq_mask_ack,
    .irq_enable	= xlp_gpio_irq_enable,
    .irq_disable	= xlp_gpio_irq_disable,
    .irq_set_type	= xlp_gpio_set_irq_type,
    .irq_unmask	= xlp_gpio_irq_unmask,
    .flags		= IRQCHIP_ONESHOT_SAFE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn xlp_gpio_generic_handler(desc: *mut irq_desc) {
    static void xlp_gpio_generic_handler(struct irq_desc *desc)
    {
    struct xlp_gpio_priv *priv = irq_desc_get_handler_data(desc);
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    int gpio, regoff;
    u32 gpio_stat;
    regoff = -1;
    gpio_stat = 0;
    chained_irq_enter(irqchip, desc);
    for_each_set_bit(gpio, priv.gpio_enabled_mask, XLP_MAX_NR_GPIO) {
    if (regoff != gpio / XLP_GPIO_REGSZ) {
    regoff = gpio / XLP_GPIO_REGSZ;
    gpio_stat = readl(priv.gpio_intr_stat + regoff * 4);
    }
    if (gpio_stat & BIT(gpio % XLP_GPIO_REGSZ))
    generic_handle_domain_irq(priv.chip.irq.domain, gpio);
    }
    chained_irq_exit(irqchip, desc);
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_dir_output(gc: *mut gpio_chip, gpio: unsigned, state: c_int) -> c_int {
    static int xlp_gpio_dir_output(struct gpio_chip *gc, unsigned gpio, int state)
    {
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    xlp_gpio_set_reg(priv.gpio_out_en, gpio, 0x1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_dir_input(gc: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int xlp_gpio_dir_input(struct gpio_chip *gc, unsigned gpio)
    {
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    xlp_gpio_set_reg(priv.gpio_out_en, gpio, 0x0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_get(gc: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int xlp_gpio_get(struct gpio_chip *gc, unsigned gpio)
    {
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    return xlp_gpio_get_reg(priv.gpio_paddrv, gpio);
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_set(gc: *mut gpio_chip, gpio: c_uint, state: c_int) -> c_int {
    static int xlp_gpio_set(struct gpio_chip *gc, unsigned int gpio, int state)
    {
    struct xlp_gpio_priv *priv = gpiochip_get_data(gc);
    xlp_gpio_set_reg(priv.gpio_paddrv, gpio, state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xlp_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int xlp_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_chip *gc;
    struct gpio_irq_chip *girq;
    struct xlp_gpio_priv *priv;
    void __iomem *gpio_base;
    int irq, err;
    priv = devm_kzalloc(&pdev.dev,	sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    gpio_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gpio_base))
    return PTR_ERR(gpio_base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    priv.gpio_out_en = gpio_base + GPIO_9XX_OUTPUT_EN;
    priv.gpio_paddrv = gpio_base + GPIO_9XX_PADDRV;
    priv.gpio_intr_stat = gpio_base + GPIO_9XX_INT_STAT;
    priv.gpio_intr_type = gpio_base + GPIO_9XX_INT_TYPE;
    priv.gpio_intr_pol = gpio_base + GPIO_9XX_INT_POL;
    priv.gpio_intr_en = gpio_base + GPIO_9XX_INT_EN00;
    bitmap_zero(priv.gpio_enabled_mask, XLP_MAX_NR_GPIO);
    gc = &priv.chip;
    gc.owner = THIS_MODULE;
    gc.label = dev_name(&pdev.dev);
    gc.base = 0;
    gc.parent = &pdev.dev;
    gc.ngpio = 70;
    gc.direction_output = xlp_gpio_dir_output;
    gc.direction_input = xlp_gpio_dir_input;
    gc.set = xlp_gpio_set;
    gc.get = xlp_gpio_get;
    spin_lock_init(&priv.lock);
    girq = &gc.irq;
    gpio_irq_chip_set_chip(girq, &xlp_gpio_irq_chip);
    girq.parent_handler = xlp_gpio_generic_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = irq;
    girq.first = 0;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_level_irq;
    err = gpiochip_add_data(gc, priv);
    if (err < 0)
    return err;
    dev_info(&pdev.dev, "registered %d GPIOs\n", gc.ngpio);
    return 0;
    }

    static const struct acpi_device_id xlp_gpio_acpi_match[] = {
    { "BRCM9006" },
    { "CAV9006" },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, xlp_gpio_acpi_match);

    static struct platform_driver xlp_gpio_driver = {
    .driver		= {
    .name	= "xlp-gpio",
    .acpi_match_table = ACPI_PTR(xlp_gpio_acpi_match),
    },
    .probe		= xlp_gpio_probe,
    };
    module_platform_driver(xlp_gpio_driver);
    MODULE_AUTHOR("Kamlakant Patel <kamlakant.patel@broadcom.com>");
    MODULE_AUTHOR("Ganesan Ramalingam <ganesanr@broadcom.com>");
    MODULE_DESCRIPTION("Netlogic XLP GPIO Driver");
    MODULE_LICENSE("GPL v2");
