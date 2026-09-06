//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov7740.c
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
// Copyright (c) 2017 Microchip Corporation.

pub const REG_OUTSIZE_LSB: c_uint = 0x34;
// OV7740 register tables
pub const REG_GAIN: c_uint = 0x00	/* Gain lower 8 bits (rest in vref) */;
pub const REG_BGAIN: c_uint = 0x01	/* blue gain */;
pub const REG_RGAIN: c_uint = 0x02	/* red gain */;
pub const REG_GGAIN: c_uint = 0x03	/* green gain */;
pub const REG_REG04: c_uint = 0x04	/* analog setting, don't change*/;
pub const REG_BAVG: c_uint = 0x05	/* b channel average */;
pub const REG_GAVG: c_uint = 0x06	/* g channel average */;
pub const REG_RAVG: c_uint = 0x07	/* r channel average */;
pub const REG_REG0C: c_uint = 0x0C	/* filp enable */;
pub const REG0C_IMG_FLIP: c_uint = 0x80;
pub const REG0C_IMG_MIRROR: c_uint = 0x40;
pub const REG_REG0E: c_uint = 0x0E	/* blc line */;
pub const REG_HAEC: c_uint = 0x0F	/* auto exposure cntrl */;
pub const REG_AEC: c_uint = 0x10	/* auto exposure cntrl */;
pub const REG_CLK: c_uint = 0x11	/* Clock control */;
pub const REG_REG55: c_uint = 0x55	/* Clock PLL DIV/PreDiv */;
pub const REG_REG12: c_uint = 0x12;
pub const REG_REG13: c_uint = 0x13	/* auto/manual AGC, AEC, Write Balance*/;
pub const REG13_AEC_EN: c_uint = 0x01;
pub const REG13_AGC_EN: c_uint = 0x04;
pub const REG_REG14: c_uint = 0x14;
pub const REG_CTRL15: c_uint = 0x15;
pub const REG15_GAIN_MSB: c_uint = 0x03;
pub const REG_REG16: c_uint = 0x16;
pub const REG_MIDH: c_uint = 0x1C	/* manufacture id byte */;
pub const REG_MIDL: c_uint = 0x1D	/* manufacture id byre */;
pub const REG_PIDH: c_uint = 0x0A	/* Product ID MSB */;
pub const REG_PIDL: c_uint = 0x0B	/* Product ID LSB */;
pub const REG_84: c_uint = 0x84	/* lots of stuff */;
pub const REG_REG38: c_uint = 0x38	/* sub-addr */;
pub const REG_AHSTART: c_uint = 0x17	/* Horiz start high bits */;
pub const REG_AHSIZE: c_uint = 0x18;
pub const REG_AVSTART: c_uint = 0x19	/* Vert start high bits */;
pub const REG_AVSIZE: c_uint = 0x1A;
pub const REG_PSHFT: c_uint = 0x1b	/* Pixel delay after HREF */;
pub const REG_HOUTSIZE: c_uint = 0x31;
pub const REG_VOUTSIZE: c_uint = 0x32;
pub const REG_HVSIZEOFF: c_uint = 0x33;
pub const REG_REG34: c_uint = 0x34	/* DSP output size H/V LSB*/;
pub const REG_ISP_CTRL00: c_uint = 0x80;
pub const ISPCTRL00_AWB_EN: c_uint = 0x10;
pub const ISPCTRL00_AWB_GAIN_EN: c_uint = 0x04;
pub const REG_YGAIN: c_uint = 0xE2	/* ygain for contrast control */;
pub const REG_YBRIGHT: c_uint = 0xE3;
pub const REG_SGNSET: c_uint = 0xE4;
pub const SGNSET_YBRIGHT_MASK: c_uint = 0x08;
pub const REG_USAT: c_uint = 0xDD;
pub const REG_VSAT: c_uint = 0xDE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov7740 {
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub format: v4l2_mbus_framefmt,
    pub /: *const *const *const ov7740_pixfmt fmt; / Current format,
    pub frmsize: *const ov7740_framesize,
    pub regmap: *mut regmap,
    pub xvclk: *mut clk,
    pub ctrl_handler: v4l2_ctrl_handler,
    struct {
// gain cluster
    pub auto_gain: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
}

    struct {
    struct v4l2_ctrl *auto_wb;
    struct v4l2_ctrl *blue_balance;
    struct v4l2_ctrl *red_balance;
    };
    struct {
    struct v4l2_ctrl *hflip;
    struct v4l2_ctrl *vflip;
    };
    struct {
// exposure cluster
    struct v4l2_ctrl *auto_exposure;
    struct v4l2_ctrl *exposure;
    };
    struct {
// saturation/hue cluster
    struct v4l2_ctrl *saturation;
    struct v4l2_ctrl *hue;
    };
    struct v4l2_ctrl *brightness;
    struct v4l2_ctrl *contrast;
    struct mutex mutex;	/* To serialize asynchronous callbacks */
    struct gpio_desc *resetb_gpio;
    struct gpio_desc *pwdn_gpio;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov7740_pixfmt {
    pub mbus_code: u32,
    pub colorspace: enum v4l2_colorspace,
    pub regs: *const reg_sequence,
    pub reg_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov7740_framesize {
    pub width: u16,
    pub height: u16,
    pub regs: *const reg_sequence,
    pub reg_num: u32,
}

    static const struct reg_sequence ov7740_vga[] = {
    {0x55, 0x40},
    {0x11, 0x02},
    {0xd5, 0x10},
    {0x0c, 0x12},
    {0x0d, 0x34},
    {0x17, 0x25},
    {0x18, 0xa0},
    {0x19, 0x03},
    {0x1a, 0xf0},
    {0x1b, 0x89},
    {0x22, 0x03},
    {0x29, 0x18},
    {0x2b, 0xf8},
    {0x2c, 0x01},
    {REG_HOUTSIZE, 0xa0},
    {REG_VOUTSIZE, 0xf0},
    {0x33, 0xc4},
    {REG_OUTSIZE_LSB, 0x0},
    {0x35, 0x05},
    {0x04, 0x60},
    {0x27, 0x80},
    {0x3d, 0x0f},
    {0x3e, 0x80},
    {0x3f, 0x40},
    {0x40, 0x7f},
    {0x41, 0x6a},
    {0x42, 0x29},
    {0x44, 0x22},
    {0x45, 0x41},
    {0x47, 0x02},
    {0x49, 0x64},
    {0x4a, 0xa1},
    {0x4b, 0x40},
    {0x4c, 0x1a},
    {0x4d, 0x50},
    {0x4e, 0x13},
    {0x64, 0x00},
    {0x67, 0x88},
    {0x68, 0x1a},
    {0x14, 0x28},
    {0x24, 0x3c},
    {0x25, 0x30},
    {0x26, 0x72},
    {0x50, 0x97},
    {0x51, 0x1f},
    {0x52, 0x00},
    {0x53, 0x00},
    {0x20, 0x00},
    {0x21, 0xcf},
    {0x50, 0x4b},
    {0x38, 0x14},
    {0xe9, 0x00},
    {0x56, 0x55},
    {0x57, 0xff},
    {0x58, 0xff},
    {0x59, 0xff},
    {0x5f, 0x04},
    {0xec, 0x00},
    {0x13, 0xff},
    {0x81, 0x3f},
    {0x82, 0x32},
    {0x38, 0x11},
    {0x84, 0x70},
    {0x85, 0x00},
    {0x86, 0x03},
    {0x87, 0x01},
    {0x88, 0x05},
    {0x89, 0x30},
    {0x8d, 0x30},
    {0x8f, 0x85},
    {0x93, 0x30},
    {0x95, 0x85},
    {0x99, 0x30},
    {0x9b, 0x85},
    {0x9c, 0x08},
    {0x9d, 0x12},
    {0x9e, 0x23},
    {0x9f, 0x45},
    {0xa0, 0x55},
    {0xa1, 0x64},
    {0xa2, 0x72},
    {0xa3, 0x7f},
    {0xa4, 0x8b},
    {0xa5, 0x95},
    {0xa6, 0xa7},
    {0xa7, 0xb5},
    {0xa8, 0xcb},
    {0xa9, 0xdd},
    {0xaa, 0xec},
    {0xab, 0x1a},
    {0xce, 0x78},
    {0xcf, 0x6e},
    {0xd0, 0x0a},
    {0xd1, 0x0c},
    {0xd2, 0x84},
    {0xd3, 0x90},
    {0xd4, 0x1e},
    {0x5a, 0x24},
    {0x5b, 0x1f},
    {0x5c, 0x88},
    {0x5d, 0x60},
    {0xac, 0x6e},
    {0xbe, 0xff},
    {0xbf, 0x00},
    {0x0f, 0x1d},
    {0x0f, 0x1f},
    };
    static const struct ov7740_framesize ov7740_framesizes[] = {
    {
    .width		= VGA_WIDTH,
    .height		= VGA_HEIGHT,
    .regs		= ov7740_vga,
    .reg_num	= ARRAY_SIZE(ov7740_vga),
    },
    };

    static int ov7740_get_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    struct regmap *regmap = ov7740.regmap;
    let mut val: c_uint = 0;
    int ret;
    ret = regmap_read(regmap, reg.reg & 0xff, &val);
    reg.val = val;
    reg.size = 1;
    return ret;
    }
    static int ov7740_set_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    struct regmap *regmap = ov7740.regmap;
    regmap_write(regmap, reg.reg & 0xff, reg.val & 0xff);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ov7740_set_power(ov7740: *mut ov7740, on: c_int) -> c_int {
    static int ov7740_set_power(struct ov7740 *ov7740, int on)
    {
    int ret;
    if (on) {
    ret = clk_prepare_enable(ov7740.xvclk);
    if (ret)
    return ret;
    if (ov7740.pwdn_gpio)
    gpiod_direction_output(ov7740.pwdn_gpio, 0);
    if (ov7740.resetb_gpio) {
    gpiod_set_value(ov7740.resetb_gpio, 1);
    usleep_range(500, 1000);
    gpiod_set_value(ov7740.resetb_gpio, 0);
    usleep_range(3000, 5000);
    }
    } else {
    clk_disable_unprepare(ov7740.xvclk);
    if (ov7740.pwdn_gpio)
    gpiod_direction_output(ov7740.pwdn_gpio, 0);
    }
    return 0;
    }
    static const struct v4l2_subdev_core_ops ov7740_subdev_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,

    .g_register = ov7740_get_register,
    .s_register = ov7740_set_register,

    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
#[no_mangle]
unsafe extern "C" fn ov7740_set_white_balance(ov7740: *mut ov7740, awb: c_int) -> c_int {
    static int ov7740_set_white_balance(struct ov7740 *ov7740, int awb)
    {
    struct regmap *regmap = ov7740.regmap;
    unsigned int value;
    int ret;
    ret = regmap_read(regmap, REG_ISP_CTRL00, &value);
    if (!ret) {
    if (awb)
    value |= (ISPCTRL00_AWB_EN | ISPCTRL00_AWB_GAIN_EN);
    else
    value &= ~(ISPCTRL00_AWB_EN | ISPCTRL00_AWB_GAIN_EN);
    ret = regmap_write(regmap, REG_ISP_CTRL00, value);
    if (ret)
    return ret;
    }
    if (!awb) {
    ret = regmap_write(regmap, REG_BGAIN,
    ov7740.blue_balance.val);
    if (ret)
    return ret;
    ret = regmap_write(regmap, REG_RGAIN, ov7740.red_balance.val);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_saturation(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_saturation(struct regmap *regmap, int value)
    {
    int ret;
    ret = regmap_write(regmap, REG_USAT, (unsigned char)value);
    if (ret)
    return ret;
    return regmap_write(regmap, REG_VSAT, (unsigned char)value);
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_gain(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_gain(struct regmap *regmap, int value)
    {
    int ret;
    ret = regmap_write(regmap, REG_GAIN, value & 0xff);
    if (ret)
    return ret;
    ret = regmap_update_bits(regmap, REG_CTRL15,
    REG15_GAIN_MSB, (value >> 8) & 0x3);
    if (!ret)
    ret = regmap_update_bits(regmap, REG_REG13, REG13_AGC_EN, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_autogain(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_autogain(struct regmap *regmap, int value)
    {
    unsigned int reg;
    int ret;
    ret = regmap_read(regmap, REG_REG13, &reg);
    if (ret)
    return ret;
    if (value)
    reg |= REG13_AGC_EN;
    else
    reg &= ~REG13_AGC_EN;
    return regmap_write(regmap, REG_REG13, reg);
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_brightness(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_brightness(struct regmap *regmap, int value)
    {
// Turn off AEC/AGC
    regmap_update_bits(regmap, REG_REG13, REG13_AEC_EN, 0);
    regmap_update_bits(regmap, REG_REG13, REG13_AGC_EN, 0);
    if (value >= 0) {
    regmap_write(regmap, REG_YBRIGHT, (unsigned char)value);
    regmap_update_bits(regmap, REG_SGNSET, SGNSET_YBRIGHT_MASK, 0);
    } else{
    regmap_write(regmap, REG_YBRIGHT, (unsigned char)(-value));
    regmap_update_bits(regmap, REG_SGNSET, SGNSET_YBRIGHT_MASK, 1);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_contrast(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_contrast(struct regmap *regmap, int value)
    {
    return regmap_write(regmap, REG_YGAIN, (unsigned char)value);
    }
#[no_mangle]
unsafe extern "C" fn ov7740_get_gain(ov7740: *mut ov7740, ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov7740_get_gain(struct ov7740 *ov7740, struct v4l2_ctrl *ctrl)
    {
    struct regmap *regmap = ov7740.regmap;
    unsigned int value0, value1;
    int ret;
    if (!ctrl.val)
    return 0;
    ret = regmap_read(regmap, REG_GAIN, &value0);
    if (ret)
    return ret;
    ret = regmap_read(regmap, REG_CTRL15, &value1);
    if (ret)
    return ret;
    ov7740.gain.val = (value1 << 8) | (value0 & 0xff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_get_exp(ov7740: *mut ov7740, ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov7740_get_exp(struct ov7740 *ov7740, struct v4l2_ctrl *ctrl)
    {
    struct regmap *regmap = ov7740.regmap;
    unsigned int value0, value1;
    int ret;
    if (ctrl.val == V4L2_EXPOSURE_MANUAL)
    return 0;
    ret = regmap_read(regmap, REG_AEC, &value0);
    if (ret)
    return ret;
    ret = regmap_read(regmap, REG_HAEC, &value1);
    if (ret)
    return ret;
    ov7740.exposure.val = (value1 << 8) | (value0 & 0xff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_exp(regmap: *mut regmap, value: c_int) -> c_int {
    static int ov7740_set_exp(struct regmap *regmap, int value)
    {
    int ret;
// Turn off AEC/AGC
    ret = regmap_update_bits(regmap, REG_REG13,
    REG13_AEC_EN | REG13_AGC_EN, 0);
    if (ret)
    return ret;
    ret = regmap_write(regmap, REG_AEC, (unsigned char)value);
    if (ret)
    return ret;
    return regmap_write(regmap, REG_HAEC, (unsigned char)(value >> 8));
    }
    static int ov7740_set_autoexp(struct regmap *regmap,
    enum v4l2_exposure_auto_type value)
    {
    unsigned int reg;
    int ret;
    ret = regmap_read(regmap, REG_REG13, &reg);
    if (!ret) {
    if (value == V4L2_EXPOSURE_AUTO)
    reg |= (REG13_AEC_EN | REG13_AGC_EN);
    else
    reg &= ~(REG13_AEC_EN | REG13_AGC_EN);
    ret = regmap_write(regmap, REG_REG13, reg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_get_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov7740_get_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov7740 *ov7740 = container_of(ctrl.handler,
    struct ov7740, ctrl_handler);
    int ret;
    switch (ctrl.id) {
    case V4L2_CID_AUTOGAIN:
    ret = ov7740_get_gain(ov7740, ctrl);
    break;
    case V4L2_CID_EXPOSURE_AUTO:
    ret = ov7740_get_exp(ov7740, ctrl);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov7740_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov7740 *ov7740 = container_of(ctrl.handler,
    struct ov7740, ctrl_handler);
    struct i2c_client *client = v4l2_get_subdevdata(&ov7740.subdev);
    struct regmap *regmap = ov7740.regmap;
    int ret;
    u8 val;
    if (!pm_runtime_get_if_in_use(&client.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_AUTO_WHITE_BALANCE:
    ret = ov7740_set_white_balance(ov7740, ctrl.val);
    break;
    case V4L2_CID_SATURATION:
    ret = ov7740_set_saturation(regmap, ctrl.val);
    break;
    case V4L2_CID_BRIGHTNESS:
    ret = ov7740_set_brightness(regmap, ctrl.val);
    break;
    case V4L2_CID_CONTRAST:
    ret = ov7740_set_contrast(regmap, ctrl.val);
    break;
    case V4L2_CID_VFLIP:
    val = ctrl.val ? REG0C_IMG_FLIP : 0x00;
    ret = regmap_update_bits(regmap, REG_REG0C,
    REG0C_IMG_FLIP, val);
    break;
    case V4L2_CID_HFLIP:
    val = ctrl.val ? REG0C_IMG_MIRROR : 0x00;
    ret = regmap_update_bits(regmap, REG_REG0C,
    REG0C_IMG_MIRROR, val);
    break;
    case V4L2_CID_AUTOGAIN:
    if (!ctrl.val)
    ret = ov7740_set_gain(regmap, ov7740.gain.val);
    else
    ret = ov7740_set_autogain(regmap, ctrl.val);
    break;
    case V4L2_CID_EXPOSURE_AUTO:
    if (ctrl.val == V4L2_EXPOSURE_MANUAL)
    ret = ov7740_set_exp(regmap, ov7740.exposure.val);
    else
    ret = ov7740_set_autoexp(regmap, ctrl.val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(&client.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov7740_ctrl_ops = {
    .g_volatile_ctrl = ov7740_get_volatile_ctrl,
    .s_ctrl = ov7740_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov7740_start_streaming(ov7740: *mut ov7740) -> c_int {
    static int ov7740_start_streaming(struct ov7740 *ov7740)
    {
    int ret;
    if (ov7740.fmt) {
    ret = regmap_multi_reg_write(ov7740.regmap,
    ov7740.fmt.regs,
    ov7740.fmt.reg_num);
    if (ret)
    return ret;
    }
    if (ov7740.frmsize) {
    ret = regmap_multi_reg_write(ov7740.regmap,
    ov7740.frmsize.regs,
    ov7740.frmsize.reg_num);
    if (ret)
    return ret;
    }
    return __v4l2_ctrl_handler_setup(ov7740.subdev.ctrl_handler);
    }
#[no_mangle]
unsafe extern "C" fn ov7740_set_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov7740_set_stream(struct v4l2_subdev *sd, int enable)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    let mut ret: c_int = 0;
    mutex_lock(&ov7740.mutex);
    if (enable) {
    ret = pm_runtime_resume_and_get(&client.dev);
    if (ret < 0)
    goto err_unlock;
    ret = ov7740_start_streaming(ov7740);
    if (ret)
    goto err_rpm_put;
    } else {
    pm_runtime_put(&client.dev);
    }
    mutex_unlock(&ov7740.mutex);
    return ret;
    err_rpm_put:
    pm_runtime_put(&client.dev);
    err_unlock:
    mutex_unlock(&ov7740.mutex);
    return ret;
    }
    static const struct v4l2_subdev_video_ops ov7740_subdev_video_ops = {
    .s_stream = ov7740_set_stream,
    };
    static const struct reg_sequence ov7740_format_yuyv[] = {
    {0x12, 0x00},
    {0x36, 0x3f},
    {0x80, 0x7f},
    {0x83, 0x01},
    };
    static const struct reg_sequence ov7740_format_bggr8[] = {
    {0x36, 0x2f},
    {0x80, 0x01},
    {0x83, 0x04},
    };
    static const struct ov7740_pixfmt ov7740_formats[] = {
    {
    .mbus_code = MEDIA_BUS_FMT_YUYV8_2X8,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .regs = ov7740_format_yuyv,
    .reg_num = ARRAY_SIZE(ov7740_format_yuyv),
    },
    {
    .mbus_code = MEDIA_BUS_FMT_SBGGR8_1X8,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .regs = ov7740_format_bggr8,
    .reg_num = ARRAY_SIZE(ov7740_format_bggr8),
    }
    };

    static int ov7740_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index >= N_OV7740_FMTS)
    return -EINVAL;
    code.code = ov7740_formats[code.index].mbus_code;
    return 0;
    }
    static int ov7740_enum_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval_enum *fie)
    {
    if (fie.pad)
    return -EINVAL;
    if (fie.index >= 1)
    return -EINVAL;
    if ((fie.width != VGA_WIDTH) || (fie.height != VGA_HEIGHT))
    return -EINVAL;
    fie.interval.numerator = 1;
    fie.interval.denominator = 60;
    return 0;
    }
    static int ov7740_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.pad)
    return -EINVAL;
    if (fse.index > 0)
    return -EINVAL;
    fse.min_width = fse.max_width = VGA_WIDTH;
    fse.min_height = fse.max_height = VGA_HEIGHT;
    return 0;
    }
    static int ov7740_try_fmt_internal(struct v4l2_subdev *sd,
    struct v4l2_mbus_framefmt *fmt,
    const struct ov7740_pixfmt **ret_fmt,
    const struct ov7740_framesize **ret_frmsize)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    const struct ov7740_framesize *fsize = &ov7740_framesizes[0];
    int index, i;
    for (index = 0; index < N_OV7740_FMTS; index++) {
    if (ov7740_formats[index].mbus_code == fmt.code)
    break;
    }
    if (index >= N_OV7740_FMTS) {
// default to first format
    index = 0;
    fmt.code = ov7740_formats[0].mbus_code;
    }
    if (ret_fmt != core::ptr::null_mut())
// ret_fmt = ov7740_formats + index;
    for (i = 0; i < ARRAY_SIZE(ov7740_framesizes); i++) {
    if ((fsize.width >= fmt.width) &&
    (fsize.height >= fmt.height)) {
    fmt.width = fsize.width;
    fmt.height = fsize.height;
    break;
    }
    fsize++;
    }
    if (i >= ARRAY_SIZE(ov7740_framesizes)) {
    fsize = &ov7740_framesizes[0];
    fmt.width = fsize.width;
    fmt.height = fsize.height;
    }
    if (ret_frmsize != core::ptr::null_mut())
// ret_frmsize = fsize;
    fmt.field = V4L2_FIELD_NONE;
    fmt.colorspace = ov7740_formats[index].colorspace;
    ov7740.format = *fmt;
    return 0;
    }
    static int ov7740_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    const struct ov7740_pixfmt *ovfmt;
    const struct ov7740_framesize *fsize;
    struct v4l2_mbus_framefmt *mbus_fmt;
    int ret;
    mutex_lock(&ov7740.mutex);
    if (format.pad) {
    ret = -EINVAL;
    goto error;
    }
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
    ret = ov7740_try_fmt_internal(sd, &format.format, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret)
    goto error;
    mbus_fmt = v4l2_subdev_state_get_format(sd_state, format.pad);
// mbus_fmt = format->format;
    mutex_unlock(&ov7740.mutex);
    return 0;
    }
    ret = ov7740_try_fmt_internal(sd, &format.format, &ovfmt, &fsize);
    if (ret)
    goto error;
    ov7740.fmt = ovfmt;
    ov7740.frmsize = fsize;
    mutex_unlock(&ov7740.mutex);
    return 0;
    error:
    mutex_unlock(&ov7740.mutex);
    return ret;
    }
    static int ov7740_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    struct v4l2_mbus_framefmt *mbus_fmt;
    mutex_lock(&ov7740.mutex);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
    mbus_fmt = v4l2_subdev_state_get_format(sd_state, 0);
    format.format = *mbus_fmt;
    } else {
    format.format = ov7740.format;
    }
    mutex_unlock(&ov7740.mutex);
    return 0;
    }
    static int ov7740_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *ival)
    {
    struct v4l2_fract *tpf = &ival.interval;
    tpf.numerator = 1;
    tpf.denominator = 60;
    return 0;
    }
    static const struct v4l2_subdev_pad_ops ov7740_subdev_pad_ops = {
    .enum_frame_interval = ov7740_enum_frame_interval,
    .enum_frame_size = ov7740_enum_frame_size,
    .enum_mbus_code = ov7740_enum_mbus_code,
    .get_fmt = ov7740_get_fmt,
    .set_fmt = ov7740_set_fmt,
    .get_frame_interval = ov7740_get_frame_interval,
    .set_frame_interval = ov7740_get_frame_interval,
    };
    static const struct v4l2_subdev_ops ov7740_subdev_ops = {
    .core	= &ov7740_subdev_core_ops,
    .video	= &ov7740_subdev_video_ops,
    .pad	= &ov7740_subdev_pad_ops,
    };
    static void ov7740_get_default_format(struct v4l2_subdev *sd,
    struct v4l2_mbus_framefmt *format)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    format.width = ov7740.frmsize.width;
    format.height = ov7740.frmsize.height;
    format.colorspace = ov7740.fmt.colorspace;
    format.code = ov7740.fmt.mbus_code;
    format.field = V4L2_FIELD_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ov7740_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    struct v4l2_mbus_framefmt *format =
    v4l2_subdev_state_get_format(fh.state, 0);
    mutex_lock(&ov7740.mutex);
    ov7740_get_default_format(sd, format);
    mutex_unlock(&ov7740.mutex);
    return 0;
    }
    static const struct v4l2_subdev_internal_ops ov7740_subdev_internal_ops = {
    .open = ov7740_open,
    };
    static int ov7740_probe_dt(struct i2c_client *client,
    struct ov7740 *ov7740)
    {
    ov7740.resetb_gpio = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov7740.resetb_gpio)) {
    dev_info(&client.dev, "can't get %s GPIO\n", "reset");
    return PTR_ERR(ov7740.resetb_gpio);
    }
    ov7740.pwdn_gpio = devm_gpiod_get_optional(&client.dev, "powerdown",
    GPIOD_OUT_LOW);
    if (IS_ERR(ov7740.pwdn_gpio)) {
    dev_info(&client.dev, "can't get %s GPIO\n", "powerdown");
    return PTR_ERR(ov7740.pwdn_gpio);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_detect(ov7740: *mut ov7740) -> c_int {
    static int ov7740_detect(struct ov7740 *ov7740)
    {
    struct regmap *regmap = ov7740.regmap;
    unsigned int midh, midl, pidh, pidl;
    int ret;
    ret = regmap_read(regmap, REG_MIDH, &midh);
    if (ret)
    return ret;
    if (midh != 0x7f)
    return -ENODEV;
    ret = regmap_read(regmap, REG_MIDL, &midl);
    if (ret)
    return ret;
    if (midl != 0xa2)
    return -ENODEV;
    ret = regmap_read(regmap, REG_PIDH, &pidh);
    if (ret)
    return ret;
    if (pidh != 0x77)
    return -ENODEV;
    ret = regmap_read(regmap, REG_PIDL, &pidl);
    if (ret)
    return ret;
    if ((pidl != 0x40) && (pidl != 0x41) && (pidl != 0x42))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_init_controls(ov7740: *mut ov7740) -> c_int {
    static int ov7740_init_controls(struct ov7740 *ov7740)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&ov7740.subdev);
    struct v4l2_ctrl_handler *ctrl_hdlr = &ov7740.ctrl_handler;
    int ret;
    ret = v4l2_ctrl_handler_init(ctrl_hdlr, 12);
    if (ret < 0)
    return ret;
    ctrl_hdlr.lock = &ov7740.mutex;
    ov7740.auto_wb = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_AUTO_WHITE_BALANCE,
    0, 1, 1, 1);
    ov7740.blue_balance = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_BLUE_BALANCE,
    0, 0xff, 1, 0x80);
    ov7740.red_balance = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_RED_BALANCE,
    0, 0xff, 1, 0x80);
    ov7740.brightness = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_BRIGHTNESS,
    -255, 255, 1, 0);
    ov7740.contrast = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_CONTRAST,
    0, 127, 1, 0x20);
    ov7740.saturation = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_SATURATION, 0, 256, 1, 0x80);
    ov7740.hflip = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    ov7740.vflip = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    ov7740.gain = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_GAIN, 0, 1023, 1, 500);
    ov7740.auto_gain = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_AUTOGAIN, 0, 1, 1, 1);
    ov7740.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &ov7740_ctrl_ops,
    V4L2_CID_EXPOSURE, 0, 65535, 1, 500);
    ov7740.auto_exposure = v4l2_ctrl_new_std_menu(ctrl_hdlr,
    &ov7740_ctrl_ops,
    V4L2_CID_EXPOSURE_AUTO,
    V4L2_EXPOSURE_MANUAL, 0,
    V4L2_EXPOSURE_AUTO);
    v4l2_ctrl_auto_cluster(3, &ov7740.auto_wb, 0, false);
    v4l2_ctrl_auto_cluster(2, &ov7740.auto_gain, 0, true);
    v4l2_ctrl_auto_cluster(2, &ov7740.auto_exposure,
    V4L2_EXPOSURE_MANUAL, true);
    if (ctrl_hdlr.error) {
    ret = ctrl_hdlr.error;
    dev_err(&client.dev, "controls initialisation failed (%d)\n",
    ret);
    goto error;
    }
    ret = v4l2_ctrl_handler_setup(ctrl_hdlr);
    if (ret) {
    dev_err(&client.dev, "%s control init failed (%d)\n",
    __func__, ret);
    goto error;
    }
    ov7740.subdev.ctrl_handler = ctrl_hdlr;
    return 0;
    error:
    v4l2_ctrl_handler_free(ctrl_hdlr);
    mutex_destroy(&ov7740.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_free_controls(ov7740: *mut ov7740) {
    static void ov7740_free_controls(struct ov7740 *ov7740)
    {
    v4l2_ctrl_handler_free(ov7740.subdev.ctrl_handler);
    mutex_destroy(&ov7740.mutex);
    }
pub const OV7740_MAX_REGISTER: c_uint = 0xff;
    static const struct regmap_config ov7740_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .max_register	= OV7740_MAX_REGISTER,
    };
#[no_mangle]
unsafe extern "C" fn ov7740_probe(client: *mut i2c_client) -> c_int {
    static int ov7740_probe(struct i2c_client *client)
    {
    struct ov7740 *ov7740;
    struct v4l2_subdev *sd;
    int ret;
    ov7740 = devm_kzalloc(&client.dev, sizeof(*ov7740), GFP_KERNEL);
    if (!ov7740)
    return -ENOMEM;
    ov7740.xvclk = devm_v4l2_sensor_clk_get(&client.dev, "xvclk");
    if (IS_ERR(ov7740.xvclk))
    return dev_err_probe(&client.dev, PTR_ERR(ov7740.xvclk),
    "OV7740: fail to get xvclk\n");
    ret = ov7740_probe_dt(client, ov7740);
    if (ret)
    return ret;
    ov7740.regmap = devm_regmap_init_sccb(client, &ov7740_regmap_config);
    if (IS_ERR(ov7740.regmap)) {
    ret = PTR_ERR(ov7740.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    sd = &ov7740.subdev;
    v4l2_i2c_subdev_init(sd, client, &ov7740_subdev_ops);
    sd.internal_ops = &ov7740_subdev_internal_ops;
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE | V4L2_SUBDEV_FL_HAS_EVENTS;
    ov7740.pad.flags = MEDIA_PAD_FL_SOURCE;
    sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&sd.entity, 1, &ov7740.pad);
    if (ret)
    return ret;
    ret = ov7740_set_power(ov7740, 1);
    if (ret)
    return ret;
    pm_runtime_set_active(&client.dev);
    pm_runtime_enable(&client.dev);
    ret = ov7740_detect(ov7740);
    if (ret)
    goto error_detect;
    mutex_init(&ov7740.mutex);
    ret = ov7740_init_controls(ov7740);
    if (ret)
    goto error_init_controls;
    v4l_info(client, "chip found @ 0x%02x (%s)\n",
    client.addr << 1, client.adapter.name);
    ov7740.fmt = &ov7740_formats[0];
    ov7740.frmsize = &ov7740_framesizes[0];
    ov7740_get_default_format(sd, &ov7740.format);
    ret = v4l2_async_register_subdev(sd);
    if (ret)
    goto error_async_register;
    pm_runtime_idle(&client.dev);
    return 0;
    error_async_register:
    v4l2_ctrl_handler_free(ov7740.subdev.ctrl_handler);
    error_init_controls:
    ov7740_free_controls(ov7740);
    error_detect:
    pm_runtime_disable(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    ov7740_set_power(ov7740, 0);
    media_entity_cleanup(&ov7740.subdev.entity);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_remove(client: *mut i2c_client) {
    static void ov7740_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    v4l2_async_unregister_subdev(sd);
    media_entity_cleanup(&ov7740.subdev.entity);
    ov7740_free_controls(ov7740);
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev))
    ov7740_set_power(ov7740, 0);
    pm_runtime_set_suspended(&client.dev);
    }
#[no_mangle]
unsafe extern "C" fn ov7740_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov7740_runtime_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    ov7740_set_power(ov7740, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov7740_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov7740_runtime_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov7740 *ov7740 = container_of(sd, struct ov7740, subdev);
    return ov7740_set_power(ov7740, 1);
    }
    static const struct i2c_device_id ov7740_id[] = {
    { .name = "ov7740" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, ov7740_id);
    static const struct dev_pm_ops ov7740_pm_ops = {
    SET_RUNTIME_PM_OPS(ov7740_runtime_suspend, ov7740_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id ov7740_of_match[] = {
    {.compatible = "ovti,ov7740", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ov7740_of_match);
    static struct i2c_driver ov7740_i2c_driver = {
    .driver = {
    .name = "ov7740",
    .pm = &ov7740_pm_ops,
    .of_match_table = ov7740_of_match,
    },
    .probe    = ov7740_probe,
    .remove   = ov7740_remove,
    .id_table = ov7740_id,
    };
    module_i2c_driver(ov7740_i2c_driver);
    MODULE_DESCRIPTION("The V4L2 driver for Omnivision 7740 sensor");
    MODULE_AUTHOR("Songjun Wu <songjun.wu@atmel.com>");
    MODULE_LICENSE("GPL v2");
