//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ml86v7667.c
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
// OKI Semiconductor ML86V7667 video decoder driver
//
// Author: Vladimir Barinov <source@cogentembedded.com>
// Copyright (C) 2013 Cogent Embedded, Inc.
// Copyright (C) 2013 Renesas Solutions Corp.
//

// Subaddresses
pub const MRA_REG: c_uint = 0x00 /* Mode Register A */;
pub const MRC_REG: c_uint = 0x02 /* Mode Register C */;
pub const LUMC_REG: c_uint = 0x0C /* Luminance Control */;
pub const CLC_REG: c_uint = 0x10 /* Contrast level control */;
pub const SSEPL_REG: c_uint = 0x11 /* Sync separation level */;
pub const CHRCA_REG: c_uint = 0x12 /* Chrominance Control A */;
pub const ACCC_REG: c_uint = 0x14 /* ACC Loop filter & Chrominance control */;
pub const ACCRC_REG: c_uint = 0x15 /* ACC Reference level control */;
pub const HUE_REG: c_uint = 0x16 /* Hue control */;
pub const ADC2_REG: c_uint = 0x1F /* ADC Register 2 */;
pub const PLLR1_REG: c_uint = 0x20 /* PLL Register 1 */;
pub const STATUS_REG: c_uint = 0x2C /* STATUS Register */;
// Mode Register A register bits

// Mode Register C register bits

// Luminance Control register bits
pub const LUMC_ONOFF_SHIFT: c_int = 7;

// Contrast level control register bits

pub const CLC_CONTRAST_MASK: c_uint = 0x0F;
// Sync separation level register bits

pub const SSEPL_LUMINANCE_MASK: c_uint = 0x7F;
// Chrominance Control A register bits
pub const CHRCA_MODE_SHIFT: c_int = 6;

// ACC Loop filter & Chrominance control register bits
pub const ACCC_CHROMA_CR_SHIFT: c_int = 3;

pub const ACCC_CHROMA_CB_SHIFT: c_int = 0;

// ACC Reference level control register bits
pub const ACCRC_CHROMA_MASK: c_uint = 0xfc;
pub const ACCRC_CHROMA_SHIFT: c_int = 2;
// ADC Register 2 register bits

// PLL Register 1 register bits

// STATUS Register register bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ml86v7667_priv {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub std: v4l2_std_id,
}

    static inline struct ml86v7667_priv *to_ml86v7667(struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct ml86v7667_priv, sd);
    }
    static inline struct v4l2_subdev *to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct ml86v7667_priv, hdl).sd;
    }
    static int ml86v7667_mask_set(struct i2c_client *client, const u8 reg,
    const u8 mask, const u8 data)
    {
    let mut val: c_int = i2c_smbus_read_byte_data(client, reg);
    if (val < 0)
    return val;
    val = (val & ~mask) | (data & mask);
    return i2c_smbus_write_byte_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ml86v7667_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_sd(ctrl);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    let mut ret: c_int = -EINVAL;
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    ret = ml86v7667_mask_set(client, SSEPL_REG,
    SSEPL_LUMINANCE_MASK, ctrl.val);
    break;
    case V4L2_CID_CONTRAST:
    ret = ml86v7667_mask_set(client, CLC_REG,
    CLC_CONTRAST_MASK, ctrl.val);
    break;
    case V4L2_CID_CHROMA_GAIN:
    ret = ml86v7667_mask_set(client, ACCRC_REG, ACCRC_CHROMA_MASK,
    ctrl.val << ACCRC_CHROMA_SHIFT);
    break;
    case V4L2_CID_HUE:
    ret = ml86v7667_mask_set(client, HUE_REG, ~0, ctrl.val);
    break;
    case V4L2_CID_RED_BALANCE:
    ret = ml86v7667_mask_set(client, ACCC_REG,
    ACCC_CHROMA_CR_MASK,
    ctrl.val << ACCC_CHROMA_CR_SHIFT);
    break;
    case V4L2_CID_BLUE_BALANCE:
    ret = ml86v7667_mask_set(client, ACCC_REG,
    ACCC_CHROMA_CB_MASK,
    ctrl.val << ACCC_CHROMA_CB_SHIFT);
    break;
    case V4L2_CID_SHARPNESS:
    ret = ml86v7667_mask_set(client, LUMC_REG,
    LUMC_ONOFF_MASK,
    ctrl.val << LUMC_ONOFF_SHIFT);
    break;
    case V4L2_CID_COLOR_KILLER:
    ret = ml86v7667_mask_set(client, CHRCA_REG,
    CHRCA_MODE_MASK,
    ctrl.val << CHRCA_MODE_SHIFT);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_querystd(sd: *mut v4l2_subdev, std: *mut v4l2_std_id) -> c_int {
    static int ml86v7667_querystd(struct v4l2_subdev *sd, v4l2_std_id *std)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int status;
    status = i2c_smbus_read_byte_data(client, STATUS_REG);
    if (status < 0)
    return status;
    if (status & STATUS_HLOCK_DETECT)
// std &= status & STATUS_NTSCPAL ? V4L2_STD_625_50 : V4L2_STD_525_60;
    else
// std = V4L2_STD_UNKNOWN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_g_input_status(sd: *mut v4l2_subdev, status: *mut u32) -> c_int {
    static int ml86v7667_g_input_status(struct v4l2_subdev *sd, u32 *status)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int status_reg;
    status_reg = i2c_smbus_read_byte_data(client, STATUS_REG);
    if (status_reg < 0)
    return status_reg;
// status = status_reg & STATUS_HLOCK_DETECT ? 0 : V4L2_IN_ST_NO_SIGNAL;
    return 0;
    }
    static int ml86v7667_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_YUYV8_2X8;
    return 0;
    }
    static int ml86v7667_fill_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ml86v7667_priv *priv = to_ml86v7667(sd);
    struct v4l2_mbus_framefmt *fmt = &format.format;
    if (format.pad)
    return -EINVAL;
    fmt.code = MEDIA_BUS_FMT_YUYV8_2X8;
    fmt.colorspace = V4L2_COLORSPACE_SMPTE170M;
// The top field is always transferred first by the chip
    fmt.field = V4L2_FIELD_INTERLACED_TB;
    fmt.width = 720;
    fmt.height = priv.std & V4L2_STD_525_60 ? 480 : 576;
    return 0;
    }
    static int ml86v7667_get_mbus_config(struct v4l2_subdev *sd,
    unsigned int pad,
    struct v4l2_mbus_config *cfg)
    {
    cfg.type = V4L2_MBUS_BT656;
    cfg.bus.parallel.flags = V4L2_MBUS_MASTER |
    V4L2_MBUS_PCLK_SAMPLE_RISING |
    V4L2_MBUS_DATA_ACTIVE_HIGH;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_g_std(sd: *mut v4l2_subdev, std: *mut v4l2_std_id) -> c_int {
    static int ml86v7667_g_std(struct v4l2_subdev *sd, v4l2_std_id *std)
    {
    struct ml86v7667_priv *priv = to_ml86v7667(sd);
// std = priv->std;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_s_std(sd: *mut v4l2_subdev, std: v4l2_std_id) -> c_int {
    static int ml86v7667_s_std(struct v4l2_subdev *sd, v4l2_std_id std)
    {
    struct ml86v7667_priv *priv = to_ml86v7667(sd);
    struct i2c_client *client = v4l2_get_subdevdata(&priv.sd);
    int ret;
    u8 mode;
// PAL/NTSC ITU-R BT.601 input mode
    mode = std & V4L2_STD_525_60 ? MRA_NTSC_BT601 : MRA_PAL_BT601;
    ret = ml86v7667_mask_set(client, MRA_REG, MRA_INPUT_MODE_MASK, mode);
    if (ret < 0)
    return ret;
    priv.std = std;
    return 0;
    }

    static int ml86v7667_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int ret;
    ret = i2c_smbus_read_byte_data(client, (u8)reg.reg);
    if (ret < 0)
    return ret;
    reg.val = ret;
    reg.size = sizeof(u8);
    return 0;
    }
    static int ml86v7667_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    return i2c_smbus_write_byte_data(client, (u8)reg.reg, (u8)reg.val);
    }

    static const struct v4l2_ctrl_ops ml86v7667_ctrl_ops = {
    .s_ctrl = ml86v7667_s_ctrl,
    };
    static const struct v4l2_subdev_video_ops ml86v7667_subdev_video_ops = {
    .g_std = ml86v7667_g_std,
    .s_std = ml86v7667_s_std,
    .querystd = ml86v7667_querystd,
    .g_input_status = ml86v7667_g_input_status,
    };
    static const struct v4l2_subdev_pad_ops ml86v7667_subdev_pad_ops = {
    .enum_mbus_code = ml86v7667_enum_mbus_code,
    .get_fmt = ml86v7667_fill_fmt,
    .set_fmt = ml86v7667_fill_fmt,
    .get_mbus_config = ml86v7667_get_mbus_config,
    };
    static const struct v4l2_subdev_core_ops ml86v7667_subdev_core_ops = {

    .g_register = ml86v7667_g_register,
    .s_register = ml86v7667_s_register,

    };
    static const struct v4l2_subdev_ops ml86v7667_subdev_ops = {
    .core = &ml86v7667_subdev_core_ops,
    .video = &ml86v7667_subdev_video_ops,
    .pad = &ml86v7667_subdev_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn ml86v7667_init(priv: *mut ml86v7667_priv) -> c_int {
    static int ml86v7667_init(struct ml86v7667_priv *priv)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&priv.sd);
    int val;
    int ret;
// BT.656-4 output mode, register mode
    ret = ml86v7667_mask_set(client, MRA_REG,
    MRA_OUTPUT_MODE_MASK | MRA_REGISTER_MODE,
    MRA_ITUR_BT656 | MRA_REGISTER_MODE);
// PLL circuit fixed clock, 32MHz
    ret |= ml86v7667_mask_set(client, PLLR1_REG, PLLR1_FIXED_CLOCK,
    PLLR1_FIXED_CLOCK);
// ADC2 clamping voltage maximum
    ret |= ml86v7667_mask_set(client, ADC2_REG, ADC2_CLAMP_VOLTAGE_MASK,
    ADC2_CLAMP_VOLTAGE(7));
// enable luminance function
    ret |= ml86v7667_mask_set(client, SSEPL_REG, SSEPL_LUMINANCE_ONOFF,
    SSEPL_LUMINANCE_ONOFF);
// enable contrast function
    ret |= ml86v7667_mask_set(client, CLC_REG, CLC_CONTRAST_ONOFF, 0);
//
// PAL/NTSC autodetection is enabled after reset,
// set the autodetected std in manual std mode and
// disable autodetection
//
    val = i2c_smbus_read_byte_data(client, STATUS_REG);
    if (val < 0)
    return val;
    priv.std = val & STATUS_NTSCPAL ? V4L2_STD_625_50 : V4L2_STD_525_60;
    ret |= ml86v7667_mask_set(client, MRC_REG, MRC_AUTOSELECT, 0);
    val = priv.std & V4L2_STD_525_60 ? MRA_NTSC_BT601 : MRA_PAL_BT601;
    ret |= ml86v7667_mask_set(client, MRA_REG, MRA_INPUT_MODE_MASK, val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_probe(client: *mut i2c_client) -> c_int {
    static int ml86v7667_probe(struct i2c_client *client)
    {
    struct ml86v7667_priv *priv;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    v4l2_i2c_subdev_init(&priv.sd, client, &ml86v7667_subdev_ops);
    v4l2_ctrl_handler_init(&priv.hdl, 8);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_BRIGHTNESS, -64, 63, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_CONTRAST, -8, 7, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_CHROMA_GAIN, -32, 31, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_HUE, -128, 127, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_RED_BALANCE, -4, 3, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_BLUE_BALANCE, -4, 3, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_SHARPNESS, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&priv.hdl, &ml86v7667_ctrl_ops,
    V4L2_CID_COLOR_KILLER, 0, 1, 1, 0);
    priv.sd.ctrl_handler = &priv.hdl;
    ret = priv.hdl.error;
    if (ret)
    goto cleanup;
    v4l2_ctrl_handler_setup(&priv.hdl);
    ret = ml86v7667_init(priv);
    if (ret)
    goto cleanup;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr, client.adapter.name);
    return 0;
    cleanup:
    v4l2_ctrl_handler_free(&priv.hdl);
    v4l2_device_unregister_subdev(&priv.sd);
    v4l_err(client, "failed to probe @ 0x%02x (%s)\n",
    client.addr, client.adapter.name);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ml86v7667_remove(client: *mut i2c_client) {
    static void ml86v7667_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ml86v7667_priv *priv = to_ml86v7667(sd);
    v4l2_ctrl_handler_free(&priv.hdl);
    v4l2_device_unregister_subdev(&priv.sd);
    }
    static const struct i2c_device_id ml86v7667_id[] = {
    { .name = DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ml86v7667_id);
    static struct i2c_driver ml86v7667_i2c_driver = {
    .driver = {
    .name	= DRV_NAME,
    },
    .probe		= ml86v7667_probe,
    .remove		= ml86v7667_remove,
    .id_table	= ml86v7667_id,
    };
    module_i2c_driver(ml86v7667_i2c_driver);
    MODULE_DESCRIPTION("OKI Semiconductor ML86V7667 video decoder driver");
    MODULE_AUTHOR("Vladimir Barinov");
    MODULE_LICENSE("GPL");
