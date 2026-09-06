//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ad5820.c
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
// drivers/media/i2c/ad5820.c
//
// AD5820 DAC driver for camera voice coil focus.
//
// Copyright (C) 2008 Nokia Corporation
// Copyright (C) 2007 Texas Instruments
// Copyright (C) 2016 Pavel Machek <pavel@ucw.cz>
//
// Contact: Tuukka Toivonen <tuukkat76@gmail.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
// Based on af_d88.c by Texas Instruments.
//

// Register definitions

pub const AD5820_DAC_SHIFT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad5820_device {
    pub subdev: v4l2_subdev,
    pub platform_data: *mut ad5820_platform_data,
    pub vana: *mut regulator,
    pub ctrls: v4l2_ctrl_handler,
    pub focus_absolute: u32,
    pub focus_ramp_time: u32,
    pub focus_ramp_mode: u32,
    pub enable_gpio: *mut gpio_desc,
    pub power_lock: mutex,
    pub power_count: c_int,
    pub standby: bool,
}

#[no_mangle]
unsafe extern "C" fn ad5820_write(coil: *mut ad5820_device, data: u16) -> c_int {
    static int ad5820_write(struct ad5820_device *coil, u16 data)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&coil.subdev);
    struct i2c_msg msg;
    __be16 be_data;
    int r;
    if (!client.adapter)
    return -ENODEV;
    be_data = cpu_to_be16(data);
    msg.addr  = client.addr;
    msg.flags = 0;
    msg.len   = 2;
    msg.buf   = (u8 *)&be_data;
    r = i2c_transfer(client.adapter, &msg, 1);
    if (r < 0) {
    dev_err(&client.dev, "write failed, error %d\n", r);
    return r;
    }
    return 0;
    }
//
// Calculate status word and write it to the device based on current
// values of V4L2 controls. It is assumed that the stored V4L2 control
// values are properly limited and rounded.
//
#[no_mangle]
unsafe extern "C" fn ad5820_update_hw(coil: *mut ad5820_device) -> c_int {
    static int ad5820_update_hw(struct ad5820_device *coil)
    {
    u16 status;
    status = RAMP_US_TO_CODE(coil.focus_ramp_time);
    status |= coil.focus_ramp_mode
    ? AD5820_RAMP_MODE_64_16 : AD5820_RAMP_MODE_LINEAR;
    status |= coil.focus_absolute << AD5820_DAC_SHIFT;
    if (coil.standby)
    status |= AD5820_POWER_DOWN;
    return ad5820_write(coil, status);
    }
//
// Power handling
//
#[no_mangle]
unsafe extern "C" fn ad5820_power_off(coil: *mut ad5820_device, standby: bool) -> c_int {
    static int ad5820_power_off(struct ad5820_device *coil, bool standby)
    {
    let mut ret: c_int = 0, ret2;
//
// Go to standby first as real power off my be denied by the hardware
// (single power line control for both coil and sensor).
//
    if (standby) {
    coil.standby = true;
    ret = ad5820_update_hw(coil);
    }
    gpiod_set_value_cansleep(coil.enable_gpio, 0);
    ret2 = regulator_disable(coil.vana);
    if (ret)
    return ret;
    return ret2;
    }
#[no_mangle]
unsafe extern "C" fn ad5820_power_on(coil: *mut ad5820_device, restore: bool) -> c_int {
    static int ad5820_power_on(struct ad5820_device *coil, bool restore)
    {
    int ret;
    ret = regulator_enable(coil.vana);
    if (ret < 0)
    return ret;
    gpiod_set_value_cansleep(coil.enable_gpio, 1);
    if (restore) {
// Restore the hardware settings.
    coil.standby = false;
    ret = ad5820_update_hw(coil);
    if (ret)
    goto fail;
    }
    return 0;
    fail:
    gpiod_set_value_cansleep(coil.enable_gpio, 0);
    coil.standby = true;
    regulator_disable(coil.vana);
    return ret;
    }
//
// V4L2 controls
//
#[no_mangle]
unsafe extern "C" fn ad5820_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ad5820_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ad5820_device *coil =
    container_of(ctrl.handler, struct ad5820_device, ctrls);
    switch (ctrl.id) {
    case V4L2_CID_FOCUS_ABSOLUTE:
    coil.focus_absolute = ctrl.val;
    return ad5820_update_hw(coil);
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops ad5820_ctrl_ops = {
    .s_ctrl = ad5820_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ad5820_init_controls(coil: *mut ad5820_device) -> c_int {
    static int ad5820_init_controls(struct ad5820_device *coil)
    {
    v4l2_ctrl_handler_init(&coil.ctrls, 1);
//
// V4L2_CID_FOCUS_ABSOLUTE
//
// Minimum current is 0 mA, maximum is 100 mA. Thus, 1 code is
// equivalent to 100/1023 = 0.0978 mA. Nevertheless, we do not use [mA]
// for focus position, because it is meaningless for user. Meaningful
// would be to use focus distance or even its inverse, but since the
// driver doesn't have sufficiently knowledge to do the conversion, we
// will just use abstract codes here. In any case, smaller value = focus
// position farther from camera. The default zero value means focus at
// infinity, and also least current consumption.
//
    v4l2_ctrl_new_std(&coil.ctrls, &ad5820_ctrl_ops,
    V4L2_CID_FOCUS_ABSOLUTE, 0, 1023, 1, 0);
    if (coil.ctrls.error)
    return coil.ctrls.error;
    coil.focus_absolute = 0;
    coil.focus_ramp_time = 0;
    coil.focus_ramp_mode = 0;
    coil.subdev.ctrl_handler = &coil.ctrls;
    return 0;
    }
//
// V4L2 subdev operations
//
#[no_mangle]
unsafe extern "C" fn ad5820_registered(subdev: *mut v4l2_subdev) -> c_int {
    static int ad5820_registered(struct v4l2_subdev *subdev)
    {
    struct ad5820_device *coil = to_ad5820_device(subdev);
    return ad5820_init_controls(coil);
    }
    static int
    ad5820_set_power(struct v4l2_subdev *subdev, int on)
    {
    struct ad5820_device *coil = to_ad5820_device(subdev);
    let mut ret: c_int = 0;
    mutex_lock(&coil.power_lock);
//
// If the power count is modified from 0 to != 0 or from != 0 to 0,
// update the power state.
//
    if (coil.power_count == !on) {
    ret = on ? ad5820_power_on(coil, true) :
    ad5820_power_off(coil, true);
    if (ret < 0)
    goto done;
    }
// Update the power count.
    coil.power_count += on ? 1 : -1;
    WARN_ON(coil.power_count < 0);
    done:
    mutex_unlock(&coil.power_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5820_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ad5820_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    return ad5820_set_power(sd, 1);
    }
#[no_mangle]
unsafe extern "C" fn ad5820_close(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ad5820_close(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    return ad5820_set_power(sd, 0);
    }
    static const struct v4l2_subdev_core_ops ad5820_core_ops = {
    .s_power = ad5820_set_power,
    };
    static const struct v4l2_subdev_ops ad5820_ops = {
    .core = &ad5820_core_ops,
    };
    static const struct v4l2_subdev_internal_ops ad5820_internal_ops = {
    .registered = ad5820_registered,
    .open = ad5820_open,
    .close = ad5820_close,
    };
//
// I2C driver
//
#[no_mangle]
unsafe extern "C" fn ad5820_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ad5820_suspend(struct device *dev)
    {
    struct v4l2_subdev *subdev = dev_get_drvdata(dev);
    struct ad5820_device *coil = to_ad5820_device(subdev);
    if (!coil.power_count)
    return 0;
    return ad5820_power_off(coil, false);
    }
#[no_mangle]
unsafe extern "C" fn ad5820_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ad5820_resume(struct device *dev)
    {
    struct v4l2_subdev *subdev = dev_get_drvdata(dev);
    struct ad5820_device *coil = to_ad5820_device(subdev);
    if (!coil.power_count)
    return 0;
    return ad5820_power_on(coil, true);
    }
#[no_mangle]
unsafe extern "C" fn ad5820_probe(client: *mut i2c_client) -> c_int {
    static int ad5820_probe(struct i2c_client *client)
    {
    struct ad5820_device *coil;
    int ret;
    coil = devm_kzalloc(&client.dev, sizeof(*coil), GFP_KERNEL);
    if (!coil)
    return -ENOMEM;
    coil.vana = devm_regulator_get(&client.dev, "VANA");
    if (IS_ERR(coil.vana))
    return dev_err_probe(&client.dev, PTR_ERR(coil.vana),
    "could not get regulator for vana\n");
    coil.enable_gpio = devm_gpiod_get_optional(&client.dev, "enable",
    GPIOD_OUT_LOW);
    if (IS_ERR(coil.enable_gpio))
    return dev_err_probe(&client.dev, PTR_ERR(coil.enable_gpio),
    "could not get enable gpio\n");
    mutex_init(&coil.power_lock);
    v4l2_i2c_subdev_init(&coil.subdev, client, &ad5820_ops);
    coil.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    coil.subdev.internal_ops = &ad5820_internal_ops;
    coil.subdev.entity.function = MEDIA_ENT_F_LENS;
    strscpy(coil.subdev.name, "ad5820 focus", sizeof(coil.subdev.name));
    ret = media_entity_pads_init(&coil.subdev.entity, 0, core::ptr::null_mut());
    if (ret < 0)
    goto clean_mutex;
    ret = v4l2_async_register_subdev(&coil.subdev);
    if (ret < 0)
    goto clean_entity;
    return ret;
    clean_entity:
    media_entity_cleanup(&coil.subdev.entity);
    clean_mutex:
    mutex_destroy(&coil.power_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ad5820_remove(client: *mut i2c_client) {
    static void ad5820_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct ad5820_device *coil = to_ad5820_device(subdev);
    v4l2_async_unregister_subdev(&coil.subdev);
    v4l2_ctrl_handler_free(&coil.ctrls);
    media_entity_cleanup(&coil.subdev.entity);
    mutex_destroy(&coil.power_lock);
    }
    static const struct i2c_device_id ad5820_id_table[] = {
    { .name = "ad5820" },
    { .name = "ad5821" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad5820_id_table);
    static const struct of_device_id ad5820_of_table[] = {
    { .compatible = "adi,ad5820" },
    { .compatible = "adi,ad5821" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad5820_of_table);
    static SIMPLE_DEV_PM_OPS(ad5820_pm, ad5820_suspend, ad5820_resume);
    static struct i2c_driver ad5820_i2c_driver = {
    .driver		= {
    .name	= "ad5820",
    .pm	= &ad5820_pm,
    .of_match_table = ad5820_of_table,
    },
    .probe		= ad5820_probe,
    .remove		= ad5820_remove,
    .id_table	= ad5820_id_table,
    };
    module_i2c_driver(ad5820_i2c_driver);
    MODULE_AUTHOR("Tuukka Toivonen");
    MODULE_DESCRIPTION("AD5820 camera lens driver");
    MODULE_LICENSE("GPL");
