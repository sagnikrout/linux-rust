//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-sifive.c
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
// Copyright (C) 2019 SiFive
//

pub const SIFIVE_GPIO_INPUT_VAL: c_uint = 0x00;
pub const SIFIVE_GPIO_INPUT_EN: c_uint = 0x04;
pub const SIFIVE_GPIO_OUTPUT_EN: c_uint = 0x08;
pub const SIFIVE_GPIO_OUTPUT_VAL: c_uint = 0x0C;
pub const SIFIVE_GPIO_RISE_IE: c_uint = 0x18;
pub const SIFIVE_GPIO_RISE_IP: c_uint = 0x1C;
pub const SIFIVE_GPIO_FALL_IE: c_uint = 0x20;
pub const SIFIVE_GPIO_FALL_IP: c_uint = 0x24;
pub const SIFIVE_GPIO_HIGH_IE: c_uint = 0x28;
pub const SIFIVE_GPIO_HIGH_IP: c_uint = 0x2C;
pub const SIFIVE_GPIO_LOW_IE: c_uint = 0x30;
pub const SIFIVE_GPIO_LOW_IP: c_uint = 0x34;
pub const SIFIVE_GPIO_OUTPUT_XOR: c_uint = 0x40;
pub const SIFIVE_GPIO_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sifive_gpio {
    pub base: *mut void __iomem,
    pub gen_gc: gpio_generic_chip,
    pub regs: *mut regmap,
    pub irq_state: c_ulong,
    pub trigger: [c_uint; SIFIVE_GPIO_MAX],
    pub irq_number: [c_uint; SIFIVE_GPIO_MAX],
}

#[no_mangle]
unsafe extern "C" fn sifive_gpio_set_ie(chip: *mut sifive_gpio, offset: c_uint) {
    static void sifive_gpio_set_ie(struct sifive_gpio *chip, unsigned int offset)
    {
    unsigned int trigger;
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    trigger = (chip.irq_state & BIT(offset)) ? chip.trigger[offset] : 0;
    regmap_update_bits(chip.regs, SIFIVE_GPIO_RISE_IE, BIT(offset),
    (trigger & IRQ_TYPE_EDGE_RISING) ? BIT(offset) : 0);
    regmap_update_bits(chip.regs, SIFIVE_GPIO_FALL_IE, BIT(offset),
    (trigger & IRQ_TYPE_EDGE_FALLING) ? BIT(offset) : 0);
    regmap_update_bits(chip.regs, SIFIVE_GPIO_HIGH_IE, BIT(offset),
    (trigger & IRQ_TYPE_LEVEL_HIGH) ? BIT(offset) : 0);
    regmap_update_bits(chip.regs, SIFIVE_GPIO_LOW_IE, BIT(offset),
    (trigger & IRQ_TYPE_LEVEL_LOW) ? BIT(offset) : 0);
    }
#[no_mangle]
unsafe extern "C" fn sifive_gpio_irq_set_type(d: *mut irq_data, trigger: c_uint) -> c_int {
    static int sifive_gpio_irq_set_type(struct irq_data *d, unsigned int trigger)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sifive_gpio *chip = gpiochip_get_data(gc);
    let mut offset: c_int = irqd_to_hwirq(d);
    if (offset < 0 || offset >= gc.ngpio)
    return -EINVAL;
    chip.trigger[offset] = trigger;
    sifive_gpio_set_ie(chip, offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sifive_gpio_irq_enable(d: *mut irq_data) {
    static void sifive_gpio_irq_enable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sifive_gpio *chip = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    let mut offset: c_int = hwirq % SIFIVE_GPIO_MAX;
    let mut bit: u32 = BIT(offset);
    gpiochip_enable_irq(gc, hwirq);
    irq_chip_enable_parent(d);
// Switch to input
    gc.direction_input(gc, offset);
    scoped_guard(gpio_generic_lock_irqsave, &chip.gen_gc) {
// Clear any sticky pending interrupts
    regmap_write(chip.regs, SIFIVE_GPIO_RISE_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_FALL_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_HIGH_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_LOW_IP, bit);
    }
// Enable interrupts
    assign_bit(offset, &chip.irq_state, 1);
    sifive_gpio_set_ie(chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn sifive_gpio_irq_disable(d: *mut irq_data) {
    static void sifive_gpio_irq_disable(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sifive_gpio *chip = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    let mut offset: c_int = hwirq % SIFIVE_GPIO_MAX;
    assign_bit(offset, &chip.irq_state, 0);
    sifive_gpio_set_ie(chip, offset);
    irq_chip_disable_parent(d);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn sifive_gpio_irq_eoi(d: *mut irq_data) {
    static void sifive_gpio_irq_eoi(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sifive_gpio *chip = gpiochip_get_data(gc);
    let mut offset: c_int = irqd_to_hwirq(d) % SIFIVE_GPIO_MAX;
    let mut bit: u32 = BIT(offset);
    scoped_guard(gpio_generic_lock_irqsave, &chip.gen_gc) {
// Clear all pending interrupts
    regmap_write(chip.regs, SIFIVE_GPIO_RISE_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_FALL_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_HIGH_IP, bit);
    regmap_write(chip.regs, SIFIVE_GPIO_LOW_IP, bit);
    }
    irq_chip_eoi_parent(d);
    }
    static int sifive_gpio_irq_set_affinity(struct irq_data *data,
    const struct cpumask *dest,
    bool force)
    {
    if (data.parent_data)
    return irq_chip_set_affinity_parent(data, dest, force);
    return -EINVAL;
    }
    static const struct irq_chip sifive_gpio_irqchip = {
    .name		= "sifive-gpio",
    .irq_set_type	= sifive_gpio_irq_set_type,
    .irq_mask	= irq_chip_mask_parent,
    .irq_unmask	= irq_chip_unmask_parent,
    .irq_enable	= sifive_gpio_irq_enable,
    .irq_disable	= sifive_gpio_irq_disable,
    .irq_eoi	= sifive_gpio_irq_eoi,
    .irq_set_affinity = sifive_gpio_irq_set_affinity,
    .irq_set_wake	= irq_chip_set_wake_parent,
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int sifive_gpio_child_to_parent_hwirq(struct gpio_chip *gc,
    unsigned int child,
    unsigned int child_type,
    unsigned int *parent,
    unsigned int *parent_type)
    {
    struct sifive_gpio *chip = gpiochip_get_data(gc);
    struct irq_data *d = irq_get_irq_data(chip.irq_number[child]);
// parent_type = IRQ_TYPE_NONE;
// parent = irqd_to_hwirq(d);
    return 0;
    }
    static const struct regmap_config sifive_gpio_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .disable_locking = true,
    };
#[no_mangle]
unsafe extern "C" fn sifive_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int sifive_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct irq_domain *parent;
    struct gpio_irq_chip *girq;
    struct sifive_gpio *chip;
    int ret, ngpio;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base)) {
    dev_err(dev, "failed to allocate device memory\n");
    return PTR_ERR(chip.base);
    }
    chip.regs = devm_regmap_init_mmio(dev, chip.base,
    &sifive_gpio_regmap_config);
    if (IS_ERR(chip.regs))
    return PTR_ERR(chip.regs);
    for (ngpio = 0; ngpio < SIFIVE_GPIO_MAX; ngpio++) {
    ret = platform_get_irq_optional(pdev, ngpio);
    if (ret < 0)
    break;
    chip.irq_number[ngpio] = ret;
    }
    if (!ngpio) {
    dev_err(dev, "no IRQ found\n");
    return -ENODEV;
    }
//
// The check above ensures at least one parent IRQ is valid.
// Assume all parent IRQs belong to the same domain.
//
    parent = irq_get_irq_data(chip.irq_number[0]).domain;
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = chip.base + SIFIVE_GPIO_INPUT_VAL,
    .set = chip.base + SIFIVE_GPIO_OUTPUT_VAL,
    .dirout = chip.base + SIFIVE_GPIO_OUTPUT_EN,
    .dirin = chip.base + SIFIVE_GPIO_INPUT_EN,
    .flags = GPIO_GENERIC_READ_OUTPUT_REG_SET,
    };
    ret = gpio_generic_chip_init(&chip.gen_gc, &config);
    if (ret) {
    dev_err(dev, "unable to init generic GPIO\n");
    return ret;
    }
// Disable all GPIO interrupts before enabling parent interrupts
    regmap_write(chip.regs, SIFIVE_GPIO_RISE_IE, 0);
    regmap_write(chip.regs, SIFIVE_GPIO_FALL_IE, 0);
    regmap_write(chip.regs, SIFIVE_GPIO_HIGH_IE, 0);
    regmap_write(chip.regs, SIFIVE_GPIO_LOW_IE, 0);
    chip.irq_state = 0;
    chip.gen_gc.gc.base = -1;
    chip.gen_gc.gc.ngpio = ngpio;
    chip.gen_gc.gc.label = dev_name(dev);
    chip.gen_gc.gc.parent = dev;
    chip.gen_gc.gc.owner = THIS_MODULE;
    girq = &chip.gen_gc.gc.irq;
    gpio_irq_chip_set_chip(girq, &sifive_gpio_irqchip);
    girq.fwnode = dev_fwnode(dev);
    girq.parent_domain = parent;
    girq.child_to_parent_hwirq = sifive_gpio_child_to_parent_hwirq;
    girq.handler = handle_bad_irq;
    girq.default_type = IRQ_TYPE_NONE;
    return gpiochip_add_data(&chip.gen_gc.gc, chip);
    }
    static const struct of_device_id sifive_gpio_match[] = {
    { .compatible = "sifive,gpio0" },
    { .compatible = "sifive,fu540-c000-gpio" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sifive_gpio_match);
    static struct platform_driver sifive_gpio_driver = {
    .probe		= sifive_gpio_probe,
    .driver = {
    .name	= "sifive_gpio",
    .of_match_table = sifive_gpio_match,
    },
    };
    module_platform_driver(sifive_gpio_driver)
    MODULE_AUTHOR("Yash Shah <yash.shah@sifive.com>");
    MODULE_DESCRIPTION("SiFive GPIO driver");
    MODULE_LICENSE("GPL");
