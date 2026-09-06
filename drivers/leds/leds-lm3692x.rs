//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lm3692x.c
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
// TI LM3692x LED chip family driver
// Copyright (C) 2017-18 Texas Instruments Incorporated - https://www.ti.com

pub const LM36922_MODEL: c_int = 0;
pub const LM36923_MODEL: c_int = 1;
pub const LM3692X_REV: c_uint = 0x0;
pub const LM3692X_RESET: c_uint = 0x1;
pub const LM3692X_EN: c_uint = 0x10;
pub const LM3692X_BRT_CTRL: c_uint = 0x11;
pub const LM3692X_PWM_CTRL: c_uint = 0x12;
pub const LM3692X_BOOST_CTRL: c_uint = 0x13;
pub const LM3692X_AUTO_FREQ_HI: c_uint = 0x15;
pub const LM3692X_AUTO_FREQ_LO: c_uint = 0x16;
pub const LM3692X_BL_ADJ_THRESH: c_uint = 0x17;
pub const LM3692X_BRT_LSB: c_uint = 0x18;
pub const LM3692X_BRT_MSB: c_uint = 0x19;
pub const LM3692X_FAULT_CTRL: c_uint = 0x1e;
pub const LM3692X_FAULT_FLAGS: c_uint = 0x1f;

    LM3692X_LED2_EN | LM36923_LED3_EN)
// Brightness Control Bits

pub const LM3692X_RAMP_RATE_125us: c_uint = 0x00;

pub const LM3692X_BRHT_MODE_REG: c_uint = 0x00;

// PWM Register Bits

// Boost Control Bits

// Fault Control Bits

// Fault Flag Bits

//
// struct lm3692x_led
// @lock: Lock for reading/writing the device
// @client: Pointer to the I2C client
// @led_dev: LED class device pointer
// @regmap: Devices register map
// @enable_gpio: VDDIO/EN gpio to enable communication interface
// @regulator: LED supply regulator pointer
// @led_enable: LED sync to be enabled
// @model_id: Current device model ID enumerated
// @boost_ctrl: Cached configuration for the boost control register
// @brightness_ctrl: Cached configuration for brightness/brightness control
// @enabled: Cached enable state of the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3692x_led {
    pub lock: mutex,
    pub client: *mut i2c_client,
    pub led_dev: led_classdev,
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub regulator: *mut regulator,
    pub led_enable: c_int,
    pub model_id: c_int,
    pub brightness_ctrl: u8 boost_ctrl,,
    pub enabled: bool,
}

    static const struct reg_default lm3692x_reg_defs[] = {
    {LM3692X_EN, 0xf},
    {LM3692X_BRT_CTRL, 0x61},
    {LM3692X_PWM_CTRL, 0x73},
    {LM3692X_BOOST_CTRL, 0x6f},
    {LM3692X_AUTO_FREQ_HI, 0x0},
    {LM3692X_AUTO_FREQ_LO, 0x0},
    {LM3692X_BL_ADJ_THRESH, 0x0},
    {LM3692X_BRT_LSB, 0x7},
    {LM3692X_BRT_MSB, 0xff},
    {LM3692X_FAULT_CTRL, 0x7},
    };
    static const struct regmap_config lm3692x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = LM3692X_FAULT_FLAGS,
    .reg_defaults = lm3692x_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(lm3692x_reg_defs),
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn lm3692x_fault_check(led: *mut lm3692x_led) -> c_int {
    static int lm3692x_fault_check(struct lm3692x_led *led)
    {
    int ret;
    unsigned int read_buf;
    ret = regmap_read(led.regmap, LM3692X_FAULT_FLAGS, &read_buf);
    if (ret)
    return ret;
    if (read_buf)
    dev_err(&led.client.dev, "Detected a fault 0x%X\n", read_buf);
// The first read may clear the fault.  Check again to see if the fault
// still exits and return that value.
//
    regmap_read(led.regmap, LM3692X_FAULT_FLAGS, &read_buf);
    if (read_buf)
    dev_err(&led.client.dev, "Second read of fault flags 0x%X\n",
    read_buf);
    return read_buf;
    }
#[no_mangle]
unsafe extern "C" fn lm3692x_leds_enable(led: *mut lm3692x_led) -> c_int {
    static int lm3692x_leds_enable(struct lm3692x_led *led)
    {
    int enable_state;
    int ret, reg_ret;
    if (led.enabled)
    return 0;
    if (led.regulator) {
    ret = regulator_enable(led.regulator);
    if (ret) {
    dev_err(&led.client.dev,
    "Failed to enable regulator: %d\n", ret);
    return ret;
    }
    }
    if (led.enable_gpio)
    gpiod_direction_output(led.enable_gpio, 1);
    ret = lm3692x_fault_check(led);
    if (ret) {
    dev_err(&led.client.dev, "Cannot read/clear faults: %d\n",
    ret);
    goto out;
    }
    ret = regmap_write(led.regmap, LM3692X_BRT_CTRL, 0x00);
    if (ret)
    goto out;
//
// For glitch free operation, the following data should
// only be written while LEDx enable bits are 0 and the device enable
// bit is set to 1.
// per Section 7.5.14 of the data sheet
//
    ret = regmap_write(led.regmap, LM3692X_EN, LM3692X_DEVICE_EN);
    if (ret)
    goto out;
// Set the brightness to 0 so when enabled the LEDs do not come
// on with full brightness.
//
    ret = regmap_write(led.regmap, LM3692X_BRT_MSB, 0);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_BRT_LSB, 0);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_PWM_CTRL,
    LM3692X_PWM_FILTER_100 | LM3692X_PWM_SAMP_24MHZ);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_BOOST_CTRL, led.boost_ctrl);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_AUTO_FREQ_HI, 0x00);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_AUTO_FREQ_LO, 0x00);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_BL_ADJ_THRESH, 0x00);
    if (ret)
    goto out;
    ret = regmap_write(led.regmap, LM3692X_BRT_CTRL,
    LM3692X_BL_ADJ_POL | LM3692X_RAMP_EN);
    if (ret)
    goto out;
    switch (led.led_enable) {
    case 0:
    default:
    if (led.model_id == LM36923_MODEL)
    enable_state = LM3692X_LED1_EN | LM3692X_LED2_EN |
    LM36923_LED3_EN;
    else
    enable_state = LM3692X_LED1_EN | LM3692X_LED2_EN;
    break;
    case 1:
    enable_state = LM3692X_LED1_EN;
    break;
    case 2:
    enable_state = LM3692X_LED2_EN;
    break;
    case 3:
    if (led.model_id == LM36923_MODEL) {
    enable_state = LM36923_LED3_EN;
    break;
    }
    ret = -EINVAL;
    dev_err(&led.client.dev,
    "LED3 sync not available on this device\n");
    goto out;
    }
    ret = regmap_update_bits(led.regmap, LM3692X_EN, LM3692X_ENABLE_MASK,
    enable_state | LM3692X_DEVICE_EN);
    led.enabled = true;
    return ret;
    out:
    dev_err(&led.client.dev, "Fail writing initialization values\n");
    if (led.enable_gpio)
    gpiod_direction_output(led.enable_gpio, 0);
    if (led.regulator) {
    reg_ret = regulator_disable(led.regulator);
    if (reg_ret)
    dev_err(&led.client.dev,
    "Failed to disable regulator: %d\n", reg_ret);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lm3692x_leds_disable(led: *mut lm3692x_led) -> c_int {
    static int lm3692x_leds_disable(struct lm3692x_led *led)
    {
    int ret;
    if (!led.enabled)
    return 0;
    ret = regmap_update_bits(led.regmap, LM3692X_EN, LM3692X_DEVICE_EN, 0);
    if (ret) {
    dev_err(&led.client.dev, "Failed to disable regulator: %d\n",
    ret);
    return ret;
    }
    if (led.enable_gpio)
    gpiod_direction_output(led.enable_gpio, 0);
    if (led.regulator) {
    ret = regulator_disable(led.regulator);
    if (ret)
    dev_err(&led.client.dev,
    "Failed to disable regulator: %d\n", ret);
    }
    led.enabled = false;
    return ret;
    }
    static int lm3692x_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brt_val)
    {
    struct lm3692x_led *led =
    container_of(led_cdev, struct lm3692x_led, led_dev);
    int ret;
    let mut led_brightness_lsb: c_int = (brt_val >> 5);
    mutex_lock(&led.lock);
    if (brt_val == 0) {
    ret = lm3692x_leds_disable(led);
    goto out;
    } else {
    lm3692x_leds_enable(led);
    }
    ret = lm3692x_fault_check(led);
    if (ret) {
    dev_err(&led.client.dev, "Cannot read/clear faults: %d\n",
    ret);
    goto out;
    }
    ret = regmap_write(led.regmap, LM3692X_BRT_MSB, brt_val);
    if (ret) {
    dev_err(&led.client.dev, "Cannot write MSB: %d\n", ret);
    goto out;
    }
    ret = regmap_write(led.regmap, LM3692X_BRT_LSB, led_brightness_lsb);
    if (ret) {
    dev_err(&led.client.dev, "Cannot write LSB: %d\n", ret);
    goto out;
    }
    out:
    mutex_unlock(&led.lock);
    return ret;
    }
    static enum led_brightness lm3692x_max_brightness(struct lm3692x_led *led,
    u32 max_cur)
    {
    u32 max_code;
// see p.12 of LM36922 data sheet for brightness formula
    max_code = ((max_cur * 1000) - 37806) / 12195;
    if (max_code > 0x7FF)
    max_code = 0x7FF;
    return max_code >> 3;
    }
#[no_mangle]
unsafe extern "C" fn lm3692x_probe_dt(led: *mut lm3692x_led) -> c_int {
    static int lm3692x_probe_dt(struct lm3692x_led *led)
    {
    struct fwnode_handle *child = core::ptr::null_mut();
    let mut init_data: led_init_data = {};
    u32 ovp, max_cur;
    int ret;
    led.enable_gpio = devm_gpiod_get_optional(&led.client.dev,
    "enable", GPIOD_OUT_LOW);
    if (IS_ERR(led.enable_gpio)) {
    ret = PTR_ERR(led.enable_gpio);
    dev_err(&led.client.dev, "Failed to get enable gpio: %d\n",
    ret);
    return ret;
    }
    led.regulator = devm_regulator_get_optional(&led.client.dev, "vled");
    if (IS_ERR(led.regulator)) {
    ret = PTR_ERR(led.regulator);
    if (ret != -ENODEV)
    return dev_err_probe(&led.client.dev, ret,
    "Failed to get vled regulator\n");
    led.regulator = core::ptr::null_mut();
    }
    led.boost_ctrl = LM3692X_BOOST_SW_1MHZ |
    LM3692X_BOOST_SW_NO_SHIFT |
    LM3692X_OCP_PROT_1_5A;
    ret = device_property_read_u32(&led.client.dev,
    "ti,ovp-microvolt", &ovp);
    if (ret) {
    led.boost_ctrl |= LM3692X_OVP_29V;
    } else {
    switch (ovp) {
    case 17000000:
    break;
    case 21000000:
    led.boost_ctrl |= LM3692X_OVP_21V;
    break;
    case 25000000:
    led.boost_ctrl |= LM3692X_OVP_25V;
    break;
    case 29000000:
    led.boost_ctrl |= LM3692X_OVP_29V;
    break;
    default:
    dev_err(&led.client.dev, "Invalid OVP %d\n", ovp);
    return -EINVAL;
    }
    }
    child = device_get_next_child_node(&led.client.dev, child);
    if (!child) {
    dev_err(&led.client.dev, "No LED Child node\n");
    return -ENODEV;
    }
    ret = fwnode_property_read_u32(child, "reg", &led.led_enable);
    if (ret) {
    fwnode_handle_put(child);
    dev_err(&led.client.dev, "reg DT property missing\n");
    return ret;
    }
    ret = fwnode_property_read_u32(child, "led-max-microamp", &max_cur);
    led.led_dev.max_brightness = ret ? LED_FULL :
    lm3692x_max_brightness(led, max_cur);
    init_data.fwnode = child;
    init_data.devicename = led.client.name;
    init_data.default_label = ":";
    ret = devm_led_classdev_register_ext(&led.client.dev, &led.led_dev,
    &init_data);
    if (ret)
    dev_err(&led.client.dev, "led register err: %d\n", ret);
    fwnode_handle_put(init_data.fwnode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lm3692x_probe(client: *mut i2c_client) -> c_int {
    static int lm3692x_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct lm3692x_led *led;
    int ret;
    led = devm_kzalloc(&client.dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    mutex_init(&led.lock);
    led.client = client;
    led.led_dev.brightness_set_blocking = lm3692x_brightness_set;
    led.model_id = id.driver_data;
    i2c_set_clientdata(client, led);
    led.regmap = devm_regmap_init_i2c(client, &lm3692x_regmap_config);
    if (IS_ERR(led.regmap)) {
    ret = PTR_ERR(led.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    ret = lm3692x_probe_dt(led);
    if (ret)
    return ret;
    ret = lm3692x_leds_enable(led);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3692x_remove(client: *mut i2c_client) {
    static void lm3692x_remove(struct i2c_client *client)
    {
    struct lm3692x_led *led = i2c_get_clientdata(client);
    lm3692x_leds_disable(led);
    mutex_destroy(&led.lock);
    }
    static const struct i2c_device_id lm3692x_id[] = {
    { .name = "lm36922", .driver_data = LM36922_MODEL },
    { .name = "lm36923", .driver_data = LM36923_MODEL },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm3692x_id);
    static const struct of_device_id of_lm3692x_leds_match[] = {
    { .compatible = "ti,lm36922", },
    { .compatible = "ti,lm36923", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lm3692x_leds_match);
    static struct i2c_driver lm3692x_driver = {
    .driver = {
    .name	= "lm3692x",
    .of_match_table = of_lm3692x_leds_match,
    },
    .probe		= lm3692x_probe,
    .remove		= lm3692x_remove,
    .id_table	= lm3692x_id,
    };
    module_i2c_driver(lm3692x_driver);
    MODULE_DESCRIPTION("Texas Instruments LM3692X LED driver");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
    MODULE_LICENSE("GPL v2");
