//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-rda.c
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
// RDA Micro GPIO driver
//
// Copyright (C) 2012 RDA Micro Inc.
// Copyright (C) 2019 Manivannan Sadhasivam
//

pub const RDA_GPIO_OEN_VAL: c_uint = 0x00;
pub const RDA_GPIO_OEN_SET_OUT: c_uint = 0x04;
pub const RDA_GPIO_OEN_SET_IN: c_uint = 0x08;
pub const RDA_GPIO_VAL: c_uint = 0x0c;
pub const RDA_GPIO_SET: c_uint = 0x10;
pub const RDA_GPIO_CLR: c_uint = 0x14;
pub const RDA_GPIO_INT_CTRL_SET: c_uint = 0x18;
pub const RDA_GPIO_INT_CTRL_CLR: c_uint = 0x1c;
pub const RDA_GPIO_INT_CLR: c_uint = 0x20;
pub const RDA_GPIO_INT_STATUS: c_uint = 0x24;
pub const RDA_GPIO_IRQ_RISE_SHIFT: c_int = 0;
pub const RDA_GPIO_IRQ_FALL_SHIFT: c_int = 8;
pub const RDA_GPIO_DEBOUCE_SHIFT: c_int = 16;
pub const RDA_GPIO_LEVEL_SHIFT: c_int = 24;
pub const RDA_GPIO_IRQ_MASK: c_uint = 0xff;
// Each bank consists of 32 GPIOs
pub const RDA_GPIO_BANK_NR: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rda_gpio {
    pub chip: gpio_generic_chip,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub irq: c_int,
}

    static inline void rda_gpio_update(struct gpio_chip *chip, unsigned int offset,
    u16 reg, int val)
    {
    struct rda_gpio *rda_gpio = gpiochip_get_data(chip);
    void __iomem *base = rda_gpio.base;
    unsigned long flags;
    u32 tmp;
    spin_lock_irqsave(&rda_gpio.lock, flags);
    tmp = readl_relaxed(base + reg);
    if (val)
    tmp |= BIT(offset);
    else
    tmp &= ~BIT(offset);
    writel_relaxed(tmp, base + reg);
    spin_unlock_irqrestore(&rda_gpio.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn rda_gpio_irq_mask(data: *mut irq_data) {
    static void rda_gpio_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct rda_gpio *rda_gpio = gpiochip_get_data(chip);
    void __iomem *base = rda_gpio.base;
    let mut offset: u32 = irqd_to_hwirq(data);
    u32 value;
    value = BIT(offset) << RDA_GPIO_IRQ_RISE_SHIFT;
    value |= BIT(offset) << RDA_GPIO_IRQ_FALL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_CLR);
    gpiochip_disable_irq(chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn rda_gpio_irq_ack(data: *mut irq_data) {
    static void rda_gpio_irq_ack(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    rda_gpio_update(chip, offset, RDA_GPIO_INT_CLR, 1);
    }
    static int rda_gpio_set_irq(struct gpio_chip *chip, u32 offset,
    unsigned int flow_type)
    {
    struct rda_gpio *rda_gpio = gpiochip_get_data(chip);
    void __iomem *base = rda_gpio.base;
    u32 value;
    switch (flow_type) {
    case IRQ_TYPE_EDGE_RISING:
// Set rising edge trigger
    value = BIT(offset) << RDA_GPIO_IRQ_RISE_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_SET);
// Switch to edge trigger interrupt
    value = BIT(offset) << RDA_GPIO_LEVEL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_CLR);
    break;
    case IRQ_TYPE_EDGE_FALLING:
// Set falling edge trigger
    value = BIT(offset) << RDA_GPIO_IRQ_FALL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_SET);
// Switch to edge trigger interrupt
    value = BIT(offset) << RDA_GPIO_LEVEL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_CLR);
    break;
    case IRQ_TYPE_EDGE_BOTH:
// Set both edge trigger
    value = BIT(offset) << RDA_GPIO_IRQ_RISE_SHIFT;
    value |= BIT(offset) << RDA_GPIO_IRQ_FALL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_SET);
// Switch to edge trigger interrupt
    value = BIT(offset) << RDA_GPIO_LEVEL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_CLR);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
// Set high level trigger
    value = BIT(offset) << RDA_GPIO_IRQ_RISE_SHIFT;
// Switch to level trigger interrupt
    value |= BIT(offset) << RDA_GPIO_LEVEL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_SET);
    break;
    case IRQ_TYPE_LEVEL_LOW:
// Set low level trigger
    value = BIT(offset) << RDA_GPIO_IRQ_FALL_SHIFT;
// Switch to level trigger interrupt
    value |= BIT(offset) << RDA_GPIO_LEVEL_SHIFT;
    writel_relaxed(value, base + RDA_GPIO_INT_CTRL_SET);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_gpio_irq_unmask(data: *mut irq_data) {
    static void rda_gpio_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    let mut trigger: u32 = irqd_get_trigger_type(data);
    gpiochip_enable_irq(chip, offset);
    rda_gpio_set_irq(chip, offset, trigger);
    }
#[no_mangle]
unsafe extern "C" fn rda_gpio_irq_set_type(data: *mut irq_data, flow_type: c_uint) -> c_int {
    static int rda_gpio_irq_set_type(struct irq_data *data, unsigned int flow_type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    int ret;
    ret = rda_gpio_set_irq(chip, offset, flow_type);
    if (ret)
    return ret;
    if (flow_type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(data, handle_level_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_EDGE_BOTH: flow_type &) -> else {
    else if (flow_type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(data, handle_edge_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_gpio_irq_handler(desc: *mut irq_desc) {
    static void rda_gpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *chip = irq_desc_get_handler_data(desc);
    struct irq_chip *ic = irq_desc_get_chip(desc);
    struct rda_gpio *rda_gpio = gpiochip_get_data(chip);
    unsigned long status;
    u32 n;
    chained_irq_enter(ic, desc);
    status = readl_relaxed(rda_gpio.base + RDA_GPIO_INT_STATUS);
// Only lower 8 bits are capable of generating interrupts
    status &= RDA_GPIO_IRQ_MASK;
    for_each_set_bit(n, &status, RDA_GPIO_BANK_NR)
    generic_handle_domain_irq(chip.irq.domain, n);
    chained_irq_exit(ic, desc);
    }
    static const struct irq_chip rda_gpio_irq_chip = {
    .name = "rda-gpio",
    .irq_ack = rda_gpio_irq_ack,
    .irq_mask = rda_gpio_irq_mask,
    .irq_unmask = rda_gpio_irq_unmask,
    .irq_set_type = rda_gpio_irq_set_type,
    .flags = IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn rda_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int rda_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct gpio_irq_chip *girq;
    struct rda_gpio *rda_gpio;
    u32 ngpios;
    int ret;
    rda_gpio = devm_kzalloc(dev, sizeof(*rda_gpio), GFP_KERNEL);
    if (!rda_gpio)
    return -ENOMEM;
    ret = device_property_read_u32(dev, "ngpios", &ngpios);
    if (ret < 0)
    return ret;
//
// Not all ports have interrupt capability. For instance, on
// RDA8810PL, GPIOC doesn't support interrupt. So we must handle
// those also.
//
    rda_gpio.irq = platform_get_irq(pdev, 0);
    rda_gpio.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rda_gpio.base))
    return PTR_ERR(rda_gpio.base);
    spin_lock_init(&rda_gpio.lock);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = rda_gpio.base + RDA_GPIO_VAL,
    .set = rda_gpio.base + RDA_GPIO_SET,
    .clr = rda_gpio.base + RDA_GPIO_CLR,
    .dirout = rda_gpio.base + RDA_GPIO_OEN_SET_OUT,
    .dirin = rda_gpio.base + RDA_GPIO_OEN_SET_IN,
    .flags = GPIO_GENERIC_READ_OUTPUT_REG_SET,
    };
    ret = gpio_generic_chip_init(&rda_gpio.chip, &config);
    if (ret) {
    dev_err(dev, "failed to initialize the generic GPIO chip\n");
    return ret;
    }
    rda_gpio.chip.gc.label = dev_name(dev);
    rda_gpio.chip.gc.ngpio = ngpios;
    rda_gpio.chip.gc.base = -1;
    if (rda_gpio.irq >= 0) {
    girq = &rda_gpio.chip.gc.irq;
    gpio_irq_chip_set_chip(girq, &rda_gpio_irq_chip);
    girq.handler = handle_bad_irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.parent_handler = rda_gpio_irq_handler;
    girq.parent_handler_data = rda_gpio;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = rda_gpio.irq;
    }
    platform_set_drvdata(pdev, rda_gpio);
    return devm_gpiochip_add_data(dev, &rda_gpio.chip.gc, rda_gpio);
    }
    static const struct of_device_id rda_gpio_of_match[] = {
    { .compatible = "rda,8810pl-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rda_gpio_of_match);
    static struct platform_driver rda_gpio_driver = {
    .probe = rda_gpio_probe,
    .driver = {
    .name = "rda-gpio",
    .of_match_table	= rda_gpio_of_match,
    },
    };
    module_platform_driver_probe(rda_gpio_driver, rda_gpio_probe);
    MODULE_DESCRIPTION("RDA Micro GPIO driver");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
