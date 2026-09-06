//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/mt9v011.c
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
// mt9v011 -Micron 1/4-Inch VGA Digital Image Sensor
//
// Copyright (c) 2009 Mauro Carvalho Chehab <mchehab@kernel.org>

    MODULE_DESCRIPTION("Micron mt9v011 sensor driver");
    MODULE_AUTHOR("Mauro Carvalho Chehab");
    MODULE_LICENSE("GPL v2");
    static int debug;
    module_param(debug, int, 0);
    MODULE_PARM_DESC(debug, "Debug level (0-2)");
pub const R00_MT9V011_CHIP_VERSION: c_uint = 0x00;
pub const R01_MT9V011_ROWSTART: c_uint = 0x01;
pub const R02_MT9V011_COLSTART: c_uint = 0x02;
pub const R03_MT9V011_HEIGHT: c_uint = 0x03;
pub const R04_MT9V011_WIDTH: c_uint = 0x04;
pub const R05_MT9V011_HBLANK: c_uint = 0x05;
pub const R06_MT9V011_VBLANK: c_uint = 0x06;
pub const R07_MT9V011_OUT_CTRL: c_uint = 0x07;
pub const R09_MT9V011_SHUTTER_WIDTH: c_uint = 0x09;
pub const R0A_MT9V011_CLK_SPEED: c_uint = 0x0a;
pub const R0B_MT9V011_RESTART: c_uint = 0x0b;
pub const R0C_MT9V011_SHUTTER_DELAY: c_uint = 0x0c;
pub const R0D_MT9V011_RESET: c_uint = 0x0d;
pub const R1E_MT9V011_DIGITAL_ZOOM: c_uint = 0x1e;
pub const R20_MT9V011_READ_MODE: c_uint = 0x20;
pub const R2B_MT9V011_GREEN_1_GAIN: c_uint = 0x2b;
pub const R2C_MT9V011_BLUE_GAIN: c_uint = 0x2c;
pub const R2D_MT9V011_RED_GAIN: c_uint = 0x2d;
pub const R2E_MT9V011_GREEN_2_GAIN: c_uint = 0x2e;
pub const R35_MT9V011_GLOBAL_GAIN: c_uint = 0x35;
pub const RF1_MT9V011_CHIP_ENABLE: c_uint = 0xf1;
pub const MT9V011_VERSION: c_uint = 0x8232;
pub const MT9V011_REV_B_VERSION: c_uint = 0x8243;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9v011 {
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub ctrls: v4l2_ctrl_handler,
    pub height: unsigned width,,
    pub xtal: unsigned,
    pub hflip:1: unsigned,
    pub vflip:1: unsigned,
    pub exposure: u16 global_gain,,
    pub blue_bal: s16 red_bal,,
}

    static inline struct mt9v011 *to_mt9v011(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct mt9v011, sd);
    }
#[no_mangle]
unsafe extern "C" fn mt9v011_read(sd: *mut v4l2_subdev, addr: c_uchar) -> c_int {
    static int mt9v011_read(struct v4l2_subdev *sd, unsigned char addr)
    {
    struct i2c_client *c = v4l2_get_subdevdata(sd);
    __be16 buffer;
    int rc, val;
    rc = i2c_master_send(c, &addr, 1);
    if (rc != 1)
    v4l2_dbg(0, debug, sd,
    "i2c i/o error: rc == %d (should be 1)\n", rc);
    msleep(10);
    rc = i2c_master_recv(c, (char *)&buffer, 2);
    if (rc != 2)
    v4l2_dbg(0, debug, sd,
    "i2c i/o error: rc == %d (should be 2)\n", rc);
    val = be16_to_cpu(buffer);
    v4l2_dbg(2, debug, sd, "mt9v011: read 0x%02x = 0x%04x\n", addr, val);
    return val;
    }
    static void mt9v011_write(struct v4l2_subdev *sd, unsigned char addr,
    u16 value)
    {
    struct i2c_client *c = v4l2_get_subdevdata(sd);
    unsigned char buffer[3];
    int rc;
    buffer[0] = addr;
    buffer[1] = value >> 8;
    buffer[2] = value & 0xff;
    v4l2_dbg(2, debug, sd,
    "mt9v011: writing 0x%02x 0x%04x\n", buffer[0], value);
    rc = i2c_master_send(c, buffer, 3);
    if (rc != 3)
    v4l2_dbg(0, debug, sd,
    "i2c i/o error: rc == %d (should be 3)\n", rc);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_reg_value {
    pub reg: c_uchar,
    pub value: u16,
}

//
// Values used at the original driver
// Some values are marked as Reserved at the datasheet
//
    static const struct i2c_reg_value mt9v011_init_default[] = {
    { R0D_MT9V011_RESET, 0x0001 },
    { R0D_MT9V011_RESET, 0x0000 },
    { R0C_MT9V011_SHUTTER_DELAY, 0x0000 },
    { R09_MT9V011_SHUTTER_WIDTH, 0x1fc },
    { R0A_MT9V011_CLK_SPEED, 0x0000 },
    { R1E_MT9V011_DIGITAL_ZOOM,  0x0000 },
    { R07_MT9V011_OUT_CTRL, 0x0002 },	/* chip enable */
    };
#[no_mangle]
unsafe extern "C" fn calc_mt9v011_gain(lineargain: i16) -> u16 {
    static u16 calc_mt9v011_gain(s16 lineargain)
    {
    let mut digitalgain: u16 = 0;
    let mut analogmult: u16 = 0;
    let mut analoginit: u16 = 0;
    if (lineargain < 0)
    lineargain = 0;
// recommended minimum
    lineargain += 0x0020;
    if (lineargain > 2047)
    lineargain = 2047;
    if (lineargain > 1023) {
    digitalgain = 3;
    analogmult = 3;
    analoginit = lineargain / 16;
    } else if (lineargain > 511) {
    digitalgain = 1;
    analogmult = 3;
    analoginit = lineargain / 8;
    } else if (lineargain > 255) {
    analogmult = 3;
    analoginit = lineargain / 4;
    } else if (lineargain > 127) {
    analogmult = 1;
    analoginit = lineargain / 2;
    } else
    analoginit = lineargain;
    return analoginit + (analogmult << 7) + (digitalgain << 9);
    }
#[no_mangle]
unsafe extern "C" fn set_balance(sd: *mut v4l2_subdev) {
    static void set_balance(struct v4l2_subdev *sd)
    {
    struct mt9v011 *core = to_mt9v011(sd);
    u16 green_gain, blue_gain, red_gain;
    u16 exposure;
    s16 bal;
    exposure = core.exposure;
    green_gain = calc_mt9v011_gain(core.global_gain);
    bal = core.global_gain;
    bal += (core.blue_bal * core.global_gain / (1 << 7));
    blue_gain = calc_mt9v011_gain(bal);
    bal = core.global_gain;
    bal += (core.red_bal * core.global_gain / (1 << 7));
    red_gain = calc_mt9v011_gain(bal);
    mt9v011_write(sd, R2B_MT9V011_GREEN_1_GAIN, green_gain);
    mt9v011_write(sd, R2E_MT9V011_GREEN_2_GAIN, green_gain);
    mt9v011_write(sd, R2C_MT9V011_BLUE_GAIN, blue_gain);
    mt9v011_write(sd, R2D_MT9V011_RED_GAIN, red_gain);
    mt9v011_write(sd, R09_MT9V011_SHUTTER_WIDTH, exposure);
    }
#[no_mangle]
unsafe extern "C" fn calc_fps(sd: *mut v4l2_subdev, numerator: *mut u32, denominator: *mut u32) {
    static void calc_fps(struct v4l2_subdev *sd, u32 *numerator, u32 *denominator)
    {
    struct mt9v011 *core = to_mt9v011(sd);
    unsigned height, width, hblank, vblank, speed;
    unsigned row_time, t_time;
    u64 frames_per_ms;
    unsigned tmp;
    height = mt9v011_read(sd, R03_MT9V011_HEIGHT);
    width = mt9v011_read(sd, R04_MT9V011_WIDTH);
    hblank = mt9v011_read(sd, R05_MT9V011_HBLANK);
    vblank = mt9v011_read(sd, R06_MT9V011_VBLANK);
    speed = mt9v011_read(sd, R0A_MT9V011_CLK_SPEED);
    row_time = (width + 113 + hblank) * (speed + 2);
    t_time = row_time * (height + vblank + 1);
    frames_per_ms = core.xtal * 1000l;
    do_div(frames_per_ms, t_time);
    tmp = frames_per_ms;
    v4l2_dbg(1, debug, sd, "Programmed to %u.%03u fps (%d pixel clcks)\n",
    tmp / 1000, tmp % 1000, t_time);
    if (numerator && denominator) {
// numerator = 1000;
// denominator = (u32)frames_per_ms;
    }
    }
#[no_mangle]
unsafe extern "C" fn calc_speed(sd: *mut v4l2_subdev, numerator: u32, denominator: u32) -> u16 {
    static u16 calc_speed(struct v4l2_subdev *sd, u32 numerator, u32 denominator)
    {
    struct mt9v011 *core = to_mt9v011(sd);
    unsigned height, width, hblank, vblank;
    unsigned row_time, line_time;
    u64 t_time, speed;
// Avoid bogus calculus
    if (!numerator || !denominator)
    return 0;
    height = mt9v011_read(sd, R03_MT9V011_HEIGHT);
    width = mt9v011_read(sd, R04_MT9V011_WIDTH);
    hblank = mt9v011_read(sd, R05_MT9V011_HBLANK);
    vblank = mt9v011_read(sd, R06_MT9V011_VBLANK);
    row_time = width + 113 + hblank;
    line_time = height + vblank + 1;
    t_time = core.xtal * ((u64)numerator);
// round to the closest value
    t_time += denominator / 2;
    do_div(t_time, denominator);
    speed = t_time;
    do_div(speed, row_time * line_time);
// Avoid having a negative value for speed
    if (speed < 2)
    speed = 0;
    else
    speed -= 2;
// Avoid speed overflow
    if (speed > 15)
    return 15;
    return (u16)speed;
    }
#[no_mangle]
unsafe extern "C" fn set_res(sd: *mut v4l2_subdev) {
    static void set_res(struct v4l2_subdev *sd)
    {
    struct mt9v011 *core = to_mt9v011(sd);
    unsigned vstart, hstart;
//
// The mt9v011 doesn't have scaling. So, in order to select the desired
// resolution, we're cropping at the middle of the sensor.
// hblank and vblank should be adjusted, in order to warrant that
// we'll preserve the line timings for 30 fps, no matter what resolution
// is selected.
// NOTE: datasheet says that width (and height) should be filled with
// width-1. However, this doesn't work, since one pixel per line will
// be missing.
//
    hstart = 20 + (640 - core.width) / 2;
    mt9v011_write(sd, R02_MT9V011_COLSTART, hstart);
    mt9v011_write(sd, R04_MT9V011_WIDTH, core.width);
    mt9v011_write(sd, R05_MT9V011_HBLANK, 771 - core.width);
    vstart = 8 + (480 - core.height) / 2;
    mt9v011_write(sd, R01_MT9V011_ROWSTART, vstart);
    mt9v011_write(sd, R03_MT9V011_HEIGHT, core.height);
    mt9v011_write(sd, R06_MT9V011_VBLANK, 508 - core.height);
    calc_fps(sd, core::ptr::null_mut(), core::ptr::null_mut());
    };
#[no_mangle]
unsafe extern "C" fn set_read_mode(sd: *mut v4l2_subdev) {
    static void set_read_mode(struct v4l2_subdev *sd)
    {
    struct mt9v011 *core = to_mt9v011(sd);
    let mut mode: unsigned = 0x1000;
    if (core.hflip)
    mode |= 0x4000;
    if (core.vflip)
    mode |= 0x8000;
    mt9v011_write(sd, R20_MT9V011_READ_MODE, mode);
    }
#[no_mangle]
unsafe extern "C" fn mt9v011_reset(sd: *mut v4l2_subdev, val: u32) -> c_int {
    static int mt9v011_reset(struct v4l2_subdev *sd, u32 val)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(mt9v011_init_default); i++)
    mt9v011_write(sd, mt9v011_init_default[i].reg,
    mt9v011_init_default[i].value);
    set_balance(sd);
    set_res(sd);
    set_read_mode(sd);
    return 0;
    }
    static int mt9v011_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SGRBG8_1X8;
    return 0;
    }
    static int mt9v011_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *fmt = &format.format;
    struct mt9v011 *core = to_mt9v011(sd);
    if (format.pad || fmt.code != MEDIA_BUS_FMT_SGRBG8_1X8)
    return -EINVAL;
    v4l_bound_align_image(&fmt.width, 48, 639, 1,
    &fmt.height, 32, 480, 1, 0);
    fmt.field = V4L2_FIELD_NONE;
    fmt.colorspace = V4L2_COLORSPACE_SRGB;
    if (format.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    core.width = fmt.width;
    core.height = fmt.height;
    set_res(sd);
    } else {
// v4l2_subdev_state_get_format(sd_state, 0) = *fmt;
    }
    return 0;
    }
    static int mt9v011_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *ival)
    {
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (ival.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    calc_fps(sd,
    &ival.interval.numerator,
    &ival.interval.denominator);
    return 0;
    }
    static int mt9v011_set_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *ival)
    {
    struct v4l2_fract *tpf = &ival.interval;
    u16 speed;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (ival.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    speed = calc_speed(sd, tpf.numerator, tpf.denominator);
    mt9v011_write(sd, R0A_MT9V011_CLK_SPEED, speed);
    v4l2_dbg(1, debug, sd, "Setting speed to %d\n", speed);
// Recalculate and update fps info
    calc_fps(sd, &tpf.numerator, &tpf.denominator);
    return 0;
    }

    static int mt9v011_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    reg.val = mt9v011_read(sd, reg.reg & 0xff);
    reg.size = 2;
    return 0;
    }
    static int mt9v011_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    mt9v011_write(sd, reg.reg & 0xff, reg.val & 0xffff);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn mt9v011_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int mt9v011_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct mt9v011 *core =
    container_of(ctrl.handler, struct mt9v011, ctrls);
    struct v4l2_subdev *sd = &core.sd;
    switch (ctrl.id) {
    case V4L2_CID_GAIN:
    core.global_gain = ctrl.val;
    break;
    case V4L2_CID_EXPOSURE:
    core.exposure = ctrl.val;
    break;
    case V4L2_CID_RED_BALANCE:
    core.red_bal = ctrl.val;
    break;
    case V4L2_CID_BLUE_BALANCE:
    core.blue_bal = ctrl.val;
    break;
    case V4L2_CID_HFLIP:
    core.hflip = ctrl.val;
    set_read_mode(sd);
    return 0;
    case V4L2_CID_VFLIP:
    core.vflip = ctrl.val;
    set_read_mode(sd);
    return 0;
    default:
    return -EINVAL;
    }
    set_balance(sd);
    return 0;
    }
    static const struct v4l2_ctrl_ops mt9v011_ctrl_ops = {
    .s_ctrl = mt9v011_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops mt9v011_core_ops = {
    .reset = mt9v011_reset,

    .g_register = mt9v011_g_register,
    .s_register = mt9v011_s_register,

    };
    static const struct v4l2_subdev_pad_ops mt9v011_pad_ops = {
    .enum_mbus_code = mt9v011_enum_mbus_code,
    .set_fmt = mt9v011_set_fmt,
    .get_frame_interval = mt9v011_get_frame_interval,
    .set_frame_interval = mt9v011_set_frame_interval,
    };
    static const struct v4l2_subdev_ops mt9v011_ops = {
    .core  = &mt9v011_core_ops,
    .pad   = &mt9v011_pad_ops,
    };
//
    I2C Client & Driver
//
#[no_mangle]
unsafe extern "C" fn mt9v011_probe(c: *mut i2c_client) -> c_int {
    static int mt9v011_probe(struct i2c_client *c)
    {
    u16 version;
    struct mt9v011 *core;
    struct v4l2_subdev *sd;
    int ret;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(c.adapter,
    I2C_FUNC_SMBUS_READ_BYTE | I2C_FUNC_SMBUS_WRITE_BYTE_DATA))
    return -EIO;
    core = devm_kzalloc(&c.dev, sizeof(struct mt9v011), GFP_KERNEL);
    if (!core)
    return -ENOMEM;
    sd = &core.sd;
    v4l2_i2c_subdev_init(sd, c, &mt9v011_ops);
    core.pad.flags = MEDIA_PAD_FL_SOURCE;
    sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&sd.entity, 1, &core.pad);
    if (ret < 0)
    return ret;
// Check if the sensor is really a MT9V011
    version = mt9v011_read(sd, R00_MT9V011_CHIP_VERSION);
    if ((version != MT9V011_VERSION) &&
    (version != MT9V011_REV_B_VERSION)) {
    v4l2_info(sd, "*** unknown micron chip detected (0x%04x).\n",
    version);
    media_entity_cleanup(&sd.entity);
    return -EINVAL;
    }
    v4l2_ctrl_handler_init(&core.ctrls, 5);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_GAIN, 0, (1 << 12) - 1 - 0x20, 1, 0x20);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_EXPOSURE, 0, 2047, 1, 0x01fc);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_RED_BALANCE, -(1 << 9), (1 << 9) - 1, 1, 0);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_BLUE_BALANCE, -(1 << 9), (1 << 9) - 1, 1, 0);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(&core.ctrls, &mt9v011_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    if (core.ctrls.error) {
    let mut ret: c_int = core.ctrls.error;
    v4l2_err(sd, "control initialization error %d\n", ret);
    v4l2_ctrl_handler_free(&core.ctrls);
    media_entity_cleanup(&sd.entity);
    return ret;
    }
    core.sd.ctrl_handler = &core.ctrls;
    core.global_gain = 0x0024;
    core.exposure = 0x01fc;
    core.width  = 640;
    core.height = 480;
    core.xtal = 27000000;	/* Hz */
    if (c.dev.platform_data) {
    struct mt9v011_platform_data *pdata = c.dev.platform_data;
    core.xtal = pdata.xtal;
    v4l2_dbg(1, debug, sd, "xtal set to %d.%03d MHz\n",
    core.xtal / 1000000, (core.xtal / 1000) % 1000);
    }
    v4l_info(c, "chip found @ 0x%02x (%s - chip version 0x%04x)\n",
    c.addr << 1, c.adapter.name, version);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt9v011_remove(c: *mut i2c_client) {
    static void mt9v011_remove(struct i2c_client *c)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(c);
    struct mt9v011 *core = to_mt9v011(sd);
    v4l2_dbg(1, debug, sd,
    "mt9v011.c: removing mt9v011 adapter on address 0x%x\n",
    c.addr << 1);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&core.ctrls);
    media_entity_cleanup(&sd.entity);
    }
// -----------------------------------------------------------------------
    static const struct i2c_device_id mt9v011_id[] = {
    { .name = "mt9v011" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mt9v011_id);
    static struct i2c_driver mt9v011_driver = {
    .driver = {
    .name	= "mt9v011",
    },
    .probe		= mt9v011_probe,
    .remove		= mt9v011_remove,
    .id_table	= mt9v011_id,
    };
    module_i2c_driver(mt9v011_driver);
