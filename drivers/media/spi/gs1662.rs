//! Automatically rewritten from C to Rust
//! Source: drivers/media/spi/gs1662.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// GS1662 device registration.
//
// Copyright (C) 2015-2016 Nexvision
// Author: Charles-Antoine Couret <charles-antoine.couret@nexvision.fr>
//

pub const REG_STATUS: c_uint = 0x04;
pub const REG_FORCE_FMT: c_uint = 0x06;
pub const REG_LINES_PER_FRAME: c_uint = 0x12;
pub const REG_WORDS_PER_LINE: c_uint = 0x13;
pub const REG_WORDS_PER_ACT_LINE: c_uint = 0x14;
pub const REG_ACT_LINES_PER_FRAME: c_uint = 0x15;
pub const MASK_H_LOCK: c_uint = 0x001;
pub const MASK_V_LOCK: c_uint = 0x002;
pub const MASK_STD_LOCK: c_uint = 0x004;
pub const MASK_FORCE_STD: c_uint = 0x020;
pub const MASK_STD_STATUS: c_uint = 0x3E0;
pub const GS_WIDTH_MIN: c_int = 720;
pub const GS_WIDTH_MAX: c_int = 2048;
pub const GS_HEIGHT_MIN: c_int = 487;
pub const GS_HEIGHT_MAX: c_int = 1080;
pub const GS_PIXELCLOCK_MIN: c_int = 10519200;
pub const GS_PIXELCLOCK_MAX: c_int = 74250000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gs {
    pub pdev: *mut spi_device,
    pub sd: v4l2_subdev,
    pub current_timings: v4l2_dv_timings,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gs_reg_fmt {
    pub reg_value: u16,
    pub format: v4l2_dv_timings,
}

    static const struct spi_device_id gs_id[] = {
    { "gs1662", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, gs_id);
    static const struct v4l2_dv_timings fmt_cap[] = {
    V4L2_DV_BT_SDI_720X487I60,
    V4L2_DV_BT_CEA_720X576P50,
    V4L2_DV_BT_CEA_1280X720P24,
    V4L2_DV_BT_CEA_1280X720P25,
    V4L2_DV_BT_CEA_1280X720P30,
    V4L2_DV_BT_CEA_1280X720P50,
    V4L2_DV_BT_CEA_1280X720P60,
    V4L2_DV_BT_CEA_1920X1080P24,
    V4L2_DV_BT_CEA_1920X1080P25,
    V4L2_DV_BT_CEA_1920X1080P30,
    V4L2_DV_BT_CEA_1920X1080I50,
    V4L2_DV_BT_CEA_1920X1080I60,
    };
    static const struct gs_reg_fmt reg_fmt[] = {
    { 0x00, V4L2_DV_BT_CEA_1280X720P60 },
    { 0x01, V4L2_DV_BT_CEA_1280X720P60 },
    { 0x02, V4L2_DV_BT_CEA_1280X720P30 },
    { 0x03, V4L2_DV_BT_CEA_1280X720P30 },
    { 0x04, V4L2_DV_BT_CEA_1280X720P50 },
    { 0x05, V4L2_DV_BT_CEA_1280X720P50 },
    { 0x06, V4L2_DV_BT_CEA_1280X720P25 },
    { 0x07, V4L2_DV_BT_CEA_1280X720P25 },
    { 0x08, V4L2_DV_BT_CEA_1280X720P24 },
    { 0x09, V4L2_DV_BT_CEA_1280X720P24 },
    { 0x0A, V4L2_DV_BT_CEA_1920X1080I60 },
    { 0x0B, V4L2_DV_BT_CEA_1920X1080P30 },
// Default value: keep this field before 0xC
    { 0x14, V4L2_DV_BT_CEA_1920X1080I50 },
    { 0x0C, V4L2_DV_BT_CEA_1920X1080I50 },
    { 0x0D, V4L2_DV_BT_CEA_1920X1080P25 },
    { 0x0E, V4L2_DV_BT_CEA_1920X1080P25 },
    { 0x10, V4L2_DV_BT_CEA_1920X1080P24 },
    { 0x12, V4L2_DV_BT_CEA_1920X1080P24 },
    { 0x16, V4L2_DV_BT_SDI_720X487I60 },
    { 0x19, V4L2_DV_BT_SDI_720X487I60 },
    { 0x18, V4L2_DV_BT_CEA_720X576P50 },
    { 0x1A, V4L2_DV_BT_CEA_720X576P50 },
// Implement following timings before enable it.
// Because of we don't have access to these theoretical timings yet.
// Workaround: use functions to get and set registers for these formats.
//

    { 0x0F, V4L2_DV_BT_XXX_1920X1080I25 }, /* SMPTE 274M */
    { 0x11, V4L2_DV_BT_XXX_1920X1080I24 }, /* SMPTE 274M */
    { 0x13, V4L2_DV_BT_XXX_1920X1080I25 }, /* SMPTE 274M */
    { 0x15, V4L2_DV_BT_XXX_1920X1035I60 }, /* SMPTE 260M */
    { 0x17, V4L2_DV_BT_SDI_720X507I60 }, /* SMPTE 125M */
    { 0x1B, V4L2_DV_BT_SDI_720X507I60 }, /* SMPTE 125M */
    { 0x1C, V4L2_DV_BT_XXX_2048X1080P25 }, /* SMPTE 428.1M */

    };
    static const struct v4l2_dv_timings_cap gs_timings_cap = {
    .type = V4L2_DV_BT_656_1120,
// keep this initialization for compatibility with GCC < 4.4.6
    .reserved = { 0 },
    V4L2_INIT_BT_TIMINGS(GS_WIDTH_MIN, GS_WIDTH_MAX, GS_HEIGHT_MIN,
    GS_HEIGHT_MAX, GS_PIXELCLOCK_MIN,
    GS_PIXELCLOCK_MAX,
    V4L2_DV_BT_STD_CEA861 | V4L2_DV_BT_STD_SDI,
    V4L2_DV_BT_CAP_PROGRESSIVE
    | V4L2_DV_BT_CAP_INTERLACED)
    };
#[no_mangle]
unsafe extern "C" fn gs_read_register(spi: *mut spi_device, addr: u16, value: *mut u16) -> c_int {
    static int gs_read_register(struct spi_device *spi, u16 addr, u16 *value)
    {
    int ret;
    let mut buf_addr: u16 = (0x8000 | (0x0FFF & addr));
    let mut buf_value: u16 = 0;
    struct spi_message msg;
    struct spi_transfer tx[] = {
    {
    .tx_buf = &buf_addr,
    .len = 2,
    .delay = {
    .value = 1,
    .unit = SPI_DELAY_UNIT_USECS
    },
    }, {
    .rx_buf = &buf_value,
    .len = 2,
    .delay = {
    .value = 1,
    .unit = SPI_DELAY_UNIT_USECS
    },
    },
    };
    spi_message_init(&msg);
    spi_message_add_tail(&tx[0], &msg);
    spi_message_add_tail(&tx[1], &msg);
    ret = spi_sync(spi, &msg);
// value = buf_value;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gs_write_register(spi: *mut spi_device, addr: u16, value: u16) -> c_int {
    static int gs_write_register(struct spi_device *spi, u16 addr, u16 value)
    {
    int ret;
    let mut buf_addr: u16 = addr;
    let mut buf_value: u16 = value;
    struct spi_message msg;
    struct spi_transfer tx[] = {
    {
    .tx_buf = &buf_addr,
    .len = 2,
    .delay = {
    .value = 1,
    .unit = SPI_DELAY_UNIT_USECS
    },
    }, {
    .tx_buf = &buf_value,
    .len = 2,
    .delay = {
    .value = 1,
    .unit = SPI_DELAY_UNIT_USECS
    },
    },
    };
    spi_message_init(&msg);
    spi_message_add_tail(&tx[0], &msg);
    spi_message_add_tail(&tx[1], &msg);
    ret = spi_sync(spi, &msg);
    return ret;
    }

    static int gs_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct spi_device *spi = v4l2_get_subdevdata(sd);
    u16 val;
    int ret;
    ret = gs_read_register(spi, reg.reg & 0xFFFF, &val);
    reg.val = val;
    reg.size = 2;
    return ret;
    }
    static int gs_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct spi_device *spi = v4l2_get_subdevdata(sd);
    return gs_write_register(spi, reg.reg & 0xFFFF, reg.val & 0xFFFF);
    }

#[no_mangle]
unsafe extern "C" fn gs_status_format(status: u16, timings: *mut v4l2_dv_timings) -> c_int {
    static int gs_status_format(u16 status, struct v4l2_dv_timings *timings)
    {
    let mut std: c_int = (status & MASK_STD_STATUS) >> 5;
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_fmt); i++) {
    if (reg_fmt[i].reg_value == std) {
// timings = reg_fmt[i].format;
    return 0;
    }
    }
    return -ERANGE;
    }
#[no_mangle]
unsafe extern "C" fn get_register_timings(timings: *mut v4l2_dv_timings) -> u16 {
    static u16 get_register_timings(struct v4l2_dv_timings *timings)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(reg_fmt); i++) {
    if (v4l2_match_dv_timings(timings, &reg_fmt[i].format, 0,
    false))
    return reg_fmt[i].reg_value | MASK_FORCE_STD;
    }
    return 0x0;
    }
    static inline struct gs *to_gs(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct gs, sd);
    }
    static int gs_s_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    struct gs *gs = to_gs(sd);
    int reg_value;
    if (pad != 0)
    return -EINVAL;
    reg_value = get_register_timings(timings);
    if (reg_value == 0x0)
    return -EINVAL;
    gs.current_timings = *timings;
    return 0;
    }
    static int gs_g_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    struct gs *gs = to_gs(sd);
    if (pad != 0)
    return -EINVAL;
// timings = gs->current_timings;
    return 0;
    }
    static int gs_query_dv_timings(struct v4l2_subdev *sd, unsigned int pad,
    struct v4l2_dv_timings *timings)
    {
    struct gs *gs = to_gs(sd);
    struct v4l2_dv_timings fmt;
    u16 reg_value, i;
    int ret;
    if (pad != 0)
    return -EINVAL;
    if (gs.enabled)
    return -EBUSY;
//
// Check if the component detect a line, a frame or something else
// which looks like a video signal activity.
//
    for (i = 0; i < 4; i++) {
    gs_read_register(gs.pdev, REG_LINES_PER_FRAME + i, &reg_value);
    if (reg_value)
    break;
    }
// If no register reports a video signal
    if (i >= 4)
    return -ENOLINK;
    gs_read_register(gs.pdev, REG_STATUS, &reg_value);
    if (!(reg_value & MASK_H_LOCK) || !(reg_value & MASK_V_LOCK))
    return -ENOLCK;
    if (!(reg_value & MASK_STD_LOCK))
    return -ERANGE;
    ret = gs_status_format(reg_value, &fmt);
    if (ret < 0)
    return ret;
// timings = fmt;
    return 0;
    }
    static int gs_enum_dv_timings(struct v4l2_subdev *sd,
    struct v4l2_enum_dv_timings *timings)
    {
    if (timings.index >= ARRAY_SIZE(fmt_cap))
    return -EINVAL;
    if (timings.pad != 0)
    return -EINVAL;
    timings.timings = fmt_cap[timings.index];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gs_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int gs_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct gs *gs = to_gs(sd);
    int reg_value;
    if (gs.enabled == enable)
    return 0;
    gs.enabled = enable;
    if (enable) {
// To force the specific format
    reg_value = get_register_timings(&gs.current_timings);
    return gs_write_register(gs.pdev, REG_FORCE_FMT, reg_value);
    }
// To renable auto-detection mode
    return gs_write_register(gs.pdev, REG_FORCE_FMT, 0x0);
    }
#[no_mangle]
unsafe extern "C" fn gs_g_input_status(sd: *mut v4l2_subdev, status: *mut u32) -> c_int {
    static int gs_g_input_status(struct v4l2_subdev *sd, u32 *status)
    {
    struct gs *gs = to_gs(sd);
    u16 reg_value, i;
    int ret;
//
// Check if the component detect a line, a frame or something else
// which looks like a video signal activity.
//
    for (i = 0; i < 4; i++) {
    ret = gs_read_register(gs.pdev,
    REG_LINES_PER_FRAME + i, &reg_value);
    if (reg_value)
    break;
    if (ret) {
// status = V4L2_IN_ST_NO_POWER;
    return ret;
    }
    }
// If no register reports a video signal
    if (i >= 4)
// status |= V4L2_IN_ST_NO_SIGNAL;
    ret = gs_read_register(gs.pdev, REG_STATUS, &reg_value);
    if (!(reg_value & MASK_H_LOCK))
// status |=  V4L2_IN_ST_NO_H_LOCK;
    if (!(reg_value & MASK_V_LOCK))
// status |=  V4L2_IN_ST_NO_V_LOCK;
    if (!(reg_value & MASK_STD_LOCK))
// status |=  V4L2_IN_ST_NO_STD_LOCK;
    return ret;
    }
    static int gs_dv_timings_cap(struct v4l2_subdev *sd,
    struct v4l2_dv_timings_cap *cap)
    {
    if (cap.pad != 0)
    return -EINVAL;
// cap = gs_timings_cap;
    return 0;
    }
// V4L2 core operation handlers
    static const struct v4l2_subdev_core_ops gs_core_ops = {

    .g_register = gs_g_register,
    .s_register = gs_s_register,

    };
    static const struct v4l2_subdev_video_ops gs_video_ops = {
    .s_stream = gs_s_stream,
    .g_input_status = gs_g_input_status,
    };
    static const struct v4l2_subdev_pad_ops gs_pad_ops = {
    .s_dv_timings = gs_s_dv_timings,
    .g_dv_timings = gs_g_dv_timings,
    .query_dv_timings = gs_query_dv_timings,
    .enum_dv_timings = gs_enum_dv_timings,
    .dv_timings_cap = gs_dv_timings_cap,
    };
// V4L2 top level operation handlers
    static const struct v4l2_subdev_ops gs_ops = {
    .core = &gs_core_ops,
    .video = &gs_video_ops,
    .pad = &gs_pad_ops,
    };
#[no_mangle]
unsafe extern "C" fn gs_probe(spi: *mut spi_device) -> c_int {
    static int gs_probe(struct spi_device *spi)
    {
    int ret;
    struct gs *gs;
    struct v4l2_subdev *sd;
    gs = devm_kzalloc(&spi.dev, sizeof(struct gs), GFP_KERNEL);
    if (!gs)
    return -ENOMEM;
    gs.pdev = spi;
    sd = &gs.sd;
    spi.mode = SPI_MODE_0;
    spi.irq = -1;
    spi.max_speed_hz = 10000000;
    spi.bits_per_word = 16;
    ret = spi_setup(spi);
    v4l2_spi_subdev_init(sd, spi, &gs_ops);
    gs.current_timings = reg_fmt[0].format;
    gs.enabled = 0;
// Set H_CONFIG to SMPTE timings
    gs_write_register(spi, 0x0, 0x300);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gs_remove(spi: *mut spi_device) {
    static void gs_remove(struct spi_device *spi)
    {
    struct v4l2_subdev *sd = spi_get_drvdata(spi);
    v4l2_device_unregister_subdev(sd);
    }
    static struct spi_driver gs_driver = {
    .driver = {
    .name		= "gs1662",
    },
    .probe		= gs_probe,
    .remove		= gs_remove,
    .id_table	= gs_id,
    };
    module_spi_driver(gs_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Charles-Antoine Couret <charles-antoine.couret@nexvision.fr>");
    MODULE_DESCRIPTION("Gennum GS1662 HD/SD-SDI Serializer driver");
