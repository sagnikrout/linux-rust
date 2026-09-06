//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/imx219.c
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
// A V4L2 driver for Sony IMX219 cameras.
// Copyright (C) 2019, Raspberry Pi (Trading) Ltd
//
// Based on Sony imx258 camera driver
// Copyright (C) 2018 Intel Corporation
//
// DT / fwnode changes, and regulator / GPIO control taken from imx214 driver
// Copyright 2018 Qtechnology A/S
//
// Flip handling taken from the Sony IMX319 driver.
// Copyright (C) 2018 Intel Corporation
//

// Chip ID

pub const IMX219_CHIP_ID: c_uint = 0x0219;

pub const IMX219_MODE_STANDBY: c_uint = 0x00;
pub const IMX219_MODE_STREAMING: c_uint = 0x01;

pub const IMX219_CSI_2_LANE_MODE: c_uint = 0x01;
pub const IMX219_CSI_4_LANE_MODE: c_uint = 0x03;

pub const IMX219_DPHY_CTRL_TIMING_AUTO: c_int = 0;
pub const IMX219_DPHY_CTRL_TIMING_MANUAL: c_int = 1;

// Analog gain control

pub const IMX219_ANA_GAIN_MIN: c_int = 0;
pub const IMX219_ANA_GAIN_MAX: c_int = 232;
pub const IMX219_ANA_GAIN_STEP: c_int = 1;
pub const IMX219_ANA_GAIN_DEFAULT: c_uint = 0x0;
// Digital gain control

pub const IMX219_DGTL_GAIN_MIN: c_uint = 0x0100;
pub const IMX219_DGTL_GAIN_MAX: c_uint = 0x0fff;
pub const IMX219_DGTL_GAIN_DEFAULT: c_uint = 0x0100;
pub const IMX219_DGTL_GAIN_STEP: c_int = 1;
// Exposure control

pub const IMX219_EXPOSURE_MIN: c_int = 4;
pub const IMX219_EXPOSURE_STEP: c_int = 1;
pub const IMX219_EXPOSURE_DEFAULT: c_uint = 0x640;
pub const IMX219_EXPOSURE_MAX: c_int = 65535;
pub const IMX219_EXPOSURE_OFFSET: c_int = 4;
// V_TIMING internal

pub const IMX219_FLL_MAX: c_uint = 0xfffe;
pub const IMX219_VBLANK_MIN: c_int = 32;

pub const IMX219_LLP_MIN: c_uint = 0x0d78;
pub const IMX219_BINNED_LLP_MIN: c_uint = 0x0de8;
pub const IMX219_LLP_MAX: c_uint = 0x7ff0;

// Binning  Mode

pub const IMX219_BINNING_NONE: c_uint = 0x00;
pub const IMX219_BINNING_X2: c_uint = 0x01;
pub const IMX219_BINNING_X2_ANALOG: c_uint = 0x03;

// PLL Settings

// Test Pattern Control

pub const IMX219_TEST_PATTERN_DISABLE: c_int = 0;
pub const IMX219_TEST_PATTERN_SOLID_COLOR: c_int = 1;
pub const IMX219_TEST_PATTERN_COLOR_BARS: c_int = 2;
pub const IMX219_TEST_PATTERN_GREY_COLOR: c_int = 3;
pub const IMX219_TEST_PATTERN_PN9: c_int = 4;
pub const IMX219_TEST_PATTERN_16SPLIT_COLOR_BARS: c_int = 5;
pub const IMX219_TEST_PATTERN_16SPLIT_INV_COLOR_BARS: c_int = 6;
pub const IMX219_TEST_PATTERN_COLUMN_COUNTER: c_int = 7;
pub const IMX219_TEST_PATTERN_INV_COLUMN_COUNTER: c_int = 8;
pub const IMX219_TEST_PATTERN_PN31: c_int = 9;
// Test pattern colour components

pub const IMX219_TESTP_COLOUR_MIN: c_int = 0;
pub const IMX219_TESTP_COLOUR_MAX: c_uint = 0x03ff;
pub const IMX219_TESTP_COLOUR_STEP: c_int = 1;

// External clock frequency is 24.0M
pub const IMX219_XCLK_FREQ: c_int = 24000000;
// Pixel rate is fixed for all the modes
pub const IMX219_PIXEL_RATE: c_int = 182400000;
pub const IMX219_PIXEL_RATE_4LANE: c_int = 281600000;
pub const IMX219_DEFAULT_LINK_FREQ: c_int = 456000000;
pub const IMX219_DEFAULT_LINK_FREQ_4LANE_UNSUPPORTED: c_int = 363000000;
pub const IMX219_DEFAULT_LINK_FREQ_4LANE: c_int = 364000000;
// IMX219 native and active pixel array size.

// Mode : resolution and related config&values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx219_mode {
// Frame width
    pub width: c_uint,
// Frame height
    pub height: c_uint,
// V-timing
    pub fll_def: c_uint,
}

    static const struct cci_reg_sequence imx219_common_regs[] = {
    { IMX219_REG_MODE_SELECT, 0x00 },	/* Mode Select */
// To Access Addresses 3000-5fff, send the following commands
    { CCI_REG8(0x30eb), 0x05 },
    { CCI_REG8(0x30eb), 0x0c },
    { CCI_REG8(0x300a), 0xff },
    { CCI_REG8(0x300b), 0xff },
    { CCI_REG8(0x30eb), 0x05 },
    { CCI_REG8(0x30eb), 0x09 },
// Undocumented registers
    { CCI_REG8(0x455e), 0x00 },
    { CCI_REG8(0x471e), 0x4b },
    { CCI_REG8(0x4767), 0x0f },
    { CCI_REG8(0x4750), 0x14 },
    { CCI_REG8(0x4540), 0x00 },
    { CCI_REG8(0x47b4), 0x14 },
    { CCI_REG8(0x4713), 0x30 },
    { CCI_REG8(0x478b), 0x10 },
    { CCI_REG8(0x478f), 0x10 },
    { CCI_REG8(0x4793), 0x10 },
    { CCI_REG8(0x4797), 0x0e },
    { CCI_REG8(0x479b), 0x0e },
// Frame Bank Register Group "A"
    { IMX219_REG_X_ODD_INC_A, 1 },
    { IMX219_REG_Y_ODD_INC_A, 1 },
// Output setup registers
    { IMX219_REG_DPHY_CTRL, IMX219_DPHY_CTRL_TIMING_AUTO },
    { IMX219_REG_EXCK_FREQ, IMX219_EXCK_FREQ(IMX219_XCLK_FREQ / 1000000) },
    };
    static const struct cci_reg_sequence imx219_2lane_regs[] = {
// PLL Clock Table
    { IMX219_REG_VTPXCK_DIV, 5 },
    { IMX219_REG_VTSYCK_DIV, 1 },
    { IMX219_REG_PREPLLCK_VT_DIV, 3 },	/* 0x03 = AUTO set */
    { IMX219_REG_PREPLLCK_OP_DIV, 3 },	/* 0x03 = AUTO set */
    { IMX219_REG_PLL_VT_MPY, 57 },
    { IMX219_REG_OPSYCK_DIV, 1 },
    { IMX219_REG_PLL_OP_MPY, 114 },
// 2-Lane CSI Mode
    { IMX219_REG_CSI_LANE_MODE, IMX219_CSI_2_LANE_MODE },
    };
    static const struct cci_reg_sequence imx219_4lane_regs[] = {
// PLL Clock Table
    { IMX219_REG_VTPXCK_DIV, 5 },
    { IMX219_REG_VTSYCK_DIV, 1 },
    { IMX219_REG_PREPLLCK_VT_DIV, 3 },	/* 0x03 = AUTO set */
    { IMX219_REG_PREPLLCK_OP_DIV, 3 },	/* 0x03 = AUTO set */
    { IMX219_REG_PLL_VT_MPY, 88 },
    { IMX219_REG_OPSYCK_DIV, 1 },
    { IMX219_REG_PLL_OP_MPY, 91 },
// 4-Lane CSI Mode
    { IMX219_REG_CSI_LANE_MODE, IMX219_CSI_4_LANE_MODE },
    };
    static const s64 imx219_link_freq_menu[] = {
    IMX219_DEFAULT_LINK_FREQ,
    };
    static const s64 imx219_link_freq_4lane_menu[] = {
    IMX219_DEFAULT_LINK_FREQ_4LANE,
//
// This will never be advertised to userspace, but will be used for
// v4l2_link_freq_to_bitmap
//
    IMX219_DEFAULT_LINK_FREQ_4LANE_UNSUPPORTED,
    };
    static const char * const imx219_test_pattern_menu[] = {
    "Disabled",
    "Color Bars",
    "Solid Color",
    "Grey Color Bars",
    "PN9",
    "16 Split Color Bars",
    "16 Split Inverted Color Bars",
    "Column Counter",
    "Inverted Column Counter",
    "PN31"
    };
    static const int imx219_test_pattern_val[] = {
    IMX219_TEST_PATTERN_DISABLE,
    IMX219_TEST_PATTERN_COLOR_BARS,
    IMX219_TEST_PATTERN_SOLID_COLOR,
    IMX219_TEST_PATTERN_GREY_COLOR,
    IMX219_TEST_PATTERN_PN9,
    IMX219_TEST_PATTERN_16SPLIT_COLOR_BARS,
    IMX219_TEST_PATTERN_16SPLIT_INV_COLOR_BARS,
    IMX219_TEST_PATTERN_COLUMN_COUNTER,
    IMX219_TEST_PATTERN_INV_COLUMN_COUNTER,
    IMX219_TEST_PATTERN_PN31
    };
// regulator supplies
    static const char * const imx219_supply_name[] = {
// Supplies can be enabled in any order
    "VANA",  /* Analog (2.8V) supply */
    "VDIG",  /* Digital Core (1.8V) supply */
    "VDDL",  /* IF (1.2V) supply */
    };

//
// The supported formats.
// This table MUST contain 4 entries per format, to cover the various flip
// combinations in the order
// - no flip
// - h flip
// - v flip
// - h&v flips
//
    static const u32 imx219_mbus_formats[] = {
    MEDIA_BUS_FMT_SRGGB10_1X10,
    MEDIA_BUS_FMT_SGRBG10_1X10,
    MEDIA_BUS_FMT_SGBRG10_1X10,
    MEDIA_BUS_FMT_SBGGR10_1X10,
    MEDIA_BUS_FMT_SRGGB8_1X8,
    MEDIA_BUS_FMT_SGRBG8_1X8,
    MEDIA_BUS_FMT_SGBRG8_1X8,
    MEDIA_BUS_FMT_SBGGR8_1X8,
    };
//
// Initialisation delay between XCLR low->high and the moment when the sensor
// can start capture (i.e. can leave software stanby) must be not less than:
// t4 + max(t5, t6 + <time to initialize the sensor register over I2C>)
// where
// t4 is fixed, and is max 200uS,
// t5 is fixed, and is 6000uS,
// t6 depends on the sensor external clock, and is max 32000 clock periods.
// As per sensor datasheet, the external clock must be from 6MHz to 27MHz.
// So for any acceptable external clock t6 is always within the range of
// 1185 to 5333 uS, and is always less than t5.
// For this reason this is always safe to wait (t4 + t5) = 6200 uS, then
// initialize the sensor over I2C, and then exit the software standby.
//
// This start-up time can be optimized a bit more, if we start the writes
// over I2C after (t4+t6), but before (t4+t5) expires. But then sensor
// initialization over I2C may complete before (t4+t5) expires, and we must
// ensure that capture is not started before (t4+t5).
//
// This delay doesn't account for the power supply startup time. If needed,
// this should be taken care of via the regulator framework. E.g. in the
// case of DT for regulator-fixed one should define the startup-delay-us
// property.
//
pub const IMX219_XCLR_MIN_DELAY_US: c_int = 6200;
pub const IMX219_XCLR_DELAY_RANGE_US: c_int = 1000;
// Mode configs
    static const struct imx219_mode supported_modes[] = {
    {
// 8MPix 15fps mode
    .width = 3280,
    .height = 2464,
    .fll_def = 3526,
    },
    {
// 1080P 30fps cropped
    .width = 1920,
    .height = 1080,
    .fll_def = 1763,
    },
    {
// 2x2 binned 60fps mode
    .width = 1640,
    .height = 1232,
    .fll_def = 1707,
    },
    {
// 640x480 60fps mode
    .width = 640,
    .height = 480,
    .fll_def = 1707,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx219 {
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub regmap: *mut regmap,
    pub /: *mut *mut *mut clk xclk; / system clock to IMX219,
    pub xclk_freq: u32,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; IMX219_NUM_SUPPLIES],
    pub ctrl_handler: v4l2_ctrl_handler,
// V4L2 Controls
    pub pixel_rate: *mut v4l2_ctrl,
    pub link_freq: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
// Two or Four lanes
    pub lanes: u8,
}

    static inline struct imx219 *to_imx219(struct v4l2_subdev *_sd)
    {
    return container_of(_sd, struct imx219, sd);
    }
// Get bayer order based on flip setting.
#[no_mangle]
unsafe extern "C" fn imx219_get_format_code(imx219: *mut imx219, code: u32) -> u32 {
    static u32 imx219_get_format_code(struct imx219 *imx219, u32 code)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(imx219_mbus_formats); i++)
    if (imx219_mbus_formats[i] == code)
    break;
    if (i >= ARRAY_SIZE(imx219_mbus_formats))
    i = 0;
    i = (i & ~3) | (imx219.vflip.val ? 2 : 0) |
    (imx219.hflip.val ? 1 : 0);
    return imx219_mbus_formats[i];
    }
#[no_mangle]
unsafe extern "C" fn imx219_get_format_bpp(format: *const v4l2_mbus_framefmt) -> u32 {
    static u32 imx219_get_format_bpp(const struct v4l2_mbus_framefmt *format)
    {
    switch (format.code) {
    case MEDIA_BUS_FMT_SRGGB8_1X8:
    case MEDIA_BUS_FMT_SGRBG8_1X8:
    case MEDIA_BUS_FMT_SGBRG8_1X8:
    case MEDIA_BUS_FMT_SBGGR8_1X8:
    return 8;
    case MEDIA_BUS_FMT_SRGGB10_1X10:
    case MEDIA_BUS_FMT_SGRBG10_1X10:
    case MEDIA_BUS_FMT_SGBRG10_1X10:
    case MEDIA_BUS_FMT_SBGGR10_1X10:
    default:
    return 10;
    }
    }
    static void imx219_get_binning(struct v4l2_subdev_state *state, u8 *bin_h,
    u8 *bin_v)
    {
    const struct v4l2_mbus_framefmt *format =
    v4l2_subdev_state_get_format(state, 0);
    const struct v4l2_rect *crop = v4l2_subdev_state_get_crop(state, 0);
    let mut hbin: u32 = crop.width / format.width;
    let mut vbin: u32 = crop.height / format.height;
    if (hbin == 2 && vbin == 2) {
// bin_h = IMX219_BINNING_X2_ANALOG;
// bin_v = IMX219_BINNING_X2_ANALOG;
    } else {
// bin_h = IMX219_BINNING_NONE;
// bin_v = IMX219_BINNING_NONE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn imx219_get_rate_factor(state: *mut v4l2_subdev_state) -> u32 {
    static inline u32 imx219_get_rate_factor(struct v4l2_subdev_state *state)
    {
    u8 bin_h, bin_v;
    imx219_get_binning(state, &bin_h, &bin_v);
    return (bin_h & bin_v) == IMX219_BINNING_X2_ANALOG ? 2 : 1;
    }
// -----------------------------------------------------------------------------
// Controls
//
#[no_mangle]
unsafe extern "C" fn imx219_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int imx219_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct imx219 *imx219 =
    container_of(ctrl.handler, struct imx219, ctrl_handler);
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    const struct v4l2_mbus_framefmt *format;
    struct v4l2_subdev_state *state;
    u32 rate_factor;
    let mut ret: c_int = 0;
    state = v4l2_subdev_get_locked_active_state(&imx219.sd);
    format = v4l2_subdev_state_get_format(state, 0);
    rate_factor = imx219_get_rate_factor(state);
    if (ctrl.id == V4L2_CID_VBLANK) {
    int exposure_max, exposure_def;
// Update max exposure while meeting expected vblanking
    exposure_max = format.height + ctrl.val - IMX219_EXPOSURE_OFFSET;
    exposure_def = (exposure_max < IMX219_EXPOSURE_DEFAULT) ?
    exposure_max : IMX219_EXPOSURE_DEFAULT;
    ret = __v4l2_ctrl_modify_range(imx219.exposure,
    imx219.exposure.minimum,
    exposure_max,
    imx219.exposure.step,
    exposure_def);
    if (ret)
    return ret;
    }
//
// Applying V4L2 control value only happens
// when power is up for streaming
//
    if (pm_runtime_get_if_in_use(&client.dev) == 0)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_ANALOGUE_GAIN:
    cci_write(imx219.regmap, IMX219_REG_ANALOG_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_EXPOSURE:
    cci_write(imx219.regmap, IMX219_REG_EXPOSURE,
    ctrl.val / rate_factor, &ret);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    cci_write(imx219.regmap, IMX219_REG_DIGITAL_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN:
    cci_write(imx219.regmap, IMX219_REG_TEST_PATTERN,
    imx219_test_pattern_val[ctrl.val], &ret);
    break;
    case V4L2_CID_HFLIP:
    case V4L2_CID_VFLIP:
    cci_write(imx219.regmap, IMX219_REG_ORIENTATION,
    imx219.hflip.val | imx219.vflip.val << 1, &ret);
    break;
    case V4L2_CID_VBLANK:
    cci_write(imx219.regmap, IMX219_REG_FRM_LENGTH_A,
    (format.height + ctrl.val) / rate_factor, &ret);
    break;
    case V4L2_CID_HBLANK:
    cci_write(imx219.regmap, IMX219_REG_LINE_LENGTH_A,
    format.width + ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_RED:
    cci_write(imx219.regmap, IMX219_REG_TESTP_RED,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_GREENR:
    cci_write(imx219.regmap, IMX219_REG_TESTP_GREENR,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_BLUE:
    cci_write(imx219.regmap, IMX219_REG_TESTP_BLUE,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_GREENB:
    cci_write(imx219.regmap, IMX219_REG_TESTP_GREENB,
    ctrl.val, &ret);
    break;
    default:
    dev_info(&client.dev,
    "ctrl(id:0x%x,val:0x%x) is not handled\n",
    ctrl.id, ctrl.val);
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(&client.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops imx219_ctrl_ops = {
    .s_ctrl = imx219_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn imx219_get_pixel_rate(imx219: *mut imx219) -> c_ulong {
    static unsigned long imx219_get_pixel_rate(struct imx219 *imx219)
    {
    return (imx219.lanes == 2) ? IMX219_PIXEL_RATE : IMX219_PIXEL_RATE_4LANE;
    }
// Initialize control handlers
#[no_mangle]
unsafe extern "C" fn imx219_init_controls(imx219: *mut imx219) -> c_int {
    static int imx219_init_controls(struct imx219 *imx219)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    const struct imx219_mode *mode = &supported_modes[0];
    struct v4l2_ctrl_handler *ctrl_hdlr;
    struct v4l2_fwnode_device_properties props;
    int exposure_max, exposure_def;
    int i, ret;
    ctrl_hdlr = &imx219.ctrl_handler;
    ret = v4l2_ctrl_handler_init(ctrl_hdlr, 12);
    if (ret)
    return ret;
// By default, PIXEL_RATE is read only
    imx219.pixel_rate = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_PIXEL_RATE,
    imx219_get_pixel_rate(imx219),
    imx219_get_pixel_rate(imx219), 1,
    imx219_get_pixel_rate(imx219));
    imx219.link_freq =
    v4l2_ctrl_new_int_menu(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(imx219_link_freq_menu) - 1, 0,
    (imx219.lanes == 2) ? imx219_link_freq_menu :
    imx219_link_freq_4lane_menu);
    if (imx219.link_freq)
    imx219.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
// Initial blanking and exposure. Limits are updated during set_fmt
    imx219.vblank = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_VBLANK, IMX219_VBLANK_MIN,
    IMX219_FLL_MAX - mode.height, 1,
    mode.fll_def - mode.height);
    imx219.hblank = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_HBLANK,
    IMX219_LLP_MIN - mode.width,
    IMX219_LLP_MAX - mode.width, 1,
    IMX219_LLP_MIN - mode.width);
    exposure_max = mode.fll_def - IMX219_EXPOSURE_OFFSET;
    exposure_def = (exposure_max < IMX219_EXPOSURE_DEFAULT) ?
    exposure_max : IMX219_EXPOSURE_DEFAULT;
    imx219.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_EXPOSURE,
    IMX219_EXPOSURE_MIN, exposure_max,
    IMX219_EXPOSURE_STEP,
    exposure_def);
    v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops, V4L2_CID_ANALOGUE_GAIN,
    IMX219_ANA_GAIN_MIN, IMX219_ANA_GAIN_MAX,
    IMX219_ANA_GAIN_STEP, IMX219_ANA_GAIN_DEFAULT);
    v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops, V4L2_CID_DIGITAL_GAIN,
    IMX219_DGTL_GAIN_MIN, IMX219_DGTL_GAIN_MAX,
    IMX219_DGTL_GAIN_STEP, IMX219_DGTL_GAIN_DEFAULT);
    imx219.hflip = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    if (imx219.hflip)
    imx219.hflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    imx219.vflip = v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    if (imx219.vflip)
    imx219.vflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    v4l2_ctrl_new_std_menu_items(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(imx219_test_pattern_menu) - 1,
    0, 0, imx219_test_pattern_menu);
    for (i = 0; i < 4; i++) {
//
// The assumption is that
// V4L2_CID_TEST_PATTERN_GREENR == V4L2_CID_TEST_PATTERN_RED + 1
// V4L2_CID_TEST_PATTERN_BLUE   == V4L2_CID_TEST_PATTERN_RED + 2
// V4L2_CID_TEST_PATTERN_GREENB == V4L2_CID_TEST_PATTERN_RED + 3
//
    v4l2_ctrl_new_std(ctrl_hdlr, &imx219_ctrl_ops,
    V4L2_CID_TEST_PATTERN_RED + i,
    IMX219_TESTP_COLOUR_MIN,
    IMX219_TESTP_COLOUR_MAX,
    IMX219_TESTP_COLOUR_STEP,
    IMX219_TESTP_COLOUR_MAX);
// The "Solid color" pattern is white by default
    }
    if (ctrl_hdlr.error) {
    ret = ctrl_hdlr.error;
    dev_err_probe(&client.dev, ret, "Control init failed\n");
    goto error;
    }
    ret = v4l2_fwnode_device_parse(&client.dev, &props);
    if (ret)
    goto error;
    ret = v4l2_ctrl_new_fwnode_properties(ctrl_hdlr, &imx219_ctrl_ops,
    &props);
    if (ret)
    goto error;
    imx219.sd.ctrl_handler = ctrl_hdlr;
    return 0;
    error:
    v4l2_ctrl_handler_free(ctrl_hdlr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx219_free_controls(imx219: *mut imx219) {
    static void imx219_free_controls(struct imx219 *imx219)
    {
    v4l2_ctrl_handler_free(imx219.sd.ctrl_handler);
    }
// -----------------------------------------------------------------------------
// Subdev operations
//
    static int imx219_set_framefmt(struct imx219 *imx219,
    struct v4l2_subdev_state *state)
    {
    const struct v4l2_mbus_framefmt *format;
    const struct v4l2_rect *crop;
    u8 bin_h, bin_v;
    u32 bpp;
    let mut ret: c_int = 0;
    format = v4l2_subdev_state_get_format(state, 0);
    crop = v4l2_subdev_state_get_crop(state, 0);
    bpp = imx219_get_format_bpp(format);
    cci_write(imx219.regmap, IMX219_REG_X_ADD_STA_A,
    crop.left - IMX219_ACTIVE_AREA_LEFT, &ret);
    cci_write(imx219.regmap, IMX219_REG_X_ADD_END_A,
    crop.left - IMX219_ACTIVE_AREA_LEFT + crop.width - 1, &ret);
    cci_write(imx219.regmap, IMX219_REG_Y_ADD_STA_A,
    crop.top - IMX219_ACTIVE_AREA_TOP, &ret);
    cci_write(imx219.regmap, IMX219_REG_Y_ADD_END_A,
    crop.top - IMX219_ACTIVE_AREA_TOP + crop.height - 1, &ret);
    imx219_get_binning(state, &bin_h, &bin_v);
    cci_write(imx219.regmap, IMX219_REG_BINNING_MODE_H, bin_h, &ret);
    cci_write(imx219.regmap, IMX219_REG_BINNING_MODE_V, bin_v, &ret);
    cci_write(imx219.regmap, IMX219_REG_X_OUTPUT_SIZE,
    format.width, &ret);
    cci_write(imx219.regmap, IMX219_REG_Y_OUTPUT_SIZE,
    format.height, &ret);
    cci_write(imx219.regmap, IMX219_REG_TP_WINDOW_WIDTH,
    format.width, &ret);
    cci_write(imx219.regmap, IMX219_REG_TP_WINDOW_HEIGHT,
    format.height, &ret);
    cci_write(imx219.regmap, IMX219_REG_CSI_DATA_FORMAT_A,
    (bpp << 8) | bpp, &ret);
    cci_write(imx219.regmap, IMX219_REG_OPPXCK_DIV, bpp, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx219_configure_lanes(imx219: *mut imx219) -> c_int {
    static int imx219_configure_lanes(struct imx219 *imx219)
    {
// Write the appropriate PLL settings for the number of MIPI lanes
    return cci_multi_reg_write(imx219.regmap,
    imx219.lanes == 2 ? imx219_2lane_regs : imx219_4lane_regs,
    imx219.lanes == 2 ? ARRAY_SIZE(imx219_2lane_regs) :
    ARRAY_SIZE(imx219_4lane_regs), core::ptr::null_mut());
    };
    static int imx219_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 pad,
    u64 streams_mask)
    {
    struct imx219 *imx219 = to_imx219(sd);
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    int ret;
    ret = pm_runtime_resume_and_get(&client.dev);
    if (ret < 0)
    return ret;
// Send all registers that are common to all modes
    ret = cci_multi_reg_write(imx219.regmap, imx219_common_regs,
    ARRAY_SIZE(imx219_common_regs), core::ptr::null_mut());
    if (ret) {
    dev_err(&client.dev, "%s failed to send mfg header\n", __func__);
    goto err_rpm_put;
    }
// Configure two or four Lane mode
    ret = imx219_configure_lanes(imx219);
    if (ret) {
    dev_err(&client.dev, "%s failed to configure lanes\n", __func__);
    goto err_rpm_put;
    }
// Apply format and crop settings.
    ret = imx219_set_framefmt(imx219, state);
    if (ret) {
    dev_err(&client.dev, "%s failed to set frame format: %d\n",
    __func__, ret);
    goto err_rpm_put;
    }
// Apply customized values from user
    ret =  __v4l2_ctrl_handler_setup(imx219.sd.ctrl_handler);
    if (ret)
    goto err_rpm_put;
// set stream on register
    ret = cci_write(imx219.regmap, IMX219_REG_MODE_SELECT,
    IMX219_MODE_STREAMING, core::ptr::null_mut());
    if (ret)
    goto err_rpm_put;
// vflip and hflip cannot change during streaming
    __v4l2_ctrl_grab(imx219.vflip, true);
    __v4l2_ctrl_grab(imx219.hflip, true);
    return 0;
    err_rpm_put:
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    }
    static int imx219_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 pad,
    u64 streams_mask)
    {
    struct imx219 *imx219 = to_imx219(sd);
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    int ret;
// set stream off register
    ret = cci_write(imx219.regmap, IMX219_REG_MODE_SELECT,
    IMX219_MODE_STANDBY, core::ptr::null_mut());
    if (ret)
    dev_err(&client.dev, "%s failed to set stream\n", __func__);
    __v4l2_ctrl_grab(imx219.vflip, false);
    __v4l2_ctrl_grab(imx219.hflip, false);
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    }
    static int imx219_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    struct imx219 *imx219 = to_imx219(sd);
    if (code.index >= (ARRAY_SIZE(imx219_mbus_formats) / 4))
    return -EINVAL;
    code.code = imx219_get_format_code(imx219, imx219_mbus_formats[code.index * 4]);
    return 0;
    }
    static int imx219_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    struct imx219 *imx219 = to_imx219(sd);
    u32 code;
    if (fse.index >= ARRAY_SIZE(supported_modes))
    return -EINVAL;
    code = imx219_get_format_code(imx219, fse.code);
    if (fse.code != code)
    return -EINVAL;
    fse.min_width = supported_modes[fse.index].width;
    fse.max_width = fse.min_width;
    fse.min_height = supported_modes[fse.index].height;
    fse.max_height = fse.min_height;
    return 0;
    }
    static int imx219_set_pad_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *fmt)
    {
    struct imx219 *imx219 = to_imx219(sd);
    const struct imx219_mode *mode;
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    u8 bin_h, bin_v, binning;
    int ret;
    format = v4l2_subdev_state_get_format(state, 0);
//
// Adjust the requested format to match the closest mode. The Bayer
// order varies with flips.
//
    mode = v4l2_find_nearest_size(supported_modes,
    ARRAY_SIZE(supported_modes),
    width, height,
    fmt.format.width, fmt.format.height);
    fmt.format.code = imx219_get_format_code(imx219, fmt.format.code);
    fmt.format.width = mode.width;
    fmt.format.height = mode.height;
    fmt.format.field = V4L2_FIELD_NONE;
    fmt.format.colorspace = V4L2_COLORSPACE_RAW;
    fmt.format.ycbcr_enc = V4L2_YCBCR_ENC_601;
    fmt.format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    fmt.format.xfer_func = V4L2_XFER_FUNC_NONE;
// format = fmt->format;
//
// Use binning to maximize the crop rectangle size, and centre it in the
// sensor.
//
    bin_h = min(IMX219_ACTIVE_AREA_WIDTH / format.width, 2U);
    bin_v = min(IMX219_ACTIVE_AREA_HEIGHT / format.height, 2U);
// Ensure bin_h and bin_v are same to avoid 1:2 or 2:1 stretching
    binning = min(bin_h, bin_v);
    crop = v4l2_subdev_state_get_crop(state, 0);
    crop.width = format.width * binning;
    crop.height = format.height * binning;
    crop.left = (IMX219_NATIVE_WIDTH - crop.width) / 2;
    crop.top = (IMX219_NATIVE_HEIGHT - crop.height) / 2;
    if (fmt.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    int exposure_max;
    int exposure_def;
    int llp_min;
    int pixel_rate;
// Update limits and set FPS to default
    ret = __v4l2_ctrl_modify_range(imx219.vblank, IMX219_VBLANK_MIN,
    IMX219_FLL_MAX - mode.height, 1,
    mode.fll_def - mode.height);
    if (ret)
    return ret;
    ret = __v4l2_ctrl_s_ctrl(imx219.vblank,
    mode.fll_def - mode.height);
    if (ret)
    return ret;
// Update max exposure while meeting expected vblanking
    exposure_max = mode.fll_def - IMX219_EXPOSURE_OFFSET;
    exposure_def = (exposure_max < IMX219_EXPOSURE_DEFAULT) ?
    exposure_max : IMX219_EXPOSURE_DEFAULT;
    ret = __v4l2_ctrl_modify_range(imx219.exposure,
    imx219.exposure.minimum,
    exposure_max,
    imx219.exposure.step,
    exposure_def);
    if (ret)
    return ret;
//
// With analog binning the default minimum line length of 3448
// can cause artefacts with RAW10 formats, because the ADC
// operates on two lines together. So we switch to a higher
// minimum of 3560.
//
    imx219_get_binning(state, &bin_h, &bin_v);
    llp_min = (bin_h & bin_v) == IMX219_BINNING_X2_ANALOG ?
    IMX219_BINNED_LLP_MIN : IMX219_LLP_MIN;
    ret = __v4l2_ctrl_modify_range(imx219.hblank,
    llp_min - mode.width,
    IMX219_LLP_MAX - mode.width, 1,
    llp_min - mode.width);
    if (ret)
    return ret;
    ret = __v4l2_ctrl_s_ctrl(imx219.hblank, llp_min - mode.width);
    if (ret)
    return ret;
// Scale the pixel rate based on the mode specific factor
    pixel_rate = imx219_get_pixel_rate(imx219) *
    imx219_get_rate_factor(state);
    ret = __v4l2_ctrl_modify_range(imx219.pixel_rate, pixel_rate,
    pixel_rate, 1, pixel_rate);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int imx219_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    sel.r = *v4l2_subdev_state_get_crop(state, 0);
    return 0;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = IMX219_NATIVE_WIDTH;
    sel.r.height = IMX219_NATIVE_HEIGHT;
    return 0;
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = IMX219_ACTIVE_AREA_TOP;
    sel.r.left = IMX219_ACTIVE_AREA_LEFT;
    sel.r.width = IMX219_ACTIVE_AREA_WIDTH;
    sel.r.height = IMX219_ACTIVE_AREA_HEIGHT;
    return 0;
    }
    return -EINVAL;
    }
    static int imx219_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct v4l2_subdev_format fmt = {
    .which = V4L2_SUBDEV_FORMAT_TRY,
    .pad = 0,
    .format = {
    .code = MEDIA_BUS_FMT_SRGGB10_1X10,
    .width = supported_modes[0].width,
    .height = supported_modes[0].height,
    },
    };
    return imx219_set_pad_format(sd, state, &fmt);
    }
    static const struct v4l2_subdev_video_ops imx219_video_ops = {
    .s_stream = v4l2_subdev_s_stream_helper,
    };
    static const struct v4l2_subdev_pad_ops imx219_pad_ops = {
    .enum_mbus_code = imx219_enum_mbus_code,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = imx219_set_pad_format,
    .get_selection = imx219_get_selection,
    .enum_frame_size = imx219_enum_frame_size,
    .enable_streams = imx219_enable_streams,
    .disable_streams = imx219_disable_streams,
    };
    static const struct v4l2_subdev_ops imx219_subdev_ops = {
    .video = &imx219_video_ops,
    .pad = &imx219_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops imx219_internal_ops = {
    .init_state = imx219_init_state,
    };
// -----------------------------------------------------------------------------
// Power management
//
#[no_mangle]
unsafe extern "C" fn imx219_power_on(dev: *mut device) -> c_int {
    static int imx219_power_on(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct imx219 *imx219 = to_imx219(sd);
    int ret;
    ret = regulator_bulk_enable(IMX219_NUM_SUPPLIES,
    imx219.supplies);
    if (ret) {
    dev_err(dev, "%s: failed to enable regulators\n",
    __func__);
    return ret;
    }
    ret = clk_prepare_enable(imx219.xclk);
    if (ret) {
    dev_err(dev, "%s: failed to enable clock\n",
    __func__);
    goto reg_off;
    }
//
// Note: Misinterpretation of reset assertion - do not re-use this code.
// XCLR pin is using incorrect (for reset signal) logical level.
//
    gpiod_set_value_cansleep(imx219.reset_gpio, 1);
    usleep_range(IMX219_XCLR_MIN_DELAY_US,
    IMX219_XCLR_MIN_DELAY_US + IMX219_XCLR_DELAY_RANGE_US);
    return 0;
    reg_off:
    regulator_bulk_disable(IMX219_NUM_SUPPLIES, imx219.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx219_power_off(dev: *mut device) -> c_int {
    static int imx219_power_off(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct imx219 *imx219 = to_imx219(sd);
    gpiod_set_value_cansleep(imx219.reset_gpio, 0);
    regulator_bulk_disable(IMX219_NUM_SUPPLIES, imx219.supplies);
    clk_disable_unprepare(imx219.xclk);
    return 0;
    }
// -----------------------------------------------------------------------------
// Probe & remove
//
#[no_mangle]
unsafe extern "C" fn imx219_get_regulators(imx219: *mut imx219) -> c_int {
    static int imx219_get_regulators(struct imx219 *imx219)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    unsigned int i;
    for (i = 0; i < IMX219_NUM_SUPPLIES; i++)
    imx219.supplies[i].supply = imx219_supply_name[i];
    return devm_regulator_bulk_get(&client.dev,
    IMX219_NUM_SUPPLIES,
    imx219.supplies);
    }
// Verify chip ID
#[no_mangle]
unsafe extern "C" fn imx219_identify_module(imx219: *mut imx219) -> c_int {
    static int imx219_identify_module(struct imx219 *imx219)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&imx219.sd);
    int ret;
    u64 val;
    ret = cci_read(imx219.regmap, IMX219_REG_CHIP_ID, &val, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "failed to read chip id %x\n",
    IMX219_CHIP_ID);
    if (val != IMX219_CHIP_ID)
    return dev_err_probe(&client.dev, -EIO,
    "chip id mismatch: %x!=%llx\n",
    IMX219_CHIP_ID, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx219_check_hwcfg(dev: *mut device, imx219: *mut imx219) -> c_int {
    static int imx219_check_hwcfg(struct device *dev, struct imx219 *imx219)
    {
    struct fwnode_handle *endpoint;
    struct v4l2_fwnode_endpoint ep_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY
    };
    unsigned long link_freq_bitmap;
    let mut ret: c_int = -EINVAL;
    endpoint = fwnode_graph_get_next_endpoint(dev_fwnode(dev), core::ptr::null_mut());
    if (!endpoint)
    return dev_err_probe(dev, -EINVAL, "endpoint node not found\n");
    if (v4l2_fwnode_endpoint_alloc_parse(endpoint, &ep_cfg)) {
    dev_err_probe(dev, -EINVAL, "could not parse endpoint\n");
    goto error_out;
    }
// Check the number of MIPI CSI2 data lanes
    if (ep_cfg.bus.mipi_csi2.num_data_lanes != 2 &&
    ep_cfg.bus.mipi_csi2.num_data_lanes != 4) {
    dev_err_probe(dev, -EINVAL,
    "only 2 or 4 data lanes are currently supported\n");
    goto error_out;
    }
    imx219.lanes = ep_cfg.bus.mipi_csi2.num_data_lanes;
// Check the link frequency set in device tree
    switch (imx219.lanes) {
    case 2:
    ret = v4l2_link_freq_to_bitmap(dev,
    ep_cfg.link_frequencies,
    ep_cfg.nr_of_link_frequencies,
    imx219_link_freq_menu,
    ARRAY_SIZE(imx219_link_freq_menu),
    &link_freq_bitmap);
    break;
    case 4:
    ret = v4l2_link_freq_to_bitmap(dev,
    ep_cfg.link_frequencies,
    ep_cfg.nr_of_link_frequencies,
    imx219_link_freq_4lane_menu,
    ARRAY_SIZE(imx219_link_freq_4lane_menu),
    &link_freq_bitmap);
    if (!ret && (link_freq_bitmap & BIT(1))) {
    dev_warn(dev, "Link frequency of %d not supported, but has been incorrectly advertised previously\n",
    IMX219_DEFAULT_LINK_FREQ_4LANE_UNSUPPORTED);
    dev_warn(dev, "Using link frequency of %d\n",
    IMX219_DEFAULT_LINK_FREQ_4LANE);
    link_freq_bitmap |= BIT(0);
    }
    break;
    }
    if (ret || !(link_freq_bitmap & BIT(0))) {
    ret = -EINVAL;
    dev_err_probe(dev, -EINVAL,
    "Link frequency not supported: %lld\n",
    ep_cfg.link_frequencies[0]);
    }
    error_out:
    v4l2_fwnode_endpoint_free(&ep_cfg);
    fwnode_handle_put(endpoint);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx219_probe(client: *mut i2c_client) -> c_int {
    static int imx219_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct imx219 *imx219;
    int ret;
    imx219 = devm_kzalloc(&client.dev, sizeof(*imx219), GFP_KERNEL);
    if (!imx219)
    return -ENOMEM;
    v4l2_i2c_subdev_init(&imx219.sd, client, &imx219_subdev_ops);
    imx219.sd.internal_ops = &imx219_internal_ops;
// Check the hardware configuration in device tree
    if (imx219_check_hwcfg(dev, imx219))
    return -EINVAL;
    imx219.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(imx219.regmap))
    return dev_err_probe(dev, PTR_ERR(imx219.regmap),
    "failed to initialize CCI\n");
// Get system clock (xclk)
    imx219.xclk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(imx219.xclk))
    return dev_err_probe(dev, PTR_ERR(imx219.xclk),
    "failed to get xclk\n");
    imx219.xclk_freq = clk_get_rate(imx219.xclk);
    if (imx219.xclk_freq != IMX219_XCLK_FREQ)
    return dev_err_probe(dev, -EINVAL,
    "xclk frequency not supported: %d Hz\n",
    imx219.xclk_freq);
    ret = imx219_get_regulators(imx219);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get regulators\n");
// Request optional enable pin
    imx219.reset_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(imx219.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(imx219.reset_gpio),
    "failed to get reset gpio\n");
//
// The sensor must be powered for imx219_identify_module()
// to be able to read the CHIP_ID register
//
    ret = imx219_power_on(dev);
    if (ret)
    return ret;
    ret = imx219_identify_module(imx219);
    if (ret)
    goto error_power_off;
//
// Sensor doesn't enter LP-11 state upon power up until and unless
// streaming is started, so upon power up switch the modes to:
// streaming -> standby
//
    ret = cci_write(imx219.regmap, IMX219_REG_MODE_SELECT,
    IMX219_MODE_STREAMING, core::ptr::null_mut());
    if (ret < 0)
    goto error_power_off;
    usleep_range(100, 110);
// put sensor back to standby mode
    ret = cci_write(imx219.regmap, IMX219_REG_MODE_SELECT,
    IMX219_MODE_STANDBY, core::ptr::null_mut());
    if (ret < 0)
    goto error_power_off;
    usleep_range(100, 110);
    ret = imx219_init_controls(imx219);
    if (ret)
    goto error_power_off;
// Initialize subdev
    imx219.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    imx219.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
// Initialize source pad
    imx219.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&imx219.sd.entity, 1, &imx219.pad);
    if (ret) {
    dev_err_probe(dev, ret, "failed to init entity pads\n");
    goto error_handler_free;
    }
    imx219.sd.state_lock = imx219.ctrl_handler.lock;
    ret = v4l2_subdev_init_finalize(&imx219.sd);
    if (ret < 0) {
    dev_err_probe(dev, ret, "subdev init error\n");
    goto error_media_entity;
    }
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    ret = v4l2_async_register_subdev_sensor(&imx219.sd);
    if (ret < 0) {
    dev_err_probe(dev, ret,
    "failed to register sensor sub-device\n");
    goto error_subdev_cleanup;
    }
    pm_runtime_idle(dev);
    pm_runtime_set_autosuspend_delay(dev, 1000);
    pm_runtime_use_autosuspend(dev);
    return 0;
    error_subdev_cleanup:
    v4l2_subdev_cleanup(&imx219.sd);
    pm_runtime_disable(dev);
    pm_runtime_set_suspended(dev);
    error_media_entity:
    media_entity_cleanup(&imx219.sd.entity);
    error_handler_free:
    imx219_free_controls(imx219);
    error_power_off:
    imx219_power_off(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx219_remove(client: *mut i2c_client) {
    static void imx219_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct imx219 *imx219 = to_imx219(sd);
    v4l2_async_unregister_subdev(sd);
    v4l2_subdev_cleanup(sd);
    media_entity_cleanup(&sd.entity);
    imx219_free_controls(imx219);
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev)) {
    imx219_power_off(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    }
    }
    static const struct of_device_id imx219_dt_ids[] = {
    { .compatible = "sony,imx219" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx219_dt_ids);
    static const struct dev_pm_ops imx219_pm_ops = {
    SET_RUNTIME_PM_OPS(imx219_power_off, imx219_power_on, core::ptr::null_mut())
    };
    static struct i2c_driver imx219_i2c_driver = {
    .driver = {
    .name = "imx219",
    .of_match_table	= imx219_dt_ids,
    .pm = &imx219_pm_ops,
    },
    .probe = imx219_probe,
    .remove = imx219_remove,
    };
    module_i2c_driver(imx219_i2c_driver);
    MODULE_AUTHOR("Dave Stevenson <dave.stevenson@raspberrypi.com");
    MODULE_DESCRIPTION("Sony IMX219 sensor driver");
    MODULE_LICENSE("GPL v2");
