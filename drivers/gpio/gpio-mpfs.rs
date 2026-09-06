//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mpfs.c
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


// SPDX-License-Identifier: (GPL-2.0)
//
// Microchip PolarFire SoC (MPFS) GPIO controller driver
//
// Copyright (c) 2018-2024 Microchip Technology Inc. and its subsidiaries
//

pub const MPFS_MAX_NUM_GPIO: c_int = 32;

pub const MPFS_GPIO_TYPE_INT_EDGE_BOTH: c_uint = 0x80;
pub const MPFS_GPIO_TYPE_INT_EDGE_NEG: c_uint = 0x60;
pub const MPFS_GPIO_TYPE_INT_EDGE_POS: c_uint = 0x40;
pub const MPFS_GPIO_TYPE_INT_LEVEL_LOW: c_uint = 0x20;
pub const MPFS_GPIO_TYPE_INT_LEVEL_HIGH: c_uint = 0x00;

pub const MPFS_IRQ_REG: c_uint = 0x80;
pub const MPFS_INP_REG: c_uint = 0x84;
pub const COREGPIO_INP_REG: c_uint = 0x90;
pub const MPFS_OUTP_REG: c_uint = 0x88;
pub const COREGPIO_OUTP_REG: c_uint = 0xA0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_gpio_reg_offsets {
    pub inp: u8,
    pub outp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpfs_gpio_chip {
    pub regs: *mut regmap,
    pub offsets: *const mpfs_gpio_reg_offsets,
    pub gc: gpio_chip,
}

    static const struct regmap_config mpfs_gpio_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .use_raw_spinlock = true,
    };
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_direction_input(gc: *mut gpio_chip, gpio_index: c_uint) -> c_int {
    static int mpfs_gpio_direction_input(struct gpio_chip *gc, unsigned int gpio_index)
    {
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    regmap_update_bits(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index),
    MPFS_GPIO_DIR_MASK, MPFS_GPIO_EN_IN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_direction_output(gc: *mut gpio_chip, gpio_index: c_uint, value: c_int) -> c_int {
    static int mpfs_gpio_direction_output(struct gpio_chip *gc, unsigned int gpio_index, int value)
    {
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    regmap_update_bits(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index),
    MPFS_GPIO_DIR_MASK, MPFS_GPIO_EN_OUT | MPFS_GPIO_EN_OUT_BUF);
    regmap_update_bits(mpfs_gpio.regs, mpfs_gpio.offsets.outp, BIT(gpio_index),
    value << gpio_index);
    return 0;
    }
    static int mpfs_gpio_get_direction(struct gpio_chip *gc,
    unsigned int gpio_index)
    {
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    unsigned int gpio_cfg;
    regmap_read(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index), &gpio_cfg);
    if (gpio_cfg & MPFS_GPIO_EN_IN)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_get(gc: *mut gpio_chip, gpio_index: c_uint) -> c_int {
    static int mpfs_gpio_get(struct gpio_chip *gc, unsigned int gpio_index)
    {
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    if (mpfs_gpio_get_direction(gc, gpio_index) == GPIO_LINE_DIRECTION_OUT)
    return regmap_test_bits(mpfs_gpio.regs, mpfs_gpio.offsets.outp, BIT(gpio_index));
    else
    return regmap_test_bits(mpfs_gpio.regs, mpfs_gpio.offsets.inp, BIT(gpio_index));
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_set(gc: *mut gpio_chip, gpio_index: c_uint, value: c_int) -> c_int {
    static int mpfs_gpio_set(struct gpio_chip *gc, unsigned int gpio_index, int value)
    {
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    int ret;
    mpfs_gpio_get(gc, gpio_index);
    ret = regmap_update_bits(mpfs_gpio.regs, mpfs_gpio.offsets.outp,
    BIT(gpio_index), value << gpio_index);
    mpfs_gpio_get(gc, gpio_index);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int mpfs_gpio_irq_set_type(struct irq_data *data, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    let mut gpio_index: c_int = irqd_to_hwirq(data) % 32;
    u32 interrupt_type;
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    interrupt_type = MPFS_GPIO_TYPE_INT_EDGE_BOTH;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    interrupt_type = MPFS_GPIO_TYPE_INT_EDGE_NEG;
    break;
    case IRQ_TYPE_EDGE_RISING:
    interrupt_type = MPFS_GPIO_TYPE_INT_EDGE_POS;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    interrupt_type = MPFS_GPIO_TYPE_INT_LEVEL_HIGH;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    interrupt_type = MPFS_GPIO_TYPE_INT_LEVEL_LOW;
    break;
    }
    regmap_update_bits(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index),
    MPFS_GPIO_TYPE_INT_MASK, interrupt_type);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_irq_unmask(data: *mut irq_data) {
    static void mpfs_gpio_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    let mut gpio_index: c_int = irqd_to_hwirq(data) % 32;
    gpiochip_enable_irq(gc, gpio_index);
    mpfs_gpio_direction_input(gc, gpio_index);
    regmap_update_bits(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index),
    MPFS_GPIO_EN_INT, MPFS_GPIO_EN_INT);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_irq_mask(data: *mut irq_data) {
    static void mpfs_gpio_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    struct mpfs_gpio_chip *mpfs_gpio = gpiochip_get_data(gc);
    let mut gpio_index: c_int = irqd_to_hwirq(data) % 32;
    regmap_update_bits(mpfs_gpio.regs, MPFS_GPIO_CTRL(gpio_index),
    MPFS_GPIO_EN_INT, 0);
    gpiochip_disable_irq(gc, gpio_index);
    }
    static const struct irq_chip mpfs_gpio_irqchip = {
    .name = "MPFS GPIO",
    .irq_set_type = mpfs_gpio_irq_set_type,
    .irq_mask = mpfs_gpio_irq_mask,
    .irq_unmask = mpfs_gpio_irq_unmask,
    .flags = IRQCHIP_IMMUTABLE | IRQCHIP_MASK_ON_SUSPEND,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_irq_handler(desc: *mut irq_desc) {
    static void mpfs_gpio_irq_handler(struct irq_desc *desc)
    {
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    struct mpfs_gpio_chip *mpfs_gpio = irq_desc_get_handler_data(desc);
    unsigned long status;
    u32 val;
    int i;
    chained_irq_enter(irqchip, desc);
    regmap_read(mpfs_gpio.regs, MPFS_IRQ_REG, &val);
    status = val;
    for_each_set_bit(i, &status, MPFS_MAX_NUM_GPIO) {
    regmap_write(mpfs_gpio.regs, MPFS_IRQ_REG, BIT(i));
    generic_handle_domain_irq(mpfs_gpio.gc.irq.domain, i);
    }
    chained_irq_exit(irqchip, desc);
    }
#[no_mangle]
unsafe extern "C" fn mpfs_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mpfs_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct mpfs_gpio_chip *mpfs_gpio;
    struct gpio_irq_chip *girq;
    struct clk *clk;
    void __iomem *base;
    int ngpios, nirqs, ret;
    mpfs_gpio = devm_kzalloc(dev, sizeof(*mpfs_gpio), GFP_KERNEL);
    if (!mpfs_gpio)
    return -ENOMEM;
    mpfs_gpio.offsets = device_get_match_data(&pdev.dev);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return dev_err_probe(dev, PTR_ERR(base), "failed to ioremap memory resource\n");
    mpfs_gpio.regs = devm_regmap_init_mmio(dev, base, &mpfs_gpio_regmap_config);
    if (IS_ERR(mpfs_gpio.regs))
    return dev_err_probe(dev, PTR_ERR(mpfs_gpio.regs),
    "failed to initialise regmap\n");
    clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "failed to get and enable clock\n");
    ngpios = MPFS_MAX_NUM_GPIO;
    device_property_read_u32(dev, "ngpios", &ngpios);
    if (ngpios > MPFS_MAX_NUM_GPIO)
    ngpios = MPFS_MAX_NUM_GPIO;
    mpfs_gpio.gc.direction_input = mpfs_gpio_direction_input;
    mpfs_gpio.gc.direction_output = mpfs_gpio_direction_output;
    mpfs_gpio.gc.get_direction = mpfs_gpio_get_direction;
    mpfs_gpio.gc.get = mpfs_gpio_get;
    mpfs_gpio.gc.set = mpfs_gpio_set;
    mpfs_gpio.gc.base = -1;
    mpfs_gpio.gc.ngpio = ngpios;
    mpfs_gpio.gc.label = dev_name(dev);
    mpfs_gpio.gc.parent = dev;
    mpfs_gpio.gc.owner = THIS_MODULE;
    nirqs = of_irq_count(node);
    if (nirqs > MPFS_MAX_NUM_GPIO)
    return -ENXIO;
    if (nirqs) {
    girq = &mpfs_gpio.gc.irq;
    gpio_irq_chip_set_chip(girq, &mpfs_gpio_irqchip);
    girq.num_parents = nirqs;
    girq.parents = devm_kcalloc(&pdev.dev, girq.num_parents,
    sizeof(*girq.parents), GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    for (int i = 0; i < nirqs; i++) {
    ret = platform_get_irq(pdev, i);
    if (ret < 0)
    return ret;
    girq.parents[i] = ret;
    girq.parent_handler_data = mpfs_gpio;
    girq.parent_handler = mpfs_gpio_irq_handler;
    }
    girq.handler = handle_level_irq;
    girq.default_type = IRQ_TYPE_NONE;
    }
    return devm_gpiochip_add_data(dev, &mpfs_gpio.gc, mpfs_gpio);
    }
    static const struct mpfs_gpio_reg_offsets mpfs_reg_offsets = {
    .inp = MPFS_INP_REG,
    .outp = MPFS_OUTP_REG,
    };
    static const struct mpfs_gpio_reg_offsets coregpio_reg_offsets = {
    .inp = COREGPIO_INP_REG,
    .outp = COREGPIO_OUTP_REG,
    };
    static const struct of_device_id mpfs_gpio_of_ids[] = {
    {
    .compatible = "microchip,mpfs-gpio",
    .data = &mpfs_reg_offsets,
    }, {
    .compatible = "microchip,coregpio-rtl-v3",
    .data = &coregpio_reg_offsets,
    },
    { /* end of list */ }
    };
    static struct platform_driver mpfs_gpio_driver = {
    .probe = mpfs_gpio_probe,
    .driver = {
    .name = "microchip,mpfs-gpio",
    .of_match_table = mpfs_gpio_of_ids,
    },
    };
    builtin_platform_driver(mpfs_gpio_driver);
