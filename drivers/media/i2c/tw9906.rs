//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tw9906.c
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
// Copyright (C) 2005-2006 Micronas USA Inc.
//

    MODULE_DESCRIPTION("TW9906 I2C subdev driver");
    MODULE_LICENSE("GPL v2");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw9906 {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub norm: v4l2_std_id,
}

    static inline struct tw9906 *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct tw9906, sd);
    }
    static const u8 initial_registers[] = {
    0x02, 0x40, /* input 0, composite */
    0x03, 0xa2, /* correct digital format */
    0x05, 0x81, /* or 0x01 for PAL */
    0x07, 0x02, /* window */
    0x08, 0x14, /* window */
    0x09, 0xf0, /* window */
    0x0a, 0x10, /* window */
    0x0b, 0xd0, /* window */
    0x0d, 0x00, /* scaling */
    0x0e, 0x11, /* scaling */
    0x0f, 0x00, /* scaling */
    0x10, 0x00, /* brightness */
    0x11, 0x60, /* contrast */
    0x12, 0x11, /* sharpness */
    0x13, 0x7e, /* U gain */
    0x14, 0x7e, /* V gain */
    0x15, 0x00, /* hue */
    0x19, 0x57, /* vbi */
    0x1a, 0x0f,
    0x1b, 0x40,
    0x29, 0x03,
    0x55, 0x00,
    0x6b, 0x26,
    0x6c, 0x36,
    0x6d, 0xf0,
    0x6e, 0x41,
    0x6f, 0x13,
    0xad, 0x70,
    0x00, 0x00, /* Terminator (reg 0x00 is read-only) */
    };
#[no_mangle]
unsafe extern "C" fn write_reg(sd: *mut v4l2_subdev, reg: u8, value: u8) -> c_int {
    static int write_reg(struct v4l2_subdev *sd, u8 reg, u8 value)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    return i2c_smbus_write_byte_data(client, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn write_regs(sd: *mut v4l2_subdev, regs: *const u8) -> c_int {
    static int write_regs(struct v4l2_subdev *sd, const u8 *regs)
    {
    int i;
    for (i = 0; regs[i] != 0x00; i += 2)
    if (write_reg(sd, regs[i], regs[i + 1]) < 0)
    return -1;
    return 0;
    }
    static int tw9906_s_video_routing(struct v4l2_subdev *sd, u32 input,
    u32 output, u32 config)
    {
    write_reg(sd, 0x02, 0x40 | (input << 1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9906_s_std(sd: *mut v4l2_subdev, norm: v4l2_std_id) -> c_int {
    static int tw9906_s_std(struct v4l2_subdev *sd, v4l2_std_id norm)
    {
    struct tw9906 *dec = to_state(sd);
    let mut is_60hz: bool = norm & V4L2_STD_525_60;
    static const u8 config_60hz[] = {
    0x05, 0x81,
    0x07, 0x02,
    0x08, 0x14,
    0x09, 0xf0,
    0,    0,
    };
    static const u8 config_50hz[] = {
    0x05, 0x01,
    0x07, 0x12,
    0x08, 0x18,
    0x09, 0x20,
    0,    0,
    };
    write_regs(sd, is_60hz ? config_60hz : config_50hz);
    dec.norm = norm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9906_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int tw9906_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct tw9906 *dec = container_of(ctrl.handler, struct tw9906, hdl);
    struct v4l2_subdev *sd = &dec.sd;
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    write_reg(sd, 0x10, ctrl.val);
    break;
    case V4L2_CID_CONTRAST:
    write_reg(sd, 0x11, ctrl.val);
    break;
    case V4L2_CID_HUE:
    write_reg(sd, 0x15, ctrl.val);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9906_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int tw9906_log_status(struct v4l2_subdev *sd)
    {
    struct tw9906 *dec = to_state(sd);
    let mut is_60hz: bool = dec.norm & V4L2_STD_525_60;
    v4l2_info(sd, "Standard: %d Hz\n", is_60hz ? 60 : 50);
    v4l2_ctrl_subdev_log_status(sd);
    return 0;
    }
// --------------------------------------------------------------------------
    static const struct v4l2_ctrl_ops tw9906_ctrl_ops = {
    .s_ctrl = tw9906_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops tw9906_core_ops = {
    .log_status = tw9906_log_status,
    };
    static const struct v4l2_subdev_video_ops tw9906_video_ops = {
    .s_std = tw9906_s_std,
    .s_routing = tw9906_s_video_routing,
    };
    static const struct v4l2_subdev_ops tw9906_ops = {
    .core = &tw9906_core_ops,
    .video = &tw9906_video_ops,
    };
#[no_mangle]
unsafe extern "C" fn tw9906_probe(client: *mut i2c_client) -> c_int {
    static int tw9906_probe(struct i2c_client *client)
    {
    struct tw9906 *dec;
    struct v4l2_subdev *sd;
    struct v4l2_ctrl_handler *hdl;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    dec = devm_kzalloc(&client.dev, sizeof(*dec), GFP_KERNEL);
    if (dec == core::ptr::null_mut())
    return -ENOMEM;
    sd = &dec.sd;
    v4l2_i2c_subdev_init(sd, client, &tw9906_ops);
    hdl = &dec.hdl;
    v4l2_ctrl_handler_init(hdl, 4);
    v4l2_ctrl_new_std(hdl, &tw9906_ctrl_ops,
    V4L2_CID_BRIGHTNESS, -128, 127, 1, 0);
    v4l2_ctrl_new_std(hdl, &tw9906_ctrl_ops,
    V4L2_CID_CONTRAST, 0, 255, 1, 0x60);
    v4l2_ctrl_new_std(hdl, &tw9906_ctrl_ops,
    V4L2_CID_HUE, -128, 127, 1, 0);
    sd.ctrl_handler = hdl;
    if (hdl.error) {
    let mut err: c_int = hdl.error;
    v4l2_ctrl_handler_free(hdl);
    return err;
    }
// Initialize tw9906
    dec.norm = V4L2_STD_NTSC;
    if (write_regs(sd, initial_registers) < 0) {
    v4l2_err(client, "error initializing TW9906\n");
    v4l2_ctrl_handler_free(hdl);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9906_remove(client: *mut i2c_client) {
    static void tw9906_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&to_state(sd).hdl);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id tw9906_id[] = {
    { .name = "tw9906" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tw9906_id);
    static struct i2c_driver tw9906_driver = {
    .driver = {
    .name	= "tw9906",
    },
    .probe = tw9906_probe,
    .remove = tw9906_remove,
    .id_table = tw9906_id,
    };
    module_i2c_driver(tw9906_driver);
