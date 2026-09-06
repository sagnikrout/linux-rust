//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/s5k6a3.c
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
// Samsung S5K6A3 image sensor driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// Author: Sylwester Nawrocki <s.nawrocki@samsung.com>
//

pub const S5K6A3_SENSOR_MAX_WIDTH: c_int = 1412;
pub const S5K6A3_SENSOR_MAX_HEIGHT: c_int = 1412;
pub const S5K6A3_SENSOR_MIN_WIDTH: c_int = 32;
pub const S5K6A3_SENSOR_MIN_HEIGHT: c_int = 32;
pub const S5K6A3_DEFAULT_WIDTH: c_int = 1296;
pub const S5K6A3_DEFAULT_HEIGHT: c_int = 732;

    enum {
    S5K6A3_SUPP_VDDA,
    S5K6A3_SUPP_VDDIO,
    S5K6A3_SUPP_AFVDD,
    S5K6A3_NUM_SUPPLIES,
    };
//
// struct s5k6a3 - fimc-is sensor data structure
// @dev: pointer to this I2C client device structure
// @subdev: the image sensor's v4l2 subdev
// @pad: subdev media source pad
// @supplies: image sensor's voltage regulator supplies
// @gpio_reset: GPIO connected to the sensor's reset pin
// @lock: mutex protecting the structure's members below
// @format: media bus format at the sensor's source pad
// @clock: pointer to &struct clk.
// @power_count: stores state if device is powered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5k6a3 {
    pub dev: *mut device,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub supplies: [regulator_bulk_data; S5K6A3_NUM_SUPPLIES],
    pub gpio_reset: *mut gpio_desc,
    pub lock: mutex,
    pub format: v4l2_mbus_framefmt,
    pub clock: *mut clk,
    pub power_count: c_int,
}

    static const char * const s5k6a3_supply_names[] = {
    [S5K6A3_SUPP_VDDA]	= "svdda",
    [S5K6A3_SUPP_VDDIO]	= "svddio",
    [S5K6A3_SUPP_AFVDD]	= "afvdd",
    };
    static inline struct s5k6a3 *sd_to_s5k6a3(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct s5k6a3, subdev);
    }
    static const struct v4l2_mbus_framefmt s5k6a3_formats[] = {
    {
    .code = MEDIA_BUS_FMT_SGRBG10_1X10,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .field = V4L2_FIELD_NONE,
    }
    };
    static const struct v4l2_mbus_framefmt *find_sensor_format(
    struct v4l2_mbus_framefmt *mf)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(s5k6a3_formats); i++)
    if (mf.code == s5k6a3_formats[i].code)
    return &s5k6a3_formats[i];
    return &s5k6a3_formats[0];
    }
    static int s5k6a3_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index >= ARRAY_SIZE(s5k6a3_formats))
    return -EINVAL;
    code.code = s5k6a3_formats[code.index].code;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5k6a3_try_format(mf: *mut v4l2_mbus_framefmt) {
    static void s5k6a3_try_format(struct v4l2_mbus_framefmt *mf)
    {
    const struct v4l2_mbus_framefmt *fmt;
    fmt = find_sensor_format(mf);
    mf.code = fmt.code;
    mf.field = V4L2_FIELD_NONE;
    v4l_bound_align_image(&mf.width, S5K6A3_SENSOR_MIN_WIDTH,
    S5K6A3_SENSOR_MAX_WIDTH, 0,
    &mf.height, S5K6A3_SENSOR_MIN_HEIGHT,
    S5K6A3_SENSOR_MAX_HEIGHT, 0, 0);
    }
    static struct v4l2_mbus_framefmt *__s5k6a3_get_format(
    struct s5k6a3 *sensor, struct v4l2_subdev_state *sd_state,
    u32 pad, enum v4l2_subdev_format_whence which)
    {
    if (which == V4L2_SUBDEV_FORMAT_TRY)
    return sd_state ? v4l2_subdev_state_get_format(sd_state, pad) : core::ptr::null_mut();
    return &sensor.format;
    }
    static int s5k6a3_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct s5k6a3 *sensor = sd_to_s5k6a3(sd);
    struct v4l2_mbus_framefmt *mf;
    s5k6a3_try_format(&fmt.format);
    mf = __s5k6a3_get_format(sensor, sd_state, fmt.pad, fmt.which);
    if (mf) {
    mutex_lock(&sensor.lock);
// mf = fmt->format;
    mutex_unlock(&sensor.lock);
    }
    return 0;
    }
    static int s5k6a3_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct s5k6a3 *sensor = sd_to_s5k6a3(sd);
    struct v4l2_mbus_framefmt *mf;
    mf = __s5k6a3_get_format(sensor, sd_state, fmt.pad, fmt.which);
    mutex_lock(&sensor.lock);
    fmt.format = *mf;
    mutex_unlock(&sensor.lock);
    return 0;
    }
    static const struct v4l2_subdev_pad_ops s5k6a3_pad_ops = {
    .enum_mbus_code	= s5k6a3_enum_mbus_code,
    .get_fmt	= s5k6a3_get_fmt,
    .set_fmt	= s5k6a3_set_fmt,
    };
#[no_mangle]
unsafe extern "C" fn s5k6a3_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int s5k6a3_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    struct v4l2_mbus_framefmt *format = v4l2_subdev_state_get_format(fh.state,
    0);
// format		= s5k6a3_formats[0];
    format.width	= S5K6A3_DEFAULT_WIDTH;
    format.height	= S5K6A3_DEFAULT_HEIGHT;
    return 0;
    }
    static const struct v4l2_subdev_internal_ops s5k6a3_sd_internal_ops = {
    .open = s5k6a3_open,
    };
#[no_mangle]
unsafe extern "C" fn __s5k6a3_power_on(sensor: *mut s5k6a3) -> c_int {
    static int __s5k6a3_power_on(struct s5k6a3 *sensor)
    {
    let mut i: c_int = S5K6A3_SUPP_VDDA;
    int ret;
    ret = pm_runtime_get(sensor.dev);
    if (ret < 0)
    goto error_rpm_put;
    ret = regulator_enable(sensor.supplies[i].consumer);
    if (ret < 0)
    goto error_rpm_put;
    ret = clk_prepare_enable(sensor.clock);
    if (ret < 0)
    goto error_reg_dis;
    for (i++; i < S5K6A3_NUM_SUPPLIES; i++) {
    ret = regulator_enable(sensor.supplies[i].consumer);
    if (ret < 0)
    goto error_clk;
    }
    gpiod_set_value_cansleep(sensor.gpio_reset, 0);
    usleep_range(600, 800);
    gpiod_set_value_cansleep(sensor.gpio_reset, 1);
    usleep_range(600, 800);
    gpiod_set_value_cansleep(sensor.gpio_reset, 0);
// Delay needed for the sensor initialization
    msleep(20);
    return 0;
    error_clk:
    clk_disable_unprepare(sensor.clock);
    error_reg_dis:
    for (--i; i >= 0; --i)
    regulator_disable(sensor.supplies[i].consumer);
    error_rpm_put:
    pm_runtime_put(sensor.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __s5k6a3_power_off(sensor: *mut s5k6a3) -> c_int {
    static int __s5k6a3_power_off(struct s5k6a3 *sensor)
    {
    int i;
    gpiod_set_value_cansleep(sensor.gpio_reset, 1);
    for (i = S5K6A3_NUM_SUPPLIES - 1; i >= 0; i--)
    regulator_disable(sensor.supplies[i].consumer);
    clk_disable_unprepare(sensor.clock);
    pm_runtime_put(sensor.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s5k6a3_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int s5k6a3_s_power(struct v4l2_subdev *sd, int on)
    {
    struct s5k6a3 *sensor = sd_to_s5k6a3(sd);
    let mut ret: c_int = 0;
    mutex_lock(&sensor.lock);
    if (sensor.power_count == !on) {
    if (on)
    ret = __s5k6a3_power_on(sensor);
    else
    ret = __s5k6a3_power_off(sensor);
    if (ret == 0)
    sensor.power_count += on ? 1 : -1;
    }
    mutex_unlock(&sensor.lock);
    return ret;
    }
    static const struct v4l2_subdev_core_ops s5k6a3_core_ops = {
    .s_power = s5k6a3_s_power,
    };
    static const struct v4l2_subdev_ops s5k6a3_subdev_ops = {
    .core = &s5k6a3_core_ops,
    .pad = &s5k6a3_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn s5k6a3_probe(client: *mut i2c_client) -> c_int {
    static int s5k6a3_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct s5k6a3 *sensor;
    struct v4l2_subdev *sd;
    int i, ret;
    sensor = devm_kzalloc(dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    mutex_init(&sensor.lock);
    sensor.dev = dev;
    sensor.clock = devm_v4l2_sensor_clk_get_legacy(sensor.dev,
    S5K6A3_CLK_NAME, false,
    S5K6A3_DEFAULT_CLK_FREQ);
    if (IS_ERR(sensor.clock))
    return dev_err_probe(sensor.dev, PTR_ERR(sensor.clock),
    "failed to get extclk\n");
    sensor.gpio_reset = devm_gpiod_get(dev, core::ptr::null_mut(), GPIOD_OUT_HIGH);
    ret = PTR_ERR_OR_ZERO(sensor.gpio_reset);
    if (ret)
    return ret;
    for (i = 0; i < S5K6A3_NUM_SUPPLIES; i++)
    sensor.supplies[i].supply = s5k6a3_supply_names[i];
    ret = devm_regulator_bulk_get(&client.dev, S5K6A3_NUM_SUPPLIES,
    sensor.supplies);
    if (ret < 0)
    return ret;
    sd = &sensor.subdev;
    v4l2_i2c_subdev_init(sd, client, &s5k6a3_subdev_ops);
    sensor.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    sd.internal_ops = &s5k6a3_sd_internal_ops;
    sensor.format.code = s5k6a3_formats[0].code;
    sensor.format.width = S5K6A3_DEFAULT_WIDTH;
    sensor.format.height = S5K6A3_DEFAULT_HEIGHT;
    sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    sensor.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&sd.entity, 1, &sensor.pad);
    if (ret < 0)
    return ret;
    pm_runtime_no_callbacks(dev);
    pm_runtime_enable(dev);
    ret = v4l2_async_register_subdev(sd);
    if (ret < 0) {
    pm_runtime_disable(&client.dev);
    media_entity_cleanup(&sd.entity);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s5k6a3_remove(client: *mut i2c_client) {
    static void s5k6a3_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    pm_runtime_disable(&client.dev);
    v4l2_async_unregister_subdev(sd);
    media_entity_cleanup(&sd.entity);
    }
    static const struct i2c_device_id s5k6a3_ids[] = {
    { }
    };
    MODULE_DEVICE_TABLE(i2c, s5k6a3_ids);

    static const struct of_device_id s5k6a3_of_match[] = {
    { .compatible = "samsung,s5k6a3" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, s5k6a3_of_match);

    static struct i2c_driver s5k6a3_driver = {
    .driver = {
    .of_match_table	= of_match_ptr(s5k6a3_of_match),
    .name		= S5K6A3_DRV_NAME,
    },
    .probe		= s5k6a3_probe,
    .remove		= s5k6a3_remove,
    .id_table	= s5k6a3_ids,
    };
    module_i2c_driver(s5k6a3_driver);
    MODULE_DESCRIPTION("S5K6A3 image sensor subdev driver");
    MODULE_AUTHOR("Sylwester Nawrocki <s.nawrocki@samsung.com>");
    MODULE_LICENSE("GPL v2");
