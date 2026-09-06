//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/cs5345.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cs5345 Cirrus Logic 24-bit, 192 kHz Stereo Audio ADC
// Copyright (C) 2007 Hans Verkuil
//

    MODULE_DESCRIPTION("i2c device driver for cs5345 Audio ADC");
    MODULE_AUTHOR("Hans Verkuil");
    MODULE_LICENSE("GPL");
    static bool debug;
    module_param(debug, bool, 0644);
    MODULE_PARM_DESC(debug, "Debugging messages, 0=Off (default), 1=On");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs5345_state {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
}

    static inline struct cs5345_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct cs5345_state, sd);
    }
    static inline struct v4l2_subdev *to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct cs5345_state, hdl).sd;
    }
// -----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn cs5345_write(sd: *mut v4l2_subdev, reg: u8, value: u8) -> c_int {
    static inline int cs5345_write(struct v4l2_subdev *sd, u8 reg, u8 value)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    return i2c_smbus_write_byte_data(client, reg, value);
    }
#[no_mangle]
pub unsafe extern "C" fn cs5345_read(sd: *mut v4l2_subdev, reg: u8) -> c_int {
    static inline int cs5345_read(struct v4l2_subdev *sd, u8 reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    return i2c_smbus_read_byte_data(client, reg);
    }
    static int cs5345_s_routing(struct v4l2_subdev *sd,
    u32 input, u32 output, u32 config)
    {
    if ((input & 0xf) > 6) {
    v4l2_err(sd, "Invalid input %d.\n", input);
    return -EINVAL;
    }
    cs5345_write(sd, 0x09, input & 0xf);
    cs5345_write(sd, 0x05, input & 0xf0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs5345_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int cs5345_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_sd(ctrl);
    switch (ctrl.id) {
    case V4L2_CID_AUDIO_MUTE:
    cs5345_write(sd, 0x04, ctrl.val ? 0x80 : 0);
    return 0;
    case V4L2_CID_AUDIO_VOLUME:
    cs5345_write(sd, 0x07, ((u8)ctrl.val) & 0x3f);
    cs5345_write(sd, 0x08, ((u8)ctrl.val) & 0x3f);
    return 0;
    }
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn cs5345_g_register(sd: *mut v4l2_subdev, reg: *mut v4l2_dbg_register) -> c_int {
    static int cs5345_g_register(struct v4l2_subdev *sd, struct v4l2_dbg_register *reg)
    {
    reg.size = 1;
    reg.val = cs5345_read(sd, reg.reg & 0x1f);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs5345_s_register(sd: *mut v4l2_subdev, reg: *const v4l2_dbg_register) -> c_int {
    static int cs5345_s_register(struct v4l2_subdev *sd, const struct v4l2_dbg_register *reg)
    {
    cs5345_write(sd, reg.reg & 0x1f, reg.val & 0xff);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn cs5345_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int cs5345_log_status(struct v4l2_subdev *sd)
    {
    let mut v: u8 = cs5345_read(sd, 0x09) & 7;
    let mut m: u8 = cs5345_read(sd, 0x04);
    let mut vol: c_int = cs5345_read(sd, 0x08) & 0x3f;
    v4l2_info(sd, "Input:  %d%s\n", v,
    (m & 0x80) ? " (muted)" : "");
    if (vol >= 32)
    vol = vol - 64;
    v4l2_info(sd, "Volume: %d dB\n", vol);
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_ctrl_ops cs5345_ctrl_ops = {
    .s_ctrl = cs5345_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops cs5345_core_ops = {
    .log_status = cs5345_log_status,

    .g_register = cs5345_g_register,
    .s_register = cs5345_s_register,

    };
    static const struct v4l2_subdev_audio_ops cs5345_audio_ops = {
    .s_routing = cs5345_s_routing,
    };
    static const struct v4l2_subdev_ops cs5345_ops = {
    .core = &cs5345_core_ops,
    .audio = &cs5345_audio_ops,
    };
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn cs5345_probe(client: *mut i2c_client) -> c_int {
    static int cs5345_probe(struct i2c_client *client)
    {
    struct cs5345_state *state;
    struct v4l2_subdev *sd;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    state = devm_kzalloc(&client.dev, sizeof(*state), GFP_KERNEL);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &cs5345_ops);
    v4l2_ctrl_handler_init(&state.hdl, 2);
    v4l2_ctrl_new_std(&state.hdl, &cs5345_ctrl_ops,
    V4L2_CID_AUDIO_MUTE, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&state.hdl, &cs5345_ctrl_ops,
    V4L2_CID_AUDIO_VOLUME, -24, 24, 1, 0);
    sd.ctrl_handler = &state.hdl;
    if (state.hdl.error) {
    let mut err: c_int = state.hdl.error;
    v4l2_ctrl_handler_free(&state.hdl);
    return err;
    }
// set volume/mute
    v4l2_ctrl_handler_setup(&state.hdl);
    cs5345_write(sd, 0x02, 0x00);
    cs5345_write(sd, 0x04, 0x01);
    cs5345_write(sd, 0x09, 0x01);
    return 0;
    }
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn cs5345_remove(client: *mut i2c_client) {
    static void cs5345_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct cs5345_state *state = to_state(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&state.hdl);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id cs5345_id[] = {
    { .name = "cs5345" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs5345_id);
    static struct i2c_driver cs5345_driver = {
    .driver = {
    .name	= "cs5345",
    },
    .probe		= cs5345_probe,
    .remove		= cs5345_remove,
    .id_table	= cs5345_id,
    };
    module_i2c_driver(cs5345_driver);
