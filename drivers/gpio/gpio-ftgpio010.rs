//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ftgpio010.c
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
// Faraday Technolog FTGPIO010 gpiochip and interrupt routines
// Copyright (C) 2017 Linus Walleij <linus.walleij@linaro.org>
//
// Based on arch/arm/mach-gemini/gpio.c:
// Copyright (C) 2008-2009 Paulius Zaleckas <paulius.zaleckas@teltonika.lt>
//
// Based on plat-mxc/gpio.c:
// MXC GPIO support. (c) 2008 Daniel Mack <daniel@caiaq.de>
// Copyright 2008 Juergen Beisert, kernel@pengutronix.de
//

// GPIO registers definition
pub const GPIO_DATA_OUT: c_uint = 0x00;
pub const GPIO_DATA_IN: c_uint = 0x04;
pub const GPIO_DIR: c_uint = 0x08;
pub const GPIO_BYPASS_IN: c_uint = 0x0C;
pub const GPIO_DATA_SET: c_uint = 0x10;
pub const GPIO_DATA_CLR: c_uint = 0x14;
pub const GPIO_PULL_EN: c_uint = 0x18;
pub const GPIO_PULL_TYPE: c_uint = 0x1C;
pub const GPIO_INT_EN: c_uint = 0x20;
pub const GPIO_INT_STAT_RAW: c_uint = 0x24;
pub const GPIO_INT_STAT_MASKED: c_uint = 0x28;
pub const GPIO_INT_MASK: c_uint = 0x2C;
pub const GPIO_INT_CLR: c_uint = 0x30;
pub const GPIO_INT_TYPE: c_uint = 0x34;
pub const GPIO_INT_BOTH_EDGE: c_uint = 0x38;
pub const GPIO_INT_LEVEL: c_uint = 0x3C;
pub const GPIO_DEBOUNCE_EN: c_uint = 0x40;
pub const GPIO_DEBOUNCE_PRESCALE: c_uint = 0x44;
//
// struct ftgpio_gpio - Gemini GPIO state container
// @dev: containing device for this instance
// @chip: generic GPIO chip for this instance
// @base: remapped I/O-memory base
// @clk: silicon clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftgpio_gpio {
    pub dev: *mut device,
    pub chip: gpio_generic_chip,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_ack_irq(d: *mut irq_data) {
    static void ftgpio_gpio_ack_irq(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    writel(BIT(irqd_to_hwirq(d)), g.base + GPIO_INT_CLR);
    }
#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_mask_irq(d: *mut irq_data) {
    static void ftgpio_gpio_mask_irq(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    u32 val;
    val = readl(g.base + GPIO_INT_EN);
    val &= ~BIT(irqd_to_hwirq(d));
    writel(val, g.base + GPIO_INT_EN);
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_unmask_irq(d: *mut irq_data) {
    static void ftgpio_gpio_unmask_irq(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    u32 val;
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    val = readl(g.base + GPIO_INT_EN);
    val |= BIT(irqd_to_hwirq(d));
    writel(val, g.base + GPIO_INT_EN);
    }
#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_set_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int ftgpio_gpio_set_irq_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    let mut mask: u32 = BIT(irqd_to_hwirq(d));
    u32 reg_both, reg_level, reg_type;
    reg_type = readl(g.base + GPIO_INT_TYPE);
    reg_level = readl(g.base + GPIO_INT_LEVEL);
    reg_both = readl(g.base + GPIO_INT_BOTH_EDGE);
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    irq_set_handler_locked(d, handle_edge_irq);
    reg_type &= ~mask;
    reg_both |= mask;
    break;
    case IRQ_TYPE_EDGE_RISING:
    irq_set_handler_locked(d, handle_edge_irq);
    reg_type &= ~mask;
    reg_both &= ~mask;
    reg_level &= ~mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    irq_set_handler_locked(d, handle_edge_irq);
    reg_type &= ~mask;
    reg_both &= ~mask;
    reg_level |= mask;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    irq_set_handler_locked(d, handle_level_irq);
    reg_type |= mask;
    reg_level &= ~mask;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    irq_set_handler_locked(d, handle_level_irq);
    reg_type |= mask;
    reg_level |= mask;
    break;
    default:
    irq_set_handler_locked(d, handle_bad_irq);
    return -EINVAL;
    }
    writel(reg_type, g.base + GPIO_INT_TYPE);
    writel(reg_level, g.base + GPIO_INT_LEVEL);
    writel(reg_both, g.base + GPIO_INT_BOTH_EDGE);
    ftgpio_gpio_ack_irq(d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_irq_handler(desc: *mut irq_desc) {
    static void ftgpio_gpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *gc = irq_desc_get_handler_data(desc);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    int offset;
    unsigned long stat;
    chained_irq_enter(irqchip, desc);
    stat = readl(g.base + GPIO_INT_STAT_RAW);
    if (stat)
    for_each_set_bit(offset, &stat, gc.ngpio)
    generic_handle_domain_irq(gc.irq.domain, offset);
    chained_irq_exit(irqchip, desc);
    }
    static int ftgpio_gpio_set_config(struct gpio_chip *gc, unsigned int offset,
    unsigned long config)
    {
    let mut param: enum pin_config_param = pinconf_to_config_param(config);
    let mut arg: u32 = pinconf_to_config_argument(config);
    struct ftgpio_gpio *g = gpiochip_get_data(gc);
    unsigned long pclk_freq;
    u32 deb_div;
    u32 val;
    if (param != PIN_CONFIG_INPUT_DEBOUNCE)
    return -ENOTSUPP;
//
// Debounce only works if interrupts are enabled. The manual
// states that if PCLK is 66 MHz, and this is set to 0x7D0, then
// PCLK is divided down to 33 kHz for the debounce timer. 0x7D0 is
// 2000 decimal, so what they mean is simply that the PCLK is
// divided by this value.
//
// As we get a debounce setting in microseconds, we calculate the
// desired period time and see if we can get a suitable debounce
// time.
//
    pclk_freq = clk_get_rate(g.clk);
    deb_div = DIV_ROUND_CLOSEST(pclk_freq, arg);
// This register is only 24 bits wide
    if (deb_div > (1 << 24))
    return -ENOTSUPP;
    dev_dbg(g.dev, "prescale divisor: %08x, resulting frequency %lu Hz\n",
    deb_div, (pclk_freq/deb_div));
    val = readl(g.base + GPIO_DEBOUNCE_PRESCALE);
    if (val == deb_div) {
//
// The debounce timer happens to already be set to the
// desirable value, what a coincidence! We can just enable
// debounce on this GPIO line and return. This happens more
// often than you think, for example when all GPIO keys
// on a system are requesting the same debounce interval.
//
    val = readl(g.base + GPIO_DEBOUNCE_EN);
    val |= BIT(offset);
    writel(val, g.base + GPIO_DEBOUNCE_EN);
    return 0;
    }
    val = readl(g.base + GPIO_DEBOUNCE_EN);
    if (val) {
//
// Oh no! Someone is already using the debounce with
// another setting than what we need. Bummer.
//
    return -ENOTSUPP;
    }
// First come, first serve
    writel(deb_div, g.base + GPIO_DEBOUNCE_PRESCALE);
// Enable debounce
    val |= BIT(offset);
    writel(val, g.base + GPIO_DEBOUNCE_EN);
    return 0;
    }
    static const struct irq_chip ftgpio_irq_chip = {
    .name = "FTGPIO010",
    .irq_ack = ftgpio_gpio_ack_irq,
    .irq_mask = ftgpio_gpio_mask_irq,
    .irq_unmask = ftgpio_gpio_unmask_irq,
    .irq_set_type = ftgpio_gpio_set_irq_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn ftgpio_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int ftgpio_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct ftgpio_gpio *g;
    struct gpio_irq_chip *girq;
    int irq;
    int ret;
    g = devm_kzalloc(dev, sizeof(*g), GFP_KERNEL);
    if (!g)
    return -ENOMEM;
    g.dev = dev;
    g.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(g.base))
    return PTR_ERR(g.base);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    g.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(g.clk) && PTR_ERR(g.clk) == -EPROBE_DEFER)
//
// Percolate deferrals, for anything else,
// just live without the clocking.
//
    return PTR_ERR(g.clk);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = g.base + GPIO_DATA_IN,
    .set = g.base + GPIO_DATA_SET,
    .clr = g.base + GPIO_DATA_CLR,
    .dirout = g.base + GPIO_DIR,
    };
    ret = gpio_generic_chip_init(&g.chip, &config);
    if (ret)
    return dev_err_probe(dev, ret, "unable to init generic GPIO\n");
    g.chip.gc.label = dev_name(dev);
    g.chip.gc.base = -1;
    g.chip.gc.parent = dev;
    g.chip.gc.owner = THIS_MODULE;
// ngpio is set by gpio_generic_chip_init()
// We need a silicon clock to do debounce
    if (!IS_ERR(g.clk))
    g.chip.gc.set_config = ftgpio_gpio_set_config;
    girq = &g.chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &ftgpio_irq_chip);
    girq.parent_handler = ftgpio_gpio_irq_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(dev, 1, sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_bad_irq;
    girq.parents[0] = irq;
// Disable, unmask and clear all interrupts
    writel(0x0, g.base + GPIO_INT_EN);
    writel(0x0, g.base + GPIO_INT_MASK);
    writel(~0x0, g.base + GPIO_INT_CLR);
// Clear any use of debounce
    writel(0x0, g.base + GPIO_DEBOUNCE_EN);
    return devm_gpiochip_add_data(dev, &g.chip.gc, g);
    }
    static const struct of_device_id ftgpio_gpio_of_match[] = {
    {
    .compatible = "cortina,gemini-gpio",
    },
    {
    .compatible = "moxa,moxart-gpio",
    },
    {
    .compatible = "faraday,ftgpio010",
    },
    {},
    };
    static struct platform_driver ftgpio_gpio_driver = {
    .driver = {
    .name		= "ftgpio010-gpio",
    .of_match_table = ftgpio_gpio_of_match,
    },
    .probe = ftgpio_gpio_probe,
    };
    builtin_platform_driver(ftgpio_gpio_driver);
