//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/dw9714.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2015--2017 Intel Corporation.

pub const DW9714_MAX_FOCUS_POS: c_int = 1023;
//
// This sets the minimum granularity for the focus positions.
// A value of 1 gives maximum accuracy for a desired focus position
//
pub const DW9714_FOCUS_STEPS: c_int = 1;
//
// This acts as the minimum granularity of lens movement.
// Keep this value power of 2, so the control steps can be
// uniformly adjusted for gradual lens movement, with desired
// number of control steps.
//
pub const DW9714_CTRL_STEPS: c_int = 16;
pub const DW9714_CTRL_DELAY_US: c_int = 1000;
//
// S[3:2] = 0x00, codes per step for "Linear Slope Control"
// S[1:0] = 0x00, step period
//
pub const DW9714_DEFAULT_S: c_uint = 0x0;

// dw9714 device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw9714_device {
    pub ctrls_vcm: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub current_val: u16,
    pub vcc: *mut regulator,
    pub powerdown_gpio: *mut gpio_desc,
}

    static inline struct dw9714_device *to_dw9714_vcm(struct v4l2_ctrl *ctrl)
    {
    return container_of(ctrl.handler, struct dw9714_device, ctrls_vcm);
    }
    static inline struct dw9714_device *sd_to_dw9714_vcm(struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct dw9714_device, sd);
    }
#[no_mangle]
unsafe extern "C" fn dw9714_i2c_write(client: *mut i2c_client, data: u16) -> c_int {
    static int dw9714_i2c_write(struct i2c_client *client, u16 data)
    {
    int ret;
    let mut val: __be16 = cpu_to_be16(data);
    ret = i2c_master_send(client, (const char *)&val, sizeof(val));
    if (ret != sizeof(val)) {
    dev_err(&client.dev, "I2C write fail\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw9714_t_focus_vcm(dw9714_dev: *mut dw9714_device, val: u16) -> c_int {
    static int dw9714_t_focus_vcm(struct dw9714_device *dw9714_dev, u16 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&dw9714_dev.sd);
    dw9714_dev.current_val = val;
    return dw9714_i2c_write(client, DW9714_VAL(val, DW9714_DEFAULT_S));
    }
#[no_mangle]
unsafe extern "C" fn dw9714_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int dw9714_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct dw9714_device *dev_vcm = to_dw9714_vcm(ctrl);
    if (ctrl.id == V4L2_CID_FOCUS_ABSOLUTE)
    return dw9714_t_focus_vcm(dev_vcm, ctrl.val);
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops dw9714_vcm_ctrl_ops = {
    .s_ctrl = dw9714_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn dw9714_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int dw9714_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    return pm_runtime_resume_and_get(sd.dev);
    }
#[no_mangle]
unsafe extern "C" fn dw9714_close(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int dw9714_close(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    pm_runtime_put(sd.dev);
    return 0;
    }
    static const struct v4l2_subdev_internal_ops dw9714_int_ops = {
    .open = dw9714_open,
    .close = dw9714_close,
    };
    static const struct v4l2_subdev_core_ops dw9714_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
    static const struct v4l2_subdev_ops dw9714_ops = {
    .core = &dw9714_core_ops,
    };
#[no_mangle]
unsafe extern "C" fn dw9714_subdev_cleanup(dw9714_dev: *mut dw9714_device) {
    static void dw9714_subdev_cleanup(struct dw9714_device *dw9714_dev)
    {
    v4l2_async_unregister_subdev(&dw9714_dev.sd);
    v4l2_ctrl_handler_free(&dw9714_dev.ctrls_vcm);
    media_entity_cleanup(&dw9714_dev.sd.entity);
    }
#[no_mangle]
unsafe extern "C" fn dw9714_init_controls(dev_vcm: *mut dw9714_device) -> c_int {
    static int dw9714_init_controls(struct dw9714_device *dev_vcm)
    {
    struct v4l2_ctrl_handler *hdl = &dev_vcm.ctrls_vcm;
    const struct v4l2_ctrl_ops *ops = &dw9714_vcm_ctrl_ops;
    v4l2_ctrl_handler_init(hdl, 1);
    v4l2_ctrl_new_std(hdl, ops, V4L2_CID_FOCUS_ABSOLUTE,
    0, DW9714_MAX_FOCUS_POS, DW9714_FOCUS_STEPS, 0);
    if (hdl.error)
    dev_err(dev_vcm.sd.dev, "%s fail error: 0x%x\n",
    __func__, hdl.error);
    dev_vcm.sd.ctrl_handler = hdl;
    return hdl.error;
    }
#[no_mangle]
unsafe extern "C" fn dw9714_power_up(dw9714_dev: *mut dw9714_device) -> c_int {
    static int dw9714_power_up(struct dw9714_device *dw9714_dev)
    {
    int ret;
    ret = regulator_enable(dw9714_dev.vcc);
    if (ret)
    return ret;
    gpiod_set_value_cansleep(dw9714_dev.powerdown_gpio, 0);
    usleep_range(12000, 14000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw9714_power_down(dw9714_dev: *mut dw9714_device) -> c_int {
    static int dw9714_power_down(struct dw9714_device *dw9714_dev)
    {
    gpiod_set_value_cansleep(dw9714_dev.powerdown_gpio, 1);
    return regulator_disable(dw9714_dev.vcc);
    }
#[no_mangle]
unsafe extern "C" fn dw9714_probe(client: *mut i2c_client) -> c_int {
    static int dw9714_probe(struct i2c_client *client)
    {
    struct dw9714_device *dw9714_dev;
    int rval;
    dw9714_dev = devm_kzalloc(&client.dev, sizeof(*dw9714_dev),
    GFP_KERNEL);
    if (!dw9714_dev)
    return -ENOMEM;
    dw9714_dev.vcc = devm_regulator_get(&client.dev, "vcc");
    if (IS_ERR(dw9714_dev.vcc))
    return PTR_ERR(dw9714_dev.vcc);
    dw9714_dev.powerdown_gpio = devm_gpiod_get_optional(&client.dev,
    "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(dw9714_dev.powerdown_gpio))
    return dev_err_probe(&client.dev,
    PTR_ERR(dw9714_dev.powerdown_gpio),
    "could not get powerdown gpio\n");
    rval = dw9714_power_up(dw9714_dev);
    if (rval)
    return dev_err_probe(&client.dev, rval,
    "failed to power up: %d\n", rval);
    v4l2_i2c_subdev_init(&dw9714_dev.sd, client, &dw9714_ops);
    dw9714_dev.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    dw9714_dev.sd.internal_ops = &dw9714_int_ops;
    rval = dw9714_init_controls(dw9714_dev);
    if (rval)
    goto err_cleanup;
    rval = media_entity_pads_init(&dw9714_dev.sd.entity, 0, core::ptr::null_mut());
    if (rval < 0)
    goto err_cleanup;
    dw9714_dev.sd.entity.function = MEDIA_ENT_F_LENS;
    rval = v4l2_async_register_subdev(&dw9714_dev.sd);
    if (rval < 0)
    goto err_cleanup;
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
    pm_runtime_idle(&client.dev);
    return 0;
    err_cleanup:
    dw9714_power_down(dw9714_dev);
    v4l2_ctrl_handler_free(&dw9714_dev.ctrls_vcm);
    media_entity_cleanup(&dw9714_dev.sd.entity);
    return rval;
    }
#[no_mangle]
unsafe extern "C" fn dw9714_remove(client: *mut i2c_client) {
    static void dw9714_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9714_device *dw9714_dev = sd_to_dw9714_vcm(sd);
    int ret;
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev)) {
    ret = dw9714_power_down(dw9714_dev);
    if (ret) {
    dev_err(&client.dev,
    "Failed to power down: %d\n", ret);
    }
    }
    pm_runtime_set_suspended(&client.dev);
    dw9714_subdev_cleanup(dw9714_dev);
    }
//
// This function sets the vcm position, so it consumes least current
// The lens position is gradually moved in units of DW9714_CTRL_STEPS,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn dw9714_vcm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dw9714_vcm_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9714_device *dw9714_dev = sd_to_dw9714_vcm(sd);
    int ret, val;
    if (pm_runtime_suspended(&client.dev))
    return 0;
    for (val = dw9714_dev.current_val & ~(DW9714_CTRL_STEPS - 1);
    val >= 0; val -= DW9714_CTRL_STEPS) {
    ret = dw9714_i2c_write(client,
    DW9714_VAL(val, DW9714_DEFAULT_S));
    if (ret)
    dev_err_once(dev, "%s I2C failure: %d", __func__, ret);
    usleep_range(DW9714_CTRL_DELAY_US, DW9714_CTRL_DELAY_US + 10);
    }
    ret = dw9714_power_down(dw9714_dev);
    if (ret)
    dev_err(dev, "Failed to power down: %d\n", ret);
    return ret;
    }
//
// This function sets the vcm position to the value set by the user
// through v4l2_ctrl_ops s_ctrl handler
// The lens position is gradually moved in units of DW9714_CTRL_STEPS,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn dw9714_vcm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dw9714_vcm_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9714_device *dw9714_dev = sd_to_dw9714_vcm(sd);
    int ret, val;
    if (pm_runtime_suspended(&client.dev))
    return 0;
    ret = dw9714_power_up(dw9714_dev);
    if (ret) {
    dev_err(dev, "Failed to power up: %d\n", ret);
    return ret;
    }
    for (val = dw9714_dev.current_val % DW9714_CTRL_STEPS;
    val < dw9714_dev.current_val + DW9714_CTRL_STEPS - 1;
    val += DW9714_CTRL_STEPS) {
    ret = dw9714_i2c_write(client,
    DW9714_VAL(val, DW9714_DEFAULT_S));
    if (ret)
    dev_err_ratelimited(dev, "%s I2C failure: %d",
    __func__, ret);
    usleep_range(DW9714_CTRL_DELAY_US, DW9714_CTRL_DELAY_US + 10);
    }
    return 0;
    }
    static const struct i2c_device_id dw9714_id_table[] = {
    { .name = DW9714_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, dw9714_id_table);
    static const struct of_device_id dw9714_of_table[] = {
    { .compatible = "dongwoon,dw9714" },
    { { 0 } }
    };
    MODULE_DEVICE_TABLE(of, dw9714_of_table);
    static const struct dev_pm_ops dw9714_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(dw9714_vcm_suspend, dw9714_vcm_resume)
    SET_RUNTIME_PM_OPS(dw9714_vcm_suspend, dw9714_vcm_resume, core::ptr::null_mut())
    };
    static struct i2c_driver dw9714_i2c_driver = {
    .driver = {
    .name = DW9714_NAME,
    .pm = &dw9714_pm_ops,
    .of_match_table = dw9714_of_table,
    },
    .probe = dw9714_probe,
    .remove = dw9714_remove,
    .id_table = dw9714_id_table,
    };
    module_i2c_driver(dw9714_i2c_driver);
    MODULE_AUTHOR("Tianshu Qiu <tian.shu.qiu@intel.com>");
    MODULE_AUTHOR("Jian Xu Zheng");
    MODULE_AUTHOR("Yuning Pu");
    MODULE_AUTHOR("Jouni Ukkonen");
    MODULE_AUTHOR("Tommi Franttila");
    MODULE_DESCRIPTION("DW9714 VCM driver");
    MODULE_LICENSE("GPL v2");
