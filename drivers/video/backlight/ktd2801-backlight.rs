//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ktd2801-backlight.c
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
// Datasheet:
// https://www.kinet-ic.com/uploads/web/KTD2801/KTD2801-04b.pdf
//

pub const KTD2801_DEFAULT_BRIGHTNESS: c_int = 100;
pub const KTD2801_MAX_BRIGHTNESS: c_int = 255;
// These values have been extracted from Samsung's driver.
    static const struct expresswire_timing ktd2801_timing = {
    .poweroff_us = 2600,
    .detect_delay_us = 150,
    .detect_us = 270,
    .data_start_us = 5,
    .short_bitset_us = 5,
    .long_bitset_us = 15,
    .end_of_data_low_us = 10,
    .end_of_data_high_us = 350
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd2801_backlight {
    pub props: expresswire_common_props,
    pub bd: *mut backlight_device,
    pub was_on: bool,
}

#[no_mangle]
unsafe extern "C" fn ktd2801_update_status(bd: *mut backlight_device) -> c_int {
    static int ktd2801_update_status(struct backlight_device *bd)
    {
    struct ktd2801_backlight *ktd2801 = bl_get_data(bd);
    let mut brightness: u8 = (u8) backlight_get_brightness(bd);
    if (backlight_is_blank(bd)) {
    expresswire_power_off(&ktd2801.props);
    ktd2801.was_on = false;
    return 0;
    }
    if (!ktd2801.was_on) {
    expresswire_enable(&ktd2801.props);
    ktd2801.was_on = true;
    }
    expresswire_write_u8(&ktd2801.props, brightness);
    return 0;
    }
    static const struct backlight_ops ktd2801_backlight_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = ktd2801_update_status,
    };
#[no_mangle]
unsafe extern "C" fn ktd2801_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int ktd2801_backlight_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct backlight_device *bd;
    struct ktd2801_backlight *ktd2801;
    u32 brightness, max_brightness;
    int ret;
    ktd2801 = devm_kzalloc(dev, sizeof(*ktd2801), GFP_KERNEL);
    if (!ktd2801)
    return -ENOMEM;
    ktd2801.was_on = true;
    ktd2801.props.timing = ktd2801_timing;
    ret = device_property_read_u32(dev, "max-brightness", &max_brightness);
    if (ret)
    max_brightness = KTD2801_MAX_BRIGHTNESS;
    if (max_brightness > KTD2801_MAX_BRIGHTNESS) {
    dev_err(dev, "illegal max brightness specified\n");
    max_brightness = KTD2801_MAX_BRIGHTNESS;
    }
    ret = device_property_read_u32(dev, "default-brightness", &brightness);
    if (ret)
    brightness = KTD2801_DEFAULT_BRIGHTNESS;
    if (brightness > max_brightness) {
    dev_err(dev, "default brightness exceeds max\n");
    brightness = max_brightness;
    }
    ktd2801.props.ctrl_gpio = devm_gpiod_get(dev, "ctrl", GPIOD_OUT_HIGH);
    if (IS_ERR(ktd2801.props.ctrl_gpio))
    return dev_err_probe(dev, PTR_ERR(ktd2801.props.ctrl_gpio),
    "failed to get backlight GPIO");
    gpiod_set_consumer_name(ktd2801.props.ctrl_gpio, dev_name(dev));
    bd = devm_backlight_device_register(dev, dev_name(dev), dev, ktd2801,
    &ktd2801_backlight_ops, core::ptr::null_mut());
    if (IS_ERR(bd))
    return dev_err_probe(dev, PTR_ERR(bd),
    "failed to register backlight");
    bd.props.max_brightness = max_brightness;
    bd.props.brightness = brightness;
    ktd2801.bd = bd;
    platform_set_drvdata(pdev, bd);
    backlight_update_status(bd);
    return 0;
    }
    static const struct of_device_id ktd2801_of_match[] = {
    { .compatible = "kinetic,ktd2801" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ktd2801_of_match);
    static struct platform_driver ktd2801_backlight_driver = {
    .driver = {
    .name = "ktd2801-backlight",
    .of_match_table = ktd2801_of_match,
    },
    .probe = ktd2801_backlight_probe,
    };
    module_platform_driver(ktd2801_backlight_driver);
    MODULE_IMPORT_NS("EXPRESSWIRE");
    MODULE_AUTHOR("Duje Mihanović <duje.mihanovic@skole.hr>");
    MODULE_DESCRIPTION("Kinetic KTD2801 Backlight Driver");
    MODULE_LICENSE("GPL");
