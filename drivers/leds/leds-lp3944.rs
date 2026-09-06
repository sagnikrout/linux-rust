//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-lp3944.c
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
// leds-lp3944.c - driver for National Semiconductor LP3944 Funlight Chip
//
// Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
//
// I2C driver for National Semiconductor LP3944 Funlight Chip
// http://www.national.com/pf/LP/LP3944.html
//
// This helper chip can drive up to 8 leds, with two programmable DIM modes;
// it could even be used as a gpio expander but this driver assumes it is used
// as a led controller.
//
// The DIM modes are used to set _blink_ patterns for leds, the pattern is
// specified supplying two parameters:
// - period: from 0s to 1.6s
// - duty cycle: percentage of the period the led is on, from 0 to 100
//
// LP3944 can be found on Motorola A910 smartphone, where it drives the rgb
// leds, the camera flash light and the displays backlights.
//

// Read Only Registers
pub const LP3944_REG_INPUT1: c_uint = 0x00 /* LEDs 0-7 InputRegister (Read Only) */;
pub const LP3944_REG_REGISTER1: c_uint = 0x01 /* None (Read Only) */;
pub const LP3944_REG_PSC0: c_uint = 0x02 /* Frequency Prescaler 0 (R/W) */;
pub const LP3944_REG_PWM0: c_uint = 0x03 /* PWM Register 0 (R/W) */;
pub const LP3944_REG_PSC1: c_uint = 0x04 /* Frequency Prescaler 1 (R/W) */;
pub const LP3944_REG_PWM1: c_uint = 0x05 /* PWM Register 1 (R/W) */;
pub const LP3944_REG_LS0: c_uint = 0x06 /* LEDs 0-3 Selector (R/W) */;
pub const LP3944_REG_LS1: c_uint = 0x07 /* LEDs 4-7 Selector (R/W) */;
// These registers are not used to control leds in LP3944, they can store
// arbitrary values which the chip will ignore.
//
pub const LP3944_REG_REGISTER8: c_uint = 0x08;
pub const LP3944_REG_REGISTER9: c_uint = 0x09;
pub const LP3944_DIM0: c_int = 0;
pub const LP3944_DIM1: c_int = 1;
// period in ms
pub const LP3944_PERIOD_MIN: c_int = 0;
pub const LP3944_PERIOD_MAX: c_int = 1600;
// duty cycle is a percentage
pub const LP3944_DUTY_CYCLE_MIN: c_int = 0;
pub const LP3944_DUTY_CYCLE_MAX: c_int = 100;

// Saved data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3944_led_data {
    pub id: u8,
    pub type: enum lp3944_type,
    pub ldev: led_classdev,
    pub client: *mut i2c_client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp3944_data {
    pub lock: mutex,
    pub client: *mut i2c_client,
    pub leds: [lp3944_led_data; LP3944_LEDS_MAX],
}

#[no_mangle]
unsafe extern "C" fn lp3944_reg_read(client: *mut i2c_client, reg: u8, value: *mut u8) -> c_int {
    static int lp3944_reg_read(struct i2c_client *client, u8 reg, u8 *value)
    {
    int tmp;
    tmp = i2c_smbus_read_byte_data(client, reg);
    if (tmp < 0)
    return tmp;
// value = tmp;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp3944_reg_write(client: *mut i2c_client, reg: u8, value: u8) -> c_int {
    static int lp3944_reg_write(struct i2c_client *client, u8 reg, u8 value)
    {
    return i2c_smbus_write_byte_data(client, reg, value);
    }
//
// lp3944_dim_set_period() - Set the period for DIM status
//
// @client: the i2c client
// @dim: either LP3944_DIM0 or LP3944_DIM1
// @period: period of a blink, that is a on/off cycle, expressed in ms.
//
#[no_mangle]
unsafe extern "C" fn lp3944_dim_set_period(client: *mut i2c_client, dim: u8, period: u16) -> c_int {
    static int lp3944_dim_set_period(struct i2c_client *client, u8 dim, u16 period)
    {
    u8 psc_reg;
    u8 psc_value;
    int err;
    if (dim == LP3944_DIM0)
    psc_reg = LP3944_REG_PSC0;
#[no_mangle]
pub unsafe extern "C" fn if(LP3944_DIM1: dim ==) -> else {
    else if (dim == LP3944_DIM1)
    psc_reg = LP3944_REG_PSC1;
    else
    return -EINVAL;
// Convert period to Prescaler value
    if (period > LP3944_PERIOD_MAX)
    return -EINVAL;
    psc_value = (period * 255) / LP3944_PERIOD_MAX;
    err = lp3944_reg_write(client, psc_reg, psc_value);
    return err;
    }
//
// lp3944_dim_set_dutycycle - Set the duty cycle for DIM status
//
// @client: the i2c client
// @dim: either LP3944_DIM0 or LP3944_DIM1
// @duty_cycle: percentage of a period during which a led is ON
//
    static int lp3944_dim_set_dutycycle(struct i2c_client *client, u8 dim,
    u8 duty_cycle)
    {
    u8 pwm_reg;
    u8 pwm_value;
    int err;
    if (dim == LP3944_DIM0)
    pwm_reg = LP3944_REG_PWM0;
#[no_mangle]
pub unsafe extern "C" fn if(LP3944_DIM1: dim ==) -> else {
    else if (dim == LP3944_DIM1)
    pwm_reg = LP3944_REG_PWM1;
    else
    return -EINVAL;
// Convert duty cycle to PWM value
    if (duty_cycle > LP3944_DUTY_CYCLE_MAX)
    return -EINVAL;
    pwm_value = (duty_cycle * 255) / LP3944_DUTY_CYCLE_MAX;
    err = lp3944_reg_write(client, pwm_reg, pwm_value);
    return err;
    }
//
// lp3944_led_set() - Set the led status
//
// @led: a lp3944_led_data structure
// @status: one of LP3944_LED_STATUS_OFF
// LP3944_LED_STATUS_ON
// LP3944_LED_STATUS_DIM0
// LP3944_LED_STATUS_DIM1
//
#[no_mangle]
unsafe extern "C" fn lp3944_led_set(led: *mut lp3944_led_data, status: u8) -> c_int {
    static int lp3944_led_set(struct lp3944_led_data *led, u8 status)
    {
    struct lp3944_data *data = i2c_get_clientdata(led.client);
    let mut id: u8 = led.id;
    u8 reg;
    let mut val: u8 = 0;
    int err;
    dev_dbg(&led.client.dev, "%s: %s, status before normalization:%d\n",
    __func__, led.ldev.name, status);
    switch (id) {
    case LP3944_LED0:
    case LP3944_LED1:
    case LP3944_LED2:
    case LP3944_LED3:
    reg = LP3944_REG_LS0;
    break;
    case LP3944_LED4:
    case LP3944_LED5:
    case LP3944_LED6:
    case LP3944_LED7:
    id -= LP3944_LED4;
    reg = LP3944_REG_LS1;
    break;
    default:
    return -EINVAL;
    }
    if (status > LP3944_LED_STATUS_DIM1)
    return -EINVAL;
//
// Invert status only when it's < 2 (i.e. 0 or 1) which means it's
// controlling the on/off state directly.
// When, instead, status is >= 2 don't invert it because it would mean
// to mess with the hardware blinking mode.
//
    if (led.type == LP3944_LED_TYPE_LED_INVERTED && status < 2)
    status = 1 - status;
    mutex_lock(&data.lock);
    lp3944_reg_read(led.client, reg, &val);
    val &= ~(LP3944_LED_STATUS_MASK << (id << 1));
    val |= (status << (id << 1));
    dev_dbg(&led.client.dev, "%s: %s, reg:%d id:%d status:%d val:%#x\n",
    __func__, led.ldev.name, reg, id, status, val);
// set led status
    err = lp3944_reg_write(led.client, reg, val);
    mutex_unlock(&data.lock);
    return err;
    }
    static int lp3944_led_set_blink(struct led_classdev *led_cdev,
    unsigned long *delay_on,
    unsigned long *delay_off)
    {
    struct lp3944_led_data *led = ldev_to_led(led_cdev);
    u16 period;
    u8 duty_cycle;
    int err;
// units are in ms
    if (*delay_on + *delay_off > LP3944_PERIOD_MAX)
    return -EINVAL;
    if (*delay_on == 0 && *delay_off == 0) {
// Special case: the leds subsystem requires a default user
// friendly blink pattern for the LED.  Let's blink the led
// slowly (1Hz).
//
// delay_on = 500;
// delay_off = 500;
    }
    period = (*delay_on) + (*delay_off);
// duty_cycle is the percentage of period during which the led is ON
    duty_cycle = 100 * (*delay_on) / period;
// invert duty cycle for inverted leds, this has the same effect of
// swapping delay_on and delay_off
//
    if (led.type == LP3944_LED_TYPE_LED_INVERTED)
    duty_cycle = 100 - duty_cycle;
// NOTE: using always the first DIM mode, this means that all leds
// will have the same blinking pattern.
//
// We could find a way later to have two leds blinking in hardware
// with different patterns at the same time, falling back to software
// control for the other ones.
//
    err = lp3944_dim_set_period(led.client, LP3944_DIM0, period);
    if (err)
    return err;
    err = lp3944_dim_set_dutycycle(led.client, LP3944_DIM0, duty_cycle);
    if (err)
    return err;
    dev_dbg(&led.client.dev, "%s: OK hardware accelerated blink!\n",
    __func__);
    lp3944_led_set(led, LP3944_LED_STATUS_DIM0);
    return 0;
    }
    static int lp3944_led_set_brightness(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct lp3944_led_data *led = ldev_to_led(led_cdev);
    dev_dbg(&led.client.dev, "%s: %s, %d\n",
    __func__, led_cdev.name, brightness);
    return lp3944_led_set(led, !!brightness);
    }
    static int lp3944_configure(struct i2c_client *client,
    struct lp3944_data *data,
    struct lp3944_platform_data *pdata)
    {
    int i, err = 0;
    for (i = 0; i < pdata.leds_size; i++) {
    struct lp3944_led *pled = &pdata.leds[i];
    struct lp3944_led_data *led = &data.leds[i];
    led.client = client;
    led.id = i;
    switch (pled.type) {
    case LP3944_LED_TYPE_LED:
    case LP3944_LED_TYPE_LED_INVERTED:
    led.type = pled.type;
    led.ldev.name = pled.name;
    led.ldev.max_brightness = 1;
    led.ldev.brightness_set_blocking =
    lp3944_led_set_brightness;
    led.ldev.blink_set = lp3944_led_set_blink;
    led.ldev.flags = LED_CORE_SUSPENDRESUME;
    err = led_classdev_register(&client.dev, &led.ldev);
    if (err < 0) {
    dev_err(&client.dev,
    "couldn't register LED %s\n",
    led.ldev.name);
    goto exit;
    }
// to expose the default value to userspace
    led.ldev.brightness =
    (enum led_brightness) pled.status;
// Set the default led status
    err = lp3944_led_set(led, pled.status);
    if (err < 0) {
    dev_err(&client.dev,
    "%s couldn't set STATUS %d\n",
    led.ldev.name, pled.status);
    goto exit;
    }
    break;
    case LP3944_LED_TYPE_NONE:
    default:
    break;
    }
    }
    return 0;
    exit:
    if (i > 0)
    for (i = i - 1; i >= 0; i--)
    switch (pdata.leds[i].type) {
    case LP3944_LED_TYPE_LED:
    case LP3944_LED_TYPE_LED_INVERTED:
    led_classdev_unregister(&data.leds[i].ldev);
    break;
    case LP3944_LED_TYPE_NONE:
    default:
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lp3944_probe(client: *mut i2c_client) -> c_int {
    static int lp3944_probe(struct i2c_client *client)
    {
    struct lp3944_platform_data *lp3944_pdata =
    dev_get_platdata(&client.dev);
    struct lp3944_data *data;
    int err;
    if (lp3944_pdata == core::ptr::null_mut()) {
    dev_err(&client.dev, "no platform data\n");
    return -EINVAL;
    }
// Let's see whether this adapter can support what we need.
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&client.dev, "insufficient functionality!\n");
    return -ENODEV;
    }
    data = devm_kzalloc(&client.dev, sizeof(struct lp3944_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    i2c_set_clientdata(client, data);
    mutex_init(&data.lock);
    err = lp3944_configure(client, data, lp3944_pdata);
    if (err < 0)
    return err;
    dev_info(&client.dev, "lp3944 enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lp3944_remove(client: *mut i2c_client) {
    static void lp3944_remove(struct i2c_client *client)
    {
    struct lp3944_platform_data *pdata = dev_get_platdata(&client.dev);
    struct lp3944_data *data = i2c_get_clientdata(client);
    int i;
    for (i = 0; i < pdata.leds_size; i++)
    switch (data.leds[i].type) {
    case LP3944_LED_TYPE_LED:
    case LP3944_LED_TYPE_LED_INVERTED:
    led_classdev_unregister(&data.leds[i].ldev);
    break;
    case LP3944_LED_TYPE_NONE:
    default:
    break;
    }
    }
// lp3944 i2c driver struct
    static const struct i2c_device_id lp3944_id[] = {
    { .name = "lp3944" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lp3944_id);
    static struct i2c_driver lp3944_driver = {
    .driver   = {
    .name = "lp3944",
    },
    .probe    = lp3944_probe,
    .remove   = lp3944_remove,
    .id_table = lp3944_id,
    };
    module_i2c_driver(lp3944_driver);
    MODULE_AUTHOR("Antonio Ospite <ospite@studenti.unina.it>");
    MODULE_DESCRIPTION("LP3944 Fun Light Chip");
    MODULE_LICENSE("GPL");
