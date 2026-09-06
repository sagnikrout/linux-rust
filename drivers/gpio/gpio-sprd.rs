//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-sprd.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.
// Copyright (C) 2018 Linaro Ltd.
//

// GPIO registers definition
pub const SPRD_GPIO_DATA: c_uint = 0x0;
pub const SPRD_GPIO_DMSK: c_uint = 0x4;
pub const SPRD_GPIO_DIR: c_uint = 0x8;
pub const SPRD_GPIO_IS: c_uint = 0xc;
pub const SPRD_GPIO_IBE: c_uint = 0x10;
pub const SPRD_GPIO_IEV: c_uint = 0x14;
pub const SPRD_GPIO_IE: c_uint = 0x18;
pub const SPRD_GPIO_RIS: c_uint = 0x1c;
pub const SPRD_GPIO_MIS: c_uint = 0x20;
pub const SPRD_GPIO_IC: c_uint = 0x24;
pub const SPRD_GPIO_INEN: c_uint = 0x28;
// We have 16 banks GPIOs and each bank contain 16 GPIOs
pub const SPRD_GPIO_BANK_NR: c_int = 16;
pub const SPRD_GPIO_NR: c_int = 256;
pub const SPRD_GPIO_BANK_SIZE: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_gpio {
    pub chip: gpio_chip,
    pub base: *mut void __iomem,
    pub lock: raw_spinlock_t,
    pub irq: c_int,
}

    static inline void __iomem *sprd_gpio_bank_base(struct sprd_gpio *sprd_gpio,
    unsigned int bank)
    {
    return sprd_gpio.base + SPRD_GPIO_BANK_SIZE * bank;
    }
    static void sprd_gpio_update(struct gpio_chip *chip, unsigned int offset,
    u16 reg, int val)
    {
    struct sprd_gpio *sprd_gpio = gpiochip_get_data(chip);
    void __iomem *base = sprd_gpio_bank_base(sprd_gpio,
    offset / SPRD_GPIO_BANK_NR);
    unsigned long flags;
    u32 tmp;
    raw_spin_lock_irqsave(&sprd_gpio.lock, flags);
    tmp = readl_relaxed(base + reg);
    if (val)
    tmp |= BIT(SPRD_GPIO_BIT(offset));
    else
    tmp &= ~BIT(SPRD_GPIO_BIT(offset));
    writel_relaxed(tmp, base + reg);
    raw_spin_unlock_irqrestore(&sprd_gpio.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_read(chip: *mut gpio_chip, offset: c_uint, reg: u16) -> c_int {
    static int sprd_gpio_read(struct gpio_chip *chip, unsigned int offset, u16 reg)
    {
    struct sprd_gpio *sprd_gpio = gpiochip_get_data(chip);
    void __iomem *base = sprd_gpio_bank_base(sprd_gpio,
    offset / SPRD_GPIO_BANK_NR);
    return !!(readl_relaxed(base + reg) & BIT(SPRD_GPIO_BIT(offset)));
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int sprd_gpio_request(struct gpio_chip *chip, unsigned int offset)
    {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DMSK, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_free(chip: *mut gpio_chip, offset: c_uint) {
    static void sprd_gpio_free(struct gpio_chip *chip, unsigned int offset)
    {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DMSK, 0);
    }
    static int sprd_gpio_direction_input(struct gpio_chip *chip,
    unsigned int offset)
    {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DIR, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_INEN, 1);
    return 0;
    }
    static int sprd_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DIR, 1);
    sprd_gpio_update(chip, offset, SPRD_GPIO_INEN, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_DATA, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int sprd_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    return sprd_gpio_read(chip, offset, SPRD_GPIO_DATA);
    }
    static int sprd_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    sprd_gpio_update(chip, offset, SPRD_GPIO_DATA, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_irq_mask(data: *mut irq_data) {
    static void sprd_gpio_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IE, 0);
    gpiochip_disable_irq(chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_irq_ack(data: *mut irq_data) {
    static void sprd_gpio_irq_ack(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IC, 1);
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_irq_unmask(data: *mut irq_data) {
    static void sprd_gpio_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IE, 1);
    gpiochip_enable_irq(chip, offset);
    }
    static int sprd_gpio_irq_set_type(struct irq_data *data,
    unsigned int flow_type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    let mut offset: u32 = irqd_to_hwirq(data);
    switch (flow_type) {
    case IRQ_TYPE_EDGE_RISING:
    sprd_gpio_update(chip, offset, SPRD_GPIO_IS, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IBE, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IEV, 1);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IC, 1);
    irq_set_handler_locked(data, handle_edge_irq);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    sprd_gpio_update(chip, offset, SPRD_GPIO_IS, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IBE, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IEV, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IC, 1);
    irq_set_handler_locked(data, handle_edge_irq);
    break;
    case IRQ_TYPE_EDGE_BOTH:
    sprd_gpio_update(chip, offset, SPRD_GPIO_IS, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IBE, 1);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IC, 1);
    irq_set_handler_locked(data, handle_edge_irq);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    sprd_gpio_update(chip, offset, SPRD_GPIO_IS, 1);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IBE, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IEV, 1);
    irq_set_handler_locked(data, handle_level_irq);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    sprd_gpio_update(chip, offset, SPRD_GPIO_IS, 1);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IBE, 0);
    sprd_gpio_update(chip, offset, SPRD_GPIO_IEV, 0);
    irq_set_handler_locked(data, handle_level_irq);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_gpio_irq_handler(desc: *mut irq_desc) {
    static void sprd_gpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *chip = irq_desc_get_handler_data(desc);
    struct irq_chip *ic = irq_desc_get_chip(desc);
    struct sprd_gpio *sprd_gpio = gpiochip_get_data(chip);
    u32 bank, n;
    chained_irq_enter(ic, desc);
    for (bank = 0; bank * SPRD_GPIO_BANK_NR < chip.ngpio; bank++) {
    void __iomem *base = sprd_gpio_bank_base(sprd_gpio, bank);
    unsigned long reg = readl_relaxed(base + SPRD_GPIO_MIS) &
    SPRD_GPIO_BANK_MASK;
    for_each_set_bit(n, &reg, SPRD_GPIO_BANK_NR)
    generic_handle_domain_irq(chip.irq.domain,
    bank * SPRD_GPIO_BANK_NR + n);
    }
    chained_irq_exit(ic, desc);
    }
    static const struct irq_chip sprd_gpio_irqchip = {
    .name = "sprd-gpio",
    .irq_ack = sprd_gpio_irq_ack,
    .irq_mask = sprd_gpio_irq_mask,
    .irq_unmask = sprd_gpio_irq_unmask,
    .irq_set_type = sprd_gpio_irq_set_type,
    .flags = IRQCHIP_SKIP_SET_WAKE | IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn sprd_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_irq_chip *irq;
    struct sprd_gpio *sprd_gpio;
    sprd_gpio = devm_kzalloc(&pdev.dev, sizeof(*sprd_gpio), GFP_KERNEL);
    if (!sprd_gpio)
    return -ENOMEM;
    sprd_gpio.irq = platform_get_irq(pdev, 0);
    if (sprd_gpio.irq < 0)
    return sprd_gpio.irq;
    sprd_gpio.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sprd_gpio.base))
    return PTR_ERR(sprd_gpio.base);
    raw_spin_lock_init(&sprd_gpio.lock);
    sprd_gpio.chip.label = dev_name(&pdev.dev);
    sprd_gpio.chip.ngpio = SPRD_GPIO_NR;
    sprd_gpio.chip.base = -1;
    sprd_gpio.chip.parent = &pdev.dev;
    sprd_gpio.chip.request = sprd_gpio_request;
    sprd_gpio.chip.free = sprd_gpio_free;
    sprd_gpio.chip.get = sprd_gpio_get;
    sprd_gpio.chip.set = sprd_gpio_set;
    sprd_gpio.chip.direction_input = sprd_gpio_direction_input;
    sprd_gpio.chip.direction_output = sprd_gpio_direction_output;
    irq = &sprd_gpio.chip.irq;
    gpio_irq_chip_set_chip(irq, &sprd_gpio_irqchip);
    irq.handler = handle_bad_irq;
    irq.default_type = IRQ_TYPE_NONE;
    irq.parent_handler = sprd_gpio_irq_handler;
    irq.parent_handler_data = sprd_gpio;
    irq.num_parents = 1;
    irq.parents = &sprd_gpio.irq;
    return devm_gpiochip_add_data(&pdev.dev, &sprd_gpio.chip, sprd_gpio);
    }
    static const struct of_device_id sprd_gpio_of_match[] = {
    { .compatible = "sprd,sc9860-gpio", },
    { /* end of list */ }
    };
    MODULE_DEVICE_TABLE(of, sprd_gpio_of_match);
    static struct platform_driver sprd_gpio_driver = {
    .probe = sprd_gpio_probe,
    .driver = {
    .name = "sprd-gpio",
    .of_match_table	= sprd_gpio_of_match,
    },
    };
    module_platform_driver_probe(sprd_gpio_driver, sprd_gpio_probe);
    MODULE_DESCRIPTION("Spreadtrum GPIO driver");
    MODULE_LICENSE("GPL v2");
