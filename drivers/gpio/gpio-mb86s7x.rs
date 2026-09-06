//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mb86s7x.c
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
// linux/drivers/gpio/gpio-mb86s7x.c
//
// Copyright (C) 2015 Fujitsu Semiconductor Limited
// Copyright (C) 2015 Linaro Ltd.
//

//
// Only first 8bits of a register correspond to each pin,
// so there are 4 registers for 32 pins.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb86s70_gpio_chip {
    pub gc: gpio_chip,
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_request(gc: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int mb86s70_gpio_request(struct gpio_chip *gc, unsigned gpio)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&gchip.lock, flags);
    val = readl(gchip.base + PFR(gpio));
    val &= ~OFFSET(gpio);
    writel(val, gchip.base + PFR(gpio));
    spin_unlock_irqrestore(&gchip.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_free(gc: *mut gpio_chip, gpio: unsigned) {
    static void mb86s70_gpio_free(struct gpio_chip *gc, unsigned gpio)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&gchip.lock, flags);
    val = readl(gchip.base + PFR(gpio));
    val |= OFFSET(gpio);
    writel(val, gchip.base + PFR(gpio));
    spin_unlock_irqrestore(&gchip.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_direction_input(gc: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int mb86s70_gpio_direction_input(struct gpio_chip *gc, unsigned gpio)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    unsigned long flags;
    unsigned char val;
    spin_lock_irqsave(&gchip.lock, flags);
    val = readl(gchip.base + DDR(gpio));
    val &= ~OFFSET(gpio);
    writel(val, gchip.base + DDR(gpio));
    spin_unlock_irqrestore(&gchip.lock, flags);
    return 0;
    }
    static int mb86s70_gpio_direction_output(struct gpio_chip *gc,
    unsigned gpio, int value)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    unsigned long flags;
    unsigned char val;
    spin_lock_irqsave(&gchip.lock, flags);
    val = readl(gchip.base + PDR(gpio));
    if (value)
    val |= OFFSET(gpio);
    else
    val &= ~OFFSET(gpio);
    writel(val, gchip.base + PDR(gpio));
    val = readl(gchip.base + DDR(gpio));
    val |= OFFSET(gpio);
    writel(val, gchip.base + DDR(gpio));
    spin_unlock_irqrestore(&gchip.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_get(gc: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int mb86s70_gpio_get(struct gpio_chip *gc, unsigned gpio)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    return !!(readl(gchip.base + PDR(gpio)) & OFFSET(gpio));
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int mb86s70_gpio_set(struct gpio_chip *gc, unsigned int gpio, int value)
    {
    struct mb86s70_gpio_chip *gchip = gpiochip_get_data(gc);
    unsigned long flags;
    unsigned char val;
    spin_lock_irqsave(&gchip.lock, flags);
    val = readl(gchip.base + PDR(gpio));
    if (value)
    val |= OFFSET(gpio);
    else
    val &= ~OFFSET(gpio);
    writel(val, gchip.base + PDR(gpio));
    spin_unlock_irqrestore(&gchip.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int mb86s70_gpio_to_irq(struct gpio_chip *gc, unsigned int offset)
    {
    int irq, index;
    for (index = 0;; index++) {
    irq = platform_get_irq(to_platform_device(gc.parent), index);
    if (irq < 0)
    return irq;
    if (irq_get_irq_data(irq).hwirq == offset)
    return irq;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mb86s70_gpio_probe(struct platform_device *pdev)
    {
    struct mb86s70_gpio_chip *gchip;
    struct clk *clk;
    int ret;
    gchip = devm_kzalloc(&pdev.dev, sizeof(*gchip), GFP_KERNEL);
    if (gchip == core::ptr::null_mut())
    return -ENOMEM;
    platform_set_drvdata(pdev, gchip);
    gchip.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gchip.base))
    return PTR_ERR(gchip.base);
    clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    spin_lock_init(&gchip.lock);
    gchip.gc.direction_output = mb86s70_gpio_direction_output;
    gchip.gc.direction_input = mb86s70_gpio_direction_input;
    gchip.gc.request = mb86s70_gpio_request;
    gchip.gc.free = mb86s70_gpio_free;
    gchip.gc.get = mb86s70_gpio_get;
    gchip.gc.set = mb86s70_gpio_set;
    gchip.gc.to_irq = mb86s70_gpio_to_irq;
    gchip.gc.label = dev_name(&pdev.dev);
    gchip.gc.ngpio = 32;
    gchip.gc.owner = THIS_MODULE;
    gchip.gc.parent = &pdev.dev;
    gchip.gc.base = -1;
    ret = gpiochip_add_data(&gchip.gc, gchip);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "couldn't register gpio driver\n");
    acpi_gpiochip_request_interrupts(&gchip.gc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mb86s70_gpio_remove(pdev: *mut platform_device) {
    static void mb86s70_gpio_remove(struct platform_device *pdev)
    {
    struct mb86s70_gpio_chip *gchip = platform_get_drvdata(pdev);
    acpi_gpiochip_free_interrupts(&gchip.gc);
    gpiochip_remove(&gchip.gc);
    }
    static const struct of_device_id mb86s70_gpio_dt_ids[] = {
    { .compatible = "fujitsu,mb86s70-gpio" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mb86s70_gpio_dt_ids);

    static const struct acpi_device_id mb86s70_gpio_acpi_ids[] = {
    { "SCX0007" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, mb86s70_gpio_acpi_ids);

    static struct platform_driver mb86s70_gpio_driver = {
    .driver = {
    .name = "mb86s70-gpio",
    .of_match_table = mb86s70_gpio_dt_ids,
    .acpi_match_table = ACPI_PTR(mb86s70_gpio_acpi_ids),
    },
    .probe = mb86s70_gpio_probe,
    .remove = mb86s70_gpio_remove,
    };
    module_platform_driver(mb86s70_gpio_driver);
    MODULE_DESCRIPTION("MB86S7x GPIO Driver");
    MODULE_ALIAS("platform:mb86s70-gpio");
    MODULE_LICENSE("GPL");
