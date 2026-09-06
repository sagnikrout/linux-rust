//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-cadence.c
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
// Copyright 2017-2018 Cadence
// Copyright (C) 2025 Axiado Corporation.
//
// Authors:
// Jan Kotas <jank@cadence.com>
// Boris Brezillon <boris.brezillon@free-electrons.com>
//

pub const CDNS_GPIO_BYPASS_MODE: c_uint = 0x00;
pub const CDNS_GPIO_DIRECTION_MODE: c_uint = 0x04;
pub const CDNS_GPIO_OUTPUT_EN: c_uint = 0x08;
pub const CDNS_GPIO_OUTPUT_VALUE: c_uint = 0x0c;
pub const CDNS_GPIO_INPUT_VALUE: c_uint = 0x10;
pub const CDNS_GPIO_IRQ_MASK: c_uint = 0x14;
pub const CDNS_GPIO_IRQ_EN: c_uint = 0x18;
pub const CDNS_GPIO_IRQ_DIS: c_uint = 0x1c;
pub const CDNS_GPIO_IRQ_STATUS: c_uint = 0x20;
pub const CDNS_GPIO_IRQ_TYPE: c_uint = 0x24;
pub const CDNS_GPIO_IRQ_VALUE: c_uint = 0x28;
pub const CDNS_GPIO_IRQ_ANY_EDGE: c_uint = 0x2c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_gpio_quirks {
    pub skip_init: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_gpio_chip {
    pub gen_gc: gpio_generic_chip,
    pub regs: *mut void __iomem,
    pub bypass_orig: u32,
    pub quirks: *const cdns_gpio_quirks,
}

    static const struct cdns_gpio_quirks cdns_default_quirks = {
    .skip_init = false,
    };
    static const struct cdns_gpio_quirks ax3000_gpio_quirks = {
    .skip_init = true,
    };
#[no_mangle]
unsafe extern "C" fn cdns_gpio_request(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int cdns_gpio_request(struct gpio_chip *chip, unsigned int offset)
    {
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    guard(gpio_generic_lock)(&cgpio.gen_gc);
    iowrite32(ioread32(cgpio.regs + CDNS_GPIO_BYPASS_MODE) & ~BIT(offset),
    cgpio.regs + CDNS_GPIO_BYPASS_MODE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_free(chip: *mut gpio_chip, offset: c_uint) {
    static void cdns_gpio_free(struct gpio_chip *chip, unsigned int offset)
    {
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    guard(gpio_generic_lock)(&cgpio.gen_gc);
    iowrite32(ioread32(cgpio.regs + CDNS_GPIO_BYPASS_MODE) |
    (BIT(offset) & cgpio.bypass_orig),
    cgpio.regs + CDNS_GPIO_BYPASS_MODE);
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_irq_mask(d: *mut irq_data) {
    static void cdns_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    iowrite32(BIT(d.hwirq), cgpio.regs + CDNS_GPIO_IRQ_DIS);
    gpiochip_disable_irq(chip, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_irq_unmask(d: *mut irq_data) {
    static void cdns_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    gpiochip_enable_irq(chip, irqd_to_hwirq(d));
    iowrite32(BIT(d.hwirq), cgpio.regs + CDNS_GPIO_IRQ_EN);
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int cdns_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    u32 int_value;
    u32 int_type;
    u32 int_any;
    let mut mask: u32 = BIT(d.hwirq);
    let mut ret: c_int = 0;
    guard(gpio_generic_lock)(&cgpio.gen_gc);
    int_value = ioread32(cgpio.regs + CDNS_GPIO_IRQ_VALUE) & ~mask;
    int_type = ioread32(cgpio.regs + CDNS_GPIO_IRQ_TYPE) & ~mask;
//
// Interrupt polarity and trigger behaviour is configured like this:
//
// (type, value)
// (0, 0) = Falling edge triggered
// (0, 1) = Rising edge triggered
// (1, 0) = Low level triggered
// (1, 1) = High level triggered
//
    int_any = ioread32(cgpio.regs + CDNS_GPIO_IRQ_ANY_EDGE) & ~mask;
    if (type == IRQ_TYPE_LEVEL_HIGH) {
    int_type |= mask;
    int_value |= mask;
    } else if (type == IRQ_TYPE_LEVEL_LOW) {
    int_type |= mask;
    } else if (type == IRQ_TYPE_EDGE_RISING) {
    int_value |= mask;
    } else if (type == IRQ_TYPE_EDGE_FALLING) {
// edge trigger, int_value remains cleared for falling
    } else if (type == IRQ_TYPE_EDGE_BOTH) {
    int_any |= mask;
    } else {
    return -EINVAL;
    }
    iowrite32(int_value, cgpio.regs + CDNS_GPIO_IRQ_VALUE);
    iowrite32(int_type, cgpio.regs + CDNS_GPIO_IRQ_TYPE);
    iowrite32(int_any, cgpio.regs + CDNS_GPIO_IRQ_ANY_EDGE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_irq_handler(desc: *mut irq_desc) {
    static void cdns_gpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *chip = irq_desc_get_handler_data(desc);
    struct cdns_gpio_chip *cgpio = gpiochip_get_data(chip);
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    unsigned long status;
    int hwirq;
    chained_irq_enter(irqchip, desc);
    status = ioread32(cgpio.regs + CDNS_GPIO_IRQ_STATUS) &
    ~ioread32(cgpio.regs + CDNS_GPIO_IRQ_MASK);
    for_each_set_bit(hwirq, &status, chip.ngpio)
    generic_handle_domain_irq(chip.irq.domain, hwirq);
    chained_irq_exit(irqchip, desc);
    }
    static const struct irq_chip cdns_gpio_irqchip = {
    .name		= "cdns-gpio",
    .irq_mask	= cdns_gpio_irq_mask,
    .irq_unmask	= cdns_gpio_irq_unmask,
    .irq_set_type	= cdns_gpio_irq_set_type,
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static const struct of_device_id cdns_of_ids[] = {
    {
    .compatible = "axiado,ax3000-gpio",
    .data = &ax3000_gpio_quirks
    },
    {
    .compatible = "cdns,gpio-r1p02",
    .data = &cdns_default_quirks
    },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, cdns_of_ids);
#[no_mangle]
unsafe extern "C" fn cdns_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int cdns_gpio_probe(struct platform_device *pdev)
    {
    let mut config: gpio_generic_chip_config = { };
    struct cdns_gpio_chip *cgpio;
    int ret, irq;
    u32 dir_prev;
    let mut num_gpios: u32 = 32;
    struct clk *clk;
    cgpio = devm_kzalloc(&pdev.dev, sizeof(*cgpio), GFP_KERNEL);
    if (!cgpio)
    return -ENOMEM;
    cgpio.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cgpio.regs))
    return PTR_ERR(cgpio.regs);
    of_property_read_u32(pdev.dev.of_node, "ngpios", &num_gpios);
    if (num_gpios > 32) {
    dev_err(&pdev.dev, "ngpios must be less or equal 32\n");
    return -EINVAL;
    }
    cgpio.quirks = device_get_match_data(&pdev.dev);
    if (!cgpio.quirks)
    cgpio.quirks = &cdns_default_quirks;
//
// Set all pins as inputs by default, otherwise:
// gpiochip_lock_as_irq:
// tried to flag a GPIO set as output for IRQ
// Generic GPIO driver stores the direction value internally,
// so it needs to be changed before gpio_generic_chip_init() is called.
//
    dir_prev = ioread32(cgpio.regs + CDNS_GPIO_DIRECTION_MODE);
//
// The AX3000 platform performs the required configuration at boot time
// before Linux boots, so this quirk disables pinmux initialization.
//
    if (!cgpio.quirks.skip_init) {
    iowrite32(GENMASK(num_gpios - 1, 0),
    cgpio.regs + CDNS_GPIO_DIRECTION_MODE);
    }
    config.dev = &pdev.dev;
    config.sz = 4;
    config.dat = cgpio.regs + CDNS_GPIO_INPUT_VALUE;
    config.set = cgpio.regs + CDNS_GPIO_OUTPUT_VALUE;
    config.dirin = cgpio.regs + CDNS_GPIO_DIRECTION_MODE;
    config.flags = GPIO_GENERIC_READ_OUTPUT_REG_SET;
    ret = gpio_generic_chip_init(&cgpio.gen_gc, &config);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register generic gpio, %d\n",
    ret);
    goto err_revert_dir;
    }
    cgpio.gen_gc.gc.label = dev_name(&pdev.dev);
    cgpio.gen_gc.gc.ngpio = num_gpios;
    cgpio.gen_gc.gc.parent = &pdev.dev;
    cgpio.gen_gc.gc.base = -1;
    cgpio.gen_gc.gc.owner = THIS_MODULE;
    cgpio.gen_gc.gc.request = cdns_gpio_request;
    cgpio.gen_gc.gc.free = cdns_gpio_free;
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    dev_err(&pdev.dev,
    "Failed to retrieve peripheral clock, %d\n", ret);
    goto err_revert_dir;
    }
//
// Optional irq_chip support
//
    irq = platform_get_irq(pdev, 0);
    if (irq >= 0) {
    struct gpio_irq_chip *girq;
    girq = &cgpio.gen_gc.gc.irq;
    gpio_irq_chip_set_chip(girq, &cdns_gpio_irqchip);
    girq.parent_handler = cdns_gpio_irq_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents) {
    ret = -ENOMEM;
    goto err_revert_dir;
    }
    girq.parents[0] = irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_level_irq;
    }
    ret = devm_gpiochip_add_data(&pdev.dev, &cgpio.gen_gc.gc, cgpio);
    if (ret < 0) {
    dev_err(&pdev.dev, "Could not register gpiochip, %d\n", ret);
    goto err_revert_dir;
    }
    cgpio.bypass_orig = ioread32(cgpio.regs + CDNS_GPIO_BYPASS_MODE);
//
// Enable gpio outputs, ignored for input direction
//
    if (!cgpio.quirks.skip_init) {
    iowrite32(GENMASK(num_gpios - 1, 0),
    cgpio.regs + CDNS_GPIO_OUTPUT_EN);
    iowrite32(0, cgpio.regs + CDNS_GPIO_BYPASS_MODE);
    }
    platform_set_drvdata(pdev, cgpio);
    return 0;
    err_revert_dir:
    iowrite32(dir_prev, cgpio.regs + CDNS_GPIO_DIRECTION_MODE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdns_gpio_remove(pdev: *mut platform_device) {
    static void cdns_gpio_remove(struct platform_device *pdev)
    {
    struct cdns_gpio_chip *cgpio = platform_get_drvdata(pdev);
    iowrite32(cgpio.bypass_orig, cgpio.regs + CDNS_GPIO_BYPASS_MODE);
    }
    static struct platform_driver cdns_gpio_driver = {
    .driver = {
    .name = "cdns-gpio",
    .of_match_table = cdns_of_ids,
    },
    .probe = cdns_gpio_probe,
    .remove = cdns_gpio_remove,
    };
    module_platform_driver(cdns_gpio_driver);
    MODULE_AUTHOR("Jan Kotas <jank@cadence.com>");
    MODULE_DESCRIPTION("Cadence GPIO driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:cdns-gpio");
