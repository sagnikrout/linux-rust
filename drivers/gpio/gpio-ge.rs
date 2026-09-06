//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ge.c
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
// Driver for GE FPGA based GPIO
//
// Author: Martyn Welch <martyn.welch@ge.com>
//
// 2008 (c) GE Intelligent Platforms Embedded Systems, Inc.
//
// TODO:
//
// Configuration of output modes (totem-pole/open-drain).
// Interrupt configuration - interrupts are always generated, the FPGA relies
// on the I/O interrupt controllers mask to stop them from being propagated.
//

pub const GEF_GPIO_DIRECT: c_uint = 0x00;
pub const GEF_GPIO_IN: c_uint = 0x04;
pub const GEF_GPIO_OUT: c_uint = 0x08;
pub const GEF_GPIO_TRIG: c_uint = 0x0C;
pub const GEF_GPIO_POLAR_A: c_uint = 0x10;
pub const GEF_GPIO_POLAR_B: c_uint = 0x14;
pub const GEF_GPIO_INT_STAT: c_uint = 0x18;
pub const GEF_GPIO_OVERRUN: c_uint = 0x1C;
pub const GEF_GPIO_MODE: c_uint = 0x20;
    static const struct of_device_id gef_gpio_ids[] = {
    {
    .compatible	= "gef,sbc610-gpio",
    .data		= (void *)19,
    }, {
    .compatible	= "gef,sbc310-gpio",
    .data		= (void *)6,
    }, {
    .compatible	= "ge,imp3a-gpio",
    .data		= (void *)16,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, gef_gpio_ids);
#[no_mangle]
unsafe extern "C" fn gef_gpio_probe(pdev: *mut platform_device) -> int __init {
    static int __init gef_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct gpio_generic_chip *chip;
    struct gpio_chip *gc;
    void __iomem *regs;
    int ret;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = regs + GEF_GPIO_IN,
    .set = regs + GEF_GPIO_OUT,
    .dirin = regs + GEF_GPIO_DIRECT,
    .flags = GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };
    ret = gpio_generic_chip_init(chip, &config);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to initialize the generic GPIO chip\n");
    gc = &chip.gc;
// Setup pointers to chip functions
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pfw", dev_fwnode(dev));
    if (!gc.label)
    return -ENOMEM;
    gc.base = -1;
    gc.ngpio = (uintptr_t)device_get_match_data(dev);
// This function adds a memory mapped GPIO chip
    ret = devm_gpiochip_add_data(dev, gc, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(dev, ret, "GPIO chip registration failed\n");
    return 0;
    };
    static struct platform_driver gef_gpio_driver = {
    .driver = {
    .name		= "gef-gpio",
    .of_match_table	= gef_gpio_ids,
    },
    };
    module_platform_driver_probe(gef_gpio_driver, gef_gpio_probe);
    MODULE_DESCRIPTION("GE I/O FPGA GPIO driver");
    MODULE_AUTHOR("Martyn Welch <martyn.welch@ge.com>");
    MODULE_LICENSE("GPL");
