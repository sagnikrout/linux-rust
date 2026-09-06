//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-ktd2692.c
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
// LED driver : leds-ktd2692.c
//
// Copyright (C) 2015 Samsung Electronics
// Ingi Kim <ingi2.kim@samsung.com>
//

// Value related the movie mode
pub const KTD2692_MOVIE_MODE_CURRENT_LEVELS: c_int = 16;

pub const KTD2692_MM_MIN_CURR_THRESHOLD_SCALE: c_int = 8;
// Value related the flash mode
pub const KTD2692_FLASH_MODE_TIMEOUT_LEVELS: c_int = 8;
pub const KTD2692_FLASH_MODE_TIMEOUT_DISABLE: c_int = 0;

// Macro for getting offset of flash timeout

// Base register address
pub const KTD2692_REG_LVP_BASE: c_uint = 0x00;
pub const KTD2692_REG_FLASH_TIMEOUT_BASE: c_uint = 0x20;
pub const KTD2692_REG_MM_MIN_CURR_THRESHOLD_BASE: c_uint = 0x40;
pub const KTD2692_REG_MOVIE_CURRENT_BASE: c_uint = 0x60;
pub const KTD2692_REG_FLASH_CURRENT_BASE: c_uint = 0x80;
pub const KTD2692_REG_MODE_BASE: c_uint = 0xA0;
// KTD2692 default length of name
pub const KTD2692_NAME_LENGTH: c_int = 20;
// Movie / Flash Mode Control
    enum ktd2692_led_mode {
    KTD2692_MODE_DISABLE = 0,	/* default */
    KTD2692_MODE_MOVIE,
    KTD2692_MODE_FLASH,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd2692_led_config_data {
// maximum LED current in movie mode
    pub movie_max_microamp: u32,
// maximum LED current in flash mode
    pub flash_max_microamp: u32,
// maximum flash timeout
    pub flash_max_timeout: u32,
// max LED brightness level
    pub max_brightness: enum led_brightness,
}

    static const struct expresswire_timing ktd2692_timing = {
    .poweroff_us = 700,
    .data_start_us = 10,
    .end_of_data_low_us = 10,
    .end_of_data_high_us = 350,
    .short_bitset_us = 4,
    .long_bitset_us = 12
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd2692_context {
// Common ExpressWire properties (ctrl GPIO and timing)
    pub props: expresswire_common_props,
// Related LED Flash class device
    pub fled_cdev: led_classdev_flash,
// secures access to the device
    pub lock: mutex,
    pub regulator: *mut regulator,
    pub aux_gpio: *mut gpio_desc,
    pub mode: enum ktd2692_led_mode,
    pub torch_brightness: enum led_brightness,
}

    static struct ktd2692_context *fled_cdev_to_led(
    struct led_classdev_flash *fled_cdev)
    {
    return container_of(fled_cdev, struct ktd2692_context, fled_cdev);
    }
    static int ktd2692_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct led_classdev_flash *fled_cdev = lcdev_to_flcdev(led_cdev);
    struct ktd2692_context *led = fled_cdev_to_led(fled_cdev);
    mutex_lock(&led.lock);
    if (brightness == LED_OFF) {
    led.mode = KTD2692_MODE_DISABLE;
    gpiod_direction_output(led.aux_gpio, 0);
    } else {
    expresswire_write_u8(&led.props, brightness |
    KTD2692_REG_MOVIE_CURRENT_BASE);
    led.mode = KTD2692_MODE_MOVIE;
    }
    expresswire_write_u8(&led.props, led.mode | KTD2692_REG_MODE_BASE);
    mutex_unlock(&led.lock);
    return 0;
    }
    static int ktd2692_led_flash_strobe_set(struct led_classdev_flash *fled_cdev,
    bool state)
    {
    struct ktd2692_context *led = fled_cdev_to_led(fled_cdev);
    struct led_flash_setting *timeout = &fled_cdev.timeout;
    u32 flash_tm_reg;
    mutex_lock(&led.lock);
    if (state) {
    flash_tm_reg = GET_TIMEOUT_OFFSET(timeout.val, timeout.step);
    expresswire_write_u8(&led.props, flash_tm_reg
    | KTD2692_REG_FLASH_TIMEOUT_BASE);
    led.mode = KTD2692_MODE_FLASH;
    gpiod_direction_output(led.aux_gpio, 1);
    } else {
    led.mode = KTD2692_MODE_DISABLE;
    gpiod_direction_output(led.aux_gpio, 0);
    }
    expresswire_write_u8(&led.props, led.mode | KTD2692_REG_MODE_BASE);
    fled_cdev.led_cdev.brightness = LED_OFF;
    led.mode = KTD2692_MODE_DISABLE;
    mutex_unlock(&led.lock);
    return 0;
    }
    static int ktd2692_led_flash_timeout_set(struct led_classdev_flash *fled_cdev,
    u32 timeout)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd2692_init_movie_current_max(cfg: *mut ktd2692_led_config_data) {
    static void ktd2692_init_movie_current_max(struct ktd2692_led_config_data *cfg)
    {
    u32 offset, step;
    u32 movie_current_microamp;
    offset = KTD2692_MOVIE_MODE_CURRENT_LEVELS;
    step = KTD2692_MM_TO_FL_RATIO(cfg.flash_max_microamp)
    / KTD2692_MOVIE_MODE_CURRENT_LEVELS;
    do {
    movie_current_microamp = step * offset;
    offset--;
    } while ((movie_current_microamp > cfg.movie_max_microamp) &&
    (offset > 0));
    cfg.max_brightness = offset;
    }
    static void ktd2692_init_flash_timeout(struct led_classdev_flash *fled_cdev,
    struct ktd2692_led_config_data *cfg)
    {
    struct led_flash_setting *setting;
    setting = &fled_cdev.timeout;
    setting.min = KTD2692_FLASH_MODE_TIMEOUT_DISABLE;
    setting.max = cfg.flash_max_timeout;
    setting.step = cfg.flash_max_timeout
    / (KTD2692_FLASH_MODE_TIMEOUT_LEVELS - 1);
    setting.val = cfg.flash_max_timeout;
    }
#[no_mangle]
unsafe extern "C" fn ktd2692_setup(led: *mut ktd2692_context) {
    static void ktd2692_setup(struct ktd2692_context *led)
    {
    led.mode = KTD2692_MODE_DISABLE;
    expresswire_power_off(&led.props);
    gpiod_direction_output(led.aux_gpio, 0);
    expresswire_write_u8(&led.props, (KTD2692_MM_MIN_CURR_THRESHOLD_SCALE - 1)
    | KTD2692_REG_MM_MIN_CURR_THRESHOLD_BASE);
    expresswire_write_u8(&led.props, KTD2692_FLASH_MODE_CURR_PERCENT(45)
    | KTD2692_REG_FLASH_CURRENT_BASE);
    }
#[no_mangle]
unsafe extern "C" fn regulator_disable_action(_data: *mut c_void) {
    static void regulator_disable_action(void *_data)
    {
    struct device *dev = _data;
    struct ktd2692_context *led = dev_get_drvdata(dev);
    int ret;
    ret = regulator_disable(led.regulator);
    if (ret)
    dev_err(dev, "Failed to disable supply: %d\n", ret);
    }
    static int ktd2692_parse_dt(struct ktd2692_context *led, struct device *dev,
    struct ktd2692_led_config_data *cfg)
    {
    struct device_node *np = dev_of_node(dev);
    int ret;
    if (!np)
    return -ENXIO;
    led.props.ctrl_gpio = devm_gpiod_get(dev, "ctrl", GPIOD_ASIS);
    ret = PTR_ERR_OR_ZERO(led.props.ctrl_gpio);
    if (ret)
    return dev_err_probe(dev, ret, "cannot get ctrl-gpios\n");
    led.aux_gpio = devm_gpiod_get_optional(dev, "aux", GPIOD_ASIS);
    if (IS_ERR(led.aux_gpio))
    return dev_err_probe(dev, PTR_ERR(led.aux_gpio), "cannot get aux-gpios\n");
    led.regulator = devm_regulator_get(dev, "vin");
    if (IS_ERR(led.regulator))
    led.regulator = core::ptr::null_mut();
    if (led.regulator) {
    ret = regulator_enable(led.regulator);
    if (ret) {
    dev_err(dev, "Failed to enable supply: %d\n", ret);
    } else {
    ret = devm_add_action_or_reset(dev,
    regulator_disable_action, dev);
    if (ret)
    return ret;
    }
    }
    struct device_node *child_node __free(device_node) =
    of_get_next_available_child(np, core::ptr::null_mut());
    if (!child_node) {
    dev_err(dev, "No DT child node found for connected LED.\n");
    return -EINVAL;
    }
    led.fled_cdev.led_cdev.name =
    of_get_property(child_node, "label", core::ptr::null_mut()) ? : child_node.name;
    ret = of_property_read_u32(child_node, "led-max-microamp",
    &cfg.movie_max_microamp);
    if (ret) {
    dev_err(dev, "failed to parse led-max-microamp\n");
    return ret;
    }
    ret = of_property_read_u32(child_node, "flash-max-microamp",
    &cfg.flash_max_microamp);
    if (ret) {
    dev_err(dev, "failed to parse flash-max-microamp\n");
    return ret;
    }
    ret = of_property_read_u32(child_node, "flash-max-timeout-us",
    &cfg.flash_max_timeout);
    if (ret) {
    dev_err(dev, "failed to parse flash-max-timeout-us\n");
    return ret;
    }
    return 0;
    }
    static const struct led_flash_ops flash_ops = {
    .strobe_set = ktd2692_led_flash_strobe_set,
    .timeout_set = ktd2692_led_flash_timeout_set,
    };
#[no_mangle]
unsafe extern "C" fn ktd2692_probe(pdev: *mut platform_device) -> c_int {
    static int ktd2692_probe(struct platform_device *pdev)
    {
    struct ktd2692_context *led;
    struct led_classdev *led_cdev;
    struct led_classdev_flash *fled_cdev;
    struct ktd2692_led_config_data led_cfg;
    int ret;
    led = devm_kzalloc(&pdev.dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    fled_cdev = &led.fled_cdev;
    led_cdev = &fled_cdev.led_cdev;
    led.props.timing = ktd2692_timing;
    ret = ktd2692_parse_dt(led, &pdev.dev, &led_cfg);
    if (ret)
    return ret;
    ktd2692_init_flash_timeout(fled_cdev, &led_cfg);
    ktd2692_init_movie_current_max(&led_cfg);
    fled_cdev.ops = &flash_ops;
    led_cdev.max_brightness = led_cfg.max_brightness;
    led_cdev.brightness_set_blocking = ktd2692_led_brightness_set;
    led_cdev.flags |= LED_CORE_SUSPENDRESUME | LED_DEV_CAP_FLASH;
    mutex_init(&led.lock);
    platform_set_drvdata(pdev, led);
    ret = led_classdev_flash_register(&pdev.dev, fled_cdev);
    if (ret) {
    dev_err(&pdev.dev, "can't register LED %s\n", led_cdev.name);
    mutex_destroy(&led.lock);
    return ret;
    }
    ktd2692_setup(led);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd2692_remove(pdev: *mut platform_device) {
    static void ktd2692_remove(struct platform_device *pdev)
    {
    struct ktd2692_context *led = platform_get_drvdata(pdev);
    led_classdev_flash_unregister(&led.fled_cdev);
    mutex_destroy(&led.lock);
    }
    static const struct of_device_id ktd2692_match[] = {
    { .compatible = "kinetic,ktd2692", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ktd2692_match);
    static struct platform_driver ktd2692_driver = {
    .driver = {
    .name  = "ktd2692",
    .of_match_table = ktd2692_match,
    },
    .probe  = ktd2692_probe,
    .remove = ktd2692_remove,
    };
    module_platform_driver(ktd2692_driver);
    MODULE_IMPORT_NS("EXPRESSWIRE");
    MODULE_AUTHOR("Ingi Kim <ingi2.kim@samsung.com>");
    MODULE_DESCRIPTION("Kinetic KTD2692 LED driver");
    MODULE_LICENSE("GPL v2");
