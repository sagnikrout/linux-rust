//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov2685.c
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
//
// ov2685 driver
//
// Copyright (C) 2017 Fuzhou Rockchip Electronics Co., Ltd.
//

pub const CHIP_ID: c_uint = 0x2685;
pub const OV2685_REG_CHIP_ID: c_uint = 0x300a;
pub const OV2685_XVCLK_FREQ: c_int = 24000000;
pub const REG_SC_CTRL_MODE: c_uint = 0x0100;
pub const SC_CTRL_MODE_STANDBY: c_uint = 0x0;

pub const OV2685_REG_EXPOSURE: c_uint = 0x3500;
pub const OV2685_EXPOSURE_MIN: c_int = 4;
pub const OV2685_EXPOSURE_STEP: c_int = 1;
pub const OV2685_REG_VTS: c_uint = 0x380e;
pub const OV2685_VTS_MAX: c_uint = 0x7fff;
pub const OV2685_REG_GAIN: c_uint = 0x350a;
pub const OV2685_GAIN_MIN: c_int = 0;
pub const OV2685_GAIN_MAX: c_uint = 0x07ff;
pub const OV2685_GAIN_STEP: c_uint = 0x1;
pub const OV2685_GAIN_DEFAULT: c_uint = 0x0036;
pub const OV2685_REG_TEST_PATTERN: c_uint = 0x5080;
pub const OV2685_TEST_PATTERN_DISABLED: c_uint = 0x00;
pub const OV2685_TEST_PATTERN_COLOR_BAR: c_uint = 0x80;
pub const OV2685_TEST_PATTERN_RANDOM: c_uint = 0x81;
pub const OV2685_TEST_PATTERN_COLOR_BAR_FADE: c_uint = 0x88;
pub const OV2685_TEST_PATTERN_BW_SQUARE: c_uint = 0x92;
pub const OV2685_TEST_PATTERN_COLOR_SQUARE: c_uint = 0x82;
pub const REG_NULL: c_uint = 0xFFFF;
pub const OV2685_REG_VALUE_08BIT: c_int = 1;
pub const OV2685_REG_VALUE_16BIT: c_int = 2;
pub const OV2685_REG_VALUE_24BIT: c_int = 3;
pub const OV2685_NATIVE_WIDTH: c_int = 1616;
pub const OV2685_NATIVE_HEIGHT: c_int = 1216;
pub const OV2685_LANES: c_int = 1;
pub const OV2685_BITS_PER_SAMPLE: c_int = 10;
    static const char * const ov2685_supply_names[] = {
    "avdd",		/* Analog power */
    "dovdd",	/* Digital I/O power */
    "dvdd",		/* Digital core power */
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regval {
    pub addr: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2685_mode {
    pub width: u32,
    pub height: u32,
    pub exp_def: u32,
    pub hts_def: u32,
    pub vts_def: u32,
    pub analog_crop: *const v4l2_rect,
    pub reg_list: *const regval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2685 {
    pub client: *mut i2c_client,
    pub xvclk: *mut clk,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; OV2685_NUM_SUPPLIES],
    pub mutex: mutex,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub anal_gain: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub test_pattern: *mut v4l2_ctrl,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub cur_mode: *const ov2685_mode,
}

// PLL settings bases on 24M xvclk
    static struct regval ov2685_1600x1200_regs[] = {
    {0x0103, 0x01},
    {0x0100, 0x00},
    {0x3002, 0x00},
    {0x3016, 0x1c},
    {0x3018, 0x44},
    {0x301d, 0xf0},
    {0x3020, 0x00},
    {0x3082, 0x37},
    {0x3083, 0x03},
    {0x3084, 0x09},
    {0x3085, 0x04},
    {0x3086, 0x00},
    {0x3087, 0x00},
    {0x3501, 0x4e},
    {0x3502, 0xe0},
    {0x3503, 0x27},
    {0x350b, 0x36},
    {0x3600, 0xb4},
    {0x3603, 0x35},
    {0x3604, 0x24},
    {0x3605, 0x00},
    {0x3620, 0x24},
    {0x3621, 0x34},
    {0x3622, 0x03},
    {0x3628, 0x10},
    {0x3705, 0x3c},
    {0x370a, 0x21},
    {0x370c, 0x50},
    {0x370d, 0xc0},
    {0x3717, 0x58},
    {0x3718, 0x80},
    {0x3720, 0x00},
    {0x3721, 0x09},
    {0x3722, 0x06},
    {0x3723, 0x59},
    {0x3738, 0x99},
    {0x3781, 0x80},
    {0x3784, 0x0c},
    {0x3789, 0x60},
    {0x3800, 0x00},
    {0x3801, 0x00},
    {0x3802, 0x00},
    {0x3803, 0x00},
    {0x3804, 0x06},
    {0x3805, 0x4f},
    {0x3806, 0x04},
    {0x3807, 0xbf},
    {0x3808, 0x06},
    {0x3809, 0x40},
    {0x380a, 0x04},
    {0x380b, 0xb0},
    {0x380c, 0x06},
    {0x380d, 0xa4},
    {0x380e, 0x05},
    {0x380f, 0x0e},
    {0x3810, 0x00},
    {0x3811, 0x08},
    {0x3812, 0x00},
    {0x3813, 0x08},
    {0x3814, 0x11},
    {0x3815, 0x11},
    {0x3819, 0x04},
    {0x3820, 0xc0},
    {0x3821, 0x00},
    {0x3a06, 0x01},
    {0x3a07, 0x84},
    {0x3a08, 0x01},
    {0x3a09, 0x43},
    {0x3a0a, 0x24},
    {0x3a0b, 0x60},
    {0x3a0c, 0x28},
    {0x3a0d, 0x60},
    {0x3a0e, 0x04},
    {0x3a0f, 0x8c},
    {0x3a10, 0x05},
    {0x3a11, 0x0c},
    {0x4000, 0x81},
    {0x4001, 0x40},
    {0x4008, 0x02},
    {0x4009, 0x09},
    {0x4300, 0x00},
    {0x430e, 0x00},
    {0x4602, 0x02},
    {0x481b, 0x40},
    {0x481f, 0x40},
    {0x4837, 0x18},
    {0x5000, 0x1f},
    {0x5001, 0x05},
    {0x5002, 0x30},
    {0x5003, 0x04},
    {0x5004, 0x00},
    {0x5005, 0x0c},
    {0x5280, 0x15},
    {0x5281, 0x06},
    {0x5282, 0x06},
    {0x5283, 0x08},
    {0x5284, 0x1c},
    {0x5285, 0x1c},
    {0x5286, 0x20},
    {0x5287, 0x10},
    {REG_NULL, 0x00}
    };
pub const OV2685_LINK_FREQ_330MHZ: c_int = 330000000;
    static const s64 link_freq_menu_items[] = {
    OV2685_LINK_FREQ_330MHZ
    };
    static const char * const ov2685_test_pattern_menu[] = {
    "Disabled",
    "Color Bar",
    "Color Bar FADE",
    "Random Data",
    "Black White Square",
    "Color Square"
    };
    static const int ov2685_test_pattern_val[] = {
    OV2685_TEST_PATTERN_DISABLED,
    OV2685_TEST_PATTERN_COLOR_BAR,
    OV2685_TEST_PATTERN_COLOR_BAR_FADE,
    OV2685_TEST_PATTERN_RANDOM,
    OV2685_TEST_PATTERN_BW_SQUARE,
    OV2685_TEST_PATTERN_COLOR_SQUARE,
    };
    static const struct v4l2_rect ov2685_analog_crop = {
    .left	= 8,
    .top	= 8,
    .width	= 1600,
    .height	= 1200,
    };
    static const struct ov2685_mode supported_modes[] = {
    {
    .width = 1600,
    .height = 1200,
    .exp_def = 0x04ee,
    .hts_def = 0x06a4,
    .vts_def = 0x050e,
    .analog_crop = &ov2685_analog_crop,
    .reg_list = ov2685_1600x1200_regs,
    },
    };
// Write registers up to 4 at a time
    static int ov2685_write_reg(struct i2c_client *client, u16 reg,
    u32 len, u32 val)
    {
    u32 val_i, buf_i;
    u8 buf[6];
    u8 *val_p;
    __be32 val_be;
    if (len > 4)
    return -EINVAL;
    buf[0] = reg >> 8;
    buf[1] = reg & 0xff;
    val_be = cpu_to_be32(val);
    val_p = (u8 *)&val_be;
    buf_i = 2;
    val_i = 4 - len;
    while (val_i < 4)
    buf[buf_i++] = val_p[val_i++];
    if (i2c_master_send(client, buf, len + 2) != len + 2)
    return -EIO;
    return 0;
    }
    static int ov2685_write_array(struct i2c_client *client,
    const struct regval *regs)
    {
    let mut ret: c_int = 0;
    u32 i;
    for (i = 0; ret == 0 && regs[i].addr != REG_NULL; i++)
    ret = ov2685_write_reg(client, regs[i].addr,
    OV2685_REG_VALUE_08BIT, regs[i].val);
    return ret;
    }
// Read registers up to 4 at a time
    static int ov2685_read_reg(struct i2c_client *client, u16 reg,
    u32 len, u32 *val)
    {
    struct i2c_msg msgs[2];
    u8 *data_be_p;
    let mut data_be: __be32 = 0;
    let mut reg_addr_be: __be16 = cpu_to_be16(reg);
    int ret;
    if (len > 4)
    return -EINVAL;
    data_be_p = (u8 *)&data_be;
// Write register address
    msgs[0].addr = client.addr;
    msgs[0].flags = 0;
    msgs[0].len = 2;
    msgs[0].buf = (u8 *)&reg_addr_be;
// Read data from register
    msgs[1].addr = client.addr;
    msgs[1].flags = I2C_M_RD;
    msgs[1].len = len;
    msgs[1].buf = &data_be_p[4 - len];
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs))
    return -EIO;
// val = be32_to_cpu(data_be);
    return 0;
    }
    static void ov2685_fill_fmt(const struct ov2685_mode *mode,
    struct v4l2_mbus_framefmt *fmt)
    {
    fmt.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    fmt.width = mode.width;
    fmt.height = mode.height;
    fmt.field = V4L2_FIELD_NONE;
    }
    static int ov2685_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov2685 *ov2685 = to_ov2685(sd);
    struct v4l2_mbus_framefmt *mbus_fmt = &fmt.format;
// only one mode supported for now
    ov2685_fill_fmt(ov2685.cur_mode, mbus_fmt);
    return 0;
    }
    static int ov2685_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov2685 *ov2685 = to_ov2685(sd);
    struct v4l2_mbus_framefmt *mbus_fmt = &fmt.format;
    ov2685_fill_fmt(ov2685.cur_mode, mbus_fmt);
    return 0;
    }
    static int ov2685_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index >= ARRAY_SIZE(supported_modes))
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    return 0;
    }
    static int ov2685_enum_frame_sizes(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    let mut index: c_int = fse.index;
    if (index >= ARRAY_SIZE(supported_modes))
    return -EINVAL;
    fse.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    fse.min_width  = supported_modes[index].width;
    fse.max_width  = supported_modes[index].width;
    fse.max_height = supported_modes[index].height;
    fse.min_height = supported_modes[index].height;
    return 0;
    }
    static const struct v4l2_rect *
    __ov2685_get_pad_crop(struct ov2685 *ov2685,
    struct v4l2_subdev_state *state, unsigned int pad,
    enum v4l2_subdev_format_whence which)
    {
    const struct ov2685_mode *mode = ov2685.cur_mode;
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
    return v4l2_subdev_state_get_crop(state, pad);
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    return mode.analog_crop;
    }
    return core::ptr::null_mut();
    }
    static int ov2685_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct ov2685 *ov2685 = to_ov2685(sd);
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    mutex_lock(&ov2685.mutex);
    sel.r = *__ov2685_get_pad_crop(ov2685, sd_state, sel.pad,
    sel.which);
    mutex_unlock(&ov2685.mutex);
    break;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = OV2685_NATIVE_WIDTH;
    sel.r.height = OV2685_NATIVE_HEIGHT;
    break;
    case V4L2_SEL_TGT_CROP_DEFAULT:
    sel.r = ov2685_analog_crop;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
// Calculate the delay in us by clock rate and clock cycles
#[no_mangle]
pub unsafe extern "C" fn ov2685_cal_delay(cycles: u32) -> u32 {
    static inline u32 ov2685_cal_delay(u32 cycles)
    {
    return DIV_ROUND_UP(cycles, OV2685_XVCLK_FREQ / 1000 / 1000);
    }
#[no_mangle]
unsafe extern "C" fn __ov2685_power_on(ov2685: *mut ov2685) -> c_int {
    static int __ov2685_power_on(struct ov2685 *ov2685)
    {
    int ret;
    u32 delay_us;
    struct device *dev = &ov2685.client.dev;
    ret = clk_prepare_enable(ov2685.xvclk);
    if (ret < 0) {
    dev_err(dev, "Failed to enable xvclk\n");
    return ret;
    }
    gpiod_set_value_cansleep(ov2685.reset_gpio, 1);
    ret = regulator_bulk_enable(OV2685_NUM_SUPPLIES, ov2685.supplies);
    if (ret < 0) {
    dev_err(dev, "Failed to enable regulators\n");
    goto disable_clk;
    }
// The minimum delay between power supplies and reset rising can be 0
    gpiod_set_value_cansleep(ov2685.reset_gpio, 0);
// 8192 xvclk cycles prior to the first SCCB transaction
    delay_us = ov2685_cal_delay(8192);
    usleep_range(delay_us, delay_us * 2);
// HACK: ov2685 would output messy data after reset(R0103),
// writing register before .s_stream() as a workaround
//
    ret = ov2685_write_array(ov2685.client, ov2685.cur_mode.reg_list);
    if (ret) {
    dev_err(dev, "Failed to set regs for power on\n");
    goto disable_supplies;
    }
    return 0;
    disable_supplies:
    regulator_bulk_disable(OV2685_NUM_SUPPLIES, ov2685.supplies);
    disable_clk:
    clk_disable_unprepare(ov2685.xvclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __ov2685_power_off(ov2685: *mut ov2685) {
    static void __ov2685_power_off(struct ov2685 *ov2685)
    {
// 512 xvclk cycles after the last SCCB transaction or MIPI frame end
    let mut delay_us: u32 = ov2685_cal_delay(512);
    usleep_range(delay_us, delay_us * 2);
    clk_disable_unprepare(ov2685.xvclk);
    gpiod_set_value_cansleep(ov2685.reset_gpio, 1);
    regulator_bulk_disable(OV2685_NUM_SUPPLIES, ov2685.supplies);
    }
#[no_mangle]
unsafe extern "C" fn ov2685_s_stream(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov2685_s_stream(struct v4l2_subdev *sd, int on)
    {
    struct ov2685 *ov2685 = to_ov2685(sd);
    struct i2c_client *client = ov2685.client;
    let mut ret: c_int = 0;
    mutex_lock(&ov2685.mutex);
    if (on) {
    ret = pm_runtime_resume_and_get(&ov2685.client.dev);
    if (ret < 0)
    goto unlock_and_return;
    ret = __v4l2_ctrl_handler_setup(&ov2685.ctrl_handler);
    if (ret) {
    pm_runtime_put(&client.dev);
    goto unlock_and_return;
    }
    ret = ov2685_write_reg(client, REG_SC_CTRL_MODE,
    OV2685_REG_VALUE_08BIT, SC_CTRL_MODE_STREAMING);
    if (ret) {
    pm_runtime_put(&client.dev);
    goto unlock_and_return;
    }
    } else {
    ov2685_write_reg(client, REG_SC_CTRL_MODE,
    OV2685_REG_VALUE_08BIT, SC_CTRL_MODE_STANDBY);
    pm_runtime_put(&ov2685.client.dev);
    }
    unlock_and_return:
    mutex_unlock(&ov2685.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2685_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ov2685_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    struct ov2685 *ov2685 = to_ov2685(sd);
    struct v4l2_mbus_framefmt *try_fmt;
    mutex_lock(&ov2685.mutex);
    try_fmt = v4l2_subdev_state_get_format(fh.state, 0);
// Initialize try_fmt
    ov2685_fill_fmt(&supported_modes[0], try_fmt);
    mutex_unlock(&ov2685.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov2685_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov2685_runtime_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov2685 *ov2685 = to_ov2685(sd);
    return __ov2685_power_on(ov2685);
    }
#[no_mangle]
unsafe extern "C" fn ov2685_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov2685_runtime_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov2685 *ov2685 = to_ov2685(sd);
    __ov2685_power_off(ov2685);
    return 0;
    }
    static const struct dev_pm_ops ov2685_pm_ops = {
    SET_RUNTIME_PM_OPS(ov2685_runtime_suspend,
    ov2685_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn ov2685_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov2685_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov2685 *ov2685 = container_of(ctrl.handler,
    struct ov2685, ctrl_handler);
    struct i2c_client *client = ov2685.client;
    s64 max_expo;
    int ret;
// Propagate change of current control to all related controls
    switch (ctrl.id) {
    case V4L2_CID_VBLANK:
// Update max exposure while meeting expected vblanking
    max_expo = ov2685.cur_mode.height + ctrl.val - 4;
    __v4l2_ctrl_modify_range(ov2685.exposure,
    ov2685.exposure.minimum, max_expo,
    ov2685.exposure.step,
    ov2685.exposure.default_value);
    break;
    }
    if (!pm_runtime_get_if_in_use(&client.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    ret = ov2685_write_reg(ov2685.client, OV2685_REG_EXPOSURE,
    OV2685_REG_VALUE_24BIT, ctrl.val << 4);
    break;
    case V4L2_CID_ANALOGUE_GAIN:
    ret = ov2685_write_reg(ov2685.client, OV2685_REG_GAIN,
    OV2685_REG_VALUE_16BIT, ctrl.val);
    break;
    case V4L2_CID_VBLANK:
    ret = ov2685_write_reg(ov2685.client, OV2685_REG_VTS,
    OV2685_REG_VALUE_16BIT,
    ctrl.val + ov2685.cur_mode.height);
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = ov2685_write_reg(ov2685.client, OV2685_REG_TEST_PATTERN,
    OV2685_REG_VALUE_08BIT,
    ov2685_test_pattern_val[ctrl.val]);
    break;
    default:
    dev_warn(&client.dev, "%s Unhandled id:0x%x, val:0x%x\n",
    __func__, ctrl.id, ctrl.val);
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(&client.dev);
    return ret;
    }
    static const struct v4l2_subdev_video_ops ov2685_video_ops = {
    .s_stream = ov2685_s_stream,
    };
    static const struct v4l2_subdev_pad_ops ov2685_pad_ops = {
    .enum_mbus_code = ov2685_enum_mbus_code,
    .enum_frame_size = ov2685_enum_frame_sizes,
    .get_fmt = ov2685_get_fmt,
    .set_fmt = ov2685_set_fmt,
    .get_selection = ov2685_get_selection,
    .set_selection = ov2685_get_selection,
    };
    static const struct v4l2_subdev_ops ov2685_subdev_ops = {
    .video	= &ov2685_video_ops,
    .pad	= &ov2685_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops ov2685_internal_ops = {
    .open = ov2685_open,
    };
    static const struct v4l2_ctrl_ops ov2685_ctrl_ops = {
    .s_ctrl = ov2685_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov2685_initialize_controls(ov2685: *mut ov2685) -> c_int {
    static int ov2685_initialize_controls(struct ov2685 *ov2685)
    {
    const struct ov2685_mode *mode;
    struct v4l2_ctrl_handler *handler;
    struct v4l2_ctrl *ctrl;
    struct v4l2_fwnode_device_properties props;
    u64 exposure_max;
    u32 pixel_rate, h_blank;
    int ret;
    handler = &ov2685.ctrl_handler;
    mode = ov2685.cur_mode;
    ret = v4l2_ctrl_handler_init(handler, 10);
    if (ret)
    return ret;
    handler.lock = &ov2685.mutex;
    ctrl = v4l2_ctrl_new_int_menu(handler, core::ptr::null_mut(), V4L2_CID_LINK_FREQ,
    0, 0, link_freq_menu_items);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    pixel_rate = (link_freq_menu_items[0] * 2 * OV2685_LANES) /
    OV2685_BITS_PER_SAMPLE;
    v4l2_ctrl_new_std(handler, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE,
    0, pixel_rate, 1, pixel_rate);
    h_blank = mode.hts_def - mode.width;
    ov2685.hblank = v4l2_ctrl_new_std(handler, core::ptr::null_mut(), V4L2_CID_HBLANK,
    h_blank, h_blank, 1, h_blank);
    if (ov2685.hblank)
    ov2685.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    ov2685.vblank = v4l2_ctrl_new_std(handler, &ov2685_ctrl_ops,
    V4L2_CID_VBLANK, mode.vts_def - mode.height,
    OV2685_VTS_MAX - mode.height, 1,
    mode.vts_def - mode.height);
    exposure_max = mode.vts_def - 4;
    ov2685.exposure = v4l2_ctrl_new_std(handler, &ov2685_ctrl_ops,
    V4L2_CID_EXPOSURE, OV2685_EXPOSURE_MIN,
    exposure_max, OV2685_EXPOSURE_STEP,
    mode.exp_def);
    ov2685.anal_gain = v4l2_ctrl_new_std(handler, &ov2685_ctrl_ops,
    V4L2_CID_ANALOGUE_GAIN, OV2685_GAIN_MIN,
    OV2685_GAIN_MAX, OV2685_GAIN_STEP,
    OV2685_GAIN_DEFAULT);
    ov2685.test_pattern = v4l2_ctrl_new_std_menu_items(handler,
    &ov2685_ctrl_ops, V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov2685_test_pattern_menu) - 1,
    0, 0, ov2685_test_pattern_menu);
// set properties from fwnode (e.g. rotation, orientation)
    ret = v4l2_fwnode_device_parse(&ov2685.client.dev, &props);
    if (ret)
    goto err_free_handler;
    ret = v4l2_ctrl_new_fwnode_properties(handler, &ov2685_ctrl_ops, &props);
    if (ret)
    goto err_free_handler;
    if (handler.error) {
    ret = handler.error;
    dev_err(&ov2685.client.dev,
    "Failed to init controls(%d)\n", ret);
    goto err_free_handler;
    }
    ov2685.subdev.ctrl_handler = handler;
    return 0;
    err_free_handler:
    v4l2_ctrl_handler_free(handler);
    return ret;
    }
    static int ov2685_check_sensor_id(struct ov2685 *ov2685,
    struct i2c_client *client)
    {
    struct device *dev = &ov2685.client.dev;
    int ret;
    let mut id: u32 = 0;
    ret = ov2685_read_reg(client, OV2685_REG_CHIP_ID,
    OV2685_REG_VALUE_16BIT, &id);
    if (id != CHIP_ID) {
    dev_err(dev, "Unexpected sensor id(%04x), ret(%d)\n", id, ret);
    return ret;
    }
    dev_info(dev, "Detected OV%04x sensor\n", CHIP_ID);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov2685_configure_regulators(ov2685: *mut ov2685) -> c_int {
    static int ov2685_configure_regulators(struct ov2685 *ov2685)
    {
    int i;
    for (i = 0; i < OV2685_NUM_SUPPLIES; i++)
    ov2685.supplies[i].supply = ov2685_supply_names[i];
    return devm_regulator_bulk_get(&ov2685.client.dev,
    OV2685_NUM_SUPPLIES,
    ov2685.supplies);
    }
#[no_mangle]
unsafe extern "C" fn ov2685_probe(client: *mut i2c_client) -> c_int {
    static int ov2685_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct ov2685 *ov2685;
    int ret;
    ov2685 = devm_kzalloc(dev, sizeof(*ov2685), GFP_KERNEL);
    if (!ov2685)
    return -ENOMEM;
    ov2685.client = client;
    ov2685.cur_mode = &supported_modes[0];
    ov2685.xvclk = devm_v4l2_sensor_clk_get_legacy(dev, "xvclk", true,
    OV2685_XVCLK_FREQ);
    if (IS_ERR(ov2685.xvclk))
    return dev_err_probe(dev, PTR_ERR(ov2685.xvclk),
    "Failed to get xvclk\n");
    if (clk_get_rate(ov2685.xvclk) != OV2685_XVCLK_FREQ)
    dev_warn(dev, "xvclk mismatched, modes are based on 24MHz\n");
    ov2685.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ov2685.reset_gpio)) {
    dev_err(dev, "Failed to get reset-gpios\n");
    return -EINVAL;
    }
    ret = ov2685_configure_regulators(ov2685);
    if (ret) {
    dev_err(dev, "Failed to get power regulators\n");
    return ret;
    }
    mutex_init(&ov2685.mutex);
    v4l2_i2c_subdev_init(&ov2685.subdev, client, &ov2685_subdev_ops);
    ret = ov2685_initialize_controls(ov2685);
    if (ret)
    goto err_destroy_mutex;
    ret = __ov2685_power_on(ov2685);
    if (ret)
    goto err_free_handler;
    ret = ov2685_check_sensor_id(ov2685, client);
    if (ret)
    goto err_power_off;
    ov2685.subdev.internal_ops = &ov2685_internal_ops;
    ov2685.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ov2685.pad.flags = MEDIA_PAD_FL_SOURCE;
    ov2685.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&ov2685.subdev.entity, 1, &ov2685.pad);
    if (ret < 0)
    goto err_power_off;
    ret = v4l2_async_register_subdev(&ov2685.subdev);
    if (ret) {
    dev_err(dev, "v4l2 async register subdev failed\n");
    goto err_clean_entity;
    }
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    pm_runtime_idle(dev);
    return 0;
    err_clean_entity:
    media_entity_cleanup(&ov2685.subdev.entity);
    err_power_off:
    __ov2685_power_off(ov2685);
    err_free_handler:
    v4l2_ctrl_handler_free(&ov2685.ctrl_handler);
    err_destroy_mutex:
    mutex_destroy(&ov2685.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2685_remove(client: *mut i2c_client) {
    static void ov2685_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov2685 *ov2685 = to_ov2685(sd);
    v4l2_async_unregister_subdev(sd);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(&ov2685.ctrl_handler);
    mutex_destroy(&ov2685.mutex);
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev))
    __ov2685_power_off(ov2685);
    pm_runtime_set_suspended(&client.dev);
    }

    static const struct of_device_id ov2685_of_match[] = {
    { .compatible = "ovti,ov2685" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ov2685_of_match);

    static struct i2c_driver ov2685_i2c_driver = {
    .driver = {
    .name = "ov2685",
    .pm = &ov2685_pm_ops,
    .of_match_table = of_match_ptr(ov2685_of_match),
    },
    .probe		= ov2685_probe,
    .remove		= ov2685_remove,
    };
    module_i2c_driver(ov2685_i2c_driver);
    MODULE_DESCRIPTION("OmniVision ov2685 sensor driver");
    MODULE_LICENSE("GPL v2");
