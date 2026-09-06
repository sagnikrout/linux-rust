//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tw2804.c
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

pub const TW2804_REG_AUTOGAIN: c_uint = 0x02;
pub const TW2804_REG_HUE: c_uint = 0x0f;
pub const TW2804_REG_SATURATION: c_uint = 0x10;
pub const TW2804_REG_CONTRAST: c_uint = 0x11;
pub const TW2804_REG_BRIGHTNESS: c_uint = 0x12;
pub const TW2804_REG_COLOR_KILLER: c_uint = 0x14;
pub const TW2804_REG_GAIN: c_uint = 0x3c;
pub const TW2804_REG_CHROMA_GAIN: c_uint = 0x3d;
pub const TW2804_REG_BLUE_BALANCE: c_uint = 0x3e;
pub const TW2804_REG_RED_BALANCE: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw2804 {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub channel:2: u8,
    pub input:1: u8,
    pub norm: c_int,
}

    static const u8 global_registers[] = {
    0x39, 0x00,
    0x3a, 0xff,
    0x3b, 0x84,
    0x3c, 0x80,
    0x3d, 0x80,
    0x3e, 0x82,
    0x3f, 0x82,
    0x78, 0x00,
    0xff, 0xff, /* Terminator (reg 0xff does not exist) */
    };
    static const u8 channel_registers[] = {
    0x01, 0xc4,
    0x02, 0xa5,
    0x03, 0x20,
    0x04, 0xd0,
    0x05, 0x20,
    0x06, 0xd0,
    0x07, 0x88,
    0x08, 0x20,
    0x09, 0x07,
    0x0a, 0xf0,
    0x0b, 0x07,
    0x0c, 0xf0,
    0x0d, 0x40,
    0x0e, 0xd2,
    0x0f, 0x80,
    0x10, 0x80,
    0x11, 0x80,
    0x12, 0x80,
    0x13, 0x1f,
    0x14, 0x00,
    0x15, 0x00,
    0x16, 0x00,
    0x17, 0x00,
    0x18, 0xff,
    0x19, 0xff,
    0x1a, 0xff,
    0x1b, 0xff,
    0x1c, 0xff,
    0x1d, 0xff,
    0x1e, 0xff,
    0x1f, 0xff,
    0x20, 0x07,
    0x21, 0x07,
    0x22, 0x00,
    0x23, 0x91,
    0x24, 0x51,
    0x25, 0x03,
    0x26, 0x00,
    0x27, 0x00,
    0x28, 0x00,
    0x29, 0x00,
    0x2a, 0x00,
    0x2b, 0x00,
    0x2c, 0x00,
    0x2d, 0x00,
    0x2e, 0x00,
    0x2f, 0x00,
    0x30, 0x00,
    0x31, 0x00,
    0x32, 0x00,
    0x33, 0x00,
    0x34, 0x00,
    0x35, 0x00,
    0x36, 0x00,
    0x37, 0x00,
    0xff, 0xff, /* Terminator (reg 0xff does not exist) */
    };
#[no_mangle]
unsafe extern "C" fn write_reg(client: *mut i2c_client, reg: u8, value: u8, channel: u8) -> c_int {
    static int write_reg(struct i2c_client *client, u8 reg, u8 value, u8 channel)
    {
    return i2c_smbus_write_byte_data(client, reg | (channel << 6), value);
    }
#[no_mangle]
unsafe extern "C" fn write_regs(client: *mut i2c_client, regs: *const u8, channel: u8) -> c_int {
    static int write_regs(struct i2c_client *client, const u8 *regs, u8 channel)
    {
    int ret;
    int i;
    for (i = 0; regs[i] != 0xff; i += 2) {
    ret = i2c_smbus_write_byte_data(client,
    regs[i] | (channel << 6), regs[i + 1]);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_reg(client: *mut i2c_client, reg: u8, channel: u8) -> c_int {
    static int read_reg(struct i2c_client *client, u8 reg, u8 channel)
    {
    return i2c_smbus_read_byte_data(client, (reg) | (channel << 6));
    }
    static inline struct tw2804 *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct tw2804, sd);
    }
    static inline struct tw2804 *to_state_from_ctrl(struct v4l2_ctrl *ctrl)
    {
    return container_of(ctrl.handler, struct tw2804, hdl);
    }
#[no_mangle]
unsafe extern "C" fn tw2804_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int tw2804_log_status(struct v4l2_subdev *sd)
    {
    struct tw2804 *state = to_state(sd);
    v4l2_info(sd, "Standard: %s\n",
    state.norm & V4L2_STD_525_60 ? "60 Hz" : "50 Hz");
    v4l2_info(sd, "Channel: %d\n", state.channel);
    v4l2_info(sd, "Input: %d\n", state.input);
    return v4l2_ctrl_subdev_log_status(sd);
    }
//
// These volatile controls are needed because all four channels share
// these controls. So a change made to them through one channel would
// require another channel to be updated.
//
// Normally this would have been done in a different way, but since the one
// board that uses this driver sees this single chip as if it was on four
// different i2c adapters (each adapter belonging to a separate instance of
// the same USB driver) there is no reliable method that I have found to let
// the instances know about each other.
//
// So implementing these global registers as volatile is the best we can do.
//
#[no_mangle]
unsafe extern "C" fn tw2804_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int tw2804_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct tw2804 *state = to_state_from_ctrl(ctrl);
    struct i2c_client *client = v4l2_get_subdevdata(&state.sd);
    switch (ctrl.id) {
    case V4L2_CID_GAIN:
    ctrl.val = read_reg(client, TW2804_REG_GAIN, 0);
    return 0;
    case V4L2_CID_CHROMA_GAIN:
    ctrl.val = read_reg(client, TW2804_REG_CHROMA_GAIN, 0);
    return 0;
    case V4L2_CID_BLUE_BALANCE:
    ctrl.val = read_reg(client, TW2804_REG_BLUE_BALANCE, 0);
    return 0;
    case V4L2_CID_RED_BALANCE:
    ctrl.val = read_reg(client, TW2804_REG_RED_BALANCE, 0);
    return 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw2804_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int tw2804_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct tw2804 *state = to_state_from_ctrl(ctrl);
    struct i2c_client *client = v4l2_get_subdevdata(&state.sd);
    int addr;
    int reg;
    switch (ctrl.id) {
    case V4L2_CID_AUTOGAIN:
    addr = TW2804_REG_AUTOGAIN;
    reg = read_reg(client, addr, state.channel);
    if (reg < 0)
    return reg;
    if (ctrl.val == 0)
    reg &= ~(1 << 7);
    else
    reg |= 1 << 7;
    return write_reg(client, addr, reg, state.channel);
    case V4L2_CID_COLOR_KILLER:
    addr = TW2804_REG_COLOR_KILLER;
    reg = read_reg(client, addr, state.channel);
    if (reg < 0)
    return reg;
    reg = (reg & ~(0x03)) | (ctrl.val == 0 ? 0x02 : 0x03);
    return write_reg(client, addr, reg, state.channel);
    case V4L2_CID_GAIN:
    return write_reg(client, TW2804_REG_GAIN, ctrl.val, 0);
    case V4L2_CID_CHROMA_GAIN:
    return write_reg(client, TW2804_REG_CHROMA_GAIN, ctrl.val, 0);
    case V4L2_CID_BLUE_BALANCE:
    return write_reg(client, TW2804_REG_BLUE_BALANCE, ctrl.val, 0);
    case V4L2_CID_RED_BALANCE:
    return write_reg(client, TW2804_REG_RED_BALANCE, ctrl.val, 0);
    case V4L2_CID_BRIGHTNESS:
    return write_reg(client, TW2804_REG_BRIGHTNESS,
    ctrl.val, state.channel);
    case V4L2_CID_CONTRAST:
    return write_reg(client, TW2804_REG_CONTRAST,
    ctrl.val, state.channel);
    case V4L2_CID_SATURATION:
    return write_reg(client, TW2804_REG_SATURATION,
    ctrl.val, state.channel);
    case V4L2_CID_HUE:
    return write_reg(client, TW2804_REG_HUE,
    ctrl.val, state.channel);
    default:
    break;
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn tw2804_s_std(sd: *mut v4l2_subdev, norm: v4l2_std_id) -> c_int {
    static int tw2804_s_std(struct v4l2_subdev *sd, v4l2_std_id norm)
    {
    struct tw2804 *dec = to_state(sd);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    let mut is_60hz: bool = norm & V4L2_STD_525_60;
    u8 regs[] = {
    0x01, is_60hz ? 0xc4 : 0x84,
    0x09, is_60hz ? 0x07 : 0x04,
    0x0a, is_60hz ? 0xf0 : 0x20,
    0x0b, is_60hz ? 0x07 : 0x04,
    0x0c, is_60hz ? 0xf0 : 0x20,
    0x0d, is_60hz ? 0x40 : 0x4a,
    0x16, is_60hz ? 0x00 : 0x40,
    0x17, is_60hz ? 0x00 : 0x40,
    0x20, is_60hz ? 0x07 : 0x0f,
    0x21, is_60hz ? 0x07 : 0x0f,
    0xff, 0xff,
    };
    write_regs(client, regs, dec.channel);
    dec.norm = norm;
    return 0;
    }
    static int tw2804_s_video_routing(struct v4l2_subdev *sd, u32 input, u32 output,
    u32 config)
    {
    struct tw2804 *dec = to_state(sd);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int reg;
    if (config && config - 1 != dec.channel) {
    if (config > 4) {
    dev_err(&client.dev,
    "channel %d is not between 1 and 4!\n", config);
    return -EINVAL;
    }
    dec.channel = config - 1;
    dev_dbg(&client.dev, "initializing TW2804 channel %d\n",
    dec.channel);
    if (dec.channel == 0 &&
    write_regs(client, global_registers, 0) < 0) {
    dev_err(&client.dev,
    "error initializing TW2804 global registers\n");
    return -EIO;
    }
    if (write_regs(client, channel_registers, dec.channel) < 0) {
    dev_err(&client.dev,
    "error initializing TW2804 channel %d\n",
    dec.channel);
    return -EIO;
    }
    }
    if (input > 1)
    return -EINVAL;
    if (input == dec.input)
    return 0;
    reg = read_reg(client, 0x22, dec.channel);
    if (reg >= 0) {
    if (input == 0)
    reg &= ~(1 << 2);
    else
    reg |= 1 << 2;
    reg = write_reg(client, 0x22, reg, dec.channel);
    }
    if (reg >= 0)
    dec.input = input;
    else
    return reg;
    return 0;
    }
    static const struct v4l2_ctrl_ops tw2804_ctrl_ops = {
    .g_volatile_ctrl = tw2804_g_volatile_ctrl,
    .s_ctrl = tw2804_s_ctrl,
    };
    static const struct v4l2_subdev_video_ops tw2804_video_ops = {
    .s_std = tw2804_s_std,
    .s_routing = tw2804_s_video_routing,
    };
    static const struct v4l2_subdev_core_ops tw2804_core_ops = {
    .log_status = tw2804_log_status,
    };
    static const struct v4l2_subdev_ops tw2804_ops = {
    .core = &tw2804_core_ops,
    .video = &tw2804_video_ops,
    };
#[no_mangle]
unsafe extern "C" fn tw2804_probe(client: *mut i2c_client) -> c_int {
    static int tw2804_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct tw2804 *state;
    struct v4l2_subdev *sd;
    struct v4l2_ctrl *ctrl;
    int err;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    state = devm_kzalloc(&client.dev, sizeof(*state), GFP_KERNEL);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &tw2804_ops);
    state.channel = -1;
    state.norm = V4L2_STD_NTSC;
    v4l2_ctrl_handler_init(&state.hdl, 10);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_BRIGHTNESS, 0, 255, 1, 128);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_CONTRAST, 0, 255, 1, 128);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_SATURATION, 0, 255, 1, 128);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_HUE, 0, 255, 1, 128);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_COLOR_KILLER, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_AUTOGAIN, 0, 1, 1, 0);
    ctrl = v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_GAIN, 0, 255, 1, 128);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_CHROMA_GAIN, 0, 255, 1, 128);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_BLUE_BALANCE, 0, 255, 1, 122);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrl = v4l2_ctrl_new_std(&state.hdl, &tw2804_ctrl_ops,
    V4L2_CID_RED_BALANCE, 0, 255, 1, 122);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_VOLATILE;
    sd.ctrl_handler = &state.hdl;
    err = state.hdl.error;
    if (err) {
    v4l2_ctrl_handler_free(&state.hdl);
    return err;
    }
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw2804_remove(client: *mut i2c_client) {
    static void tw2804_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct tw2804 *state = to_state(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&state.hdl);
    }
    static const struct i2c_device_id tw2804_id[] = {
    { .name = "tw2804" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tw2804_id);
    static struct i2c_driver tw2804_driver = {
    .driver = {
    .name	= "tw2804",
    },
    .probe		= tw2804_probe,
    .remove		= tw2804_remove,
    .id_table	= tw2804_id,
    };
    module_i2c_driver(tw2804_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("TW2804/TW2802 V4L2 i2c driver");
    MODULE_AUTHOR("Micronas USA Inc");
