//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov5693.c
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
// Copyright (c) 2013 Intel Corporation. All Rights Reserved.
//
// Adapted from the atomisp-ov5693 driver, with contributions from:
//
// Daniel Scally
// Jean-Michel Hautbois
// Fabian Wuthrich
// Tsuchiya Yuto
// Jordan Hand
// Jake Day
//

// System Control

pub const OV5693_START_STREAMING: c_uint = 0x01;
pub const OV5693_STOP_STREAMING: c_uint = 0x00;
pub const OV5693_SW_RESET: c_uint = 0x01;

// Yes, this is right. The datasheet for the OV5693 gives its ID as 0x5690
pub const OV5693_CHIP_ID: c_uint = 0x5690;
// Exposure

pub const OV5693_INTEGRATION_TIME_MARGIN: c_int = 8;
pub const OV5693_EXPOSURE_MIN: c_int = 1;
pub const OV5693_EXPOSURE_STEP: c_int = 1;
// Analogue Gain

pub const OV5693_GAIN_MIN: c_int = 1;
pub const OV5693_GAIN_MAX: c_int = 127;
pub const OV5693_GAIN_DEF: c_int = 8;
pub const OV5693_GAIN_STEP: c_int = 1;
// Digital Gain

pub const OV5693_MWB_GAIN_MAX: c_uint = 0x0fff;
pub const OV5693_DIGITAL_GAIN_MIN: c_int = 1;
pub const OV5693_DIGITAL_GAIN_MAX: c_int = 4095;
pub const OV5693_DIGITAL_GAIN_DEF: c_int = 1024;
pub const OV5693_DIGITAL_GAIN_STEP: c_int = 1;
// Timing and Format

pub const OV5693_TIMING_MAX_VTS: c_uint = 0xffff;
pub const OV5693_TIMING_MIN_VTS: c_uint = 0x04;

// Pixel Array
pub const OV5693_NATIVE_WIDTH: c_int = 2624;
pub const OV5693_NATIVE_HEIGHT: c_int = 1956;
pub const OV5693_NATIVE_START_LEFT: c_int = 0;
pub const OV5693_NATIVE_START_TOP: c_int = 0;
pub const OV5693_ACTIVE_WIDTH: c_int = 2592;
pub const OV5693_ACTIVE_HEIGHT: c_int = 1944;
pub const OV5693_ACTIVE_START_LEFT: c_int = 16;
pub const OV5693_ACTIVE_START_TOP: c_int = 6;
pub const OV5693_MIN_CROP_WIDTH: c_int = 2;
pub const OV5693_MIN_CROP_HEIGHT: c_int = 2;
// Test Pattern

pub const OV5693_TEST_PATTERN_RANDOM: c_uint = 0x01;
pub const OV5693_TEST_PATTERN_BARS: c_uint = 0x00;
// System Frequencies
pub const OV5693_XVCLK_FREQ: c_int = 19200000;
pub const OV5693_LINK_FREQ_419_2MHZ: c_int = 419200000;
pub const OV5693_PIXEL_RATE: c_int = 167680000;

    static const char * const ov5693_supply_names[] = {
    "avdd",		/* Analog power */
    "dovdd",	/* Digital I/O power */
    "dvdd",		/* Digital circuit power */
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5693_device {
    pub dev: *mut device,
    pub regmap: *mut regmap,
// Protect against concurrent changes to controls
    pub lock: mutex,
    pub reset: *mut gpio_desc,
    pub powerdown: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; OV5693_NUM_SUPPLIES],
    pub xvclk: *mut clk,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5693_mode {
    pub crop: v4l2_rect,
    pub format: v4l2_mbus_framefmt,
    pub binning_x: bool,
    pub binning_y: bool,
    pub inc_x_odd: c_uint,
    pub inc_y_odd: c_uint,
    pub vts: c_uint,
    pub mode: },
    pub sd: v4l2_subdev,
    pub pad: media_pad,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5693_v4l2_ctrls {
    pub handler: v4l2_ctrl_handler,
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub analogue_gain: *mut v4l2_ctrl,
    pub digital_gain: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub test_pattern: *mut v4l2_ctrl,
    pub ctrls: },
}

    static const struct cci_reg_sequence ov5693_global_regs[] = {
    {CCI_REG8(0x3016), 0xf0},
    {CCI_REG8(0x3017), 0xf0},
    {CCI_REG8(0x3018), 0xf0},
    {CCI_REG8(0x3022), 0x01},
    {CCI_REG8(0x3028), 0x44},
    {CCI_REG8(0x3098), 0x02},
    {CCI_REG8(0x3099), 0x19},
    {CCI_REG8(0x309a), 0x02},
    {CCI_REG8(0x309b), 0x01},
    {CCI_REG8(0x309c), 0x00},
    {CCI_REG8(0x30a0), 0xd2},
    {CCI_REG8(0x30a2), 0x01},
    {CCI_REG8(0x30b2), 0x00},
    {CCI_REG8(0x30b3), 0x83},
    {CCI_REG8(0x30b4), 0x03},
    {CCI_REG8(0x30b5), 0x04},
    {CCI_REG8(0x30b6), 0x01},
    {CCI_REG8(0x3080), 0x01},
    {CCI_REG8(0x3104), 0x21},
    {CCI_REG8(0x3106), 0x00},
    {CCI_REG8(0x3406), 0x01},
    {CCI_REG8(0x3503), 0x07},
    {CCI_REG8(0x350b), 0x40},
    {CCI_REG8(0x3601), 0x0a},
    {CCI_REG8(0x3602), 0x38},
    {CCI_REG8(0x3612), 0x80},
    {CCI_REG8(0x3620), 0x54},
    {CCI_REG8(0x3621), 0xc7},
    {CCI_REG8(0x3622), 0x0f},
    {CCI_REG8(0x3625), 0x10},
    {CCI_REG8(0x3630), 0x55},
    {CCI_REG8(0x3631), 0xf4},
    {CCI_REG8(0x3632), 0x00},
    {CCI_REG8(0x3633), 0x34},
    {CCI_REG8(0x3634), 0x02},
    {CCI_REG8(0x364d), 0x0d},
    {CCI_REG8(0x364f), 0xdd},
    {CCI_REG8(0x3660), 0x04},
    {CCI_REG8(0x3662), 0x10},
    {CCI_REG8(0x3663), 0xf1},
    {CCI_REG8(0x3665), 0x00},
    {CCI_REG8(0x3666), 0x20},
    {CCI_REG8(0x3667), 0x00},
    {CCI_REG8(0x366a), 0x80},
    {CCI_REG8(0x3680), 0xe0},
    {CCI_REG8(0x3681), 0x00},
    {CCI_REG8(0x3700), 0x42},
    {CCI_REG8(0x3701), 0x14},
    {CCI_REG8(0x3702), 0xa0},
    {CCI_REG8(0x3703), 0xd8},
    {CCI_REG8(0x3704), 0x78},
    {CCI_REG8(0x3705), 0x02},
    {CCI_REG8(0x370a), 0x00},
    {CCI_REG8(0x370b), 0x20},
    {CCI_REG8(0x370c), 0x0c},
    {CCI_REG8(0x370d), 0x11},
    {CCI_REG8(0x370e), 0x00},
    {CCI_REG8(0x370f), 0x40},
    {CCI_REG8(0x3710), 0x00},
    {CCI_REG8(0x371a), 0x1c},
    {CCI_REG8(0x371b), 0x05},
    {CCI_REG8(0x371c), 0x01},
    {CCI_REG8(0x371e), 0xa1},
    {CCI_REG8(0x371f), 0x0c},
    {CCI_REG8(0x3721), 0x00},
    {CCI_REG8(0x3724), 0x10},
    {CCI_REG8(0x3726), 0x00},
    {CCI_REG8(0x372a), 0x01},
    {CCI_REG8(0x3730), 0x10},
    {CCI_REG8(0x3738), 0x22},
    {CCI_REG8(0x3739), 0xe5},
    {CCI_REG8(0x373a), 0x50},
    {CCI_REG8(0x373b), 0x02},
    {CCI_REG8(0x373c), 0x41},
    {CCI_REG8(0x373f), 0x02},
    {CCI_REG8(0x3740), 0x42},
    {CCI_REG8(0x3741), 0x02},
    {CCI_REG8(0x3742), 0x18},
    {CCI_REG8(0x3743), 0x01},
    {CCI_REG8(0x3744), 0x02},
    {CCI_REG8(0x3747), 0x10},
    {CCI_REG8(0x374c), 0x04},
    {CCI_REG8(0x3751), 0xf0},
    {CCI_REG8(0x3752), 0x00},
    {CCI_REG8(0x3753), 0x00},
    {CCI_REG8(0x3754), 0xc0},
    {CCI_REG8(0x3755), 0x00},
    {CCI_REG8(0x3756), 0x1a},
    {CCI_REG8(0x3758), 0x00},
    {CCI_REG8(0x3759), 0x0f},
    {CCI_REG8(0x376b), 0x44},
    {CCI_REG8(0x375c), 0x04},
    {CCI_REG8(0x3774), 0x10},
    {CCI_REG8(0x3776), 0x00},
    {CCI_REG8(0x377f), 0x08},
    {CCI_REG8(0x3780), 0x22},
    {CCI_REG8(0x3781), 0x0c},
    {CCI_REG8(0x3784), 0x2c},
    {CCI_REG8(0x3785), 0x1e},
    {CCI_REG8(0x378f), 0xf5},
    {CCI_REG8(0x3791), 0xb0},
    {CCI_REG8(0x3795), 0x00},
    {CCI_REG8(0x3796), 0x64},
    {CCI_REG8(0x3797), 0x11},
    {CCI_REG8(0x3798), 0x30},
    {CCI_REG8(0x3799), 0x41},
    {CCI_REG8(0x379a), 0x07},
    {CCI_REG8(0x379b), 0xb0},
    {CCI_REG8(0x379c), 0x0c},
    {CCI_REG8(0x3a04), 0x06},
    {CCI_REG8(0x3a05), 0x14},
    {CCI_REG8(0x3e07), 0x20},
    {CCI_REG8(0x4000), 0x08},
    {CCI_REG8(0x4001), 0x04},
    {CCI_REG8(0x4004), 0x08},
    {CCI_REG8(0x4006), 0x20},
    {CCI_REG8(0x4008), 0x24},
    {CCI_REG8(0x4009), 0x10},
    {CCI_REG8(0x4058), 0x00},
    {CCI_REG8(0x4101), 0xb2},
    {CCI_REG8(0x4307), 0x31},
    {CCI_REG8(0x4511), 0x05},
    {CCI_REG8(0x4512), 0x01},
    {CCI_REG8(0x481f), 0x30},
    {CCI_REG8(0x4826), 0x2c},
    {CCI_REG8(0x4d02), 0xfd},
    {CCI_REG8(0x4d03), 0xf5},
    {CCI_REG8(0x4d04), 0x0c},
    {CCI_REG8(0x4d05), 0xcc},
    {CCI_REG8(0x4837), 0x0a},
    {CCI_REG8(0x5003), 0x20},
    {CCI_REG8(0x5013), 0x00},
    {CCI_REG8(0x5842), 0x01},
    {CCI_REG8(0x5843), 0x2b},
    {CCI_REG8(0x5844), 0x01},
    {CCI_REG8(0x5845), 0x92},
    {CCI_REG8(0x5846), 0x01},
    {CCI_REG8(0x5847), 0x8f},
    {CCI_REG8(0x5848), 0x01},
    {CCI_REG8(0x5849), 0x0c},
    {CCI_REG8(0x5e10), 0x0c},
    {CCI_REG8(0x3820), 0x00},
    {CCI_REG8(0x3821), 0x1e},
    {CCI_REG8(0x5041), 0x14}
    };
    static const struct v4l2_rect ov5693_default_crop = {
    .left = OV5693_ACTIVE_START_LEFT,
    .top = OV5693_ACTIVE_START_TOP,
    .width = OV5693_ACTIVE_WIDTH,
    .height = OV5693_ACTIVE_HEIGHT,
    };
    static const struct v4l2_mbus_framefmt ov5693_default_fmt = {
    .width = OV5693_ACTIVE_WIDTH,
    .height = OV5693_ACTIVE_HEIGHT,
    .code = MEDIA_BUS_FMT_SBGGR10_1X10,
    };
    static const s64 link_freq_menu_items[] = {
    OV5693_LINK_FREQ_419_2MHZ
    };
    static const char * const ov5693_test_pattern_menu[] = {
    "Disabled",
    "Random Data",
    "Colour Bars",
    "Colour Bars with Rolling Bar"
    };
    static const u8 ov5693_test_pattern_bits[] = {
    0,
    OV5693_TEST_PATTERN_ENABLE | OV5693_TEST_PATTERN_RANDOM,
    OV5693_TEST_PATTERN_ENABLE | OV5693_TEST_PATTERN_BARS,
    OV5693_TEST_PATTERN_ENABLE | OV5693_TEST_PATTERN_BARS |
    OV5693_TEST_PATTERN_ROLLING,
    };
// V4L2 Controls Functions
    static int ov5693_flip_vert_configure(struct ov5693_device *ov5693,
    bool enable)
    {
    u8 bits = OV5693_FORMAT1_FLIP_VERT_ISP_EN |
    OV5693_FORMAT1_FLIP_VERT_SENSOR_EN;
    int ret;
    ret = cci_update_bits(ov5693.regmap, OV5693_FORMAT1_REG, bits,
    enable ? bits : 0, core::ptr::null_mut());
    if (ret)
    return ret;
    return 0;
    }
    static int ov5693_flip_horz_configure(struct ov5693_device *ov5693,
    bool enable)
    {
    u8 bits = OV5693_FORMAT2_FLIP_HORZ_ISP_EN |
    OV5693_FORMAT2_FLIP_HORZ_SENSOR_EN;
    int ret;
    ret = cci_update_bits(ov5693.regmap, OV5693_FORMAT2_REG, bits,
    enable ? bits : 0, core::ptr::null_mut());
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_get_exposure(ov5693: *mut ov5693_device, value: *mut i32) -> c_int {
    static int ov5693_get_exposure(struct ov5693_device *ov5693, s32 *value)
    {
    u64 exposure;
    int ret;
    ret = cci_read(ov5693.regmap, OV5693_EXPOSURE_CTRL_REG, &exposure,
    core::ptr::null_mut());
    if (ret)
    return ret;
// The lowest 4 bits are unsupported fractional bits
// value = exposure >> 4;
    return 0;
    }
    static int ov5693_exposure_configure(struct ov5693_device *ov5693,
    u32 exposure)
    {
    let mut ret: c_int = 0;
    exposure = (exposure << 4) & OV5693_EXPOSURE_CTRL_MASK;
    cci_write(ov5693.regmap, OV5693_EXPOSURE_CTRL_REG, exposure, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_get_gain(ov5693: *mut ov5693_device, gain: *mut u32) -> c_int {
    static int ov5693_get_gain(struct ov5693_device *ov5693, u32 *gain)
    {
    u64 value;
    int ret;
    ret = cci_read(ov5693.regmap, OV5693_GAIN_CTRL_REG, &value, core::ptr::null_mut());
    if (ret)
    return ret;
// As with exposure, the lowest 4 bits are fractional bits.
// gain = value >> 4;
    return ret;
    }
    static int ov5693_digital_gain_configure(struct ov5693_device *ov5693,
    u32 gain)
    {
    let mut ret: c_int = 0;
    gain &= OV5693_MWB_GAIN_MASK;
    cci_write(ov5693.regmap, OV5693_MWB_RED_GAIN_REG, gain, &ret);
    cci_write(ov5693.regmap, OV5693_MWB_GREEN_GAIN_REG, gain, &ret);
    cci_write(ov5693.regmap, OV5693_MWB_BLUE_GAIN_REG, gain, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_analog_gain_configure(ov5693: *mut ov5693_device, gain: u32) -> c_int {
    static int ov5693_analog_gain_configure(struct ov5693_device *ov5693, u32 gain)
    {
    let mut ret: c_int = 0;
    gain = (gain << 4) & OV5693_GAIN_CTRL_MASK;
    cci_write(ov5693.regmap, OV5693_GAIN_CTRL_REG, gain, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_vts_configure(ov5693: *mut ov5693_device, vblank: u32) -> c_int {
    static int ov5693_vts_configure(struct ov5693_device *ov5693, u32 vblank)
    {
    let mut vts: u16 = ov5693.mode.format.height + vblank;
    let mut ret: c_int = 0;
    cci_write(ov5693.regmap, OV5693_TIMING_VTS_REG, vts, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_test_pattern_configure(ov5693: *mut ov5693_device, idx: u32) -> c_int {
    static int ov5693_test_pattern_configure(struct ov5693_device *ov5693, u32 idx)
    {
    let mut ret: c_int = 0;
    cci_write(ov5693.regmap, OV5693_TEST_PATTERN_REG,
    ov5693_test_pattern_bits[idx], &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov5693_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov5693_device *ov5693 =
    container_of(ctrl.handler, struct ov5693_device, ctrls.handler);
    let mut ret: c_int = 0;
// If VBLANK is altered we need to update exposure to compensate
    if (ctrl.id == V4L2_CID_VBLANK) {
    int exposure_max;
    exposure_max = ov5693.mode.format.height + ctrl.val -
    OV5693_INTEGRATION_TIME_MARGIN;
    __v4l2_ctrl_modify_range(ov5693.ctrls.exposure,
    ov5693.ctrls.exposure.minimum,
    exposure_max,
    ov5693.ctrls.exposure.step,
    min(ov5693.ctrls.exposure.val,
    exposure_max));
    }
// Only apply changes to the controls if the device is powered up
    if (!pm_runtime_get_if_in_use(ov5693.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    ret = ov5693_exposure_configure(ov5693, ctrl.val);
    break;
    case V4L2_CID_ANALOGUE_GAIN:
    ret = ov5693_analog_gain_configure(ov5693, ctrl.val);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    ret = ov5693_digital_gain_configure(ov5693, ctrl.val);
    break;
    case V4L2_CID_HFLIP:
    ret = ov5693_flip_horz_configure(ov5693, !!ctrl.val);
    break;
    case V4L2_CID_VFLIP:
    ret = ov5693_flip_vert_configure(ov5693, !!ctrl.val);
    break;
    case V4L2_CID_VBLANK:
    ret = ov5693_vts_configure(ov5693, ctrl.val);
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = ov5693_test_pattern_configure(ov5693, ctrl.val);
    break;
    default:
    ret = -EINVAL;
    }
    pm_runtime_put(ov5693.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov5693_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov5693_device *ov5693 = container_of(ctrl.handler,
    struct ov5693_device,
    ctrls.handler);
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE_ABSOLUTE:
    return ov5693_get_exposure(ov5693, &ctrl.val);
    case V4L2_CID_AUTOGAIN:
    return ov5693_get_gain(ov5693, &ctrl.val);
    default:
    return -EINVAL;
    }
    }
    static const struct v4l2_ctrl_ops ov5693_ctrl_ops = {
    .s_ctrl = ov5693_s_ctrl,
    .g_volatile_ctrl = ov5693_g_volatile_ctrl
    };
// System Control Functions
#[no_mangle]
unsafe extern "C" fn ov5693_mode_configure(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_mode_configure(struct ov5693_device *ov5693)
    {
    const struct ov5693_mode *mode = &ov5693.mode;
    let mut ret: c_int = 0;
// Crop Start X
    cci_write(ov5693.regmap, OV5693_CROP_START_X_REG, mode.crop.left,
    &ret);
// Offset X
    cci_write(ov5693.regmap, OV5693_OFFSET_START_X_REG, 0, &ret);
// Output Size X
    cci_write(ov5693.regmap, OV5693_OUTPUT_SIZE_X_REG, mode.format.width,
    &ret);
// Crop End X
    cci_write(ov5693.regmap, OV5693_CROP_END_X_REG,
    mode.crop.left + mode.crop.width, &ret);
// Horizontal Total Size
    cci_write(ov5693.regmap, OV5693_TIMING_HTS_REG, OV5693_FIXED_PPL,
    &ret);
// Crop Start Y
    cci_write(ov5693.regmap, OV5693_CROP_START_Y_REG, mode.crop.top,
    &ret);
// Offset Y
    cci_write(ov5693.regmap, OV5693_OFFSET_START_Y_REG, 0, &ret);
// Output Size Y
    cci_write(ov5693.regmap, OV5693_OUTPUT_SIZE_Y_REG, mode.format.height,
    &ret);
// Crop End Y
    cci_write(ov5693.regmap, OV5693_CROP_END_Y_REG,
    mode.crop.top + mode.crop.height, &ret);
// Subsample X increase
    cci_write(ov5693.regmap, OV5693_SUB_INC_X_REG,
    ((mode.inc_x_odd << 4) & 0xf0) | 0x01, &ret);
// Subsample Y increase
    cci_write(ov5693.regmap, OV5693_SUB_INC_Y_REG,
    ((mode.inc_y_odd << 4) & 0xf0) | 0x01, &ret);
// Binning
    cci_update_bits(ov5693.regmap, OV5693_FORMAT1_REG,
    OV5693_FORMAT1_VBIN_EN,
    mode.binning_y ? OV5693_FORMAT1_VBIN_EN : 0, &ret);
    cci_update_bits(ov5693.regmap, OV5693_FORMAT2_REG,
    OV5693_FORMAT2_HBIN_EN,
    mode.binning_x ? OV5693_FORMAT2_HBIN_EN : 0, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_enable_streaming(ov5693: *mut ov5693_device, enable: bool) -> c_int {
    static int ov5693_enable_streaming(struct ov5693_device *ov5693, bool enable)
    {
    let mut ret: c_int = 0;
    cci_write(ov5693.regmap, OV5693_SW_STREAM_REG,
    enable ? OV5693_START_STREAMING : OV5693_STOP_STREAMING,
    &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sw_reset(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_sw_reset(struct ov5693_device *ov5693)
    {
    let mut ret: c_int = 0;
    cci_write(ov5693.regmap, OV5693_SW_RESET_REG, OV5693_SW_RESET, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sensor_init(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_sensor_init(struct ov5693_device *ov5693)
    {
    int ret;
    ret = ov5693_sw_reset(ov5693);
    if (ret)
    return dev_err_probe(ov5693.dev, ret,
    "software reset error\n");
    ret = cci_multi_reg_write(ov5693.regmap, ov5693_global_regs,
    ARRAY_SIZE(ov5693_global_regs), core::ptr::null_mut());
    if (ret)
    return dev_err_probe(ov5693.dev, ret,
    "global settings error\n");
    ret = ov5693_mode_configure(ov5693);
    if (ret)
    return dev_err_probe(ov5693.dev, ret,
    "mode configure error\n");
    ret = ov5693_enable_streaming(ov5693, false);
    if (ret)
    dev_err(ov5693.dev, "stop streaming error\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sensor_powerdown(ov5693: *mut ov5693_device) {
    static void ov5693_sensor_powerdown(struct ov5693_device *ov5693)
    {
    gpiod_set_value_cansleep(ov5693.reset, 1);
    gpiod_set_value_cansleep(ov5693.powerdown, 1);
    regulator_bulk_disable(OV5693_NUM_SUPPLIES, ov5693.supplies);
    clk_disable_unprepare(ov5693.xvclk);
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sensor_powerup(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_sensor_powerup(struct ov5693_device *ov5693)
    {
    int ret;
    gpiod_set_value_cansleep(ov5693.reset, 1);
    gpiod_set_value_cansleep(ov5693.powerdown, 1);
    ret = clk_prepare_enable(ov5693.xvclk);
    if (ret) {
    dev_err(ov5693.dev, "Failed to enable clk\n");
    goto fail_power;
    }
    ret = regulator_bulk_enable(OV5693_NUM_SUPPLIES, ov5693.supplies);
    if (ret) {
    dev_err(ov5693.dev, "Failed to enable regulators\n");
    goto fail_power;
    }
    gpiod_set_value_cansleep(ov5693.powerdown, 0);
    gpiod_set_value_cansleep(ov5693.reset, 0);
    usleep_range(5000, 7500);
    return 0;
    fail_power:
    ov5693_sensor_powerdown(ov5693);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sensor_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov5693_sensor_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    ov5693_sensor_powerdown(ov5693);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_sensor_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ov5693_sensor_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    int ret;
    mutex_lock(&ov5693.lock);
    ret = ov5693_sensor_powerup(ov5693);
    if (ret)
    goto out_unlock;
    ret = ov5693_sensor_init(ov5693);
    if (ret) {
    dev_err(dev, "ov5693 sensor init failure\n");
    goto err_power;
    }
    goto out_unlock;
    err_power:
    ov5693_sensor_powerdown(ov5693);
    out_unlock:
    mutex_unlock(&ov5693.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_detect(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_detect(struct ov5693_device *ov5693)
    {
    int ret;
    u64 id;
    ret = cci_read(ov5693.regmap, OV5693_REG_CHIP_ID, &id, core::ptr::null_mut());
    if (ret)
    return ret;
    if (id != OV5693_CHIP_ID)
    return dev_err_probe(ov5693.dev, -ENODEV,
    "sensor ID mismatch. Got 0x%04llx\n", id);
    return 0;
    }
// V4L2 Framework callbacks
#[no_mangle]
unsafe extern "C" fn __ov5693_calc_vts(height: u32) -> c_uint {
    static unsigned int __ov5693_calc_vts(u32 height)
    {
//
// We need to set a sensible default VTS for whatever format height we
// happen to be given from set_fmt(). This function just targets
// an even multiple of 30fps.
//
    unsigned int tgt_fps;
    tgt_fps = rounddown(OV5693_PIXEL_RATE / OV5693_FIXED_PPL / height, 30);
    return ALIGN_DOWN(OV5693_PIXEL_RATE / OV5693_FIXED_PPL / tgt_fps, 2);
    }
    static struct v4l2_mbus_framefmt *
    __ov5693_get_pad_format(struct ov5693_device *ov5693,
    struct v4l2_subdev_state *state,
    unsigned int pad, enum v4l2_subdev_format_whence which)
    {
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
    return v4l2_subdev_state_get_format(state, pad);
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    return &ov5693.mode.format;
    default:
    return core::ptr::null_mut();
    }
    }
    static struct v4l2_rect *
    __ov5693_get_pad_crop(struct ov5693_device *ov5693,
    struct v4l2_subdev_state *state,
    unsigned int pad, enum v4l2_subdev_format_whence which)
    {
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
    return v4l2_subdev_state_get_crop(state, pad);
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    return &ov5693.mode.crop;
    }
    return core::ptr::null_mut();
    }
    static int ov5693_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *format)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    format.format = ov5693.mode.format;
    return 0;
    }
    static int ov5693_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *format)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    const struct v4l2_rect *crop;
    struct v4l2_mbus_framefmt *fmt;
    unsigned int hratio, vratio;
    unsigned int width, height;
    unsigned int hblank;
    int exposure_max;
    crop = __ov5693_get_pad_crop(ov5693, state, format.pad, format.which);
//
// Align to two to simplify the binning calculations below, and clamp
// the requested format at the crop rectangle
//
    width = clamp_t(unsigned int, ALIGN(format.format.width, 2),
    OV5693_MIN_CROP_WIDTH, crop.width);
    height = clamp_t(unsigned int, ALIGN(format.format.height, 2),
    OV5693_MIN_CROP_HEIGHT, crop.height);
//
// We can only support setting either the dimensions of the crop rect
// or those dimensions binned (separately) by a factor of two.
//
    hratio = clamp_t(unsigned int,
    DIV_ROUND_CLOSEST(crop.width, width), 1, 2);
    vratio = clamp_t(unsigned int,
    DIV_ROUND_CLOSEST(crop.height, height), 1, 2);
    fmt = __ov5693_get_pad_format(ov5693, state, format.pad,
    format.which);
    fmt.width = crop.width / hratio;
    fmt.height = crop.height / vratio;
    fmt.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    format.format = *fmt;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY)
    return 0;
    mutex_lock(&ov5693.lock);
    ov5693.mode.binning_x = hratio > 1;
    ov5693.mode.inc_x_odd = hratio > 1 ? 3 : 1;
    ov5693.mode.binning_y = vratio > 1;
    ov5693.mode.inc_y_odd = vratio > 1 ? 3 : 1;
    ov5693.mode.vts = __ov5693_calc_vts(fmt.height);
    __v4l2_ctrl_modify_range(ov5693.ctrls.vblank,
    OV5693_TIMING_MIN_VTS,
    OV5693_TIMING_MAX_VTS - fmt.height,
    1, ov5693.mode.vts - fmt.height);
    __v4l2_ctrl_s_ctrl(ov5693.ctrls.vblank,
    ov5693.mode.vts - fmt.height);
    hblank = OV5693_FIXED_PPL - fmt.width;
    __v4l2_ctrl_modify_range(ov5693.ctrls.hblank, hblank, hblank, 1,
    hblank);
    exposure_max = ov5693.mode.vts - OV5693_INTEGRATION_TIME_MARGIN;
    __v4l2_ctrl_modify_range(ov5693.ctrls.exposure,
    ov5693.ctrls.exposure.minimum, exposure_max,
    ov5693.ctrls.exposure.step,
    min(ov5693.ctrls.exposure.val,
    exposure_max));
    mutex_unlock(&ov5693.lock);
    return 0;
    }
    static int ov5693_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    mutex_lock(&ov5693.lock);
    sel.r = *__ov5693_get_pad_crop(ov5693, state, sel.pad,
    sel.which);
    mutex_unlock(&ov5693.lock);
    break;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = OV5693_NATIVE_WIDTH;
    sel.r.height = OV5693_NATIVE_HEIGHT;
    break;
    case V4L2_SEL_TGT_CROP_BOUNDS:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    sel.r.top = OV5693_ACTIVE_START_TOP;
    sel.r.left = OV5693_ACTIVE_START_LEFT;
    sel.r.width = OV5693_ACTIVE_WIDTH;
    sel.r.height = OV5693_ACTIVE_HEIGHT;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ov5693_set_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *__crop;
    struct v4l2_rect rect;
    if (sel.target != V4L2_SEL_TGT_CROP)
    return -EINVAL;
//
// Clamp the boundaries of the crop rectangle to the size of the sensor
// pixel array. Align to multiples of 2 to ensure Bayer pattern isn't
// disrupted.
//
    rect.left = clamp(ALIGN(sel.r.left, 2), OV5693_NATIVE_START_LEFT,
    OV5693_NATIVE_WIDTH);
    rect.top = clamp(ALIGN(sel.r.top, 2), OV5693_NATIVE_START_TOP,
    OV5693_NATIVE_HEIGHT);
    rect.width = clamp_t(unsigned int, ALIGN(sel.r.width, 2),
    OV5693_MIN_CROP_WIDTH, OV5693_NATIVE_WIDTH);
    rect.height = clamp_t(unsigned int, ALIGN(sel.r.height, 2),
    OV5693_MIN_CROP_HEIGHT, OV5693_NATIVE_HEIGHT);
// Make sure the crop rectangle isn't outside the bounds of the array
    rect.width = min_t(unsigned int, rect.width,
    OV5693_NATIVE_WIDTH - rect.left);
    rect.height = min_t(unsigned int, rect.height,
    OV5693_NATIVE_HEIGHT - rect.top);
    __crop = __ov5693_get_pad_crop(ov5693, state, sel.pad, sel.which);
    if (rect.width != __crop.width || rect.height != __crop.height) {
//
// Reset the output image size if the crop rectangle size has
// been modified.
//
    format = __ov5693_get_pad_format(ov5693, state, sel.pad,
    sel.which);
    format.width = rect.width;
    format.height = rect.height;
    }
// __crop = rect;
    sel.r = rect;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov5693_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    int ret;
    if (enable) {
    ret = pm_runtime_resume_and_get(ov5693.dev);
    if (ret)
    return ret;
    mutex_lock(&ov5693.lock);
    ret = __v4l2_ctrl_handler_setup(&ov5693.ctrls.handler);
    if (ret) {
    mutex_unlock(&ov5693.lock);
    goto err_power_down;
    }
    ret = ov5693_enable_streaming(ov5693, true);
    mutex_unlock(&ov5693.lock);
    } else {
    mutex_lock(&ov5693.lock);
    ret = ov5693_enable_streaming(ov5693, false);
    mutex_unlock(&ov5693.lock);
    }
    if (ret)
    goto err_power_down;
    if (!enable)
    pm_runtime_put(ov5693.dev);
    return 0;
    err_power_down:
    pm_runtime_put_noidle(ov5693.dev);
    return ret;
    }
    static int ov5693_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *interval)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    unsigned int framesize = OV5693_FIXED_PPL * (ov5693.mode.format.height +
    ov5693.ctrls.vblank.val);
    let mut fps: c_uint = DIV_ROUND_CLOSEST(OV5693_PIXEL_RATE, framesize);
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (interval.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    interval.interval.numerator = 1;
    interval.interval.denominator = fps;
    return 0;
    }
    static int ov5693_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
// Only a single mbus format is supported
    if (code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    return 0;
    }
    static int ov5693_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    struct v4l2_rect *__crop;
    if (fse.index > 1 || fse.code != MEDIA_BUS_FMT_SBGGR10_1X10)
    return -EINVAL;
    __crop = __ov5693_get_pad_crop(ov5693, state, fse.pad, fse.which);
    if (!__crop)
    return -EINVAL;
    fse.min_width = __crop.width / (fse.index + 1);
    fse.min_height = __crop.height / (fse.index + 1);
    fse.max_width = fse.min_width;
    fse.max_height = fse.min_height;
    return 0;
    }
    static const struct v4l2_subdev_video_ops ov5693_video_ops = {
    .s_stream = ov5693_s_stream,
    };
    static const struct v4l2_subdev_pad_ops ov5693_pad_ops = {
    .enum_mbus_code = ov5693_enum_mbus_code,
    .enum_frame_size = ov5693_enum_frame_size,
    .get_fmt = ov5693_get_fmt,
    .set_fmt = ov5693_set_fmt,
    .get_selection = ov5693_get_selection,
    .set_selection = ov5693_set_selection,
    .get_frame_interval = ov5693_get_frame_interval,
    };
    static const struct v4l2_subdev_ops ov5693_ops = {
    .video = &ov5693_video_ops,
    .pad = &ov5693_pad_ops,
    };
// Sensor and Driver Configuration Functions
#[no_mangle]
unsafe extern "C" fn ov5693_init_controls(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_init_controls(struct ov5693_device *ov5693)
    {
    const struct v4l2_ctrl_ops *ops = &ov5693_ctrl_ops;
    struct ov5693_v4l2_ctrls *ctrls = &ov5693.ctrls;
    struct v4l2_fwnode_device_properties props;
    int vblank_max, vblank_def;
    int exposure_max;
    int hblank;
    int ret;
    ret = v4l2_ctrl_handler_init(&ctrls.handler, 12);
    if (ret)
    return ret;
// link freq
    ctrls.link_freq = v4l2_ctrl_new_int_menu(&ctrls.handler,
    core::ptr::null_mut(), V4L2_CID_LINK_FREQ,
    0, 0, link_freq_menu_items);
    if (ctrls.link_freq)
    ctrls.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
// pixel rate
    ctrls.pixel_rate = v4l2_ctrl_new_std(&ctrls.handler, core::ptr::null_mut(),
    V4L2_CID_PIXEL_RATE, 0,
    OV5693_PIXEL_RATE, 1,
    OV5693_PIXEL_RATE);
// Exposure
    exposure_max = ov5693.mode.vts - OV5693_INTEGRATION_TIME_MARGIN;
    ctrls.exposure = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_EXPOSURE,
    OV5693_EXPOSURE_MIN, exposure_max,
    OV5693_EXPOSURE_STEP, exposure_max);
// Gain
    ctrls.analogue_gain = v4l2_ctrl_new_std(&ctrls.handler,
    ops, V4L2_CID_ANALOGUE_GAIN,
    OV5693_GAIN_MIN,
    OV5693_GAIN_MAX,
    OV5693_GAIN_STEP,
    OV5693_GAIN_DEF);
    ctrls.digital_gain = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_DIGITAL_GAIN,
    OV5693_DIGITAL_GAIN_MIN,
    OV5693_DIGITAL_GAIN_MAX,
    OV5693_DIGITAL_GAIN_STEP,
    OV5693_DIGITAL_GAIN_DEF);
// Flip
    ctrls.hflip = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    ctrls.vflip = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    hblank = OV5693_FIXED_PPL - ov5693.mode.format.width;
    ctrls.hblank = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_HBLANK, hblank,
    hblank, 1, hblank);
    if (ctrls.hblank)
    ctrls.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    vblank_max = OV5693_TIMING_MAX_VTS - ov5693.mode.format.height;
    vblank_def = ov5693.mode.vts - ov5693.mode.format.height;
    ctrls.vblank = v4l2_ctrl_new_std(&ctrls.handler, ops,
    V4L2_CID_VBLANK,
    OV5693_TIMING_MIN_VTS,
    vblank_max, 1, vblank_def);
    ctrls.test_pattern = v4l2_ctrl_new_std_menu_items(
    &ctrls.handler, ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov5693_test_pattern_menu) - 1,
    0, 0, ov5693_test_pattern_menu);
    if (ctrls.handler.error) {
    dev_err(ov5693.dev, "Error initialising v4l2 ctrls\n");
    ret = ctrls.handler.error;
    goto err_free_handler;
    }
// set properties from fwnode (e.g. rotation, orientation)
    ret = v4l2_fwnode_device_parse(ov5693.dev, &props);
    if (ret)
    goto err_free_handler;
    ret = v4l2_ctrl_new_fwnode_properties(&ctrls.handler, ops,
    &props);
    if (ret)
    goto err_free_handler;
// Use same lock for controls as for everything else.
    ctrls.handler.lock = &ov5693.lock;
    ov5693.sd.ctrl_handler = &ctrls.handler;
    return 0;
    err_free_handler:
    v4l2_ctrl_handler_free(&ctrls.handler);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_configure_gpios(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_configure_gpios(struct ov5693_device *ov5693)
    {
    ov5693.reset = devm_gpiod_get_optional(ov5693.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov5693.reset)) {
    dev_err(ov5693.dev, "Error fetching reset GPIO\n");
    return PTR_ERR(ov5693.reset);
    }
    ov5693.powerdown = devm_gpiod_get_optional(ov5693.dev, "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov5693.powerdown)) {
    dev_err(ov5693.dev, "Error fetching powerdown GPIO\n");
    return PTR_ERR(ov5693.powerdown);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_get_regulators(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_get_regulators(struct ov5693_device *ov5693)
    {
    unsigned int i;
    for (i = 0; i < OV5693_NUM_SUPPLIES; i++)
    ov5693.supplies[i].supply = ov5693_supply_names[i];
    return devm_regulator_bulk_get(ov5693.dev, OV5693_NUM_SUPPLIES,
    ov5693.supplies);
    }
#[no_mangle]
unsafe extern "C" fn ov5693_check_hwcfg(ov5693: *mut ov5693_device) -> c_int {
    static int ov5693_check_hwcfg(struct ov5693_device *ov5693)
    {
    struct fwnode_handle *fwnode = dev_fwnode(ov5693.dev);
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY,
    };
    struct fwnode_handle *endpoint;
    unsigned int i;
    int ret;
//
// Sometimes the fwnode graph is initialized by the bridge driver
// Bridge drivers doing this may also add GPIO mappings, wait for this.
//
    endpoint = fwnode_graph_get_next_endpoint(fwnode, core::ptr::null_mut());
    if (!endpoint)
    return dev_err_probe(ov5693.dev, -EPROBE_DEFER,
    "waiting for fwnode graph endpoint\n");
    ret = v4l2_fwnode_endpoint_alloc_parse(endpoint, &bus_cfg);
    fwnode_handle_put(endpoint);
    if (ret)
    return ret;
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != 2) {
    dev_err(ov5693.dev, "only a 2-lane CSI2 config is supported");
    ret = -EINVAL;
    goto out_free_bus_cfg;
    }
    if (!bus_cfg.nr_of_link_frequencies) {
    dev_err(ov5693.dev, "no link frequencies defined\n");
    ret = -EINVAL;
    goto out_free_bus_cfg;
    }
    for (i = 0; i < bus_cfg.nr_of_link_frequencies; i++)
    if (bus_cfg.link_frequencies[i] == OV5693_LINK_FREQ_419_2MHZ)
    break;
    if (i == bus_cfg.nr_of_link_frequencies) {
    dev_err(ov5693.dev, "supported link freq %ull not found\n",
    OV5693_LINK_FREQ_419_2MHZ);
    ret = -EINVAL;
    goto out_free_bus_cfg;
    }
    out_free_bus_cfg:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_probe(client: *mut i2c_client) -> c_int {
    static int ov5693_probe(struct i2c_client *client)
    {
    struct ov5693_device *ov5693;
    u32 xvclk_rate;
    let mut ret: c_int = 0;
    ov5693 = devm_kzalloc(&client.dev, sizeof(*ov5693), GFP_KERNEL);
    if (!ov5693)
    return -ENOMEM;
    ov5693.dev = &client.dev;
    ov5693.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(ov5693.regmap))
    return PTR_ERR(ov5693.regmap);
    ret = ov5693_check_hwcfg(ov5693);
    if (ret)
    return ret;
    mutex_init(&ov5693.lock);
    v4l2_i2c_subdev_init(&ov5693.sd, client, &ov5693_ops);
    ov5693.xvclk = devm_v4l2_sensor_clk_get(&client.dev, "xvclk");
    if (IS_ERR(ov5693.xvclk))
    return dev_err_probe(&client.dev, PTR_ERR(ov5693.xvclk),
    "failed to get xvclk: %pe\n",
    ov5693.xvclk);
    xvclk_rate = clk_get_rate(ov5693.xvclk);
    if (xvclk_rate != OV5693_XVCLK_FREQ)
    dev_warn(&client.dev, "Found clk freq %u, expected %u\n",
    xvclk_rate, OV5693_XVCLK_FREQ);
    ret = ov5693_configure_gpios(ov5693);
    if (ret)
    return ret;
    ret = ov5693_get_regulators(ov5693);
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "Error fetching regulators\n");
    ov5693.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ov5693.pad.flags = MEDIA_PAD_FL_SOURCE;
    ov5693.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ov5693.mode.crop = ov5693_default_crop;
    ov5693.mode.format = ov5693_default_fmt;
    ov5693.mode.vts = __ov5693_calc_vts(ov5693.mode.format.height);
    ret = ov5693_init_controls(ov5693);
    if (ret)
    return ret;
    ret = media_entity_pads_init(&ov5693.sd.entity, 1, &ov5693.pad);
    if (ret)
    goto err_ctrl_handler_free;
//
// We need the driver to work in the event that pm runtime is disable in
// the kernel, so power up and verify the chip now. In the event that
// runtime pm is disabled this will leave the chip on, so that streaming
// will work.
//
    ret = ov5693_sensor_powerup(ov5693);
    if (ret)
    goto err_media_entity_cleanup;
    ret = ov5693_detect(ov5693);
    if (ret)
    goto err_powerdown;
    pm_runtime_set_active(&client.dev);
    pm_runtime_get_noresume(&client.dev);
    pm_runtime_enable(&client.dev);
    ret = v4l2_async_register_subdev_sensor(&ov5693.sd);
    if (ret) {
    dev_err(&client.dev, "failed to register V4L2 subdev: %d",
    ret);
    goto err_pm_runtime;
    }
    pm_runtime_set_autosuspend_delay(&client.dev, 1000);
    pm_runtime_use_autosuspend(&client.dev);
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    err_pm_runtime:
    pm_runtime_disable(&client.dev);
    pm_runtime_put_noidle(&client.dev);
    err_powerdown:
    ov5693_sensor_powerdown(ov5693);
    err_media_entity_cleanup:
    media_entity_cleanup(&ov5693.sd.entity);
    err_ctrl_handler_free:
    v4l2_ctrl_handler_free(&ov5693.ctrls.handler);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5693_remove(client: *mut i2c_client) {
    static void ov5693_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov5693_device *ov5693 = to_ov5693_sensor(sd);
    v4l2_async_unregister_subdev(sd);
    media_entity_cleanup(&ov5693.sd.entity);
    v4l2_ctrl_handler_free(&ov5693.ctrls.handler);
    mutex_destroy(&ov5693.lock);
//
// Disable runtime PM. In case runtime PM is disabled in the kernel,
// make sure to turn power off manually.
//
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev))
    ov5693_sensor_powerdown(ov5693);
    pm_runtime_set_suspended(&client.dev);
    }
    static const struct dev_pm_ops ov5693_pm_ops = {
    SET_RUNTIME_PM_OPS(ov5693_sensor_suspend, ov5693_sensor_resume, core::ptr::null_mut())
    };
    static const struct acpi_device_id ov5693_acpi_match[] = {
    {"INT33BE"},
    {},
    };
    MODULE_DEVICE_TABLE(acpi, ov5693_acpi_match);
    static const struct of_device_id ov5693_of_match[] = {
    { .compatible = "ovti,ov5693", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ov5693_of_match);
    static struct i2c_driver ov5693_driver = {
    .driver = {
    .name = "ov5693",
    .acpi_match_table = ov5693_acpi_match,
    .of_match_table = ov5693_of_match,
    .pm = &ov5693_pm_ops,
    },
    .probe = ov5693_probe,
    .remove = ov5693_remove,
    };
    module_i2c_driver(ov5693_driver);
    MODULE_DESCRIPTION("A low-level driver for OmniVision 5693 sensors");
    MODULE_LICENSE("GPL");
