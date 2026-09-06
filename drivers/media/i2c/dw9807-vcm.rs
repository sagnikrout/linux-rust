//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/dw9807-vcm.c
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
// Copyright (C) 2018 Intel Corporation

pub const DW9807_MAX_FOCUS_POS: c_int = 1023;
//
// This sets the minimum granularity for the focus positions.
// A value of 1 gives maximum accuracy for a desired focus position.
//
pub const DW9807_FOCUS_STEPS: c_int = 1;
//
// This acts as the minimum granularity of lens movement.
// Keep this value power of 2, so the control steps can be
// uniformly adjusted for gradual lens movement, with desired
// number of control steps.
//
pub const DW9807_CTRL_STEPS: c_int = 16;
pub const DW9807_CTRL_DELAY_US: c_int = 1000;
pub const DW9807_CTL_ADDR: c_uint = 0x02;
//
// DW9807 separates two registers to control the VCM position.
// One for MSB value, another is LSB value.
//
pub const DW9807_MSB_ADDR: c_uint = 0x03;
pub const DW9807_LSB_ADDR: c_uint = 0x04;
pub const DW9807_STATUS_ADDR: c_uint = 0x05;
pub const DW9807_MODE_ADDR: c_uint = 0x06;
pub const DW9807_RESONANCE_ADDR: c_uint = 0x07;
pub const MAX_RETRY: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw9807_device {
    pub ctrls_vcm: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub current_val: u16,
}

    static inline struct dw9807_device *sd_to_dw9807_vcm(
    struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct dw9807_device, sd);
    }
#[no_mangle]
unsafe extern "C" fn dw9807_i2c_check(client: *mut i2c_client) -> c_int {
    static int dw9807_i2c_check(struct i2c_client *client)
    {
    let mut status_addr: c_char = DW9807_STATUS_ADDR;
    char status_result;
    int ret;
    ret = i2c_master_send(client, &status_addr, sizeof(status_addr));
    if (ret < 0) {
    dev_err(&client.dev, "I2C write STATUS address fail ret = %d\n",
    ret);
    return ret;
    }
    ret = i2c_master_recv(client, &status_result, sizeof(status_result));
    if (ret < 0) {
    dev_err(&client.dev, "I2C read STATUS value fail ret = %d\n",
    ret);
    return ret;
    }
    return status_result;
    }
#[no_mangle]
unsafe extern "C" fn dw9807_set_dac(client: *mut i2c_client, data: u16) -> c_int {
    static int dw9807_set_dac(struct i2c_client *client, u16 data)
    {
    const char tx_data[3] = {
    DW9807_MSB_ADDR, ((data >> 8) & 0x03), (data & 0xff)
    };
    int val, ret;
//
// According to the datasheet, need to check the bus status before we
// write VCM position. This ensure that we really write the value
// into the register
//
    ret = readx_poll_timeout(dw9807_i2c_check, client, val, val <= 0,
    DW9807_CTRL_DELAY_US, MAX_RETRY * DW9807_CTRL_DELAY_US);
    if (ret || val < 0) {
    if (ret) {
    dev_warn(&client.dev,
    "Cannot do the write operation because VCM is busy\n");
    }
    return ret ? -EBUSY : val;
    }
// Write VCM position to registers
    ret = i2c_master_send(client, tx_data, sizeof(tx_data));
    if (ret < 0) {
    dev_err(&client.dev,
    "I2C write MSB fail ret=%d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw9807_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int dw9807_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct dw9807_device *dev_vcm = container_of(ctrl.handler,
    struct dw9807_device, ctrls_vcm);
    if (ctrl.id == V4L2_CID_FOCUS_ABSOLUTE) {
    struct i2c_client *client = v4l2_get_subdevdata(&dev_vcm.sd);
    dev_vcm.current_val = ctrl.val;
    return dw9807_set_dac(client, ctrl.val);
    }
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops dw9807_vcm_ctrl_ops = {
    .s_ctrl = dw9807_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn dw9807_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int dw9807_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    return pm_runtime_resume_and_get(sd.dev);
    }
#[no_mangle]
unsafe extern "C" fn dw9807_close(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int dw9807_close(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    pm_runtime_put(sd.dev);
    return 0;
    }
    static const struct v4l2_subdev_internal_ops dw9807_int_ops = {
    .open = dw9807_open,
    .close = dw9807_close,
    };
    let mut dw9807_ops: static struct v4l2_subdev_ops = { };
#[no_mangle]
unsafe extern "C" fn dw9807_subdev_cleanup(dw9807_dev: *mut dw9807_device) {
    static void dw9807_subdev_cleanup(struct dw9807_device *dw9807_dev)
    {
    v4l2_async_unregister_subdev(&dw9807_dev.sd);
    v4l2_ctrl_handler_free(&dw9807_dev.ctrls_vcm);
    media_entity_cleanup(&dw9807_dev.sd.entity);
    }
#[no_mangle]
unsafe extern "C" fn dw9807_init_controls(dev_vcm: *mut dw9807_device) -> c_int {
    static int dw9807_init_controls(struct dw9807_device *dev_vcm)
    {
    struct v4l2_ctrl_handler *hdl = &dev_vcm.ctrls_vcm;
    const struct v4l2_ctrl_ops *ops = &dw9807_vcm_ctrl_ops;
    struct i2c_client *client = v4l2_get_subdevdata(&dev_vcm.sd);
    v4l2_ctrl_handler_init(hdl, 1);
    v4l2_ctrl_new_std(hdl, ops, V4L2_CID_FOCUS_ABSOLUTE,
    0, DW9807_MAX_FOCUS_POS, DW9807_FOCUS_STEPS, 0);
    dev_vcm.sd.ctrl_handler = hdl;
    if (hdl.error) {
    dev_err(&client.dev, "%s fail error: 0x%x\n",
    __func__, hdl.error);
    return hdl.error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw9807_probe(client: *mut i2c_client) -> c_int {
    static int dw9807_probe(struct i2c_client *client)
    {
    struct dw9807_device *dw9807_dev;
    int rval;
    dw9807_dev = devm_kzalloc(&client.dev, sizeof(*dw9807_dev),
    GFP_KERNEL);
    if (dw9807_dev == core::ptr::null_mut())
    return -ENOMEM;
    v4l2_i2c_subdev_init(&dw9807_dev.sd, client, &dw9807_ops);
    dw9807_dev.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    dw9807_dev.sd.internal_ops = &dw9807_int_ops;
    rval = dw9807_init_controls(dw9807_dev);
    if (rval)
    goto err_cleanup;
    rval = media_entity_pads_init(&dw9807_dev.sd.entity, 0, core::ptr::null_mut());
    if (rval < 0)
    goto err_cleanup;
    dw9807_dev.sd.entity.function = MEDIA_ENT_F_LENS;
    rval = v4l2_async_register_subdev(&dw9807_dev.sd);
    if (rval < 0)
    goto err_cleanup;
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
    pm_runtime_idle(&client.dev);
    return 0;
    err_cleanup:
    v4l2_ctrl_handler_free(&dw9807_dev.ctrls_vcm);
    media_entity_cleanup(&dw9807_dev.sd.entity);
    return rval;
    }
#[no_mangle]
unsafe extern "C" fn dw9807_remove(client: *mut i2c_client) {
    static void dw9807_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9807_device *dw9807_dev = sd_to_dw9807_vcm(sd);
    pm_runtime_disable(&client.dev);
    dw9807_subdev_cleanup(dw9807_dev);
    }
//
// This function sets the vcm position, so it consumes least current
// The lens position is gradually moved in units of DW9807_CTRL_STEPS,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn dw9807_vcm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused dw9807_vcm_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9807_device *dw9807_dev = sd_to_dw9807_vcm(sd);
    const char tx_data[2] = { DW9807_CTL_ADDR, 0x01 };
    int ret, val;
    for (val = dw9807_dev.current_val & ~(DW9807_CTRL_STEPS - 1);
    val >= 0; val -= DW9807_CTRL_STEPS) {
    ret = dw9807_set_dac(client, val);
    if (ret)
    dev_err_once(dev, "%s I2C failure: %d", __func__, ret);
    usleep_range(DW9807_CTRL_DELAY_US, DW9807_CTRL_DELAY_US + 10);
    }
// Power down
    ret = i2c_master_send(client, tx_data, sizeof(tx_data));
    if (ret < 0) {
    dev_err(&client.dev, "I2C write CTL fail ret = %d\n", ret);
    return ret;
    }
    return 0;
    }
//
// This function sets the vcm position to the value set by the user
// through v4l2_ctrl_ops s_ctrl handler
// The lens position is gradually moved in units of DW9807_CTRL_STEPS,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn dw9807_vcm_resume(dev: *mut device) -> int  __maybe_unused {
    static int  __maybe_unused dw9807_vcm_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct dw9807_device *dw9807_dev = sd_to_dw9807_vcm(sd);
    const char tx_data[2] = { DW9807_CTL_ADDR, 0x00 };
    int ret, val;
// Power on
    ret = i2c_master_send(client, tx_data, sizeof(tx_data));
    if (ret < 0) {
    dev_err(&client.dev, "I2C write CTL fail ret = %d\n", ret);
    return ret;
    }
    for (val = dw9807_dev.current_val % DW9807_CTRL_STEPS;
    val < dw9807_dev.current_val + DW9807_CTRL_STEPS - 1;
    val += DW9807_CTRL_STEPS) {
    ret = dw9807_set_dac(client, val);
    if (ret)
    dev_err_ratelimited(dev, "%s I2C failure: %d",
    __func__, ret);
    usleep_range(DW9807_CTRL_DELAY_US, DW9807_CTRL_DELAY_US + 10);
    }
    return 0;
    }
    static const struct of_device_id dw9807_of_table[] = {
    { .compatible = "dongwoon,dw9807-vcm" },
// Compatibility for older firmware, NEVER USE THIS IN FIRMWARE!
    { .compatible = "dongwoon,dw9807" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dw9807_of_table);
    static const struct dev_pm_ops dw9807_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(dw9807_vcm_suspend, dw9807_vcm_resume)
    SET_RUNTIME_PM_OPS(dw9807_vcm_suspend, dw9807_vcm_resume, core::ptr::null_mut())
    };
    static struct i2c_driver dw9807_i2c_driver = {
    .driver = {
    .name = "dw9807",
    .pm = &dw9807_pm_ops,
    .of_match_table = dw9807_of_table,
    },
    .probe = dw9807_probe,
    .remove = dw9807_remove,
    };
    module_i2c_driver(dw9807_i2c_driver);
    MODULE_AUTHOR("Chiang, Alan");
    MODULE_DESCRIPTION("DW9807 VCM driver");
    MODULE_LICENSE("GPL v2");
