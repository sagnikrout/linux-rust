//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/imx214.c
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
// imx214.c - imx214 sensor driver
//
// Copyright 2018 Qtechnology A/S
//
// Ricardo Ribalda <ribalda@kernel.org>
//

// Chip ID

pub const IMX214_CHIP_ID: c_uint = 0x0214;

pub const IMX214_MODE_STANDBY: c_uint = 0x00;
pub const IMX214_MODE_STREAMING: c_uint = 0x01;

pub const IMX214_DEFAULT_LINK_FREQ: c_int = 600000000;
// Keep wrong link frequency for backward compatibility
pub const IMX214_DEFAULT_LINK_FREQ_LEGACY: c_int = 480000000;
pub const IMX214_FPS: c_int = 30;
// V-TIMING internal

pub const IMX214_VTS_MAX: c_uint = 0xffff;
pub const IMX214_VBLANK_MIN: c_int = 890;
// HBLANK control - read only
pub const IMX214_PPL_DEFAULT: c_int = 5008;
// Exposure control

pub const IMX214_EXPOSURE_OFFSET: c_int = 10;
pub const IMX214_EXPOSURE_MIN: c_int = 1;
pub const IMX214_EXPOSURE_STEP: c_int = 1;
pub const IMX214_EXPOSURE_DEFAULT: c_int = 3184;

// Analog gain control

pub const IMX214_ANA_GAIN_MIN: c_int = 0;
pub const IMX214_ANA_GAIN_MAX: c_int = 448;
pub const IMX214_ANA_GAIN_STEP: c_int = 1;
pub const IMX214_ANA_GAIN_DEFAULT: c_uint = 0x0;
// Digital gain control

pub const IMX214_DGTL_GAIN_MIN: c_uint = 0x0100;
pub const IMX214_DGTL_GAIN_MAX: c_uint = 0x0fff;
pub const IMX214_DGTL_GAIN_DEFAULT: c_uint = 0x0100;
pub const IMX214_DGTL_GAIN_STEP: c_int = 1;

pub const IMX214_CORR_FRAMES_TRANSMIT: c_int = 0;
pub const IMX214_CORR_FRAMES_MASK: c_int = 1;

pub const IMX214_CSI_DATA_FORMAT_RAW8: c_uint = 0x0808;
pub const IMX214_CSI_DATA_FORMAT_RAW10: c_uint = 0x0A0A;
pub const IMX214_CSI_DATA_FORMAT_COMP6: c_uint = 0x0A06;
pub const IMX214_CSI_DATA_FORMAT_COMP8: c_uint = 0x0A08;
pub const IMX214_BITS_PER_PIXEL_MASK: c_uint = 0xFF;

pub const IMX214_CSI_2_LANE_MODE: c_int = 1;
pub const IMX214_CSI_4_LANE_MODE: c_int = 3;

pub const IMX214_HDR_MODE_OFF: c_int = 0;
pub const IMX214_HDR_MODE_ON: c_int = 1;

pub const IMX214_HDR_RES_REDU_THROUGH: c_uint = 0x11;
pub const IMX214_HDR_RES_REDU_2_BINNING: c_uint = 0x22;
// PLL settings

pub const IMX214_PLL_SINGLE: c_int = 0;
pub const IMX214_PLL_DUAL: c_int = 1;

pub const IMX214_SCALE_NONE: c_int = 0;
pub const IMX214_SCALE_HORIZONTAL: c_int = 1;
pub const IMX214_SCALE_FULL: c_int = 2;

// Binning mode

pub const IMX214_BINNING_NONE: c_int = 0;
pub const IMX214_BINNING_ENABLE: c_int = 1;

pub const IMX214_BINNING_AVERAGE: c_uint = 0x00;
pub const IMX214_BINNING_SUMMED: c_uint = 0x01;
pub const IMX214_BINNING_BAYER: c_uint = 0x02;

pub const IMX214_SING_DEF_CORR_OFF: c_int = 0;
pub const IMX214_SING_DEF_CORR_ON: c_int = 1;
// AWB control

pub const IMX214_STATS_OUT_OFF: c_int = 0;
pub const IMX214_STATS_OUT_ON: c_int = 1;
// Chroma noise reduction

pub const IMX214_NML_NR_OFF: c_int = 0;
pub const IMX214_NML_NR_ON: c_int = 1;

pub const IMX214_EBD_NO: c_int = 0;
pub const IMX214_EBD_4_LINE: c_int = 4;

pub const IMX214_RG_STATS_LMT_10_BIT: c_uint = 0x03FF;
pub const IMX214_RG_STATS_LMT_14_BIT: c_uint = 0x3FFF;

// Test Pattern Control

pub const IMX214_TEST_PATTERN_DISABLE: c_int = 0;
pub const IMX214_TEST_PATTERN_SOLID_COLOR: c_int = 1;
pub const IMX214_TEST_PATTERN_COLOR_BARS: c_int = 2;
pub const IMX214_TEST_PATTERN_GREY_COLOR: c_int = 3;
pub const IMX214_TEST_PATTERN_PN9: c_int = 4;
// Test pattern colour components

pub const IMX214_TESTP_COLOUR_MIN: c_int = 0;
pub const IMX214_TESTP_COLOUR_MAX: c_uint = 0x03ff;
pub const IMX214_TESTP_COLOUR_STEP: c_int = 1;
// IMX214 native and active pixel array size

    static const char * const imx214_supply_name[] = {
    "vdda",
    "vddd",
    "vdddo",
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
    static const u32 imx214_mbus_formats[] = {
    MEDIA_BUS_FMT_SRGGB10_1X10,
    MEDIA_BUS_FMT_SGRBG10_1X10,
    MEDIA_BUS_FMT_SGBRG10_1X10,
    MEDIA_BUS_FMT_SBGGR10_1X10,
    };
    static const char * const imx214_test_pattern_menu[] = {
    "Disabled",
    "Color Bars",
    "Solid Color",
    "Grey Color Bars",
    "PN9"
    };
    static const int imx214_test_pattern_val[] = {
    IMX214_TEST_PATTERN_DISABLE,
    IMX214_TEST_PATTERN_COLOR_BARS,
    IMX214_TEST_PATTERN_SOLID_COLOR,
    IMX214_TEST_PATTERN_GREY_COLOR,
    IMX214_TEST_PATTERN_PN9,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx214 {
    pub dev: *mut device,
    pub xclk: *mut clk,
    pub regmap: *mut regmap,
    pub pll: ccs_pll,
    pub bus_cfg: v4l2_fwnode_endpoint,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub ctrls: v4l2_ctrl_handler,
    pub pixel_rate: *mut v4l2_ctrl,
    pub link_freq: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub unit_size: *mut v4l2_ctrl,
    struct {
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
}

    struct regulator_bulk_data	supplies[IMX214_NUM_SUPPLIES];
    struct gpio_desc *enable_gpio;
    };
// From imx214_mode_tbls.h
    static const struct cci_reg_sequence mode_4096x2304[] = {
    { IMX214_REG_HDR_MODE, IMX214_HDR_MODE_OFF },
    { IMX214_REG_HDR_RES_REDUCTION, IMX214_HDR_RES_REDU_THROUGH },
    { IMX214_REG_EXPOSURE_RATIO, 1 },
    { IMX214_REG_X_ADD_STA, 56 },
    { IMX214_REG_Y_ADD_STA, 408 },
    { IMX214_REG_X_ADD_END, 4151 },
    { IMX214_REG_Y_ADD_END, 2711 },
    { IMX214_REG_X_EVEN_INC, 1 },
    { IMX214_REG_X_ODD_INC, 1 },
    { IMX214_REG_Y_EVEN_INC, 1 },
    { IMX214_REG_Y_ODD_INC, 1 },
    { IMX214_REG_BINNING_MODE, IMX214_BINNING_NONE },
    { IMX214_REG_BINNING_TYPE, 0 },
    { IMX214_REG_BINNING_WEIGHTING, IMX214_BINNING_AVERAGE },
    { CCI_REG8(0x3000), 0x35 },
    { CCI_REG8(0x3054), 0x01 },
    { CCI_REG8(0x305C), 0x11 },
    { IMX214_REG_CSI_DATA_FORMAT, IMX214_CSI_DATA_FORMAT_RAW10 },
    { IMX214_REG_X_OUTPUT_SIZE, 4096 },
    { IMX214_REG_Y_OUTPUT_SIZE, 2304 },
    { IMX214_REG_SCALE_MODE, IMX214_SCALE_NONE },
    { IMX214_REG_SCALE_M, 2 },
    { IMX214_REG_DIG_CROP_X_OFFSET, 0 },
    { IMX214_REG_DIG_CROP_Y_OFFSET, 0 },
    { IMX214_REG_DIG_CROP_WIDTH, 4096 },
    { IMX214_REG_DIG_CROP_HEIGHT, 2304 },
    { CCI_REG8(0x3A03), 0x09 },
    { CCI_REG8(0x3A04), 0x50 },
    { CCI_REG8(0x3A05), 0x01 },
    { IMX214_REG_SING_DEF_CORR_EN, IMX214_SING_DEF_CORR_ON },
    { IMX214_REG_NML_NR_EN, IMX214_NML_NR_OFF },
    { CCI_REG8(0x30B4), 0x00 },
    { CCI_REG8(0x3A02), 0xFF },
    { CCI_REG8(0x3011), 0x00 },
    { IMX214_REG_STATS_OUT_EN, IMX214_STATS_OUT_ON },
    { IMX214_REG_SHORT_EXPOSURE, 500 },
    { CCI_REG8(0x4170), 0x00 },
    { CCI_REG8(0x4171), 0x10 },
    { CCI_REG8(0x4176), 0x00 },
    { CCI_REG8(0x4177), 0x3C },
    { CCI_REG8(0xAE20), 0x04 },
    { CCI_REG8(0xAE21), 0x5C },
    };
    static const struct cci_reg_sequence mode_1920x1080[] = {
    { IMX214_REG_HDR_MODE, IMX214_HDR_MODE_OFF },
    { IMX214_REG_HDR_RES_REDUCTION, IMX214_HDR_RES_REDU_THROUGH },
    { IMX214_REG_EXPOSURE_RATIO, 1 },
    { IMX214_REG_X_ADD_STA, 1144 },
    { IMX214_REG_Y_ADD_STA, 1020 },
    { IMX214_REG_X_ADD_END, 3063 },
    { IMX214_REG_Y_ADD_END, 2099 },
    { IMX214_REG_X_EVEN_INC, 1 },
    { IMX214_REG_X_ODD_INC, 1 },
    { IMX214_REG_Y_EVEN_INC, 1 },
    { IMX214_REG_Y_ODD_INC, 1 },
    { IMX214_REG_BINNING_MODE, IMX214_BINNING_NONE },
    { IMX214_REG_BINNING_TYPE, 0 },
    { IMX214_REG_BINNING_WEIGHTING, IMX214_BINNING_AVERAGE },
    { CCI_REG8(0x3000), 0x35 },
    { CCI_REG8(0x3054), 0x01 },
    { CCI_REG8(0x305C), 0x11 },
    { IMX214_REG_CSI_DATA_FORMAT, IMX214_CSI_DATA_FORMAT_RAW10 },
    { IMX214_REG_X_OUTPUT_SIZE, 1920 },
    { IMX214_REG_Y_OUTPUT_SIZE, 1080 },
    { IMX214_REG_SCALE_MODE, IMX214_SCALE_NONE },
    { IMX214_REG_SCALE_M, 2 },
    { IMX214_REG_DIG_CROP_X_OFFSET, 0 },
    { IMX214_REG_DIG_CROP_Y_OFFSET, 0 },
    { IMX214_REG_DIG_CROP_WIDTH, 1920 },
    { IMX214_REG_DIG_CROP_HEIGHT, 1080 },
    { CCI_REG8(0x3A03), 0x04 },
    { CCI_REG8(0x3A04), 0xF8 },
    { CCI_REG8(0x3A05), 0x02 },
    { IMX214_REG_SING_DEF_CORR_EN, IMX214_SING_DEF_CORR_ON },
    { IMX214_REG_NML_NR_EN, IMX214_NML_NR_OFF },
    { CCI_REG8(0x30B4), 0x00 },
    { CCI_REG8(0x3A02), 0xFF },
    { CCI_REG8(0x3011), 0x00 },
    { IMX214_REG_STATS_OUT_EN, IMX214_STATS_OUT_ON },
    { IMX214_REG_SHORT_EXPOSURE, 500 },
    { CCI_REG8(0x4170), 0x00 },
    { CCI_REG8(0x4171), 0x10 },
    { CCI_REG8(0x4176), 0x00 },
    { CCI_REG8(0x4177), 0x3C },
    { CCI_REG8(0xAE20), 0x04 },
    { CCI_REG8(0xAE21), 0x5C },
    };
    static const struct cci_reg_sequence mode_table_common[] = {
// software reset
// software standby settings
    { IMX214_REG_MODE_SELECT, IMX214_MODE_STANDBY },
// ATR setting
    { IMX214_REG_ATR_FAST_MOVE, 2 },
// global setting
// basic config
    { IMX214_REG_MASK_CORR_FRAMES, IMX214_CORR_FRAMES_MASK },
    { IMX214_REG_FAST_STANDBY_CTRL, 1 },
    { IMX214_REG_LINE_LENGTH_PCK, IMX214_PPL_DEFAULT },
    { CCI_REG8(0x4550), 0x02 },
    { CCI_REG8(0x4601), 0x00 },
    { CCI_REG8(0x4642), 0x05 },
    { CCI_REG8(0x6227), 0x11 },
    { CCI_REG8(0x6276), 0x00 },
    { CCI_REG8(0x900E), 0x06 },
    { CCI_REG8(0xA802), 0x90 },
    { CCI_REG8(0xA803), 0x11 },
    { CCI_REG8(0xA804), 0x62 },
    { CCI_REG8(0xA805), 0x77 },
    { CCI_REG8(0xA806), 0xAE },
    { CCI_REG8(0xA807), 0x34 },
    { CCI_REG8(0xA808), 0xAE },
    { CCI_REG8(0xA809), 0x35 },
    { CCI_REG8(0xA80A), 0x62 },
    { CCI_REG8(0xA80B), 0x83 },
    { CCI_REG8(0xAE33), 0x00 },
// analog setting
    { CCI_REG8(0x4174), 0x00 },
    { CCI_REG8(0x4175), 0x11 },
    { CCI_REG8(0x4612), 0x29 },
    { CCI_REG8(0x461B), 0x12 },
    { CCI_REG8(0x461F), 0x06 },
    { CCI_REG8(0x4635), 0x07 },
    { CCI_REG8(0x4637), 0x30 },
    { CCI_REG8(0x463F), 0x18 },
    { CCI_REG8(0x4641), 0x0D },
    { CCI_REG8(0x465B), 0x12 },
    { CCI_REG8(0x465F), 0x11 },
    { CCI_REG8(0x4663), 0x11 },
    { CCI_REG8(0x4667), 0x0F },
    { CCI_REG8(0x466F), 0x0F },
    { CCI_REG8(0x470E), 0x09 },
    { CCI_REG8(0x4909), 0xAB },
    { CCI_REG8(0x490B), 0x95 },
    { CCI_REG8(0x4915), 0x5D },
    { CCI_REG8(0x4A5F), 0xFF },
    { CCI_REG8(0x4A61), 0xFF },
    { CCI_REG8(0x4A73), 0x62 },
    { CCI_REG8(0x4A85), 0x00 },
    { CCI_REG8(0x4A87), 0xFF },
// embedded data
    { IMX214_REG_EBD_SIZE_V, IMX214_EBD_4_LINE },
    { CCI_REG8(0x583C), 0x04 },
    { CCI_REG8(0x620E), 0x04 },
    { CCI_REG8(0x6EB2), 0x01 },
    { CCI_REG8(0x6EB3), 0x00 },
    { IMX214_REG_ATR_FAST_MOVE, 2 },
// imagequality
// HDR setting
    { IMX214_REG_RMSC_NR_MODE, 0x07 },
    { IMX214_REG_RG_STATS_LMT, IMX214_RG_STATS_LMT_14_BIT },
    { CCI_REG8(0x9344), 0x03 },
    { CCI_REG8(0x9706), 0x10 },
    { CCI_REG8(0x9707), 0x03 },
    { CCI_REG8(0x9708), 0x03 },
    { CCI_REG8(0x9E04), 0x01 },
    { CCI_REG8(0x9E05), 0x00 },
    { CCI_REG8(0x9E0C), 0x01 },
    { CCI_REG8(0x9E0D), 0x02 },
    { CCI_REG8(0x9E24), 0x00 },
    { CCI_REG8(0x9E25), 0x8C },
    { CCI_REG8(0x9E26), 0x00 },
    { CCI_REG8(0x9E27), 0x94 },
    { CCI_REG8(0x9E28), 0x00 },
    { CCI_REG8(0x9E29), 0x96 },
// CNR parameter setting
    { CCI_REG8(0x69DB), 0x01 },
// Moire reduction
    { CCI_REG8(0x6957), 0x01 },
// image enhancement
    { CCI_REG8(0x6987), 0x17 },
    { CCI_REG8(0x698A), 0x03 },
    { CCI_REG8(0x698B), 0x03 },
// white balanace
    { IMX214_REG_ABS_GAIN_GREENR, 0x0100 },
    { IMX214_REG_ABS_GAIN_RED, 0x0100 },
    { IMX214_REG_ABS_GAIN_BLUE, 0x0100 },
    { IMX214_REG_ABS_GAIN_GREENB, 0x0100 },
// ATR setting
    { CCI_REG8(0x6E50), 0x00 },
    { CCI_REG8(0x6E51), 0x32 },
    { CCI_REG8(0x9340), 0x00 },
    { CCI_REG8(0x9341), 0x3C },
    { CCI_REG8(0x9342), 0x03 },
    { CCI_REG8(0x9343), 0xFF },
    };
//
// Declare modes in order, from biggest
// to smallest height.
//
    static const struct imx214_mode {
    u32 width;
    u32 height;
// V-timing
    unsigned int vts_def;
    unsigned int num_of_regs;
    const struct cci_reg_sequence *reg_table;
    } imx214_modes[] = {
    {
    .width = 4096,
    .height = 2304,
    .vts_def = 3194,
    .num_of_regs = ARRAY_SIZE(mode_4096x2304),
    .reg_table = mode_4096x2304,
    },
    {
    .width = 1920,
    .height = 1080,
    .vts_def = 3194,
    .num_of_regs = ARRAY_SIZE(mode_1920x1080),
    .reg_table = mode_1920x1080,
    },
    };
    static inline struct imx214 *to_imx214(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct imx214, sd);
    }
#[no_mangle]
unsafe extern "C" fn imx214_power_on(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx214_power_on(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct imx214 *imx214 = to_imx214(sd);
    int ret;
    ret = regulator_bulk_enable(IMX214_NUM_SUPPLIES, imx214.supplies);
    if (ret < 0) {
    dev_err(imx214.dev, "failed to enable regulators: %d\n", ret);
    return ret;
    }
    usleep_range(2000, 3000);
    ret = clk_prepare_enable(imx214.xclk);
    if (ret < 0) {
    regulator_bulk_disable(IMX214_NUM_SUPPLIES, imx214.supplies);
    dev_err(imx214.dev, "clk prepare enable failed\n");
    return ret;
    }
    gpiod_set_value_cansleep(imx214.enable_gpio, 1);
    usleep_range(12000, 15000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx214_power_off(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx214_power_off(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct imx214 *imx214 = to_imx214(sd);
    gpiod_set_value_cansleep(imx214.enable_gpio, 0);
    clk_disable_unprepare(imx214.xclk);
    regulator_bulk_disable(IMX214_NUM_SUPPLIES, imx214.supplies);
    usleep_range(10, 20);
    return 0;
    }
// Get bayer order based on flip setting.
#[no_mangle]
unsafe extern "C" fn imx214_get_format_code(imx214: *mut imx214) -> u32 {
    static u32 imx214_get_format_code(struct imx214 *imx214)
    {
    unsigned int i;
    i = (imx214.vflip.val ? 2 : 0) | (imx214.hflip.val ? 1 : 0);
    return imx214_mbus_formats[i];
    }
    static void imx214_update_pad_format(struct imx214 *imx214,
    const struct imx214_mode *mode,
    struct v4l2_mbus_framefmt *fmt, u32 code)
    {
    fmt.code = imx214_get_format_code(imx214);
    fmt.width = mode.width;
    fmt.height = mode.height;
    fmt.field = V4L2_FIELD_NONE;
    fmt.colorspace = V4L2_COLORSPACE_SRGB;
    fmt.ycbcr_enc = V4L2_MAP_YCBCR_ENC_DEFAULT(fmt.colorspace);
    fmt.quantization = V4L2_MAP_QUANTIZATION_DEFAULT(true,
    fmt.colorspace,
    fmt.ycbcr_enc);
    fmt.xfer_func = V4L2_MAP_XFER_FUNC_DEFAULT(fmt.colorspace);
    }
    static int imx214_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    struct imx214 *imx214 = to_imx214(sd);
    if (code.index >= (ARRAY_SIZE(imx214_mbus_formats) / 4))
    return -EINVAL;
    code.code = imx214_get_format_code(imx214);
    return 0;
    }
    static int imx214_enum_frame_size(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    struct imx214 *imx214 = to_imx214(subdev);
    u32 code;
    code = imx214_get_format_code(imx214);
    if (fse.code != code)
    return -EINVAL;
    if (fse.index >= ARRAY_SIZE(imx214_modes))
    return -EINVAL;
    fse.min_width = fse.max_width = imx214_modes[fse.index].width;
    fse.min_height = fse.max_height = imx214_modes[fse.index].height;
    return 0;
    }

    static int imx214_s_register(struct v4l2_subdev *subdev,
    const struct v4l2_dbg_register *reg)
    {
    struct imx214 *imx214 = container_of(subdev, struct imx214, sd);
    return regmap_write(imx214.regmap, reg.reg, reg.val);
    }
    static int imx214_g_register(struct v4l2_subdev *subdev,
    struct v4l2_dbg_register *reg)
    {
    struct imx214 *imx214 = container_of(subdev, struct imx214, sd);
    unsigned int aux;
    int ret;
    reg.size = 1;
    ret = regmap_read(imx214.regmap, reg.reg, &aux);
    reg.val = aux;
    return ret;
    }

    static const struct v4l2_subdev_core_ops imx214_core_ops = {

    .g_register = imx214_g_register,
    .s_register = imx214_s_register,

    };
    static int imx214_set_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct imx214 *imx214 = to_imx214(sd);
    struct v4l2_mbus_framefmt *__format;
    struct v4l2_rect *__crop;
    const struct imx214_mode *mode;
    mode = v4l2_find_nearest_size(imx214_modes,
    ARRAY_SIZE(imx214_modes), width, height,
    format.format.width,
    format.format.height);
    imx214_update_pad_format(imx214, mode, &format.format,
    format.format.code);
    __format = v4l2_subdev_state_get_format(sd_state, 0);
// __format = format->format;
    __crop = v4l2_subdev_state_get_crop(sd_state, 0);
    __crop.width = mode.width;
    __crop.height = mode.height;
    if (format.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    int exposure_max;
    int exposure_def;
    int hblank;
// Update blank limits
    __v4l2_ctrl_modify_range(imx214.vblank, IMX214_VBLANK_MIN,
    IMX214_VTS_MAX - mode.height, 2,
    mode.vts_def - mode.height);
// Update max exposure while meeting expected vblanking
    exposure_max = mode.vts_def - IMX214_EXPOSURE_OFFSET;
    exposure_def = min(exposure_max, IMX214_EXPOSURE_DEFAULT);
    __v4l2_ctrl_modify_range(imx214.exposure,
    imx214.exposure.minimum,
    exposure_max, imx214.exposure.step,
    exposure_def);
//
// Currently PPL is fixed to IMX214_PPL_DEFAULT, so hblank
// depends on mode->width only, and is not changeable in any
// way other than changing the mode.
//
    hblank = IMX214_PPL_DEFAULT - mode.width;
    __v4l2_ctrl_modify_range(imx214.hblank, hblank, hblank, 1,
    hblank);
    }
    return 0;
    }
    static int imx214_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    sel.r = *v4l2_subdev_state_get_crop(sd_state, 0);
    return 0;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = IMX214_NATIVE_WIDTH;
    sel.r.height = IMX214_NATIVE_HEIGHT;
    return 0;
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = IMX214_PIXEL_ARRAY_TOP;
    sel.r.left = IMX214_PIXEL_ARRAY_LEFT;
    sel.r.width = IMX214_PIXEL_ARRAY_WIDTH;
    sel.r.height = IMX214_PIXEL_ARRAY_HEIGHT;
    return 0;
    }
    return -EINVAL;
    }
    static int imx214_entity_init_state(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state)
    {
    let mut fmt: v4l2_subdev_format = { };
    fmt.which = sd_state ? V4L2_SUBDEV_FORMAT_TRY : V4L2_SUBDEV_FORMAT_ACTIVE;
    fmt.format.code = MEDIA_BUS_FMT_SRGGB10_1X10;
    fmt.format.width = imx214_modes[0].width;
    fmt.format.height = imx214_modes[0].height;
    imx214_set_format(subdev, sd_state, &fmt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx214_configure_pll(imx214: *mut imx214) -> c_int {
    static int imx214_configure_pll(struct imx214 *imx214)
    {
    let mut ret: c_int = 0;
    cci_write(imx214.regmap, IMX214_REG_VTPXCK_DIV,
    imx214.pll.vt_bk.pix_clk_div, &ret);
    cci_write(imx214.regmap, IMX214_REG_VTSYCK_DIV,
    imx214.pll.vt_bk.sys_clk_div, &ret);
    cci_write(imx214.regmap, IMX214_REG_PREPLLCK_VT_DIV,
    imx214.pll.vt_fr.pre_pll_clk_div, &ret);
    cci_write(imx214.regmap, IMX214_REG_PLL_VT_MPY,
    imx214.pll.vt_fr.pll_multiplier, &ret);
    cci_write(imx214.regmap, IMX214_REG_OPPXCK_DIV,
    imx214.pll.op_bk.pix_clk_div, &ret);
    cci_write(imx214.regmap, IMX214_REG_OPSYCK_DIV,
    imx214.pll.op_bk.sys_clk_div, &ret);
    cci_write(imx214.regmap, IMX214_REG_PLL_MULT_DRIV,
    IMX214_PLL_SINGLE, &ret);
    cci_write(imx214.regmap, IMX214_REG_EXCK_FREQ,
    IMX214_EXCK_FREQ(imx214.pll.ext_clk_freq_hz / 1000000), &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_update_digital_gain(imx214: *mut imx214, val: u32) -> c_int {
    static int imx214_update_digital_gain(struct imx214 *imx214, u32 val)
    {
    let mut ret: c_int = 0;
    cci_write(imx214.regmap, IMX214_REG_DIG_GAIN_GREENR, val, &ret);
    cci_write(imx214.regmap, IMX214_REG_DIG_GAIN_RED, val, &ret);
    cci_write(imx214.regmap, IMX214_REG_DIG_GAIN_BLUE, val, &ret);
    cci_write(imx214.regmap, IMX214_REG_DIG_GAIN_GREENB, val, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int imx214_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct imx214 *imx214 = container_of(ctrl.handler,
    struct imx214, ctrls);
    const struct v4l2_mbus_framefmt *format = core::ptr::null_mut();
    struct v4l2_subdev_state *state;
    let mut ret: c_int = 0;
    if (ctrl.id == V4L2_CID_VBLANK) {
    int exposure_max, exposure_def;
    state = v4l2_subdev_get_locked_active_state(&imx214.sd);
    format = v4l2_subdev_state_get_format(state, 0);
// Update max exposure while meeting expected vblanking
    exposure_max =
    format.height + ctrl.val - IMX214_EXPOSURE_OFFSET;
    exposure_def = min(exposure_max, IMX214_EXPOSURE_DEFAULT);
    __v4l2_ctrl_modify_range(imx214.exposure,
    imx214.exposure.minimum,
    exposure_max, imx214.exposure.step,
    exposure_def);
    }
//
// Applying V4L2 control value only happens
// when power is up for streaming
//
    if (!pm_runtime_get_if_in_use(imx214.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_ANALOGUE_GAIN:
    cci_write(imx214.regmap, IMX214_REG_ANALOG_GAIN,
    ctrl.val, &ret);
    cci_write(imx214.regmap, IMX214_REG_SHORT_ANALOG_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    ret = imx214_update_digital_gain(imx214, ctrl.val);
    break;
    case V4L2_CID_EXPOSURE:
    cci_write(imx214.regmap, IMX214_REG_EXPOSURE, ctrl.val, &ret);
    break;
    case V4L2_CID_HFLIP:
    case V4L2_CID_VFLIP:
    cci_write(imx214.regmap, IMX214_REG_ORIENTATION,
    imx214.hflip.val | imx214.vflip.val << 1, &ret);
    break;
    case V4L2_CID_VBLANK:
    cci_write(imx214.regmap, IMX214_REG_FRM_LENGTH_LINES,
    format.height + ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN:
    cci_write(imx214.regmap, IMX214_REG_TEST_PATTERN,
    imx214_test_pattern_val[ctrl.val], &ret);
    break;
    case V4L2_CID_TEST_PATTERN_RED:
    cci_write(imx214.regmap, IMX214_REG_TESTP_RED,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_GREENR:
    cci_write(imx214.regmap, IMX214_REG_TESTP_GREENR,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_BLUE:
    cci_write(imx214.regmap, IMX214_REG_TESTP_BLUE,
    ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN_GREENB:
    cci_write(imx214.regmap, IMX214_REG_TESTP_GREENB,
    ctrl.val, &ret);
    break;
    default:
    ret = -EINVAL;
    }
    pm_runtime_put(imx214.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops imx214_ctrl_ops = {
    .s_ctrl = imx214_set_ctrl,
    };
    static int imx214_pll_calculate(struct imx214 *imx214, struct ccs_pll *pll,
    unsigned int link_freq)
    {
    struct ccs_pll_limits limits = {
    .min_ext_clk_freq_hz = 6000000,
    .max_ext_clk_freq_hz = 27000000,
    .vt_fr = {
    .min_pre_pll_clk_div = 1,
    .max_pre_pll_clk_div = 15,
// Value is educated guess as we don't have a spec
    .min_pll_ip_clk_freq_hz = 6000000,
// Value is educated guess as we don't have a spec
    .max_pll_ip_clk_freq_hz = 12000000,
    .min_pll_multiplier = 12,
    .max_pll_multiplier = 1200,
    .min_pll_op_clk_freq_hz = 338000000,
    .max_pll_op_clk_freq_hz = 1200000000,
    },
    .vt_bk = {
    .min_sys_clk_div = 2,
    .max_sys_clk_div = 4,
    .min_pix_clk_div = 5,
    .max_pix_clk_div = 10,
    .min_pix_clk_freq_hz = 30000000,
    .max_pix_clk_freq_hz = 120000000,
    },
    .op_bk = {
    .min_sys_clk_div = 1,
    .max_sys_clk_div = 2,
    .min_pix_clk_div = 6,
    .max_pix_clk_div = 10,
    .min_pix_clk_freq_hz = 30000000,
    .max_pix_clk_freq_hz = 120000000,
    },
    .min_line_length_pck_bin = IMX214_PPL_DEFAULT,
    .min_line_length_pck = IMX214_PPL_DEFAULT,
    };
    let mut num_lanes: c_uint = imx214.bus_cfg.bus.mipi_csi2.num_data_lanes;
//
// There are no documented constraints on the sys clock frequency, for
// either branch. Recover them based on the PLL output clock frequency
// and sys_clk_div limits on one hand, and the pix clock frequency and
// the pix_clk_div limits on the other hand.
//
    limits.vt_bk.min_sys_clk_freq_hz =
    max(limits.vt_fr.min_pll_op_clk_freq_hz / limits.vt_bk.max_sys_clk_div,
    limits.vt_bk.min_pix_clk_freq_hz * limits.vt_bk.min_pix_clk_div);
    limits.vt_bk.max_sys_clk_freq_hz =
    min(limits.vt_fr.max_pll_op_clk_freq_hz / limits.vt_bk.min_sys_clk_div,
    limits.vt_bk.max_pix_clk_freq_hz * limits.vt_bk.max_pix_clk_div);
    limits.op_bk.min_sys_clk_freq_hz =
    max(limits.vt_fr.min_pll_op_clk_freq_hz / limits.op_bk.max_sys_clk_div,
    limits.op_bk.min_pix_clk_freq_hz * limits.op_bk.min_pix_clk_div);
    limits.op_bk.max_sys_clk_freq_hz =
    min(limits.vt_fr.max_pll_op_clk_freq_hz / limits.op_bk.min_sys_clk_div,
    limits.op_bk.max_pix_clk_freq_hz * limits.op_bk.max_pix_clk_div);
    memset(pll, 0, sizeof(*pll));
    pll.bus_type = CCS_PLL_BUS_TYPE_CSI2_DPHY;
    pll.op_lanes = num_lanes;
    pll.vt_lanes = num_lanes;
    pll.csi2.lanes = num_lanes;
    pll.binning_horizontal = 1;
    pll.binning_vertical = 1;
    pll.scale_m = 1;
    pll.scale_n = 1;
    pll.bits_per_pixel =
    IMX214_CSI_DATA_FORMAT_RAW10 & IMX214_BITS_PER_PIXEL_MASK;
    pll.flags = CCS_PLL_FLAG_LANE_SPEED_MODEL;
    pll.link_freq = link_freq;
    pll.ext_clk_freq_hz = clk_get_rate(imx214.xclk);
    return ccs_pll_calculate(imx214.dev, &limits, pll);
    }
#[no_mangle]
unsafe extern "C" fn imx214_pll_update(imx214: *mut imx214) -> c_int {
    static int imx214_pll_update(struct imx214 *imx214)
    {
    u64 link_freq;
    int ret;
    link_freq = imx214.bus_cfg.link_frequencies[imx214.link_freq.val];
    ret = imx214_pll_calculate(imx214, &imx214.pll, link_freq);
    if (ret) {
    dev_err(imx214.dev, "PLL calculations failed: %d\n", ret);
    return ret;
    }
    ret = v4l2_ctrl_s_ctrl_int64(imx214.pixel_rate,
    imx214.pll.pixel_rate_pixel_array);
    if (ret) {
    dev_err(imx214.dev, "failed to set pixel rate\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx214_ctrls_init(imx214: *mut imx214) -> c_int {
    static int imx214_ctrls_init(struct imx214 *imx214)
    {
    static const struct v4l2_area unit_size = {
    .width = 1120,
    .height = 1120,
    };
    const struct imx214_mode *mode = &imx214_modes[0];
    struct v4l2_fwnode_device_properties props;
    struct v4l2_ctrl_handler *ctrl_hdlr;
    int exposure_max, exposure_def;
    int hblank;
    int i, ret;
    ret = v4l2_fwnode_device_parse(imx214.dev, &props);
    if (ret < 0)
    return ret;
    ctrl_hdlr = &imx214.ctrls;
    ret = v4l2_ctrl_handler_init(&imx214.ctrls, 13);
    if (ret)
    return ret;
    imx214.pixel_rate =
    v4l2_ctrl_new_std(ctrl_hdlr, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE, 1,
    INT_MAX, 1, 1);
    imx214.link_freq = v4l2_ctrl_new_int_menu(ctrl_hdlr, core::ptr::null_mut(),
    V4L2_CID_LINK_FREQ,
    imx214.bus_cfg.nr_of_link_frequencies - 1,
    0, imx214.bus_cfg.link_frequencies);
//
// WARNING!
// Values obtained reverse engineering blobs and/or devices.
// Ranges and functionality might be wrong.
//
// Sony, please release some register set documentation for the
// device.
//
// Yours sincerely, Ricardo.
//
// Initial vblank/hblank/exposure parameters based on current mode
    imx214.vblank = v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_VBLANK, IMX214_VBLANK_MIN,
    IMX214_VTS_MAX - mode.height, 2,
    mode.vts_def - mode.height);
    hblank = IMX214_PPL_DEFAULT - mode.width;
    imx214.hblank = v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_HBLANK, hblank, hblank,
    1, hblank);
    exposure_max = mode.vts_def - IMX214_EXPOSURE_OFFSET;
    exposure_def = min(exposure_max, IMX214_EXPOSURE_DEFAULT);
    imx214.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_EXPOSURE,
    IMX214_EXPOSURE_MIN,
    exposure_max,
    IMX214_EXPOSURE_STEP,
    exposure_def);
    v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops, V4L2_CID_ANALOGUE_GAIN,
    IMX214_ANA_GAIN_MIN, IMX214_ANA_GAIN_MAX,
    IMX214_ANA_GAIN_STEP, IMX214_ANA_GAIN_DEFAULT);
    v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops, V4L2_CID_DIGITAL_GAIN,
    IMX214_DGTL_GAIN_MIN, IMX214_DGTL_GAIN_MAX,
    IMX214_DGTL_GAIN_STEP, IMX214_DGTL_GAIN_DEFAULT);
    imx214.hflip = v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    imx214.vflip = v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    v4l2_ctrl_cluster(2, &imx214.hflip);
    v4l2_ctrl_new_std_menu_items(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(imx214_test_pattern_menu) - 1,
    0, 0, imx214_test_pattern_menu);
    for (i = 0; i < 4; i++) {
//
// The assumption is that
// V4L2_CID_TEST_PATTERN_GREENR == V4L2_CID_TEST_PATTERN_RED + 1
// V4L2_CID_TEST_PATTERN_BLUE   == V4L2_CID_TEST_PATTERN_RED + 2
// V4L2_CID_TEST_PATTERN_GREENB == V4L2_CID_TEST_PATTERN_RED + 3
//
    v4l2_ctrl_new_std(ctrl_hdlr, &imx214_ctrl_ops,
    V4L2_CID_TEST_PATTERN_RED + i,
    IMX214_TESTP_COLOUR_MIN,
    IMX214_TESTP_COLOUR_MAX,
    IMX214_TESTP_COLOUR_STEP,
    IMX214_TESTP_COLOUR_MAX);
// The "Solid color" pattern is white by default
    }
    imx214.unit_size = v4l2_ctrl_new_std_compound(ctrl_hdlr,
    core::ptr::null_mut(),
    V4L2_CID_UNIT_CELL_SIZE,
    v4l2_ctrl_ptr_create((void *)&unit_size),
    v4l2_ctrl_ptr_create(core::ptr::null_mut()),
    v4l2_ctrl_ptr_create(core::ptr::null_mut()));
    v4l2_ctrl_new_fwnode_properties(ctrl_hdlr, &imx214_ctrl_ops, &props);
    ret = ctrl_hdlr.error;
    if (ret) {
    v4l2_ctrl_handler_free(ctrl_hdlr);
    dev_err(imx214.dev, "failed to add controls: %d\n", ret);
    return ret;
    }
// Now that the controls have been properly created, set their flags.
    imx214.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    imx214.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    imx214.hflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    imx214.vflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    ret = imx214_pll_update(imx214);
    if (ret < 0) {
    v4l2_ctrl_handler_free(ctrl_hdlr);
    dev_err(imx214.dev, "failed to update PLL\n");
    return ret;
    }
    imx214.sd.ctrl_handler = ctrl_hdlr;
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn imx214_start_streaming(imx214: *mut imx214) -> c_int {
    static int imx214_start_streaming(struct imx214 *imx214)
    {
    const struct v4l2_mbus_framefmt *fmt;
    struct v4l2_subdev_state *state;
    const struct imx214_mode *mode;
    int bit_rate_mbps;
    int ret;
    ret = cci_multi_reg_write(imx214.regmap, mode_table_common,
    ARRAY_SIZE(mode_table_common), core::ptr::null_mut());
    if (ret < 0) {
    dev_err(imx214.dev, "could not sent common table %d\n", ret);
    return ret;
    }
    ret = imx214_configure_pll(imx214);
    if (ret) {
    dev_err(imx214.dev, "failed to configure PLL: %d\n", ret);
    return ret;
    }
    bit_rate_mbps = imx214.pll.pixel_rate_csi / 1000000
// imx214->pll.bits_per_pixel;
    ret = cci_write(imx214.regmap, IMX214_REG_REQ_LINK_BIT_RATE,
    IMX214_LINK_BIT_RATE_MBPS(bit_rate_mbps), core::ptr::null_mut());
    if (ret) {
    dev_err(imx214.dev, "failed to configure link bit rate\n");
    return ret;
    }
    ret = cci_write(imx214.regmap, IMX214_REG_CSI_LANE_MODE,
    IMX214_CSI_4_LANE_MODE, core::ptr::null_mut());
    if (ret) {
    dev_err(imx214.dev, "failed to configure lanes\n");
    return ret;
    }
    state = v4l2_subdev_get_locked_active_state(&imx214.sd);
    fmt = v4l2_subdev_state_get_format(state, 0);
    mode = v4l2_find_nearest_size(imx214_modes, ARRAY_SIZE(imx214_modes),
    width, height, fmt.width, fmt.height);
    ret = cci_multi_reg_write(imx214.regmap, mode.reg_table,
    mode.num_of_regs, core::ptr::null_mut());
    if (ret < 0) {
    dev_err(imx214.dev, "could not sent mode table %d\n", ret);
    return ret;
    }
    usleep_range(10000, 10500);
    cci_write(imx214.regmap, IMX214_REG_TEMP_SENSOR_CONTROL, 0x01, core::ptr::null_mut());
    ret = __v4l2_ctrl_handler_setup(&imx214.ctrls);
    if (ret < 0) {
    dev_err(imx214.dev, "could not sync v4l2 controls\n");
    return ret;
    }
    ret = cci_write(imx214.regmap, IMX214_REG_MODE_SELECT,
    IMX214_MODE_STREAMING, core::ptr::null_mut());
    if (ret < 0)
    dev_err(imx214.dev, "could not sent start table %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_stop_streaming(imx214: *mut imx214) -> c_int {
    static int imx214_stop_streaming(struct imx214 *imx214)
    {
    int ret;
    ret = cci_write(imx214.regmap, IMX214_REG_MODE_SELECT,
    IMX214_MODE_STANDBY, core::ptr::null_mut());
    if (ret < 0)
    dev_err(imx214.dev, "could not sent stop table %d\n",	ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int imx214_s_stream(struct v4l2_subdev *subdev, int enable)
    {
    struct imx214 *imx214 = to_imx214(subdev);
    struct v4l2_subdev_state *state;
    int ret;
    if (enable) {
    ret = pm_runtime_resume_and_get(imx214.dev);
    if (ret < 0)
    return ret;
    state = v4l2_subdev_lock_and_get_active_state(subdev);
    ret = imx214_start_streaming(imx214);
    v4l2_subdev_unlock_state(state);
    if (ret < 0)
    goto err_rpm_put;
    } else {
    ret = imx214_stop_streaming(imx214);
    if (ret < 0)
    goto err_rpm_put;
    pm_runtime_put(imx214.dev);
    }
    return 0;
    err_rpm_put:
    pm_runtime_put(imx214.dev);
    return ret;
    }
    static int imx214_get_frame_interval(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fival)
    {
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fival.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    fival.interval.numerator = 1;
    fival.interval.denominator = IMX214_FPS;
    return 0;
    }
//
// Raw sensors should be using the VBLANK and HBLANK controls to determine
// the frame rate. However this driver was initially added using the
// [S|G|ENUM]_FRAME_INTERVAL ioctls with a fixed rate of 30fps.
// Retain the frame_interval ops for backwards compatibility, but they do
// nothing.
//
    static int imx214_enum_frame_interval(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval_enum *fie)
    {
    struct imx214 *imx214 = to_imx214(subdev);
    const struct imx214_mode *mode;
    dev_warn_once(imx214.dev, "frame_interval functions return an unreliable value for compatibility reasons. Use the VBLANK and HBLANK controls to determine the correct frame rate.\n");
    if (fie.index != 0)
    return -EINVAL;
    mode = v4l2_find_nearest_size(imx214_modes,
    ARRAY_SIZE(imx214_modes), width, height,
    fie.width, fie.height);
    fie.code = imx214_get_format_code(imx214);
    fie.width = mode.width;
    fie.height = mode.height;
    fie.interval.numerator = 1;
    fie.interval.denominator = IMX214_FPS;
    return 0;
    }
    static const struct v4l2_subdev_video_ops imx214_video_ops = {
    .s_stream = imx214_s_stream,
    };
    static const struct v4l2_subdev_pad_ops imx214_subdev_pad_ops = {
    .enum_mbus_code = imx214_enum_mbus_code,
    .enum_frame_size = imx214_enum_frame_size,
    .enum_frame_interval = imx214_enum_frame_interval,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = imx214_set_format,
    .get_selection = imx214_get_selection,
    .get_frame_interval = imx214_get_frame_interval,
    .set_frame_interval = imx214_get_frame_interval,
    };
    static const struct v4l2_subdev_ops imx214_subdev_ops = {
    .core = &imx214_core_ops,
    .video = &imx214_video_ops,
    .pad = &imx214_subdev_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops imx214_internal_ops = {
    .init_state = imx214_entity_init_state,
    };
#[no_mangle]
unsafe extern "C" fn imx214_get_regulators(dev: *mut device, imx214: *mut imx214) -> c_int {
    static int imx214_get_regulators(struct device *dev, struct imx214 *imx214)
    {
    unsigned int i;
    for (i = 0; i < IMX214_NUM_SUPPLIES; i++)
    imx214.supplies[i].supply = imx214_supply_name[i];
    return devm_regulator_bulk_get(dev, IMX214_NUM_SUPPLIES,
    imx214.supplies);
    }
// Verify chip ID
#[no_mangle]
unsafe extern "C" fn imx214_identify_module(imx214: *mut imx214) -> c_int {
    static int imx214_identify_module(struct imx214 *imx214)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&imx214.sd);
    int ret;
    u64 val;
    ret = cci_read(imx214.regmap, IMX214_REG_CHIP_ID, &val, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "failed to read chip id %x\n",
    IMX214_CHIP_ID);
    if (val != IMX214_CHIP_ID)
    return dev_err_probe(&client.dev, -EIO,
    "chip id mismatch: %x!=%llx\n",
    IMX214_CHIP_ID, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx214_parse_fwnode(imx214: *mut imx214) -> c_int {
    static int imx214_parse_fwnode(struct imx214 *imx214)
    {
    struct fwnode_handle *endpoint __free(fwnode_handle) = core::ptr::null_mut();
    struct v4l2_fwnode_endpoint *bus_cfg = &imx214.bus_cfg;
    struct device *dev = imx214.dev;
    unsigned int i;
    int ret;
    endpoint = fwnode_graph_get_next_endpoint(dev_fwnode(dev), core::ptr::null_mut());
    if (!endpoint)
    return dev_err_probe(dev, -EINVAL, "endpoint node not found\n");
    bus_cfg.bus_type = V4L2_MBUS_CSI2_DPHY;
    ret = v4l2_fwnode_endpoint_alloc_parse(endpoint, bus_cfg);
    if (ret)
    return dev_err_probe(dev, ret, "parsing endpoint node failed\n");
// Check the number of MIPI CSI2 data lanes
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != 4) {
    ret = dev_err_probe(dev, -EINVAL,
    "only 4 data lanes are currently supported\n");
    goto error;
    }
    if (bus_cfg.nr_of_link_frequencies != 1)
    dev_warn(dev, "Only one link-frequency supported, please review your DT. Continuing anyway\n");
    for (i = 0; i < bus_cfg.nr_of_link_frequencies; i++) {
    let mut freq: u64 = bus_cfg.link_frequencies[i];
    struct ccs_pll pll;
    if (freq == IMX214_DEFAULT_LINK_FREQ_LEGACY) {
    dev_warn(dev,
    "link-frequencies %d not supported, please review your DT. Continuing anyway\n",
    IMX214_DEFAULT_LINK_FREQ);
    freq = IMX214_DEFAULT_LINK_FREQ;
    bus_cfg.link_frequencies[i] = freq;
    }
    if (!imx214_pll_calculate(imx214, &pll, freq))
    break;
    }
    if (i == bus_cfg.nr_of_link_frequencies)
    ret = dev_err_probe(dev, -EINVAL,
    "link-frequencies %lld not supported, please review your DT\n",
    bus_cfg.nr_of_link_frequencies ?
    bus_cfg.link_frequencies[0] : 0);
    return 0;
    error:
    v4l2_fwnode_endpoint_free(&imx214.bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_probe(client: *mut i2c_client) -> c_int {
    static int imx214_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct imx214 *imx214;
    int ret;
    imx214 = devm_kzalloc(dev, sizeof(*imx214), GFP_KERNEL);
    if (!imx214)
    return -ENOMEM;
    imx214.dev = dev;
    imx214.xclk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(imx214.xclk))
    return dev_err_probe(dev, PTR_ERR(imx214.xclk),
    "failed to get xclk\n");
    ret = imx214_get_regulators(dev, imx214);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to get regulators\n");
    imx214.enable_gpio = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(imx214.enable_gpio))
    return dev_err_probe(dev, PTR_ERR(imx214.enable_gpio),
    "failed to get enable gpio\n");
    imx214.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(imx214.regmap))
    return dev_err_probe(dev, PTR_ERR(imx214.regmap),
    "failed to initialize CCI\n");
    ret = imx214_parse_fwnode(imx214);
    if (ret)
    return ret;
    v4l2_i2c_subdev_init(&imx214.sd, client, &imx214_subdev_ops);
    imx214.sd.internal_ops = &imx214_internal_ops;
//
// Enable power initially, to avoid warnings
// from clk_disable on power_off
//
    ret = imx214_power_on(imx214.dev);
    if (ret < 0)
    goto error_fwnode;
    ret = imx214_identify_module(imx214);
    if (ret)
    goto error_power_off;
    ret = imx214_ctrls_init(imx214);
    if (ret < 0)
    goto error_power_off;
    imx214.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    imx214.pad.flags = MEDIA_PAD_FL_SOURCE;
    imx214.sd.dev = &client.dev;
    imx214.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&imx214.sd.entity, 1, &imx214.pad);
    if (ret < 0) {
    dev_err_probe(dev, ret, "failed to init entity pads\n");
    goto free_ctrl;
    }
    imx214.sd.state_lock = imx214.ctrls.lock;
    ret = v4l2_subdev_init_finalize(&imx214.sd);
    if (ret < 0) {
    dev_err_probe(dev, ret, "subdev init error\n");
    goto free_entity;
    }
    pm_runtime_set_active(imx214.dev);
    pm_runtime_enable(imx214.dev);
    ret = v4l2_async_register_subdev_sensor(&imx214.sd);
    if (ret < 0) {
    dev_err_probe(dev, ret,
    "failed to register sensor sub-device\n");
    goto error_subdev_cleanup;
    }
    pm_runtime_idle(imx214.dev);
    return 0;
    error_subdev_cleanup:
    pm_runtime_disable(imx214.dev);
    pm_runtime_set_suspended(&client.dev);
    v4l2_subdev_cleanup(&imx214.sd);
    free_entity:
    media_entity_cleanup(&imx214.sd.entity);
    free_ctrl:
    v4l2_ctrl_handler_free(&imx214.ctrls);
    error_power_off:
    imx214_power_off(imx214.dev);
    error_fwnode:
    v4l2_fwnode_endpoint_free(&imx214.bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx214_remove(client: *mut i2c_client) {
    static void imx214_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct imx214 *imx214 = to_imx214(sd);
    v4l2_async_unregister_subdev(&imx214.sd);
    v4l2_subdev_cleanup(sd);
    media_entity_cleanup(&imx214.sd.entity);
    v4l2_ctrl_handler_free(&imx214.ctrls);
    v4l2_fwnode_endpoint_free(&imx214.bus_cfg);
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev)) {
    imx214_power_off(imx214.dev);
    pm_runtime_set_suspended(&client.dev);
    }
    }
    static const struct of_device_id imx214_of_match[] = {
    { .compatible = "sony,imx214" },
    { }
    };
    MODULE_DEVICE_TABLE(of, imx214_of_match);
    static const struct dev_pm_ops imx214_pm_ops = {
    SET_RUNTIME_PM_OPS(imx214_power_off, imx214_power_on, core::ptr::null_mut())
    };
    static struct i2c_driver imx214_i2c_driver = {
    .driver = {
    .of_match_table = imx214_of_match,
    .pm = &imx214_pm_ops,
    .name  = "imx214",
    },
    .probe = imx214_probe,
    .remove = imx214_remove,
    };
    module_i2c_driver(imx214_i2c_driver);
    MODULE_DESCRIPTION("Sony IMX214 Camera driver");
    MODULE_AUTHOR("Ricardo Ribalda <ribalda@kernel.org>");
    MODULE_LICENSE("GPL v2");
