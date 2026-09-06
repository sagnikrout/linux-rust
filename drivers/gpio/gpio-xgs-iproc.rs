//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-xgs-iproc.c
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
// Copyright (C) 2017 Broadcom
//

pub const IPROC_CCA_INT_STS: c_uint = 0x20;
pub const IPROC_CCA_INT_MASK: c_uint = 0x24;
pub const IPROC_GPIO_CCA_DIN: c_uint = 0x0;
pub const IPROC_GPIO_CCA_DOUT: c_uint = 0x4;
pub const IPROC_GPIO_CCA_OUT_EN: c_uint = 0x8;
pub const IPROC_GPIO_CCA_INT_LEVEL: c_uint = 0x10;
pub const IPROC_GPIO_CCA_INT_LEVEL_MASK: c_uint = 0x14;
pub const IPROC_GPIO_CCA_INT_EVENT: c_uint = 0x18;
pub const IPROC_GPIO_CCA_INT_EVENT_MASK: c_uint = 0x1C;
pub const IPROC_GPIO_CCA_INT_EDGE: c_uint = 0x24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_gpio_chip {
    pub gen_gc: gpio_generic_chip,
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub intr: *mut void __iomem,
}

    static inline struct iproc_gpio_chip *
    to_iproc_gpio(struct gpio_chip *gc)
    {
    return container_of(to_gpio_generic_chip(gc), struct iproc_gpio_chip, gen_gc);
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_ack(d: *mut irq_data) {
    static void iproc_gpio_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    let mut pin: c_int = d.hwirq;
    unsigned long flags;
    let mut irq: u32 = d.irq;
    u32 irq_type, event_status = 0;
    spin_lock_irqsave(&chip.lock, flags);
    irq_type = irq_get_trigger_type(irq);
    if (irq_type & IRQ_TYPE_EDGE_BOTH) {
    event_status |= BIT(pin);
    writel_relaxed(event_status,
    chip.base + IPROC_GPIO_CCA_INT_EVENT);
    }
    spin_unlock_irqrestore(&chip.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_unmask(d: *mut irq_data) {
    static void iproc_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    let mut pin: c_int = d.hwirq;
    unsigned long flags;
    let mut irq: u32 = d.irq;
    u32 int_mask, irq_type, event_mask;
    gpiochip_enable_irq(gc, pin);
    spin_lock_irqsave(&chip.lock, flags);
    irq_type = irq_get_trigger_type(irq);
    event_mask = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EVENT_MASK);
    int_mask = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL_MASK);
    if (irq_type & IRQ_TYPE_EDGE_BOTH) {
    event_mask |= 1 << pin;
    writel_relaxed(event_mask,
    chip.base + IPROC_GPIO_CCA_INT_EVENT_MASK);
    } else {
    int_mask |= 1 << pin;
    writel_relaxed(int_mask,
    chip.base + IPROC_GPIO_CCA_INT_LEVEL_MASK);
    }
    spin_unlock_irqrestore(&chip.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_mask(d: *mut irq_data) {
    static void iproc_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    let mut pin: c_int = d.hwirq;
    unsigned long flags;
    let mut irq: u32 = d.irq;
    u32 irq_type, int_mask, event_mask;
    spin_lock_irqsave(&chip.lock, flags);
    irq_type = irq_get_trigger_type(irq);
    event_mask = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EVENT_MASK);
    int_mask = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL_MASK);
    if (irq_type & IRQ_TYPE_EDGE_BOTH) {
    event_mask &= ~BIT(pin);
    writel_relaxed(event_mask,
    chip.base + IPROC_GPIO_CCA_INT_EVENT_MASK);
    } else {
    int_mask &= ~BIT(pin);
    writel_relaxed(int_mask,
    chip.base + IPROC_GPIO_CCA_INT_LEVEL_MASK);
    }
    spin_unlock_irqrestore(&chip.lock, flags);
    gpiochip_disable_irq(gc, pin);
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_set_type(d: *mut irq_data, type: u32) -> c_int {
    static int iproc_gpio_irq_set_type(struct irq_data *d, u32 type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    let mut pin: c_int = d.hwirq;
    unsigned long flags;
    let mut irq: u32 = d.irq;
    u32 event_pol, int_pol;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&chip.lock, flags);
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_RISING:
    event_pol = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EDGE);
    event_pol &= ~BIT(pin);
    writel_relaxed(event_pol, chip.base + IPROC_GPIO_CCA_INT_EDGE);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    event_pol = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EDGE);
    event_pol |= BIT(pin);
    writel_relaxed(event_pol, chip.base + IPROC_GPIO_CCA_INT_EDGE);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    int_pol = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL);
    int_pol &= ~BIT(pin);
    writel_relaxed(int_pol, chip.base + IPROC_GPIO_CCA_INT_LEVEL);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    int_pol = readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL);
    int_pol |= BIT(pin);
    writel_relaxed(int_pol, chip.base + IPROC_GPIO_CCA_INT_LEVEL);
    break;
    default:
// should not come here
    ret = -EINVAL;
    goto out_unlock;
    }
    if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(irq_get_irq_data(irq), handle_level_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_EDGE_BOTH: type &) -> else {
    else if (type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(irq_get_irq_data(irq), handle_edge_irq);
    out_unlock:
    spin_unlock_irqrestore(&chip.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t iproc_gpio_irq_handler(int irq, void *data)
    {
    struct gpio_chip *gc = (struct gpio_chip *)data;
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    int bit;
    let mut int_bits: c_ulong = 0;
    u32 int_status;
// go through the entire GPIOs and handle all interrupts
    int_status = readl_relaxed(chip.intr + IPROC_CCA_INT_STS);
    if (int_status & IPROC_CCA_INT_F_GPIOINT) {
    u32 event, level;
// Get level and edge interrupts
    event =
    readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EVENT_MASK);
    event &= readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_EVENT);
    level = readl_relaxed(chip.base + IPROC_GPIO_CCA_DIN);
    level ^= readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL);
    level &=
    readl_relaxed(chip.base + IPROC_GPIO_CCA_INT_LEVEL_MASK);
    int_bits = level | event;
    for_each_set_bit(bit, &int_bits, gc.ngpio)
    generic_handle_domain_irq(gc.irq.domain, bit);
    }
    return int_bits ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    static void iproc_gpio_irq_print_chip(struct irq_data *d, struct seq_file *p)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct iproc_gpio_chip *chip = to_iproc_gpio(gc);
    seq_puts(p, dev_name(chip.dev));
    }
    static const struct irq_chip iproc_gpio_irq_chip = {
    .irq_ack = iproc_gpio_irq_ack,
    .irq_mask = iproc_gpio_irq_mask,
    .irq_unmask = iproc_gpio_irq_unmask,
    .irq_set_type = iproc_gpio_irq_set_type,
    .irq_print_chip = iproc_gpio_irq_print_chip,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn iproc_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int iproc_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct device_node *dn = pdev.dev.of_node;
    struct iproc_gpio_chip *chip;
    u32 num_gpios;
    int irq, ret;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = dev;
    platform_set_drvdata(pdev, chip);
    spin_lock_init(&chip.lock);
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base))
    return PTR_ERR(chip.base);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = chip.base + IPROC_GPIO_CCA_DIN,
    .set = chip.base + IPROC_GPIO_CCA_DOUT,
    .dirout = chip.base + IPROC_GPIO_CCA_OUT_EN,
    };
    ret = gpio_generic_chip_init(&chip.gen_gc, &config);
    if (ret) {
    dev_err(dev, "unable to init GPIO chip\n");
    return ret;
    }
    chip.gen_gc.gc.label = dev_name(dev);
    if (!of_property_read_u32(dn, "ngpios", &num_gpios))
    chip.gen_gc.gc.ngpio = num_gpios;
    irq = platform_get_irq(pdev, 0);
    if (irq > 0) {
    struct gpio_irq_chip *girq;
    u32 val;
    chip.intr = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(chip.intr))
    return PTR_ERR(chip.intr);
// Enable GPIO interrupts for CCA GPIO
    val = readl_relaxed(chip.intr + IPROC_CCA_INT_MASK);
    val |= IPROC_CCA_INT_F_GPIOINT;
    writel_relaxed(val, chip.intr + IPROC_CCA_INT_MASK);
//
// Directly request the irq here instead of passing
// a flow-handler because the irq is shared.
//
    ret = devm_request_irq(dev, irq, iproc_gpio_irq_handler,
    IRQF_SHARED, chip.gen_gc.gc.label, &chip.gen_gc.gc);
    if (ret) {
    dev_err(dev, "Fail to request IRQ%d: %d\n", irq, ret);
    return ret;
    }
    girq = &chip.gen_gc.gc.irq;
    gpio_irq_chip_set_chip(girq, &iproc_gpio_irq_chip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    }
    ret = devm_gpiochip_add_data(dev, &chip.gen_gc.gc, chip);
    if (ret) {
    dev_err(dev, "unable to add GPIO chip\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_gpio_remove(pdev: *mut platform_device) {
    static void iproc_gpio_remove(struct platform_device *pdev)
    {
    struct iproc_gpio_chip *chip = platform_get_drvdata(pdev);
    if (chip.intr) {
    u32 val;
    val = readl_relaxed(chip.intr + IPROC_CCA_INT_MASK);
    val &= ~IPROC_CCA_INT_F_GPIOINT;
    writel_relaxed(val, chip.intr + IPROC_CCA_INT_MASK);
    }
    }
    static const struct of_device_id bcm_iproc_gpio_of_match[] = {
    { .compatible = "brcm,iproc-gpio-cca" },
    {}
    };
    MODULE_DEVICE_TABLE(of, bcm_iproc_gpio_of_match);
    static struct platform_driver bcm_iproc_gpio_driver = {
    .driver = {
    .name = "iproc-xgs-gpio",
    .of_match_table = bcm_iproc_gpio_of_match,
    },
    .probe = iproc_gpio_probe,
    .remove = iproc_gpio_remove,
    };
    module_platform_driver(bcm_iproc_gpio_driver);
    MODULE_DESCRIPTION("XGS IPROC GPIO driver");
    MODULE_LICENSE("GPL v2");
