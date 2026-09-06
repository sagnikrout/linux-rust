//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rpi-panel-v2-regulator.c
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
// Copyright (C) 2022 Raspberry Pi Ltd.
// Copyright (C) 2025 Marek Vasut
//

// I2C registers of the microcontroller.
pub const REG_ID: c_uint = 0x01;
pub const REG_POWERON: c_uint = 0x02;
pub const REG_PWM: c_uint = 0x03;
// Bits for poweron register

// Bits for the PWM register

// Treat LCD_RESET and CTP_RESET as GPIOs
pub const NUM_GPIO: c_int = 2;
    static const struct regmap_config rpi_panel_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_PWM,
    .can_sleep = true,
    };
    static int rpi_panel_v2_pwm_apply(struct pwm_chip *chip, struct pwm_device *pwm,
    const struct pwm_state *state)
    {
    struct regmap *regmap = pwmchip_get_drvdata(chip);
    unsigned int duty;
    if (state.polarity != PWM_POLARITY_NORMAL)
    return -EINVAL;
    if (!state.enabled)
    return regmap_write(regmap, REG_PWM, 0);
    duty = pwm_get_relative_duty_cycle(state, PWM_BL_MASK);
    return regmap_write(regmap, REG_PWM, duty | PWM_BL_ENABLE);
    }
    static const struct pwm_ops rpi_panel_v2_pwm_ops = {
    .apply = rpi_panel_v2_pwm_apply,
    };
//
// I2C driver interface functions
//
#[no_mangle]
unsafe extern "C" fn rpi_panel_v2_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int rpi_panel_v2_i2c_probe(struct i2c_client *i2c)
    {
    struct gpio_regmap_config gconfig = {
    .ngpio		= NUM_GPIO,
    .ngpio_per_reg	= NUM_GPIO,
    .parent		= &i2c.dev,
    .reg_set_base	= REG_POWERON,
    };
    struct regmap *regmap;
    struct pwm_chip *pc;
    int ret;
    pc = devm_pwmchip_alloc(&i2c.dev, 1, 0);
    if (IS_ERR(pc))
    return PTR_ERR(pc);
    pc.ops = &rpi_panel_v2_pwm_ops;
    regmap = devm_regmap_init_i2c(i2c, &rpi_panel_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&i2c.dev, PTR_ERR(regmap), "Failed to allocate regmap\n");
    pwmchip_set_drvdata(pc, regmap);
    regmap_write(regmap, REG_POWERON, 0);
    gconfig.regmap = regmap;
    ret = PTR_ERR_OR_ZERO(devm_gpio_regmap_register(&i2c.dev, &gconfig));
    if (ret)
    return dev_err_probe(&i2c.dev, ret, "Failed to create gpiochip\n");
    i2c_set_clientdata(i2c, regmap);
    return devm_pwmchip_add(&i2c.dev, pc);
    }
#[no_mangle]
unsafe extern "C" fn rpi_panel_v2_i2c_shutdown(client: *mut i2c_client) {
    static void rpi_panel_v2_i2c_shutdown(struct i2c_client *client)
    {
    struct regmap *regmap = i2c_get_clientdata(client);
    regmap_write(regmap, REG_PWM, 0);
    regmap_write(regmap, REG_POWERON, 0);
    }
    static const struct of_device_id rpi_panel_v2_dt_ids[] = {
    { .compatible = "raspberrypi,touchscreen-panel-regulator-v2" },
    { },
    };
    MODULE_DEVICE_TABLE(of, rpi_panel_v2_dt_ids);
    static struct i2c_driver rpi_panel_v2_regulator_driver = {
    .driver = {
    .name = "rpi_touchscreen_v2",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = rpi_panel_v2_dt_ids,
    },
    .probe = rpi_panel_v2_i2c_probe,
    .shutdown = rpi_panel_v2_i2c_shutdown,
    };
    module_i2c_driver(rpi_panel_v2_regulator_driver);
    MODULE_AUTHOR("Dave Stevenson <dave.stevenson@raspberrypi.com>");
    MODULE_DESCRIPTION("Regulator device driver for Raspberry Pi 7-inch V2 touchscreen");
    MODULE_LICENSE("GPL");
