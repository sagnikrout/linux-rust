//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-pca995x.c
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
// LED driver for PCA995x I2C LED drivers
//
// Copyright 2011 bct electronic GmbH
// Copyright 2013 Qtechnology/AS
// Copyright 2022 NXP
// Copyright 2023 Marek Vasut
//

// Register definition
pub const PCA995X_MODE1: c_uint = 0x00;
pub const PCA995X_MODE2: c_uint = 0x01;
pub const PCA995X_LEDOUT0: c_uint = 0x02;
// Auto-increment disabled. Normal mode
pub const PCA995X_MODE1_CFG: c_uint = 0x00;
// LED select registers determine the source that drives LED outputs
pub const PCA995X_LED_OFF: c_uint = 0x0;
pub const PCA995X_LED_ON: c_uint = 0x1;
pub const PCA995X_LED_PWM_MODE: c_uint = 0x2;
pub const PCA995X_LDRX_MASK: c_uint = 0x3;
pub const PCA995X_LDRX_BITS: c_int = 2;
pub const PCA995X_MAX_OUTPUTS: c_int = 24;
pub const PCA995X_OUTPUTS_PER_REG: c_int = 4;
pub const PCA995X_IREFALL_FULL_CFG: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca995x_chipdef {
    pub num_leds: c_uint,
    pub pwm_base: u8,
    pub irefall: u8,
}

    static const struct pca995x_chipdef pca9952_chipdef = {
    .num_leds	= 16,
    .pwm_base	= 0x0a,
    .irefall	= 0x43,
    };
    static const struct pca995x_chipdef pca9955b_chipdef = {
    .num_leds	= 16,
    .pwm_base	= 0x08,
    .irefall	= 0x45,
    };
    static const struct pca995x_chipdef pca9956b_chipdef = {
    .num_leds	= 24,
    .pwm_base	= 0x0a,
    .irefall	= 0x40,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca995x_led {
    pub led_no: c_uint,
    pub ldev: led_classdev,
    pub chip: *mut pca995x_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca995x_chip {
    pub regmap: *mut regmap,
    pub leds: [pca995x_led; PCA995X_MAX_OUTPUTS],
    pub chipdef: *const pca995x_chipdef,
}

    static int pca995x_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct pca995x_led *led = ldev_to_led(led_cdev);
    struct pca995x_chip *chip = led.chip;
    const struct pca995x_chipdef *chipdef = chip.chipdef;
    u8 ledout_addr, pwmout_addr;
    int shift, ret;
    pwmout_addr = chipdef.pwm_base + led.led_no;
    ledout_addr = PCA995X_LEDOUT0 + (led.led_no / PCA995X_OUTPUTS_PER_REG);
    shift = PCA995X_LDRX_BITS * (led.led_no % PCA995X_OUTPUTS_PER_REG);
    switch (brightness) {
    case LED_FULL:
    return regmap_update_bits(chip.regmap, ledout_addr,
    PCA995X_LDRX_MASK << shift,
    PCA995X_LED_ON << shift);
    case LED_OFF:
    return regmap_update_bits(chip.regmap, ledout_addr,
    PCA995X_LDRX_MASK << shift, 0);
    default:
// Adjust brightness as per user input by changing individual PWM
    ret = regmap_write(chip.regmap, pwmout_addr, brightness);
    if (ret)
    return ret;
//
// Change LDRx configuration to individual brightness via PWM.
// LED will stop blinking if it's doing so.
//
    return regmap_update_bits(chip.regmap, ledout_addr,
    PCA995X_LDRX_MASK << shift,
    PCA995X_LED_PWM_MODE << shift);
    }
    }
    static const struct regmap_config pca995x_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x49,
    };
#[no_mangle]
unsafe extern "C" fn pca995x_probe(client: *mut i2c_client) -> c_int {
    static int pca995x_probe(struct i2c_client *client)
    {
    struct fwnode_handle *led_fwnodes[PCA995X_MAX_OUTPUTS] = { 0 };
    struct device *dev = &client.dev;
    const struct pca995x_chipdef *chipdef;
    struct pca995x_chip *chip;
    struct pca995x_led *led;
    int i, j, reg, ret;
    chipdef = device_get_match_data(&client.dev);
    if (!dev_fwnode(dev))
    return -ENODEV;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.chipdef = chipdef;
    chip.regmap = devm_regmap_init_i2c(client, &pca995x_regmap);
    if (IS_ERR(chip.regmap))
    return PTR_ERR(chip.regmap);
    i2c_set_clientdata(client, chip);
    device_for_each_child_node_scoped(dev, child) {
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret)
    return ret;
    if (reg < 0 || reg >= PCA995X_MAX_OUTPUTS || led_fwnodes[reg])
    return -EINVAL;
    led = &chip.leds[reg];
    led_fwnodes[reg] = fwnode_handle_get(child);
    led.chip = chip;
    led.led_no = reg;
    led.ldev.brightness_set_blocking = pca995x_brightness_set;
    led.ldev.max_brightness = 255;
    }
    for (i = 0; i < PCA995X_MAX_OUTPUTS; i++) {
    let mut init_data: led_init_data = {};
    if (!led_fwnodes[i])
    continue;
    init_data.fwnode = led_fwnodes[i];
    ret = devm_led_classdev_register_ext(dev,
    &chip.leds[i].ldev,
    &init_data);
    if (ret < 0) {
    for (j = i; j < PCA995X_MAX_OUTPUTS; j++)
    fwnode_handle_put(led_fwnodes[j]);
    return dev_err_probe(dev, ret,
    "Could not register LED %s\n",
    chip.leds[i].ldev.name);
    }
    }
// Disable LED all-call address and set normal mode
    ret = regmap_write(chip.regmap, PCA995X_MODE1, PCA995X_MODE1_CFG);
    if (ret)
    return ret;
// IREF Output current value for all LEDn outputs
    return regmap_write(chip.regmap, chipdef.irefall, PCA995X_IREFALL_HALF_CFG);
    }
    static const struct i2c_device_id pca995x_id[] = {
    { .name = "pca9952", .driver_data = (kernel_ulong_t)&pca9952_chipdef },
    { .name = "pca9955b", .driver_data = (kernel_ulong_t)&pca9955b_chipdef },
    { .name = "pca9956b", .driver_data = (kernel_ulong_t)&pca9956b_chipdef },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pca995x_id);
    static const struct of_device_id pca995x_of_match[] = {
    { .compatible = "nxp,pca9952", .data = &pca9952_chipdef },
    { .compatible = "nxp,pca9955b", .data = &pca9955b_chipdef },
    { .compatible = "nxp,pca9956b", .data = &pca9956b_chipdef },
    {},
    };
    MODULE_DEVICE_TABLE(of, pca995x_of_match);
    static struct i2c_driver pca995x_driver = {
    .driver = {
    .name = "leds-pca995x",
    .of_match_table = pca995x_of_match,
    },
    .probe = pca995x_probe,
    .id_table = pca995x_id,
    };
    module_i2c_driver(pca995x_driver);
    MODULE_AUTHOR("Isai Gaspar <isaiezequiel.gaspar@nxp.com>");
    MODULE_DESCRIPTION("PCA995x LED driver");
    MODULE_LICENSE("GPL");
