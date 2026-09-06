//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/mt9p031.c
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
// Driver for MT9P031 CMOS Image Sensor from Aptina
//
// Copyright (C) 2011, Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Copyright (C) 2011, Javier Martin <javier.martin@vista-silicon.com>
// Copyright (C) 2011, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//
// Based on the MT9V032 driver and Bastian Hecht's code.
//

pub const MT9P031_PIXEL_ARRAY_WIDTH: c_int = 2752;
pub const MT9P031_PIXEL_ARRAY_HEIGHT: c_int = 2004;
pub const MT9P031_CHIP_VERSION: c_uint = 0x00;
pub const MT9P031_CHIP_VERSION_VALUE: c_uint = 0x1801;
pub const MT9P031_ROW_START: c_uint = 0x01;
pub const MT9P031_ROW_START_MIN: c_int = 0;
pub const MT9P031_ROW_START_MAX: c_int = 2004;
pub const MT9P031_ROW_START_DEF: c_int = 54;
pub const MT9P031_COLUMN_START: c_uint = 0x02;
pub const MT9P031_COLUMN_START_MIN: c_int = 0;
pub const MT9P031_COLUMN_START_MAX: c_int = 2750;
pub const MT9P031_COLUMN_START_DEF: c_int = 16;
pub const MT9P031_WINDOW_HEIGHT: c_uint = 0x03;
pub const MT9P031_WINDOW_HEIGHT_MIN: c_int = 2;
pub const MT9P031_WINDOW_HEIGHT_MAX: c_int = 2006;
pub const MT9P031_WINDOW_HEIGHT_DEF: c_int = 1944;
pub const MT9P031_WINDOW_WIDTH: c_uint = 0x04;
pub const MT9P031_WINDOW_WIDTH_MIN: c_int = 2;
pub const MT9P031_WINDOW_WIDTH_MAX: c_int = 2752;
pub const MT9P031_WINDOW_WIDTH_DEF: c_int = 2592;
pub const MT9P031_HORIZONTAL_BLANK: c_uint = 0x05;
pub const MT9P031_HORIZONTAL_BLANK_MIN: c_int = 0;
pub const MT9P031_HORIZONTAL_BLANK_MAX: c_int = 4095;
pub const MT9P031_VERTICAL_BLANK: c_uint = 0x06;
pub const MT9P031_VERTICAL_BLANK_MIN: c_int = 1;
pub const MT9P031_VERTICAL_BLANK_MAX: c_int = 4096;
pub const MT9P031_VERTICAL_BLANK_DEF: c_int = 26;
pub const MT9P031_OUTPUT_CONTROL: c_uint = 0x07;
pub const MT9P031_OUTPUT_CONTROL_CEN: c_int = 2;
pub const MT9P031_OUTPUT_CONTROL_SYN: c_int = 1;
pub const MT9P031_OUTPUT_CONTROL_DEF: c_uint = 0x1f82;
pub const MT9P031_SHUTTER_WIDTH_UPPER: c_uint = 0x08;
pub const MT9P031_SHUTTER_WIDTH_LOWER: c_uint = 0x09;
pub const MT9P031_SHUTTER_WIDTH_MIN: c_int = 1;
pub const MT9P031_SHUTTER_WIDTH_MAX: c_int = 1048575;
pub const MT9P031_SHUTTER_WIDTH_DEF: c_int = 1943;
pub const MT9P031_PLL_CONTROL: c_uint = 0x10;
pub const MT9P031_PLL_CONTROL_PWROFF: c_uint = 0x0050;
pub const MT9P031_PLL_CONTROL_PWRON: c_uint = 0x0051;
pub const MT9P031_PLL_CONTROL_USEPLL: c_uint = 0x0052;
pub const MT9P031_PLL_CONFIG_1: c_uint = 0x11;
pub const MT9P031_PLL_CONFIG_2: c_uint = 0x12;
pub const MT9P031_PIXEL_CLOCK_CONTROL: c_uint = 0x0a;

pub const MT9P031_RESTART: c_uint = 0x0b;

pub const MT9P031_SHUTTER_DELAY: c_uint = 0x0c;
pub const MT9P031_RST: c_uint = 0x0d;

pub const MT9P031_READ_MODE_1: c_uint = 0x1e;
pub const MT9P031_READ_MODE_2: c_uint = 0x20;

pub const MT9P031_ROW_ADDRESS_MODE: c_uint = 0x22;
pub const MT9P031_COLUMN_ADDRESS_MODE: c_uint = 0x23;
pub const MT9P031_GLOBAL_GAIN: c_uint = 0x35;
pub const MT9P031_GLOBAL_GAIN_MIN: c_int = 8;
pub const MT9P031_GLOBAL_GAIN_MAX: c_int = 1024;
pub const MT9P031_GLOBAL_GAIN_DEF: c_int = 8;

pub const MT9P031_ROW_BLACK_TARGET: c_uint = 0x49;
pub const MT9P031_ROW_BLACK_DEF_OFFSET: c_uint = 0x4b;
pub const MT9P031_GREEN1_OFFSET: c_uint = 0x60;
pub const MT9P031_GREEN2_OFFSET: c_uint = 0x61;
pub const MT9P031_BLACK_LEVEL_CALIBRATION: c_uint = 0x62;

pub const MT9P031_RED_OFFSET: c_uint = 0x63;
pub const MT9P031_BLUE_OFFSET: c_uint = 0x64;
pub const MT9P031_TEST_PATTERN: c_uint = 0xa0;
pub const MT9P031_TEST_PATTERN_SHIFT: c_int = 3;

pub const MT9P031_TEST_PATTERN_GREEN: c_uint = 0xa1;
pub const MT9P031_TEST_PATTERN_RED: c_uint = 0xa2;
pub const MT9P031_TEST_PATTERN_BLUE: c_uint = 0xa3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9p031_model_info {
    pub code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9p031 {
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub /: *mut *mut v4l2_rect crop; / Sensor window,
    pub format: v4l2_mbus_framefmt,
    pub /: *mut *mut mutex power_lock; / lock to protect power_count,
    pub power_count: c_int,
    pub clk: *mut clk,
    pub regulators: [regulator_bulk_data; 3],
    pub pixclk_pol:1: c_uint,
    pub ext_freq: c_int,
    pub target_freq: c_int,
    pub code: u32,
    pub pll: aptina_pll,
    pub clk_div: c_uint,
    pub use_pll: bool,
    pub reset: *mut gpio_desc,
    pub ctrls: v4l2_ctrl_handler,
    pub blc_auto: *mut v4l2_ctrl,
    pub blc_offset: *mut v4l2_ctrl,
// Registers cache
    pub output_control: u16,
    pub mode2: u16,
}

    static struct mt9p031 *to_mt9p031(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct mt9p031, subdev);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_read(client: *mut i2c_client, reg: u8) -> c_int {
    static int mt9p031_read(struct i2c_client *client, u8 reg)
    {
    return i2c_smbus_read_word_swapped(client, reg);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_write(client: *mut i2c_client, reg: u8, data: u16) -> c_int {
    static int mt9p031_write(struct i2c_client *client, u8 reg, u16 data)
    {
    return i2c_smbus_write_word_swapped(client, reg, data);
    }
    static int mt9p031_set_output_control(struct mt9p031 *mt9p031, u16 clear,
    u16 set)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    let mut value: u16 = (mt9p031.output_control & ~clear) | set;
    int ret;
    ret = mt9p031_write(client, MT9P031_OUTPUT_CONTROL, value);
    if (ret < 0)
    return ret;
    mt9p031.output_control = value;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_set_mode2(mt9p031: *mut mt9p031, clear: u16, set: u16) -> c_int {
    static int mt9p031_set_mode2(struct mt9p031 *mt9p031, u16 clear, u16 set)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    let mut value: u16 = (mt9p031.mode2 & ~clear) | set;
    int ret;
    ret = mt9p031_write(client, MT9P031_READ_MODE_2, value);
    if (ret < 0)
    return ret;
    mt9p031.mode2 = value;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_reset(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_reset(struct mt9p031 *mt9p031)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    int ret;
// Disable chip output, synchronous option update
    ret = mt9p031_write(client, MT9P031_RST, MT9P031_RST_ENABLE);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_RST, 0);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_PIXEL_CLOCK_CONTROL,
    MT9P031_PIXEL_CLOCK_DIVIDE(mt9p031.clk_div));
    if (ret < 0)
    return ret;
    return mt9p031_set_output_control(mt9p031, MT9P031_OUTPUT_CONTROL_CEN,
    0);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_clk_setup(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_clk_setup(struct mt9p031 *mt9p031)
    {
    static const struct aptina_pll_limits limits = {
    .ext_clock_min = 6000000,
    .ext_clock_max = 27000000,
    .int_clock_min = 2000000,
    .int_clock_max = 13500000,
    .out_clock_min = 180000000,
    .out_clock_max = 360000000,
    .pix_clock_max = 96000000,
    .n_min = 1,
    .n_max = 64,
    .m_min = 16,
    .m_max = 255,
    .p1_min = 1,
    .p1_max = 128,
    };
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    unsigned long ext_freq;
    int ret;
    mt9p031.clk = devm_v4l2_sensor_clk_get(&client.dev, core::ptr::null_mut());
    if (IS_ERR(mt9p031.clk))
    return dev_err_probe(&client.dev, PTR_ERR(mt9p031.clk),
    "failed to get the clock\n");
    ret = clk_set_rate(mt9p031.clk, mt9p031.ext_freq);
    if (ret < 0)
    return ret;
    ext_freq = clk_get_rate(mt9p031.clk);
// If the external clock frequency is out of bounds for the PLL use the
// pixel clock divider only and disable the PLL.
//
    if (ext_freq > limits.ext_clock_max) {
    unsigned int div;
    div = DIV_ROUND_UP(ext_freq, mt9p031.target_freq);
    div = roundup_pow_of_two(div) / 2;
    mt9p031.clk_div = min_t(unsigned int, div, 64);
    mt9p031.use_pll = false;
    return 0;
    }
    mt9p031.pll.ext_clock = ext_freq;
    mt9p031.pll.pix_clock = mt9p031.target_freq;
    mt9p031.use_pll = true;
    return aptina_pll_calculate(&client.dev, &limits, &mt9p031.pll);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_pll_enable(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_pll_enable(struct mt9p031 *mt9p031)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    int ret;
    if (!mt9p031.use_pll)
    return 0;
    ret = mt9p031_write(client, MT9P031_PLL_CONTROL,
    MT9P031_PLL_CONTROL_PWRON);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_PLL_CONFIG_1,
    (mt9p031.pll.m << 8) | (mt9p031.pll.n - 1));
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_PLL_CONFIG_2, mt9p031.pll.p1 - 1);
    if (ret < 0)
    return ret;
    usleep_range(1000, 2000);
    ret = mt9p031_write(client, MT9P031_PLL_CONTROL,
    MT9P031_PLL_CONTROL_PWRON |
    MT9P031_PLL_CONTROL_USEPLL);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mt9p031_pll_disable(mt9p031: *mut mt9p031) -> c_int {
    static inline int mt9p031_pll_disable(struct mt9p031 *mt9p031)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    if (!mt9p031.use_pll)
    return 0;
    return mt9p031_write(client, MT9P031_PLL_CONTROL,
    MT9P031_PLL_CONTROL_PWROFF);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_power_on(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_power_on(struct mt9p031 *mt9p031)
    {
    unsigned long rate, delay;
    int ret;
// Ensure RESET_BAR is active
    if (mt9p031.reset) {
    gpiod_set_value(mt9p031.reset, 1);
    usleep_range(1000, 2000);
    }
// Bring up the supplies
    ret = regulator_bulk_enable(ARRAY_SIZE(mt9p031.regulators),
    mt9p031.regulators);
    if (ret < 0)
    return ret;
// Enable clock
    if (mt9p031.clk) {
    ret = clk_prepare_enable(mt9p031.clk);
    if (ret) {
    regulator_bulk_disable(ARRAY_SIZE(mt9p031.regulators),
    mt9p031.regulators);
    return ret;
    }
    }
// Now RESET_BAR must be high
    if (mt9p031.reset) {
    gpiod_set_value(mt9p031.reset, 0);
// Wait 850000 EXTCLK cycles before de-asserting reset.
    rate = clk_get_rate(mt9p031.clk);
    if (!rate)
    rate = 6000000;	/* Slowest supported clock, 6 MHz */
    delay = DIV_ROUND_UP(850000 * 1000, rate);
    msleep(delay);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_power_off(mt9p031: *mut mt9p031) {
    static void mt9p031_power_off(struct mt9p031 *mt9p031)
    {
    if (mt9p031.reset) {
    gpiod_set_value(mt9p031.reset, 1);
    usleep_range(1000, 2000);
    }
    regulator_bulk_disable(ARRAY_SIZE(mt9p031.regulators),
    mt9p031.regulators);
    clk_disable_unprepare(mt9p031.clk);
    }
#[no_mangle]
unsafe extern "C" fn __mt9p031_set_power(mt9p031: *mut mt9p031, on: bool) -> c_int {
    static int __mt9p031_set_power(struct mt9p031 *mt9p031, bool on)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    int ret;
    if (!on) {
    mt9p031_power_off(mt9p031);
    return 0;
    }
    ret = mt9p031_power_on(mt9p031);
    if (ret < 0)
    return ret;
    ret = mt9p031_reset(mt9p031);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to reset the camera\n");
    return ret;
    }
// Configure the pixel clock polarity
    if (mt9p031.pixclk_pol) {
    ret = mt9p031_write(client, MT9P031_PIXEL_CLOCK_CONTROL,
    MT9P031_PIXEL_CLOCK_INVERT);
    if (ret < 0)
    return ret;
    }
    return v4l2_ctrl_handler_setup(&mt9p031.ctrls);
    }
// -----------------------------------------------------------------------------
// V4L2 subdev video operations
//
#[no_mangle]
unsafe extern "C" fn mt9p031_set_params(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_set_params(struct mt9p031 *mt9p031)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    struct v4l2_mbus_framefmt *format = &mt9p031.format;
    const struct v4l2_rect *crop = &mt9p031.crop;
    unsigned int hblank;
    unsigned int vblank;
    unsigned int xskip;
    unsigned int yskip;
    unsigned int xbin;
    unsigned int ybin;
    int ret;
// Windows position and size.
//
// TODO: Make sure the start coordinates and window size match the
// skipping, binning and mirroring (see description of registers 2 and 4
// in table 13, and Binning section on page 41).
//
    ret = mt9p031_write(client, MT9P031_COLUMN_START, crop.left);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_ROW_START, crop.top);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_WINDOW_WIDTH, crop.width - 1);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_WINDOW_HEIGHT, crop.height - 1);
    if (ret < 0)
    return ret;
// Row and column binning and skipping. Use the maximum binning value
// compatible with the skipping settings.
//
    xskip = DIV_ROUND_CLOSEST(crop.width, format.width);
    yskip = DIV_ROUND_CLOSEST(crop.height, format.height);
    xbin = 1 << (ffs(xskip) - 1);
    ybin = 1 << (ffs(yskip) - 1);
    ret = mt9p031_write(client, MT9P031_COLUMN_ADDRESS_MODE,
    ((xbin - 1) << 4) | (xskip - 1));
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_ROW_ADDRESS_MODE,
    ((ybin - 1) << 4) | (yskip - 1));
    if (ret < 0)
    return ret;
// Blanking - use minimum value for horizontal blanking and default
// value for vertical blanking.
//
    hblank = 346 * ybin + 64 + (80 >> min_t(unsigned int, xbin, 3));
    vblank = MT9P031_VERTICAL_BLANK_DEF;
    ret = mt9p031_write(client, MT9P031_HORIZONTAL_BLANK, hblank - 1);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_VERTICAL_BLANK, vblank - 1);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_s_stream(subdev: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int mt9p031_s_stream(struct v4l2_subdev *subdev, int enable)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    struct i2c_client *client = v4l2_get_subdevdata(subdev);
    int val;
    int ret;
    if (!enable) {
// enable pause restart
    val = MT9P031_FRAME_PAUSE_RESTART;
    ret = mt9p031_write(client, MT9P031_RESTART, val);
    if (ret < 0)
    return ret;
// enable restart + keep pause restart set
    val |= MT9P031_FRAME_RESTART;
    ret = mt9p031_write(client, MT9P031_RESTART, val);
    if (ret < 0)
    return ret;
// Stop sensor readout
    ret = mt9p031_set_output_control(mt9p031,
    MT9P031_OUTPUT_CONTROL_CEN, 0);
    if (ret < 0)
    return ret;
    return mt9p031_pll_disable(mt9p031);
    }
    ret = mt9p031_set_params(mt9p031);
    if (ret < 0)
    return ret;
// Switch to master "normal" mode
    ret = mt9p031_set_output_control(mt9p031, 0,
    MT9P031_OUTPUT_CONTROL_CEN);
    if (ret < 0)
    return ret;
//
// - clear pause restart
// - don't clear restart as clearing restart manually can cause
// undefined behavior
//
    val = MT9P031_FRAME_RESTART;
    ret = mt9p031_write(client, MT9P031_RESTART, val);
    if (ret < 0)
    return ret;
    return mt9p031_pll_enable(mt9p031);
    }
    static int mt9p031_enum_mbus_code(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    if (code.pad || code.index)
    return -EINVAL;
    code.code = mt9p031.format.code;
    return 0;
    }
    static int mt9p031_enum_frame_size(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    if (fse.index >= 8 || fse.code != mt9p031.format.code)
    return -EINVAL;
    fse.min_width = MT9P031_WINDOW_WIDTH_DEF
    / min_t(unsigned int, 7, fse.index + 1);
    fse.max_width = fse.min_width;
    fse.min_height = MT9P031_WINDOW_HEIGHT_DEF / (fse.index + 1);
    fse.max_height = fse.min_height;
    return 0;
    }
    static struct v4l2_mbus_framefmt *
    __mt9p031_get_pad_format(struct mt9p031 *mt9p031,
    struct v4l2_subdev_state *sd_state,
    unsigned int pad, u32 which)
    {
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
    return v4l2_subdev_state_get_format(sd_state, pad);
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    return &mt9p031.format;
    default:
    return core::ptr::null_mut();
    }
    }
    static struct v4l2_rect *
    __mt9p031_get_pad_crop(struct mt9p031 *mt9p031,
    struct v4l2_subdev_state *sd_state,
    unsigned int pad, u32 which)
    {
    switch (which) {
    case V4L2_SUBDEV_FORMAT_TRY:
    return v4l2_subdev_state_get_crop(sd_state, pad);
    case V4L2_SUBDEV_FORMAT_ACTIVE:
    return &mt9p031.crop;
    default:
    return core::ptr::null_mut();
    }
    }
    static int mt9p031_get_format(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    fmt.format = *__mt9p031_get_pad_format(mt9p031, sd_state, fmt.pad,
    fmt.which);
    return 0;
    }
    static int mt9p031_set_format(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    struct v4l2_mbus_framefmt *__format;
    struct v4l2_rect *__crop;
    unsigned int width;
    unsigned int height;
    unsigned int hratio;
    unsigned int vratio;
    __crop = __mt9p031_get_pad_crop(mt9p031, sd_state, format.pad,
    format.which);
// Clamp the width and height to avoid dividing by zero.
    width = clamp_t(unsigned int, ALIGN(format.format.width, 2),
    max_t(unsigned int, __crop.width / 7,
    MT9P031_WINDOW_WIDTH_MIN),
    __crop.width);
    height = clamp_t(unsigned int, ALIGN(format.format.height, 2),
    max_t(unsigned int, __crop.height / 8,
    MT9P031_WINDOW_HEIGHT_MIN),
    __crop.height);
    hratio = DIV_ROUND_CLOSEST(__crop.width, width);
    vratio = DIV_ROUND_CLOSEST(__crop.height, height);
    __format = __mt9p031_get_pad_format(mt9p031, sd_state, format.pad,
    format.which);
    __format.width = __crop.width / hratio;
    __format.height = __crop.height / vratio;
    format.format = *__format;
    return 0;
    }
    static int mt9p031_get_selection(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.left = MT9P031_COLUMN_START_MIN;
    sel.r.top = MT9P031_ROW_START_MIN;
    sel.r.width = MT9P031_WINDOW_WIDTH_MAX;
    sel.r.height = MT9P031_WINDOW_HEIGHT_MAX;
    return 0;
    case V4L2_SEL_TGT_CROP:
    sel.r = *__mt9p031_get_pad_crop(mt9p031, sd_state,
    sel.pad, sel.which);
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int mt9p031_set_selection(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    struct v4l2_mbus_framefmt *__format;
    struct v4l2_rect *__crop;
    struct v4l2_rect rect;
    if (sel.target != V4L2_SEL_TGT_CROP)
    return -EINVAL;
// Clamp the crop rectangle boundaries and align them to a multiple of 2
// pixels to ensure a GRBG Bayer pattern.
//
    rect.left = clamp(ALIGN(sel.r.left, 2), MT9P031_COLUMN_START_MIN,
    MT9P031_COLUMN_START_MAX);
    rect.top = clamp(ALIGN(sel.r.top, 2), MT9P031_ROW_START_MIN,
    MT9P031_ROW_START_MAX);
    rect.width = clamp_t(unsigned int, ALIGN(sel.r.width, 2),
    MT9P031_WINDOW_WIDTH_MIN,
    MT9P031_WINDOW_WIDTH_MAX);
    rect.height = clamp_t(unsigned int, ALIGN(sel.r.height, 2),
    MT9P031_WINDOW_HEIGHT_MIN,
    MT9P031_WINDOW_HEIGHT_MAX);
    rect.width = min_t(unsigned int, rect.width,
    MT9P031_PIXEL_ARRAY_WIDTH - rect.left);
    rect.height = min_t(unsigned int, rect.height,
    MT9P031_PIXEL_ARRAY_HEIGHT - rect.top);
    __crop = __mt9p031_get_pad_crop(mt9p031, sd_state, sel.pad,
    sel.which);
    if (rect.width != __crop.width || rect.height != __crop.height) {
// Reset the output image size if the crop rectangle size has
// been modified.
//
    __format = __mt9p031_get_pad_format(mt9p031, sd_state,
    sel.pad,
    sel.which);
    __format.width = rect.width;
    __format.height = rect.height;
    }
// __crop = rect;
    sel.r = rect;
    return 0;
    }
    static int mt9p031_init_state(struct v4l2_subdev *subdev,
    struct v4l2_subdev_state *sd_state)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    struct v4l2_mbus_framefmt *format;
    struct v4l2_rect *crop;
    const int which = sd_state == core::ptr::null_mut() ? V4L2_SUBDEV_FORMAT_ACTIVE :
    V4L2_SUBDEV_FORMAT_TRY;
    crop = __mt9p031_get_pad_crop(mt9p031, sd_state, 0, which);
    crop.left = MT9P031_COLUMN_START_DEF;
    crop.top = MT9P031_ROW_START_DEF;
    crop.width = MT9P031_WINDOW_WIDTH_DEF;
    crop.height = MT9P031_WINDOW_HEIGHT_DEF;
    format = __mt9p031_get_pad_format(mt9p031, sd_state, 0, which);
    format.code = mt9p031.code;
    format.width = MT9P031_WINDOW_WIDTH_DEF;
    format.height = MT9P031_WINDOW_HEIGHT_DEF;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_SRGB;
    return 0;
    }
// -----------------------------------------------------------------------------
// V4L2 subdev control operations
//

#[no_mangle]
unsafe extern "C" fn mt9p031_restore_blc(mt9p031: *mut mt9p031) -> c_int {
    static int mt9p031_restore_blc(struct mt9p031 *mt9p031)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    int ret;
    if (mt9p031.blc_auto.cur.val != 0) {
    ret = mt9p031_set_mode2(mt9p031, 0,
    MT9P031_READ_MODE_2_ROW_BLC);
    if (ret < 0)
    return ret;
    }
    if (mt9p031.blc_offset.cur.val != 0) {
    ret = mt9p031_write(client, MT9P031_ROW_BLACK_TARGET,
    mt9p031.blc_offset.cur.val);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9p031_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9p031 *mt9p031 =
    container_of(ctrl.handler, struct mt9p031, ctrls);
    struct i2c_client *client = v4l2_get_subdevdata(&mt9p031.subdev);
    u16 data;
    int ret;
    if (ctrl.flags & V4L2_CTRL_FLAG_INACTIVE)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_EXPOSURE:
    ret = mt9p031_write(client, MT9P031_SHUTTER_WIDTH_UPPER,
    (ctrl.val >> 16) & 0xffff);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_SHUTTER_WIDTH_LOWER,
    ctrl.val & 0xffff);
    case V4L2_CID_GAIN:
// Gain is controlled by 2 analog stages and a digital stage.
// Valid values for the 3 stages are
//
// Stage                Min     Max     Step
// ------------------------------------------
// First analog stage   x1      x2      1
// Second analog stage  x1      x4      0.125
// Digital stage        x1      x16     0.125
//
// To minimize noise, the gain stages should be used in the
// second analog stage, first analog stage, digital stage order.
// Gain from a previous stage should be pushed to its maximum
// value before the next stage is used.
//
    if (ctrl.val <= 32) {
    data = ctrl.val;
    } else if (ctrl.val <= 64) {
    ctrl.val &= ~1;
    data = (1 << 6) | (ctrl.val >> 1);
    } else {
    ctrl.val &= ~7;
    data = ((ctrl.val - 64) >> 3) & 0x7f;
    data = (data << 8) | (1 << 6) | 32;
    }
    return mt9p031_write(client, MT9P031_GLOBAL_GAIN, data);
    case V4L2_CID_HFLIP:
    if (ctrl.val)
    return mt9p031_set_mode2(mt9p031,
    0, MT9P031_READ_MODE_2_COL_MIR);
    else
    return mt9p031_set_mode2(mt9p031,
    MT9P031_READ_MODE_2_COL_MIR, 0);
    case V4L2_CID_VFLIP:
    if (ctrl.val)
    return mt9p031_set_mode2(mt9p031,
    0, MT9P031_READ_MODE_2_ROW_MIR);
    else
    return mt9p031_set_mode2(mt9p031,
    MT9P031_READ_MODE_2_ROW_MIR, 0);
    case V4L2_CID_TEST_PATTERN:
// The digital side of the Black Level Calibration function must
// be disabled when generating a test pattern to avoid artifacts
// in the image. Activate (deactivate) the BLC-related controls
// when the test pattern is enabled (disabled).
//
    v4l2_ctrl_activate(mt9p031.blc_auto, ctrl.val == 0);
    v4l2_ctrl_activate(mt9p031.blc_offset, ctrl.val == 0);
    if (!ctrl.val) {
// Restore the BLC settings.
    ret = mt9p031_restore_blc(mt9p031);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_TEST_PATTERN, 0);
    }
    ret = mt9p031_write(client, MT9P031_TEST_PATTERN_GREEN, 0x05a0);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_TEST_PATTERN_RED, 0x0a50);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_TEST_PATTERN_BLUE, 0x0aa0);
    if (ret < 0)
    return ret;
// Disable digital BLC when generating a test pattern.
    ret = mt9p031_set_mode2(mt9p031, MT9P031_READ_MODE_2_ROW_BLC,
    0);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_ROW_BLACK_DEF_OFFSET, 0);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_TEST_PATTERN,
    ((ctrl.val - 1) << MT9P031_TEST_PATTERN_SHIFT)
    | MT9P031_TEST_PATTERN_ENABLE);
    case V4L2_CID_BLC_AUTO:
    ret = mt9p031_set_mode2(mt9p031,
    ctrl.val ? 0 : MT9P031_READ_MODE_2_ROW_BLC,
    ctrl.val ? MT9P031_READ_MODE_2_ROW_BLC : 0);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_BLACK_LEVEL_CALIBRATION,
    ctrl.val ? 0 : MT9P031_BLC_MANUAL_BLC);
    case V4L2_CID_BLC_TARGET_LEVEL:
    return mt9p031_write(client, MT9P031_ROW_BLACK_TARGET,
    ctrl.val);
    case V4L2_CID_BLC_ANALOG_OFFSET:
    data = ctrl.val & ((1 << 9) - 1);
    ret = mt9p031_write(client, MT9P031_GREEN1_OFFSET, data);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_GREEN2_OFFSET, data);
    if (ret < 0)
    return ret;
    ret = mt9p031_write(client, MT9P031_RED_OFFSET, data);
    if (ret < 0)
    return ret;
    return mt9p031_write(client, MT9P031_BLUE_OFFSET, data);
    case V4L2_CID_BLC_DIGITAL_OFFSET:
    return mt9p031_write(client, MT9P031_ROW_BLACK_DEF_OFFSET,
    ctrl.val & ((1 << 12) - 1));
    }
    return 0;
    }
    static const struct v4l2_ctrl_ops mt9p031_ctrl_ops = {
    .s_ctrl = mt9p031_s_ctrl,
    };
    static const char * const mt9p031_test_pattern_menu[] = {
    "Disabled",
    "Color Field",
    "Horizontal Gradient",
    "Vertical Gradient",
    "Diagonal Gradient",
    "Classic Test Pattern",
    "Walking 1s",
    "Monochrome Horizontal Bars",
    "Monochrome Vertical Bars",
    "Vertical Color Bars",
    };
    static const struct v4l2_ctrl_config mt9p031_ctrls[] = {
    {
    .ops		= &mt9p031_ctrl_ops,
    .id		= V4L2_CID_BLC_AUTO,
    .type		= V4L2_CTRL_TYPE_BOOLEAN,
    .name		= "BLC, Auto",
    .min		= 0,
    .max		= 1,
    .step		= 1,
    .def		= 1,
    .flags		= 0,
    }, {
    .ops		= &mt9p031_ctrl_ops,
    .id		= V4L2_CID_BLC_TARGET_LEVEL,
    .type		= V4L2_CTRL_TYPE_INTEGER,
    .name		= "BLC Target Level",
    .min		= 0,
    .max		= 4095,
    .step		= 1,
    .def		= 168,
    .flags		= 0,
    }, {
    .ops		= &mt9p031_ctrl_ops,
    .id		= V4L2_CID_BLC_ANALOG_OFFSET,
    .type		= V4L2_CTRL_TYPE_INTEGER,
    .name		= "BLC Analog Offset",
    .min		= -255,
    .max		= 255,
    .step		= 1,
    .def		= 32,
    .flags		= 0,
    }, {
    .ops		= &mt9p031_ctrl_ops,
    .id		= V4L2_CID_BLC_DIGITAL_OFFSET,
    .type		= V4L2_CTRL_TYPE_INTEGER,
    .name		= "BLC Digital Offset",
    .min		= -2048,
    .max		= 2047,
    .step		= 1,
    .def		= 40,
    .flags		= 0,
    }
    };
// -----------------------------------------------------------------------------
// V4L2 subdev core operations
//
#[no_mangle]
unsafe extern "C" fn mt9p031_set_power(subdev: *mut v4l2_subdev, on: c_int) -> c_int {
    static int mt9p031_set_power(struct v4l2_subdev *subdev, int on)
    {
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    let mut ret: c_int = 0;
    mutex_lock(&mt9p031.power_lock);
// If the power count is modified from 0 to != 0 or from != 0 to 0,
// update the power state.
//
    if (mt9p031.power_count == !on) {
    ret = __mt9p031_set_power(mt9p031, !!on);
    if (ret < 0)
    goto out;
    }
// Update the power count.
    mt9p031.power_count += on ? 1 : -1;
    WARN_ON(mt9p031.power_count < 0);
    out:
    mutex_unlock(&mt9p031.power_lock);
    return ret;
    }
// -----------------------------------------------------------------------------
// V4L2 subdev internal operations
//
#[no_mangle]
unsafe extern "C" fn mt9p031_registered(subdev: *mut v4l2_subdev) -> c_int {
    static int mt9p031_registered(struct v4l2_subdev *subdev)
    {
    struct i2c_client *client = v4l2_get_subdevdata(subdev);
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    s32 data;
    int ret;
    ret = mt9p031_power_on(mt9p031);
    if (ret < 0) {
    dev_err(&client.dev, "MT9P031 power up failed\n");
    return ret;
    }
// Read out the chip version register
    data = mt9p031_read(client, MT9P031_CHIP_VERSION);
    mt9p031_power_off(mt9p031);
    if (data != MT9P031_CHIP_VERSION_VALUE) {
    dev_err(&client.dev, "MT9P031 not detected, wrong version "
    "0x%04x\n", data);
    return -ENODEV;
    }
    dev_info(&client.dev, "MT9P031 detected at address 0x%02x\n",
    client.addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_open(subdev: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int mt9p031_open(struct v4l2_subdev *subdev, struct v4l2_subdev_fh *fh)
    {
    return mt9p031_set_power(subdev, 1);
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_close(subdev: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int mt9p031_close(struct v4l2_subdev *subdev, struct v4l2_subdev_fh *fh)
    {
    return mt9p031_set_power(subdev, 0);
    }
    static const struct v4l2_subdev_core_ops mt9p031_subdev_core_ops = {
    .s_power        = mt9p031_set_power,
    };
    static const struct v4l2_subdev_video_ops mt9p031_subdev_video_ops = {
    .s_stream       = mt9p031_s_stream,
    };
    static const struct v4l2_subdev_pad_ops mt9p031_subdev_pad_ops = {
    .enum_mbus_code = mt9p031_enum_mbus_code,
    .enum_frame_size = mt9p031_enum_frame_size,
    .get_fmt = mt9p031_get_format,
    .set_fmt = mt9p031_set_format,
    .get_selection = mt9p031_get_selection,
    .set_selection = mt9p031_set_selection,
    };
    static const struct v4l2_subdev_ops mt9p031_subdev_ops = {
    .core   = &mt9p031_subdev_core_ops,
    .video  = &mt9p031_subdev_video_ops,
    .pad    = &mt9p031_subdev_pad_ops,
    };
    static const struct v4l2_subdev_internal_ops mt9p031_subdev_internal_ops = {
    .init_state = mt9p031_init_state,
    .registered = mt9p031_registered,
    .open = mt9p031_open,
    .close = mt9p031_close,
    };
// -----------------------------------------------------------------------------
// Driver initialization and probing
//
#[no_mangle]
unsafe extern "C" fn mt9p031_parse_properties(mt9p031: *mut mt9p031, dev: *mut device) -> c_int {
    static int mt9p031_parse_properties(struct mt9p031 *mt9p031, struct device *dev)
    {
    struct v4l2_fwnode_endpoint endpoint = {
    .bus_type = V4L2_MBUS_PARALLEL
    };
    struct fwnode_handle *np;
    int ret;
    np = fwnode_graph_get_next_endpoint(dev_fwnode(dev), core::ptr::null_mut());
    if (!np)
    return dev_err_probe(dev, -EINVAL, "endpoint node not found\n");
    ret = v4l2_fwnode_endpoint_parse(np, &endpoint);
    fwnode_handle_put(np);
    if (ret)
    return dev_err_probe(dev, -EINVAL, "could not parse endpoint\n");
    fwnode_property_read_u32(np, "input-clock-frequency",
    &mt9p031.ext_freq);
    fwnode_property_read_u32(np, "pixel-clock-frequency",
    &mt9p031.target_freq);
    mt9p031.pixclk_pol = !!(endpoint.bus.parallel.flags &
    V4L2_MBUS_PCLK_SAMPLE_RISING);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_probe(client: *mut i2c_client) -> c_int {
    static int mt9p031_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    const struct mt9p031_model_info *info;
    struct mt9p031 *mt9p031;
    unsigned int i;
    int ret;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_WORD_DATA)) {
    dev_warn(&client.dev,
    "I2C-Adapter doesn't support I2C_FUNC_SMBUS_WORD\n");
    return -EIO;
    }
    mt9p031 = devm_kzalloc(&client.dev, sizeof(*mt9p031), GFP_KERNEL);
    if (mt9p031 == core::ptr::null_mut())
    return -ENOMEM;
    ret = mt9p031_parse_properties(mt9p031, &client.dev);
    if (ret)
    return ret;
    mt9p031.output_control	= MT9P031_OUTPUT_CONTROL_DEF;
    mt9p031.mode2 = MT9P031_READ_MODE_2_ROW_BLC;
    info = device_get_match_data(&client.dev);
    mt9p031.code = info.code;
    mt9p031.regulators[0].supply = "vdd";
    mt9p031.regulators[1].supply = "vdd_io";
    mt9p031.regulators[2].supply = "vaa";
    ret = devm_regulator_bulk_get(&client.dev, 3, mt9p031.regulators);
    if (ret < 0) {
    dev_err(&client.dev, "Unable to get regulators\n");
    return ret;
    }
    mutex_init(&mt9p031.power_lock);
    v4l2_ctrl_handler_init(&mt9p031.ctrls, ARRAY_SIZE(mt9p031_ctrls) + 6);
    v4l2_ctrl_new_std(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_EXPOSURE, MT9P031_SHUTTER_WIDTH_MIN,
    MT9P031_SHUTTER_WIDTH_MAX, 1,
    MT9P031_SHUTTER_WIDTH_DEF);
    v4l2_ctrl_new_std(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_GAIN, MT9P031_GLOBAL_GAIN_MIN,
    MT9P031_GLOBAL_GAIN_MAX, 1, MT9P031_GLOBAL_GAIN_DEF);
    v4l2_ctrl_new_std(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_PIXEL_RATE, mt9p031.target_freq,
    mt9p031.target_freq, 1, mt9p031.target_freq);
    v4l2_ctrl_new_std_menu_items(&mt9p031.ctrls, &mt9p031_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(mt9p031_test_pattern_menu) - 1, 0,
    0, mt9p031_test_pattern_menu);
    for (i = 0; i < ARRAY_SIZE(mt9p031_ctrls); ++i)
    v4l2_ctrl_new_custom(&mt9p031.ctrls, &mt9p031_ctrls[i], core::ptr::null_mut());
    mt9p031.subdev.ctrl_handler = &mt9p031.ctrls;
    if (mt9p031.ctrls.error) {
    printk(KERN_INFO "%s: control initialization error %d\n",
    __func__, mt9p031.ctrls.error);
    ret = mt9p031.ctrls.error;
    goto done;
    }
    mt9p031.blc_auto = v4l2_ctrl_find(&mt9p031.ctrls, V4L2_CID_BLC_AUTO);
    mt9p031.blc_offset = v4l2_ctrl_find(&mt9p031.ctrls,
    V4L2_CID_BLC_DIGITAL_OFFSET);
    v4l2_i2c_subdev_init(&mt9p031.subdev, client, &mt9p031_subdev_ops);
    mt9p031.subdev.internal_ops = &mt9p031_subdev_internal_ops;
    mt9p031.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    mt9p031.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&mt9p031.subdev.entity, 1, &mt9p031.pad);
    if (ret < 0)
    goto done;
    mt9p031.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ret = mt9p031_init_state(&mt9p031.subdev, core::ptr::null_mut());
    if (ret)
    goto done;
    mt9p031.reset = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(mt9p031.reset)) {
    ret = PTR_ERR(mt9p031.reset);
    goto done;
    }
    ret = mt9p031_clk_setup(mt9p031);
    if (ret)
    goto done;
    ret = v4l2_async_register_subdev(&mt9p031.subdev);
    done:
    if (ret < 0) {
    v4l2_ctrl_handler_free(&mt9p031.ctrls);
    media_entity_cleanup(&mt9p031.subdev.entity);
    mutex_destroy(&mt9p031.power_lock);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt9p031_remove(client: *mut i2c_client) {
    static void mt9p031_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *subdev = i2c_get_clientdata(client);
    struct mt9p031 *mt9p031 = to_mt9p031(subdev);
    v4l2_ctrl_handler_free(&mt9p031.ctrls);
    v4l2_async_unregister_subdev(subdev);
    media_entity_cleanup(&subdev.entity);
    mutex_destroy(&mt9p031.power_lock);
    }
    static const struct mt9p031_model_info mt9p031_models_bayer = {
    .code = MEDIA_BUS_FMT_SGRBG12_1X12
    };
    static const struct mt9p031_model_info mt9p031_models_mono = {
    .code = MEDIA_BUS_FMT_Y12_1X12
    };
    static const struct of_device_id mt9p031_of_match[] = {
    { .compatible = "aptina,mt9p006", .data = &mt9p031_models_bayer },
    { .compatible = "aptina,mt9p031", .data = &mt9p031_models_bayer },
    { .compatible = "aptina,mt9p031m", .data = &mt9p031_models_mono },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mt9p031_of_match);
    static struct i2c_driver mt9p031_i2c_driver = {
    .driver = {
    .of_match_table = mt9p031_of_match,
    .name = "mt9p031",
    },
    .probe          = mt9p031_probe,
    .remove         = mt9p031_remove,
    };
    module_i2c_driver(mt9p031_i2c_driver);
    MODULE_DESCRIPTION("Aptina MT9P031 Camera driver");
    MODULE_AUTHOR("Bastian Hecht <hechtb@gmail.com>");
    MODULE_LICENSE("GPL v2");
