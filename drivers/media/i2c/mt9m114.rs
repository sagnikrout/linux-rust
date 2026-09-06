//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/mt9m114.c
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
// mt9m114.c onsemi MT9M114 sensor driver
//
// Copyright (c) 2020-2023 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Copyright (c) 2012 Analog Devices Inc.
//
// Almost complete rewrite of work by Scott Jiang <Scott.Jiang.Linux@gmail.com>
// itself based on work from Andrew Chew <achew@nvidia.com>.
//

// Sysctl registers

pub const MT9M114_PAD_SLEW_MIN: c_int = 0;
pub const MT9M114_PAD_SLEW_MAX: c_int = 7;
pub const MT9M114_PAD_SLEW_DEFAULT: c_int = 7;

// XDMA registers

// Sensor Core registers

// Monitor registers

// Auto-Exposure Track registers

// Color Correction Matrix registers

// Camera Control registers

pub const MT9M114_CAM_SENSOR_CFG_FRAME_LENGTH_LINES_MAX: c_int = 65535;

pub const MT9M114_CAM_SENSOR_CFG_LINE_LENGTH_PCK_MAX: c_int = 8191;

pub const MT9M114_CAM_SYSCTL_PLL_DISABLE_VALUE: c_uint = 0x00;

// System Manager registers

// Patch Loader registers

// SYS_STATE values (for SYSMGR_NEXT_STATE and SYSMGR_CURRENT_STATE)
pub const MT9M114_SYS_STATE_ENTER_CONFIG_CHANGE: c_uint = 0x28;
pub const MT9M114_SYS_STATE_STREAMING: c_uint = 0x31;
pub const MT9M114_SYS_STATE_START_STREAMING: c_uint = 0x34;
pub const MT9M114_SYS_STATE_ENTER_SUSPEND: c_uint = 0x40;
pub const MT9M114_SYS_STATE_SUSPENDED: c_uint = 0x41;
pub const MT9M114_SYS_STATE_ENTER_STANDBY: c_uint = 0x50;
pub const MT9M114_SYS_STATE_STANDBY: c_uint = 0x52;
pub const MT9M114_SYS_STATE_LEAVE_STANDBY: c_uint = 0x54;
// Result status of last SET_STATE comamnd
pub const MT9M114_SET_STATE_RESULT_ENOERR: c_uint = 0x00;
pub const MT9M114_SET_STATE_RESULT_EINVAL: c_uint = 0x0c;
pub const MT9M114_SET_STATE_RESULT_ENOSPC: c_uint = 0x0d;
//
// The minimum amount of horizontal and vertical blanking is undocumented. The
// minimum values that have been seen in register lists are 303 and 21, use
// them.
//
// Set the default to achieve full resolution (1296x976 analog crop
// rectangle, 1280x960 output size) at 30fps with a 48 MHz pixclock.
//
pub const MT9M114_MIN_HBLANK: c_int = 303;
pub const MT9M114_MIN_VBLANK: c_int = 21;
pub const MT9M114_DEF_HBLANK: c_int = 308;
pub const MT9M114_DEF_VBLANK: c_int = 21;
pub const MT9M114_DEF_FRAME_RATE: c_int = 30;
pub const MT9M114_MAX_FRAME_RATE: c_int = 120;
pub const MT9M114_DEF_PIXCLOCK: c_int = 48000000;

//
// These values are not well documented and are semi-arbitrary. The pixel array
// minimum output size is 8 pixels larger than the minimum scaler cropped input
// width to account for the demosaicing.
//

// Indices into the mt9m114.ifp.tpg array.
pub const MT9M114_TPG_PATTERN: c_int = 0;
pub const MT9M114_TPG_RED: c_int = 1;
pub const MT9M114_TPG_GREEN: c_int = 2;
pub const MT9M114_TPG_BLUE: c_int = 3;
// -----------------------------------------------------------------------------
// Data Structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m114_model_info {
    pub state_standby_polling: bool,
}

    enum mt9m114_format_flag {
    MT9M114_FMT_FLAG_PARALLEL = BIT(0),
    MT9M114_FMT_FLAG_CSI2 = BIT(1),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m114_format_info {
    pub code: u32,
    pub output_format: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9m114 {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub clk: *mut clk,
    pub reset: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; 3],
    pub bus_cfg: v4l2_fwnode_endpoint,
    pub bypass_pll: bool,
    pub pll: aptina_pll,
    pub pixrate: c_uint,
    pub streaming: bool,
    pub pad_slew_rate: u32,
// Pixel Array
    struct {
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub hdl: v4l2_ctrl_handler,
    pub exposure: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub pa: },
// Image Flow Processor
    struct {
    pub sd: v4l2_subdev,
    pub pads: [media_pad; 2],
    pub hdl: v4l2_ctrl_handler,
    pub frame_rate: c_uint,
    pub tpg: [*mut v4l2_ctrl; 4],
    pub ifp: },
    pub info: *const mt9m114_model_info,
}

// -----------------------------------------------------------------------------
// Formats
//
    static const struct mt9m114_format_info mt9m114_format_infos[] = {
    {
//
// The first two entries are used as defaults, for parallel and
// CSI-2 buses respectively. Keep them in that order.
//
    .code = MEDIA_BUS_FMT_UYVY8_2X8,
    .flags = MT9M114_FMT_FLAG_PARALLEL,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_FORMAT_YUV,
    }, {
    .code = MEDIA_BUS_FMT_UYVY8_1X16,
    .flags = MT9M114_FMT_FLAG_CSI2,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_FORMAT_YUV,
    }, {
    .code = MEDIA_BUS_FMT_YUYV8_2X8,
    .flags = MT9M114_FMT_FLAG_PARALLEL,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_FORMAT_YUV
    | MT9M114_CAM_OUTPUT_FORMAT_SWAP_BYTES,
    }, {
    .code = MEDIA_BUS_FMT_YUYV8_1X16,
    .flags = MT9M114_FMT_FLAG_CSI2,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_FORMAT_YUV
    | MT9M114_CAM_OUTPUT_FORMAT_SWAP_BYTES,
    }, {
    .code = MEDIA_BUS_FMT_RGB565_2X8_LE,
    .flags = MT9M114_FMT_FLAG_PARALLEL,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_RGB_FORMAT_565RGB
    | MT9M114_CAM_OUTPUT_FORMAT_FORMAT_RGB
    | MT9M114_CAM_OUTPUT_FORMAT_SWAP_BYTES,
    }, {
    .code = MEDIA_BUS_FMT_RGB565_2X8_BE,
    .flags = MT9M114_FMT_FLAG_PARALLEL,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_RGB_FORMAT_565RGB
    | MT9M114_CAM_OUTPUT_FORMAT_FORMAT_RGB,
    }, {
    .code = MEDIA_BUS_FMT_RGB565_1X16,
    .flags = MT9M114_FMT_FLAG_CSI2,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_RGB_FORMAT_565RGB
    | MT9M114_CAM_OUTPUT_FORMAT_FORMAT_RGB,
    }, {
    .code = MEDIA_BUS_FMT_SGRBG8_1X8,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_BAYER_FORMAT_PROCESSED8
    | MT9M114_CAM_OUTPUT_FORMAT_FORMAT_BAYER,
    .flags = MT9M114_FMT_FLAG_PARALLEL | MT9M114_FMT_FLAG_CSI2,
    }, {
// Keep the format compatible with the IFP sink pad last.
    .code = MEDIA_BUS_FMT_SGRBG10_1X10,
    .output_format = MT9M114_CAM_OUTPUT_FORMAT_BAYER_FORMAT_RAWR10
    | MT9M114_CAM_OUTPUT_FORMAT_FORMAT_BAYER,
    .flags = MT9M114_FMT_FLAG_PARALLEL | MT9M114_FMT_FLAG_CSI2,
    }
    };
    static const struct mt9m114_format_info *
    mt9m114_default_format_info(struct mt9m114 *sensor)
    {
    if (sensor.bus_cfg.bus_type == V4L2_MBUS_CSI2_DPHY)
    return &mt9m114_format_infos[1];
    else
    return &mt9m114_format_infos[0];
    }
    static const struct mt9m114_format_info *
    mt9m114_format_info(struct mt9m114 *sensor, unsigned int pad, u32 code)
    {
    let mut num_formats: c_uint = ARRAY_SIZE(mt9m114_format_infos);
    unsigned int flag;
    unsigned int i;
    switch (pad) {
    case 0:
    return &mt9m114_format_infos[num_formats - 1];
    case 1:
    if (sensor.bus_cfg.bus_type == V4L2_MBUS_CSI2_DPHY)
    flag = MT9M114_FMT_FLAG_CSI2;
    else
    flag = MT9M114_FMT_FLAG_PARALLEL;
    for (i = 0; i < num_formats; ++i) {
    const struct mt9m114_format_info *info =
    &mt9m114_format_infos[i];
    if (info.code == code && info.flags & flag)
    return info;
    }
    return mt9m114_default_format_info(sensor);
    default:
    return core::ptr::null_mut();
    }
    }
// -----------------------------------------------------------------------------
// Initialization
//
    static const struct cci_reg_sequence mt9m114_init[] = {
    { MT9M114_RESET_REGISTER, MT9M114_RESET_REGISTER_MASK_BAD |
    MT9M114_RESET_REGISTER_LOCK_REG |
    0x0010 },
// Sensor optimization
    { CCI_REG16(0x316a), 0x8270 },
    { CCI_REG16(0x316c), 0x8270 },
    { CCI_REG16(0x3ed0), 0x2305 },
    { CCI_REG16(0x3ed2), 0x77cf },
    { CCI_REG16(0x316e), 0x8202 },
    { CCI_REG16(0x3180), 0x87ff },
    { CCI_REG16(0x30d4), 0x6080 },
    { CCI_REG16(0xa802), 0x0008 },
    { CCI_REG16(0x3e14), 0xff39 },
// APGA
    { MT9M114_CAM_PGA_PGA_CONTROL,			0x0000 },
// Automatic White balance
    { MT9M114_CAM_AWB_CCM_L(0),			0x0267 },
    { MT9M114_CAM_AWB_CCM_L(1),			0xff1a },
    { MT9M114_CAM_AWB_CCM_L(2),			0xffb3 },
    { MT9M114_CAM_AWB_CCM_L(3),			0xff80 },
    { MT9M114_CAM_AWB_CCM_L(4),			0x0166 },
    { MT9M114_CAM_AWB_CCM_L(5),			0x0003 },
    { MT9M114_CAM_AWB_CCM_L(6),			0xff9a },
    { MT9M114_CAM_AWB_CCM_L(7),			0xfeb4 },
    { MT9M114_CAM_AWB_CCM_L(8),			0x024d },
    { MT9M114_CAM_AWB_CCM_M(0),			0x01bf },
    { MT9M114_CAM_AWB_CCM_M(1),			0xff01 },
    { MT9M114_CAM_AWB_CCM_M(2),			0xfff3 },
    { MT9M114_CAM_AWB_CCM_M(3),			0xff75 },
    { MT9M114_CAM_AWB_CCM_M(4),			0x0198 },
    { MT9M114_CAM_AWB_CCM_M(5),			0xfffd },
    { MT9M114_CAM_AWB_CCM_M(6),			0xff9a },
    { MT9M114_CAM_AWB_CCM_M(7),			0xfee7 },
    { MT9M114_CAM_AWB_CCM_M(8),			0x02a8 },
    { MT9M114_CAM_AWB_CCM_R(0),			0x01d9 },
    { MT9M114_CAM_AWB_CCM_R(1),			0xff26 },
    { MT9M114_CAM_AWB_CCM_R(2),			0xfff3 },
    { MT9M114_CAM_AWB_CCM_R(3),			0xffb3 },
    { MT9M114_CAM_AWB_CCM_R(4),			0x0132 },
    { MT9M114_CAM_AWB_CCM_R(5),			0xffe8 },
    { MT9M114_CAM_AWB_CCM_R(6),			0xffda },
    { MT9M114_CAM_AWB_CCM_R(7),			0xfecd },
    { MT9M114_CAM_AWB_CCM_R(8),			0x02c2 },
    { MT9M114_CAM_AWB_CCM_L_RG_GAIN,		0x0075 },
    { MT9M114_CAM_AWB_CCM_L_BG_GAIN,		0x011c },
    { MT9M114_CAM_AWB_CCM_M_RG_GAIN,		0x009a },
    { MT9M114_CAM_AWB_CCM_M_BG_GAIN,		0x0105 },
    { MT9M114_CAM_AWB_CCM_R_RG_GAIN,		0x00a4 },
    { MT9M114_CAM_AWB_CCM_R_BG_GAIN,		0x00ac },
    { MT9M114_CAM_AWB_CCM_L_CTEMP,			0x0a8c },
    { MT9M114_CAM_AWB_CCM_M_CTEMP,			0x0f0a },
    { MT9M114_CAM_AWB_CCM_R_CTEMP,			0x1964 },
    { MT9M114_CAM_AWB_AWB_XSHIFT_PRE_ADJ,		51 },
    { MT9M114_CAM_AWB_AWB_YSHIFT_PRE_ADJ,		60 },
    { MT9M114_CAM_AWB_AWB_XSCALE,			3 },
    { MT9M114_CAM_AWB_AWB_YSCALE,			2 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(0),		0x0000 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(1),		0x0000 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(2),		0x0000 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(3),		0xe724 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(4),		0x1583 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(5),		0x2045 },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(6),		0x03ff },
    { MT9M114_CAM_AWB_AWB_WEIGHTS(7),		0x007c },
    { MT9M114_CAM_AWB_K_R_L,			0x80 },
    { MT9M114_CAM_AWB_K_G_L,			0x80 },
    { MT9M114_CAM_AWB_K_B_L,			0x80 },
    { MT9M114_CAM_AWB_K_R_R,			0x88 },
    { MT9M114_CAM_AWB_K_G_R,			0x80 },
    { MT9M114_CAM_AWB_K_B_R,			0x80 },
// Low-Light Image Enhancements
    { MT9M114_CAM_LL_START_BRIGHTNESS,		0x0020 },
    { MT9M114_CAM_LL_STOP_BRIGHTNESS,		0x009a },
    { MT9M114_CAM_LL_START_GAIN_METRIC,		0x0070 },
    { MT9M114_CAM_LL_STOP_GAIN_METRIC,		0x00f3 },
    { MT9M114_CAM_LL_START_CONTRAST_LUMA_PERCENTAGE, 0x20 },
    { MT9M114_CAM_LL_STOP_CONTRAST_LUMA_PERCENTAGE,	0x9a },
    { MT9M114_CAM_LL_START_SATURATION,		0x80 },
    { MT9M114_CAM_LL_END_SATURATION,		0x4b },
    { MT9M114_CAM_LL_START_DESATURATION,		0x00 },
    { MT9M114_CAM_LL_END_DESATURATION,		0xff },
    { MT9M114_CAM_LL_START_DEMOSAICING,		0x3c },
    { MT9M114_CAM_LL_START_AP_GAIN,			0x02 },
    { MT9M114_CAM_LL_START_AP_THRESH,		0x06 },
    { MT9M114_CAM_LL_STOP_DEMOSAICING,		0x64 },
    { MT9M114_CAM_LL_STOP_AP_GAIN,			0x01 },
    { MT9M114_CAM_LL_STOP_AP_THRESH,		0x0c },
    { MT9M114_CAM_LL_START_NR_RED,			0x3c },
    { MT9M114_CAM_LL_START_NR_GREEN,		0x3c },
    { MT9M114_CAM_LL_START_NR_BLUE,			0x3c },
    { MT9M114_CAM_LL_START_NR_THRESH,		0x0f },
    { MT9M114_CAM_LL_STOP_NR_RED,			0x64 },
    { MT9M114_CAM_LL_STOP_NR_GREEN,			0x64 },
    { MT9M114_CAM_LL_STOP_NR_BLUE,			0x64 },
    { MT9M114_CAM_LL_STOP_NR_THRESH,		0x32 },
    { MT9M114_CAM_LL_START_CONTRAST_BM,		0x0020 },
    { MT9M114_CAM_LL_STOP_CONTRAST_BM,		0x009a },
    { MT9M114_CAM_LL_GAMMA,				0x00dc },
    { MT9M114_CAM_LL_START_CONTRAST_GRADIENT,	0x38 },
    { MT9M114_CAM_LL_STOP_CONTRAST_GRADIENT,	0x30 },
    { MT9M114_CAM_LL_START_CONTRAST_LUMA_PERCENTAGE, 0x50 },
    { MT9M114_CAM_LL_STOP_CONTRAST_LUMA_PERCENTAGE,	0x19 },
    { MT9M114_CAM_LL_START_FADE_TO_BLACK_LUMA,	0x0230 },
    { MT9M114_CAM_LL_STOP_FADE_TO_BLACK_LUMA,	0x0010 },
    { MT9M114_CAM_LL_CLUSTER_DC_TH_BM,		0x01cd },
    { MT9M114_CAM_LL_CLUSTER_DC_GATE_PERCENTAGE,	0x05 },
    { MT9M114_CAM_LL_SUMMING_SENSITIVITY_FACTOR,	0x40 },
// Auto-Exposure
    { MT9M114_CAM_AET_TARGET_AVERAGE_LUMA_DARK,	0x1b },
    { MT9M114_CAM_AET_AEMODE,			0x00 },
    { MT9M114_CAM_AET_TARGET_GAIN,			0x0080 },
    { MT9M114_CAM_AET_AE_MAX_VIRT_AGAIN,		0x0100 },
    { MT9M114_CAM_AET_BLACK_CLIPPING_TARGET,	0x005a },
    { MT9M114_CCM_DELTA_GAIN,			0x05 },
    { MT9M114_AE_TRACK_AE_TRACKING_DAMPENING_SPEED,	0x20 },
// Pixel array timings and integration time
    { MT9M114_CAM_SENSOR_CFG_ROW_SPEED,		1 },
    { MT9M114_CAM_SENSOR_CFG_FINE_INTEG_TIME_MIN,	219 },
    { MT9M114_CAM_SENSOR_CFG_FINE_INTEG_TIME_MAX,	1459 },
    { MT9M114_CAM_SENSOR_CFG_FINE_CORRECTION,	96 },
    { MT9M114_CAM_SENSOR_CFG_REG_0_DATA,		32 },
    };
// -----------------------------------------------------------------------------
// Hardware Configuration
//
// Wait for a command to complete.
#[no_mangle]
unsafe extern "C" fn mt9m114_poll_command(sensor: *mut mt9m114, command: u32) -> c_int {
    static int mt9m114_poll_command(struct mt9m114 *sensor, u32 command)
    {
    unsigned int i;
    u64 value;
    int ret;
    for (i = 0; i < 100; ++i) {
    ret = cci_read(sensor.regmap, MT9M114_COMMAND_REGISTER, &value,
    core::ptr::null_mut());
    if (ret < 0)
    return ret;
    if (!(value & command))
    break;
    usleep_range(5000, 6000);
    }
    if (value & command) {
    dev_err(&sensor.client.dev, "Command %u completion timeout\n",
    command);
    return -ETIMEDOUT;
    }
    if (!(value & MT9M114_COMMAND_REGISTER_OK)) {
    dev_err(&sensor.client.dev, "Command %u failed\n", command);
    return -EIO;
    }
    return 0;
    }
// Wait for a state to be entered.
#[no_mangle]
unsafe extern "C" fn mt9m114_poll_state(sensor: *mut mt9m114, state: u32) -> c_int {
    static int mt9m114_poll_state(struct mt9m114 *sensor, u32 state)
    {
    unsigned int i;
    u64 value;
    int ret;
    for (i = 0; i < 100; ++i) {
    ret = cci_read(sensor.regmap, MT9M114_SYSMGR_CURRENT_STATE,
    &value, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    if (value == state)
    return 0;
    usleep_range(1000, 1500);
    }
    dev_err(&sensor.client.dev, "Timeout waiting for state 0x%02x\n",
    state);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_set_state(sensor: *mut mt9m114, next_state: u8) -> c_int {
    static int mt9m114_set_state(struct mt9m114 *sensor, u8 next_state)
    {
    let mut ret: c_int = 0;
// Set the next desired state and start the state transition.
    cci_write(sensor.regmap, MT9M114_SYSMGR_NEXT_STATE, next_state, &ret);
    cci_write(sensor.regmap, MT9M114_COMMAND_REGISTER,
    MT9M114_COMMAND_REGISTER_OK |
    MT9M114_COMMAND_REGISTER_SET_STATE, &ret);
    if (ret < 0)
    return ret;
// Wait for the state transition to complete.
    ret = mt9m114_poll_command(sensor, MT9M114_COMMAND_REGISTER_SET_STATE);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_initialize(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_initialize(struct mt9m114 *sensor)
    {
    u32 value;
    int ret;
    ret = cci_multi_reg_write(sensor.regmap, mt9m114_init,
    ARRAY_SIZE(mt9m114_init), core::ptr::null_mut());
    if (ret < 0) {
    dev_err(&sensor.client.dev,
    "Failed to initialize the sensor\n");
    return ret;
    }
// Configure the PLL.
    if (sensor.bypass_pll) {
    cci_write(sensor.regmap, MT9M114_CAM_SYSCTL_PLL_ENABLE,
    MT9M114_CAM_SYSCTL_PLL_DISABLE_VALUE, &ret);
    } else {
    cci_write(sensor.regmap, MT9M114_CAM_SYSCTL_PLL_ENABLE,
    MT9M114_CAM_SYSCTL_PLL_ENABLE_VALUE, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SYSCTL_PLL_DIVIDER_M_N,
    MT9M114_CAM_SYSCTL_PLL_DIVIDER_VALUE(sensor.pll.m,
    sensor.pll.n),
    &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SYSCTL_PLL_DIVIDER_P,
    MT9M114_CAM_SYSCTL_PLL_DIVIDER_P_VALUE(sensor.pll.p1),
    &ret);
    }
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_PIXCLK,
    sensor.pixrate, &ret);
// Configure the output mode.
    if (sensor.bus_cfg.bus_type == V4L2_MBUS_CSI2_DPHY) {
    value = MT9M114_CAM_PORT_PORT_SELECT_MIPI
    | MT9M114_CAM_PORT_CHAN_NUM(0)
    | 0x8000;
    if (!(sensor.bus_cfg.bus.mipi_csi2.flags &
    V4L2_MBUS_CSI2_NONCONTINUOUS_CLOCK))
    value |= MT9M114_CAM_PORT_CONT_MIPI_CLK;
    } else {
    value = MT9M114_CAM_PORT_PORT_SELECT_PARALLEL
    | 0x8000;
    }
    cci_write(sensor.regmap, MT9M114_CAM_PORT_OUTPUT_CONTROL, value, &ret);
    if (ret < 0)
    return ret;
    value = sensor.pad_slew_rate
    | sensor.pad_slew_rate << 4
    |	sensor.pad_slew_rate << 8;
    cci_write(sensor.regmap, MT9M114_PAD_SLEW, value, &ret);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int mt9m114_configure_pa(struct mt9m114 *sensor,
    struct v4l2_subdev_state *state)
    {
    const struct v4l2_mbus_framefmt *format;
    const struct v4l2_rect *crop;
    unsigned int hratio, vratio;
    u64 read_mode;
    int ret;
    format = v4l2_subdev_state_get_format(state, 0);
    crop = v4l2_subdev_state_get_crop(state, 0);
    ret = cci_read(sensor.regmap, MT9M114_CAM_SENSOR_CONTROL_READ_MODE,
    &read_mode, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    hratio = crop.width / format.width;
    vratio = crop.height / format.height;
//
// Pixel array crop and binning. The CAM_SENSOR_CFG_CPIPE_LAST_ROW
// register isn't clearly documented, but is always set to the number
// of active rows minus 4 divided by the vertical binning factor in all
// example sensor modes.
//
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_X_ADDR_START,
    crop.left, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_Y_ADDR_START,
    crop.top, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_X_ADDR_END,
    crop.width + crop.left - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_Y_ADDR_END,
    crop.height + crop.top - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_CPIPE_LAST_ROW,
    (crop.height - 4) / vratio - 1, &ret);
    read_mode &= ~(MT9M114_CAM_SENSOR_CONTROL_X_READ_OUT_MASK |
    MT9M114_CAM_SENSOR_CONTROL_Y_READ_OUT_MASK);
    if (hratio > 1)
    read_mode |= MT9M114_CAM_SENSOR_CONTROL_X_READ_OUT_SUMMING;
    if (vratio > 1)
    read_mode |= MT9M114_CAM_SENSOR_CONTROL_Y_READ_OUT_SUMMING;
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CONTROL_READ_MODE,
    read_mode, &ret);
    return ret;
    }
//
// For source pad formats other then RAW10 the IFP removes a 4 pixel border from
// its sink pad format size for demosaicing.
//
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_get_border(state: *mut v4l2_subdev_state) -> c_int {
    static int mt9m114_ifp_get_border(struct v4l2_subdev_state *state)
    {
    const struct v4l2_mbus_framefmt *format =
    v4l2_subdev_state_get_format(state, 1);
    return format.code == MEDIA_BUS_FMT_SGRBG10_1X10 ? 0 : 4;
    }
    static int mt9m114_configure_ifp(struct mt9m114 *sensor,
    struct v4l2_subdev_state *state)
    {
    const struct mt9m114_format_info *info;
    const struct v4l2_mbus_framefmt *format;
    const struct v4l2_rect *crop;
    const struct v4l2_rect *compose;
    unsigned int border;
    u64 output_format;
    let mut ret: c_int = 0;
    format = v4l2_subdev_state_get_format(state, 1);
    info = mt9m114_format_info(sensor, 1, format.code);
    crop = v4l2_subdev_state_get_crop(state, 0);
    compose = v4l2_subdev_state_get_compose(state, 0);
    ret = cci_read(sensor.regmap, MT9M114_CAM_OUTPUT_FORMAT,
    &output_format, core::ptr::null_mut());
    if (ret < 0)
    return ret;
//
// Color pipeline (IFP) cropping and scaling. The crop window registers
// apply cropping after demosaicing, which itself consumes 4 pixels on
// each side of the image. The crop rectangle exposed to userspace
// includes that demosaicing border, subtract it from the left and top
// coordinates to configure the crop window.
//
    border = mt9m114_ifp_get_border(state);
    cci_write(sensor.regmap, MT9M114_CAM_CROP_WINDOW_XOFFSET,
    crop.left - border, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_CROP_WINDOW_YOFFSET,
    crop.top - border, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_CROP_WINDOW_WIDTH,
    crop.width, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_CROP_WINDOW_HEIGHT,
    crop.height, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_OUTPUT_WIDTH,
    compose.width, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_OUTPUT_HEIGHT,
    compose.height, &ret);
// AWB and AE windows, use the full frame.
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AWB_CLIP_WINDOW_XSTART,
    0, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AWB_CLIP_WINDOW_YSTART,
    0, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AWB_CLIP_WINDOW_XEND,
    compose.width - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AWB_CLIP_WINDOW_YEND,
    compose.height - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AE_INITIAL_WINDOW_XSTART,
    0, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AE_INITIAL_WINDOW_YSTART,
    0, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AE_INITIAL_WINDOW_XEND,
    compose.width / 5 - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_STAT_AE_INITIAL_WINDOW_YEND,
    compose.height / 5 - 1, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_CROP_CROPMODE,
    MT9M114_CAM_CROP_MODE_AWB_AUTO_CROP_EN |
    MT9M114_CAM_CROP_MODE_AE_AUTO_CROP_EN, &ret);
// Set the media bus code.
    output_format &= ~(MT9M114_CAM_OUTPUT_FORMAT_RGB_FORMAT_MASK |
    MT9M114_CAM_OUTPUT_FORMAT_BAYER_FORMAT_MASK |
    MT9M114_CAM_OUTPUT_FORMAT_FORMAT_MASK |
    MT9M114_CAM_OUTPUT_FORMAT_SWAP_BYTES |
    MT9M114_CAM_OUTPUT_FORMAT_SWAP_RED_BLUE);
    output_format |= info.output_format;
    cci_write(sensor.regmap, MT9M114_CAM_OUTPUT_FORMAT,
    output_format, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_set_frame_rate(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_set_frame_rate(struct mt9m114 *sensor)
    {
    let mut frame_rate: u16 = sensor.ifp.frame_rate << 8;
    let mut ret: c_int = 0;
    cci_write(sensor.regmap, MT9M114_CAM_AET_MIN_FRAME_RATE,
    frame_rate, &ret);
    cci_write(sensor.regmap, MT9M114_CAM_AET_MAX_FRAME_RATE,
    frame_rate, &ret);
    return ret;
    }
    static int mt9m114_start_streaming(struct mt9m114 *sensor,
    struct v4l2_subdev_state *pa_state,
    struct v4l2_subdev_state *ifp_state)
    {
    int ret;
    ret = pm_runtime_resume_and_get(&sensor.client.dev);
    if (ret)
    return ret;
    ret = mt9m114_initialize(sensor);
    if (ret)
    goto error;
    ret = mt9m114_configure_ifp(sensor, ifp_state);
    if (ret)
    goto error;
    ret = mt9m114_configure_pa(sensor, pa_state);
    if (ret)
    goto error;
    ret = mt9m114_set_frame_rate(sensor);
    if (ret)
    goto error;
    ret = __v4l2_ctrl_handler_setup(&sensor.pa.hdl);
    if (ret)
    goto error;
    ret = __v4l2_ctrl_handler_setup(&sensor.ifp.hdl);
    if (ret)
    goto error;
//
// The Change-Config state is transient and moves to the streaming
// state automatically.
//
    ret = mt9m114_set_state(sensor, MT9M114_SYS_STATE_ENTER_CONFIG_CHANGE);
    if (ret)
    goto error;
    sensor.streaming = true;
    return 0;
    error:
    pm_runtime_put_autosuspend(&sensor.client.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_stop_streaming(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_stop_streaming(struct mt9m114 *sensor)
    {
    int ret;
    sensor.streaming = false;
    ret = mt9m114_set_state(sensor, MT9M114_SYS_STATE_ENTER_SUSPEND);
    pm_runtime_put_autosuspend(&sensor.client.dev);
    return ret;
    }
// -----------------------------------------------------------------------------
// Common Subdev Operations
//
    static const struct media_entity_operations mt9m114_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
// -----------------------------------------------------------------------------
// Pixel Array Control Operations
//
    static inline struct mt9m114 *pa_ctrl_to_mt9m114(struct v4l2_ctrl *ctrl)
    {
    return container_of(ctrl.handler, struct mt9m114, pa.hdl);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_pa_g_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9m114_pa_g_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9m114 *sensor = pa_ctrl_to_mt9m114(ctrl);
    u64 value;
    int ret;
    if (!pm_runtime_get_if_in_use(&sensor.client.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    ret = cci_read(sensor.regmap,
    MT9M114_CAM_SENSOR_CONTROL_COARSE_INTEGRATION_TIME,
    &value, core::ptr::null_mut());
    if (ret)
    break;
    ctrl.val = value;
    break;
    case V4L2_CID_ANALOGUE_GAIN:
    ret = cci_read(sensor.regmap,
    MT9M114_CAM_SENSOR_CONTROL_ANALOG_GAIN,
    &value, core::ptr::null_mut());
    if (ret)
    break;
    ctrl.val = value;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put_autosuspend(&sensor.client.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_pa_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9m114_pa_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9m114 *sensor = pa_ctrl_to_mt9m114(ctrl);
    const struct v4l2_mbus_framefmt *format;
    struct v4l2_subdev_state *state;
    let mut ret: c_int = 0;
    u64 mask;
// V4L2 controls values are applied only when power is up.
    if (!pm_runtime_get_if_in_use(&sensor.client.dev))
    return 0;
    state = v4l2_subdev_get_locked_active_state(&sensor.pa.sd);
    format = v4l2_subdev_state_get_format(state, 0);
    switch (ctrl.id) {
    case V4L2_CID_HBLANK:
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_LINE_LENGTH_PCK,
    ctrl.val + format.width, &ret);
    break;
    case V4L2_CID_VBLANK:
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CFG_FRAME_LENGTH_LINES,
    ctrl.val + format.height, &ret);
    break;
    case V4L2_CID_EXPOSURE:
    cci_write(sensor.regmap,
    MT9M114_CAM_SENSOR_CONTROL_COARSE_INTEGRATION_TIME,
    ctrl.val, &ret);
    break;
    case V4L2_CID_ANALOGUE_GAIN:
//
// The CAM_SENSOR_CONTROL_ANALOG_GAIN contains linear analog
// gain values that are mapped to the GLOBAL_GAIN register
// values by the sensor firmware.
//
    cci_write(sensor.regmap, MT9M114_CAM_SENSOR_CONTROL_ANALOG_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_HFLIP:
    mask = MT9M114_CAM_SENSOR_CONTROL_HORZ_MIRROR_EN;
    ret = cci_update_bits(sensor.regmap,
    MT9M114_CAM_SENSOR_CONTROL_READ_MODE,
    mask, ctrl.val ? mask : 0, core::ptr::null_mut());
    break;
    case V4L2_CID_VFLIP:
    mask = MT9M114_CAM_SENSOR_CONTROL_VERT_FLIP_EN;
    ret = cci_update_bits(sensor.regmap,
    MT9M114_CAM_SENSOR_CONTROL_READ_MODE,
    mask, ctrl.val ? mask : 0, core::ptr::null_mut());
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put_autosuspend(&sensor.client.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops mt9m114_pa_ctrl_ops = {
    .g_volatile_ctrl = mt9m114_pa_g_ctrl,
    .s_ctrl = mt9m114_pa_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn mt9m114_pa_ctrl_update_exposure(sensor: *mut mt9m114, manual: bool) {
    static void mt9m114_pa_ctrl_update_exposure(struct mt9m114 *sensor, bool manual)
    {
//
// Update the volatile flag on the manual exposure and gain controls.
// If the controls have switched to manual, read their current value
// from the hardware to ensure that control read and write operations
// will behave correctly
//
    if (manual) {
    mt9m114_pa_g_ctrl(sensor.pa.exposure);
    sensor.pa.exposure.cur.val = sensor.pa.exposure.val;
    sensor.pa.exposure.flags &= ~V4L2_CTRL_FLAG_VOLATILE;
    mt9m114_pa_g_ctrl(sensor.pa.gain);
    sensor.pa.gain.cur.val = sensor.pa.gain.val;
    sensor.pa.gain.flags &= ~V4L2_CTRL_FLAG_VOLATILE;
    } else {
    sensor.pa.exposure.flags |= V4L2_CTRL_FLAG_VOLATILE;
    sensor.pa.gain.flags |= V4L2_CTRL_FLAG_VOLATILE;
    }
    }
    static void mt9m114_pa_ctrl_update_blanking(struct mt9m114 *sensor,
    const struct v4l2_mbus_framefmt *format)
    {
    unsigned int max_blank;
// Update the blanking controls ranges based on the output size.
    max_blank = MT9M114_CAM_SENSOR_CFG_LINE_LENGTH_PCK_MAX
    - format.width;
    __v4l2_ctrl_modify_range(sensor.pa.hblank, MT9M114_MIN_HBLANK,
    max_blank, 1, MT9M114_DEF_HBLANK);
    max_blank = MT9M114_CAM_SENSOR_CFG_FRAME_LENGTH_LINES_MAX
    - format.height;
    __v4l2_ctrl_modify_range(sensor.pa.vblank, MT9M114_MIN_VBLANK,
    max_blank, 1, MT9M114_DEF_VBLANK);
    }
// -----------------------------------------------------------------------------
// Pixel Array Subdev Operations
//
    static inline struct mt9m114 *pa_to_mt9m114(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct mt9m114, pa.sd);
    }
    static int mt9m114_pa_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    crop = v4l2_subdev_state_get_crop(state, 0);
    crop.left = 0;
    crop.top = 0;
    crop.width = MT9M114_PIXEL_ARRAY_WIDTH;
    crop.height = MT9M114_PIXEL_ARRAY_HEIGHT;
    format = v4l2_subdev_state_get_format(state, 0);
    format.width = MT9M114_PIXEL_ARRAY_WIDTH;
    format.height = MT9M114_PIXEL_ARRAY_HEIGHT;
    format.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_RAW;
    format.ycbcr_enc = V4L2_YCBCR_ENC_601;
    format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    format.xfer_func = V4L2_XFER_FUNC_NONE;
    return 0;
    }
    static int mt9m114_pa_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    return 0;
    }
    static int mt9m114_pa_enum_framesizes(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.index > 1)
    return -EINVAL;
    if (fse.code != MEDIA_BUS_FMT_SGRBG10_1X10)
    return -EINVAL;
// Report binning capability through frame size enumeration.
    fse.min_width = MT9M114_PIXEL_ARRAY_WIDTH / (fse.index + 1);
    fse.max_width = MT9M114_PIXEL_ARRAY_WIDTH / (fse.index + 1);
    fse.min_height = MT9M114_PIXEL_ARRAY_HEIGHT / (fse.index + 1);
    fse.max_height = MT9M114_PIXEL_ARRAY_HEIGHT / (fse.index + 1);
    return 0;
    }
    static int mt9m114_pa_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *fmt)
    {
    struct mt9m114 *sensor = pa_to_mt9m114(sd);
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    unsigned int hscale;
    unsigned int vscale;
    crop = v4l2_subdev_state_get_crop(state, fmt.pad);
    format = v4l2_subdev_state_get_format(state, fmt.pad);
// The sensor can bin horizontally and vertically.
    hscale = DIV_ROUND_CLOSEST(crop.width, fmt.format.width ? : 1);
    vscale = DIV_ROUND_CLOSEST(crop.height, fmt.format.height ? : 1);
    format.width = crop.width / clamp(hscale, 1U, 2U);
    format.height = crop.height / clamp(vscale, 1U, 2U);
    fmt.format = *format;
    if (fmt.which == V4L2_SUBDEV_FORMAT_ACTIVE)
    mt9m114_pa_ctrl_update_blanking(sensor, format);
    return 0;
    }
    static int mt9m114_pa_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    sel.r = *v4l2_subdev_state_get_crop(state, sel.pad);
    return 0;
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    case V4L2_SEL_TGT_NATIVE_SIZE:
    sel.r.left = 0;
    sel.r.top = 0;
    sel.r.width = MT9M114_PIXEL_ARRAY_WIDTH;
    sel.r.height = MT9M114_PIXEL_ARRAY_HEIGHT;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int mt9m114_pa_set_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    struct mt9m114 *sensor = pa_to_mt9m114(sd);
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    let mut ret: c_int = 0;
    if (sel.target != V4L2_SEL_TGT_CROP)
    return -EINVAL;
    crop = v4l2_subdev_state_get_crop(state, sel.pad);
    format = v4l2_subdev_state_get_format(state, sel.pad);
//
// Clamp the crop rectangle. The vertical coordinates must be even, and
// the horizontal coordinates must be a multiple of 4.
//
// FIXME: The horizontal coordinates must be a multiple of 8 when
// binning, but binning is configured after setting the selection, so
// we can't know tell here if it will be used.
//
    sel.r.left = ALIGN(sel.r.left, 4);
    sel.r.top = ALIGN(sel.r.top, 2);
    sel.r.width = clamp_t(unsigned int, ALIGN(sel.r.width, 4),
    MT9M114_PIXEL_ARRAY_MIN_OUTPUT_WIDTH,
    MT9M114_PIXEL_ARRAY_WIDTH - sel.r.left);
    sel.r.height = clamp_t(unsigned int, ALIGN(sel.r.height, 2),
    MT9M114_PIXEL_ARRAY_MIN_OUTPUT_HEIGHT,
    MT9M114_PIXEL_ARRAY_HEIGHT - sel.r.top);
// Changing the selection size is not allowed in streaming state.
    if (sensor.streaming &&
    (sel.r.height != crop.height || sel.r.width != crop.width))
    return -EBUSY;
// crop = sel->r;
// Reset the format.
    format.width = crop.width;
    format.height = crop.height;
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return ret;
    mt9m114_pa_ctrl_update_blanking(sensor, format);
// Apply values immediately if streaming.
    if (sensor.streaming) {
    ret = mt9m114_configure_pa(sensor, state);
    if (ret)
    return ret;
// Changing the cropping config requires a CONFIG_CHANGE.
    ret = mt9m114_set_state(sensor,
    MT9M114_SYS_STATE_ENTER_CONFIG_CHANGE);
    }
    return ret;
    }
    static const struct v4l2_subdev_pad_ops mt9m114_pa_pad_ops = {
    .enum_mbus_code = mt9m114_pa_enum_mbus_code,
    .enum_frame_size = mt9m114_pa_enum_framesizes,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = mt9m114_pa_set_fmt,
    .get_selection = mt9m114_pa_get_selection,
    .set_selection = mt9m114_pa_set_selection,
    };
    static const struct v4l2_subdev_ops mt9m114_pa_ops = {
    .pad = &mt9m114_pa_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops mt9m114_pa_internal_ops = {
    .init_state = mt9m114_pa_init_state,
    };
#[no_mangle]
unsafe extern "C" fn mt9m114_pa_init(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_pa_init(struct mt9m114 *sensor)
    {
    struct v4l2_ctrl_handler *hdl = &sensor.pa.hdl;
    struct v4l2_subdev *sd = &sensor.pa.sd;
    struct media_pad *pads = &sensor.pa.pad;
    const struct v4l2_mbus_framefmt *format;
    struct v4l2_subdev_state *state;
    unsigned int max_exposure;
    int ret;
// Initialize the subdev.
    v4l2_subdev_init(sd, &mt9m114_pa_ops);
    sd.internal_ops = &mt9m114_pa_internal_ops;
    v4l2_i2c_subdev_set_name(sd, sensor.client, core::ptr::null_mut(), " pixel array");
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    sd.owner = THIS_MODULE;
    sd.dev = &sensor.client.dev;
    v4l2_set_subdevdata(sd, sensor.client);
// Initialize the media entity.
    sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    sd.entity.ops = &mt9m114_entity_ops;
    pads[0].flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&sd.entity, 1, pads);
    if (ret < 0)
    return ret;
// Initialize the control handler.
    v4l2_ctrl_handler_init(hdl, 7);
// The range of the HBLANK and VBLANK controls will be updated below.
    sensor.pa.hblank = v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_HBLANK,
    MT9M114_DEF_HBLANK,
    MT9M114_DEF_HBLANK, 1,
    MT9M114_DEF_HBLANK);
    sensor.pa.vblank = v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_VBLANK,
    MT9M114_DEF_VBLANK,
    MT9M114_DEF_VBLANK, 1,
    MT9M114_DEF_VBLANK);
//
// The maximum coarse integration time is the frame length in lines
// minus two. The default is taken directly from the datasheet, but
// makes little sense as auto-exposure is enabled by default.
//
    max_exposure = MT9M114_PIXEL_ARRAY_HEIGHT + MT9M114_MIN_VBLANK - 2;
    sensor.pa.exposure = v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_EXPOSURE, 1,
    max_exposure, 1, 16);
    if (sensor.pa.exposure)
    sensor.pa.exposure.flags |= V4L2_CTRL_FLAG_VOLATILE;
    sensor.pa.gain = v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_ANALOGUE_GAIN, 1,
    511, 1, 32);
    if (sensor.pa.gain)
    sensor.pa.gain.flags |= V4L2_CTRL_FLAG_VOLATILE;
    v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_PIXEL_RATE,
    sensor.pixrate, sensor.pixrate, 1,
    sensor.pixrate);
    v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_HFLIP,
    0, 1, 1, 0);
    v4l2_ctrl_new_std(hdl, &mt9m114_pa_ctrl_ops,
    V4L2_CID_VFLIP,
    0, 1, 1, 0);
    if (hdl.error) {
    ret = hdl.error;
    goto error;
    }
    sd.state_lock = hdl.lock;
    ret = v4l2_subdev_init_finalize(sd);
    if (ret)
    goto error;
// Update the range of the blanking controls based on the format.
    state = v4l2_subdev_lock_and_get_active_state(sd);
    format = v4l2_subdev_state_get_format(state, 0);
    mt9m114_pa_ctrl_update_blanking(sensor, format);
    v4l2_subdev_unlock_state(state);
    sd.ctrl_handler = hdl;
    return 0;
    error:
    v4l2_ctrl_handler_free(&sensor.pa.hdl);
    media_entity_cleanup(&sensor.pa.sd.entity);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_pa_cleanup(sensor: *mut mt9m114) {
    static void mt9m114_pa_cleanup(struct mt9m114 *sensor)
    {
    v4l2_ctrl_handler_free(&sensor.pa.hdl);
    media_entity_cleanup(&sensor.pa.sd.entity);
    }
// -----------------------------------------------------------------------------
// Image Flow Processor Control Operations
//
    static const char * const mt9m114_test_pattern_menu[] = {
    "Disabled",
    "Solid Color",
    "100% Color Bars",
    "Pseudo-Random",
    "Fade-to-Gray Color Bars",
    "Walking Ones 10-bit",
    "Walking Ones 8-bit",
    };
// Keep in sync with mt9m114_test_pattern_menu
    static const unsigned int mt9m114_test_pattern_value[] = {
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_SOLID,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_SOLID_BARS,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_RANDOM,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_FADING_BARS,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_WALKING_1S_10B,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT_WALKING_1S_8B,
    };
    static inline struct mt9m114 *ifp_ctrl_to_mt9m114(struct v4l2_ctrl *ctrl)
    {
    return container_of(ctrl.handler, struct mt9m114, ifp.hdl);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9m114_ifp_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9m114 *sensor = ifp_ctrl_to_mt9m114(ctrl);
    u32 value;
    let mut ret: c_int = 0;
    if (ctrl.id == V4L2_CID_EXPOSURE_AUTO)
    mt9m114_pa_ctrl_update_exposure(sensor,
    ctrl.val != V4L2_EXPOSURE_AUTO);
// V4L2 controls values are applied only when power is up.
    if (!pm_runtime_get_if_in_use(&sensor.client.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_AUTO_WHITE_BALANCE:
// Control both the AWB mode and the CCM algorithm.
    if (ctrl.val)
    value = MT9M114_CAM_AWB_MODE_AUTO
    | MT9M114_CAM_AWB_MODE_EXCLUSIVE_AE;
    else
    value = 0;
    cci_write(sensor.regmap, MT9M114_CAM_AWB_AWBMODE, value, &ret);
    if (ctrl.val)
    value = MT9M114_CCM_EXEC_CALC_CCM_MATRIX | 0x22;
    else
    value = 0;
    cci_write(sensor.regmap, MT9M114_CCM_ALGO, value, &ret);
    break;
    case V4L2_CID_EXPOSURE_AUTO:
    if (ctrl.val == V4L2_EXPOSURE_AUTO)
    value = MT9M114_AE_TRACK_EXEC_AUTOMATIC_EXPOSURE
    | 0x00fe;
    else
    value = 0;
    cci_write(sensor.regmap, MT9M114_AE_TRACK_ALGO, value, &ret);
    if (ret)
    break;
    break;
    case V4L2_CID_TEST_PATTERN:
    case V4L2_CID_TEST_PATTERN_RED:
    case V4L2_CID_TEST_PATTERN_GREENR:
    case V4L2_CID_TEST_PATTERN_BLUE: {
    let mut pattern: c_uint = sensor.ifp.tpg[MT9M114_TPG_PATTERN].val;
    if (pattern) {
    cci_write(sensor.regmap, MT9M114_CAM_MODE_SELECT,
    MT9M114_CAM_MODE_SELECT_TEST_PATTERN, &ret);
    cci_write(sensor.regmap,
    MT9M114_CAM_MODE_TEST_PATTERN_SELECT,
    mt9m114_test_pattern_value[pattern - 1], &ret);
    cci_write(sensor.regmap,
    MT9M114_CAM_MODE_TEST_PATTERN_RED,
    sensor.ifp.tpg[MT9M114_TPG_RED].val, &ret);
    cci_write(sensor.regmap,
    MT9M114_CAM_MODE_TEST_PATTERN_GREEN,
    sensor.ifp.tpg[MT9M114_TPG_GREEN].val, &ret);
    cci_write(sensor.regmap,
    MT9M114_CAM_MODE_TEST_PATTERN_BLUE,
    sensor.ifp.tpg[MT9M114_TPG_BLUE].val, &ret);
    } else {
    cci_write(sensor.regmap, MT9M114_CAM_MODE_SELECT,
    MT9M114_CAM_MODE_SELECT_NORMAL, &ret);
    }
//
// A Config-Change needs to be issued for the change to take
// effect. If we're not streaming ignore this, the change will
// be applied when the stream is started.
//
    if (ret || !sensor.streaming)
    break;
    ret = mt9m114_set_state(sensor,
    MT9M114_SYS_STATE_ENTER_CONFIG_CHANGE);
    break;
    }
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put_autosuspend(&sensor.client.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops mt9m114_ifp_ctrl_ops = {
    .s_ctrl = mt9m114_ifp_s_ctrl,
    };
// -----------------------------------------------------------------------------
// Image Flow Processor Subdev Operations
//
    static inline struct mt9m114 *ifp_to_mt9m114(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct mt9m114, ifp.sd);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int mt9m114_ifp_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    struct v4l2_subdev_state *pa_state;
    struct v4l2_subdev_state *ifp_state;
    int ret;
    if (!enable)
    return mt9m114_stop_streaming(sensor);
    ifp_state = v4l2_subdev_lock_and_get_active_state(&sensor.ifp.sd);
    pa_state = v4l2_subdev_lock_and_get_active_state(&sensor.pa.sd);
    ret = mt9m114_start_streaming(sensor, pa_state, ifp_state);
    v4l2_subdev_unlock_state(pa_state);
    v4l2_subdev_unlock_state(ifp_state);
    return ret;
    }
    static int mt9m114_ifp_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *interval)
    {
    struct v4l2_fract *ival = &interval.interval;
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (interval.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    ival.numerator = 1;
    ival.denominator = sensor.ifp.frame_rate;
    return 0;
    }
    static int mt9m114_ifp_set_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *interval)
    {
    struct v4l2_fract *ival = &interval.interval;
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    let mut ret: c_int = 0;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (interval.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    if (ival.numerator != 0 && ival.denominator != 0)
    sensor.ifp.frame_rate = min_t(unsigned int,
    ival.denominator / ival.numerator,
    MT9M114_MAX_FRAME_RATE);
    else
    sensor.ifp.frame_rate = MT9M114_MAX_FRAME_RATE;
    ival.numerator = 1;
    ival.denominator = sensor.ifp.frame_rate;
    if (sensor.streaming)
    ret = mt9m114_set_frame_rate(sensor);
    return ret;
    }
    static int mt9m114_ifp_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    struct v4l2_rect *compose;
    format = v4l2_subdev_state_get_format(state, 0);
    format.width = MT9M114_PIXEL_ARRAY_WIDTH;
    format.height = MT9M114_PIXEL_ARRAY_HEIGHT;
    format.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_RAW;
    format.ycbcr_enc = V4L2_YCBCR_ENC_601;
    format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    format.xfer_func = V4L2_XFER_FUNC_NONE;
    crop = v4l2_subdev_state_get_crop(state, 0);
    crop.left = 4;
    crop.top = 4;
    crop.width = format.width - 8;
    crop.height = format.height - 8;
    compose = v4l2_subdev_state_get_compose(state, 0);
    compose.left = 0;
    compose.top = 0;
    compose.width = crop.width;
    compose.height = crop.height;
    format = v4l2_subdev_state_get_format(state, 1);
    format.width = compose.width;
    format.height = compose.height;
    format.code = mt9m114_default_format_info(sensor).code;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_SRGB;
    format.ycbcr_enc = V4L2_YCBCR_ENC_DEFAULT;
    format.quantization = V4L2_QUANTIZATION_DEFAULT;
    format.xfer_func = V4L2_XFER_FUNC_DEFAULT;
    return 0;
    }
    static int mt9m114_ifp_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    let mut num_formats: c_uint = ARRAY_SIZE(mt9m114_format_infos);
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    let mut index: c_uint = 0;
    unsigned int flag;
    unsigned int i;
    switch (code.pad) {
    case 0:
    if (code.index != 0)
    return -EINVAL;
    code.code = mt9m114_format_infos[num_formats - 1].code;
    return 0;
    case 1:
    if (sensor.bus_cfg.bus_type == V4L2_MBUS_CSI2_DPHY)
    flag = MT9M114_FMT_FLAG_CSI2;
    else
    flag = MT9M114_FMT_FLAG_PARALLEL;
    for (i = 0; i < num_formats; ++i) {
    const struct mt9m114_format_info *info =
    &mt9m114_format_infos[i];
    if (info.flags & flag) {
    if (index == code.index) {
    code.code = info.code;
    return 0;
    }
    index++;
    }
    }
    return -EINVAL;
    default:
    return -EINVAL;
    }
    }
    static int mt9m114_ifp_enum_framesizes(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    const struct mt9m114_format_info *info;
    if (fse.index > 0)
    return -EINVAL;
    info = mt9m114_format_info(sensor, fse.pad, fse.code);
    if (!info || info.code != fse.code)
    return -EINVAL;
    if (fse.pad == 0) {
    fse.min_width = MT9M114_PIXEL_ARRAY_MIN_OUTPUT_WIDTH;
    fse.max_width = MT9M114_PIXEL_ARRAY_WIDTH;
    fse.min_height = MT9M114_PIXEL_ARRAY_MIN_OUTPUT_HEIGHT;
    fse.max_height = MT9M114_PIXEL_ARRAY_HEIGHT;
    } else {
    const struct v4l2_rect *crop;
    crop = v4l2_subdev_state_get_crop(state, 0);
    fse.max_width = crop.width;
    fse.max_height = crop.height;
    fse.min_width = fse.max_width / 4;
    fse.min_height = fse.max_height / 4;
    }
    return 0;
    }
    static int mt9m114_ifp_enum_frameintervals(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_interval_enum *fie)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    const struct mt9m114_format_info *info;
    if (fie.index > 0)
    return -EINVAL;
    info = mt9m114_format_info(sensor, fie.pad, fie.code);
    if (!info || info.code != fie.code)
    return -EINVAL;
    fie.interval.numerator = 1;
    fie.interval.denominator = MT9M114_MAX_FRAME_RATE;
    return 0;
    }
//
// Helper function to update IFP crop, compose rectangles and source format
// when the pixel border size changes, which requires resetting these.
//
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_update_sel_and_src_fmt(state: *mut v4l2_subdev_state) {
    static void mt9m114_ifp_update_sel_and_src_fmt(struct v4l2_subdev_state *state)
    {
    struct v4l2_mbus_framefmt *src_format, *sink_format;
    struct v4l2_rect *crop;
    unsigned int border;
    sink_format = v4l2_subdev_state_get_format(state, 0);
    src_format = v4l2_subdev_state_get_format(state, 1);
    crop = v4l2_subdev_state_get_crop(state, 0);
    border = mt9m114_ifp_get_border(state);
    crop.left = border;
    crop.top = border;
    crop.width = sink_format.width - 2 * border;
    crop.height = sink_format.height - 2 * border;
// v4l2_subdev_state_get_compose(state, 0) = *crop;
    src_format.width = crop.width;
    src_format.height = crop.height;
    if (src_format.code == MEDIA_BUS_FMT_SGRBG10_1X10) {
    src_format.colorspace = V4L2_COLORSPACE_RAW;
    src_format.ycbcr_enc = V4L2_YCBCR_ENC_601;
    src_format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    } else {
    src_format.colorspace = V4L2_COLORSPACE_SRGB;
    src_format.ycbcr_enc = V4L2_YCBCR_ENC_DEFAULT;
    src_format.quantization = V4L2_QUANTIZATION_DEFAULT;
    }
    }
    static int mt9m114_ifp_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *fmt)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    struct v4l2_mbus_framefmt *format;
    format = v4l2_subdev_state_get_format(state, fmt.pad);
    if (fmt.pad == 0) {
// Only the size can be changed on the sink pad.
    format.width = clamp(ALIGN(fmt.format.width, 8),
    MT9M114_PIXEL_ARRAY_MIN_OUTPUT_WIDTH,
    MT9M114_PIXEL_ARRAY_WIDTH);
    format.height = clamp(ALIGN(fmt.format.height, 8),
    MT9M114_PIXEL_ARRAY_MIN_OUTPUT_HEIGHT,
    MT9M114_PIXEL_ARRAY_HEIGHT);
// Propagate changes downstream.
    mt9m114_ifp_update_sel_and_src_fmt(state);
    } else {
    const struct mt9m114_format_info *info;
// Only the media bus code can be changed on the source pad.
    info = mt9m114_format_info(sensor, 1, fmt.format.code);
//
// If the output format changes from/to RAW10 then the crop
// rectangle needs to be adjusted to add / remove the 4 pixel
// border used for demosaicing. And these changes then need to
// be propagated to the compose rectangle and source format.
//
    if ((format.code == MEDIA_BUS_FMT_SGRBG10_1X10) !=
    (info.code == MEDIA_BUS_FMT_SGRBG10_1X10)) {
    format.code = info.code;
    mt9m114_ifp_update_sel_and_src_fmt(state);
    } else {
    format.code = info.code;
    }
    }
    fmt.format = *format;
    return 0;
    }
    static int mt9m114_ifp_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    const struct v4l2_mbus_framefmt *format;
    const struct v4l2_rect *crop;
    unsigned int border;
    let mut ret: c_int = 0;
// Crop and compose are only supported on the sink pad.
    if (sel.pad != 0)
    return -EINVAL;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    sel.r = *v4l2_subdev_state_get_crop(state, 0);
    break;
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
//
// Crop defaults and bounds are equal to the sink format size.
// For source pad formats other then RAW10 this gets reduced
// by 4 pixels on each side for demosaicing.
//
    format = v4l2_subdev_state_get_format(state, 0);
    border = mt9m114_ifp_get_border(state);
    sel.r.left = border;
    sel.r.top = border;
    sel.r.width = format.width - 2 * border;
    sel.r.height = format.height - 2 * border;
    break;
    case V4L2_SEL_TGT_COMPOSE:
    sel.r = *v4l2_subdev_state_get_compose(state, 0);
    break;
    case V4L2_SEL_TGT_COMPOSE_DEFAULT:
    case V4L2_SEL_TGT_COMPOSE_BOUNDS:
//
// The compose default and bounds sizes are equal to the sink
// crop rectangle size.
//
    crop = v4l2_subdev_state_get_crop(state, 0);
    sel.r.left = 0;
    sel.r.top = 0;
    sel.r.width = crop.width;
    sel.r.height = crop.height;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static int mt9m114_ifp_set_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    struct v4l2_mbus_framefmt *format, *src_format;
    struct v4l2_rect *crop;
    struct v4l2_rect *compose;
    unsigned int border;
    if (sel.target != V4L2_SEL_TGT_CROP &&
    sel.target != V4L2_SEL_TGT_COMPOSE)
    return -EINVAL;
// Crop and compose are only supported on the sink pad.
    if (sel.pad != 0)
    return -EINVAL;
    crop = v4l2_subdev_state_get_crop(state, 0);
// Crop and compose cannot be changed when bypassing the scaler.
    src_format = v4l2_subdev_state_get_format(state, 1);
    if (src_format.code == MEDIA_BUS_FMT_SGRBG10_1X10) {
    sel.r = *crop;
    return 0;
    }
    format = v4l2_subdev_state_get_format(state, 0);
    compose = v4l2_subdev_state_get_compose(state, 0);
    if (sel.target == V4L2_SEL_TGT_CROP) {
//
// Clamp the crop rectangle. For source pad formats other then
// RAW10 demosaicing removes 4 pixels on each side of the image.
//
    border = mt9m114_ifp_get_border(state);
    crop.left = clamp_t(unsigned int, ALIGN(sel.r.left, 2), border,
    format.width - border -
    MT9M114_SCALER_CROPPED_INPUT_WIDTH);
    crop.top = clamp_t(unsigned int, ALIGN(sel.r.top, 2), border,
    format.height - border -
    MT9M114_SCALER_CROPPED_INPUT_HEIGHT);
    crop.width = clamp_t(unsigned int, ALIGN(sel.r.width, 2),
    MT9M114_SCALER_CROPPED_INPUT_WIDTH,
    format.width - border - crop.left);
    crop.height = clamp_t(unsigned int, ALIGN(sel.r.height, 2),
    MT9M114_SCALER_CROPPED_INPUT_HEIGHT,
    format.height - border - crop.top);
    sel.r = *crop;
// Propagate to the compose rectangle.
    compose.width = crop.width;
    compose.height = crop.height;
    } else {
//
// Clamp the compose rectangle. The scaler can only downscale.
//
    compose.left = 0;
    compose.top = 0;
    compose.width = clamp_t(unsigned int, ALIGN(sel.r.width, 2),
    MT9M114_SCALER_CROPPED_INPUT_WIDTH,
    crop.width);
    compose.height = clamp_t(unsigned int, ALIGN(sel.r.height, 2),
    MT9M114_SCALER_CROPPED_INPUT_HEIGHT,
    crop.height);
    sel.r = *compose;
    }
// Propagate the compose rectangle to the source format.
    src_format.width = compose.width;
    src_format.height = compose.height;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_unregistered(sd: *mut v4l2_subdev) {
    static void mt9m114_ifp_unregistered(struct v4l2_subdev *sd)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    v4l2_device_unregister_subdev(&sensor.pa.sd);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_registered(sd: *mut v4l2_subdev) -> c_int {
    static int mt9m114_ifp_registered(struct v4l2_subdev *sd)
    {
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    int ret;
    ret = v4l2_device_register_subdev(sd.v4l2_dev, &sensor.pa.sd);
    if (ret < 0) {
    dev_err(&sensor.client.dev,
    "Failed to register pixel array subdev\n");
    return ret;
    }
    ret = media_create_pad_link(&sensor.pa.sd.entity, 0,
    &sensor.ifp.sd.entity, 0,
    MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    if (ret < 0) {
    dev_err(&sensor.client.dev,
    "Failed to link pixel array to ifp\n");
    v4l2_device_unregister_subdev(&sensor.pa.sd);
    return ret;
    }
    return 0;
    }
    static const struct v4l2_subdev_video_ops mt9m114_ifp_video_ops = {
    .s_stream = mt9m114_ifp_s_stream,
    };
    static const struct v4l2_subdev_pad_ops mt9m114_ifp_pad_ops = {
    .enum_mbus_code = mt9m114_ifp_enum_mbus_code,
    .enum_frame_size = mt9m114_ifp_enum_framesizes,
    .enum_frame_interval = mt9m114_ifp_enum_frameintervals,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = mt9m114_ifp_set_fmt,
    .get_selection = mt9m114_ifp_get_selection,
    .set_selection = mt9m114_ifp_set_selection,
    .get_frame_interval = mt9m114_ifp_get_frame_interval,
    .set_frame_interval = mt9m114_ifp_set_frame_interval,
    };
    static const struct v4l2_subdev_ops mt9m114_ifp_ops = {
    .video = &mt9m114_ifp_video_ops,
    .pad = &mt9m114_ifp_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops mt9m114_ifp_internal_ops = {
    .init_state = mt9m114_ifp_init_state,
    .registered = mt9m114_ifp_registered,
    .unregistered = mt9m114_ifp_unregistered,
    };
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_init(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_ifp_init(struct mt9m114 *sensor)
    {
    struct v4l2_subdev *sd = &sensor.ifp.sd;
    struct media_pad *pads = sensor.ifp.pads;
    struct v4l2_ctrl_handler *hdl = &sensor.ifp.hdl;
    struct v4l2_ctrl *link_freq;
    int ret;
// Initialize the subdev.
    v4l2_i2c_subdev_init(sd, sensor.client, &mt9m114_ifp_ops);
    v4l2_i2c_subdev_set_name(sd, sensor.client, core::ptr::null_mut(), " ifp");
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    sd.internal_ops = &mt9m114_ifp_internal_ops;
// Initialize the media entity.
    sd.entity.function = MEDIA_ENT_F_PROC_VIDEO_ISP;
    sd.entity.ops = &mt9m114_entity_ops;
    pads[0].flags = MEDIA_PAD_FL_SINK;
    pads[1].flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&sd.entity, 2, pads);
    if (ret < 0)
    return ret;
    sensor.ifp.frame_rate = MT9M114_DEF_FRAME_RATE;
// Initialize the control handler.
    v4l2_ctrl_handler_init(hdl, 8);
    v4l2_ctrl_new_std(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_AUTO_WHITE_BALANCE,
    0, 1, 1, 1);
    v4l2_ctrl_new_std_menu(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_EXPOSURE_AUTO,
    V4L2_EXPOSURE_MANUAL, 0,
    V4L2_EXPOSURE_AUTO);
    link_freq = v4l2_ctrl_new_int_menu(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    sensor.bus_cfg.nr_of_link_frequencies - 1,
    0, sensor.bus_cfg.link_frequencies);
    if (link_freq)
    link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    v4l2_ctrl_new_std(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_PIXEL_RATE,
    sensor.pixrate, sensor.pixrate, 1,
    sensor.pixrate);
    sensor.ifp.tpg[MT9M114_TPG_PATTERN] =
    v4l2_ctrl_new_std_menu_items(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(mt9m114_test_pattern_menu) - 1,
    0, 0, mt9m114_test_pattern_menu);
    sensor.ifp.tpg[MT9M114_TPG_RED] =
    v4l2_ctrl_new_std(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_TEST_PATTERN_RED,
    0, 1023, 1, 1023);
    sensor.ifp.tpg[MT9M114_TPG_GREEN] =
    v4l2_ctrl_new_std(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_TEST_PATTERN_GREENR,
    0, 1023, 1, 1023);
    sensor.ifp.tpg[MT9M114_TPG_BLUE] =
    v4l2_ctrl_new_std(hdl, &mt9m114_ifp_ctrl_ops,
    V4L2_CID_TEST_PATTERN_BLUE,
    0, 1023, 1, 1023);
    v4l2_ctrl_cluster(ARRAY_SIZE(sensor.ifp.tpg), sensor.ifp.tpg);
    if (hdl.error) {
    ret = hdl.error;
    goto error;
    }
    sd.ctrl_handler = hdl;
    sd.state_lock = hdl.lock;
    ret = v4l2_subdev_init_finalize(sd);
    if (ret)
    goto error;
    return 0;
    error:
    v4l2_ctrl_handler_free(&sensor.ifp.hdl);
    media_entity_cleanup(&sensor.ifp.sd.entity);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_ifp_cleanup(sensor: *mut mt9m114) {
    static void mt9m114_ifp_cleanup(struct mt9m114 *sensor)
    {
    v4l2_ctrl_handler_free(&sensor.ifp.hdl);
    media_entity_cleanup(&sensor.ifp.sd.entity);
    }
// -----------------------------------------------------------------------------
// Power Management
//
#[no_mangle]
unsafe extern "C" fn mt9m114_power_on(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_power_on(struct mt9m114 *sensor)
    {
    int ret;
// Enable power and clocks.
    ret = regulator_bulk_enable(ARRAY_SIZE(sensor.supplies),
    sensor.supplies);
    if (ret < 0)
    return ret;
    ret = clk_prepare_enable(sensor.clk);
    if (ret < 0)
    goto error_regulator;
// Perform a hard reset if available, or a soft reset otherwise.
    if (sensor.reset) {
    let mut freq: c_long = clk_get_rate(sensor.clk);
    unsigned int duration;
//
// The minimum duration is 50 clock cycles, thus typically
// around 2µs. Double it to be safe.
//
    duration = DIV_ROUND_UP(2 * 50 * 1000000, freq);
    gpiod_set_value(sensor.reset, 1);
    fsleep(duration);
    gpiod_set_value(sensor.reset, 0);
    } else {
//
// The power may have just been turned on, we need to wait for
// the sensor to be ready to accept I2C commands.
//
    usleep_range(44500, 50000);
    cci_write(sensor.regmap, MT9M114_RESET_AND_MISC_CONTROL,
    MT9M114_RESET_SOC, &ret);
    cci_write(sensor.regmap, MT9M114_RESET_AND_MISC_CONTROL, 0,
    &ret);
    if (ret < 0) {
    dev_err(&sensor.client.dev, "Soft reset failed\n");
    goto error_clock;
    }
    }
//
// Wait for the sensor to be ready to accept I2C commands by polling the
// command register to wait for initialization to complete.
//
    usleep_range(44500, 50000);
    ret = mt9m114_poll_command(sensor, MT9M114_COMMAND_REGISTER_SET_STATE);
    if (ret < 0)
    goto error_clock;
    if (sensor.bus_cfg.bus_type == V4L2_MBUS_PARALLEL) {
//
// In parallel mode (OE set to low), the sensor will enter the
// streaming state after initialization. Enter the standby
// manually to stop streaming.
//
    ret = mt9m114_set_state(sensor,
    MT9M114_SYS_STATE_ENTER_STANDBY);
    if (ret < 0)
    goto error_clock;
    }
//
// Before issuing any Set-State command, we must ensure that the sensor
// reaches the standby mode (either initiated manually above in
// parallel mode, or automatically after reset in MIPI mode).
//
    if (sensor.info.state_standby_polling) {
    ret = mt9m114_poll_state(sensor, MT9M114_SYS_STATE_STANDBY);
    if (ret < 0)
    goto error_clock;
    }
    return 0;
    error_clock:
    clk_disable_unprepare(sensor.clk);
    error_regulator:
    regulator_bulk_disable(ARRAY_SIZE(sensor.supplies), sensor.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_power_off(sensor: *mut mt9m114) {
    static void mt9m114_power_off(struct mt9m114 *sensor)
    {
    unsigned int duration;
    gpiod_set_value(sensor.reset, 1);
// Power off takes 10 clock cycles. Double it to be safe.
    duration = DIV_ROUND_UP(2 * 10 * 1000000, clk_get_rate(sensor.clk));
    fsleep(duration);
    clk_disable_unprepare(sensor.clk);
    regulator_bulk_disable(ARRAY_SIZE(sensor.supplies), sensor.supplies);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt9m114_runtime_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    return mt9m114_power_on(sensor);
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt9m114_runtime_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    mt9m114_power_off(sensor);
    return 0;
    }
    static const struct dev_pm_ops mt9m114_pm_ops = {
    SET_RUNTIME_PM_OPS(mt9m114_runtime_suspend, mt9m114_runtime_resume, core::ptr::null_mut())
    };
// -----------------------------------------------------------------------------
// Probe & Remove
//
    static int mt9m114_verify_link_frequency(struct mt9m114 *sensor,
    unsigned int pixrate)
    {
    unsigned int link_freq = sensor.bus_cfg.bus_type == V4L2_MBUS_CSI2_DPHY
    ? pixrate * 8 : pixrate * 2;
    if (sensor.bus_cfg.nr_of_link_frequencies != 1 ||
    sensor.bus_cfg.link_frequencies[0] != link_freq)
    return -EINVAL;
    return 0;
    }
//
// Based on the docs the PLL is believed to have the following setup:
//
// +-----+     +-----+     +-----+     +-----+     +-----+
// Fin --> | / N | --> | x M | --> | x 2 | --> | / P | --> | / 2 | -->
// +-----+     +-----+     +-----+     +-----+     +-----+
// fBit       fWord       fSensor
// ext_clock    int_clock   out_clock                             pix_clock
//
// The MT9M114 docs give a max fBit rate of 768 MHz which translates to
// an out_clock_max of 384 MHz.
//
#[no_mangle]
unsafe extern "C" fn mt9m114_clk_init(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_clk_init(struct mt9m114 *sensor)
    {
    static const struct aptina_pll_limits limits = {
    .ext_clock_min = 6000000,
    .ext_clock_max = 54000000,
// int_clock_* limits are not documented taken from mt9p031.c
    .int_clock_min = 2000000,
    .int_clock_max = 13500000,
// out_clock_min is not documented, taken from mt9p031.c
    .out_clock_min = 180000000,
    .out_clock_max = 384000000,
    .pix_clock_max = 48000000,
    .n_min = 1,
    .n_max = 64,
    .m_min = 16,
    .m_max = 192,
    .p1_min = 8,
    .p1_max = 8,
    };
    unsigned int pixrate;
    int ret;
//
// Calculate the pixel rate and link frequency. The CSI-2 bus is clocked
// for 16-bit per pixel, transmitted in DDR over a single lane. For
// parallel mode, the sensor ouputs one pixel in two PIXCLK cycles.
//
// Check if EXTCLK fits the configured link frequency. Bypass the PLL
// in this case.
//
    pixrate = clk_get_rate(sensor.clk) / 2;
    if (mt9m114_verify_link_frequency(sensor, pixrate) == 0) {
    sensor.pixrate = pixrate;
    sensor.bypass_pll = true;
    return 0;
    }
// Check if the PLL configuration fits the configured link frequency.
    sensor.pll.ext_clock = clk_get_rate(sensor.clk);
    sensor.pll.pix_clock = MT9M114_DEF_PIXCLOCK;
    ret = aptina_pll_calculate(&sensor.client.dev, &limits, &sensor.pll);
    if (ret)
    return ret;
    pixrate = sensor.pll.ext_clock * sensor.pll.m
    / (sensor.pll.n * sensor.pll.p1);
    if (mt9m114_verify_link_frequency(sensor, pixrate) == 0) {
    sensor.pixrate = pixrate;
    sensor.bypass_pll = false;
    return 0;
    }
    dev_err(&sensor.client.dev, "Unsupported DT link-frequencies\n");
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_identify(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_identify(struct mt9m114 *sensor)
    {
    u64 major, minor, release, customer;
    u64 value;
    int ret;
    ret = cci_read(sensor.regmap, MT9M114_CHIP_ID, &value, core::ptr::null_mut());
    if (ret) {
    dev_err(&sensor.client.dev, "Failed to read chip ID\n");
    return -ENXIO;
    }
    if (value != 0x2481) {
    dev_err(&sensor.client.dev, "Invalid chip ID 0x%04llx\n",
    value);
    return -ENXIO;
    }
    cci_read(sensor.regmap, MT9M114_MON_MAJOR_VERSION, &major, &ret);
    cci_read(sensor.regmap, MT9M114_MON_MINOR_VERSION, &minor, &ret);
    cci_read(sensor.regmap, MT9M114_MON_RELEASE_VERSION, &release, &ret);
    cci_read(sensor.regmap, MT9M114_CUSTOMER_REV, &customer, &ret);
    if (ret) {
    dev_err(&sensor.client.dev, "Failed to read version\n");
    return -ENXIO;
    }
    dev_dbg(&sensor.client.dev,
    "monitor v%llu.%llu.%04llx customer rev 0x%04llx\n",
    major, minor, release, customer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_parse_dt(sensor: *mut mt9m114) -> c_int {
    static int mt9m114_parse_dt(struct mt9m114 *sensor)
    {
    struct fwnode_handle *fwnode = dev_fwnode(&sensor.client.dev);
    struct fwnode_handle *ep;
    int ret;
//
// On ACPI systems the fwnode graph can be initialized by a bridge
// driver, which may not have probed yet. Wait for this.
//
// TODO: Return an error once bridge driver code will have moved
// to the ACPI core.
//
    ep = fwnode_graph_get_next_endpoint(fwnode, core::ptr::null_mut());
    if (!ep)
    return dev_err_probe(&sensor.client.dev, -EPROBE_DEFER,
    "waiting for fwnode graph endpoint\n");
    sensor.bus_cfg.bus_type = V4L2_MBUS_UNKNOWN;
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &sensor.bus_cfg);
    fwnode_handle_put(ep);
    if (ret < 0) {
    dev_err(&sensor.client.dev, "Failed to parse endpoint\n");
    goto error;
    }
    switch (sensor.bus_cfg.bus_type) {
    case V4L2_MBUS_CSI2_DPHY:
    case V4L2_MBUS_PARALLEL:
    break;
    default:
    dev_err(&sensor.client.dev, "unsupported bus type %u\n",
    sensor.bus_cfg.bus_type);
    ret = -EINVAL;
    goto error;
    }
    sensor.pad_slew_rate = MT9M114_PAD_SLEW_DEFAULT;
    device_property_read_u32(&sensor.client.dev, "slew-rate",
    &sensor.pad_slew_rate);
    if (sensor.pad_slew_rate < MT9M114_PAD_SLEW_MIN ||
    sensor.pad_slew_rate > MT9M114_PAD_SLEW_MAX) {
    dev_err(&sensor.client.dev, "Invalid slew-rate %u\n",
    sensor.pad_slew_rate);
    return -EINVAL;
    }
    return 0;
    error:
    v4l2_fwnode_endpoint_free(&sensor.bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_probe(client: *mut i2c_client) -> c_int {
    static int mt9m114_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct mt9m114 *sensor;
    int ret;
    sensor = devm_kzalloc(dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.client = client;
    sensor.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(sensor.regmap)) {
    dev_err(dev, "Unable to initialize I2C\n");
    return -ENODEV;
    }
    ret = mt9m114_parse_dt(sensor);
    if (ret < 0)
    return ret;
    sensor.info = device_get_match_data(dev);
    if (!sensor.info)
    return -ENODEV;
// Acquire clocks, GPIOs and regulators.
    sensor.clk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(sensor.clk)) {
    ret = dev_err_probe(dev, PTR_ERR(sensor.clk),
    "Failed to get clock\n");
    goto error_ep_free;
    }
    sensor.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(sensor.reset)) {
    ret = PTR_ERR(sensor.reset);
    dev_err_probe(dev, ret, "Failed to get reset GPIO\n");
    goto error_ep_free;
    }
    sensor.supplies[0].supply = "vddio";
    sensor.supplies[1].supply = "vdd";
    sensor.supplies[2].supply = "vaa";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(sensor.supplies),
    sensor.supplies);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Failed to get regulators\n");
    goto error_ep_free;
    }
    ret = mt9m114_clk_init(sensor);
    if (ret)
    goto error_ep_free;
//
// Identify the sensor. The driver supports runtime PM, but needs to
// work when runtime PM is disabled in the kernel. To that end, power
// the sensor on manually here to reach the same state as if resumed
// through runtime PM.
//
    ret = mt9m114_power_on(sensor);
    if (ret < 0) {
    dev_err_probe(dev, ret, "Could not power on the device\n");
    goto error_ep_free;
    }
    ret = mt9m114_identify(sensor);
    if (ret < 0)
    goto error_power_off;
//
// Enable runtime PM with autosuspend. As the device has been powered
// manually, mark it as active, and increase the usage count without
// resuming the device.
//
    pm_runtime_set_active(dev);
    pm_runtime_get_noresume(dev);
    pm_runtime_enable(dev);
    pm_runtime_set_autosuspend_delay(dev, 1000);
    pm_runtime_use_autosuspend(dev);
// Initialize the subdevices.
    ret = mt9m114_pa_init(sensor);
    if (ret < 0)
    goto error_pm_cleanup;
    ret = mt9m114_ifp_init(sensor);
    if (ret < 0)
    goto error_pa_cleanup;
    ret = v4l2_async_register_subdev(&sensor.ifp.sd);
    if (ret < 0)
    goto error_ifp_cleanup;
//
// Decrease the PM usage count. The device will get suspended after the
// autosuspend delay, turning the power off.
//
    pm_runtime_put_autosuspend(dev);
    return 0;
    error_ifp_cleanup:
    mt9m114_ifp_cleanup(sensor);
    error_pa_cleanup:
    mt9m114_pa_cleanup(sensor);
    error_pm_cleanup:
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    error_power_off:
    mt9m114_power_off(sensor);
    error_ep_free:
    v4l2_fwnode_endpoint_free(&sensor.bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9m114_remove(client: *mut i2c_client) {
    static void mt9m114_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct mt9m114 *sensor = ifp_to_mt9m114(sd);
    struct device *dev = &client.dev;
    v4l2_async_unregister_subdev(&sensor.ifp.sd);
    mt9m114_ifp_cleanup(sensor);
    mt9m114_pa_cleanup(sensor);
    v4l2_fwnode_endpoint_free(&sensor.bus_cfg);
//
// Disable runtime PM. In case runtime PM is disabled in the kernel,
// make sure to turn power off manually.
//
    pm_runtime_disable(dev);
    if (!pm_runtime_status_suspended(dev))
    mt9m114_power_off(sensor);
    pm_runtime_set_suspended(dev);
    }
    static const struct mt9m114_model_info mt9m114_models_default = {
    .state_standby_polling = true,
    };
    static const struct mt9m114_model_info mt9m114_models_aptina = {
    .state_standby_polling = false,
    };
    static const struct of_device_id mt9m114_of_ids[] = {
    { .compatible = "onnn,mt9m114", .data = &mt9m114_models_default },
    { .compatible = "aptina,mi1040", .data = &mt9m114_models_aptina },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mt9m114_of_ids);
    static const struct acpi_device_id mt9m114_acpi_ids[] = {
    { "INT33F0", (kernel_ulong_t)&mt9m114_models_default },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, mt9m114_acpi_ids);
    static struct i2c_driver mt9m114_driver = {
    .driver = {
    .name	= "mt9m114",
    .pm	= &mt9m114_pm_ops,
    .of_match_table = mt9m114_of_ids,
    .acpi_match_table = mt9m114_acpi_ids,
    },
    .probe		= mt9m114_probe,
    .remove		= mt9m114_remove,
    };
    module_i2c_driver(mt9m114_driver);
    MODULE_DESCRIPTION("onsemi MT9M114 Sensor Driver");
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_LICENSE("GPL");
