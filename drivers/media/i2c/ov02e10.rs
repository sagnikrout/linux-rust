//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov02e10.c
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
// Copyright (c) 2023 Intel Corporation.

pub const OV02E10_MCLK: c_int = 19200000;
pub const OV02E10_DATA_LANES: c_int = 2;
pub const OV02E10_RGB_DEPTH: c_int = 10;

pub const OV02E10_PAGE_0: c_uint = 0x0;
pub const OV02E10_PAGE_1: c_uint = 0x1;
pub const OV02E10_PAGE_2: c_uint = 0x2;
pub const OV02E10_PAGE_3: c_uint = 0x3;
pub const OV02E10_PAGE_5: c_uint = 0x4;
pub const OV02E10_PAGE_7: c_uint = 0x5;
pub const OV02E10_PAGE_8: c_uint = 0x6;
pub const OV02E10_PAGE_9: c_uint = 0xF;
pub const OV02E10_PAGE_D: c_uint = 0x8;
pub const OV02E10_PAGE_E: c_uint = 0x9;
pub const OV02E10_PAGE_F: c_uint = 0xA;

pub const OV02E10_CHIP_ID: c_uint = 0x45025610;
// Horizontal and vertical flip

// vertical-timings from sensor

pub const OV02E10_VTS_DEF: c_int = 2244;
pub const OV02E10_VTS_MIN: c_int = 2244;
pub const OV02E10_VTS_MAX: c_uint = 0x7fff;
// horizontal-timings from sensor

// Exposure controls from sensor

pub const OV02E10_EXPOSURE_MIN: c_int = 1;
pub const OV02E10_EXPOSURE_MAX_MARGIN: c_int = 2;
pub const OV02E10_EXPOSURE_STEP: c_int = 1;
// Analog gain controls from sensor

pub const OV02E10_ANAL_GAIN_MIN: c_uint = 0x10;
pub const OV02E10_ANAL_GAIN_MAX: c_uint = 0xf8;
pub const OV02E10_ANAL_GAIN_STEP: c_int = 1;
// Digital gain controls from sensor

pub const OV02E10_DGTL_GAIN_MIN: c_int = 256;
pub const OV02E10_DGTL_GAIN_MAX: c_int = 1020;
pub const OV02E10_DGTL_GAIN_STEP: c_int = 1;
pub const OV02E10_DGTL_GAIN_DEFAULT: c_int = 256;
// Register update control

pub const OV02E10_COMMAND_UPDATE: c_uint = 0x00;
pub const OV02E10_COMMAND_HOLD: c_uint = 0x01;
// Test Pattern Control

pub const OV02E10_TEST_PATTERN_BAR_SHIFT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence_list {
    pub num_regs: u32,
    pub regs: *const reg_sequence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov02e10_mode {
// Frame width in pixels
    pub width: u32,
// Frame height in pixels
    pub height: u32,
// Horizontal timining size
    pub hts: u32,
// Default vertical timing
    pub vts_def: u32,
// Min vertical timining size
    pub vts_min: u32,
// Sensor register settings for this resolution
    pub reg_list: reg_sequence_list,
}

    static const struct reg_sequence mode_1928x1088_30fps_2lane[] = {
    { 0xfd, 0x00 },
    { 0x20, 0x00 },
    { 0x20, 0x0b },
    { 0x21, 0x02 },
    { 0x10, 0x23 },
    { 0xc5, 0x04 },
    { 0x21, 0x00 },
    { 0x14, 0x96 },
    { 0x17, 0x01 },
    { 0xfd, 0x01 },
    { 0x03, 0x00 },
    { 0x04, 0x04 },
    { 0x05, 0x04 },
    { 0x06, 0x62 },
    { 0x07, 0x01 },
    { 0x22, 0x80 },
    { 0x24, 0xff },
    { 0x40, 0xc6 },
    { 0x41, 0x18 },
    { 0x45, 0x3f },
    { 0x48, 0x0c },
    { 0x4c, 0x08 },
    { 0x51, 0x12 },
    { 0x52, 0x10 },
    { 0x57, 0x98 },
    { 0x59, 0x06 },
    { 0x5a, 0x04 },
    { 0x5c, 0x38 },
    { 0x5e, 0x10 },
    { 0x67, 0x11 },
    { 0x7b, 0x04 },
    { 0x81, 0x12 },
    { 0x90, 0x51 },
    { 0x91, 0x09 },
    { 0x92, 0x21 },
    { 0x93, 0x28 },
    { 0x95, 0x54 },
    { 0x9d, 0x20 },
    { 0x9e, 0x04 },
    { 0xb1, 0x9a },
    { 0xb2, 0x86 },
    { 0xb6, 0x3f },
    { 0xb9, 0x30 },
    { 0xc1, 0x01 },
    { 0xc5, 0xa0 },
    { 0xc6, 0x73 },
    { 0xc7, 0x04 },
    { 0xc8, 0x25 },
    { 0xc9, 0x05 },
    { 0xca, 0x28 },
    { 0xcb, 0x00 },
    { 0xcf, 0x16 },
    { 0xd2, 0xd0 },
    { 0xd7, 0x3f },
    { 0xd8, 0x40 },
    { 0xd9, 0x40 },
    { 0xda, 0x44 },
    { 0xdb, 0x3d },
    { 0xdc, 0x3d },
    { 0xdd, 0x3d },
    { 0xde, 0x3d },
    { 0xdf, 0xf0 },
    { 0xea, 0x0f },
    { 0xeb, 0x04 },
    { 0xec, 0x29 },
    { 0xee, 0x47 },
    { 0xfd, 0x01 },
    { 0x31, 0x01 },
    { 0x27, 0x00 },
    { 0x2f, 0x41 },
    { 0xfd, 0x02 },
    { 0xa1, 0x01 },
    { 0xfd, 0x02 },
    { 0x9a, 0x03 },
    { 0xfd, 0x03 },
    { 0x9d, 0x0f },
    { 0xfd, 0x07 },
    { 0x42, 0x00 },
    { 0x43, 0xad },
    { 0x44, 0x00 },
    { 0x45, 0xa8 },
    { 0x46, 0x00 },
    { 0x47, 0xa8 },
    { 0x48, 0x00 },
    { 0x49, 0xad },
    { 0xfd, 0x00 },
    { 0xc4, 0x01 },
    { 0xfd, 0x01 },
    { 0x33, 0x03 },
    { 0xfd, 0x00 },
    { 0x20, 0x1f },
    };
    static const char *const ov02e10_test_pattern_menu[] = {
    "Disabled",
    "Color Bar",
    };
    static const s64 link_freq_menu_items[] = {
    OV02E10_LINK_FREQ_360MHZ,
    };
    static const struct ov02e10_mode supported_modes[] = {
    {
    .width = 1928,
    .height = 1088,
    .hts = 534,
    .vts_def = 2244,
    .vts_min = 2244,
    .reg_list = {
    .num_regs = ARRAY_SIZE(mode_1928x1088_30fps_2lane),
    .regs = mode_1928x1088_30fps_2lane,
    },
    },
    };
    static const char * const ov02e10_supply_names[] = {
    "dovdd",	/* Digital I/O power */
    "avdd",		/* Analog power */
    "dvdd",		/* Digital core power */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov02e10 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub ctrl_handler: v4l2_ctrl_handler,
// V4L2 Controls
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub img_clk: *mut clk,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(ov02e10_supply_names)],
    pub reset: *mut gpio_desc,
// Current mode
    pub cur_mode: *const ov02e10_mode,
// MIPI lanes info
    pub link_freq_index: u32,
    pub mipi_lanes: u8,
}

    static inline struct ov02e10 *to_ov02e10(struct v4l2_subdev *subdev)
    {
    return container_of(subdev, struct ov02e10, sd);
    }
#[no_mangle]
unsafe extern "C" fn to_pixel_rate(f_index: u32) -> u64 {
    static u64 to_pixel_rate(u32 f_index)
    {
    let mut pixel_rate: u64 = link_freq_menu_items[f_index] * 2 * OV02E10_DATA_LANES;
    do_div(pixel_rate, OV02E10_RGB_DEPTH);
    return pixel_rate;
    }
#[no_mangle]
unsafe extern "C" fn to_pixels_per_line(hts: u32, f_index: u32) -> u64 {
    static u64 to_pixels_per_line(u32 hts, u32 f_index)
    {
    let mut ppl: u64 = hts * to_pixel_rate(f_index);
    do_div(ppl, OV02E10_SCLK);
    return ppl;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_test_pattern(ov02e10: *mut ov02e10, pattern: u32, pret: *mut c_int) {
    static void ov02e10_test_pattern(struct ov02e10 *ov02e10, u32 pattern, int *pret)
    {
    if (pattern)
    pattern = pattern << OV02E10_TEST_PATTERN_BAR_SHIFT |
    OV02E10_TEST_PATTERN_ENABLE;
    cci_write(ov02e10.regmap, OV02E10_REG_TEST_PATTERN, pattern, pret);
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov02e10_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov02e10 *ov02e10 = container_of(ctrl.handler,
    struct ov02e10, ctrl_handler);
    s64 exposure_max;
    int ret;
// Propagate change of current control to all related controls
    if (ctrl.id == V4L2_CID_VBLANK) {
// Update max exposure while meeting expected vblanking
    exposure_max = ov02e10.cur_mode.height + ctrl.val -
    OV02E10_EXPOSURE_MAX_MARGIN;
    ret = __v4l2_ctrl_modify_range(ov02e10.exposure,
    ov02e10.exposure.minimum,
    exposure_max,
    ov02e10.exposure.step,
    exposure_max);
    if (ret)
    return ret;
    }
// V4L2 controls values will be applied only when power is already up
    if (!pm_runtime_get_if_in_use(ov02e10.dev))
    return 0;
    ret = cci_write(ov02e10.regmap, OV02E10_REG_COMMAND_UPDATE,
    OV02E10_COMMAND_HOLD, core::ptr::null_mut());
    switch (ctrl.id) {
    case V4L2_CID_ANALOGUE_GAIN:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_ANALOG_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_DIGITAL_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_EXPOSURE:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_EXPOSURE,
    ctrl.val, &ret);
    break;
    case V4L2_CID_HFLIP:
    case V4L2_CID_VFLIP:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_ORIENTATION,
    ov02e10.hflip.val | ov02e10.vflip.val << 1, &ret);
    break;
    case V4L2_CID_VBLANK:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_VTS,
    ov02e10.cur_mode.height + ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN:
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_1, &ret);
    ov02e10_test_pattern(ov02e10, ctrl.val, &ret);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    cci_write(ov02e10.regmap, OV02E10_REG_COMMAND_UPDATE,
    OV02E10_COMMAND_UPDATE, &ret);
    pm_runtime_put(ov02e10.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov02e10_ctrl_ops = {
    .s_ctrl = ov02e10_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov02e10_init_controls(ov02e10: *mut ov02e10) -> c_int {
    static int ov02e10_init_controls(struct ov02e10 *ov02e10)
    {
    struct v4l2_ctrl_handler *ctrl_hdlr = &ov02e10.ctrl_handler;
    const struct ov02e10_mode *mode = ov02e10.cur_mode;
    u32 vblank_min, vblank_max, vblank_def;
    struct v4l2_fwnode_device_properties props;
    s64 exposure_max, h_blank, pixel_rate;
    int ret;
    v4l2_ctrl_handler_init(ctrl_hdlr, 12);
    ov02e10.link_freq = v4l2_ctrl_new_int_menu(ctrl_hdlr,
    &ov02e10_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    ov02e10.link_freq_index,
    0, link_freq_menu_items);
    if (ov02e10.link_freq)
    ov02e10.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    pixel_rate = to_pixel_rate(ov02e10.link_freq_index);
    ov02e10.pixel_rate = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_PIXEL_RATE, 0,
    pixel_rate, 1, pixel_rate);
    vblank_min = mode.vts_min - mode.height;
    vblank_max = OV02E10_VTS_MAX - mode.height;
    vblank_def = mode.vts_def - mode.height;
    ov02e10.vblank = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_VBLANK, vblank_min,
    vblank_max, 1, vblank_def);
    h_blank = mode.hts - mode.width;
    ov02e10.hblank = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_HBLANK, h_blank, h_blank,
    1, h_blank);
    if (ov02e10.hblank)
    ov02e10.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops, V4L2_CID_ANALOGUE_GAIN,
    OV02E10_ANAL_GAIN_MIN, OV02E10_ANAL_GAIN_MAX,
    OV02E10_ANAL_GAIN_STEP, OV02E10_ANAL_GAIN_MIN);
    v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops, V4L2_CID_DIGITAL_GAIN,
    OV02E10_DGTL_GAIN_MIN, OV02E10_DGTL_GAIN_MAX,
    OV02E10_DGTL_GAIN_STEP, OV02E10_DGTL_GAIN_DEFAULT);
    exposure_max = mode.vts_def - OV02E10_EXPOSURE_MAX_MARGIN;
    ov02e10.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_EXPOSURE,
    OV02E10_EXPOSURE_MIN,
    exposure_max,
    OV02E10_EXPOSURE_STEP,
    exposure_max);
    ov02e10.hflip = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    if (ov02e10.hflip)
    ov02e10.hflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    ov02e10.vflip = v4l2_ctrl_new_std(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    if (ov02e10.vflip)
    ov02e10.vflip.flags |= V4L2_CTRL_FLAG_MODIFY_LAYOUT;
    v4l2_ctrl_new_std_menu_items(ctrl_hdlr, &ov02e10_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov02e10_test_pattern_menu) - 1,
    0, 0, ov02e10_test_pattern_menu);
    ret = v4l2_fwnode_device_parse(ov02e10.dev, &props);
    if (ret)
    return ret;
    v4l2_ctrl_new_fwnode_properties(ctrl_hdlr, &ov02e10_ctrl_ops, &props);
    if (ctrl_hdlr.error)
    return ctrl_hdlr.error;
    ov02e10.sd.ctrl_handler = ctrl_hdlr;
    return 0;
    }
    static void ov02e10_update_pad_format(const struct ov02e10_mode *mode,
    struct v4l2_mbus_framefmt *fmt)
    {
    fmt.width = mode.width;
    fmt.height = mode.height;
    fmt.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    fmt.field = V4L2_FIELD_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_set_stream_mode(ov02e10: *mut ov02e10, val: u8) -> c_int {
    static int ov02e10_set_stream_mode(struct ov02e10 *ov02e10, u8 val)
    {
    let mut ret: c_int = 0;
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG, OV02E10_PAGE_0, &ret);
    cci_write(ov02e10.regmap, CCI_REG8(0xa0), val, &ret);
    cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG, OV02E10_PAGE_1, &ret);
    cci_write(ov02e10.regmap, CCI_REG8(0x01), 0x02, &ret);
    return ret;
    }
    static int ov02e10_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    const struct reg_sequence_list *reg_list;
    int ret;
    ret = pm_runtime_resume_and_get(ov02e10.dev);
    if (ret)
    return ret;
    reg_list = &ov02e10.cur_mode.reg_list;
    ret = regmap_multi_reg_write(ov02e10.regmap, reg_list.regs,
    reg_list.num_regs);
    if (ret) {
    dev_err(ov02e10.dev, "failed to set mode\n");
    goto out;
    }
    ret = __v4l2_ctrl_handler_setup(ov02e10.sd.ctrl_handler);
    if (ret)
    goto out;
    ret = ov02e10_set_stream_mode(ov02e10, 1);
    out:
    if (ret)
    pm_runtime_put(ov02e10.dev);
    return ret;
    }
    static int ov02e10_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    ov02e10_set_stream_mode(ov02e10, 0);
    pm_runtime_put(ov02e10.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_get_pm_resources(dev: *mut device) -> c_int {
    static int ov02e10_get_pm_resources(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    int i;
    ov02e10.reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(ov02e10.reset))
    return dev_err_probe(dev, PTR_ERR(ov02e10.reset),
    "failed to get reset gpio\n");
    for (i = 0; i < ARRAY_SIZE(ov02e10_supply_names); i++)
    ov02e10.supplies[i].supply = ov02e10_supply_names[i];
    return devm_regulator_bulk_get(dev, ARRAY_SIZE(ov02e10_supply_names),
    ov02e10.supplies);
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_power_off(dev: *mut device) -> c_int {
    static int ov02e10_power_off(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    if (ov02e10.reset)
    gpiod_set_value_cansleep(ov02e10.reset, 1);
    regulator_bulk_disable(ARRAY_SIZE(ov02e10_supply_names),
    ov02e10.supplies);
    clk_disable_unprepare(ov02e10.img_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_power_on(dev: *mut device) -> c_int {
    static int ov02e10_power_on(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    int ret;
    ret = clk_prepare_enable(ov02e10.img_clk);
    if (ret < 0) {
    dev_err(dev, "failed to enable imaging clock: %d\n", ret);
    return ret;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE(ov02e10_supply_names),
    ov02e10.supplies);
    if (ret < 0) {
    dev_err(dev, "failed to enable regulators\n");
    goto disable_clk;
    }
    if (ov02e10.reset) {
    usleep_range(5000, 5100);
    gpiod_set_value_cansleep(ov02e10.reset, 0);
    usleep_range(8000, 8100);
    }
    return 0;
    disable_clk:
    clk_disable_unprepare(ov02e10.img_clk);
    return ret;
    }
    static int ov02e10_set_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    const struct ov02e10_mode *mode;
    s32 vblank_def, h_blank;
    let mut ret: c_int = 0;
    mode = v4l2_find_nearest_size(supported_modes,
    ARRAY_SIZE(supported_modes),
    width, height, fmt.format.width,
    fmt.format.height);
    ov02e10_update_pad_format(mode, &fmt.format);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY) {
// v4l2_subdev_state_get_format(sd_state, fmt->pad) = fmt->format;
    } else {
    ov02e10.cur_mode = mode;
    ret = __v4l2_ctrl_s_ctrl(ov02e10.link_freq,
    ov02e10.link_freq_index);
    if (ret)
    return ret;
    ret = __v4l2_ctrl_s_ctrl_int64(ov02e10.pixel_rate,
    to_pixel_rate(ov02e10.link_freq_index));
    if (ret)
    return ret;
// Update limits and set FPS to default
    vblank_def = mode.vts_def - mode.height;
    ret = __v4l2_ctrl_modify_range(ov02e10.vblank,
    mode.vts_min - mode.height,
    OV02E10_VTS_MAX - mode.height,
    1, vblank_def);
    if (ret)
    return ret;
    ret = __v4l2_ctrl_s_ctrl(ov02e10.vblank, vblank_def);
    if (ret)
    return ret;
    h_blank = to_pixels_per_line(mode.hts, ov02e10.link_freq_index);
    h_blank -= mode.width;
    ret = __v4l2_ctrl_modify_range(ov02e10.hblank, h_blank,
    h_blank, 1, h_blank);
    }
    return ret;
    }
    static int ov02e10_get_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY)
    fmt.format = *v4l2_subdev_state_get_format(sd_state, fmt.pad);
    else
    ov02e10_update_pad_format(ov02e10.cur_mode, &fmt.format);
    return 0;
    }
    static int ov02e10_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    return 0;
    }
    static int ov02e10_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.index >= ARRAY_SIZE(supported_modes))
    return -EINVAL;
    if (fse.code != MEDIA_BUS_FMT_SGRBG10_1X10)
    return -EINVAL;
    fse.min_width = supported_modes[fse.index].width;
    fse.max_width = fse.min_width;
    fse.min_height = supported_modes[fse.index].height;
    fse.max_height = fse.min_height;
    return 0;
    }
    static int ov02e10_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    ov02e10_update_pad_format(&supported_modes[0],
    v4l2_subdev_state_get_format(sd_state, 0));
    return 0;
    }
    static const struct v4l2_subdev_video_ops ov02e10_video_ops = {
    .s_stream = v4l2_subdev_s_stream_helper,
    };
    static const struct v4l2_subdev_pad_ops ov02e10_pad_ops = {
    .set_fmt = ov02e10_set_format,
    .get_fmt = ov02e10_get_format,
    .enum_mbus_code = ov02e10_enum_mbus_code,
    .enum_frame_size = ov02e10_enum_frame_size,
    .enable_streams = ov02e10_enable_streams,
    .disable_streams = ov02e10_disable_streams,
    };
    static const struct v4l2_subdev_ops ov02e10_subdev_ops = {
    .video = &ov02e10_video_ops,
    .pad = &ov02e10_pad_ops,
    };
    static const struct media_entity_operations ov02e10_subdev_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
    static const struct v4l2_subdev_internal_ops ov02e10_internal_ops = {
    .init_state = ov02e10_init_state,
    };
#[no_mangle]
unsafe extern "C" fn ov02e10_identify_module(ov02e10: *mut ov02e10) -> c_int {
    static int ov02e10_identify_module(struct ov02e10 *ov02e10)
    {
    int ret;
    u64 val;
    ret = cci_write(ov02e10.regmap, OV02E10_REG_PAGE_FLAG,
    OV02E10_PAGE_0, core::ptr::null_mut());
    cci_read(ov02e10.regmap, OV02E10_REG_CHIP_ID, &val, &ret);
    if (ret)
    return ret;
    if (val != OV02E10_CHIP_ID) {
    dev_err(ov02e10.dev, "chip id mismatch: %x!=%x\n",
    OV02E10_CHIP_ID, (u32)val);
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_check_hwcfg(ov02e10: *mut ov02e10) -> c_int {
    static int ov02e10_check_hwcfg(struct ov02e10 *ov02e10)
    {
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY
    };
    struct device *dev = ov02e10.dev;
    struct fwnode_handle *ep;
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    unsigned long link_freq_bitmap;
    int ret;
    ep = fwnode_graph_get_next_endpoint(fwnode, core::ptr::null_mut());
    if (!ep)
    return dev_err_probe(dev, -EPROBE_DEFER,
    "waiting for fwnode graph endpoint\n");
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    fwnode_handle_put(ep);
    if (ret)
    return dev_err_probe(dev, ret, "parsing endpoint failed\n");
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != OV02E10_DATA_LANES) {
    dev_err(dev, "number of CSI2 data lanes %d is not supported\n",
    bus_cfg.bus.mipi_csi2.num_data_lanes);
    ret = -EINVAL;
    goto out_err;
    }
    if (!bus_cfg.nr_of_link_frequencies) {
    dev_err(dev, "no link frequencies defined\n");
    ret = -EINVAL;
    goto out_err;
    }
    ret = v4l2_link_freq_to_bitmap(dev, bus_cfg.link_frequencies,
    bus_cfg.nr_of_link_frequencies,
    link_freq_menu_items,
    ARRAY_SIZE(link_freq_menu_items),
    &link_freq_bitmap);
    if (ret)
    goto out_err;
// v4l2_link_freq_to_bitmap() guarantees at least 1 bit is set
    ov02e10.link_freq_index = ffs(link_freq_bitmap) - 1;
    ov02e10.mipi_lanes = bus_cfg.bus.mipi_csi2.num_data_lanes;
    out_err:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_remove(client: *mut i2c_client) {
    static void ov02e10_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov02e10 *ov02e10 = to_ov02e10(sd);
    v4l2_async_unregister_subdev(sd);
    v4l2_subdev_cleanup(sd);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(sd.ctrl_handler);
    pm_runtime_disable(ov02e10.dev);
    if (!pm_runtime_status_suspended(ov02e10.dev)) {
    ov02e10_power_off(ov02e10.dev);
    pm_runtime_set_suspended(ov02e10.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn ov02e10_probe(client: *mut i2c_client) -> c_int {
    static int ov02e10_probe(struct i2c_client *client)
    {
    struct ov02e10 *ov02e10;
    unsigned long freq;
    int ret;
    ov02e10 = devm_kzalloc(&client.dev, sizeof(*ov02e10), GFP_KERNEL);
    if (!ov02e10)
    return -ENOMEM;
    ov02e10.dev = &client.dev;
    ov02e10.img_clk = devm_v4l2_sensor_clk_get(ov02e10.dev, core::ptr::null_mut());
    if (IS_ERR(ov02e10.img_clk))
    return dev_err_probe(ov02e10.dev, PTR_ERR(ov02e10.img_clk),
    "failed to get imaging clock\n");
    freq = clk_get_rate(ov02e10.img_clk);
    if (freq != OV02E10_MCLK)
    return dev_err_probe(ov02e10.dev, -EINVAL,
    "external clock %lu is not supported",
    freq);
    v4l2_i2c_subdev_init(&ov02e10.sd, client, &ov02e10_subdev_ops);
// Check HW config
    ret = ov02e10_check_hwcfg(ov02e10);
    if (ret)
    return ret;
// Initialize subdev
    ov02e10.regmap = devm_cci_regmap_init_i2c(client, 8);
    if (IS_ERR(ov02e10.regmap))
    return PTR_ERR(ov02e10.regmap);
    ret = ov02e10_get_pm_resources(ov02e10.dev);
    if (ret)
    return ret;
    ret = ov02e10_power_on(ov02e10.dev);
    if (ret) {
    dev_err_probe(ov02e10.dev, ret, "failed to power on\n");
    return ret;
    }
// Check module identity
    ret = ov02e10_identify_module(ov02e10);
    if (ret) {
    dev_err(ov02e10.dev, "failed to find sensor: %d\n", ret);
    goto probe_error_power_off;
    }
    ov02e10.cur_mode = &supported_modes[0];
    ret = ov02e10_init_controls(ov02e10);
    if (ret) {
    dev_err(ov02e10.dev, "failed to init controls: %d\n", ret);
    goto probe_error_v4l2_ctrl_handler_free;
    }
// Initialize subdev
    ov02e10.sd.internal_ops = &ov02e10_internal_ops;
    ov02e10.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ov02e10.sd.entity.ops = &ov02e10_subdev_entity_ops;
    ov02e10.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
// Initialize source pad
    ov02e10.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&ov02e10.sd.entity, 1, &ov02e10.pad);
    if (ret) {
    dev_err(ov02e10.dev, "failed to init entity pads: %d", ret);
    goto probe_error_v4l2_ctrl_handler_free;
    }
    ov02e10.sd.state_lock = ov02e10.ctrl_handler.lock;
    ret = v4l2_subdev_init_finalize(&ov02e10.sd);
    if (ret < 0) {
    dev_err(ov02e10.dev, "failed to init subdev: %d", ret);
    goto probe_error_media_entity_cleanup;
    }
    pm_runtime_set_active(ov02e10.dev);
    pm_runtime_enable(ov02e10.dev);
    ret = v4l2_async_register_subdev_sensor(&ov02e10.sd);
    if (ret < 0) {
    dev_err(ov02e10.dev, "failed to register V4L2 subdev: %d",
    ret);
    goto probe_error_v4l2_subdev_cleanup;
    }
    pm_runtime_idle(ov02e10.dev);
    return 0;
    probe_error_v4l2_subdev_cleanup:
    pm_runtime_disable(ov02e10.dev);
    pm_runtime_set_suspended(ov02e10.dev);
    v4l2_subdev_cleanup(&ov02e10.sd);
    probe_error_media_entity_cleanup:
    media_entity_cleanup(&ov02e10.sd.entity);
    probe_error_v4l2_ctrl_handler_free:
    v4l2_ctrl_handler_free(ov02e10.sd.ctrl_handler);
    probe_error_power_off:
    ov02e10_power_off(ov02e10.dev);
    return ret;
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(ov02e10_pm_ops, ov02e10_power_off,
    ov02e10_power_on, core::ptr::null_mut());
    static const struct acpi_device_id ov02e10_acpi_ids[] = {
    { "OVTI02E1" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(acpi, ov02e10_acpi_ids);
    static const struct of_device_id ov02e10_of_match[] = {
    { .compatible = "ovti,ov02e10" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ov02e10_of_match);
    static struct i2c_driver ov02e10_i2c_driver = {
    .driver = {
    .name = "ov02e10",
    .pm = pm_sleep_ptr(&ov02e10_pm_ops),
    .acpi_match_table = ov02e10_acpi_ids,
    .of_match_table = ov02e10_of_match,
    },
    .probe = ov02e10_probe,
    .remove = ov02e10_remove,
    };
    module_i2c_driver(ov02e10_i2c_driver);
    MODULE_AUTHOR("Jingjing Xiong");
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_AUTHOR("Alan Stern <stern@rowland.harvard.edu>");
    MODULE_AUTHOR("Bryan O'Donoghue <bryan.odonoghue@linaro.org>");
    MODULE_DESCRIPTION("OmniVision OV02E10 sensor driver");
    MODULE_LICENSE("GPL");
