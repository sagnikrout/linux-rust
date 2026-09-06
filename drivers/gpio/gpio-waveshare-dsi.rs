//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-waveshare-dsi.c
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
// Copyright (C) 2024 Waveshare International Limited
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

// I2C registers of the microcontroller.
pub const REG_TP: c_uint = 0x94;
pub const REG_LCD: c_uint = 0x95;
pub const REG_PWM: c_uint = 0x96;
pub const REG_SIZE: c_uint = 0x97;
pub const REG_ID: c_uint = 0x98;
pub const REG_VERSION: c_uint = 0x99;
    enum {
    GPIO_AVDD = 0,
    GPIO_PANEL_RESET = 1,
    GPIO_BL_ENABLE = 2,
    GPIO_IOVCC = 4,
    GPIO_VCC = 8,
    GPIO_TS_RESET = 9,
    };
pub const NUM_GPIO: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waveshare_gpio {
    pub dir_lock: mutex,
    pub pwr_lock: mutex,
    pub regmap: *mut regmap,
    pub poweron_state: u16,
    pub gc: gpio_chip,
}

    static const struct regmap_config waveshare_gpio_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_VERSION,
    };
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_get(state: *mut waveshare_gpio, offset: c_uint) -> c_int {
    static int waveshare_gpio_get(struct waveshare_gpio *state, unsigned int offset)
    {
    u16 pwr_state;
    guard(mutex)(&state.pwr_lock);
    pwr_state = state.poweron_state & BIT(offset);
    return !!pwr_state;
    }
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_set(state: *mut waveshare_gpio, offset: c_uint, value: c_int) -> c_int {
    static int waveshare_gpio_set(struct waveshare_gpio *state, unsigned int offset, int value)
    {
    u16 last_val;
    int err;
    guard(mutex)(&state.pwr_lock);
    last_val = state.poweron_state;
    if (value)
    last_val |= BIT(offset);
    else
    last_val &= ~BIT(offset);
    state.poweron_state = last_val;
    err = regmap_write(state.regmap, REG_TP, last_val >> 8);
    if (!err)
    err = regmap_write(state.regmap, REG_LCD, last_val & 0xff);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int waveshare_gpio_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int waveshare_gpio_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct waveshare_gpio *state = gpiochip_get_data(gc);
    return waveshare_gpio_get(state, offset);
    }
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int waveshare_gpio_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct waveshare_gpio *state = gpiochip_get_data(gc);
    return waveshare_gpio_set(state, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_update_status(bl: *mut backlight_device) -> c_int {
    static int waveshare_gpio_update_status(struct backlight_device *bl)
    {
    struct waveshare_gpio *state = bl_get_data(bl);
    let mut brightness: c_int = backlight_get_brightness(bl);
    waveshare_gpio_set(state, GPIO_BL_ENABLE, brightness);
    return regmap_write(state.regmap, REG_PWM, brightness);
    }
    static const struct backlight_ops waveshare_gpio_bl = {
    .update_status = waveshare_gpio_update_status,
    };
#[no_mangle]
unsafe extern "C" fn waveshare_gpio_probe(i2c: *mut i2c_client) -> c_int {
    static int waveshare_gpio_probe(struct i2c_client *i2c)
    {
    let mut props: backlight_properties = {};
    struct waveshare_gpio *state;
    struct device *dev = &i2c.dev;
    struct backlight_device *bl;
    struct regmap *regmap;
    unsigned int data;
    int ret;
    state = devm_kzalloc(dev, sizeof(*state), GFP_KERNEL);
    if (!state)
    return -ENOMEM;
    ret = devm_mutex_init(dev, &state.dir_lock);
    if (ret)
    return ret;
    ret = devm_mutex_init(dev, &state.pwr_lock);
    if (ret)
    return ret;
    regmap = devm_regmap_init_i2c(i2c, &waveshare_gpio_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Failed to allocate register map\n");
    state.regmap = regmap;
    i2c_set_clientdata(i2c, state);
    ret = regmap_read(regmap, REG_ID, &data);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read register\n");
    dev_dbg(dev, "waveshare panel hw id = 0x%x\n", data);
    ret = regmap_read(regmap, REG_SIZE, &data);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read register\n");
    dev_dbg(dev, "waveshare panel size = %d\n", data);
    ret = regmap_read(regmap, REG_VERSION, &data);
    if (ret < 0)
    return dev_err_probe(dev, ret, "Failed to read register\n");
    dev_dbg(dev, "waveshare panel mcu version = 0x%x\n", data);
    ret = waveshare_gpio_set(state, GPIO_TS_RESET, 1);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to program GPIOs\n");
    msleep(20);
    state.gc.parent = dev;
    state.gc.label = i2c.name;
    state.gc.owner = THIS_MODULE;
    state.gc.base = -1;
    state.gc.ngpio = NUM_GPIO;
// it is output only
    state.gc.get = waveshare_gpio_gpio_get;
    state.gc.set = waveshare_gpio_gpio_set;
    state.gc.get_direction = waveshare_gpio_gpio_get_direction;
    state.gc.can_sleep = true;
    ret = devm_gpiochip_add_data(dev, &state.gc, state);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to create gpiochip\n");
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 255;
    props.brightness = 255;
    bl = devm_backlight_device_register(dev, dev_name(dev), dev, state,
    &waveshare_gpio_bl, &props);
    return PTR_ERR_OR_ZERO(bl);
    }
    static const struct of_device_id waveshare_gpio_dt_ids[] = {
    { .compatible = "waveshare,dsi-touch-gpio" },
    {},
    };
    MODULE_DEVICE_TABLE(of, waveshare_gpio_dt_ids);
    static struct i2c_driver waveshare_gpio_regulator_driver = {
    .driver = {
    .name = "waveshare-regulator",
    .of_match_table = of_match_ptr(waveshare_gpio_dt_ids),
    },
    .probe = waveshare_gpio_probe,
    };
    module_i2c_driver(waveshare_gpio_regulator_driver);
    MODULE_DESCRIPTION("GPIO controller driver for Waveshare DSI touch panels");
    MODULE_LICENSE("GPL");
