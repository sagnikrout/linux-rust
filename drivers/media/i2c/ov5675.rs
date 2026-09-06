//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov5675.c
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
// Copyright (c) 2019 Intel Corporation.

pub const OV5675_REG_VALUE_08BIT: c_int = 1;
pub const OV5675_REG_VALUE_16BIT: c_int = 2;
pub const OV5675_REG_VALUE_24BIT: c_int = 3;

pub const OV5675_XVCLK_19_2: c_int = 19200000;
pub const OV5675_DATA_LANES: c_int = 2;
pub const OV5675_RGB_DEPTH: c_int = 10;
pub const OV5675_REG_CHIP_ID: c_uint = 0x300a;
pub const OV5675_CHIP_ID: c_uint = 0x5675;
pub const OV5675_REG_MODE_SELECT: c_uint = 0x0100;
pub const OV5675_MODE_STANDBY: c_uint = 0x00;
pub const OV5675_MODE_STREAMING: c_uint = 0x01;
// vertical-timings from sensor
pub const OV5675_REG_VTS: c_uint = 0x380e;
pub const OV5675_VTS_30FPS: c_uint = 0x07e4;
pub const OV5675_VTS_30FPS_MIN: c_uint = 0x07e4;
pub const OV5675_VTS_MAX: c_uint = 0x7fff;
// horizontal-timings from sensor
pub const OV5675_REG_HTS: c_uint = 0x380c;
// Exposure controls from sensor
pub const OV5675_REG_EXPOSURE: c_uint = 0x3500;
pub const OV5675_EXPOSURE_MIN: c_int = 4;
pub const OV5675_EXPOSURE_MAX_MARGIN: c_int = 4;
pub const OV5675_EXPOSURE_STEP: c_int = 1;
// Analog gain controls from sensor
pub const OV5675_REG_ANALOG_GAIN: c_uint = 0x3508;
pub const OV5675_ANAL_GAIN_MIN: c_int = 128;
pub const OV5675_ANAL_GAIN_MAX: c_int = 2047;
pub const OV5675_ANAL_GAIN_STEP: c_int = 1;
// Digital gain controls from sensor
pub const OV5675_REG_DIGITAL_GAIN: c_uint = 0x350a;
pub const OV5675_REG_MWB_R_GAIN: c_uint = 0x5019;
pub const OV5675_REG_MWB_G_GAIN: c_uint = 0x501b;
pub const OV5675_REG_MWB_B_GAIN: c_uint = 0x501d;
pub const OV5675_DGTL_GAIN_MIN: c_int = 1024;
pub const OV5675_DGTL_GAIN_MAX: c_int = 4095;
pub const OV5675_DGTL_GAIN_STEP: c_int = 1;
pub const OV5675_DGTL_GAIN_DEFAULT: c_int = 1024;
// Group Access
pub const OV5675_REG_GROUP_ACCESS: c_uint = 0x3208;
pub const OV5675_GROUP_HOLD_START: c_uint = 0x0;
pub const OV5675_GROUP_HOLD_END: c_uint = 0x10;
pub const OV5675_GROUP_HOLD_LAUNCH: c_uint = 0xa0;
// Test Pattern Control
pub const OV5675_REG_TEST_PATTERN: c_uint = 0x4503;

pub const OV5675_TEST_PATTERN_BAR_SHIFT: c_int = 2;
// Flip Mirror Controls from sensor
pub const OV5675_REG_FORMAT1: c_uint = 0x3820;
pub const OV5675_REG_FORMAT2: c_uint = 0x373d;

    static const char * const ov5675_supply_names[] = {
    "avdd",		/* Analog power */
    "dovdd",	/* Digital I/O power */
    "dvdd",		/* Digital core power */
    };

    enum {
    OV5675_LINK_FREQ_900MBPS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5675_reg {
    pub address: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5675_reg_list {
    pub num_of_regs: u32,
    pub regs: *const ov5675_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5675_link_freq_config {
    pub reg_list: ov5675_reg_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5675_mode {
// Frame width in pixels
    pub width: u32,
// Frame height in pixels
    pub height: u32,
// Horizontal timining size
    pub hts: u32,
// Default vertical timining size
    pub vts_def: u32,
// Min vertical timining size
    pub vts_min: u32,
// Link frequency needed for this resolution
    pub link_freq_index: u32,
// Sensor register settings for this resolution
    pub reg_list: ov5675_reg_list,
}

    static const struct ov5675_reg mipi_data_rate_900mbps[] = {
    {0x0103, 0x01},
    {0x0100, 0x00},
    {0x0300, 0x04},
    {0x0302, 0x8d},
    {0x0303, 0x00},
    {0x030d, 0x26},
    };
    static const struct ov5675_reg mode_2592x1944_regs[] = {
    {0x3002, 0x21},
    {0x3107, 0x23},
    {0x3501, 0x20},
    {0x3503, 0x0c},
    {0x3508, 0x03},
    {0x3509, 0x00},
    {0x3600, 0x66},
    {0x3602, 0x30},
    {0x3610, 0xa5},
    {0x3612, 0x93},
    {0x3620, 0x80},
    {0x3642, 0x0e},
    {0x3661, 0x00},
    {0x3662, 0x10},
    {0x3664, 0xf3},
    {0x3665, 0x9e},
    {0x3667, 0xa5},
    {0x366e, 0x55},
    {0x366f, 0x55},
    {0x3670, 0x11},
    {0x3671, 0x11},
    {0x3672, 0x11},
    {0x3673, 0x11},
    {0x3714, 0x24},
    {0x371a, 0x3e},
    {0x3733, 0x10},
    {0x3734, 0x00},
    {0x373d, 0x24},
    {0x3764, 0x20},
    {0x3765, 0x20},
    {0x3766, 0x12},
    {0x37a1, 0x14},
    {0x37a8, 0x1c},
    {0x37ab, 0x0f},
    {0x37c2, 0x04},
    {0x37cb, 0x00},
    {0x37cc, 0x00},
    {0x37cd, 0x00},
    {0x37ce, 0x00},
    {0x37d8, 0x02},
    {0x37d9, 0x08},
    {0x37dc, 0x04},
    {0x3800, 0x00},
    {0x3801, 0x00},
    {0x3802, 0x00},
    {0x3803, 0x04},
    {0x3804, 0x0a},
    {0x3805, 0x3f},
    {0x3806, 0x07},
    {0x3807, 0xb3},
    {0x3808, 0x0a},
    {0x3809, 0x20},
    {0x380a, 0x07},
    {0x380b, 0x98},
    {0x380c, 0x02},
    {0x380d, 0xee},
    {0x380e, 0x07},
    {0x380f, 0xe4},
    {0x3811, 0x10},
    {0x3813, 0x0d},
    {0x3814, 0x01},
    {0x3815, 0x01},
    {0x3816, 0x01},
    {0x3817, 0x01},
    {0x381e, 0x02},
    {0x3820, 0x88},
    {0x3821, 0x01},
    {0x3832, 0x04},
    {0x3c80, 0x01},
    {0x3c82, 0x00},
    {0x3c83, 0xc8},
    {0x3c8c, 0x0f},
    {0x3c8d, 0xa0},
    {0x3c90, 0x07},
    {0x3c91, 0x00},
    {0x3c92, 0x00},
    {0x3c93, 0x00},
    {0x3c94, 0xd0},
    {0x3c95, 0x50},
    {0x3c96, 0x35},
    {0x3c97, 0x00},
    {0x4001, 0xe0},
    {0x4008, 0x02},
    {0x4009, 0x0d},
    {0x400f, 0x80},
    {0x4013, 0x02},
    {0x4040, 0x00},
    {0x4041, 0x07},
    {0x404c, 0x50},
    {0x404e, 0x20},
    {0x4500, 0x06},
    {0x4503, 0x00},
    {0x450a, 0x04},
    {0x4809, 0x04},
    {0x480c, 0x12},
    {0x4819, 0x70},
    {0x4825, 0x32},
    {0x4826, 0x32},
    {0x482a, 0x06},
    {0x4833, 0x08},
    {0x4837, 0x0d},
    {0x5000, 0x77},
    {0x5b00, 0x01},
    {0x5b01, 0x10},
    {0x5b02, 0x01},
    {0x5b03, 0xdb},
    {0x5b05, 0x6c},
    {0x5e10, 0xfc},
    {0x3500, 0x00},
    {0x3501, 0x3E},
    {0x3502, 0x60},
    {0x3503, 0x08},
    {0x3508, 0x04},
    {0x3509, 0x00},
    {0x3832, 0x48},
    {0x5780, 0x3e},
    {0x5781, 0x0f},
    {0x5782, 0x44},
    {0x5783, 0x02},
    {0x5784, 0x01},
    {0x5785, 0x01},
    {0x5786, 0x00},
    {0x5787, 0x04},
    {0x5788, 0x02},
    {0x5789, 0x0f},
    {0x578a, 0xfd},
    {0x578b, 0xf5},
    {0x578c, 0xf5},
    {0x578d, 0x03},
    {0x578e, 0x08},
    {0x578f, 0x0c},
    {0x5790, 0x08},
    {0x5791, 0x06},
    {0x5792, 0x00},
    {0x5793, 0x52},
    {0x5794, 0xa3},
    {0x4003, 0x40},
    {0x3107, 0x01},
    {0x3c80, 0x08},
    {0x3c83, 0xb1},
    {0x3c8c, 0x10},
    {0x3c8d, 0x00},
    {0x3c90, 0x00},
    {0x3c94, 0x00},
    {0x3c95, 0x00},
    {0x3c96, 0x00},
    {0x37cb, 0x09},
    {0x37cc, 0x15},
    {0x37cd, 0x1f},
    {0x37ce, 0x1f},
    };
    static const struct ov5675_reg mode_1296x972_regs[] = {
    {0x3002, 0x21},
    {0x3107, 0x23},
    {0x3501, 0x20},
    {0x3503, 0x0c},
    {0x3508, 0x03},
    {0x3509, 0x00},
    {0x3600, 0x66},
    {0x3602, 0x30},
    {0x3610, 0xa5},
    {0x3612, 0x93},
    {0x3620, 0x80},
    {0x3642, 0x0e},
    {0x3661, 0x00},
    {0x3662, 0x08},
    {0x3664, 0xf3},
    {0x3665, 0x9e},
    {0x3667, 0xa5},
    {0x366e, 0x55},
    {0x366f, 0x55},
    {0x3670, 0x11},
    {0x3671, 0x11},
    {0x3672, 0x11},
    {0x3673, 0x11},
    {0x3714, 0x28},
    {0x371a, 0x3e},
    {0x3733, 0x10},
    {0x3734, 0x00},
    {0x373d, 0x24},
    {0x3764, 0x20},
    {0x3765, 0x20},
    {0x3766, 0x12},
    {0x37a1, 0x14},
    {0x37a8, 0x1c},
    {0x37ab, 0x0f},
    {0x37c2, 0x14},
    {0x37cb, 0x00},
    {0x37cc, 0x00},
    {0x37cd, 0x00},
    {0x37ce, 0x00},
    {0x37d8, 0x02},
    {0x37d9, 0x04},
    {0x37dc, 0x04},
    {0x3800, 0x00},
    {0x3801, 0x00},
    {0x3802, 0x00},
    {0x3803, 0x00},
    {0x3804, 0x0a},
    {0x3805, 0x3f},
    {0x3806, 0x07},
    {0x3807, 0xb7},
    {0x3808, 0x05},
    {0x3809, 0x10},
    {0x380a, 0x03},
    {0x380b, 0xcc},
    {0x380c, 0x02},
    {0x380d, 0xee},
    {0x380e, 0x07},
    {0x380f, 0xd0},
    {0x3811, 0x08},
    {0x3813, 0x0d},
    {0x3814, 0x03},
    {0x3815, 0x01},
    {0x3816, 0x03},
    {0x3817, 0x01},
    {0x381e, 0x02},
    {0x3820, 0x8b},
    {0x3821, 0x01},
    {0x3832, 0x04},
    {0x3c80, 0x01},
    {0x3c82, 0x00},
    {0x3c83, 0xc8},
    {0x3c8c, 0x0f},
    {0x3c8d, 0xa0},
    {0x3c90, 0x07},
    {0x3c91, 0x00},
    {0x3c92, 0x00},
    {0x3c93, 0x00},
    {0x3c94, 0xd0},
    {0x3c95, 0x50},
    {0x3c96, 0x35},
    {0x3c97, 0x00},
    {0x4001, 0xe0},
    {0x4008, 0x00},
    {0x4009, 0x07},
    {0x400f, 0x80},
    {0x4013, 0x02},
    {0x4040, 0x00},
    {0x4041, 0x03},
    {0x404c, 0x50},
    {0x404e, 0x20},
    {0x4500, 0x06},
    {0x4503, 0x00},
    {0x450a, 0x04},
    {0x4809, 0x04},
    {0x480c, 0x12},
    {0x4819, 0x70},
    {0x4825, 0x32},
    {0x4826, 0x32},
    {0x482a, 0x06},
    {0x4833, 0x08},
    {0x4837, 0x0d},
    {0x5000, 0x77},
    {0x5b00, 0x01},
    {0x5b01, 0x10},
    {0x5b02, 0x01},
    {0x5b03, 0xdb},
    {0x5b05, 0x6c},
    {0x5e10, 0xfc},
    {0x3500, 0x00},
    {0x3501, 0x1F},
    {0x3502, 0x20},
    {0x3503, 0x08},
    {0x3508, 0x04},
    {0x3509, 0x00},
    {0x3832, 0x48},
    {0x5780, 0x3e},
    {0x5781, 0x0f},
    {0x5782, 0x44},
    {0x5783, 0x02},
    {0x5784, 0x01},
    {0x5785, 0x01},
    {0x5786, 0x00},
    {0x5787, 0x04},
    {0x5788, 0x02},
    {0x5789, 0x0f},
    {0x578a, 0xfd},
    {0x578b, 0xf5},
    {0x578c, 0xf5},
    {0x578d, 0x03},
    {0x578e, 0x08},
    {0x578f, 0x0c},
    {0x5790, 0x08},
    {0x5791, 0x06},
    {0x5792, 0x00},
    {0x5793, 0x52},
    {0x5794, 0xa3},
    {0x4003, 0x40},
    {0x3107, 0x01},
    {0x3c80, 0x08},
    {0x3c83, 0xb1},
    {0x3c8c, 0x10},
    {0x3c8d, 0x00},
    {0x3c90, 0x00},
    {0x3c94, 0x00},
    {0x3c95, 0x00},
    {0x3c96, 0x00},
    {0x37cb, 0x09},
    {0x37cc, 0x15},
    {0x37cd, 0x1f},
    {0x37ce, 0x1f},
    };
    static const char * const ov5675_test_pattern_menu[] = {
    "Disabled",
    "Standard Color Bar",
    "Top-Bottom Darker Color Bar",
    "Right-Left Darker Color Bar",
    "Bottom-Top Darker Color Bar"
    };
    static const s64 link_freq_menu_items[] = {
    OV5675_LINK_FREQ_450MHZ,
    };
    static const struct ov5675_link_freq_config link_freq_configs[] = {
    [OV5675_LINK_FREQ_900MBPS] = {
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(mipi_data_rate_900mbps),
    .regs = mipi_data_rate_900mbps,
    }
    }
    };
    static const struct ov5675_mode supported_modes[] = {
    {
    .width = 2592,
    .height = 1944,
    .hts = 1500,
    .vts_def = OV5675_VTS_30FPS,
    .vts_min = OV5675_VTS_30FPS_MIN,
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(mode_2592x1944_regs),
    .regs = mode_2592x1944_regs,
    },
    .link_freq_index = OV5675_LINK_FREQ_900MBPS,
    },
    {
    .width = 1296,
    .height = 972,
    .hts = 1500,
    .vts_def = OV5675_VTS_30FPS,
    .vts_min = OV5675_VTS_30FPS_MIN,
    .reg_list = {
    .num_of_regs = ARRAY_SIZE(mode_1296x972_regs),
    .regs = mode_1296x972_regs,
    },
    .link_freq_index = OV5675_LINK_FREQ_900MBPS,
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov5675 {
    pub dev: *mut device,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub xvclk: *mut clk,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; OV5675_NUM_SUPPLIES],
// V4L2 Controls
    pub link_freq: *mut v4l2_ctrl,
    pub pixel_rate: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
// Current mode
    pub cur_mode: *const ov5675_mode,
// To serialize asynchronous callbacks
    pub mutex: mutex,
// True if the device has been identified
    pub identified: bool,
}

#[no_mangle]
unsafe extern "C" fn to_pixel_rate(f_index: u32) -> u64 {
    static u64 to_pixel_rate(u32 f_index)
    {
    let mut pixel_rate: u64 = link_freq_menu_items[f_index] * 2 * OV5675_DATA_LANES;
    do_div(pixel_rate, OV5675_RGB_DEPTH);
    return pixel_rate;
    }
#[no_mangle]
unsafe extern "C" fn to_pixels_per_line(hts: u32, f_index: u32) -> u64 {
    static u64 to_pixels_per_line(u32 hts, u32 f_index)
    {
    let mut ppl: u64 = hts * to_pixel_rate(f_index);
    do_div(ppl, OV5675_SCLK);
    return ppl;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_read_reg(ov5675: *mut ov5675, reg: u16, len: u16, val: *mut u32) -> c_int {
    static int ov5675_read_reg(struct ov5675 *ov5675, u16 reg, u16 len, u32 *val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&ov5675.sd);
    struct i2c_msg msgs[2];
    u8 addr_buf[2];
    u8 data_buf[4] = {0};
    int ret;
    if (len > 4)
    return -EINVAL;
    put_unaligned_be16(reg, addr_buf);
    msgs[0].addr = client.addr;
    msgs[0].flags = 0;
    msgs[0].len = sizeof(addr_buf);
    msgs[0].buf = addr_buf;
    msgs[1].addr = client.addr;
    msgs[1].flags = I2C_M_RD;
    msgs[1].len = len;
    msgs[1].buf = &data_buf[4 - len];
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs))
    return -EIO;
// val = get_unaligned_be32(data_buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_write_reg(ov5675: *mut ov5675, reg: u16, len: u16, val: u32) -> c_int {
    static int ov5675_write_reg(struct ov5675 *ov5675, u16 reg, u16 len, u32 val)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&ov5675.sd);
    u8 buf[6];
    if (len > 4)
    return -EINVAL;
    put_unaligned_be16(reg, buf);
    put_unaligned_be32(val << 8 * (4 - len), buf + 2);
    if (i2c_master_send(client, buf, len + 2) != len + 2)
    return -EIO;
    return 0;
    }
    static int ov5675_write_reg_list(struct ov5675 *ov5675,
    const struct ov5675_reg_list *r_list)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < r_list.num_of_regs; i++) {
    ret = ov5675_write_reg(ov5675, r_list.regs[i].address, 1,
    r_list.regs[i].val);
    if (ret) {
    dev_err_ratelimited(ov5675.dev,
    "failed to write reg 0x%4.4x. error = %d",
    r_list.regs[i].address, ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_update_digital_gain(ov5675: *mut ov5675, d_gain: u32) -> c_int {
    static int ov5675_update_digital_gain(struct ov5675 *ov5675, u32 d_gain)
    {
    int ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_GROUP_ACCESS,
    OV5675_REG_VALUE_08BIT,
    OV5675_GROUP_HOLD_START);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_MWB_R_GAIN,
    OV5675_REG_VALUE_16BIT, d_gain);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_MWB_G_GAIN,
    OV5675_REG_VALUE_16BIT, d_gain);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_MWB_B_GAIN,
    OV5675_REG_VALUE_16BIT, d_gain);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_GROUP_ACCESS,
    OV5675_REG_VALUE_08BIT,
    OV5675_GROUP_HOLD_END);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_GROUP_ACCESS,
    OV5675_REG_VALUE_08BIT,
    OV5675_GROUP_HOLD_LAUNCH);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_test_pattern(ov5675: *mut ov5675, pattern: u32) -> c_int {
    static int ov5675_test_pattern(struct ov5675 *ov5675, u32 pattern)
    {
    if (pattern)
    pattern = (pattern - 1) << OV5675_TEST_PATTERN_BAR_SHIFT |
    OV5675_TEST_PATTERN_ENABLE;
    return ov5675_write_reg(ov5675, OV5675_REG_TEST_PATTERN,
    OV5675_REG_VALUE_08BIT, pattern);
    }
//
// OV5675 supports keeping the pixel order by mirror and flip function
// The Bayer order isn't affected by the flip controls
//
#[no_mangle]
unsafe extern "C" fn ov5675_set_ctrl_hflip(ov5675: *mut ov5675, ctrl_val: u32) -> c_int {
    static int ov5675_set_ctrl_hflip(struct ov5675 *ov5675, u32 ctrl_val)
    {
    int ret;
    u32 val;
    ret = ov5675_read_reg(ov5675, OV5675_REG_FORMAT1,
    OV5675_REG_VALUE_08BIT, &val);
    if (ret)
    return ret;
    return ov5675_write_reg(ov5675, OV5675_REG_FORMAT1,
    OV5675_REG_VALUE_08BIT,
    ctrl_val ? val & ~BIT(3) : val | BIT(3));
    }
#[no_mangle]
unsafe extern "C" fn ov5675_set_ctrl_vflip(ov5675: *mut ov5675, ctrl_val: u8) -> c_int {
    static int ov5675_set_ctrl_vflip(struct ov5675 *ov5675, u8 ctrl_val)
    {
    int ret;
    u32 val;
    ret = ov5675_read_reg(ov5675, OV5675_REG_FORMAT1,
    OV5675_REG_VALUE_08BIT, &val);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_FORMAT1,
    OV5675_REG_VALUE_08BIT,
    ctrl_val ? val | BIT(4) | BIT(5)  : val & ~BIT(4) & ~BIT(5));
    if (ret)
    return ret;
    ret = ov5675_read_reg(ov5675, OV5675_REG_FORMAT2,
    OV5675_REG_VALUE_08BIT, &val);
    if (ret)
    return ret;
    return ov5675_write_reg(ov5675, OV5675_REG_FORMAT2,
    OV5675_REG_VALUE_08BIT,
    ctrl_val ? val | BIT(1) : val & ~BIT(1));
    }
#[no_mangle]
unsafe extern "C" fn ov5675_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov5675_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov5675 *ov5675 = container_of(ctrl.handler,
    struct ov5675, ctrl_handler);
    s64 exposure_max;
    let mut ret: c_int = 0;
// Propagate change of current control to all related controls
    if (ctrl.id == V4L2_CID_VBLANK) {
// Update max exposure while meeting expected vblanking
    exposure_max = ov5675.cur_mode.height + ctrl.val -
    OV5675_EXPOSURE_MAX_MARGIN;
    __v4l2_ctrl_modify_range(ov5675.exposure,
    ov5675.exposure.minimum,
    exposure_max, ov5675.exposure.step,
    exposure_max);
    }
// V4L2 controls values will be applied only when power is already up
    if (!pm_runtime_get_if_in_use(ov5675.dev))
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_ANALOGUE_GAIN:
    ret = ov5675_write_reg(ov5675, OV5675_REG_ANALOG_GAIN,
    OV5675_REG_VALUE_16BIT, ctrl.val);
    break;
    case V4L2_CID_DIGITAL_GAIN:
    ret = ov5675_update_digital_gain(ov5675, ctrl.val);
    break;
    case V4L2_CID_EXPOSURE:
// 4 least significant bits of expsoure are fractional part
// val = val << 4
// for ov5675, the unit of exposure is different from other
// OmniVision sensors, its exposure value is twice of the
// register value, the exposure should be divided by 2 before
// set register, e.g. val << 3.
//
    ret = ov5675_write_reg(ov5675, OV5675_REG_EXPOSURE,
    OV5675_REG_VALUE_24BIT, ctrl.val << 3);
    break;
    case V4L2_CID_VBLANK:
    ret = ov5675_write_reg(ov5675, OV5675_REG_VTS,
    OV5675_REG_VALUE_16BIT,
    ov5675.cur_mode.height + ctrl.val +
    10);
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = ov5675_test_pattern(ov5675, ctrl.val);
    break;
    case V4L2_CID_HFLIP:
    ov5675_set_ctrl_hflip(ov5675, ctrl.val);
    break;
    case V4L2_CID_VFLIP:
    ov5675_set_ctrl_vflip(ov5675, ctrl.val);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(ov5675.dev);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov5675_ctrl_ops = {
    .s_ctrl = ov5675_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn ov5675_init_controls(ov5675: *mut ov5675) -> c_int {
    static int ov5675_init_controls(struct ov5675 *ov5675)
    {
    struct v4l2_fwnode_device_properties props;
    struct v4l2_ctrl_handler *ctrl_hdlr;
    s64 exposure_max, h_blank;
    int ret;
    ctrl_hdlr = &ov5675.ctrl_handler;
    ret = v4l2_ctrl_handler_init(ctrl_hdlr, 10);
    if (ret)
    return ret;
    ctrl_hdlr.lock = &ov5675.mutex;
    ov5675.link_freq = v4l2_ctrl_new_int_menu(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(link_freq_menu_items) - 1,
    0, link_freq_menu_items);
    if (ov5675.link_freq)
    ov5675.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    ov5675.pixel_rate = v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_PIXEL_RATE, 0,
    to_pixel_rate(OV5675_LINK_FREQ_900MBPS),
    1,
    to_pixel_rate(OV5675_LINK_FREQ_900MBPS));
    ov5675.vblank = v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_VBLANK,
    ov5675.cur_mode.vts_min - ov5675.cur_mode.height,
    OV5675_VTS_MAX - ov5675.cur_mode.height, 1,
    ov5675.cur_mode.vts_def - ov5675.cur_mode.height);
    h_blank = to_pixels_per_line(ov5675.cur_mode.hts,
    ov5675.cur_mode.link_freq_index) - ov5675.cur_mode.width;
    ov5675.hblank = v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_HBLANK, h_blank, h_blank, 1,
    h_blank);
    if (ov5675.hblank)
    ov5675.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops, V4L2_CID_ANALOGUE_GAIN,
    OV5675_ANAL_GAIN_MIN, OV5675_ANAL_GAIN_MAX,
    OV5675_ANAL_GAIN_STEP, OV5675_ANAL_GAIN_MIN);
    v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops, V4L2_CID_DIGITAL_GAIN,
    OV5675_DGTL_GAIN_MIN, OV5675_DGTL_GAIN_MAX,
    OV5675_DGTL_GAIN_STEP, OV5675_DGTL_GAIN_DEFAULT);
    exposure_max = (ov5675.cur_mode.vts_def - OV5675_EXPOSURE_MAX_MARGIN);
    ov5675.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_EXPOSURE,
    OV5675_EXPOSURE_MIN, exposure_max,
    OV5675_EXPOSURE_STEP,
    exposure_max);
    v4l2_ctrl_new_std_menu_items(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov5675_test_pattern_menu) - 1,
    0, 0, ov5675_test_pattern_menu);
    v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    v4l2_ctrl_new_std(ctrl_hdlr, &ov5675_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    if (ctrl_hdlr.error) {
    v4l2_ctrl_handler_free(ctrl_hdlr);
    return ctrl_hdlr.error;
    }
    ret = v4l2_fwnode_device_parse(ov5675.dev, &props);
    if (ret)
    goto error;
    ret = v4l2_ctrl_new_fwnode_properties(ctrl_hdlr, &ov5675_ctrl_ops,
    &props);
    if (ret)
    goto error;
    ov5675.sd.ctrl_handler = ctrl_hdlr;
    return 0;
    error:
    v4l2_ctrl_handler_free(ctrl_hdlr);
    return ret;
    }
    static void ov5675_update_pad_format(const struct ov5675_mode *mode,
    struct v4l2_mbus_framefmt *fmt)
    {
    fmt.width = mode.width;
    fmt.height = mode.height;
    fmt.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    fmt.field = V4L2_FIELD_NONE;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_identify_module(ov5675: *mut ov5675) -> c_int {
    static int ov5675_identify_module(struct ov5675 *ov5675)
    {
    int ret;
    u32 val;
    if (ov5675.identified)
    return 0;
    ret = ov5675_read_reg(ov5675, OV5675_REG_CHIP_ID,
    OV5675_REG_VALUE_24BIT, &val);
    if (ret)
    return ret;
    if (val != OV5675_CHIP_ID) {
    dev_err(ov5675.dev, "chip id mismatch: %x!=%x",
    OV5675_CHIP_ID, val);
    return -ENXIO;
    }
    ov5675.identified = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_start_streaming(ov5675: *mut ov5675) -> c_int {
    static int ov5675_start_streaming(struct ov5675 *ov5675)
    {
    const struct ov5675_reg_list *reg_list;
    int link_freq_index, ret;
    ret = ov5675_identify_module(ov5675);
    if (ret)
    return ret;
    link_freq_index = ov5675.cur_mode.link_freq_index;
    reg_list = &link_freq_configs[link_freq_index].reg_list;
    ret = ov5675_write_reg_list(ov5675, reg_list);
    if (ret) {
    dev_err(ov5675.dev, "failed to set plls");
    return ret;
    }
    reg_list = &ov5675.cur_mode.reg_list;
    ret = ov5675_write_reg_list(ov5675, reg_list);
    if (ret) {
    dev_err(ov5675.dev, "failed to set mode");
    return ret;
    }
    ret = __v4l2_ctrl_handler_setup(ov5675.sd.ctrl_handler);
    if (ret)
    return ret;
    ret = ov5675_write_reg(ov5675, OV5675_REG_MODE_SELECT,
    OV5675_REG_VALUE_08BIT, OV5675_MODE_STREAMING);
    if (ret) {
    dev_err(ov5675.dev, "failed to set stream");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_stop_streaming(ov5675: *mut ov5675) {
    static void ov5675_stop_streaming(struct ov5675 *ov5675)
    {
    if (ov5675_write_reg(ov5675, OV5675_REG_MODE_SELECT,
    OV5675_REG_VALUE_08BIT, OV5675_MODE_STANDBY))
    dev_err(ov5675.dev, "failed to set stream");
    }
#[no_mangle]
unsafe extern "C" fn ov5675_set_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov5675_set_stream(struct v4l2_subdev *sd, int enable)
    {
    struct ov5675 *ov5675 = to_ov5675(sd);
    let mut ret: c_int = 0;
    mutex_lock(&ov5675.mutex);
    if (enable) {
    ret = pm_runtime_resume_and_get(ov5675.dev);
    if (ret < 0) {
    mutex_unlock(&ov5675.mutex);
    return ret;
    }
    ret = ov5675_start_streaming(ov5675);
    if (ret) {
    enable = 0;
    ov5675_stop_streaming(ov5675);
    pm_runtime_put(ov5675.dev);
    }
    } else {
    ov5675_stop_streaming(ov5675);
    pm_runtime_put(ov5675.dev);
    }
    mutex_unlock(&ov5675.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_power_off(dev: *mut device) -> c_int {
    static int ov5675_power_off(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov5675 *ov5675 = to_ov5675(sd);
    usleep_range(90, 100);
    clk_disable_unprepare(ov5675.xvclk);
    gpiod_set_value_cansleep(ov5675.reset_gpio, 1);
    regulator_bulk_disable(OV5675_NUM_SUPPLIES, ov5675.supplies);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_power_on(dev: *mut device) -> c_int {
    static int ov5675_power_on(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct ov5675 *ov5675 = to_ov5675(sd);
    int ret;
    ret = clk_prepare_enable(ov5675.xvclk);
    if (ret < 0) {
    dev_err(dev, "failed to enable xvclk: %d\n", ret);
    return ret;
    }
    gpiod_set_value_cansleep(ov5675.reset_gpio, 1);
    ret = regulator_bulk_enable(OV5675_NUM_SUPPLIES, ov5675.supplies);
    if (ret) {
    clk_disable_unprepare(ov5675.xvclk);
    return ret;
    }
// Reset pulse should be at least 2ms and reset gpio released only once
// regulators are stable.
//
    usleep_range(2000, 2200);
    gpiod_set_value_cansleep(ov5675.reset_gpio, 0);
// Worst case quiesence gap is 1.365 milliseconds @ 6MHz XVCLK
// Add an additional threshold grace period to ensure reset
// completion before initiating our first I2C transaction.
//
    usleep_range(1500, 1600);
    return 0;
    }
    static int ov5675_set_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov5675 *ov5675 = to_ov5675(sd);
    const struct ov5675_mode *mode;
    s32 vblank_def, h_blank;
    mode = v4l2_find_nearest_size(supported_modes,
    ARRAY_SIZE(supported_modes), width,
    height, fmt.format.width,
    fmt.format.height);
    mutex_lock(&ov5675.mutex);
    ov5675_update_pad_format(mode, &fmt.format);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY) {
// v4l2_subdev_state_get_format(sd_state, fmt->pad) = fmt->format;
    } else {
    ov5675.cur_mode = mode;
    __v4l2_ctrl_s_ctrl(ov5675.link_freq, mode.link_freq_index);
    __v4l2_ctrl_s_ctrl_int64(ov5675.pixel_rate,
    to_pixel_rate(mode.link_freq_index));
// Update limits and set FPS to default
    vblank_def = mode.vts_def - mode.height;
    __v4l2_ctrl_modify_range(ov5675.vblank,
    mode.vts_min - mode.height,
    OV5675_VTS_MAX - mode.height, 1,
    vblank_def);
    __v4l2_ctrl_s_ctrl(ov5675.vblank, vblank_def);
    h_blank = to_pixels_per_line(mode.hts, mode.link_freq_index) -
    mode.width;
    __v4l2_ctrl_modify_range(ov5675.hblank, h_blank, h_blank, 1,
    h_blank);
    }
    mutex_unlock(&ov5675.mutex);
    return 0;
    }
    static int ov5675_get_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov5675 *ov5675 = to_ov5675(sd);
    mutex_lock(&ov5675.mutex);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY)
    fmt.format = *v4l2_subdev_state_get_format(sd_state,
    fmt.pad);
    else
    ov5675_update_pad_format(ov5675.cur_mode, &fmt.format);
    mutex_unlock(&ov5675.mutex);
    return 0;
    }
    static int ov5675_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_selection *sel)
    {
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r.top = 0;
    sel.r.left = 0;
    sel.r.width = 2624;
    sel.r.height = 2000;
    return 0;
    case V4L2_SEL_TGT_CROP:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    sel.r.top = 16;
    sel.r.left = 16;
    sel.r.width = 2592;
    sel.r.height = 1944;
    return 0;
    }
    return -EINVAL;
    }
    static int ov5675_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index > 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_SGRBG10_1X10;
    return 0;
    }
    static int ov5675_enum_frame_size(struct v4l2_subdev *sd,
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
#[no_mangle]
unsafe extern "C" fn ov5675_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ov5675_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    struct ov5675 *ov5675 = to_ov5675(sd);
    mutex_lock(&ov5675.mutex);
    ov5675_update_pad_format(&supported_modes[0],
    v4l2_subdev_state_get_format(fh.state, 0));
    mutex_unlock(&ov5675.mutex);
    return 0;
    }
    static const struct v4l2_subdev_video_ops ov5675_video_ops = {
    .s_stream = ov5675_set_stream,
    };
    static const struct v4l2_subdev_pad_ops ov5675_pad_ops = {
    .set_fmt = ov5675_set_format,
    .get_fmt = ov5675_get_format,
    .get_selection = ov5675_get_selection,
    .enum_mbus_code = ov5675_enum_mbus_code,
    .enum_frame_size = ov5675_enum_frame_size,
    };
    static const struct v4l2_subdev_ops ov5675_subdev_ops = {
    .video = &ov5675_video_ops,
    .pad = &ov5675_pad_ops,
    };
    static const struct media_entity_operations ov5675_subdev_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
    static const struct v4l2_subdev_internal_ops ov5675_internal_ops = {
    .open = ov5675_open,
    };
#[no_mangle]
unsafe extern "C" fn ov5675_get_hwcfg(ov5675: *mut ov5675) -> c_int {
    static int ov5675_get_hwcfg(struct ov5675 *ov5675)
    {
    struct device *dev = ov5675.dev;
    struct fwnode_handle *ep;
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY
    };
    u32 xvclk_rate;
    int ret;
    unsigned int i, j;
    if (!fwnode)
    return -ENXIO;
    ep = fwnode_graph_get_endpoint_by_id(fwnode, 0, 0,
    FWNODE_GRAPH_ENDPOINT_NEXT);
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    fwnode_handle_put(ep);
    if (ret)
    return ret;
    ov5675.xvclk = devm_v4l2_sensor_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ov5675.xvclk)) {
    ret = dev_err_probe(dev, PTR_ERR(ov5675.xvclk),
    "failed to get xvclk\n");
    goto check_hwcfg_error;
    }
    xvclk_rate = clk_get_rate(ov5675.xvclk);
    if (xvclk_rate != OV5675_XVCLK_19_2) {
    dev_err(dev, "external clock rate %u is unsupported",
    xvclk_rate);
    ret = -EINVAL;
    goto check_hwcfg_error;
    }
    ov5675.reset_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov5675.reset_gpio)) {
    ret = PTR_ERR(ov5675.reset_gpio);
    dev_err(dev, "failed to get reset-gpios: %d\n", ret);
    goto check_hwcfg_error;
    }
    for (i = 0; i < OV5675_NUM_SUPPLIES; i++)
    ov5675.supplies[i].supply = ov5675_supply_names[i];
    ret = devm_regulator_bulk_get(dev, OV5675_NUM_SUPPLIES,
    ov5675.supplies);
    if (ret)
    goto check_hwcfg_error;
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != OV5675_DATA_LANES) {
    dev_err(dev, "number of CSI2 data lanes %d is not supported",
    bus_cfg.bus.mipi_csi2.num_data_lanes);
    ret = -EINVAL;
    goto check_hwcfg_error;
    }
    if (!bus_cfg.nr_of_link_frequencies) {
    dev_err(dev, "no link frequencies defined");
    ret = -EINVAL;
    goto check_hwcfg_error;
    }
    for (i = 0; i < ARRAY_SIZE(link_freq_menu_items); i++) {
    for (j = 0; j < bus_cfg.nr_of_link_frequencies; j++) {
    if (link_freq_menu_items[i] ==
    bus_cfg.link_frequencies[j])
    break;
    }
    if (j == bus_cfg.nr_of_link_frequencies) {
    dev_err(dev, "no link frequency %lld supported",
    link_freq_menu_items[i]);
    ret = -EINVAL;
    goto check_hwcfg_error;
    }
    }
    check_hwcfg_error:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov5675_remove(client: *mut i2c_client) {
    static void ov5675_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov5675 *ov5675 = to_ov5675(sd);
    v4l2_async_unregister_subdev(sd);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(sd.ctrl_handler);
    pm_runtime_disable(ov5675.dev);
    mutex_destroy(&ov5675.mutex);
    if (!pm_runtime_status_suspended(ov5675.dev))
    ov5675_power_off(ov5675.dev);
    pm_runtime_set_suspended(ov5675.dev);
    }
#[no_mangle]
unsafe extern "C" fn ov5675_probe(client: *mut i2c_client) -> c_int {
    static int ov5675_probe(struct i2c_client *client)
    {
    struct ov5675 *ov5675;
    bool full_power;
    int ret;
    ov5675 = devm_kzalloc(&client.dev, sizeof(*ov5675), GFP_KERNEL);
    if (!ov5675)
    return -ENOMEM;
    ov5675.dev = &client.dev;
    ret = ov5675_get_hwcfg(ov5675);
    if (ret)
    return ret;
    v4l2_i2c_subdev_init(&ov5675.sd, client, &ov5675_subdev_ops);
    ret = ov5675_power_on(ov5675.dev);
    if (ret) {
    dev_err(ov5675.dev, "failed to power on: %d\n", ret);
    return ret;
    }
    full_power = acpi_dev_state_d0(ov5675.dev);
    if (full_power) {
    ret = ov5675_identify_module(ov5675);
    if (ret) {
    dev_err(ov5675.dev, "failed to find sensor: %d", ret);
    goto probe_power_off;
    }
    }
    mutex_init(&ov5675.mutex);
    ov5675.cur_mode = &supported_modes[0];
    ret = ov5675_init_controls(ov5675);
    if (ret) {
    dev_err(ov5675.dev, "failed to init controls: %d", ret);
    goto probe_error_v4l2_ctrl_handler_free;
    }
    ov5675.sd.internal_ops = &ov5675_internal_ops;
    ov5675.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    ov5675.sd.entity.ops = &ov5675_subdev_entity_ops;
    ov5675.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ov5675.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&ov5675.sd.entity, 1, &ov5675.pad);
    if (ret) {
    dev_err(ov5675.dev, "failed to init entity pads: %d", ret);
    goto probe_error_v4l2_ctrl_handler_free;
    }
    ret = v4l2_async_register_subdev_sensor(&ov5675.sd);
    if (ret < 0) {
    dev_err(ov5675.dev, "failed to register V4L2 subdev: %d",
    ret);
    goto probe_error_media_entity_cleanup;
    }
// Set the device's state to active if it's in D0 state.
    if (full_power)
    pm_runtime_set_active(ov5675.dev);
    pm_runtime_enable(ov5675.dev);
    pm_runtime_idle(ov5675.dev);
    return 0;
    probe_error_media_entity_cleanup:
    media_entity_cleanup(&ov5675.sd.entity);
    probe_error_v4l2_ctrl_handler_free:
    v4l2_ctrl_handler_free(ov5675.sd.ctrl_handler);
    mutex_destroy(&ov5675.mutex);
    probe_power_off:
    ov5675_power_off(ov5675.dev);
    return ret;
    }
    static const struct dev_pm_ops ov5675_pm_ops = {
    SET_RUNTIME_PM_OPS(ov5675_power_off, ov5675_power_on, core::ptr::null_mut())
    };

    static const struct acpi_device_id ov5675_acpi_ids[] = {
    {"OVTI5675"},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, ov5675_acpi_ids);

    static const struct of_device_id ov5675_of_match[] = {
    { .compatible = "ovti,ov5675", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ov5675_of_match);
    static struct i2c_driver ov5675_i2c_driver = {
    .driver = {
    .name = "ov5675",
    .pm = &ov5675_pm_ops,
    .acpi_match_table = ACPI_PTR(ov5675_acpi_ids),
    .of_match_table = ov5675_of_match,
    },
    .probe = ov5675_probe,
    .remove = ov5675_remove,
    .flags = I2C_DRV_ACPI_WAIVE_D0_PROBE,
    };
    module_i2c_driver(ov5675_i2c_driver);
    MODULE_AUTHOR("Shawn Tu");
    MODULE_DESCRIPTION("OmniVision OV5675 sensor driver");
    MODULE_LICENSE("GPL v2");
