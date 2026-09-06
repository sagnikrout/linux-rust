//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-an30259a.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for Panasonic AN30259A 3-channel LED driver
//
// Copyright (c) 2018 Simon Shields <simon@lineageos.org>
//
// Datasheet:
// https://www.alliedelec.com/m/d/a9d2b3ee87c2d1a535a41dd747b1c247.pdf

pub const AN30259A_MAX_LEDS: c_int = 3;
pub const AN30259A_REG_SRESET: c_uint = 0x00;

// LED power registers
pub const AN30259A_REG_LED_ON: c_uint = 0x01;

// slope control registers

// detention time control (length of each slope step)

pub const AN30259A_REG_MAX: c_uint = 0x14;

    struct an30259a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct an30259a_led {
    pub chip: *mut an30259a,
    pub fwnode: *mut fwnode_handle,
    pub cdev: led_classdev,
    pub num: u32,
    pub default_state: enum led_default_state,
    pub sloping: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct an30259a {
    pub /: *mut *mut mutex mutex; / held when writing to registers,
    pub client: *mut i2c_client,
    pub leds: [an30259a_led; AN30259A_MAX_LEDS],
    pub regmap: *mut regmap,
    pub num_leds: c_int,
}

    static int an30259a_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct an30259a_led *led;
    int ret;
    unsigned int led_on;
    led = container_of(cdev, struct an30259a_led, cdev);
    mutex_lock(&led.chip.mutex);
    ret = regmap_read(led.chip.regmap, AN30259A_REG_LED_ON, &led_on);
    if (ret)
    goto error;
    switch (brightness) {
    case LED_OFF:
    led_on &= ~AN30259A_LED_EN(led.num);
    led_on &= ~AN30259A_LED_SLOPE(led.num);
    led.sloping = false;
    break;
    default:
    led_on |= AN30259A_LED_EN(led.num);
    if (led.sloping)
    led_on |= AN30259A_LED_SLOPE(led.num);
    ret = regmap_write(led.chip.regmap,
    AN30259A_REG_LEDCNT1(led.num),
    AN30259A_LED_DUTYMAX(0xf) |
    AN30259A_LED_DUTYMID(0xf));
    if (ret)
    goto error;
    break;
    }
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LED_ON, led_on);
    if (ret)
    goto error;
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LEDCC(led.num),
    brightness);
    error:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int an30259a_blink_set(struct led_classdev *cdev,
    unsigned long *delay_off, unsigned long *delay_on)
    {
    struct an30259a_led *led;
    int ret, num;
    unsigned int led_on;
    let mut off: c_ulong = *delay_off, on = *delay_on;
    led = container_of(cdev, struct an30259a_led, cdev);
    mutex_lock(&led.chip.mutex);
    num = led.num;
// slope time can only be a multiple of 500ms.
    if (off % AN30259A_SLOPE_RESOLUTION || on % AN30259A_SLOPE_RESOLUTION) {
    ret = -EINVAL;
    goto error;
    }
// up to a maximum of 7500ms.
    if (off > AN30259A_BLINK_MAX_TIME || on > AN30259A_BLINK_MAX_TIME) {
    ret = -EINVAL;
    goto error;
    }
// if no blink specified, default to 1 Hz.
    if (!off && !on) {
// delay_off = off = 500;
// delay_on = on = 500;
    }
// convert into values the HW will understand.
    off /= AN30259A_SLOPE_RESOLUTION;
    on /= AN30259A_SLOPE_RESOLUTION;
// duty min should be zero (=off), delay should be zero.
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LEDCNT2(num),
    AN30259A_LED_DELAY(0) | AN30259A_LED_DUTYMIN(0));
    if (ret)
    goto error;
// reset detention time (no "breathing" effect).
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LEDCNT3(num),
    AN30259A_LED_DT1(0) | AN30259A_LED_DT2(0));
    if (ret)
    goto error;
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LEDCNT4(num),
    AN30259A_LED_DT3(0) | AN30259A_LED_DT4(0));
    if (ret)
    goto error;
// slope time controls on/off cycle length.
    ret = regmap_write(led.chip.regmap, AN30259A_REG_SLOPE(num),
    AN30259A_LED_SLOPETIME1(off) |
    AN30259A_LED_SLOPETIME2(on));
    if (ret)
    goto error;
// Finally, enable slope mode.
    ret = regmap_read(led.chip.regmap, AN30259A_REG_LED_ON, &led_on);
    if (ret)
    goto error;
    led_on |= AN30259A_LED_SLOPE(num) | AN30259A_LED_EN(led.num);
    ret = regmap_write(led.chip.regmap, AN30259A_REG_LED_ON, led_on);
    if (!ret)
    led.sloping = true;
    error:
    mutex_unlock(&led.chip.mutex);
    return ret;
    }
    static int an30259a_dt_init(struct i2c_client *client,
    struct an30259a *chip)
    {
    struct device_node *np = dev_of_node(&client.dev), *child;
    int count, ret;
    let mut i: c_int = 0;
    struct an30259a_led *led;
    count = of_get_available_child_count(np);
    if (!count || count > AN30259A_MAX_LEDS)
    return -EINVAL;
    for_each_available_child_of_node(np, child) {
    u32 source;
    ret = of_property_read_u32(child, "reg", &source);
    if (ret != 0 || !source || source > AN30259A_MAX_LEDS) {
    dev_err(&client.dev, "Couldn't read LED address: %d\n",
    ret);
    count--;
    continue;
    }
    led = &chip.leds[i];
    led.num = source;
    led.chip = chip;
    led.fwnode = of_fwnode_handle(child);
    led.default_state = led_init_default_state_get(led.fwnode);
    i++;
    }
    if (!count)
    return -EINVAL;
    chip.num_leds = i;
    return 0;
    }
    static const struct regmap_config an30259a_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AN30259A_REG_MAX,
    };
#[no_mangle]
unsafe extern "C" fn an30259a_init_default_state(led: *mut an30259a_led) {
    static void an30259a_init_default_state(struct an30259a_led *led)
    {
    struct an30259a *chip = led.chip;
    int led_on, err;
    switch (led.default_state) {
    case LEDS_DEFSTATE_ON:
    led.cdev.brightness = LED_FULL;
    break;
    case LEDS_DEFSTATE_KEEP:
    err = regmap_read(chip.regmap, AN30259A_REG_LED_ON, &led_on);
    if (err)
    break;
    if (!(led_on & AN30259A_LED_EN(led.num))) {
    led.cdev.brightness = LED_OFF;
    break;
    }
    regmap_read(chip.regmap, AN30259A_REG_LEDCC(led.num),
    &led.cdev.brightness);
    break;
    default:
    led.cdev.brightness = LED_OFF;
    }
    an30259a_brightness_set(&led.cdev, led.cdev.brightness);
    }
#[no_mangle]
unsafe extern "C" fn an30259a_probe(client: *mut i2c_client) -> c_int {
    static int an30259a_probe(struct i2c_client *client)
    {
    struct an30259a *chip;
    int i, err;
    chip = devm_kzalloc(&client.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    err = an30259a_dt_init(client, chip);
    if (err < 0)
    return err;
    err = devm_mutex_init(&client.dev, &chip.mutex);
    if (err)
    return err;
    chip.client = client;
    i2c_set_clientdata(client, chip);
    chip.regmap = devm_regmap_init_i2c(client, &an30259a_regmap_config);
    if (IS_ERR(chip.regmap)) {
    err = PTR_ERR(chip.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    err);
    goto exit;
    }
    for (i = 0; i < chip.num_leds; i++) {
    let mut init_data: led_init_data = {};
    an30259a_init_default_state(&chip.leds[i]);
    chip.leds[i].cdev.brightness_set_blocking =
    an30259a_brightness_set;
    chip.leds[i].cdev.blink_set = an30259a_blink_set;
    init_data.fwnode = chip.leds[i].fwnode;
    init_data.devicename = AN30259A_NAME;
    init_data.default_label = ":";
    err = devm_led_classdev_register_ext(&client.dev,
    &chip.leds[i].cdev,
    &init_data);
    if (err < 0)
    goto exit;
    }
    return 0;
    exit:
    return err;
    }
    static const struct of_device_id an30259a_match_table[] = {
    { .compatible = "panasonic,an30259a", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, an30259a_match_table);
    static const struct i2c_device_id an30259a_id[] = {
    { .name = "an30259a" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, an30259a_id);
    static struct i2c_driver an30259a_driver = {
    .driver = {
    .name = "leds-an30259a",
    .of_match_table = an30259a_match_table,
    },
    .probe = an30259a_probe,
    .id_table = an30259a_id,
    };
    module_i2c_driver(an30259a_driver);
    MODULE_AUTHOR("Simon Shields <simon@lineageos.org>");
    MODULE_DESCRIPTION("AN30259A LED driver");
    MODULE_LICENSE("GPL v2");
