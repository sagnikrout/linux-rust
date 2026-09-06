//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/gpio_backlight.c
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
// gpio_backlight.c - Simple GPIO-controlled backlight
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_backlight {
    pub dev: *mut device,
    pub gpiod: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn gpio_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int gpio_backlight_update_status(struct backlight_device *bl)
    {
    struct gpio_backlight *gbl = bl_get_data(bl);
    gpiod_set_value_cansleep(gbl.gpiod, backlight_get_brightness(bl));
    return 0;
    }
    static bool gpio_backlight_controls_device(struct backlight_device *bl,
    struct device *display_dev)
    {
    struct gpio_backlight *gbl = bl_get_data(bl);
    return !gbl.dev || gbl.dev == display_dev;
    }
    static const struct backlight_ops gpio_backlight_ops = {
    .options	 = BL_CORE_SUSPENDRESUME,
    .update_status	 = gpio_backlight_update_status,
    .controls_device = gpio_backlight_controls_device,
    };
#[no_mangle]
unsafe extern "C" fn gpio_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_backlight_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_backlight_platform_data *pdata = dev_get_platdata(dev);
    struct device_node *of_node = dev.of_node;
    struct backlight_properties props;
    struct backlight_device *bl;
    struct gpio_backlight *gbl;
    int ret, init_brightness, def_value;
    gbl = devm_kzalloc(dev, sizeof(*gbl), GFP_KERNEL);
    if (gbl == core::ptr::null_mut())
    return -ENOMEM;
    if (pdata)
    gbl.dev = pdata.dev;
    def_value = device_property_read_bool(dev, "default-on");
    gbl.gpiod = devm_gpiod_get(dev, core::ptr::null_mut(), GPIOD_ASIS);
    if (IS_ERR(gbl.gpiod))
    return dev_err_probe(dev, PTR_ERR(gbl.gpiod),
    "The gpios parameter is missing or invalid\n");
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 1;
    bl = devm_backlight_device_register(dev, dev_name(dev), dev, gbl,
    &gpio_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
// Set the initial power state
    if (!of_node || !of_node.phandle)
// Not booted with device tree or no phandle link to the node
    bl.props.power = def_value ? BACKLIGHT_POWER_ON
    : BACKLIGHT_POWER_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(0: gpiod_get_value_cansleep(gbl->gpiod) ==) -> else {
    else if (gpiod_get_value_cansleep(gbl.gpiod) == 0)
    bl.props.power = BACKLIGHT_POWER_OFF;
    else
    bl.props.power = BACKLIGHT_POWER_ON;
    bl.props.brightness = 1;
    init_brightness = backlight_get_brightness(bl);
    ret = gpiod_direction_output(gbl.gpiod, init_brightness);
    if (ret) {
    dev_err(dev, "failed to set initial brightness\n");
    return ret;
    }
    platform_set_drvdata(pdev, bl);
    return 0;
    }
    static struct of_device_id gpio_backlight_of_match[] = {
    { .compatible = "gpio-backlight" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, gpio_backlight_of_match);
    static struct platform_driver gpio_backlight_driver = {
    .driver		= {
    .name		= "gpio-backlight",
    .of_match_table = gpio_backlight_of_match,
    },
    .probe		= gpio_backlight_probe,
    };
    module_platform_driver(gpio_backlight_driver);
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_DESCRIPTION("GPIO-based Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:gpio-backlight");
