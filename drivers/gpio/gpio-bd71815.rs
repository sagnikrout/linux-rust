//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-bd71815.c
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
// Support to GPOs on ROHM BD71815
// Copyright 2021 ROHM Semiconductors.
// Author: Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>
//
// Copyright 2014 Embest Technology Co. Ltd. Inc.
// Author: yanglsh@embest-tech.com
//

// For the BD71815 register definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd71815_gpio {
// chip.parent points the MFD which provides DT node and regmap
    pub chip: gpio_chip,
// dev points to the platform device for devm and prints
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn bd71815gpo_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int bd71815gpo_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct bd71815_gpio *bd71815 = gpiochip_get_data(chip);
    int ret, val;
    ret = regmap_read(bd71815.regmap, BD71815_REG_GPO, &val);
    if (ret)
    return ret;
    return (val >> offset) & 1;
    }
    static int bd71815gpo_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct bd71815_gpio *bd71815 = gpiochip_get_data(chip);
    int bit;
    bit = BIT(offset);
    if (value)
    return regmap_set_bits(bd71815.regmap, BD71815_REG_GPO, bit);
    return regmap_clear_bits(bd71815.regmap, BD71815_REG_GPO, bit);
    }
    static int bd71815_gpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    struct bd71815_gpio *bdgpio = gpiochip_get_data(chip);
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return regmap_update_bits(bdgpio.regmap,
    BD71815_REG_GPO,
    BD71815_GPIO_DRIVE_MASK << offset,
    BD71815_GPIO_OPEN_DRAIN << offset);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return regmap_update_bits(bdgpio.regmap,
    BD71815_REG_GPO,
    BD71815_GPIO_DRIVE_MASK << offset,
    BD71815_GPIO_CMOS << offset);
    default:
    break;
    }
    return -ENOTSUPP;
    }
// BD71815 GPIO is actually GPO
#[no_mangle]
unsafe extern "C" fn bd71815gpo_direction_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int bd71815gpo_direction_get(struct gpio_chip *gc, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
// Template for GPIO chip
    static const struct gpio_chip bd71815gpo_chip = {
    .label			= "bd71815",
    .owner			= THIS_MODULE,
    .get			= bd71815gpo_get,
    .get_direction		= bd71815gpo_direction_get,
    .set			= bd71815gpo_set,
    .set_config		= bd71815_gpio_set_config,
    .can_sleep		= true,
    };

//
// Sigh. The BD71815 and BD71817 were originally designed to support two GPO
// pins. At some point it was noticed the second GPO pin which is the E5 pin
// located at the center of IC is hard to use on PCB (due to the location). It
// was decided to not promote this second GPO and the pin is marked as GND in
// the datasheet. The functionality is still there though! I guess driving a GPO
// connected to the ground is a bad idea. Thus we do not support it by default.
// OTOH - the original driver written by colleagues at Embest did support
// controlling this second GPO. It is thus possible this is used in some of the
// products.
//
// This driver does not by default support configuring this second GPO
// but allows using it by providing the DT property
// "rohm,enable-hidden-gpo".
//
    static int bd71815_init_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask,
    unsigned int ngpios)
    {
    if (ngpios != 2)
    return 0;
    if (gc.parent && device_property_present(gc.parent,
    "rohm,enable-hidden-gpo"))
// valid_mask = BD71815_TWO_GPIOS;
    else
// valid_mask = BD71815_ONE_GPIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpo_bd71815_probe(pdev: *mut platform_device) -> c_int {
    static int gpo_bd71815_probe(struct platform_device *pdev)
    {
    struct bd71815_gpio *g;
    struct device *parent, *dev;
//
// Bind devm lifetime to this platform device => use dev for devm.
// also the prints should originate from this device.
//
    dev = &pdev.dev;
// The device-tree and regmap come from MFD => use parent for that
    parent = dev.parent;
    g = devm_kzalloc(dev, sizeof(*g), GFP_KERNEL);
    if (!g)
    return -ENOMEM;
    g.chip = bd71815gpo_chip;
//
// FIXME: As writing of this the sysfs interface for GPIO control does
// not respect the valid_mask. Do not trust it but rather set the ngpios
// to 1 if "rohm,enable-hidden-gpo" is not given.
//
// This check can be removed later if the sysfs export is fixed and
// if the fix is backported.
//
// For now it is safest to just set the ngpios though.
//
    if (device_property_present(parent, "rohm,enable-hidden-gpo"))
    g.chip.ngpio = 2;
    else
    g.chip.ngpio = 1;
    g.chip.init_valid_mask = bd71815_init_valid_mask;
    g.chip.base = -1;
    g.chip.parent = parent;
    g.regmap = dev_get_regmap(parent, core::ptr::null_mut());
    g.dev = dev;
    return devm_gpiochip_add_data(dev, &g.chip, g);
    }
    static struct platform_driver gpo_bd71815_driver = {
    .driver = {
    .name	= "bd71815-gpo",
    },
    .probe		= gpo_bd71815_probe,
    };
    module_platform_driver(gpo_bd71815_driver);
    MODULE_ALIAS("platform:bd71815-gpo");
    MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
    MODULE_AUTHOR("Peter Yang <yanglsh@embest-tech.com>");
    MODULE_DESCRIPTION("GPO interface for BD71815");
    MODULE_LICENSE("GPL");
