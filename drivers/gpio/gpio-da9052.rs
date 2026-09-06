//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-da9052.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// GPIO Driver for Dialog DA9052 PMICs.
//
// Copyright(c) 2011 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

pub const DA9052_INPUT: c_int = 1;
pub const DA9052_OUTPUT_OPENDRAIN: c_int = 2;
pub const DA9052_OUTPUT_PUSHPULL: c_int = 3;
pub const DA9052_SUPPLY_VDD_IO1: c_int = 0;
pub const DA9052_DEBOUNCING_OFF: c_int = 0;
pub const DA9052_DEBOUNCING_ON: c_int = 1;
pub const DA9052_OUTPUT_LOWLEVEL: c_int = 0;
pub const DA9052_ACTIVE_LOW: c_int = 0;
pub const DA9052_ACTIVE_HIGH: c_int = 1;
pub const DA9052_GPIO_MAX_PORTS_PER_REGISTER: c_int = 8;

pub const DA9052_GPIO_MASK_UPPER_NIBBLE: c_uint = 0xF0;
pub const DA9052_GPIO_MASK_LOWER_NIBBLE: c_uint = 0x0F;
pub const DA9052_GPIO_NIBBLE_SHIFT: c_int = 4;
pub const DA9052_IRQ_GPI0: c_int = 16;
pub const DA9052_GPIO_ODD_SHIFT: c_int = 7;
pub const DA9052_GPIO_EVEN_SHIFT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_gpio {
    pub da9052: *mut da9052,
    pub gp: gpio_chip,
}

#[no_mangle]
unsafe extern "C" fn da9052_gpio_port_odd(offset: unsigned) -> c_uchar {
    static unsigned char da9052_gpio_port_odd(unsigned offset)
    {
    return offset % 2;
    }
#[no_mangle]
unsafe extern "C" fn da9052_gpio_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int da9052_gpio_get(struct gpio_chip *gc, unsigned offset)
    {
    struct da9052_gpio *gpio = gpiochip_get_data(gc);
    let mut da9052_port_direction: c_int = 0;
    int ret;
    ret = da9052_reg_read(gpio.da9052,
    DA9052_GPIO_0_1_REG + (offset >> 1));
    if (ret < 0)
    return ret;
    if (da9052_gpio_port_odd(offset)) {
    da9052_port_direction = ret & DA9052_GPIO_ODD_PORT_PIN;
    da9052_port_direction >>= 4;
    } else {
    da9052_port_direction = ret & DA9052_GPIO_EVEN_PORT_PIN;
    }
    switch (da9052_port_direction) {
    case DA9052_INPUT:
    if (offset < DA9052_GPIO_MAX_PORTS_PER_REGISTER)
    ret = da9052_reg_read(gpio.da9052,
    DA9052_STATUS_C_REG);
    else
    ret = da9052_reg_read(gpio.da9052,
    DA9052_STATUS_D_REG);
    if (ret < 0)
    return ret;
    return !!(ret & (1 << DA9052_GPIO_SHIFT_COUNT(offset)));
    case DA9052_OUTPUT_PUSHPULL:
    if (da9052_gpio_port_odd(offset))
    return !!(ret & DA9052_GPIO_ODD_PORT_MODE);
    else
    return !!(ret & DA9052_GPIO_EVEN_PORT_MODE);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn da9052_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int da9052_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct da9052_gpio *gpio = gpiochip_get_data(gc);
    if (da9052_gpio_port_odd(offset))
    return da9052_reg_update(gpio.da9052, (offset >> 1) +
    DA9052_GPIO_0_1_REG,
    DA9052_GPIO_ODD_PORT_MODE,
    value << DA9052_GPIO_ODD_SHIFT);
    return da9052_reg_update(gpio.da9052,
    (offset >> 1) + DA9052_GPIO_0_1_REG,
    DA9052_GPIO_EVEN_PORT_MODE,
    value << DA9052_GPIO_EVEN_SHIFT);
    }
#[no_mangle]
unsafe extern "C" fn da9052_gpio_direction_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int da9052_gpio_direction_input(struct gpio_chip *gc, unsigned offset)
    {
    struct da9052_gpio *gpio = gpiochip_get_data(gc);
    unsigned char register_value;
    int ret;
// Format: function - 2 bits type - 1 bit mode - 1 bit
    register_value = DA9052_INPUT | DA9052_ACTIVE_LOW << 2 |
    DA9052_DEBOUNCING_ON << 3;
    if (da9052_gpio_port_odd(offset))
    ret = da9052_reg_update(gpio.da9052, (offset >> 1) +
    DA9052_GPIO_0_1_REG,
    DA9052_GPIO_MASK_UPPER_NIBBLE,
    (register_value <<
    DA9052_GPIO_NIBBLE_SHIFT));
    else
    ret = da9052_reg_update(gpio.da9052, (offset >> 1) +
    DA9052_GPIO_0_1_REG,
    DA9052_GPIO_MASK_LOWER_NIBBLE,
    register_value);
    return ret;
    }
    static int da9052_gpio_direction_output(struct gpio_chip *gc,
    unsigned offset, int value)
    {
    struct da9052_gpio *gpio = gpiochip_get_data(gc);
    unsigned char register_value;
    int ret;
// Format: Function - 2 bits Type - 1 bit Mode - 1 bit
    register_value = DA9052_OUTPUT_PUSHPULL | DA9052_SUPPLY_VDD_IO1 << 2 |
    value << 3;
    if (da9052_gpio_port_odd(offset))
    ret = da9052_reg_update(gpio.da9052, (offset >> 1) +
    DA9052_GPIO_0_1_REG,
    DA9052_GPIO_MASK_UPPER_NIBBLE,
    (register_value <<
    DA9052_GPIO_NIBBLE_SHIFT));
    else
    ret = da9052_reg_update(gpio.da9052, (offset >> 1) +
    DA9052_GPIO_0_1_REG,
    DA9052_GPIO_MASK_LOWER_NIBBLE,
    register_value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9052_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> c_int {
    static int da9052_gpio_to_irq(struct gpio_chip *gc, u32 offset)
    {
    struct da9052_gpio *gpio = gpiochip_get_data(gc);
    struct da9052 *da9052 = gpio.da9052;
    int irq;
    irq = regmap_irq_get_virq(da9052.irq_data, DA9052_IRQ_GPI0 + offset);
    return irq;
    }
    static const struct gpio_chip reference_gp = {
    .label = "da9052-gpio",
    .owner = THIS_MODULE,
    .get = da9052_gpio_get,
    .set = da9052_gpio_set,
    .direction_input = da9052_gpio_direction_input,
    .direction_output = da9052_gpio_direction_output,
    .to_irq = da9052_gpio_to_irq,
    .can_sleep = true,
    .ngpio = 16,
    .base = -1,
    };
#[no_mangle]
unsafe extern "C" fn da9052_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_gpio_probe(struct platform_device *pdev)
    {
    struct da9052_gpio *gpio;
    struct da9052_pdata *pdata;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.da9052 = dev_get_drvdata(pdev.dev.parent);
    pdata = dev_get_platdata(gpio.da9052.dev);
    gpio.gp = reference_gp;
    if (pdata && pdata.gpio_base)
    gpio.gp.base = pdata.gpio_base;
    return devm_gpiochip_add_data(&pdev.dev, &gpio.gp, gpio);
    }
    static struct platform_driver da9052_gpio_driver = {
    .probe = da9052_gpio_probe,
    .driver = {
    .name	= "da9052-gpio",
    },
    };
    module_platform_driver(da9052_gpio_driver);
    MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
    MODULE_DESCRIPTION("DA9052 GPIO Device Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:da9052-gpio");
