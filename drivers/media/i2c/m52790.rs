//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/m52790.c
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
// m52790 i2c ivtv driver.
// Copyright (C) 2007  Hans Verkuil
//
// A/V source switching Mitsubishi M52790SP/FP
//

    MODULE_DESCRIPTION("i2c device driver for m52790 A/V switch");
    MODULE_AUTHOR("Hans Verkuil");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m52790_state {
    pub sd: v4l2_subdev,
    pub input: u16,
    pub output: u16,
}

    static inline struct m52790_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct m52790_state, sd);
    }
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn m52790_write(sd: *mut v4l2_subdev) -> c_int {
    static int m52790_write(struct v4l2_subdev *sd)
    {
    struct m52790_state *state = to_state(sd);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    let mut sw1: u8 = (state.input | state.output) & 0xff;
    let mut sw2: u8 = (state.input | state.output) >> 8;
    return i2c_smbus_write_byte_data(client, sw1, sw2);
    }
// Note: audio and video are linked and cannot be switched separately.
    So audio and video routing commands are identical for this chip.
    In theory the video amplifier and audio modes could be handled
    separately for the output, but that seems to be overkill right now.
    The same holds for implementing an audio mute control, this is now
    part of the audio output routing. The normal case is that another
    chip takes care of the actual muting so making it part of the
    output routing seems to be the right thing to do for now. */
    static int m52790_s_routing(struct v4l2_subdev *sd,
    u32 input, u32 output, u32 config)
    {
    struct m52790_state *state = to_state(sd);
    state.input = input;
    state.output = output;
    m52790_write(sd);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn m52790_g_register(sd: *mut v4l2_subdev, reg: *mut v4l2_dbg_register) -> c_int {
    static int m52790_g_register(struct v4l2_subdev *sd, struct v4l2_dbg_register *reg)
    {
    struct m52790_state *state = to_state(sd);
    if (reg.reg != 0)
    return -EINVAL;
    reg.size = 1;
    reg.val = state.input | state.output;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m52790_s_register(sd: *mut v4l2_subdev, reg: *const v4l2_dbg_register) -> c_int {
    static int m52790_s_register(struct v4l2_subdev *sd, const struct v4l2_dbg_register *reg)
    {
    struct m52790_state *state = to_state(sd);
    if (reg.reg != 0)
    return -EINVAL;
    state.input = reg.val & 0x0303;
    state.output = reg.val & ~0x0303;
    m52790_write(sd);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn m52790_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int m52790_log_status(struct v4l2_subdev *sd)
    {
    struct m52790_state *state = to_state(sd);
    v4l2_info(sd, "Switch 1: %02x\n",
    (state.input | state.output) & 0xff);
    v4l2_info(sd, "Switch 2: %02x\n",
    (state.input | state.output) >> 8);
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_subdev_core_ops m52790_core_ops = {
    .log_status = m52790_log_status,

    .g_register = m52790_g_register,
    .s_register = m52790_s_register,

    };
    static const struct v4l2_subdev_audio_ops m52790_audio_ops = {
    .s_routing = m52790_s_routing,
    };
    static const struct v4l2_subdev_video_ops m52790_video_ops = {
    .s_routing = m52790_s_routing,
    };
    static const struct v4l2_subdev_ops m52790_ops = {
    .core = &m52790_core_ops,
    .audio = &m52790_audio_ops,
    .video = &m52790_video_ops,
    };
// -----------------------------------------------------------------------
// i2c implementation
#[no_mangle]
unsafe extern "C" fn m52790_probe(client: *mut i2c_client) -> c_int {
    static int m52790_probe(struct i2c_client *client)
    {
    struct m52790_state *state;
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
    v4l2_i2c_subdev_init(sd, client, &m52790_ops);
    state.input = M52790_IN_TUNER;
    state.output = M52790_OUT_STEREO;
    m52790_write(sd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn m52790_remove(client: *mut i2c_client) {
    static void m52790_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id m52790_id[] = {
    { .name = "m52790" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, m52790_id);
    static struct i2c_driver m52790_driver = {
    .driver = {
    .name	= "m52790",
    },
    .probe		= m52790_probe,
    .remove		= m52790_remove,
    .id_table	= m52790_id,
    };
    module_i2c_driver(m52790_driver);
