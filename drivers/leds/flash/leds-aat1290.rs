//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-aat1290.c
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
// LED Flash class driver for the AAT1290
// 1.5A Step-Up Current Regulator for Flash LEDs
//
// Copyright (C) 2015, Samsung Electronics Co., Ltd.
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//

pub const AAT1290_MOVIE_MODE_CURRENT_ADDR: c_int = 17;
pub const AAT1290_MAX_MM_CURR_PERCENT_0: c_int = 16;
pub const AAT1290_MAX_MM_CURR_PERCENT_100: c_int = 1;
pub const AAT1290_FLASH_SAFETY_TIMER_ADDR: c_int = 18;
pub const AAT1290_MOVIE_MODE_CONFIG_ADDR: c_int = 19;
pub const AAT1290_MOVIE_MODE_OFF: c_int = 1;
pub const AAT1290_MOVIE_MODE_ON: c_int = 3;
pub const AAT1290_MM_CURRENT_RATIO_ADDR: c_int = 20;
pub const AAT1290_MM_TO_FL_1_92: c_int = 1;

pub const AAT1290_LATCH_TIME_MIN_US: c_int = 500;
pub const AAT1290_LATCH_TIME_MAX_US: c_int = 1000;
pub const AAT1290_EN_SET_TICK_TIME_US: c_int = 1;
pub const AAT1290_FLEN_OFF_DELAY_TIME_US: c_int = 10;
pub const AAT1290_FLASH_TM_NUM_LEVELS: c_int = 16;
pub const AAT1290_MM_CURRENT_SCALE_SIZE: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat1290_led_config_data {
// maximum LED current in movie mode
    pub max_mm_current: u32,
// maximum LED current in flash mode
    pub max_flash_current: u32,
// maximum flash timeout
    pub max_flash_tm: u32,
// external strobe capability
    pub has_external_strobe: bool,
// max LED brightness level
    pub max_brightness: enum led_brightness,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aat1290_led {
// platform device data
    pub pdev: *mut platform_device,
// secures access to the device
    pub lock: mutex,
// corresponding LED Flash class device
    pub fled_cdev: led_classdev_flash,
// V4L2 Flash device
    pub v4l2_flash: *mut v4l2_flash,
// FLEN pin
    pub gpio_fl_en: *mut gpio_desc,
// EN|SET pin
    pub gpio_en_set: *mut gpio_desc,
// movie mode current scale
    pub mm_current_scale: *mut c_int,
// device mode
    pub movie_mode: bool,
}

    static struct aat1290_led *fled_cdev_to_led(
    struct led_classdev_flash *fled_cdev)
    {
    return container_of(fled_cdev, struct aat1290_led, fled_cdev);
    }
    static struct led_classdev_flash *led_cdev_to_fled_cdev(
    struct led_classdev *led_cdev)
    {
    return container_of(led_cdev, struct led_classdev_flash, led_cdev);
    }
#[no_mangle]
unsafe extern "C" fn aat1290_as2cwire_write(led: *mut aat1290_led, addr: c_int, value: c_int) {
    static void aat1290_as2cwire_write(struct aat1290_led *led, int addr, int value)
    {
    int i;
    gpiod_direction_output(led.gpio_fl_en, 0);
    gpiod_direction_output(led.gpio_en_set, 0);
    udelay(AAT1290_FLEN_OFF_DELAY_TIME_US);
// write address
    for (i = 0; i < addr; ++i) {
    udelay(AAT1290_EN_SET_TICK_TIME_US);
    gpiod_direction_output(led.gpio_en_set, 0);
    udelay(AAT1290_EN_SET_TICK_TIME_US);
    gpiod_direction_output(led.gpio_en_set, 1);
    }
    usleep_range(AAT1290_LATCH_TIME_MIN_US, AAT1290_LATCH_TIME_MAX_US);
// write data
    for (i = 0; i < value; ++i) {
    udelay(AAT1290_EN_SET_TICK_TIME_US);
    gpiod_direction_output(led.gpio_en_set, 0);
    udelay(AAT1290_EN_SET_TICK_TIME_US);
    gpiod_direction_output(led.gpio_en_set, 1);
    }
    usleep_range(AAT1290_LATCH_TIME_MIN_US, AAT1290_LATCH_TIME_MAX_US);
    }
    static void aat1290_set_flash_safety_timer(struct aat1290_led *led,
    unsigned int micro_sec)
    {
    struct led_classdev_flash *fled_cdev = &led.fled_cdev;
    struct led_flash_setting *flash_tm = &fled_cdev.timeout;
    int flash_tm_reg = AAT1290_FLASH_TM_NUM_LEVELS -
    (micro_sec / flash_tm.step) + 1;
    aat1290_as2cwire_write(led, AAT1290_FLASH_SAFETY_TIMER_ADDR,
    flash_tm_reg);
    }
// LED subsystem callbacks
    static int aat1290_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct led_classdev_flash *fled_cdev = led_cdev_to_fled_cdev(led_cdev);
    struct aat1290_led *led = fled_cdev_to_led(fled_cdev);
    mutex_lock(&led.lock);
    if (brightness == 0) {
    gpiod_direction_output(led.gpio_fl_en, 0);
    gpiod_direction_output(led.gpio_en_set, 0);
    led.movie_mode = false;
    } else {
    if (!led.movie_mode) {
    aat1290_as2cwire_write(led,
    AAT1290_MM_CURRENT_RATIO_ADDR,
    AAT1290_MM_TO_FL_1_92);
    led.movie_mode = true;
    }
    aat1290_as2cwire_write(led, AAT1290_MOVIE_MODE_CURRENT_ADDR,
    AAT1290_MAX_MM_CURR_PERCENT_0 - brightness);
    aat1290_as2cwire_write(led, AAT1290_MOVIE_MODE_CONFIG_ADDR,
    AAT1290_MOVIE_MODE_ON);
    }
    mutex_unlock(&led.lock);
    return 0;
    }
    static int aat1290_led_flash_strobe_set(struct led_classdev_flash *fled_cdev,
    bool state)
    {
    struct aat1290_led *led = fled_cdev_to_led(fled_cdev);
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    struct led_flash_setting *timeout = &fled_cdev.timeout;
    mutex_lock(&led.lock);
    if (state) {
    aat1290_set_flash_safety_timer(led, timeout.val);
    gpiod_direction_output(led.gpio_fl_en, 1);
    } else {
    gpiod_direction_output(led.gpio_fl_en, 0);
    gpiod_direction_output(led.gpio_en_set, 0);
    }
//
// To reenter movie mode after a flash event the part must be cycled
// off and back on to reset the movie mode and reprogrammed via the
// AS2Cwire. Therefore the brightness and movie_mode properties needs
// to be updated here to reflect the actual state.
//
    led_cdev.brightness = 0;
    led.movie_mode = false;
    mutex_unlock(&led.lock);
    return 0;
    }
    static int aat1290_led_flash_timeout_set(struct led_classdev_flash *fled_cdev,
    u32 timeout)
    {
//
// Don't do anything - flash timeout is cached in the led-class-flash
// core and will be applied in the strobe_set op, as writing the
// safety timer register spuriously turns the torch mode on.
//
    return 0;
    }
    static int aat1290_led_parse_dt(struct aat1290_led *led,
    struct aat1290_led_config_data *cfg,
    struct device_node **sub_node)
    {
    struct device *dev = &led.pdev.dev;

    struct pinctrl *pinctrl;

    let mut ret: c_int = 0;
    led.gpio_fl_en = devm_gpiod_get(dev, "flen", GPIOD_ASIS);
    if (IS_ERR(led.gpio_fl_en)) {
    ret = PTR_ERR(led.gpio_fl_en);
    dev_err(dev, "Unable to claim gpio \"flen\".\n");
    return ret;
    }
    led.gpio_en_set = devm_gpiod_get(dev, "enset", GPIOD_ASIS);
    if (IS_ERR(led.gpio_en_set)) {
    ret = PTR_ERR(led.gpio_en_set);
    dev_err(dev, "Unable to claim gpio \"enset\".\n");
    return ret;
    }

    pinctrl = devm_pinctrl_get_select_default(&led.pdev.dev);
    if (IS_ERR(pinctrl)) {
    cfg.has_external_strobe = false;
    dev_info(dev,
    "No support for external strobe detected.\n");
    } else {
    cfg.has_external_strobe = true;
    }

    struct device_node *child_node __free(device_node) =
    of_get_next_available_child(dev_of_node(dev), core::ptr::null_mut());
    if (!child_node) {
    dev_err(dev, "No DT child node found for connected LED.\n");
    return -EINVAL;
    }
    ret = of_property_read_u32(child_node, "led-max-microamp",
    &cfg.max_mm_current);
//
// led-max-microamp will default to 1/20 of flash-max-microamp
// in case it is missing.
//
    if (ret < 0)
    dev_warn(dev,
    "led-max-microamp DT property missing\n");
    ret = of_property_read_u32(child_node, "flash-max-microamp",
    &cfg.max_flash_current);
    if (ret < 0) {
    dev_err(dev,
    "flash-max-microamp DT property missing\n");
    return ret;
    }
    ret = of_property_read_u32(child_node, "flash-max-timeout-us",
    &cfg.max_flash_tm);
    if (ret < 0) {
    dev_err(dev,
    "flash-max-timeout-us DT property missing\n");
    return ret;
    }
// sub_node = child_node;
    return 0;
    }
    static void aat1290_led_validate_mm_current(struct aat1290_led *led,
    struct aat1290_led_config_data *cfg)
    {
    int i, b = 0, e = AAT1290_MM_CURRENT_SCALE_SIZE;
    while (e - b > 1) {
    i = b + (e - b) / 2;
    if (cfg.max_mm_current < led.mm_current_scale[i])
    e = i;
    else
    b = i;
    }
    cfg.max_mm_current = led.mm_current_scale[b];
    cfg.max_brightness = b + 1;
    }
    static int init_mm_current_scale(struct aat1290_led *led,
    struct aat1290_led_config_data *cfg)
    {
    static const int max_mm_current_percent[] = {
    20, 22, 25, 28, 32, 36, 40, 45, 50, 56,
    63, 71, 79, 89, 100
    };
    int i, max_mm_current =
    AAT1290_MAX_MM_CURRENT(cfg.max_flash_current);
    led.mm_current_scale = devm_kzalloc(&led.pdev.dev,
    sizeof(max_mm_current_percent),
    GFP_KERNEL);
    if (!led.mm_current_scale)
    return -ENOMEM;
    for (i = 0; i < AAT1290_MM_CURRENT_SCALE_SIZE; ++i)
    led.mm_current_scale[i] = max_mm_current *
    max_mm_current_percent[i] / 100;
    return 0;
    }
    static int aat1290_led_get_configuration(struct aat1290_led *led,
    struct aat1290_led_config_data *cfg,
    struct device_node **sub_node)
    {
    int ret;
    ret = aat1290_led_parse_dt(led, cfg, sub_node);
    if (ret < 0)
    return ret;
//
// Init non-linear movie mode current scale basing
// on the max flash current from led configuration.
//
    ret = init_mm_current_scale(led, cfg);
    if (ret < 0)
    return ret;
    aat1290_led_validate_mm_current(led, cfg);

    devm_kfree(&led.pdev.dev, led.mm_current_scale);

    return 0;
    }
    static void aat1290_init_flash_timeout(struct aat1290_led *led,
    struct aat1290_led_config_data *cfg)
    {
    struct led_classdev_flash *fled_cdev = &led.fled_cdev;
    struct led_flash_setting *setting;
// Init flash timeout setting
    setting = &fled_cdev.timeout;
    setting.min = cfg.max_flash_tm / AAT1290_FLASH_TM_NUM_LEVELS;
    setting.max = cfg.max_flash_tm;
    setting.step = setting.min;
    setting.val = setting.max;
    }

    static enum led_brightness aat1290_intensity_to_brightness(
    struct v4l2_flash *v4l2_flash,
    s32 intensity)
    {
    struct led_classdev_flash *fled_cdev = v4l2_flash.fled_cdev;
    struct aat1290_led *led = fled_cdev_to_led(fled_cdev);
    int i;
    for (i = AAT1290_MM_CURRENT_SCALE_SIZE - 1; i >= 0; --i)
    if (intensity >= led.mm_current_scale[i])
    return i + 1;
    return 1;
    }
    static s32 aat1290_brightness_to_intensity(struct v4l2_flash *v4l2_flash,
    enum led_brightness brightness)
    {
    struct led_classdev_flash *fled_cdev = v4l2_flash.fled_cdev;
    struct aat1290_led *led = fled_cdev_to_led(fled_cdev);
    return led.mm_current_scale[brightness - 1];
    }
    static int aat1290_led_external_strobe_set(struct v4l2_flash *v4l2_flash,
    bool enable)
    {
    struct aat1290_led *led = fled_cdev_to_led(v4l2_flash.fled_cdev);
    struct led_classdev_flash *fled_cdev = v4l2_flash.fled_cdev;
    struct led_classdev *led_cdev = &fled_cdev.led_cdev;
    struct pinctrl *pinctrl;
    gpiod_direction_output(led.gpio_fl_en, 0);
    gpiod_direction_output(led.gpio_en_set, 0);
    led.movie_mode = false;
    led_cdev.brightness = 0;
    pinctrl = devm_pinctrl_get_select(&led.pdev.dev,
    enable ? "isp" : "host");
    if (IS_ERR(pinctrl)) {
    dev_warn(&led.pdev.dev, "Unable to switch strobe source.\n");
    return PTR_ERR(pinctrl);
    }
    return 0;
    }
    static void aat1290_init_v4l2_flash_config(struct aat1290_led *led,
    struct aat1290_led_config_data *led_cfg,
    struct v4l2_flash_config *v4l2_sd_cfg)
    {
    struct led_classdev *led_cdev = &led.fled_cdev.led_cdev;
    struct led_flash_setting *s;
    strscpy(v4l2_sd_cfg.dev_name, led_cdev.dev.kobj.name,
    sizeof(v4l2_sd_cfg.dev_name));
    s = &v4l2_sd_cfg.intensity;
    s.min = led.mm_current_scale[0];
    s.max = led_cfg.max_mm_current;
    s.step = 1;
    s.val = s.max;
    v4l2_sd_cfg.has_external_strobe = led_cfg.has_external_strobe;
    }
    static const struct v4l2_flash_ops v4l2_flash_ops = {
    .external_strobe_set = aat1290_led_external_strobe_set,
    .intensity_to_led_brightness = aat1290_intensity_to_brightness,
    .led_brightness_to_intensity = aat1290_brightness_to_intensity,
    };

    static inline void aat1290_init_v4l2_flash_config(struct aat1290_led *led,
    struct aat1290_led_config_data *led_cfg,
    struct v4l2_flash_config *v4l2_sd_cfg)
    {
    }
    static const struct v4l2_flash_ops v4l2_flash_ops;

    static const struct led_flash_ops flash_ops = {
    .strobe_set = aat1290_led_flash_strobe_set,
    .timeout_set = aat1290_led_flash_timeout_set,
    };
#[no_mangle]
unsafe extern "C" fn aat1290_led_probe(pdev: *mut platform_device) -> c_int {
    static int aat1290_led_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *sub_node = core::ptr::null_mut();
    struct aat1290_led *led;
    struct led_classdev *led_cdev;
    struct led_classdev_flash *fled_cdev;
    let mut init_data: led_init_data = {};
    let mut led_cfg: aat1290_led_config_data = {};
    let mut v4l2_sd_cfg: v4l2_flash_config = {};
    int ret;
    led = devm_kzalloc(dev, sizeof(*led), GFP_KERNEL);
    if (!led)
    return -ENOMEM;
    led.pdev = pdev;
    platform_set_drvdata(pdev, led);
    fled_cdev = &led.fled_cdev;
    fled_cdev.ops = &flash_ops;
    led_cdev = &fled_cdev.led_cdev;
    ret = aat1290_led_get_configuration(led, &led_cfg, &sub_node);
    if (ret < 0)
    return ret;
    mutex_init(&led.lock);
// Initialize LED Flash class device
    led_cdev.brightness_set_blocking = aat1290_led_brightness_set;
    led_cdev.max_brightness = led_cfg.max_brightness;
    led_cdev.flags |= LED_DEV_CAP_FLASH;
    aat1290_init_flash_timeout(led, &led_cfg);
    init_data.fwnode = of_fwnode_handle(sub_node);
    init_data.devicename = AAT1290_NAME;
// Register LED Flash class device
    ret = led_classdev_flash_register_ext(&pdev.dev, fled_cdev,
    &init_data);
    if (ret < 0)
    goto err_flash_register;
    aat1290_init_v4l2_flash_config(led, &led_cfg, &v4l2_sd_cfg);
// Create V4L2 Flash subdev.
    led.v4l2_flash = v4l2_flash_init(dev, of_fwnode_handle(sub_node),
    fled_cdev, &v4l2_flash_ops,
    &v4l2_sd_cfg);
    if (IS_ERR(led.v4l2_flash)) {
    ret = PTR_ERR(led.v4l2_flash);
    goto error_v4l2_flash_init;
    }
    return 0;
    error_v4l2_flash_init:
    led_classdev_flash_unregister(fled_cdev);
    err_flash_register:
    mutex_destroy(&led.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aat1290_led_remove(pdev: *mut platform_device) {
    static void aat1290_led_remove(struct platform_device *pdev)
    {
    struct aat1290_led *led = platform_get_drvdata(pdev);
    v4l2_flash_release(led.v4l2_flash);
    led_classdev_flash_unregister(&led.fled_cdev);
    mutex_destroy(&led.lock);
    }
    static const struct of_device_id aat1290_led_dt_match[] = {
    { .compatible = "skyworks,aat1290" },
    {},
    };
    MODULE_DEVICE_TABLE(of, aat1290_led_dt_match);
    static struct platform_driver aat1290_led_driver = {
    .probe		= aat1290_led_probe,
    .remove		= aat1290_led_remove,
    .driver		= {
    .name	= "aat1290",
    .of_match_table = aat1290_led_dt_match,
    },
    };
    module_platform_driver(aat1290_led_driver);
    MODULE_AUTHOR("Jacek Anaszewski <j.anaszewski@samsung.com>");
    MODULE_DESCRIPTION("Skyworks Current Regulator for Flash LEDs");
    MODULE_LICENSE("GPL v2");
