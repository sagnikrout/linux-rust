//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/vp27smpx.c
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
// vp27smpx - driver version 0.0.1
//
// Copyright (C) 2007 Hans Verkuil <hverkuil@kernel.org>
//
// Based on a tvaudio patch from Takahiro Adachi <tadachi@tadachi-net.com>
// and Kazuhiko Kawakami <kazz-0@mail.goo.ne.jp>
//

    MODULE_DESCRIPTION("vp27smpx driver");
    MODULE_AUTHOR("Hans Verkuil");
    MODULE_LICENSE("GPL");
// -----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vp27smpx_state {
    pub sd: v4l2_subdev,
    pub radio: c_int,
    pub audmode: u32,
}

    static inline struct vp27smpx_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct vp27smpx_state, sd);
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_set_audmode(sd: *mut v4l2_subdev, audmode: u32) {
    static void vp27smpx_set_audmode(struct v4l2_subdev *sd, u32 audmode)
    {
    struct vp27smpx_state *state = to_state(sd);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    u8 data[3] = { 0x00, 0x00, 0x04 };
    switch (audmode) {
    case V4L2_TUNER_MODE_MONO:
    case V4L2_TUNER_MODE_LANG1:
    break;
    case V4L2_TUNER_MODE_STEREO:
    case V4L2_TUNER_MODE_LANG1_LANG2:
    data[1] = 0x01;
    break;
    case V4L2_TUNER_MODE_LANG2:
    data[1] = 0x02;
    break;
    }
    if (i2c_master_send(client, data, sizeof(data)) != sizeof(data))
    v4l2_err(sd, "I/O error setting audmode\n");
    else
    state.audmode = audmode;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_s_radio(sd: *mut v4l2_subdev) -> c_int {
    static int vp27smpx_s_radio(struct v4l2_subdev *sd)
    {
    struct vp27smpx_state *state = to_state(sd);
    state.radio = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_s_std(sd: *mut v4l2_subdev, norm: v4l2_std_id) -> c_int {
    static int vp27smpx_s_std(struct v4l2_subdev *sd, v4l2_std_id norm)
    {
    struct vp27smpx_state *state = to_state(sd);
    state.radio = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_s_tuner(sd: *mut v4l2_subdev, vt: *const v4l2_tuner) -> c_int {
    static int vp27smpx_s_tuner(struct v4l2_subdev *sd, const struct v4l2_tuner *vt)
    {
    struct vp27smpx_state *state = to_state(sd);
    if (!state.radio)
    vp27smpx_set_audmode(sd, vt.audmode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_g_tuner(sd: *mut v4l2_subdev, vt: *mut v4l2_tuner) -> c_int {
    static int vp27smpx_g_tuner(struct v4l2_subdev *sd, struct v4l2_tuner *vt)
    {
    struct vp27smpx_state *state = to_state(sd);
    if (state.radio)
    return 0;
    vt.audmode = state.audmode;
    vt.capability = V4L2_TUNER_CAP_STEREO |
    V4L2_TUNER_CAP_LANG1 | V4L2_TUNER_CAP_LANG2;
    vt.rxsubchans = V4L2_TUNER_SUB_MONO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_log_status(sd: *mut v4l2_subdev) -> c_int {
    static int vp27smpx_log_status(struct v4l2_subdev *sd)
    {
    struct vp27smpx_state *state = to_state(sd);
    v4l2_info(sd, "Audio Mode: %u%s\n", state.audmode,
    state.radio ? " (Radio)" : "");
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_subdev_core_ops vp27smpx_core_ops = {
    .log_status = vp27smpx_log_status,
    };
    static const struct v4l2_subdev_tuner_ops vp27smpx_tuner_ops = {
    .s_radio = vp27smpx_s_radio,
    .s_tuner = vp27smpx_s_tuner,
    .g_tuner = vp27smpx_g_tuner,
    };
    static const struct v4l2_subdev_video_ops vp27smpx_video_ops = {
    .s_std = vp27smpx_s_std,
    };
    static const struct v4l2_subdev_ops vp27smpx_ops = {
    .core = &vp27smpx_core_ops,
    .tuner = &vp27smpx_tuner_ops,
    .video = &vp27smpx_video_ops,
    };
// -----------------------------------------------------------------------
// i2c implementation
//
// Generic i2c probe
// concerning the addresses: i2c wants 7 bit (without the r/w bit), so '>>1'
//
#[no_mangle]
unsafe extern "C" fn vp27smpx_probe(client: *mut i2c_client) -> c_int {
    static int vp27smpx_probe(struct i2c_client *client)
    {
    struct vp27smpx_state *state;
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
    v4l2_i2c_subdev_init(sd, client, &vp27smpx_ops);
    state.audmode = V4L2_TUNER_MODE_STEREO;
// initialize vp27smpx
    vp27smpx_set_audmode(sd, state.audmode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vp27smpx_remove(client: *mut i2c_client) {
    static void vp27smpx_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id vp27smpx_id[] = {
    { .name = "vp27smpx" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, vp27smpx_id);
    static struct i2c_driver vp27smpx_driver = {
    .driver = {
    .name	= "vp27smpx",
    },
    .probe		= vp27smpx_probe,
    .remove		= vp27smpx_remove,
    .id_table	= vp27smpx_id,
    };
    module_i2c_driver(vp27smpx_driver);
