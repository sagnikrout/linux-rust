//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/ktd253-backlight.c
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
// Backlight driver for the Kinetic KTD253
// Based on code and know-how from the Samsung GT-S7710
// Gareth Phillips <gareth.phillips@samsung.com>
//

// Current ratio is n/32 from 1/32 to 32/32
pub const KTD253_MIN_RATIO: c_int = 1;
pub const KTD253_MAX_RATIO: c_int = 32;
pub const KTD253_DEFAULT_RATIO: c_int = 13;

pub const KTD253_T_OFF_MS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktd253_backlight {
    pub dev: *mut device,
    pub bl: *mut backlight_device,
    pub gpiod: *mut gpio_desc,
    pub ratio: u16,
}

#[no_mangle]
unsafe extern "C" fn ktd253_backlight_set_max_ratio(ktd253: *mut ktd253_backlight) {
    static void ktd253_backlight_set_max_ratio(struct ktd253_backlight *ktd253)
    {
    gpiod_set_value_cansleep(ktd253.gpiod, 1);
    ndelay(KTD253_T_HIGH_NS);
// We always fall back to this when we power on
    }
#[no_mangle]
unsafe extern "C" fn ktd253_backlight_stepdown(ktd253: *mut ktd253_backlight) -> c_int {
    static int ktd253_backlight_stepdown(struct ktd253_backlight *ktd253)
    {
//
// These GPIO operations absolutely can NOT sleep so no _cansleep
// suffixes, and no using GPIO expanders on slow buses for this!
//
// The maximum number of cycles of the loop is 32  so the time taken
// should nominally be:
// (T_LOW_NS + T_HIGH_NS + loop_time) * 32
//
// Architectures do not always support ndelay() and we will get a few us
// instead. If we get to a critical time limit an interrupt has likely
// occured in the low part of the loop and we need to restart from the
// top so we have the backlight in a known state.
//
    u64 ns;
    ns = ktime_get_ns();
    gpiod_set_value(ktd253.gpiod, 0);
    ndelay(KTD253_T_LOW_NS);
    gpiod_set_value(ktd253.gpiod, 1);
    ns = ktime_get_ns() - ns;
    if (ns >= KTD253_T_OFF_CRIT_NS) {
    dev_err(ktd253.dev, "PCM on backlight took too long (%llu ns)\n", ns);
    return -EAGAIN;
    }
    ndelay(KTD253_T_HIGH_NS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ktd253_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int ktd253_backlight_update_status(struct backlight_device *bl)
    {
    struct ktd253_backlight *ktd253 = bl_get_data(bl);
    let mut brightness: c_int = backlight_get_brightness(bl);
    u16 target_ratio;
    let mut current_ratio: u16 = ktd253.ratio;
    int ret;
    dev_dbg(ktd253.dev, "new brightness/ratio: %d/32\n", brightness);
    target_ratio = brightness;
    if (target_ratio == current_ratio)
// This is already right
    return 0;
    if (target_ratio == 0) {
    gpiod_set_value_cansleep(ktd253.gpiod, 0);
//
// We need to keep the GPIO low for at least this long
// to actually switch the KTD253 off.
//
    msleep(KTD253_T_OFF_MS);
    ktd253.ratio = 0;
    return 0;
    }
    if (current_ratio == 0) {
    ktd253_backlight_set_max_ratio(ktd253);
    current_ratio = KTD253_MAX_RATIO;
    }
    while (current_ratio != target_ratio) {
//
// These GPIO operations absolutely can NOT sleep so no
// _cansleep suffixes, and no using GPIO expanders on
// slow buses for this!
//
    ret = ktd253_backlight_stepdown(ktd253);
    if (ret == -EAGAIN) {
//
// Something disturbed the backlight setting code when
// running so we need to bring the PWM back to a known
// state. This shouldn't happen too much.
//
    gpiod_set_value_cansleep(ktd253.gpiod, 0);
    msleep(KTD253_T_OFF_MS);
    ktd253_backlight_set_max_ratio(ktd253);
    current_ratio = KTD253_MAX_RATIO;
    } else if (current_ratio == KTD253_MIN_RATIO) {
// After 1/32 we loop back to 32/32
    current_ratio = KTD253_MAX_RATIO;
    } else {
    current_ratio--;
    }
    }
    ktd253.ratio = current_ratio;
    dev_dbg(ktd253.dev, "new ratio set to %d/32\n", target_ratio);
    return 0;
    }
    static const struct backlight_ops ktd253_backlight_ops = {
    .options	= BL_CORE_SUSPENDRESUME,
    .update_status	= ktd253_backlight_update_status,
    };
#[no_mangle]
unsafe extern "C" fn ktd253_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int ktd253_backlight_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct backlight_device *bl;
    struct ktd253_backlight *ktd253;
    u32 max_brightness;
    u32 brightness;
    int ret;
    ktd253 = devm_kzalloc(dev, sizeof(*ktd253), GFP_KERNEL);
    if (!ktd253)
    return -ENOMEM;
    ktd253.dev = dev;
    ret = device_property_read_u32(dev, "max-brightness", &max_brightness);
    if (ret)
    max_brightness = KTD253_MAX_RATIO;
    if (max_brightness > KTD253_MAX_RATIO) {
// Clamp brightness to hardware max
    dev_err(dev, "illegal max brightness specified\n");
    max_brightness = KTD253_MAX_RATIO;
    }
    ret = device_property_read_u32(dev, "default-brightness", &brightness);
    if (ret)
    brightness = KTD253_DEFAULT_RATIO;
    if (brightness > max_brightness) {
// Clamp default brightness to max brightness
    dev_err(dev, "default brightness exceeds max brightness\n");
    brightness = max_brightness;
    }
    ktd253.gpiod = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(ktd253.gpiod))
    return dev_err_probe(dev, PTR_ERR(ktd253.gpiod),
    "gpio line missing or invalid.\n");
    gpiod_set_consumer_name(ktd253.gpiod, dev_name(dev));
// Bring backlight to a known off state
    msleep(KTD253_T_OFF_MS);
    bl = devm_backlight_device_register(dev, dev_name(dev), dev, ktd253,
    &ktd253_backlight_ops, core::ptr::null_mut());
    if (IS_ERR(bl)) {
    dev_err(dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    bl.props.max_brightness = max_brightness;
// When we just enable the GPIO line we set max brightness
    if (brightness) {
    bl.props.brightness = brightness;
    bl.props.power = BACKLIGHT_POWER_ON;
    } else {
    bl.props.brightness = 0;
    bl.props.power = BACKLIGHT_POWER_OFF;
    }
    ktd253.bl = bl;
    platform_set_drvdata(pdev, bl);
    backlight_update_status(bl);
    return 0;
    }
    static const struct of_device_id ktd253_backlight_of_match[] = {
    { .compatible = "kinetic,ktd253" },
    { .compatible = "kinetic,ktd259" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ktd253_backlight_of_match);
    static struct platform_driver ktd253_backlight_driver = {
    .driver = {
    .name = "ktd253-backlight",
    .of_match_table = ktd253_backlight_of_match,
    },
    .probe		= ktd253_backlight_probe,
    };
    module_platform_driver(ktd253_backlight_driver);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("Kinetic KTD253 Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:ktd253-backlight");
