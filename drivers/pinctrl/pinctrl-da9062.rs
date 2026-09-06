//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/pinctrl-da9062.c
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
// Dialog DA9062 pinctrl and GPIO driver.
// Based on DA9055 GPIO driver.
//
// TODO:
// - add pinmux and pinctrl support (gpio alternate mode)
//
// Documents:
// [1] https://www.dialog-semiconductor.com/sites/default/files/da9062_datasheet_3v6.pdf
//
// Copyright (C) 2019 Pengutronix, Marco Felsch <kernel@pengutronix.de>
//

pub const DA9062_PIN_ALTERNATE: c_uint = 0x00 /* gpio alternate mode */;
pub const DA9062_PIN_GPI: c_uint = 0x01 /* gpio in */;
pub const DA9062_PIN_GPO_OD: c_uint = 0x02 /* gpio out open-drain */;
pub const DA9062_PIN_GPO_PP: c_uint = 0x03 /* gpio out push-pull */;
pub const DA9062_GPIO_NUM: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9062_pctl {
    pub da9062: *mut da9062,
    pub gc: gpio_chip,
    pub pin_config: [c_uint; DA9062_GPIO_NUM],
}

    static int da9062_pctl_get_pin_mode(struct da9062_pctl *pctl,
    unsigned int offset)
    {
    struct regmap *regmap = pctl.da9062.regmap;
    int ret, val;
    ret = regmap_read(regmap, DA9062AA_GPIO_0_1 + (offset >> 1), &val);
    if (ret < 0)
    return ret;
    val >>= DA9062_PIN_SHIFT(offset);
    val &= DA9062AA_GPIO0_PIN_MASK;
    return val;
    }
    static int da9062_pctl_set_pin_mode(struct da9062_pctl *pctl,
    unsigned int offset, unsigned int mode_req)
    {
    struct regmap *regmap = pctl.da9062.regmap;
    let mut mode: c_uint = mode_req;
    unsigned int mask;
    int ret;
    mode &= DA9062AA_GPIO0_PIN_MASK;
    mode <<= DA9062_PIN_SHIFT(offset);
    mask = DA9062AA_GPIO0_PIN_MASK << DA9062_PIN_SHIFT(offset);
    ret = regmap_update_bits(regmap, DA9062AA_GPIO_0_1 + (offset >> 1),
    mask, mode);
    if (!ret)
    pctl.pin_config[offset] = mode_req;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn da9062_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int da9062_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    struct regmap *regmap = pctl.da9062.regmap;
    int gpio_mode, val;
    int ret;
    gpio_mode = da9062_pctl_get_pin_mode(pctl, offset);
    if (gpio_mode < 0)
    return gpio_mode;
    switch (gpio_mode) {
    case DA9062_PIN_ALTERNATE:
    return -ENOTSUPP;
    case DA9062_PIN_GPI:
    ret = regmap_read(regmap, DA9062AA_STATUS_B, &val);
    if (ret < 0)
    return ret;
    break;
    case DA9062_PIN_GPO_OD:
    case DA9062_PIN_GPO_PP:
    ret = regmap_read(regmap, DA9062AA_GPIO_MODE0_4, &val);
    if (ret < 0)
    return ret;
    }
    return !!(val & BIT(offset));
    }
    static int da9062_gpio_set(struct gpio_chip *gc, unsigned int offset,
    int value)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    struct regmap *regmap = pctl.da9062.regmap;
    return regmap_update_bits(regmap, DA9062AA_GPIO_MODE0_4, BIT(offset),
    value << offset);
    }
#[no_mangle]
unsafe extern "C" fn da9062_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int da9062_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    int gpio_mode;
    gpio_mode = da9062_pctl_get_pin_mode(pctl, offset);
    if (gpio_mode < 0)
    return gpio_mode;
    switch (gpio_mode) {
    case DA9062_PIN_ALTERNATE:
    return -ENOTSUPP;
    case DA9062_PIN_GPI:
    return GPIO_LINE_DIRECTION_IN;
    case DA9062_PIN_GPO_OD:
    case DA9062_PIN_GPO_PP:
    return GPIO_LINE_DIRECTION_OUT;
    }
    return -EINVAL;
    }
    static int da9062_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    struct regmap *regmap = pctl.da9062.regmap;
    struct gpio_desc *desc = gpio_device_get_desc(gc.gpiodev, offset);
    unsigned int gpi_type;
    int ret;
    ret = da9062_pctl_set_pin_mode(pctl, offset, DA9062_PIN_GPI);
    if (ret)
    return ret;
//
// If the gpio is active low we should set it in hw too. No worries
// about gpio_get() because we read and return the gpio-level. So the
// gpiolib active_low handling is still correct.
//
// 0 - active low, 1 - active high
//
    gpi_type = !gpiod_is_active_low(desc);
    return regmap_update_bits(regmap, DA9062AA_GPIO_0_1 + (offset >> 1),
    DA9062AA_GPIO0_TYPE_MASK << DA9062_TYPE(offset),
    gpi_type << DA9062_TYPE(offset));
    }
    static int da9062_gpio_direction_output(struct gpio_chip *gc,
    unsigned int offset, int value)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    let mut pin_config: c_uint = pctl.pin_config[offset];
    int ret;
    ret = da9062_pctl_set_pin_mode(pctl, offset, pin_config);
    if (ret)
    return ret;
    return da9062_gpio_set(gc, offset, value);
    }
    static int da9062_gpio_set_config(struct gpio_chip *gc, unsigned int offset,
    unsigned long config)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    struct regmap *regmap = pctl.da9062.regmap;
    int gpio_mode;
//
// We need to meet the following restrictions [1, Figure 18]:
// - PIN_CONFIG_BIAS_PULL_DOWN -> only allowed if the pin is used as
// gpio input
// - PIN_CONFIG_BIAS_PULL_UP   -> only allowed if the pin is used as
// gpio output open-drain.
//
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_BIAS_DISABLE:
    return regmap_update_bits(regmap, DA9062AA_CONFIG_K,
    BIT(offset), 0);
    case PIN_CONFIG_BIAS_PULL_DOWN:
    gpio_mode = da9062_pctl_get_pin_mode(pctl, offset);
    if (gpio_mode < 0)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(DA9062_PIN_GPI: gpio_mode !=) -> else {
    else if (gpio_mode != DA9062_PIN_GPI)
    return -ENOTSUPP;
    return regmap_update_bits(regmap, DA9062AA_CONFIG_K,
    BIT(offset), BIT(offset));
    case PIN_CONFIG_BIAS_PULL_UP:
    gpio_mode = da9062_pctl_get_pin_mode(pctl, offset);
    if (gpio_mode < 0)
    return -EINVAL;
#[no_mangle]
pub unsafe extern "C" fn if(DA9062_PIN_GPO_OD: gpio_mode !=) -> else {
    else if (gpio_mode != DA9062_PIN_GPO_OD)
    return -ENOTSUPP;
    return regmap_update_bits(regmap, DA9062AA_CONFIG_K,
    BIT(offset), BIT(offset));
    case PIN_CONFIG_DRIVE_OPEN_DRAIN:
    return da9062_pctl_set_pin_mode(pctl, offset,
    DA9062_PIN_GPO_OD);
    case PIN_CONFIG_DRIVE_PUSH_PULL:
    return da9062_pctl_set_pin_mode(pctl, offset,
    DA9062_PIN_GPO_PP);
    default:
    return -ENOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn da9062_gpio_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int da9062_gpio_to_irq(struct gpio_chip *gc, unsigned int offset)
    {
    struct da9062_pctl *pctl = gpiochip_get_data(gc);
    struct da9062 *da9062 = pctl.da9062;
    return regmap_irq_get_virq(da9062.regmap_irq,
    DA9062_IRQ_GPI0 + offset);
    }
    static const struct gpio_chip reference_gc = {
    .owner = THIS_MODULE,
    .get = da9062_gpio_get,
    .set = da9062_gpio_set,
    .get_direction = da9062_gpio_get_direction,
    .direction_input = da9062_gpio_direction_input,
    .direction_output = da9062_gpio_direction_output,
    .set_config = da9062_gpio_set_config,
    .to_irq = da9062_gpio_to_irq,
    .can_sleep = true,
    .ngpio = DA9062_GPIO_NUM,
    .base = -1,
    };
#[no_mangle]
unsafe extern "C" fn da9062_pctl_probe(pdev: *mut platform_device) -> c_int {
    static int da9062_pctl_probe(struct platform_device *pdev)
    {
    struct device *parent = pdev.dev.parent;
    struct da9062_pctl *pctl;
    int i;
    device_set_node(&pdev.dev, dev_fwnode(pdev.dev.parent));
    pctl = devm_kzalloc(&pdev.dev, sizeof(*pctl), GFP_KERNEL);
    if (!pctl)
    return -ENOMEM;
    pctl.da9062 = dev_get_drvdata(parent);
    if (!pctl.da9062)
    return -EINVAL;
    if (!device_property_present(parent, "gpio-controller"))
    return 0;
    for (i = 0; i < ARRAY_SIZE(pctl.pin_config); i++)
    pctl.pin_config[i] = DA9062_PIN_GPO_PP;
//
// Currently the driver handles only the GPIO support. The
// pinctrl/pinmux support can be added later if needed.
//
    pctl.gc = reference_gc;
    pctl.gc.label = dev_name(&pdev.dev);
    pctl.gc.parent = &pdev.dev;
    platform_set_drvdata(pdev, pctl);
    return devm_gpiochip_add_data(&pdev.dev, &pctl.gc, pctl);
    }
    static const struct of_device_id da9062_compatible_reg_id_table[] = {
    { .compatible = "dlg,da9062-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, da9062_compatible_reg_id_table);
    static struct platform_driver da9062_pctl_driver = {
    .probe = da9062_pctl_probe,
    .driver = {
    .name	= "da9062-gpio",
    .of_match_table = da9062_compatible_reg_id_table,
    },
    };
    module_platform_driver(da9062_pctl_driver);
    MODULE_AUTHOR("Marco Felsch <kernel@pengutronix.de>");
    MODULE_DESCRIPTION("DA9062 PMIC pinctrl and GPIO Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:da9062-gpio");
