//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ak7375.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak73xx_chipdef {
    pub reg_position: u8,
    pub reg_cont: u8,
    pub shift_pos: u8,
    pub mode_active: u8,
    pub mode_standby: u8,
    pub /: *mut *mut bool has_standby; / Some chips may not have standby mode,
    pub focus_pos_max: u16,
//
// This sets the minimum granularity for the focus positions.
// A value of 1 gives maximum accuracy for a desired focus position
//
    pub focus_steps: u16,
//
// This acts as the minimum granularity of lens movement.
// Keep this value power of 2, so the control steps can be
// uniformly adjusted for gradual lens movement, with desired
// number of control steps.
//
    pub ctrl_steps: u16,
    pub ctrl_delay_us: u16,
//
// The vcm may take time (tDELAY) to power on and start taking
// I2C messages.
//
    pub power_delay_us: u16,
}

    static const struct ak73xx_chipdef ak7345_cdef = {
    .reg_position	= 0x0,
    .reg_cont	= 0x2,
    .shift_pos	= 7,	/* 9 bits position values, need to << 7 */
    .mode_active	= 0x0,
    .has_standby	= false,
    .focus_pos_max	= 511,
    .focus_steps	= 1,
    .ctrl_steps	= 16,
    .ctrl_delay_us	= 1000,
    .power_delay_us	= 20000,
    };
    static const struct ak73xx_chipdef ak7375_cdef = {
    .reg_position	= 0x0,
    .reg_cont	= 0x2,
    .shift_pos	= 4,	/* 12 bits position values, need to << 4 */
    .mode_active	= 0x0,
    .mode_standby	= 0x40,
    .has_standby	= true,
    .focus_pos_max	= 4095,
    .focus_steps	= 1,
    .ctrl_steps	= 64,
    .ctrl_delay_us	= 1000,
    .power_delay_us	= 10000,
    };
    static const char * const ak7375_supply_names[] = {
    "vdd",
    "vio",
    };
// ak7375 device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak7375_device {
    pub cdef: *const ak73xx_chipdef,
    pub ctrls_vcm: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub focus: *mut v4l2_ctrl,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(ak7375_supply_names)],
// active or standby mode
    pub active: bool,
}

    static inline struct ak7375_device *to_ak7375_vcm(struct v4l2_ctrl *ctrl)
    {
    return container_of(ctrl.handler, struct ak7375_device, ctrls_vcm);
    }
    static inline struct ak7375_device *sd_to_ak7375_vcm(struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct ak7375_device, sd);
    }
    static int ak7375_i2c_write(struct ak7375_device *ak7375,
    u8 addr, u16 data, u8 size)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&ak7375.sd);
    u8 buf[3];
    int ret;
    if (size != 1 && size != 2)
    return -EINVAL;
    buf[0] = addr;
    buf[size] = data & 0xff;
    if (size == 2)
    buf[1] = (data >> 8) & 0xff;
    ret = i2c_master_send(client, (const char *)buf, size + 1);
    if (ret < 0)
    return ret;
    if (ret != size + 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak7375_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ak7375_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ak7375_device *dev_vcm = to_ak7375_vcm(ctrl);
    const struct ak73xx_chipdef *cdef = dev_vcm.cdef;
    if (ctrl.id == V4L2_CID_FOCUS_ABSOLUTE)
    return ak7375_i2c_write(dev_vcm, cdef.reg_position,
    ctrl.val << cdef.shift_pos, 2);
    return -EINVAL;
    }
    static const struct v4l2_ctrl_ops ak7375_vcm_ctrl_ops = {
    .s_ctrl = ak7375_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ak7375_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ak7375_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    return pm_runtime_resume_and_get(sd.dev);
    }
#[no_mangle]
unsafe extern "C" fn ak7375_close(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ak7375_close(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    pm_runtime_put(sd.dev);
    return 0;
    }
    static const struct v4l2_subdev_internal_ops ak7375_int_ops = {
    .open = ak7375_open,
    .close = ak7375_close,
    };
    let mut ak7375_ops: static struct v4l2_subdev_ops = { };
#[no_mangle]
unsafe extern "C" fn ak7375_subdev_cleanup(ak7375_dev: *mut ak7375_device) {
    static void ak7375_subdev_cleanup(struct ak7375_device *ak7375_dev)
    {
    v4l2_async_unregister_subdev(&ak7375_dev.sd);
    v4l2_ctrl_handler_free(&ak7375_dev.ctrls_vcm);
    media_entity_cleanup(&ak7375_dev.sd.entity);
    }
#[no_mangle]
unsafe extern "C" fn ak7375_init_controls(dev_vcm: *mut ak7375_device) -> c_int {
    static int ak7375_init_controls(struct ak7375_device *dev_vcm)
    {
    struct v4l2_ctrl_handler *hdl = &dev_vcm.ctrls_vcm;
    const struct v4l2_ctrl_ops *ops = &ak7375_vcm_ctrl_ops;
    const struct ak73xx_chipdef *cdef = dev_vcm.cdef;
    v4l2_ctrl_handler_init(hdl, 1);
    dev_vcm.focus = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_FOCUS_ABSOLUTE,
    0, cdef.focus_pos_max, cdef.focus_steps, 0);
    if (hdl.error)
    dev_err(dev_vcm.sd.dev, "%s fail error: 0x%x\n",
    __func__, hdl.error);
    dev_vcm.sd.ctrl_handler = hdl;
    return hdl.error;
    }
#[no_mangle]
unsafe extern "C" fn ak7375_probe(client: *mut i2c_client) -> c_int {
    static int ak7375_probe(struct i2c_client *client)
    {
    struct ak7375_device *ak7375_dev;
    int ret;
    unsigned int i;
    ak7375_dev = devm_kzalloc(&client.dev, sizeof(*ak7375_dev),
    GFP_KERNEL);
    if (!ak7375_dev)
    return -ENOMEM;
    ak7375_dev.cdef = device_get_match_data(&client.dev);
    for (i = 0; i < ARRAY_SIZE(ak7375_supply_names); i++)
    ak7375_dev.supplies[i].supply = ak7375_supply_names[i];
    ret = devm_regulator_bulk_get(&client.dev,
    ARRAY_SIZE(ak7375_supply_names),
    ak7375_dev.supplies);
    if (ret) {
    dev_err_probe(&client.dev, ret, "Failed to get regulators\n");
    return ret;
    }
    v4l2_i2c_subdev_init(&ak7375_dev.sd, client, &ak7375_ops);
    ak7375_dev.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ak7375_dev.sd.internal_ops = &ak7375_int_ops;
    ak7375_dev.sd.entity.function = MEDIA_ENT_F_LENS;
    ret = ak7375_init_controls(ak7375_dev);
    if (ret)
    goto err_cleanup;
    ret = media_entity_pads_init(&ak7375_dev.sd.entity, 0, core::ptr::null_mut());
    if (ret < 0)
    goto err_cleanup;
    ret = v4l2_async_register_subdev(&ak7375_dev.sd);
    if (ret < 0)
    goto err_cleanup;
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
    pm_runtime_idle(&client.dev);
    return 0;
    err_cleanup:
    v4l2_ctrl_handler_free(&ak7375_dev.ctrls_vcm);
    media_entity_cleanup(&ak7375_dev.sd.entity);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ak7375_remove(client: *mut i2c_client) {
    static void ak7375_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ak7375_device *ak7375_dev = sd_to_ak7375_vcm(sd);
    ak7375_subdev_cleanup(ak7375_dev);
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    }
//
// This function sets the vcm position, so it consumes least current
// The lens position is gradually moved in units of ctrl_steps,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn ak7375_vcm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ak7375_vcm_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ak7375_device *ak7375_dev = sd_to_ak7375_vcm(sd);
    const struct ak73xx_chipdef *cdef = ak7375_dev.cdef;
    int ret, val;
    if (!ak7375_dev.active)
    return 0;
    for (val = ak7375_dev.focus.val & ~(cdef.ctrl_steps - 1);
    val >= 0; val -= cdef.ctrl_steps) {
    ret = ak7375_i2c_write(ak7375_dev, cdef.reg_position,
    val << cdef.shift_pos, 2);
    if (ret)
    dev_err_once(dev, "%s I2C failure: %d\n",
    __func__, ret);
    usleep_range(cdef.ctrl_delay_us, cdef.ctrl_delay_us + 10);
    }
    if (cdef.has_standby) {
    ret = ak7375_i2c_write(ak7375_dev, cdef.reg_cont,
    cdef.mode_standby, 1);
    if (ret)
    dev_err(dev, "%s I2C failure: %d\n", __func__, ret);
    }
    ret = regulator_bulk_disable(ARRAY_SIZE(ak7375_supply_names),
    ak7375_dev.supplies);
    if (ret)
    return ret;
    ak7375_dev.active = false;
    return 0;
    }
//
// This function sets the vcm position to the value set by the user
// through v4l2_ctrl_ops s_ctrl handler
// The lens position is gradually moved in units of ctrl_steps,
// to make the movements smoothly.
//
#[no_mangle]
unsafe extern "C" fn ak7375_vcm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ak7375_vcm_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ak7375_device *ak7375_dev = sd_to_ak7375_vcm(sd);
    const struct ak73xx_chipdef *cdef = ak7375_dev.cdef;
    int ret, val;
    if (ak7375_dev.active)
    return 0;
    ret = regulator_bulk_enable(ARRAY_SIZE(ak7375_supply_names),
    ak7375_dev.supplies);
    if (ret)
    return ret;
// Wait for vcm to become ready
    usleep_range(cdef.power_delay_us, cdef.power_delay_us + 500);
    ret = ak7375_i2c_write(ak7375_dev, cdef.reg_cont,
    cdef.mode_active, 1);
    if (ret) {
    dev_err(dev, "%s I2C failure: %d\n", __func__, ret);
    return ret;
    }
    for (val = ak7375_dev.focus.val % cdef.ctrl_steps;
    val <= ak7375_dev.focus.val;
    val += cdef.ctrl_steps) {
    ret = ak7375_i2c_write(ak7375_dev, cdef.reg_position,
    val << cdef.shift_pos, 2);
    if (ret)
    dev_err_ratelimited(dev, "%s I2C failure: %d\n",
    __func__, ret);
    usleep_range(cdef.ctrl_delay_us, cdef.ctrl_delay_us + 10);
    }
    ak7375_dev.active = true;
    return 0;
    }
    static const struct of_device_id ak7375_of_table[] = {
    { .compatible = "asahi-kasei,ak7345", .data = &ak7345_cdef, },
    { .compatible = "asahi-kasei,ak7375", .data = &ak7375_cdef, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ak7375_of_table);
    static const struct dev_pm_ops ak7375_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(ak7375_vcm_suspend, ak7375_vcm_resume)
    SET_RUNTIME_PM_OPS(ak7375_vcm_suspend, ak7375_vcm_resume, core::ptr::null_mut())
    };
    static struct i2c_driver ak7375_i2c_driver = {
    .driver = {
    .name = "ak7375",
    .pm = &ak7375_pm_ops,
    .of_match_table = ak7375_of_table,
    },
    .probe = ak7375_probe,
    .remove = ak7375_remove,
    };
    module_i2c_driver(ak7375_i2c_driver);
    MODULE_AUTHOR("Tianshu Qiu <tian.shu.qiu@intel.com>");
    MODULE_AUTHOR("Bingbu Cao <bingbu.cao@intel.com>");
    MODULE_DESCRIPTION("AK7375 VCM driver");
    MODULE_LICENSE("GPL v2");
