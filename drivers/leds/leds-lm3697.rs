//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lm3697.c
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
// TI LM3697 LED chip family driver
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com

pub const LM3697_REV: c_uint = 0x0;
pub const LM3697_RESET: c_uint = 0x1;
pub const LM3697_OUTPUT_CONFIG: c_uint = 0x10;
pub const LM3697_CTRL_A_RAMP: c_uint = 0x11;
pub const LM3697_CTRL_B_RAMP: c_uint = 0x12;
pub const LM3697_CTRL_A_B_RT_RAMP: c_uint = 0x13;
pub const LM3697_CTRL_A_B_RAMP_CFG: c_uint = 0x14;
pub const LM3697_CTRL_A_B_BRT_CFG: c_uint = 0x16;
pub const LM3697_CTRL_A_FS_CURR_CFG: c_uint = 0x17;
pub const LM3697_CTRL_B_FS_CURR_CFG: c_uint = 0x18;
pub const LM3697_PWM_CFG: c_uint = 0x1c;
pub const LM3697_CTRL_A_BRT_LSB: c_uint = 0x20;
pub const LM3697_CTRL_A_BRT_MSB: c_uint = 0x21;
pub const LM3697_CTRL_B_BRT_LSB: c_uint = 0x22;
pub const LM3697_CTRL_B_BRT_MSB: c_uint = 0x23;
pub const LM3697_CTRL_ENABLE: c_uint = 0x24;

pub const LM3697_MAX_LED_STRINGS: c_int = 3;
pub const LM3697_CONTROL_A: c_int = 0;
pub const LM3697_CONTROL_B: c_int = 1;
pub const LM3697_MAX_CONTROL_BANKS: c_int = 2;
//
// struct lm3697_led -
// @hvled_strings: Array of LED strings associated with a control bank
// @label: LED label
// @led_dev: LED class device
// @priv: Pointer to the device struct
// @lmu_data: Register and setting values for common code
// @control_bank: Control bank the LED is associated to. 0 is control bank A
// 1 is control bank B
// @enabled: LED brightness level (or LED_OFF)
// @num_leds: Number of LEDs available
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3697_led {
    pub hvled_strings: [u32; LM3697_MAX_LED_STRINGS],
    pub label: [c_char; LED_MAX_NAME_SIZE],
    pub led_dev: led_classdev,
    pub priv: *mut lm3697,
    pub lmu_data: ti_lmu_bank,
    pub control_bank: c_int,
    pub enabled: c_int,
    pub num_leds: c_int,
}

//
// struct lm3697 -
// @enable_gpio: Hardware enable gpio
// @regulator: LED supply regulator pointer
// @client: Pointer to the I2C client
// @regmap: Devices register map
// @dev: Pointer to the devices device struct
// @lock: Lock for reading/writing the device
// @leds: Array of LED strings
// @bank_cfg: OUTPUT_CONFIG register values
// @num_banks: Number of control banks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3697 {
    pub enable_gpio: *mut gpio_desc,
    pub regulator: *mut regulator,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub lock: mutex,
    pub bank_cfg: c_int,
    pub num_banks: c_int,
    pub __counted_by(num_banks): lm3697_led leds[],
}

    static const struct reg_default lm3697_reg_defs[] = {
    {LM3697_OUTPUT_CONFIG, 0x6},
    {LM3697_CTRL_A_RAMP, 0x0},
    {LM3697_CTRL_B_RAMP, 0x0},
    {LM3697_CTRL_A_B_RT_RAMP, 0x0},
    {LM3697_CTRL_A_B_RAMP_CFG, 0x0},
    {LM3697_CTRL_A_B_BRT_CFG, 0x0},
    {LM3697_CTRL_A_FS_CURR_CFG, 0x13},
    {LM3697_CTRL_B_FS_CURR_CFG, 0x13},
    {LM3697_PWM_CFG, 0xc},
    {LM3697_CTRL_A_BRT_LSB, 0x0},
    {LM3697_CTRL_A_BRT_MSB, 0x0},
    {LM3697_CTRL_B_BRT_LSB, 0x0},
    {LM3697_CTRL_B_BRT_MSB, 0x0},
    {LM3697_CTRL_ENABLE, 0x0},
    };
    static const struct regmap_config lm3697_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = LM3697_CTRL_ENABLE,
    .reg_defaults = lm3697_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(lm3697_reg_defs),
    .cache_type = REGCACHE_FLAT,
    };
    static int lm3697_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brt_val)
    {
    struct lm3697_led *led = container_of(led_cdev, struct lm3697_led,
    led_dev);
    let mut ctrl_en_val: c_int = (1 << led.control_bank);
    struct device *dev = led.priv.dev;
    int ret;
    mutex_lock(&led.priv.lock);
    if (brt_val == LED_OFF) {
    ret = regmap_update_bits(led.priv.regmap, LM3697_CTRL_ENABLE,
    ctrl_en_val, ~ctrl_en_val);
    if (ret) {
    dev_err(dev, "Cannot write ctrl register\n");
    goto brightness_out;
    }
    led.enabled = LED_OFF;
    } else {
    ret = ti_lmu_common_set_brightness(&led.lmu_data, brt_val);
    if (ret) {
    dev_err(dev, "Cannot write brightness\n");
    goto brightness_out;
    }
    if (!led.enabled) {
    ret = regmap_update_bits(led.priv.regmap,
    LM3697_CTRL_ENABLE,
    ctrl_en_val, ctrl_en_val);
    if (ret) {
    dev_err(dev, "Cannot enable the device\n");
    goto brightness_out;
    }
    led.enabled = brt_val;
    }
    }
    brightness_out:
    mutex_unlock(&led.priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lm3697_init(priv: *mut lm3697) -> c_int {
    static int lm3697_init(struct lm3697 *priv)
    {
    struct device *dev = priv.dev;
    struct lm3697_led *led;
    int i, ret;
    if (priv.enable_gpio) {
    gpiod_direction_output(priv.enable_gpio, 1);
    } else {
    ret = regmap_write(priv.regmap, LM3697_RESET, LM3697_SW_RESET);
    if (ret) {
    dev_err(dev, "Cannot reset the device\n");
    goto out;
    }
    }
    ret = regmap_write(priv.regmap, LM3697_CTRL_ENABLE, 0x0);
    if (ret) {
    dev_err(dev, "Cannot write ctrl enable\n");
    goto out;
    }
    ret = regmap_write(priv.regmap, LM3697_OUTPUT_CONFIG, priv.bank_cfg);
    if (ret)
    dev_err(dev, "Cannot write OUTPUT config\n");
    for (i = 0; i < priv.num_banks; i++) {
    led = &priv.leds[i];
    ret = ti_lmu_common_set_ramp(&led.lmu_data);
    if (ret)
    dev_err(dev, "Setting the ramp rate failed\n");
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lm3697_probe_dt(priv: *mut lm3697) -> c_int {
    static int lm3697_probe_dt(struct lm3697 *priv)
    {
    struct device *dev = priv.dev;
    struct lm3697_led *led;
    let mut ret: c_int = -EINVAL;
    int control_bank;
    let mut i: usize = 0;
    int j;
    priv.enable_gpio = devm_gpiod_get_optional(dev, "enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.enable_gpio))
    return dev_err_probe(dev, PTR_ERR(priv.enable_gpio),
    "Failed to get enable GPIO\n");
    priv.regulator = devm_regulator_get(dev, "vled");
    if (IS_ERR(priv.regulator))
    priv.regulator = core::ptr::null_mut();
    device_for_each_child_node_scoped(dev, child) {
    let mut init_data: led_init_data = {};
    ret = fwnode_property_read_u32(child, "reg", &control_bank);
    if (ret) {
    dev_err(dev, "reg property missing\n");
    return ret;
    }
    if (control_bank > LM3697_CONTROL_B) {
    dev_err(dev, "reg property is invalid\n");
    return -EINVAL;
    }
    led = &priv.leds[i];
    ret = ti_lmu_common_get_brt_res(dev, child, &led.lmu_data);
    if (ret)
    dev_warn(dev,
    "brightness resolution property missing\n");
    led.control_bank = control_bank;
    led.lmu_data.regmap = priv.regmap;
    led.lmu_data.runtime_ramp_reg = LM3697_CTRL_A_RAMP +
    control_bank;
    led.lmu_data.msb_brightness_reg = LM3697_CTRL_A_BRT_MSB +
    led.control_bank * 2;
    led.lmu_data.lsb_brightness_reg = LM3697_CTRL_A_BRT_LSB +
    led.control_bank * 2;
    led.num_leds = fwnode_property_count_u32(child, "led-sources");
    if (led.num_leds > LM3697_MAX_LED_STRINGS) {
    dev_err(dev, "Too many LED strings defined\n");
    continue;
    }
    ret = fwnode_property_read_u32_array(child, "led-sources",
    led.hvled_strings,
    led.num_leds);
    if (ret) {
    dev_err(dev, "led-sources property missing\n");
    return ret;
    }
    for (j = 0; j < led.num_leds; j++)
    priv.bank_cfg |=
    (led.control_bank << led.hvled_strings[j]);
    ret = ti_lmu_common_get_ramp_params(dev, child, &led.lmu_data);
    if (ret)
    dev_warn(dev, "runtime-ramp properties missing\n");
    init_data.fwnode = child;
    init_data.devicename = priv.client.name;
// for backwards compatibility if `label` is not present
    init_data.default_label = ":";
    led.priv = priv;
    led.led_dev.max_brightness = led.lmu_data.max_brightness;
    led.led_dev.brightness_set_blocking = lm3697_brightness_set;
    ret = devm_led_classdev_register_ext(dev, &led.led_dev,
    &init_data);
    if (ret) {
    dev_err(dev, "led register err: %d\n", ret);
    return ret;
    }
    i++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3697_probe(client: *mut i2c_client) -> c_int {
    static int lm3697_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct lm3697 *led;
    int count;
    int ret;
    count = device_get_child_node_count(dev);
    if (!count || count > LM3697_MAX_CONTROL_BANKS) {
    dev_err(dev, "Strange device tree!");
    return -ENODEV;
    }
    led = devm_kzalloc(dev, struct_size(led, leds, count), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    mutex_init(&led.lock);
    i2c_set_clientdata(client, led);
    led.client = client;
    led.dev = dev;
    led.num_banks = count;
    led.regmap = devm_regmap_init_i2c(client, &lm3697_regmap_config);
    if (IS_ERR(led.regmap)) {
    ret = PTR_ERR(led.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    ret = lm3697_probe_dt(led);
    if (ret)
    return ret;
    return lm3697_init(led);
    }
#[no_mangle]
unsafe extern "C" fn lm3697_remove(client: *mut i2c_client) {
    static void lm3697_remove(struct i2c_client *client)
    {
    struct lm3697 *led = i2c_get_clientdata(client);
    struct device *dev = &led.client.dev;
    int ret;
    ret = regmap_update_bits(led.regmap, LM3697_CTRL_ENABLE,
    LM3697_CTRL_A_B_EN, 0);
    if (ret)
    dev_err(dev, "Failed to disable the device\n");
    if (led.enable_gpio)
    gpiod_direction_output(led.enable_gpio, 0);
    if (led.regulator) {
    ret = regulator_disable(led.regulator);
    if (ret)
    dev_err(dev, "Failed to disable regulator\n");
    }
    mutex_destroy(&led.lock);
    }
    static const struct i2c_device_id lm3697_id[] = {
    { .name = "lm3697" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm3697_id);
    static const struct of_device_id of_lm3697_leds_match[] = {
    { .compatible = "ti,lm3697", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lm3697_leds_match);
    static struct i2c_driver lm3697_driver = {
    .driver = {
    .name	= "lm3697",
    .of_match_table = of_lm3697_leds_match,
    },
    .probe		= lm3697_probe,
    .remove		= lm3697_remove,
    .id_table	= lm3697_id,
    };
    module_i2c_driver(lm3697_driver);
    MODULE_DESCRIPTION("Texas Instruments LM3697 LED driver");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
    MODULE_LICENSE("GPL v2");
