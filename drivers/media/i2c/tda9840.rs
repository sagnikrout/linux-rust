//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tda9840.c
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
    tda9840 - i2c-driver for the tda9840 by SGS Thomson
    Copyright (C) 1998-2003 Michael Hunold <michael@mihu.de>
    Copyright (C) 2008 Hans Verkuil <hverkuil@kernel.org>
    The tda9840 is a stereo/dual sound processor with digital
    identification. It can be found at address 0x84 on the i2c-bus.
    For detailed information download the specifications directly
    from SGS Thomson at http://www.st.com
//

    MODULE_AUTHOR("Michael Hunold <michael@mihu.de>");
    MODULE_DESCRIPTION("tda9840 driver");
    MODULE_LICENSE("GPL");
    static int debug;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "Debug level (0-1)");
pub const SWITCH: c_uint = 0x00;
pub const LEVEL_ADJUST: c_uint = 0x02;
pub const STEREO_ADJUST: c_uint = 0x03;
pub const TEST: c_uint = 0x04;
pub const TDA9840_SET_MUTE: c_uint = 0x00;
pub const TDA9840_SET_MONO: c_uint = 0x10;
pub const TDA9840_SET_STEREO: c_uint = 0x2a;
pub const TDA9840_SET_LANG1: c_uint = 0x12;
pub const TDA9840_SET_LANG2: c_uint = 0x1e;
pub const TDA9840_SET_BOTH: c_uint = 0x1a;
pub const TDA9840_SET_BOTH_R: c_uint = 0x16;
pub const TDA9840_SET_EXTERNAL: c_uint = 0x7a;
#[no_mangle]
unsafe extern "C" fn tda9840_write(sd: *mut v4l2_subdev, reg: u8, val: u8) {
    static void tda9840_write(struct v4l2_subdev *sd, u8 reg, u8 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (i2c_smbus_write_byte_data(client, reg, val))
    v4l2_dbg(1, debug, sd, "error writing %02x to %02x\n",
    val, reg);
    }
#[no_mangle]
unsafe extern "C" fn tda9840_status(sd: *mut v4l2_subdev) -> c_int {
    static int tda9840_status(struct v4l2_subdev *sd)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int rc;
    u8 byte;
    rc = i2c_master_recv(client, &byte, 1);
    if (rc != 1) {
    v4l2_dbg(1, debug, sd,
    "i2c_master_recv() failed\n");
    if (rc < 0)
    return rc;
    return -EIO;
    }
    if (byte & 0x80) {
    v4l2_dbg(1, debug, sd,
    "TDA9840_DETECT: register contents invalid\n");
    return -EINVAL;
    }
    v4l2_dbg(1, debug, sd, "TDA9840_DETECT: byte: 0x%02x\n", byte);
    return byte & 0x60;
    }
#[no_mangle]
unsafe extern "C" fn tda9840_s_tuner(sd: *mut v4l2_subdev, t: *const v4l2_tuner) -> c_int {
    static int tda9840_s_tuner(struct v4l2_subdev *sd, const struct v4l2_tuner *t)
    {
    let mut stat: c_int = tda9840_status(sd);
    int byte;
    if (t.index)
    return -EINVAL;
    stat = stat < 0 ? 0 : stat;
    if (stat == 0 || stat == 0x60) /* mono input */
    byte = TDA9840_SET_MONO;
    else if (stat == 0x40) /* stereo input */
    byte = (t.audmode == V4L2_TUNER_MODE_MONO) ?
    TDA9840_SET_MONO : TDA9840_SET_STEREO;
    else { /* bilingual */
    switch (t.audmode) {
    case V4L2_TUNER_MODE_LANG1_LANG2:
    byte = TDA9840_SET_BOTH;
    break;
    case V4L2_TUNER_MODE_LANG2:
    byte = TDA9840_SET_LANG2;
    break;
    default:
    byte = TDA9840_SET_LANG1;
    break;
    }
    }
    v4l2_dbg(1, debug, sd, "TDA9840_SWITCH: 0x%02x\n", byte);
    tda9840_write(sd, SWITCH, byte);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda9840_g_tuner(sd: *mut v4l2_subdev, t: *mut v4l2_tuner) -> c_int {
    static int tda9840_g_tuner(struct v4l2_subdev *sd, struct v4l2_tuner *t)
    {
    let mut stat: c_int = tda9840_status(sd);
    if (stat < 0)
    return stat;
    t.rxsubchans = V4L2_TUNER_SUB_MONO;
    switch (stat & 0x60) {
    case 0x00:
    t.rxsubchans = V4L2_TUNER_SUB_MONO;
    break;
    case 0x20:
    t.rxsubchans = V4L2_TUNER_SUB_LANG1 | V4L2_TUNER_SUB_LANG2;
    break;
    case 0x40:
    t.rxsubchans = V4L2_TUNER_SUB_STEREO | V4L2_TUNER_SUB_MONO;
    break;
    default: /* Incorrect detect */
    t.rxsubchans = V4L2_TUNER_MODE_MONO;
    break;
    }
    return 0;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_subdev_tuner_ops tda9840_tuner_ops = {
    .s_tuner = tda9840_s_tuner,
    .g_tuner = tda9840_g_tuner,
    };
    static const struct v4l2_subdev_ops tda9840_ops = {
    .tuner = &tda9840_tuner_ops,
    };
// -----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn tda9840_probe(client: *mut i2c_client) -> c_int {
    static int tda9840_probe(struct i2c_client *client)
    {
    struct v4l2_subdev *sd;
// let's see whether this adapter can support what we need
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_READ_BYTE_DATA |
    I2C_FUNC_SMBUS_WRITE_BYTE_DATA))
    return -EIO;
    v4l_info(client, "chip found @ 0x%x (%s)\n",
    client.addr << 1, client.adapter.name);
    sd = devm_kzalloc(&client.dev, sizeof(*sd), GFP_KERNEL);
    if (sd == core::ptr::null_mut())
    return -ENOMEM;
    v4l2_i2c_subdev_init(sd, client, &tda9840_ops);
// set initial values for level & stereo - adjustment, mode
    tda9840_write(sd, LEVEL_ADJUST, 0);
    tda9840_write(sd, STEREO_ADJUST, 0);
    tda9840_write(sd, SWITCH, TDA9840_SET_STEREO);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tda9840_remove(client: *mut i2c_client) {
    static void tda9840_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    v4l2_device_unregister_subdev(sd);
    }
    static const struct i2c_device_id tda9840_id[] = {
    { .name = "tda9840" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tda9840_id);
    static struct i2c_driver tda9840_driver = {
    .driver = {
    .name	= "tda9840",
    },
    .probe		= tda9840_probe,
    .remove		= tda9840_remove,
    .id_table	= tda9840_id,
    };
    module_i2c_driver(tda9840_driver);
