//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/tm2-touchkey.c
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
// TM2 touchkey device driver
//
// Copyright 2005 Phil Blundell
// Copyright 2016 Samsung Electronics Co., Ltd.
//
// Author: Beomho Seo <beomho.seo@samsung.com>
// Author: Jaechul Lee <jcsing.lee@samsung.com>
//

pub const ARIES_TOUCHKEY_CMD_LED_ON: c_uint = 0x1;
pub const ARIES_TOUCHKEY_CMD_LED_OFF: c_uint = 0x2;
pub const TM2_TOUCHKEY_CMD_LED_ON: c_uint = 0x10;
pub const TM2_TOUCHKEY_CMD_LED_OFF: c_uint = 0x20;

pub const TM2_TOUCHKEY_LED_VOLTAGE_MIN: c_int = 2500000;
pub const TM2_TOUCHKEY_LED_VOLTAGE_MAX: c_int = 3300000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct touchkey_variant {
    pub keycode_reg: u8,
    pub base_reg: u8,
    pub cmd_led_on: u8,
    pub cmd_led_off: u8,
    pub no_reg: bool,
    pub fixed_regulator: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm2_touchkey_data {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
    pub led_dev: led_classdev,
    pub vdd: *mut regulator,
    pub regulators: [regulator_bulk_data; 3],
    pub variant: *const touchkey_variant,
    pub keycodes: [u32; 4],
    pub num_keycodes: c_int,
}

    static const struct touchkey_variant tm2_touchkey_variant = {
    .keycode_reg = 0x03,
    .base_reg = 0x00,
    .cmd_led_on = TM2_TOUCHKEY_CMD_LED_ON,
    .cmd_led_off = TM2_TOUCHKEY_CMD_LED_OFF,
    };
    static const struct touchkey_variant midas_touchkey_variant = {
    .keycode_reg = 0x00,
    .base_reg = 0x00,
    .cmd_led_on = TM2_TOUCHKEY_CMD_LED_ON,
    .cmd_led_off = TM2_TOUCHKEY_CMD_LED_OFF,
    };
    static struct touchkey_variant aries_touchkey_variant = {
    .no_reg = true,
    .fixed_regulator = true,
    .cmd_led_on = ARIES_TOUCHKEY_CMD_LED_ON,
    .cmd_led_off = ARIES_TOUCHKEY_CMD_LED_OFF,
    };
    static const struct touchkey_variant tc360_touchkey_variant = {
    .keycode_reg = 0x00,
    .base_reg = 0x00,
    .fixed_regulator = true,
    .cmd_led_on = TM2_TOUCHKEY_CMD_LED_ON,
    .cmd_led_off = TM2_TOUCHKEY_CMD_LED_OFF,
    };
    static int tm2_touchkey_led_brightness_set(struct led_classdev *led_dev,
    enum led_brightness brightness)
    {
    struct tm2_touchkey_data *touchkey =
    container_of(led_dev, struct tm2_touchkey_data, led_dev);
    u32 volt;
    u8 data;
    if (brightness == LED_OFF) {
    volt = TM2_TOUCHKEY_LED_VOLTAGE_MIN;
    data = touchkey.variant.cmd_led_off;
    } else {
    volt = TM2_TOUCHKEY_LED_VOLTAGE_MAX;
    data = touchkey.variant.cmd_led_on;
    }
    if (!touchkey.variant.fixed_regulator)
    regulator_set_voltage(touchkey.vdd, volt, volt);
    return touchkey.variant.no_reg ?
    i2c_smbus_write_byte(touchkey.client, data) :
    i2c_smbus_write_byte_data(touchkey.client,
    touchkey.variant.base_reg, data);
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_power_enable(touchkey: *mut tm2_touchkey_data) -> c_int {
    static int tm2_touchkey_power_enable(struct tm2_touchkey_data *touchkey)
    {
    int error;
    error = regulator_bulk_enable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error)
    return error;
// waiting for device initialization, at least 150ms
    msleep(150);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_power_disable(data: *mut c_void) {
    static void tm2_touchkey_power_disable(void *data)
    {
    struct tm2_touchkey_data *touchkey = data;
    regulator_bulk_disable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_irq_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t tm2_touchkey_irq_handler(int irq, void *devid)
    {
    struct tm2_touchkey_data *touchkey = devid;
    int data;
    int index;
    int i;
    if (touchkey.variant.no_reg)
    data = i2c_smbus_read_byte(touchkey.client);
    else
    data = i2c_smbus_read_byte_data(touchkey.client,
    touchkey.variant.keycode_reg);
    if (data < 0) {
    dev_err(&touchkey.client.dev,
    "failed to read i2c data: %d\n", data);
    goto out;
    }
    index = (data & TM2_TOUCHKEY_BIT_KEYCODE) - 1;
    if (index < 0 || index >= touchkey.num_keycodes) {
    dev_warn(&touchkey.client.dev,
    "invalid keycode index %d\n", index);
    goto out;
    }
    input_event(touchkey.input_dev, EV_MSC, MSC_SCAN, index);
    if (data & TM2_TOUCHKEY_BIT_PRESS_EV) {
    for (i = 0; i < touchkey.num_keycodes; i++)
    input_report_key(touchkey.input_dev,
    touchkey.keycodes[i], 0);
    } else {
    input_report_key(touchkey.input_dev,
    touchkey.keycodes[index], 1);
    }
    input_sync(touchkey.input_dev);
    out:
    if (touchkey.variant.fixed_regulator &&
    data & TM2_TOUCHKEY_BIT_PRESS_EV) {
// touch turns backlight on, so make sure we're in sync
    if (touchkey.led_dev.brightness == LED_OFF)
    tm2_touchkey_led_brightness_set(&touchkey.led_dev,
    LED_OFF);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_probe(client: *mut i2c_client) -> c_int {
    static int tm2_touchkey_probe(struct i2c_client *client)
    {
    struct device_node *np = client.dev.of_node;
    struct tm2_touchkey_data *touchkey;
    int error;
    int i;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&client.dev, "incompatible I2C adapter\n");
    return -EIO;
    }
    touchkey = devm_kzalloc(&client.dev, sizeof(*touchkey), GFP_KERNEL);
    if (!touchkey)
    return -ENOMEM;
    touchkey.client = client;
    i2c_set_clientdata(client, touchkey);
    touchkey.variant = of_device_get_match_data(&client.dev);
    touchkey.regulators[0].supply = "vcc";
    touchkey.regulators[1].supply = "vdd";
    touchkey.regulators[2].supply = "vddio";
    error = devm_regulator_bulk_get(&client.dev,
    ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error) {
    dev_err(&client.dev, "failed to get regulators: %d\n", error);
    return error;
    }
// Save VDD for easy access
    touchkey.vdd = touchkey.regulators[1].consumer;
    touchkey.num_keycodes = of_property_read_variable_u32_array(np,
    "linux,keycodes", touchkey.keycodes, 0,
    ARRAY_SIZE(touchkey.keycodes));
    if (touchkey.num_keycodes <= 0) {
// default keycodes
    touchkey.keycodes[0] = KEY_PHONE;
    touchkey.keycodes[1] = KEY_BACK;
    touchkey.num_keycodes = 2;
    }
    error = tm2_touchkey_power_enable(touchkey);
    if (error) {
    dev_err(&client.dev, "failed to power up device: %d\n", error);
    return error;
    }
    error = devm_add_action_or_reset(&client.dev,
    tm2_touchkey_power_disable, touchkey);
    if (error) {
    dev_err(&client.dev,
    "failed to install poweroff handler: %d\n", error);
    return error;
    }
// input device
    touchkey.input_dev = devm_input_allocate_device(&client.dev);
    if (!touchkey.input_dev) {
    dev_err(&client.dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    touchkey.input_dev.name = TM2_TOUCHKEY_DEV_NAME;
    touchkey.input_dev.id.bustype = BUS_I2C;
    touchkey.input_dev.keycode = touchkey.keycodes;
    touchkey.input_dev.keycodemax = touchkey.num_keycodes;
    touchkey.input_dev.keycodesize = sizeof(touchkey.keycodes[0]);
    input_set_capability(touchkey.input_dev, EV_MSC, MSC_SCAN);
    for (i = 0; i < touchkey.num_keycodes; i++)
    input_set_capability(touchkey.input_dev, EV_KEY,
    touchkey.keycodes[i]);
    error = input_register_device(touchkey.input_dev);
    if (error) {
    dev_err(&client.dev,
    "failed to register input device: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), tm2_touchkey_irq_handler,
    IRQF_ONESHOT,
    TM2_TOUCHKEY_DEV_NAME, touchkey);
    if (error) {
    dev_err(&client.dev,
    "failed to request threaded irq: %d\n", error);
    return error;
    }
// led device
    touchkey.led_dev.name = TM2_TOUCHKEY_DEV_NAME;
    touchkey.led_dev.brightness = LED_ON;
    touchkey.led_dev.max_brightness = LED_ON;
    touchkey.led_dev.brightness_set_blocking =
    tm2_touchkey_led_brightness_set;
    error = devm_led_classdev_register(&client.dev, &touchkey.led_dev);
    if (error) {
    dev_err(&client.dev,
    "failed to register touchkey led: %d\n", error);
    return error;
    }
    if (touchkey.variant.fixed_regulator)
    tm2_touchkey_led_brightness_set(&touchkey.led_dev, LED_ON);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_suspend(dev: *mut device) -> c_int {
    static int tm2_touchkey_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct tm2_touchkey_data *touchkey = i2c_get_clientdata(client);
    disable_irq(client.irq);
    tm2_touchkey_power_disable(touchkey);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tm2_touchkey_resume(dev: *mut device) -> c_int {
    static int tm2_touchkey_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct tm2_touchkey_data *touchkey = i2c_get_clientdata(client);
    int ret;
    enable_irq(client.irq);
    ret = tm2_touchkey_power_enable(touchkey);
    if (ret)
    dev_err(dev, "failed to enable power: %d\n", ret);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tm2_touchkey_pm_ops,
    tm2_touchkey_suspend, tm2_touchkey_resume);
    static const struct i2c_device_id tm2_touchkey_id_table[] = {
    { .name = TM2_TOUCHKEY_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tm2_touchkey_id_table);
    static const struct of_device_id tm2_touchkey_of_match[] = {
    {
    .compatible = "cypress,tm2-touchkey",
    .data = &tm2_touchkey_variant,
    }, {
    .compatible = "cypress,midas-touchkey",
    .data = &midas_touchkey_variant,
    }, {
    .compatible = "cypress,aries-touchkey",
    .data = &aries_touchkey_variant,
    }, {
    .compatible = "coreriver,tc360-touchkey",
    .data = &tc360_touchkey_variant,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, tm2_touchkey_of_match);
    static struct i2c_driver tm2_touchkey_driver = {
    .driver = {
    .name = TM2_TOUCHKEY_DEV_NAME,
    .pm = pm_sleep_ptr(&tm2_touchkey_pm_ops),
    .of_match_table = tm2_touchkey_of_match,
    },
    .probe = tm2_touchkey_probe,
    .id_table = tm2_touchkey_id_table,
    };
    module_i2c_driver(tm2_touchkey_driver);
    MODULE_AUTHOR("Beomho Seo <beomho.seo@samsung.com>");
    MODULE_AUTHOR("Jaechul Lee <jcsing.lee@samsung.com>");
    MODULE_DESCRIPTION("Samsung touchkey driver");
    MODULE_LICENSE("GPL v2");
