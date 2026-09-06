//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-lpc18xx.c
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
// GPIO driver for NXP LPC18xx/43xx.
//
// Copyright (C) 2018 Vladimir Zapolskiy <vz@mleia.com>
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//

// LPC18xx GPIO register offsets

pub const LPC18XX_MAX_PORTS: c_int = 8;
pub const LPC18XX_PINS_PER_PORT: c_int = 32;
// LPC18xx GPIO pin interrupt controller register offsets
pub const LPC18XX_GPIO_PIN_IC_ISEL: c_uint = 0x00;
pub const LPC18XX_GPIO_PIN_IC_IENR: c_uint = 0x04;
pub const LPC18XX_GPIO_PIN_IC_SIENR: c_uint = 0x08;
pub const LPC18XX_GPIO_PIN_IC_CIENR: c_uint = 0x0c;
pub const LPC18XX_GPIO_PIN_IC_IENF: c_uint = 0x10;
pub const LPC18XX_GPIO_PIN_IC_SIENF: c_uint = 0x14;
pub const LPC18XX_GPIO_PIN_IC_CIENF: c_uint = 0x18;
pub const LPC18XX_GPIO_PIN_IC_RISE: c_uint = 0x1c;
pub const LPC18XX_GPIO_PIN_IC_FALL: c_uint = 0x20;
pub const LPC18XX_GPIO_PIN_IC_IST: c_uint = 0x24;
pub const NR_LPC18XX_GPIO_PIN_IC_IRQS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_gpio_pin_ic {
    pub base: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub lock: raw_spinlock,
    pub gpio: *mut gpio_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_gpio_chip {
    pub gpio: gpio_chip,
    pub base: *mut void __iomem,
    pub pin_ic: *mut lpc18xx_gpio_pin_ic,
    pub lock: spinlock_t,
}

    static inline void lpc18xx_gpio_pin_ic_isel(struct lpc18xx_gpio_pin_ic *ic,
    u32 pin, bool set)
    {
    let mut val: u32 = readl_relaxed(ic.base + LPC18XX_GPIO_PIN_IC_ISEL);
    if (set)
    val &= ~BIT(pin);
    else
    val |= BIT(pin);
    writel_relaxed(val, ic.base + LPC18XX_GPIO_PIN_IC_ISEL);
    }
    static inline void lpc18xx_gpio_pin_ic_set(struct lpc18xx_gpio_pin_ic *ic,
    u32 pin, u32 reg)
    {
    writel_relaxed(BIT(pin), ic.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_pin_ic_mask(d: *mut irq_data) {
    static void lpc18xx_gpio_pin_ic_mask(struct irq_data *d)
    {
    struct lpc18xx_gpio_pin_ic *ic = d.chip_data;
    let mut type: u32 = irqd_get_trigger_type(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    raw_spin_lock(&ic.lock);
    if (type & IRQ_TYPE_LEVEL_MASK || type & IRQ_TYPE_EDGE_RISING)
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_CIENR);
    if (type & IRQ_TYPE_EDGE_FALLING)
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_CIENF);
    raw_spin_unlock(&ic.lock);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq(ic.gpio, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_pin_ic_unmask(d: *mut irq_data) {
    static void lpc18xx_gpio_pin_ic_unmask(struct irq_data *d)
    {
    struct lpc18xx_gpio_pin_ic *ic = d.chip_data;
    let mut type: u32 = irqd_get_trigger_type(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(ic.gpio, hwirq);
    raw_spin_lock(&ic.lock);
    if (type & IRQ_TYPE_LEVEL_MASK || type & IRQ_TYPE_EDGE_RISING)
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_SIENR);
    if (type & IRQ_TYPE_EDGE_FALLING)
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_SIENF);
    raw_spin_unlock(&ic.lock);
    irq_chip_unmask_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_pin_ic_eoi(d: *mut irq_data) {
    static void lpc18xx_gpio_pin_ic_eoi(struct irq_data *d)
    {
    struct lpc18xx_gpio_pin_ic *ic = d.chip_data;
    let mut type: u32 = irqd_get_trigger_type(d);
    raw_spin_lock(&ic.lock);
    if (type & IRQ_TYPE_EDGE_BOTH)
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_IST);
    raw_spin_unlock(&ic.lock);
    irq_chip_eoi_parent(d);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_pin_ic_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int lpc18xx_gpio_pin_ic_set_type(struct irq_data *d, unsigned int type)
    {
    struct lpc18xx_gpio_pin_ic *ic = d.chip_data;
    raw_spin_lock(&ic.lock);
    if (type & IRQ_TYPE_LEVEL_HIGH) {
    lpc18xx_gpio_pin_ic_isel(ic, d.hwirq, true);
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_SIENF);
    } else if (type & IRQ_TYPE_LEVEL_LOW) {
    lpc18xx_gpio_pin_ic_isel(ic, d.hwirq, true);
    lpc18xx_gpio_pin_ic_set(ic, d.hwirq,
    LPC18XX_GPIO_PIN_IC_CIENF);
    } else {
    lpc18xx_gpio_pin_ic_isel(ic, d.hwirq, false);
    }
    raw_spin_unlock(&ic.lock);
    return 0;
    }
    static const struct irq_chip lpc18xx_gpio_pin_ic = {
    .name		= "LPC18xx GPIO pin",
    .irq_mask	= lpc18xx_gpio_pin_ic_mask,
    .irq_unmask	= lpc18xx_gpio_pin_ic_unmask,
    .irq_eoi	= lpc18xx_gpio_pin_ic_eoi,
    .irq_set_type	= lpc18xx_gpio_pin_ic_set_type,
    .flags		= IRQCHIP_IMMUTABLE | IRQCHIP_SET_TYPE_MASKED,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int lpc18xx_gpio_pin_ic_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_irqs, void *data)
    {
    struct irq_fwspec parent_fwspec, *fwspec = data;
    struct lpc18xx_gpio_pin_ic *ic = domain.host_data;
    irq_hw_number_t hwirq;
    int ret;
    if (nr_irqs != 1)
    return -EINVAL;
    hwirq = fwspec.param[0];
    if (hwirq >= NR_LPC18XX_GPIO_PIN_IC_IRQS)
    return -EINVAL;
//
// All LPC18xx/LPC43xx GPIO pin hardware interrupts are translated
// into edge interrupts 32...39 on parent Cortex-M3/M4 NVIC
//
    parent_fwspec.fwnode = domain.parent.fwnode;
    parent_fwspec.param_count = 1;
    parent_fwspec.param[0] = hwirq + 32;
    ret = irq_domain_alloc_irqs_parent(domain, virq, 1, &parent_fwspec);
    if (ret < 0) {
    pr_err("failed to allocate parent irq %u: %d\n",
    parent_fwspec.param[0], ret);
    return ret;
    }
    return irq_domain_set_hwirq_and_chip(domain, virq, hwirq,
    &lpc18xx_gpio_pin_ic, ic);
    }
    static const struct irq_domain_ops lpc18xx_gpio_pin_ic_domain_ops = {
    .alloc	= lpc18xx_gpio_pin_ic_domain_alloc,
    .xlate	= irq_domain_xlate_twocell,
    .free	= irq_domain_free_irqs_common,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_pin_ic_probe(gc: *mut lpc18xx_gpio_chip) -> c_int {
    static int lpc18xx_gpio_pin_ic_probe(struct lpc18xx_gpio_chip *gc)
    {
    struct device *dev = gc.gpio.parent;
    struct irq_domain *parent_domain;
    struct device_node *parent_node;
    struct lpc18xx_gpio_pin_ic *ic;
    struct resource res;
    int ret, index;
    parent_node = of_irq_find_parent(dev.of_node);
    if (!parent_node)
    return -ENXIO;
    parent_domain = irq_find_host(parent_node);
    of_node_put(parent_node);
    if (!parent_domain)
    return -ENXIO;
    ic = devm_kzalloc(dev, sizeof(*ic), GFP_KERNEL);
    if (!ic)
    return -ENOMEM;
    index = of_property_match_string(dev.of_node, "reg-names",
    "gpio-pin-ic");
    if (index < 0) {
    ret = -ENODEV;
    goto free_ic;
    }
    ret = of_address_to_resource(dev.of_node, index, &res);
    if (ret < 0)
    goto free_ic;
    ic.base = devm_ioremap_resource(dev, &res);
    if (IS_ERR(ic.base)) {
    ret = PTR_ERR(ic.base);
    goto free_ic;
    }
    raw_spin_lock_init(&ic.lock);
    ic.domain = irq_domain_create_hierarchy(parent_domain, 0, NR_LPC18XX_GPIO_PIN_IC_IRQS,
    dev_fwnode(dev), &lpc18xx_gpio_pin_ic_domain_ops,
    ic);
    if (!ic.domain) {
    pr_err("unable to add irq domain\n");
    ret = -ENODEV;
    goto free_iomap;
    }
    ic.gpio = &gc.gpio;
    gc.pin_ic = ic;
    return 0;
    free_iomap:
    devm_iounmap(dev, ic.base);
    free_ic:
    devm_kfree(dev, ic);
    return ret;
    }
    static int lpc18xx_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct lpc18xx_gpio_chip *gc = gpiochip_get_data(chip);
    writeb(value ? 1 : 0, gc.base + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int lpc18xx_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct lpc18xx_gpio_chip *gc = gpiochip_get_data(chip);
    return !!readb(gc.base + offset);
    }
    static int lpc18xx_gpio_direction(struct gpio_chip *chip, unsigned offset,
    bool out)
    {
    struct lpc18xx_gpio_chip *gc = gpiochip_get_data(chip);
    unsigned long flags;
    u32 port, pin, dir;
    port = offset / LPC18XX_PINS_PER_PORT;
    pin  = offset % LPC18XX_PINS_PER_PORT;
    spin_lock_irqsave(&gc.lock, flags);
    dir = readl(gc.base + LPC18XX_REG_DIR(port));
    if (out)
    dir |= BIT(pin);
    else
    dir &= ~BIT(pin);
    writel(dir, gc.base + LPC18XX_REG_DIR(port));
    spin_unlock_irqrestore(&gc.lock, flags);
    return 0;
    }
    static int lpc18xx_gpio_direction_input(struct gpio_chip *chip,
    unsigned offset)
    {
    return lpc18xx_gpio_direction(chip, offset, false);
    }
    static int lpc18xx_gpio_direction_output(struct gpio_chip *chip,
    unsigned offset, int value)
    {
    lpc18xx_gpio_set(chip, offset, value);
    return lpc18xx_gpio_direction(chip, offset, true);
    }
    static const struct gpio_chip lpc18xx_chip = {
    .label			= "lpc18xx/43xx-gpio",
    .request		= gpiochip_generic_request,
    .free			= gpiochip_generic_free,
    .direction_input	= lpc18xx_gpio_direction_input,
    .direction_output	= lpc18xx_gpio_direction_output,
    .set			= lpc18xx_gpio_set,
    .get			= lpc18xx_gpio_get,
    .ngpio			= LPC18XX_MAX_PORTS * LPC18XX_PINS_PER_PORT,
    .owner			= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct lpc18xx_gpio_chip *gc;
    int index, ret;
    struct clk *clk;
    gc = devm_kzalloc(dev, sizeof(*gc), GFP_KERNEL);
    if (!gc)
    return -ENOMEM;
    gc.gpio = lpc18xx_chip;
    platform_set_drvdata(pdev, gc);
    index = of_property_match_string(dev.of_node, "reg-names", "gpio");
    if (index < 0) {
// To support backward compatibility take the first resource
    gc.base = devm_platform_ioremap_resource(pdev, 0);
    } else {
    struct resource res;
    ret = of_address_to_resource(dev.of_node, index, &res);
    if (ret < 0)
    return ret;
    gc.base = devm_ioremap_resource(dev, &res);
    }
    if (IS_ERR(gc.base))
    return PTR_ERR(gc.base);
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    dev_err(dev, "input clock not found\n");
    return PTR_ERR(clk);
    }
    spin_lock_init(&gc.lock);
    gc.gpio.parent = dev;
    ret = devm_gpiochip_add_data(dev, &gc.gpio, gc);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add gpio chip\n");
// On error GPIO pin interrupt controller just won't be registered
    lpc18xx_gpio_pin_ic_probe(gc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_gpio_remove(pdev: *mut platform_device) {
    static void lpc18xx_gpio_remove(struct platform_device *pdev)
    {
    struct lpc18xx_gpio_chip *gc = platform_get_drvdata(pdev);
    if (gc.pin_ic)
    irq_domain_remove(gc.pin_ic.domain);
    }
    static const struct of_device_id lpc18xx_gpio_match[] = {
    { .compatible = "nxp,lpc1850-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_gpio_match);
    static struct platform_driver lpc18xx_gpio_driver = {
    .probe	= lpc18xx_gpio_probe,
    .remove	= lpc18xx_gpio_remove,
    .driver	= {
    .name		= "lpc18xx-gpio",
    .of_match_table	= lpc18xx_gpio_match,
    },
    };
    module_platform_driver(lpc18xx_gpio_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_AUTHOR("Vladimir Zapolskiy <vz@mleia.com>");
    MODULE_DESCRIPTION("GPIO driver for LPC18xx/43xx");
    MODULE_LICENSE("GPL v2");
