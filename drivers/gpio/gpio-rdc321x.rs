//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-rdc321x.c
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
// RDC321x GPIO driver
//
// Copyright (C) 2008, Volker Weiss <dev@tintuc.de>
// Copyright (C) 2007-2010 Florian Fainelli <florian@openwrt.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdc321x_gpio {
    pub lock: spinlock_t,
    pub sb_pdev: *mut pci_dev,
    pub data_reg: [u32; 2],
    pub reg1_ctrl_base: c_int,
    pub reg1_data_base: c_int,
    pub reg2_ctrl_base: c_int,
    pub reg2_data_base: c_int,
    pub chip: gpio_chip,
}

// read GPIO pin
#[no_mangle]
unsafe extern "C" fn rdc_gpio_get_value(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int rdc_gpio_get_value(struct gpio_chip *chip, unsigned gpio)
    {
    struct rdc321x_gpio *gpch;
    let mut value: u32 = 0;
    int reg;
    gpch = gpiochip_get_data(chip);
    reg = gpio < 32 ? gpch.reg1_data_base : gpch.reg2_data_base;
    spin_lock(&gpch.lock);
    pci_write_config_dword(gpch.sb_pdev, reg,
    gpch.data_reg[gpio < 32 ? 0 : 1]);
    pci_read_config_dword(gpch.sb_pdev, reg, &value);
    spin_unlock(&gpch.lock);
    return (1 << (gpio & 0x1f)) & value ? 1 : 0;
    }
    static void rdc_gpio_set_value_impl(struct gpio_chip *chip,
    unsigned gpio, int value)
    {
    struct rdc321x_gpio *gpch;
    let mut reg: c_int = (gpio < 32) ? 0 : 1;
    gpch = gpiochip_get_data(chip);
    if (value)
    gpch.data_reg[reg] |= 1 << (gpio & 0x1f);
    else
    gpch.data_reg[reg] &= ~(1 << (gpio & 0x1f));
    pci_write_config_dword(gpch.sb_pdev,
    reg ? gpch.reg2_data_base : gpch.reg1_data_base,
    gpch.data_reg[reg]);
    }
// set GPIO pin to value
    static int rdc_gpio_set_value(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct rdc321x_gpio *gpch;
    gpch = gpiochip_get_data(chip);
    spin_lock(&gpch.lock);
    rdc_gpio_set_value_impl(chip, gpio, value);
    spin_unlock(&gpch.lock);
    return 0;
    }
    static int rdc_gpio_config(struct gpio_chip *chip,
    unsigned gpio, int value)
    {
    struct rdc321x_gpio *gpch;
    int err;
    u32 reg;
    gpch = gpiochip_get_data(chip);
    spin_lock(&gpch.lock);
    err = pci_read_config_dword(gpch.sb_pdev, gpio < 32 ?
    gpch.reg1_ctrl_base : gpch.reg2_ctrl_base, &reg);
    if (err)
    goto unlock;
    reg |= 1 << (gpio & 0x1f);
    err = pci_write_config_dword(gpch.sb_pdev, gpio < 32 ?
    gpch.reg1_ctrl_base : gpch.reg2_ctrl_base, reg);
    if (err)
    goto unlock;
    rdc_gpio_set_value_impl(chip, gpio, value);
    unlock:
    spin_unlock(&gpch.lock);
    return pcibios_err_to_errno(err);
    }
// configure GPIO pin as input
#[no_mangle]
unsafe extern "C" fn rdc_gpio_direction_input(chip: *mut gpio_chip, gpio: unsigned) -> c_int {
    static int rdc_gpio_direction_input(struct gpio_chip *chip, unsigned gpio)
    {
    return rdc_gpio_config(chip, gpio, 1);
    }
//
// Cache the initial value of both GPIO data registers
//
#[no_mangle]
unsafe extern "C" fn rdc321x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int rdc321x_gpio_probe(struct platform_device *pdev)
    {
    int err;
    struct resource *r;
    struct rdc321x_gpio *rdc321x_gpio_dev;
    struct rdc321x_gpio_pdata *pdata;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata) {
    dev_err(&pdev.dev, "no platform data supplied\n");
    return -ENODEV;
    }
    rdc321x_gpio_dev = devm_kzalloc(&pdev.dev, sizeof(struct rdc321x_gpio),
    GFP_KERNEL);
    if (!rdc321x_gpio_dev)
    return -ENOMEM;
    r = platform_get_resource_byname(pdev, IORESOURCE_IO, "gpio-reg1");
    if (!r) {
    dev_err(&pdev.dev, "failed to get gpio-reg1 resource\n");
    return -ENODEV;
    }
    spin_lock_init(&rdc321x_gpio_dev.lock);
    rdc321x_gpio_dev.sb_pdev = pdata.sb_pdev;
    rdc321x_gpio_dev.reg1_ctrl_base = r.start;
    rdc321x_gpio_dev.reg1_data_base = r.start + 0x4;
    r = platform_get_resource_byname(pdev, IORESOURCE_IO, "gpio-reg2");
    if (!r) {
    dev_err(&pdev.dev, "failed to get gpio-reg2 resource\n");
    return -ENODEV;
    }
    rdc321x_gpio_dev.reg2_ctrl_base = r.start;
    rdc321x_gpio_dev.reg2_data_base = r.start + 0x4;
    rdc321x_gpio_dev.chip.label = "rdc321x-gpio";
    rdc321x_gpio_dev.chip.owner = THIS_MODULE;
    rdc321x_gpio_dev.chip.direction_input = rdc_gpio_direction_input;
    rdc321x_gpio_dev.chip.direction_output = rdc_gpio_config;
    rdc321x_gpio_dev.chip.get = rdc_gpio_get_value;
    rdc321x_gpio_dev.chip.set = rdc_gpio_set_value;
    rdc321x_gpio_dev.chip.base = 0;
    rdc321x_gpio_dev.chip.ngpio = pdata.max_gpios;
    platform_set_drvdata(pdev, rdc321x_gpio_dev);
// This might not be, what others (BIOS, bootloader, etc.)
    wrote to these registers before, but it's a good guess. Still
    better than just using 0xffffffff. */
    err = pci_read_config_dword(rdc321x_gpio_dev.sb_pdev,
    rdc321x_gpio_dev.reg1_data_base,
    &rdc321x_gpio_dev.data_reg[0]);
    if (err)
    return pcibios_err_to_errno(err);
    err = pci_read_config_dword(rdc321x_gpio_dev.sb_pdev,
    rdc321x_gpio_dev.reg2_data_base,
    &rdc321x_gpio_dev.data_reg[1]);
    if (err)
    return pcibios_err_to_errno(err);
    dev_info(&pdev.dev, "registering %d GPIOs\n",
    rdc321x_gpio_dev.chip.ngpio);
    return devm_gpiochip_add_data(&pdev.dev, &rdc321x_gpio_dev.chip,
    rdc321x_gpio_dev);
    }
    static struct platform_driver rdc321x_gpio_driver = {
    .driver.name	= "rdc321x-gpio",
    .probe		= rdc321x_gpio_probe,
    };
    module_platform_driver(rdc321x_gpio_driver);
    MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
    MODULE_DESCRIPTION("RDC321x GPIO driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rdc321x-gpio");
