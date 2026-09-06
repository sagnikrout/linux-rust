//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lp8860.c
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
// TI LP8860 4-Channel LED Driver
//
// Copyright (C) 2014 Texas Instruments
//
// Author: Dan Murphy <dmurphy@ti.com>
//

pub const LP8860_DISP_CL1_BRT_MSB: c_uint = 0x00;
pub const LP8860_DISP_CL1_BRT_LSB: c_uint = 0x01;
pub const LP8860_DISP_CL1_CURR_MSB: c_uint = 0x02;
pub const LP8860_DISP_CL1_CURR_LSB: c_uint = 0x03;
pub const LP8860_CL2_BRT_MSB: c_uint = 0x04;
pub const LP8860_CL2_BRT_LSB: c_uint = 0x05;
pub const LP8860_CL2_CURRENT: c_uint = 0x06;
pub const LP8860_CL3_BRT_MSB: c_uint = 0x07;
pub const LP8860_CL3_BRT_LSB: c_uint = 0x08;
pub const LP8860_CL3_CURRENT: c_uint = 0x09;
pub const LP8860_CL4_BRT_MSB: c_uint = 0x0a;
pub const LP8860_CL4_BRT_LSB: c_uint = 0x0b;
pub const LP8860_CL4_CURRENT: c_uint = 0x0c;
pub const LP8860_CONFIG: c_uint = 0x0d;
pub const LP8860_STATUS: c_uint = 0x0e;
pub const LP8860_FAULT: c_uint = 0x0f;
pub const LP8860_LED_FAULT: c_uint = 0x10;
pub const LP8860_FAULT_CLEAR: c_uint = 0x11;
pub const LP8860_ID: c_uint = 0x12;
pub const LP8860_TEMP_MSB: c_uint = 0x13;
pub const LP8860_TEMP_LSB: c_uint = 0x14;
pub const LP8860_DISP_LED_CURR_MSB: c_uint = 0x15;
pub const LP8860_DISP_LED_CURR_LSB: c_uint = 0x16;
pub const LP8860_DISP_LED_PWM_MSB: c_uint = 0x17;
pub const LP8860_DISP_LED_PWM_LSB: c_uint = 0x18;
pub const LP8860_EEPROM_CNTRL: c_uint = 0x19;
pub const LP8860_EEPROM_UNLOCK: c_uint = 0x1a;
pub const LP8860_EEPROM_REG_0: c_uint = 0x60;
pub const LP8860_EEPROM_REG_1: c_uint = 0x61;
pub const LP8860_EEPROM_REG_2: c_uint = 0x62;
pub const LP8860_EEPROM_REG_3: c_uint = 0x63;
pub const LP8860_EEPROM_REG_4: c_uint = 0x64;
pub const LP8860_EEPROM_REG_5: c_uint = 0x65;
pub const LP8860_EEPROM_REG_6: c_uint = 0x66;
pub const LP8860_EEPROM_REG_7: c_uint = 0x67;
pub const LP8860_EEPROM_REG_8: c_uint = 0x68;
pub const LP8860_EEPROM_REG_9: c_uint = 0x69;
pub const LP8860_EEPROM_REG_10: c_uint = 0x6a;
pub const LP8860_EEPROM_REG_11: c_uint = 0x6b;
pub const LP8860_EEPROM_REG_12: c_uint = 0x6c;
pub const LP8860_EEPROM_REG_13: c_uint = 0x6d;
pub const LP8860_EEPROM_REG_14: c_uint = 0x6e;
pub const LP8860_EEPROM_REG_15: c_uint = 0x6f;
pub const LP8860_EEPROM_REG_16: c_uint = 0x70;
pub const LP8860_EEPROM_REG_17: c_uint = 0x71;
pub const LP8860_EEPROM_REG_18: c_uint = 0x72;
pub const LP8860_EEPROM_REG_19: c_uint = 0x73;
pub const LP8860_EEPROM_REG_20: c_uint = 0x74;
pub const LP8860_EEPROM_REG_21: c_uint = 0x75;
pub const LP8860_EEPROM_REG_22: c_uint = 0x76;
pub const LP8860_EEPROM_REG_23: c_uint = 0x77;
pub const LP8860_EEPROM_REG_24: c_uint = 0x78;
pub const LP8860_LOCK_EEPROM: c_uint = 0x00;
pub const LP8860_UNLOCK_EEPROM: c_uint = 0x01;
pub const LP8860_PROGRAM_EEPROM: c_uint = 0x02;
pub const LP8860_EEPROM_CODE_1: c_uint = 0x08;
pub const LP8860_EEPROM_CODE_2: c_uint = 0xba;
pub const LP8860_EEPROM_CODE_3: c_uint = 0xef;
pub const LP8860_CLEAR_FAULTS: c_uint = 0x01;

//
// struct lp8860_led
// @lock: Lock for reading/writing the device
// @client: Pointer to the I2C client
// @led_dev: led class device pointer
// @regmap: Devices register map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8860_led {
    pub lock: mutex,
    pub client: *mut i2c_client,
    pub led_dev: led_classdev,
    pub regmap: *mut regmap,
}

    static bool program_eeprom;
    module_param(program_eeprom, bool, 0644);
    MODULE_PARM_DESC(program_eeprom, "Program the configuration EEPROM on device startup");
//
// EEPROM bits are intended to be set/programmed before normal operation only
// once during silicon production, but can be reprogrammed for evaluation purposes
// up to 1000 cycles. To program this EEPROM using this driver, update the below
// table and set the module param "program_eeprom" to 1
//
    static const struct reg_sequence lp8860_eeprom_disp_regs[] = {
    { LP8860_EEPROM_REG_0, 0xed },
    { LP8860_EEPROM_REG_1, 0xdf },
    { LP8860_EEPROM_REG_2, 0xdc },
    { LP8860_EEPROM_REG_3, 0xf0 },
    { LP8860_EEPROM_REG_4, 0xdf },
    { LP8860_EEPROM_REG_5, 0xe5 },
    { LP8860_EEPROM_REG_6, 0xf2 },
    { LP8860_EEPROM_REG_7, 0x77 },
    { LP8860_EEPROM_REG_8, 0x77 },
    { LP8860_EEPROM_REG_9, 0x71 },
    { LP8860_EEPROM_REG_10, 0x3f },
    { LP8860_EEPROM_REG_11, 0xb7 },
    { LP8860_EEPROM_REG_12, 0x17 },
    { LP8860_EEPROM_REG_13, 0xef },
    { LP8860_EEPROM_REG_14, 0xb0 },
    { LP8860_EEPROM_REG_15, 0x87 },
    { LP8860_EEPROM_REG_16, 0xce },
    { LP8860_EEPROM_REG_17, 0x72 },
    { LP8860_EEPROM_REG_18, 0xe5 },
    { LP8860_EEPROM_REG_19, 0xdf },
    { LP8860_EEPROM_REG_20, 0x35 },
    { LP8860_EEPROM_REG_21, 0x06 },
    { LP8860_EEPROM_REG_22, 0xdc },
    { LP8860_EEPROM_REG_23, 0x88 },
    { LP8860_EEPROM_REG_24, 0x3E },
    };
#[no_mangle]
unsafe extern "C" fn lp8860_fault_check(led: *mut lp8860_led) -> c_int {
    static int lp8860_fault_check(struct lp8860_led *led)
    {
    int ret, fault;
    unsigned int read_buf;
    ret = regmap_read(led.regmap, LP8860_LED_FAULT, &read_buf);
    if (ret)
    goto out;
    fault = read_buf;
    ret = regmap_read(led.regmap, LP8860_FAULT, &read_buf);
    if (ret)
    goto out;
    fault |= read_buf;
// Attempt to clear any faults
    if (fault)
    ret = regmap_write(led.regmap, LP8860_FAULT_CLEAR,
    LP8860_CLEAR_FAULTS);
    out:
    return ret;
    }
    static int lp8860_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brt_val)
    {
    struct lp8860_led *led =
    container_of(led_cdev, struct lp8860_led, led_dev);
    let mut disp_brightness: c_int = brt_val * 255;
    int ret;
    guard(mutex)(&led.lock);
    ret = lp8860_fault_check(led);
    if (ret) {
    dev_err(&led.client.dev, "Cannot read/clear faults\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_DISP_CL1_BRT_MSB,
    (disp_brightness & 0xff00) >> 8);
    if (ret) {
    dev_err(&led.client.dev, "Cannot write CL1 MSB\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_DISP_CL1_BRT_LSB,
    disp_brightness & 0xff);
    if (ret) {
    dev_err(&led.client.dev, "Cannot write CL1 LSB\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp8860_program_eeprom(led: *mut lp8860_led) -> c_int {
    static int lp8860_program_eeprom(struct lp8860_led *led)
    {
    int ret, reg_count;
    guard(mutex)(&led.lock);
    ret = lp8860_fault_check(led);
    if (ret) {
    dev_err(&led.client.dev, "Cannot read/clear faults\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_EEPROM_UNLOCK, LP8860_EEPROM_CODE_1);
    if (ret) {
    dev_err(&led.client.dev, "EEPROM Unlock failed\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_EEPROM_UNLOCK, LP8860_EEPROM_CODE_2);
    if (ret) {
    dev_err(&led.client.dev, "EEPROM Unlock failed\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_EEPROM_UNLOCK, LP8860_EEPROM_CODE_3);
    if (ret) {
    dev_err(&led.client.dev, "EEPROM Unlock failed\n");
    return ret;
    }
    reg_count = ARRAY_SIZE(lp8860_eeprom_disp_regs);
    ret = regmap_multi_reg_write(led.regmap, lp8860_eeprom_disp_regs, reg_count);
    if (ret) {
    dev_err(&led.client.dev, "Failed writing EEPROM\n");
    return ret;
    }
    ret = regmap_write(led.regmap, LP8860_EEPROM_UNLOCK, LP8860_LOCK_EEPROM);
    if (ret)
    return ret;
    ret = regmap_write(led.regmap,
    LP8860_EEPROM_CNTRL,
    LP8860_PROGRAM_EEPROM);
    if (ret) {
    dev_err(&led.client.dev, "Failed programming EEPROM\n");
    return ret;
    }
    return 0;
    }
    static const struct regmap_range lp8860_reg_ranges[] = {
    regmap_reg_range(LP8860_DISP_CL1_BRT_MSB, LP8860_EEPROM_UNLOCK),
    regmap_reg_range(LP8860_EEPROM_REG_0, LP8860_EEPROM_REG_24),
    };
    static const struct regmap_access_table lp8860_reg_table = {
    .yes_ranges = lp8860_reg_ranges,
    .n_yes_ranges = ARRAY_SIZE(lp8860_reg_ranges),
    };
    static const struct regmap_config lp8860_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &lp8860_reg_table,
    .wr_table = &lp8860_reg_table,
    .max_register = LP8860_EEPROM_REG_24,
    };
#[no_mangle]
unsafe extern "C" fn lp8860_disable_gpio(data: *mut c_void) {
    static void lp8860_disable_gpio(void *data)
    {
    struct gpio_desc *gpio = data;
    gpiod_set_value(gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn lp8860_probe(client: *mut i2c_client) -> c_int {
    static int lp8860_probe(struct i2c_client *client)
    {
    int ret;
    struct lp8860_led *led;
    struct device_node *np = dev_of_node(&client.dev);
    struct device_node *child_node;
    let mut init_data: led_init_data = {};
    struct gpio_desc *enable_gpio;
    led = devm_kzalloc(&client.dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    child_node = of_get_next_available_child(np, core::ptr::null_mut());
    if (!child_node)
    return -EINVAL;
    enable_gpio = devm_gpiod_get_optional(&client.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(enable_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(enable_gpio),
    "Failed to get enable GPIO\n");
    devm_add_action_or_reset(&client.dev, lp8860_disable_gpio, enable_gpio);
    ret = devm_regulator_get_enable_optional(&client.dev, "vled");
    if (ret && ret != -ENODEV)
    return dev_err_probe(&client.dev, ret,
    "Failed to enable vled regulator\n");
    led.client = client;
    led.led_dev.brightness_set_blocking = lp8860_brightness_set;
    ret = devm_mutex_init(&client.dev, &led.lock);
    if (ret)
    return dev_err_probe(&client.dev, ret, "Failed to initialize lock\n");
    led.regmap = devm_regmap_init_i2c(client, &lp8860_regmap_config);
    if (IS_ERR(led.regmap)) {
    ret = PTR_ERR(led.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    if (program_eeprom) {
    ret = lp8860_program_eeprom(led);
    if (ret)
    return ret;
    }
    init_data.fwnode = of_fwnode_handle(child_node);
    init_data.devicename = LP8860_NAME;
    init_data.default_label = ":display_cluster";
    ret = devm_led_classdev_register_ext(&client.dev, &led.led_dev,
    &init_data);
    if (ret) {
    dev_err(&client.dev, "led register err: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct i2c_device_id lp8860_id[] = {
    { .name = "lp8860" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp8860_id);
    static const struct of_device_id of_lp8860_leds_match[] = {
    { .compatible = "ti,lp8860", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_lp8860_leds_match);
    static struct i2c_driver lp8860_driver = {
    .driver = {
    .name	= "lp8860",
    .of_match_table = of_lp8860_leds_match,
    },
    .probe		= lp8860_probe,
    .id_table	= lp8860_id,
    };
    module_i2c_driver(lp8860_driver);
    MODULE_DESCRIPTION("Texas Instruments LP8860 LED driver");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
    MODULE_LICENSE("GPL v2");
