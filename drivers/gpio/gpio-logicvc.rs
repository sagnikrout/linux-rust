//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-logicvc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2019 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

pub const LOGICVC_CTRL_REG: c_uint = 0x40;
pub const LOGICVC_CTRL_GPIO_SHIFT: c_int = 11;
pub const LOGICVC_CTRL_GPIO_BITS: c_int = 5;
pub const LOGICVC_POWER_CTRL_REG: c_uint = 0x78;
pub const LOGICVC_POWER_CTRL_GPIO_SHIFT: c_int = 0;
pub const LOGICVC_POWER_CTRL_GPIO_BITS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct logicvc_gpio {
    pub chip: gpio_chip,
    pub regmap: *mut regmap,
}

    static void logicvc_gpio_offset(struct logicvc_gpio *logicvc, unsigned offset,
    unsigned int *reg, unsigned int *bit)
    {
    if (offset >= LOGICVC_CTRL_GPIO_BITS) {
// reg = LOGICVC_POWER_CTRL_REG;
// To the (virtual) power ctrl offset.
    offset -= LOGICVC_CTRL_GPIO_BITS;
// To the actual bit offset in reg.
    offset += LOGICVC_POWER_CTRL_GPIO_SHIFT;
    } else {
// reg = LOGICVC_CTRL_REG;
// To the actual bit offset in reg.
    offset += LOGICVC_CTRL_GPIO_SHIFT;
    }
// bit = BIT(offset);
    }
#[no_mangle]
unsafe extern "C" fn logicvc_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int logicvc_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct logicvc_gpio *logicvc = gpiochip_get_data(chip);
    unsigned int reg, bit, value;
    int ret;
    logicvc_gpio_offset(logicvc, offset, &reg, &bit);
    ret = regmap_read(logicvc.regmap, reg, &value);
    if (ret)
    return ret;
    return !!(value & bit);
    }
    static int logicvc_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct logicvc_gpio *logicvc = gpiochip_get_data(chip);
    unsigned int reg, bit;
    logicvc_gpio_offset(logicvc, offset, &reg, &bit);
    return regmap_update_bits(logicvc.regmap, reg, bit, value ? bit : 0);
    }
    static int logicvc_gpio_direction_output(struct gpio_chip *chip,
    unsigned offset, int value)
    {
// Pins are always configured as output, so just set the value.
    return logicvc_gpio_set(chip, offset, value);
    }
    static struct regmap_config logicvc_gpio_regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .name		= "logicvc-gpio",
    };
#[no_mangle]
unsafe extern "C" fn logicvc_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int logicvc_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *of_node = dev.of_node;
    struct logicvc_gpio *logicvc;
    int ret;
    logicvc = devm_kzalloc(dev, sizeof(*logicvc), GFP_KERNEL);
    if (!logicvc)
    return -ENOMEM;
// Try to get regmap from parent first.
    logicvc.regmap = syscon_node_to_regmap(of_node.parent);
// Grab our own regmap if that fails.
    if (IS_ERR(logicvc.regmap)) {
    struct resource res;
    void __iomem *base;
    ret = of_address_to_resource(of_node, 0, &res);
    if (ret) {
    dev_err(dev, "Failed to get resource from address\n");
    return ret;
    }
    base = devm_ioremap_resource(dev, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    logicvc_gpio_regmap_config.max_register = resource_size(&res) -
    logicvc_gpio_regmap_config.reg_stride;
    logicvc.regmap =
    devm_regmap_init_mmio(dev, base,
    &logicvc_gpio_regmap_config);
    if (IS_ERR(logicvc.regmap)) {
    dev_err(dev, "Failed to create regmap for I/O\n");
    return PTR_ERR(logicvc.regmap);
    }
    }
    logicvc.chip.parent = dev;
    logicvc.chip.owner = THIS_MODULE;
    logicvc.chip.label = dev_name(dev);
    logicvc.chip.base = -1;
    logicvc.chip.ngpio = LOGICVC_CTRL_GPIO_BITS +
    LOGICVC_POWER_CTRL_GPIO_BITS;
    logicvc.chip.get = logicvc_gpio_get;
    logicvc.chip.set = logicvc_gpio_set;
    logicvc.chip.direction_output = logicvc_gpio_direction_output;
    return devm_gpiochip_add_data(dev, &logicvc.chip, logicvc);
    }
    static const struct of_device_id logicivc_gpio_of_table[] = {
    {
    .compatible	= "xylon,logicvc-3.02.a-gpio",
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, logicivc_gpio_of_table);
    static struct platform_driver logicvc_gpio_driver = {
    .driver	= {
    .name		= "gpio-logicvc",
    .of_match_table	= logicivc_gpio_of_table,
    },
    .probe	= logicvc_gpio_probe,
    };
    module_platform_driver(logicvc_gpio_driver);
    MODULE_AUTHOR("Paul Kocialkowski <paul.kocialkowski@bootlin.com>");
    MODULE_DESCRIPTION("Xylon LogiCVC GPIO driver");
    MODULE_LICENSE("GPL");
