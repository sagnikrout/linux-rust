//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ak881x.c
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
// Driver for AK8813 / AK8814 TV-ecoders from Asahi Kasei Microsystems Co., Ltd. (AKM)
//
// Copyright (C) 2010, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

pub const AK881X_INTERFACE_MODE: c_int = 0;
pub const AK881X_VIDEO_PROCESS1: c_int = 1;
pub const AK881X_VIDEO_PROCESS2: c_int = 2;
pub const AK881X_VIDEO_PROCESS3: c_int = 3;
pub const AK881X_DAC_MODE: c_int = 5;
pub const AK881X_STATUS: c_uint = 0x24;
pub const AK881X_DEVICE_ID: c_uint = 0x25;
pub const AK881X_DEVICE_REVISION: c_uint = 0x26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak881x {
    pub subdev: v4l2_subdev,
    pub pdata: *mut ak881x_pdata,
    pub lines: c_uint,
    pub /: *mut *mut char revision; / DEVICE_REVISION content,
}

#[no_mangle]
unsafe extern "C" fn reg_read(client: *mut i2c_client, reg: u8) -> c_int {
    static int reg_read(struct i2c_client *client, const u8 reg)
    {
    return i2c_smbus_read_byte_data(client, reg);
    }
    static int reg_write(struct i2c_client *client, const u8 reg,
    const u8 data)
    {
    return i2c_smbus_write_byte_data(client, reg, data);
    }
    static int reg_set(struct i2c_client *client, const u8 reg,
    const u8 data, u8 mask)
    {
    let mut ret: c_int = reg_read(client, reg);
    if (ret < 0)
    return ret;
    return reg_write(client, reg, (ret & ~mask) | (data & mask));
    }
    static struct ak881x *to_ak881x(const struct i2c_client *client)
    {
    return container_of(i2c_get_clientdata(client), struct ak881x, subdev);
    }

    static int ak881x_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (reg.reg > 0x26)
    return -EINVAL;
    reg.size = 1;
    reg.val = reg_read(client, reg.reg);
    if (reg.val > 0xffff)
    return -EIO;
    return 0;
    }
    static int ak881x_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (reg.reg > 0x26)
    return -EINVAL;
    if (reg_write(client, reg.reg, reg.val) < 0)
    return -EIO;
    return 0;
    }

    static int ak881x_fill_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ak881x *ak881x = to_ak881x(client);
    if (format.pad)
    return -EINVAL;
    v4l_bound_align_image(&mf.width, 0, 720, 2,
    &mf.height, 0, ak881x.lines, 1, 0);
    mf.field	= V4L2_FIELD_INTERLACED;
    mf.code	= MEDIA_BUS_FMT_YUYV8_2X8;
    mf.colorspace	= V4L2_COLORSPACE_SMPTE170M;
    return 0;
    }
    static int ak881x_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_YUYV8_2X8;
    return 0;
    }
    static int ak881x_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ak881x *ak881x = to_ak881x(client);
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.left = 0;
    sel.r.top = 0;
    sel.r.width = 720;
    sel.r.height = ak881x.lines;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ak881x_s_std_output(sd: *mut v4l2_subdev, std: v4l2_std_id) -> c_int {
    static int ak881x_s_std_output(struct v4l2_subdev *sd, v4l2_std_id std)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ak881x *ak881x = to_ak881x(client);
    u8 vp1;
    if (std == V4L2_STD_NTSC_443) {
    vp1 = 3;
    ak881x.lines = 480;
    } else if (std == V4L2_STD_PAL_M) {
    vp1 = 5;
    ak881x.lines = 480;
    } else if (std == V4L2_STD_PAL_60) {
    vp1 = 7;
    ak881x.lines = 480;
    } else if (std & V4L2_STD_NTSC) {
    vp1 = 0;
    ak881x.lines = 480;
    } else if (std & V4L2_STD_PAL) {
    vp1 = 0xf;
    ak881x.lines = 576;
    } else {
// No SECAM or PAL_N/Nc supported
    return -EINVAL;
    }
    reg_set(client, AK881X_VIDEO_PROCESS1, vp1, 0xf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak881x_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ak881x_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ak881x *ak881x = to_ak881x(client);
    if (enable) {
    u8 dac;
// For colour-bar testing set bit 6 of AK881X_VIDEO_PROCESS1
// Default: composite output
    if (ak881x.pdata.flags & AK881X_COMPONENT)
    dac = 3;
    else
    dac = 4;
// Turn on the DAC(s)
    reg_write(client, AK881X_DAC_MODE, dac);
    dev_dbg(&client.dev, "chip status 0x%x\n",
    reg_read(client, AK881X_STATUS));
    } else {
// ...and clear bit 6 of AK881X_VIDEO_PROCESS1 here
    reg_write(client, AK881X_DAC_MODE, 0);
    dev_dbg(&client.dev, "chip status 0x%x\n",
    reg_read(client, AK881X_STATUS));
    }
    return 0;
    }
    static const struct v4l2_subdev_core_ops ak881x_subdev_core_ops = {

    .g_register	= ak881x_g_register,
    .s_register	= ak881x_s_register,

    };
    static const struct v4l2_subdev_video_ops ak881x_subdev_video_ops = {
    .s_std_output	= ak881x_s_std_output,
    .s_stream	= ak881x_s_stream,
    };
    static const struct v4l2_subdev_pad_ops ak881x_subdev_pad_ops = {
    .enum_mbus_code = ak881x_enum_mbus_code,
    .get_selection	= ak881x_get_selection,
    .set_fmt	= ak881x_fill_fmt,
    .get_fmt	= ak881x_fill_fmt,
    };
    static const struct v4l2_subdev_ops ak881x_subdev_ops = {
    .core	= &ak881x_subdev_core_ops,
    .video	= &ak881x_subdev_video_ops,
    .pad	= &ak881x_subdev_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn ak881x_probe(client: *mut i2c_client) -> c_int {
    static int ak881x_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct ak881x *ak881x;
    u8 ifmode, data;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_warn(&adapter.dev,
    "I2C-Adapter doesn't support I2C_FUNC_SMBUS_WORD\n");
    return -EIO;
    }
    ak881x = devm_kzalloc(&client.dev, sizeof(*ak881x), GFP_KERNEL);
    if (!ak881x)
    return -ENOMEM;
    v4l2_i2c_subdev_init(&ak881x.subdev, client, &ak881x_subdev_ops);
    data = reg_read(client, AK881X_DEVICE_ID);
    switch (data) {
    case 0x13:
    case 0x14:
    break;
    default:
    dev_err(&client.dev,
    "No ak881x chip detected, register read %x\n", data);
    return -ENODEV;
    }
    ak881x.revision = reg_read(client, AK881X_DEVICE_REVISION);
    ak881x.pdata = client.dev.platform_data;
    if (ak881x.pdata) {
    if (ak881x.pdata.flags & AK881X_FIELD)
    ifmode = 4;
    else
    ifmode = 0;
    switch (ak881x.pdata.flags & AK881X_IF_MODE_MASK) {
    case AK881X_IF_MODE_BT656:
    ifmode |= 1;
    break;
    case AK881X_IF_MODE_MASTER:
    ifmode |= 2;
    break;
    case AK881X_IF_MODE_SLAVE:
    default:
    break;
    }
    dev_dbg(&client.dev, "IF mode %x\n", ifmode);
//
// "Line Blanking No." seems to be the same as the number of
// "black" lines on, e.g., SuperH VOU, whose default value of 20
// "incidentally" matches ak881x' default
//
    reg_write(client, AK881X_INTERFACE_MODE, ifmode | (20 << 3));
    }
// Hardware default: NTSC-M
    ak881x.lines = 480;
    dev_info(&client.dev, "Detected an ak881x chip ID %x, revision %x\n",
    data, ak881x.revision);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ak881x_remove(client: *mut i2c_client) {
    static void ak881x_remove(struct i2c_client *client)
    {
    struct ak881x *ak881x = to_ak881x(client);
    v4l2_device_unregister_subdev(&ak881x.subdev);
    }
    static const struct i2c_device_id ak881x_id[] = {
    { .name = "ak8813" },
    { .name = "ak8814" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ak881x_id);
    static struct i2c_driver ak881x_i2c_driver = {
    .driver = {
    .name = "ak881x",
    },
    .probe		= ak881x_probe,
    .remove		= ak881x_remove,
    .id_table	= ak881x_id,
    };
    module_i2c_driver(ak881x_i2c_driver);
    MODULE_DESCRIPTION("TV-output driver for ak8813/ak8814");
    MODULE_AUTHOR("Guennadi Liakhovetski <g.liakhovetski@gmx.de>");
    MODULE_LICENSE("GPL v2");
