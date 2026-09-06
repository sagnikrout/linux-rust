//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-blzp1600.c
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
// Copyright (C) 2019 VeriSilicon Limited.
// Copyright (C) 2025 Blaize, Inc.
//

pub const GPIO_DIR_REG: c_uint = 0x00;
pub const GPIO_CTRL_REG: c_uint = 0x04;
pub const GPIO_SET_REG: c_uint = 0x08;
pub const GPIO_CLR_REG: c_uint = 0x0C;
pub const GPIO_ODATA_REG: c_uint = 0x10;
pub const GPIO_IDATA_REG: c_uint = 0x14;
pub const GPIO_IEN_REG: c_uint = 0x18;
pub const GPIO_IS_REG: c_uint = 0x1C;
pub const GPIO_IBE_REG: c_uint = 0x20;
pub const GPIO_IEV_REG: c_uint = 0x24;
pub const GPIO_RIS_REG: c_uint = 0x28;
pub const GPIO_IM_REG: c_uint = 0x2C;
pub const GPIO_MIS_REG: c_uint = 0x30;
pub const GPIO_IC_REG: c_uint = 0x34;
pub const GPIO_DB_REG: c_uint = 0x38;
pub const GPIO_DFG_REG: c_uint = 0x3C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blzp1600_gpio {
    pub base: *mut void __iomem,
    pub gen_gc: gpio_generic_chip,
    pub irq: c_int,
}

    static inline struct blzp1600_gpio *get_blzp1600_gpio_from_irq_data(struct irq_data *d)
    {
    return gpiochip_get_data(irq_data_get_irq_chip_data(d));
    }
    static inline struct blzp1600_gpio *get_blzp1600_gpio_from_irq_desc(struct irq_desc *d)
    {
    return gpiochip_get_data(irq_desc_get_handler_data(d));
    }
#[no_mangle]
pub unsafe extern "C" fn blzp1600_gpio_read(chip: *mut blzp1600_gpio, offset: c_uint) -> u32 {
    static inline u32 blzp1600_gpio_read(struct blzp1600_gpio *chip, unsigned int offset)
    {
    return readl_relaxed(chip.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn blzp1600_gpio_write(chip: *mut blzp1600_gpio, offset: c_uint, val: u32) {
    static inline void blzp1600_gpio_write(struct blzp1600_gpio *chip, unsigned int offset, u32 val)
    {
    writel_relaxed(val, chip.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn blzp1600_gpio_rmw(reg: *mut void __iomem, mask: u32, set: bool) {
    static inline void blzp1600_gpio_rmw(void __iomem *reg, u32 mask, bool set)
    {
    let mut val: u32 = readl_relaxed(reg);
    if (set)
    val |= mask;
    else
    val &= ~mask;
    writel_relaxed(val, reg);
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_mask(d: *mut irq_data) {
    static void blzp1600_gpio_irq_mask(struct irq_data *d)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    blzp1600_gpio_rmw(chip.base + GPIO_IM_REG, BIT(d.hwirq), 1);
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_unmask(d: *mut irq_data) {
    static void blzp1600_gpio_irq_unmask(struct irq_data *d)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    blzp1600_gpio_rmw(chip.base + GPIO_IM_REG, BIT(d.hwirq), 0);
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_ack(d: *mut irq_data) {
    static void blzp1600_gpio_irq_ack(struct irq_data *d)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    blzp1600_gpio_write(chip, GPIO_IC_REG, BIT(d.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_enable(d: *mut irq_data) {
    static void blzp1600_gpio_irq_enable(struct irq_data *d)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    gpiochip_enable_irq(&chip.gen_gc.gc, irqd_to_hwirq(d));
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    blzp1600_gpio_rmw(chip.base + GPIO_DIR_REG, BIT(d.hwirq), 0);
    blzp1600_gpio_rmw(chip.base + GPIO_IEN_REG, BIT(d.hwirq), 1);
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_disable(d: *mut irq_data) {
    static void blzp1600_gpio_irq_disable(struct irq_data *d)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    blzp1600_gpio_rmw(chip.base + GPIO_IEN_REG, BIT(d.hwirq), 0);
    gpiochip_disable_irq(&chip.gen_gc.gc, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irq_set_type(d: *mut irq_data, type: u32) -> c_int {
    static int blzp1600_gpio_irq_set_type(struct irq_data *d, u32 type)
    {
    struct blzp1600_gpio *chip = get_blzp1600_gpio_from_irq_data(d);
    u32 edge_level, single_both, fall_rise;
    let mut mask: c_int = BIT(d.hwirq);
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    edge_level = blzp1600_gpio_read(chip, GPIO_IS_REG);
    single_both = blzp1600_gpio_read(chip, GPIO_IBE_REG);
    fall_rise = blzp1600_gpio_read(chip, GPIO_IEV_REG);
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    edge_level &= ~mask;
    single_both |= mask;
    break;
    case IRQ_TYPE_EDGE_RISING:
    edge_level &= ~mask;
    single_both &= ~mask;
    fall_rise |= mask;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    edge_level &= ~mask;
    single_both &= ~mask;
    fall_rise &= ~mask;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    edge_level |= mask;
    fall_rise |= mask;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    edge_level |= mask;
    fall_rise &= ~mask;
    break;
    default:
    return -EINVAL;
    }
    blzp1600_gpio_write(chip, GPIO_IS_REG, edge_level);
    blzp1600_gpio_write(chip, GPIO_IBE_REG, single_both);
    blzp1600_gpio_write(chip, GPIO_IEV_REG, fall_rise);
    if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
    else
    irq_set_handler_locked(d, handle_edge_irq);
    return 0;
    }
    static const struct irq_chip blzp1600_gpio_irqchip = {
    .name = DRIVER_NAME,
    .irq_ack = blzp1600_gpio_irq_ack,
    .irq_mask = blzp1600_gpio_irq_mask,
    .irq_unmask = blzp1600_gpio_irq_unmask,
    .irq_set_type = blzp1600_gpio_irq_set_type,
    .irq_enable = blzp1600_gpio_irq_enable,
    .irq_disable = blzp1600_gpio_irq_disable,
    .flags = IRQCHIP_IMMUTABLE | IRQCHIP_MASK_ON_SUSPEND,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_irqhandler(desc: *mut irq_desc) {
    static void blzp1600_gpio_irqhandler(struct irq_desc *desc)
    {
    struct blzp1600_gpio *gpio = get_blzp1600_gpio_from_irq_desc(desc);
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    unsigned long irq_status;
    let mut hwirq: c_int = 0;
    chained_irq_enter(irqchip, desc);
    irq_status = blzp1600_gpio_read(gpio, GPIO_RIS_REG);
    for_each_set_bit(hwirq, &irq_status, gpio.gen_gc.gc.ngpio)
    generic_handle_domain_irq(gpio.gen_gc.gc.irq.domain, hwirq);
    chained_irq_exit(irqchip, desc);
    }
    static int blzp1600_gpio_set_debounce(struct gpio_chip *gc, unsigned int offset,
    unsigned int debounce)
    {
    struct blzp1600_gpio *chip = gpiochip_get_data(gc);
    guard(gpio_generic_lock_irqsave)(&chip.gen_gc);
    blzp1600_gpio_rmw(chip.base + GPIO_DB_REG, BIT(offset), debounce);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_set_config(gc: *mut gpio_chip, offset: c_uint, config: c_ulong) -> c_int {
    static int blzp1600_gpio_set_config(struct gpio_chip *gc, unsigned int offset, unsigned long config)
    {
    u32 debounce;
    if (pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE)
    return -ENOTSUPP;
    debounce = pinconf_to_config_argument(config);
    return blzp1600_gpio_set_debounce(gc, offset, debounce);
    }
#[no_mangle]
unsafe extern "C" fn blzp1600_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int blzp1600_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct blzp1600_gpio *chip;
    struct gpio_chip *gc;
    int ret;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(chip.base))
    return PTR_ERR(chip.base);
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 4,
    .dat = chip.base + GPIO_IDATA_REG,
    .set = chip.base + GPIO_SET_REG,
    .clr = chip.base + GPIO_CLR_REG,
    .dirout = chip.base + GPIO_DIR_REG,
    };
    ret = gpio_generic_chip_init(&chip.gen_gc, &config);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to register generic gpio\n");
// configure the gpio chip
    gc = &chip.gen_gc.gc;
    gc.set_config = blzp1600_gpio_set_config;
    if (device_property_present(&pdev.dev, "interrupt-controller")) {
    struct gpio_irq_chip *girq;
    chip.irq = platform_get_irq(pdev, 0);
    if (chip.irq < 0)
    return chip.irq;
    girq = &gc.irq;
    gpio_irq_chip_set_chip(girq, &blzp1600_gpio_irqchip);
    girq.parent_handler = blzp1600_gpio_irqhandler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1, sizeof(*girq.parents), GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = chip.irq;
    girq.default_type = IRQ_TYPE_NONE;
    }
    return devm_gpiochip_add_data(&pdev.dev, gc, chip);
    }
    static const struct of_device_id blzp1600_gpio_of_match[] = {
    { .compatible = "blaize,blzp1600-gpio", },
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, blzp1600_gpio_of_match);
    static struct platform_driver blzp1600_gpio_driver = {
    .driver		= {
    .name	= DRIVER_NAME,
    .of_match_table = blzp1600_gpio_of_match,
    },
    .probe		= blzp1600_gpio_probe,
    };
    module_platform_driver(blzp1600_gpio_driver);
    MODULE_AUTHOR("Nikolaos Pasaloukos <nikolaos.pasaloukos@blaize.com>");
    MODULE_DESCRIPTION("Blaize BLZP1600 GPIO driver");
    MODULE_LICENSE("GPL");
