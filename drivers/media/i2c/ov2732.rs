//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov2732.c
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
// ov2732 driver
//
// Copyright (C) 2017 Fuzhou Rockchip Electronics Co., Ltd.
// Copyright (C) 2025-2026 Walter Werner Schneider <contact@schnwalter.eu>
//

pub const OV2732_LANES: c_int = 2;
pub const OV2732_BITS_PER_SAMPLE: c_int = 10;
pub const OV2732_LINK_FREQ_DEFAULT: c_int = 360000000;
pub const OV2732_XVCLK_FREQ: c_int = 24000000;

    (OV2732_LINK_FREQ_DEFAULT * 2 * OV2732_LANES / OV2732_BITS_PER_SAMPLE)

// Delay from power up to the first SCCB transaction.
pub const OV2732_POWER_UP_DELAY_CYCLES: c_int = 8192;
// Delay from the last SCCB transaction to power down.
pub const OV2732_POWER_DOWN_DELAY_CYCLES: c_int = 512;

    (DIV_ROUND_UP((cycles), OV2732_XVCLK_FREQ / USEC_PER_SEC))

pub const OV2732_CHIP_ID: c_uint = 0x002732;

pub const OV2732_MODE_STANDBY: c_uint = 0x00;
pub const OV2732_MODE_STREAMING: c_uint = 0x01;

pub const OV2732_ANALOGUE_GAIN_MIN: c_uint = 0x80;
// Max analogue gain is documented as 0x3fff, but it overflows after 0x3ff.
pub const OV2732_ANALOGUE_GAIN_MAX: c_uint = 0x3ff;
pub const OV2732_ANALOGUE_GAIN_DEFAULT: c_uint = 0x80;
pub const OV2732_DIGITAL_GAIN_MIN: c_uint = 0x00;
pub const OV2732_DIGITAL_GAIN_MAX: c_uint = 0x3fff;
pub const OV2732_DIGITAL_GAIN_DEFAULT: c_uint = 0x0400;
pub const OV2732_EXPOSURE_DEFAULT: c_uint = 0x40;
pub const OV2732_EXPOSURE_MIN: c_uint = 0x10;
pub const OV2732_EXPOSURE_OFFSET: c_int = 4;
pub const OV2732_HBLANK_DEFAULT: c_uint = 0x0068;
pub const OV2732_VTS_MAX: c_uint = 0x7fff;

pub const OV2732_TEST_PATTERN_DISABLE: c_uint = 0x00;
pub const OV2732_TEST_PATTERN_BAR1: c_uint = 0x80;
pub const OV2732_TEST_PATTERN_BAR2: c_uint = 0x84;
pub const OV2732_TEST_PATTERN_BAR3: c_uint = 0x88;
pub const OV2732_TEST_PATTERN_BAR4: c_uint = 0x8c;
pub const OV2732_TEST_PATTERN_BAR5: c_uint = 0xC0;
pub const OV2732_TEST_PATTERN_RANDOM: c_uint = 0x81;
pub const OV2732_TEST_PATTERN_SQUARES_C: c_uint = 0x82;
pub const OV2732_TEST_PATTERN_SQUARES_BW: c_uint = 0x92;
    static const char * const ov2732_supply_names[] = {
    "avdd", /* Analog power */
    "dovdd", /* Digital I/O power */
    "dvdd", /* Digital core power */
    };
    static const struct cci_reg_sequence ov2732_common_regs[] = {
// PLL control, reset all registers.
    { CCI_REG8(0x0103), 0x01 },
// Analog control.
    { CCI_REG8(0x3600), 0x55 },
    { CCI_REG8(0x3601), 0x52 },
    { CCI_REG8(0x3612), 0xb5 },
    { CCI_REG8(0x3613), 0xb3 },
    { CCI_REG8(0x3616), 0x83 },
    { CCI_REG8(0x3621), 0x00 },
    { CCI_REG8(0x3624), 0x06 },
    { CCI_REG8(0x3642), 0x88 },
    { CCI_REG8(0x3660), 0x00 },
    { CCI_REG8(0x3661), 0x00 },
    { CCI_REG8(0x366a), 0x64 },
    { CCI_REG8(0x366c), 0x00 },
    { CCI_REG8(0x366e), 0xff },
    { CCI_REG8(0x366f), 0xff },
    { CCI_REG8(0x3677), 0x11 },
    { CCI_REG8(0x3678), 0x11 },
    { CCI_REG8(0x3679), 0x0c },
    { CCI_REG8(0x3680), 0xff },
    { CCI_REG8(0x3681), 0x16 },
    { CCI_REG8(0x3682), 0x16 },
    { CCI_REG8(0x3683), 0x90 },
    { CCI_REG8(0x3684), 0x90 },
// ADC sync control.
    { CCI_REG8(0x4503), 0x00 },
    { CCI_REG8(0x4508), 0x14 },
    { CCI_REG8(0x450a), 0x00 },
    { CCI_REG8(0x450b), 0x40 },
// ISP control, enable: WIN, DPC & ISP.
    { CCI_REG8(0x5000), 0xa1 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2732_mode {
    pub width: u32,
    pub height: u32,
    pub vts: u32,
}

    static const struct ov2732_mode supported_modes[] = {
    {
    .width = 1920,
    .height = 1080,
    .vts = 1184,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov2732 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pad: media_pad,
    pub sd: v4l2_subdev,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub xvclk: *mut clk,
    pub xvclk_freq: u32,
    pub powerdown_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(ov2732_supply_names)],
}

    static const s64 link_freq_menu_items[] = {
    OV2732_LINK_FREQ_DEFAULT,
    };
    static const char * const ov2732_test_pattern_menu[] = {
    "Disabled",
    "Vertical Color Bar Type 1",
    "Vertical Color Bar Type 2",
    "Vertical Color Bar Type 3",
    "Vertical Color Bar Type 4",
    "Vertical Color Bar Type 5",
    "Random",
    "Color Squares",
    "Black and White Squares",
    };
    static const int ov2732_test_pattern_val[] = {
    OV2732_TEST_PATTERN_DISABLE,
    OV2732_TEST_PATTERN_BAR1,
    OV2732_TEST_PATTERN_BAR2,
    OV2732_TEST_PATTERN_BAR3,
    OV2732_TEST_PATTERN_BAR4,
    OV2732_TEST_PATTERN_BAR5,
    OV2732_TEST_PATTERN_RANDOM,
    OV2732_TEST_PATTERN_SQUARES_C,
    OV2732_TEST_PATTERN_SQUARES_BW,
    };
#[no_mangle]
unsafe extern "C" fn ov2732_power_on(dev: *mut device) -> c_int {
    static int ov2732_power_on(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov2732 *ov2732 = to_ov2732(sd);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ov2732_supply_names),
    ov2732.supplies);
    if (ret) {
    dev_err(dev, "failed to enable regulators\n");
    return ret;
    }
    ret = clk_prepare_enable(ov2732.xvclk);
    if (ret) {
    dev_err(dev, "failed to enable clock\n");
    goto reg_off;
    }
// Wait 10ms before power up, as per datasheet.
    fsleep(10 * USEC_PER_MSEC);
    gpiod_set_value_cansleep(ov2732.reset_gpio, 0);
    gpiod_set_value_cansleep(ov2732.powerdown_gpio, 0);
// Datasheet requires an 8192 cycles wait, but that isn't enough.
    fsleep(OV2732_DELAY_US(OV2732_POWER_UP_DELAY_CYCLES * 2));
    return 0;
    reg_off:
    regulator_bulk_disable(ARRAY_SIZE(ov2732_supply_names),
    ov2732.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2732_power_off(dev: *mut device) -> c_int {
    static int ov2732_power_off(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov2732 *ov2732 = to_ov2732(sd);
    clk_disable_unprepare(ov2732.xvclk);
// Wait for 512 cycles as per datasheet.
    fsleep(OV2732_DELAY_US(OV2732_POWER_DOWN_DELAY_CYCLES));
    gpiod_set_value_cansleep(ov2732.powerdown_gpio, 1);
    gpiod_set_value_cansleep(ov2732.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(ov2732_supply_names),
    ov2732.supplies);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov2732_identify_chip(ov2732: *mut ov2732) -> c_int {
    static int ov2732_identify_chip(struct ov2732 *ov2732)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&ov2732.sd);
    int ret;
    u64 val;
    ret = cci_read(ov2732.regmap, OV2732_REG_CHIP_ID, &val, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(&client.dev, ret,
    "failed to read chip id\n");
    if (val != OV2732_CHIP_ID)
    return dev_err_probe(&client.dev, -ENODEV,
    "chip id mismatch: %x!=%llx\n",
    OV2732_CHIP_ID, val);
    return 0;
    }
    static int ov2732_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index != 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    return 0;
    }
    static int ov2732_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.index >= ARRAY_SIZE(supported_modes))
    return -EINVAL;
    fse.min_width = supported_modes[fse.index].width;
    fse.max_width = fse.min_width;
    fse.min_height = supported_modes[fse.index].height;
    fse.max_height = fse.min_height;
    return 0;
    }
    static int ov2732_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov2732 *ov2732 = to_ov2732(sd);
    const struct ov2732_mode *mode;
    s64 vblank_def;
    int ret;
    mode = v4l2_find_nearest_size(supported_modes,
    ARRAY_SIZE(supported_modes),
    width, height,
    fmt.format.width, fmt.format.height);
    fmt.format.code = MEDIA_BUS_FMT_SBGGR10_1X10;
    fmt.format.width = mode.width;
    fmt.format.height = mode.height;
    fmt.format.field = V4L2_FIELD_NONE;
    fmt.format.colorspace = V4L2_COLORSPACE_RAW;
    fmt.format.ycbcr_enc = V4L2_YCBCR_ENC_601;
    fmt.format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    fmt.format.xfer_func = V4L2_XFER_FUNC_NONE;
// v4l2_subdev_state_get_format(state, fmt->pad) = fmt->format;
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY)
    return 0;
    vblank_def = mode.vts - mode.height;
    ret = __v4l2_ctrl_modify_range(ov2732.vblank, vblank_def,
    OV2732_VTS_MAX - mode.height, 1,
    vblank_def);
    return ret;
    }
    static int ov2732_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP:
    sel.r = *v4l2_subdev_state_get_crop(state, 0);
    return 0;
    case V4L2_SEL_TGT_NATIVE_SIZE:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = OV2732_NATIVE_WIDTH;
    sel.r.height = OV2732_NATIVE_HEIGHT;
    return 0;
    }
    return -EINVAL;
    }
    static int ov2732_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 pad,
    u64 streams_mask)
    {
    struct ov2732 *ov2732 = to_ov2732(sd);
    struct i2c_client *client = v4l2_get_subdevdata(&ov2732.sd);
    int ret;
    ret = pm_runtime_resume_and_get(&client.dev);
    if (ret < 0)
    return ret;
// Set stream off register for PLL changes.
    ret = cci_write(ov2732.regmap, OV2732_REG_MODE_SELECT,
    OV2732_MODE_STANDBY, core::ptr::null_mut());
    if (ret)
    goto err_put_autosuspend;
// Send all registers that are common to all modes
    ret = cci_multi_reg_write(ov2732.regmap, ov2732_common_regs,
    ARRAY_SIZE(ov2732_common_regs), core::ptr::null_mut());
    if (ret) {
    dev_err(&client.dev, "failed to init registers\n");
    goto err_put_autosuspend;
    }
// Apply customized values from user
    ret =  __v4l2_ctrl_handler_setup(ov2732.sd.ctrl_handler);
    if (ret)
    goto err_put_autosuspend;
// Set stream on register
    ret = cci_write(ov2732.regmap, OV2732_REG_MODE_SELECT,
    OV2732_MODE_STREAMING, core::ptr::null_mut());
    if (ret)
    goto err_put_autosuspend;
    return 0;
    err_put_autosuspend:
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    }
    static int ov2732_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 pad,
    u64 streams_mask)
    {
    struct ov2732 *ov2732 = to_ov2732(sd);
    struct i2c_client *client = v4l2_get_subdevdata(&ov2732.sd);
    int ret;
// set stream off register
    ret = cci_write(ov2732.regmap, OV2732_REG_MODE_SELECT,
    OV2732_MODE_STANDBY, core::ptr::null_mut());
    if (ret)
    dev_err(&client.dev, "%s failed to set stream\n", __func__);
// Wait for 512 cycles as per datasheet.
    fsleep(OV2732_DELAY_US(OV2732_POWER_DOWN_DELAY_CYCLES));
    pm_runtime_put_autosuspend(&client.dev);
    return ret;
    }
    static const struct v4l2_subdev_video_ops ov2732_video_ops = {
    .s_stream = v4l2_subdev_s_stream_helper,
    };
    static const struct v4l2_subdev_pad_ops ov2732_pad_ops = {
    .enum_mbus_code = ov2732_enum_mbus_code,
    .enum_frame_size = ov2732_enum_frame_size,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = ov2732_set_fmt,
    .get_selection = ov2732_get_selection,
    .enable_streams = ov2732_enable_streams,
    .disable_streams = ov2732_disable_streams,
    };
    static const struct v4l2_subdev_ops ov2732_subdev_ops = {
    .video  = &ov2732_video_ops,
    .pad = &ov2732_pad_ops,
    };
    static int ov2732_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_subdev_format fmt = {
    .which = V4L2_SUBDEV_FORMAT_TRY,
    .format = {
    .width = 1920,
    .height = 1080,
    .code = MEDIA_BUS_FMT_SBGGR10_1X10,
    .colorspace = V4L2_COLORSPACE_RAW,
    .field = V4L2_FIELD_NONE,
    .quantization = V4L2_QUANTIZATION_FULL_RANGE,
    .xfer_func = V4L2_XFER_FUNC_NONE,
    .ycbcr_enc = V4L2_YCBCR_ENC_601,
    }
    };
    return ov2732_set_fmt(sd, sd_state, &fmt);
    }
    static const struct v4l2_subdev_internal_ops ov2732_internal_ops = {
    .init_state = ov2732_init_state,
    };
#[no_mangle]
unsafe extern "C" fn ov2732_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov2732_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov2732 *ov2732 =
    container_of(ctrl.handler, struct ov2732, ctrl_handler);
    struct i2c_client *client = v4l2_get_subdevdata(&ov2732.sd);
    const struct v4l2_mbus_framefmt *format;
    struct v4l2_subdev_state *state;
    let mut ret: c_int = 0;
    state = v4l2_subdev_get_locked_active_state(&ov2732.sd);
    format = v4l2_subdev_state_get_format(state, 0);
    if (ctrl.id == V4L2_CID_VBLANK) {
    int exposure_max, exposure_def;
    exposure_max = format.height + ctrl.val -
    OV2732_EXPOSURE_OFFSET;
    exposure_def = exposure_max;
    ret = __v4l2_ctrl_modify_range(ov2732.exposure,
    OV2732_EXPOSURE_MIN,
    exposure_max,
    ov2732.exposure.step,
    exposure_def);
    if (ret)
    return ret;
    }
    if (pm_runtime_get_if_in_use(&client.dev) == 0)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_ANALOGUE_GAIN:
    cci_write(ov2732.regmap, OV2732_REG_ANALOGUE_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    cci_write(ov2732.regmap, OV2732_REG_DIGITAL_GAIN,
    ctrl.val, &ret);
    break;
    case V4L2_CID_EXPOSURE:
// Lowest 4 bits are fraction bits.
    cci_write(ov2732.regmap, OV2732_REG_EXPOSURE,
    (u32)ctrl.val << 4, &ret);
    break;
    case V4L2_CID_VBLANK:
    cci_write(ov2732.regmap, OV2732_REG_VTS,
    format.height + ctrl.val, &ret);
    break;
    case V4L2_CID_TEST_PATTERN:
    cci_write(ov2732.regmap, OV2732_REG_TEST_PATTERN,
    ov2732_test_pattern_val[ctrl.val], &ret);
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
    };
    static const struct v4l2_ctrl_ops ov2732_ctrl_ops = {
    .s_ctrl = ov2732_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov2732_init_controls(ov2732: *mut ov2732) -> c_int {
    static int ov2732_init_controls(struct ov2732 *ov2732)
    {
    const struct ov2732_mode *mode = &supported_modes[0];
    struct v4l2_ctrl_handler *handler;
    struct v4l2_ctrl *ctrl;
    struct v4l2_fwnode_device_properties props;
    s64 exposure_max, vblank_def, vblank_max;
    int ret;
    handler = &ov2732.ctrl_handler;
    ret = v4l2_ctrl_handler_init(handler, 10);
    v4l2_ctrl_new_std(handler, core::ptr::null_mut(), V4L2_CID_PIXEL_RATE,
    OV2732_PIXEL_RATE, OV2732_PIXEL_RATE,
    1, OV2732_PIXEL_RATE);
    ctrl = v4l2_ctrl_new_int_menu(handler, &ov2732_ctrl_ops,
    V4L2_CID_LINK_FREQ, 0, 0,
    link_freq_menu_items);
    if (ctrl)
    ctrl.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    ov2732.hblank = v4l2_ctrl_new_std(handler, &ov2732_ctrl_ops,
    V4L2_CID_HBLANK,
    OV2732_HBLANK_DEFAULT,
    OV2732_HBLANK_DEFAULT,
    1, OV2732_HBLANK_DEFAULT);
    if (ov2732.hblank)
    ov2732.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    vblank_def = mode.vts - mode.height;
    vblank_max = OV2732_VTS_MAX - mode.height;
    ov2732.vblank = v4l2_ctrl_new_std(handler, &ov2732_ctrl_ops,
    V4L2_CID_VBLANK,
    vblank_def, vblank_max,
    1, vblank_def);
    exposure_max = mode.vts - OV2732_EXPOSURE_OFFSET;
    ov2732.exposure = v4l2_ctrl_new_std(handler, &ov2732_ctrl_ops,
    V4L2_CID_EXPOSURE,
    OV2732_EXPOSURE_MIN, exposure_max,
    1, OV2732_EXPOSURE_DEFAULT);
    v4l2_ctrl_new_std(handler, &ov2732_ctrl_ops, V4L2_CID_ANALOGUE_GAIN,
    OV2732_ANALOGUE_GAIN_MIN, OV2732_ANALOGUE_GAIN_MAX,
    1, OV2732_ANALOGUE_GAIN_DEFAULT);
    v4l2_ctrl_new_std(handler, &ov2732_ctrl_ops, V4L2_CID_DIGITAL_GAIN,
    OV2732_DIGITAL_GAIN_MIN, OV2732_DIGITAL_GAIN_MAX,
    1, OV2732_DIGITAL_GAIN_DEFAULT);
    v4l2_ctrl_new_std_menu_items(handler, &ov2732_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov2732_test_pattern_menu) - 1,
    0, 0, ov2732_test_pattern_menu);
    if (handler.error) {
    ret = handler.error;
    dev_err_probe(ov2732.dev, ret, "Control init failed\n");
    goto err_handler_free;
    }
    ret = v4l2_fwnode_device_parse(ov2732.dev, &props);
    if (ret)
    goto err_handler_free;
    ret = v4l2_ctrl_new_fwnode_properties(handler, &ov2732_ctrl_ops, &props);
    if (ret)
    goto err_handler_free;
    ov2732.sd.ctrl_handler = handler;
    return 0;
    err_handler_free:
    v4l2_ctrl_handler_free(handler);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2632_probe_dt(ov2732: *mut ov2732) -> c_int {
    static int ov2632_probe_dt(struct ov2732 *ov2732)
    {
    struct fwnode_handle *ep;
    struct fwnode_handle *fwnode = dev_fwnode(ov2732.dev);
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY,
    };
    int ret;
    ep = fwnode_graph_get_endpoint_by_id(fwnode, 0, 0, 0);
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    fwnode_handle_put(ep);
    if (ret) {
    dev_err_probe(ov2732.dev, -EINVAL, "could not parse endpoint\n");
    goto err_probe_dt;
    }
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != OV2732_LANES) {
    dev_err(ov2732.dev, "only a 2-lane CSI2 config is supported\n");
    ret = -EINVAL;
    }
    err_probe_dt:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2732_get_regulators(ov2732: *mut ov2732) -> c_int {
    static int ov2732_get_regulators(struct ov2732 *ov2732)
    {
    for (unsigned int i = 0; i < ARRAY_SIZE(ov2732_supply_names); i++)
    ov2732.supplies[i].supply = ov2732_supply_names[i];
    return devm_regulator_bulk_get(ov2732.dev,
    ARRAY_SIZE(ov2732_supply_names),
    ov2732.supplies);
    }
#[no_mangle]
unsafe extern "C" fn ov2732_probe(client: *mut i2c_client) -> c_int {
    static int ov2732_probe(struct i2c_client *client)
    {
    struct ov2732 *ov2732;
    int ret;
    ov2732 = devm_kzalloc(&client.dev, sizeof(*ov2732), GFP_KERNEL);
    if (!ov2732)
    return -ENOMEM;
    ov2732.dev = &client.dev;
    ret = ov2632_probe_dt(ov2732);
    if (ret)
    return ret;
    ov2732.xvclk = devm_v4l2_sensor_clk_get(ov2732.dev, core::ptr::null_mut());
    if (IS_ERR(ov2732.xvclk))
    return dev_err_probe(ov2732.dev, PTR_ERR(ov2732.xvclk),
    "failed to get xvclk\n");
    ov2732.xvclk_freq = clk_get_rate(ov2732.xvclk);
    if (ov2732.xvclk_freq != OV2732_XVCLK_FREQ)
    return dev_err_probe(ov2732.dev, -EINVAL,
    "xvclk frequency not supported: %dHz\n",
    ov2732.xvclk_freq);
    ov2732.powerdown_gpio = devm_gpiod_get_optional(ov2732.dev,
    "powerdown",
    GPIOD_OUT_HIGH);
    ov2732.reset_gpio = devm_gpiod_get_optional(ov2732.dev, "reset",
    GPIOD_OUT_HIGH);
    ov2732.regmap = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(ov2732.regmap))
    return dev_err_probe(ov2732.dev, PTR_ERR(ov2732.regmap),
    "failed to init CCI\n");
    ret = ov2732_get_regulators(ov2732);
    if (ret)
    return dev_err_probe(ov2732.dev, ret,
    "failed to get regulators\n");
    v4l2_i2c_subdev_init(&ov2732.sd, client, &ov2732_subdev_ops);
// Device must be powered on for ov2732_identify_chip().
    ret = ov2732_power_on(ov2732.dev);
    if (ret)
    return ret;
    pm_runtime_set_active(ov2732.dev);
    pm_runtime_enable(ov2732.dev);
    ret = ov2732_identify_chip(ov2732);
    if (ret)
    goto err_power_off;
    ret = ov2732_init_controls(ov2732);
    if (ret)
    goto err_power_off;
// Initialize subdev
    ov2732.sd.internal_ops = &ov2732_internal_ops;
    ov2732.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ov2732.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
// Initialize source pad
    ov2732.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&ov2732.sd.entity, 1, &ov2732.pad);
    if (ret) {
    dev_err_probe(ov2732.dev, ret, "failed to init entity pads\n");
    goto error_handler_free;
    }
    ov2732.sd.state_lock = ov2732.ctrl_handler.lock;
    ret = v4l2_subdev_init_finalize(&ov2732.sd);
    if (ret < 0) {
    dev_err_probe(ov2732.dev, ret, "subdev init error\n");
    goto err_media_entity;
    }
    ret = v4l2_async_register_subdev_sensor(&ov2732.sd);
    if (ret < 0) {
    dev_err_probe(ov2732.dev, ret,
    "failed to register sensor sub-device\n");
    goto err_subdev_cleanup;
    }
    pm_runtime_set_autosuspend_delay(ov2732.dev, 1000);
    pm_runtime_use_autosuspend(ov2732.dev);
    pm_runtime_idle(ov2732.dev);
    return 0;
    err_subdev_cleanup:
    v4l2_subdev_cleanup(&ov2732.sd);
    err_media_entity:
    media_entity_cleanup(&ov2732.sd.entity);
    error_handler_free:
    v4l2_ctrl_handler_free(&ov2732.ctrl_handler);
    err_power_off:
    pm_runtime_disable(ov2732.dev);
    pm_runtime_set_suspended(ov2732.dev);
    ov2732_power_off(ov2732.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov2732_remove(client: *mut i2c_client) {
    static void ov2732_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov2732 *ov2732 = to_ov2732(sd);
    v4l2_async_unregister_subdev(sd);
    v4l2_subdev_cleanup(sd);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(&ov2732.ctrl_handler);
    pm_runtime_disable(ov2732.dev);
    if (!pm_runtime_status_suspended(ov2732.dev)) {
    ov2732_power_off(ov2732.dev);
    pm_runtime_set_suspended(ov2732.dev);
    }
    }
    static const struct of_device_id ov2732_of_match[] = {
    { .compatible = "ovti,ov2732", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ov2732_of_match);
    static DEFINE_RUNTIME_DEV_PM_OPS(ov2732_pm_ops, ov2732_power_off,
    ov2732_power_on, core::ptr::null_mut());
    static struct i2c_driver ov2732_i2c_driver = {
    .driver = {
    .name = "ov2732",
    .of_match_table = ov2732_of_match,
    .pm = pm_sleep_ptr(&ov2732_pm_ops),
    },
    .probe = ov2732_probe,
    .remove = ov2732_remove,
    };
    module_i2c_driver(ov2732_i2c_driver);
    MODULE_DESCRIPTION("OmniVision ov2732 sensor driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Walter Werner Schneider <contact@schnwalter.eu>");
