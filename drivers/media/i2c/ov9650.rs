//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov9650.c
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
// Omnivision OV9650/OV9652 CMOS Image Sensor driver
//
// Copyright (C) 2013, Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
//
// Register definitions and initial settings based on a driver written
// by Vladimir Fonov.
// Copyright (c) 2010, Vladimir Fonov
//

    static int debug;
    module_param(debug, int, 0644);
    MODULE_PARM_DESC(debug, "Debug level (0-2)");

//
// OV9650/OV9652 register definitions
//
pub const REG_GAIN: c_uint = 0x00	/* Gain control, AGC[7:0] */;
pub const REG_BLUE: c_uint = 0x01	/* AWB - Blue channel gain */;
pub const REG_RED: c_uint = 0x02	/* AWB - Red channel gain */;
pub const REG_VREF: c_uint = 0x03	/* [7:6] - AGC[9:8], [5:3]/[2:0] */;
pub const VREF_GAIN_MASK: c_uint = 0xc0	/* - VREF end/start low 3 bits */;
pub const REG_COM1: c_uint = 0x04;
pub const COM1_CCIR656: c_uint = 0x40;
pub const REG_B_AVE: c_uint = 0x05;
pub const REG_GB_AVE: c_uint = 0x06;
pub const REG_GR_AVE: c_uint = 0x07;
pub const REG_R_AVE: c_uint = 0x08;
pub const REG_COM2: c_uint = 0x09;
pub const REG_PID: c_uint = 0x0a	/* Product ID MSB */;
pub const REG_VER: c_uint = 0x0b	/* Product ID LSB */;
pub const REG_COM3: c_uint = 0x0c;
pub const COM3_SWAP: c_uint = 0x40;
pub const COM3_VARIOPIXEL1: c_uint = 0x04;
pub const REG_COM4: c_uint = 0x0d	/* Vario Pixels  */;
pub const COM4_VARIOPIXEL2: c_uint = 0x80;
pub const REG_COM5: c_uint = 0x0e	/* System clock options */;
pub const COM5_SLAVE_MODE: c_uint = 0x10;
pub const COM5_SYSTEMCLOCK48MHZ: c_uint = 0x80;
pub const REG_COM6: c_uint = 0x0f	/* HREF & ADBLC options */;
pub const REG_AECH: c_uint = 0x10	/* Exposure value, AEC[9:2] */;
pub const REG_CLKRC: c_uint = 0x11	/* Clock control */;
pub const CLK_EXT: c_uint = 0x40	/* Use external clock directly */;
pub const CLK_SCALE: c_uint = 0x3f	/* Mask for internal clock scale */;
pub const REG_COM7: c_uint = 0x12	/* SCCB reset, output format */;
pub const COM7_RESET: c_uint = 0x80;
pub const COM7_FMT_MASK: c_uint = 0x38;
pub const COM7_FMT_VGA: c_uint = 0x40;
pub const COM7_FMT_CIF: c_uint = 0x20;
pub const COM7_FMT_QVGA: c_uint = 0x10;
pub const COM7_FMT_QCIF: c_uint = 0x08;
pub const COM7_RGB: c_uint = 0x04;
pub const COM7_YUV: c_uint = 0x00;
pub const COM7_BAYER: c_uint = 0x01;
pub const COM7_PBAYER: c_uint = 0x05;
pub const REG_COM8: c_uint = 0x13	/* AGC/AEC options */;
pub const COM8_FASTAEC: c_uint = 0x80	/* Enable fast AGC/AEC */;
pub const COM8_AECSTEP: c_uint = 0x40	/* Unlimited AEC step size */;
pub const COM8_BFILT: c_uint = 0x20	/* Band filter enable */;
pub const COM8_AGC: c_uint = 0x04	/* Auto gain enable */;
pub const COM8_AWB: c_uint = 0x02	/* White balance enable */;
pub const COM8_AEC: c_uint = 0x01	/* Auto exposure enable */;
pub const REG_COM9: c_uint = 0x14	/* Gain ceiling */;
pub const COM9_GAIN_CEIL_MASK: c_uint = 0x70	/* */;
pub const REG_COM10: c_uint = 0x15	/* PCLK, HREF, HSYNC signals polarity */;
pub const COM10_HSYNC: c_uint = 0x40	/* HSYNC instead of HREF */;
pub const COM10_PCLK_HB: c_uint = 0x20	/* Suppress PCLK on horiz blank */;
pub const COM10_HREF_REV: c_uint = 0x08	/* Reverse HREF */;
pub const COM10_VS_LEAD: c_uint = 0x04	/* VSYNC on clock leading edge */;
pub const COM10_VS_NEG: c_uint = 0x02	/* VSYNC negative */;
pub const COM10_HS_NEG: c_uint = 0x01	/* HSYNC negative */;
pub const REG_HSTART: c_uint = 0x17	/* Horiz start high bits */;
pub const REG_HSTOP: c_uint = 0x18	/* Horiz stop high bits */;
pub const REG_VSTART: c_uint = 0x19	/* Vert start high bits */;
pub const REG_VSTOP: c_uint = 0x1a	/* Vert stop high bits */;
pub const REG_PSHFT: c_uint = 0x1b	/* Pixel delay after HREF */;
pub const REG_MIDH: c_uint = 0x1c	/* Manufacturer ID MSB */;
pub const REG_MIDL: c_uint = 0x1d	/* Manufufacturer ID LSB */;
pub const REG_MVFP: c_uint = 0x1e	/* Image mirror/flip */;
pub const MVFP_MIRROR: c_uint = 0x20	/* Mirror image */;
pub const MVFP_FLIP: c_uint = 0x10	/* Vertical flip */;
pub const REG_BOS: c_uint = 0x20	/* B channel Offset */;
pub const REG_GBOS: c_uint = 0x21	/* Gb channel Offset */;
pub const REG_GROS: c_uint = 0x22	/* Gr channel Offset */;
pub const REG_ROS: c_uint = 0x23	/* R channel Offset */;
pub const REG_AEW: c_uint = 0x24	/* AGC upper limit */;
pub const REG_AEB: c_uint = 0x25	/* AGC lower limit */;
pub const REG_VPT: c_uint = 0x26	/* AGC/AEC fast mode op region */;
pub const REG_BBIAS: c_uint = 0x27	/* B channel output bias */;
pub const REG_GBBIAS: c_uint = 0x28	/* Gb channel output bias */;
pub const REG_GRCOM: c_uint = 0x29	/* Analog BLC & regulator */;
pub const REG_EXHCH: c_uint = 0x2a	/* Dummy pixel insert MSB */;
pub const REG_EXHCL: c_uint = 0x2b	/* Dummy pixel insert LSB */;
pub const REG_RBIAS: c_uint = 0x2c	/* R channel output bias */;
pub const REG_ADVFL: c_uint = 0x2d	/* LSB of dummy line insert */;
pub const REG_ADVFH: c_uint = 0x2e	/* MSB of dummy line insert */;
pub const REG_YAVE: c_uint = 0x2f	/* Y/G channel average value */;
pub const REG_HSYST: c_uint = 0x30	/* HSYNC rising edge delay LSB*/;
pub const REG_HSYEN: c_uint = 0x31	/* HSYNC falling edge delay LSB*/;
pub const REG_HREF: c_uint = 0x32	/* HREF pieces */;
pub const REG_CHLF: c_uint = 0x33	/* reserved */;
pub const REG_ADC: c_uint = 0x37	/* reserved */;
pub const REG_ACOM: c_uint = 0x38	/* reserved */;
pub const REG_OFON: c_uint = 0x39	/* Power down register */;
pub const OFON_PWRDN: c_uint = 0x08	/* Power down bit */;
pub const REG_TSLB: c_uint = 0x3a	/* YUVU format */;
pub const TSLB_YUYV_MASK: c_uint = 0x0c	/* UYVY or VYUY - see com13 */;
pub const REG_COM11: c_uint = 0x3b	/* Night mode, banding filter enable */;
pub const COM11_NIGHT: c_uint = 0x80	/* Night mode enable */;
pub const COM11_NMFR: c_uint = 0x60	/* Two bit NM frame rate */;
pub const COM11_BANDING: c_uint = 0x01	/* Banding filter */;
pub const COM11_AEC_REF_MASK: c_uint = 0x18	/* AEC reference area selection */;
pub const REG_COM12: c_uint = 0x3c	/* HREF option, UV average */;
pub const COM12_HREF: c_uint = 0x80	/* HREF always */;
pub const REG_COM13: c_uint = 0x3d	/* Gamma selection, Color matrix en. */;
pub const COM13_GAMMA: c_uint = 0x80	/* Gamma enable */;
pub const COM13_UVSAT: c_uint = 0x40	/* UV saturation auto adjustment */;
pub const COM13_UVSWAP: c_uint = 0x01	/* V before U - w/TSLB */;
pub const REG_COM14: c_uint = 0x3e	/* Edge enhancement options */;
pub const COM14_EDGE_EN: c_uint = 0x02;
pub const COM14_EEF_X2: c_uint = 0x01;
pub const REG_EDGE: c_uint = 0x3f	/* Edge enhancement factor */;
pub const EDGE_FACTOR_MASK: c_uint = 0x0f;
pub const REG_COM15: c_uint = 0x40	/* Output range, RGB 555/565 */;
pub const COM15_R10F0: c_uint = 0x00	/* Data range 10 to F0 */;
pub const COM15_R01FE: c_uint = 0x80	/* 01 to FE */;
pub const COM15_R00FF: c_uint = 0xc0	/* 00 to FF */;
pub const COM15_RGB565: c_uint = 0x10	/* RGB565 output */;
pub const COM15_RGB555: c_uint = 0x30	/* RGB555 output */;
pub const COM15_SWAPRB: c_uint = 0x04	/* Swap R&B */;
pub const REG_COM16: c_uint = 0x41	/* Color matrix coeff options */;
pub const REG_COM17: c_uint = 0x42	/* Single frame out, banding filter */;
// n = 1...9, 0x4f..0x57

pub const REG_MTXS: c_uint = 0x58;
// Lens Correction Option 1...5, __n = 0...5

pub const LCC5_LCC_ENABLE: c_uint = 0x01	/* LCC5, enable lens correction */;
pub const LCC5_LCC_COLOR: c_uint = 0x04;
pub const REG_MANU: c_uint = 0x67	/* Manual U value */;
pub const REG_MANV: c_uint = 0x68	/* Manual V value */;
pub const REG_HV: c_uint = 0x69	/* Manual banding filter MSB */;
pub const REG_MBD: c_uint = 0x6a	/* Manual banding filter value */;
pub const REG_DBLV: c_uint = 0x6b	/* reserved */;
pub const REG_GSP: c_uint = 0x6c	/* Gamma curve */;
pub const GSP_LEN: c_int = 15;
pub const REG_GST: c_uint = 0x7c	/* Gamma curve */;
pub const GST_LEN: c_int = 15;
pub const REG_COM21: c_uint = 0x8b;
pub const REG_COM22: c_uint = 0x8c	/* Edge enhancement, denoising */;
pub const COM22_WHTPCOR: c_uint = 0x02	/* White pixel correction enable */;
pub const COM22_WHTPCOROPT: c_uint = 0x01	/* White pixel correction option */;
pub const COM22_DENOISE: c_uint = 0x10	/* White pixel correction option */;
pub const REG_COM23: c_uint = 0x8d	/* Color bar test, color gain */;
pub const COM23_TEST_MODE: c_uint = 0x10;
pub const REG_DBLC1: c_uint = 0x8f	/* Digital BLC */;
pub const REG_DBLC_B: c_uint = 0x90	/* Digital BLC B channel offset */;
pub const REG_DBLC_R: c_uint = 0x91	/* Digital BLC R channel offset */;
pub const REG_DM_LNL: c_uint = 0x92	/* Dummy line low 8 bits */;
pub const REG_DM_LNH: c_uint = 0x93	/* Dummy line high 8 bits */;
pub const REG_LCCFB: c_uint = 0x9d	/* Lens Correction B channel */;
pub const REG_LCCFR: c_uint = 0x9e	/* Lens Correction R channel */;
pub const REG_DBLC_GB: c_uint = 0x9f	/* Digital BLC GB chan offset */;
pub const REG_DBLC_GR: c_uint = 0xa0	/* Digital BLC GR chan offset */;
pub const REG_AECHM: c_uint = 0xa1	/* Exposure value - bits AEC[15:10] */;
pub const REG_BD50ST: c_uint = 0xa2	/* Banding filter value for 50Hz */;
pub const REG_BD60ST: c_uint = 0xa3	/* Banding filter value for 60Hz */;
pub const REG_NULL: c_uint = 0xff	/* Array end token */;
pub const DEF_CLKRC: c_uint = 0x80;

pub const OV9650_ID: c_uint = 0x9650;
pub const OV9652_ID: c_uint = 0x9652;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov965x_ctrls {
    pub handler: v4l2_ctrl_handler,
    struct {
    pub auto_exp: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
}

    struct {
    struct v4l2_ctrl *auto_wb;
    struct v4l2_ctrl *blue_balance;
    struct v4l2_ctrl *red_balance;
    };
    struct {
    struct v4l2_ctrl *hflip;
    struct v4l2_ctrl *vflip;
    };
    struct {
    struct v4l2_ctrl *auto_gain;
    struct v4l2_ctrl *gain;
    };
    struct v4l2_ctrl *brightness;
    struct v4l2_ctrl *saturation;
    struct v4l2_ctrl *sharpness;
    struct v4l2_ctrl *light_freq;
    u8 update;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov965x_framesize {
    pub width: u16,
    pub height: u16,
    pub max_exp_lines: u16,
    pub regs: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov965x_interval {
    pub interval: v4l2_fract,
// Maximum resolution for this interval
    pub size: v4l2_frmsize_discrete,
    pub clkrc_div: u8,
}

    enum gpio_id {
    GPIO_PWDN,
    GPIO_RST,
    NUM_GPIOS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov965x {
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub bus_type: enum v4l2_mbus_type,
    pub gpios: [*mut gpio_desc; NUM_GPIOS],
// External master clock frequency
    pub mclk_frequency: c_ulong,
    pub clk: *mut clk,
// Protects the struct fields below
    pub lock: mutex,
    pub regmap: *mut regmap,
// Exposure row interval in us
    pub exp_row_interval: c_uint,
    pub id: c_ushort,
    pub frame_size: *const ov965x_framesize,
// YUYV sequence (pixel format) control register
    pub tslb_reg: u8,
    pub format: v4l2_mbus_framefmt,
    pub ctrls: ov965x_ctrls,
// Pointer to frame rate control data structure
    pub fiv: *const ov965x_interval,
    pub streaming: c_int,
    pub power: c_int,
    pub apply_frame_fmt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_rv {
    pub addr: u8,
    pub value: u8,
}

    static const struct i2c_rv ov965x_init_regs[] = {
    { REG_COM2, 0x10 },	/* Set soft sleep mode */
    { REG_COM5, 0x00 },	/* System clock options */
    { REG_COM2, 0x01 },	/* Output drive, soft sleep mode */
    { REG_COM10, 0x00 },	/* Slave mode, HREF vs HSYNC, signals negate */
    { REG_EDGE, 0xa6 },	/* Edge enhancement threshold and factor */
    { REG_COM16, 0x02 },	/* Color matrix coeff double option */
    { REG_COM17, 0x08 },	/* Single frame out, banding filter */
    { 0x16, 0x06 },
    { REG_CHLF, 0xc0 },	/* Reserved  */
    { 0x34, 0xbf },
    { 0xa8, 0x80 },
    { 0x96, 0x04 },
    { 0x8e, 0x00 },
    { REG_COM12, 0x77 },	/* HREF option, UV average  */
    { 0x8b, 0x06 },
    { 0x35, 0x91 },
    { 0x94, 0x88 },
    { 0x95, 0x88 },
    { REG_COM15, 0xc1 },	/* Output range, RGB 555/565 */
    { REG_GRCOM, 0x2f },	/* Analog BLC & regulator */
    { REG_COM6, 0x43 },	/* HREF & ADBLC options */
    { REG_COM8, 0xe5 },	/* AGC/AEC options */
    { REG_COM13, 0x90 },	/* Gamma selection, colour matrix, UV delay */
    { REG_HV, 0x80 },	/* Manual banding filter MSB  */
    { 0x5c, 0x96 },		/* Reserved up to 0xa5 */
    { 0x5d, 0x96 },
    { 0x5e, 0x10 },
    { 0x59, 0xeb },
    { 0x5a, 0x9c },
    { 0x5b, 0x55 },
    { 0x43, 0xf0 },
    { 0x44, 0x10 },
    { 0x45, 0x55 },
    { 0x46, 0x86 },
    { 0x47, 0x64 },
    { 0x48, 0x86 },
    { 0x5f, 0xe0 },
    { 0x60, 0x8c },
    { 0x61, 0x20 },
    { 0xa5, 0xd9 },
    { 0xa4, 0x74 },		/* reserved */
    { REG_COM23, 0x02 },	/* Color gain analog/_digital_ */
    { REG_COM8, 0xe7 },	/* Enable AEC, AWB, AEC */
    { REG_COM22, 0x23 },	/* Edge enhancement, denoising */
    { 0xa9, 0xb8 },
    { 0xaa, 0x92 },
    { 0xab, 0x0a },
    { REG_DBLC1, 0xdf },	/* Digital BLC */
    { REG_DBLC_B, 0x00 },	/* Digital BLC B chan offset */
    { REG_DBLC_R, 0x00 },	/* Digital BLC R chan offset */
    { REG_DBLC_GB, 0x00 },	/* Digital BLC GB chan offset */
    { REG_DBLC_GR, 0x00 },
    { REG_COM9, 0x3a },	/* Gain ceiling 16x */
    { REG_NULL, 0 }
    };
pub const NUM_FMT_REGS: c_int = 14;
//
// COM7,  COM3,  COM4, HSTART, HSTOP, HREF, VSTART, VSTOP, VREF,
// EXHCH, EXHCL, ADC,  OCOM,   OFON
//
    static const u8 frame_size_reg_addr[NUM_FMT_REGS] = {
    0x12, 0x0c, 0x0d, 0x17, 0x18, 0x32, 0x19, 0x1a, 0x03,
    0x2a, 0x2b, 0x37, 0x38, 0x39,
    };
    static const u8 ov965x_sxga_regs[NUM_FMT_REGS] = {
    0x00, 0x00, 0x00, 0x1e, 0xbe, 0xbf, 0x01, 0x81, 0x12,
    0x10, 0x34, 0x81, 0x93, 0x51,
    };
    static const u8 ov965x_vga_regs[NUM_FMT_REGS] = {
    0x40, 0x04, 0x80, 0x26, 0xc6, 0xed, 0x01, 0x3d, 0x00,
    0x10, 0x40, 0x91, 0x12, 0x43,
    };
// Determined empirically.
    static const u8 ov965x_qvga_regs[NUM_FMT_REGS] = {
    0x10, 0x04, 0x80, 0x25, 0xc5, 0xbf, 0x00, 0x80, 0x12,
    0x10, 0x40, 0x91, 0x12, 0x43,
    };
    static const struct ov965x_framesize ov965x_framesizes[] = {
    {
    .width		= SXGA_WIDTH,
    .height		= SXGA_HEIGHT,
    .regs		= ov965x_sxga_regs,
    .max_exp_lines	= 1048,
    }, {
    .width		= VGA_WIDTH,
    .height		= VGA_HEIGHT,
    .regs		= ov965x_vga_regs,
    .max_exp_lines	= 498,
    }, {
    .width		= QVGA_WIDTH,
    .height		= QVGA_HEIGHT,
    .regs		= ov965x_qvga_regs,
    .max_exp_lines	= 248,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov965x_pixfmt {
    pub code: u32,
    pub colorspace: u32,
// REG_TSLB value, only bits [3:2] may be set.
    pub tslb_reg: u8,
}

    static const struct ov965x_pixfmt ov965x_formats[] = {
    { MEDIA_BUS_FMT_YUYV8_2X8, V4L2_COLORSPACE_JPEG, 0x00},
    { MEDIA_BUS_FMT_YVYU8_2X8, V4L2_COLORSPACE_JPEG, 0x04},
    { MEDIA_BUS_FMT_UYVY8_2X8, V4L2_COLORSPACE_JPEG, 0x0c},
    { MEDIA_BUS_FMT_VYUY8_2X8, V4L2_COLORSPACE_JPEG, 0x08},
    };
//
// This table specifies possible frame resolution and interval
// combinations. Default CLKRC[5:0] divider values are valid
// only for 24 MHz external clock frequency.
//
    static struct ov965x_interval ov965x_intervals[] = {
    {{ 100, 625 }, { SXGA_WIDTH, SXGA_HEIGHT }, 0 },  /* 6.25 fps */
    {{ 10,  125 }, { VGA_WIDTH, VGA_HEIGHT },   1 },  /* 12.5 fps */
    {{ 10,  125 }, { QVGA_WIDTH, QVGA_HEIGHT }, 3 },  /* 12.5 fps */
    {{ 1,   25  }, { VGA_WIDTH, VGA_HEIGHT },   0 },  /* 25 fps */
    {{ 1,   25  }, { QVGA_WIDTH, QVGA_HEIGHT }, 1 },  /* 25 fps */
    };
    static inline struct v4l2_subdev *ctrl_to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct ov965x, ctrls.handler).sd;
    }
    static inline struct ov965x *to_ov965x(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct ov965x, sd);
    }
#[no_mangle]
unsafe extern "C" fn ov965x_read(ov965x: *mut ov965x, addr: u8, val: *mut u8) -> c_int {
    static int ov965x_read(struct ov965x *ov965x, u8 addr, u8 *val)
    {
    int ret;
    unsigned int buf;
    ret = regmap_read(ov965x.regmap, addr, &buf);
    if (!ret)
// val = buf;
    else
// val = -1;
    v4l2_dbg(2, debug, &ov965x.sd, "%s: 0x%02x @ 0x%02x. (%d)\n",
    __func__, *val, addr, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_write(ov965x: *mut ov965x, addr: u8, val: u8) -> c_int {
    static int ov965x_write(struct ov965x *ov965x, u8 addr, u8 val)
    {
    int ret;
    ret = regmap_write(ov965x.regmap, addr, val);
    v4l2_dbg(2, debug, &ov965x.sd, "%s: 0x%02x @ 0x%02X (%d)\n",
    __func__, val, addr, ret);
    return ret;
    }
    static int ov965x_write_array(struct ov965x *ov965x,
    const struct i2c_rv *regs)
    {
    int i, ret = 0;
    for (i = 0; ret == 0 && regs[i].addr != REG_NULL; i++)
    ret = ov965x_write(ov965x, regs[i].addr, regs[i].value);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_default_gamma_curve(ov965x: *mut ov965x) -> c_int {
    static int ov965x_set_default_gamma_curve(struct ov965x *ov965x)
    {
    static const u8 gamma_curve[] = {
// Values taken from OV application note.
    0x40, 0x30, 0x4b, 0x60, 0x70, 0x70, 0x70, 0x70,
    0x60, 0x60, 0x50, 0x48, 0x3a, 0x2e, 0x28, 0x22,
    0x04, 0x07, 0x10, 0x28,	0x36, 0x44, 0x52, 0x60,
    0x6c, 0x78, 0x8c, 0x9e, 0xbb, 0xd2, 0xe6
    };
    let mut addr: u8 = REG_GSP;
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(gamma_curve); i++) {
    let mut ret: c_int = ov965x_write(ov965x, addr, gamma_curve[i]);
    if (ret < 0)
    return ret;
    addr++;
    }
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn ov965x_set_color_matrix(ov965x: *mut ov965x) -> c_int {
    static int ov965x_set_color_matrix(struct ov965x *ov965x)
    {
    static const u8 mtx[] = {
// MTX1..MTX9, MTXS
    0x3a, 0x3d, 0x03, 0x12, 0x26, 0x38, 0x40, 0x40, 0x40, 0x0d
    };
    let mut addr: u8 = REG_MTX(1);
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(mtx); i++) {
    let mut ret: c_int = ov965x_write(ov965x, addr, mtx[i]);
    if (ret < 0)
    return ret;
    addr++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __ov965x_set_power(ov965x: *mut ov965x, on: c_int) -> c_int {
    static int __ov965x_set_power(struct ov965x *ov965x, int on)
    {
    if (on) {
    let mut ret: c_int = clk_prepare_enable(ov965x.clk);
    if (ret)
    return ret;
    gpiod_set_value_cansleep(ov965x.gpios[GPIO_PWDN], 0);
    gpiod_set_value_cansleep(ov965x.gpios[GPIO_RST], 0);
    msleep(25);
    } else {
    gpiod_set_value_cansleep(ov965x.gpios[GPIO_RST], 1);
    gpiod_set_value_cansleep(ov965x.gpios[GPIO_PWDN], 1);
    clk_disable_unprepare(ov965x.clk);
    }
    ov965x.streaming = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov965x_s_power(struct v4l2_subdev *sd, int on)
    {
    struct ov965x *ov965x = to_ov965x(sd);
    let mut ret: c_int = 0;
    v4l2_dbg(1, debug, sd, "%s: on: %d\n", __func__, on);
    mutex_lock(&ov965x.lock);
    if (ov965x.power == !on) {
    ret = __ov965x_set_power(ov965x, on);
    if (!ret && on) {
    ret = ov965x_write_array(ov965x,
    ov965x_init_regs);
    ov965x.apply_frame_fmt = 1;
    ov965x.ctrls.update = 1;
    }
    }
    if (!ret)
    ov965x.power += on ? 1 : -1;
    WARN_ON(ov965x.power < 0);
    mutex_unlock(&ov965x.lock);
    return ret;
    }
//
// V4L2 controls
//
#[no_mangle]
unsafe extern "C" fn ov965x_update_exposure_ctrl(ov965x: *mut ov965x) {
    static void ov965x_update_exposure_ctrl(struct ov965x *ov965x)
    {
    struct v4l2_ctrl *ctrl = ov965x.ctrls.exposure;
    unsigned long fint, trow;
    int min, max, def;
    u8 clkrc;
    mutex_lock(&ov965x.lock);
    if (WARN_ON(!ctrl || !ov965x.frame_size)) {
    mutex_unlock(&ov965x.lock);
    return;
    }
    clkrc = DEF_CLKRC + ov965x.fiv.clkrc_div;
// Calculate internal clock frequency
    fint = ov965x.mclk_frequency * ((clkrc >> 7) + 1) /
    ((2 * ((clkrc & 0x3f) + 1)));
// and the row interval (in us).
    trow = (2 * 1520 * 1000000UL) / fint;
    max = ov965x.frame_size.max_exp_lines * trow;
    ov965x.exp_row_interval = trow;
    mutex_unlock(&ov965x.lock);
    v4l2_dbg(1, debug, &ov965x.sd, "clkrc: %#x, fi: %lu, tr: %lu, %d\n",
    clkrc, fint, trow, max);
// Update exposure time range to match current frame format.
    min = (trow + 100) / 100;
    max = (max - 100) / 100;
    def = min + (max - min) / 2;
    if (v4l2_ctrl_modify_range(ctrl, min, max, 1, def))
    v4l2_err(&ov965x.sd, "Exposure ctrl range update failed\n");
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_banding_filter(ov965x: *mut ov965x, value: c_int) -> c_int {
    static int ov965x_set_banding_filter(struct ov965x *ov965x, int value)
    {
    unsigned long mbd, light_freq;
    int ret;
    u8 reg;
    ret = ov965x_read(ov965x, REG_COM8, &reg);
    if (!ret) {
    if (value == V4L2_CID_POWER_LINE_FREQUENCY_DISABLED)
    reg &= ~COM8_BFILT;
    else
    reg |= COM8_BFILT;
    ret = ov965x_write(ov965x, REG_COM8, reg);
    }
    if (value == V4L2_CID_POWER_LINE_FREQUENCY_DISABLED)
    return 0;
    if (WARN_ON(!ov965x.fiv))
    return -EINVAL;
// Set minimal exposure time for 50/60 HZ lighting
    if (value == V4L2_CID_POWER_LINE_FREQUENCY_50HZ)
    light_freq = 50;
    else
    light_freq = 60;
    mbd = (1000UL * ov965x.fiv.interval.denominator *
    ov965x.frame_size.max_exp_lines) /
    ov965x.fiv.interval.numerator;
    mbd = ((mbd / (light_freq * 2)) + 500) / 1000UL;
    return ov965x_write(ov965x, REG_MBD, mbd);
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_white_balance(ov965x: *mut ov965x, awb: c_int) -> c_int {
    static int ov965x_set_white_balance(struct ov965x *ov965x, int awb)
    {
    int ret;
    u8 reg;
    ret = ov965x_read(ov965x, REG_COM8, &reg);
    if (!ret) {
    reg = awb ? reg | REG_COM8 : reg & ~REG_COM8;
    ret = ov965x_write(ov965x, REG_COM8, reg);
    }
    if (!ret && !awb) {
    ret = ov965x_write(ov965x, REG_BLUE,
    ov965x.ctrls.blue_balance.val);
    if (ret < 0)
    return ret;
    ret = ov965x_write(ov965x, REG_RED,
    ov965x.ctrls.red_balance.val);
    }
    return ret;
    }
pub const NUM_BR_LEVELS: c_int = 7;
pub const NUM_BR_REGS: c_int = 3;
#[no_mangle]
unsafe extern "C" fn ov965x_set_brightness(ov965x: *mut ov965x, val: c_int) -> c_int {
    static int ov965x_set_brightness(struct ov965x *ov965x, int val)
    {
    static const u8 regs[NUM_BR_LEVELS + 1][NUM_BR_REGS] = {
    { REG_AEW, REG_AEB, REG_VPT },
    { 0x1c, 0x12, 0x50 }, /* -3 */
    { 0x3d, 0x30, 0x71 }, /* -2 */
    { 0x50, 0x44, 0x92 }, /* -1 */
    { 0x70, 0x64, 0xc3 }, /*  0 */
    { 0x90, 0x84, 0xd4 }, /* +1 */
    { 0xc4, 0xbf, 0xf9 }, /* +2 */
    { 0xd8, 0xd0, 0xfa }, /* +3 */
    };
    int i, ret = 0;
    val += (NUM_BR_LEVELS / 2 + 1);
    if (val > NUM_BR_LEVELS)
    return -EINVAL;
    for (i = 0; i < NUM_BR_REGS && !ret; i++)
    ret = ov965x_write(ov965x, regs[0][i],
    regs[val][i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_gain(ov965x: *mut ov965x, auto_gain: c_int) -> c_int {
    static int ov965x_set_gain(struct ov965x *ov965x, int auto_gain)
    {
    struct ov965x_ctrls *ctrls = &ov965x.ctrls;
    let mut ret: c_int = 0;
    u8 reg;
//
// For manual mode we need to disable AGC first, so
// gain value in REG_VREF, REG_GAIN is not overwritten.
//
    if (ctrls.auto_gain.is_new) {
    ret = ov965x_read(ov965x, REG_COM8, &reg);
    if (ret < 0)
    return ret;
    if (ctrls.auto_gain.val)
    reg |= COM8_AGC;
    else
    reg &= ~COM8_AGC;
    ret = ov965x_write(ov965x, REG_COM8, reg);
    if (ret < 0)
    return ret;
    }
    if (ctrls.gain.is_new && !auto_gain) {
    let mut gain: c_uint = ctrls.gain.val;
    unsigned int rgain;
    int m;
//
// Convert gain control value to the sensor's gain
// registers (VREF[7:6], GAIN[7:0]) format.
//
    for (m = 6; m >= 0; m--)
    if (gain >= (1 << m) * 16)
    break;
// Sanity check: don't adjust the gain with a negative value
    if (m < 0)
    return -EINVAL;
    rgain = (gain - ((1 << m) * 16)) / (1 << m);
    rgain |= (((1 << m) - 1) << 4);
    ret = ov965x_write(ov965x, REG_GAIN, rgain & 0xff);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_VREF, &reg);
    if (ret < 0)
    return ret;
    reg &= ~VREF_GAIN_MASK;
    reg |= (((rgain >> 8) & 0x3) << 6);
    ret = ov965x_write(ov965x, REG_VREF, reg);
    if (ret < 0)
    return ret;
// Return updated control's value to userspace
    ctrls.gain.val = (1 << m) * (16 + (rgain & 0xf));
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_sharpness(ov965x: *mut ov965x, value: c_uint) -> c_int {
    static int ov965x_set_sharpness(struct ov965x *ov965x, unsigned int value)
    {
    u8 com14, edge;
    int ret;
    ret = ov965x_read(ov965x, REG_COM14, &com14);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_EDGE, &edge);
    if (ret < 0)
    return ret;
    com14 = value ? com14 | COM14_EDGE_EN : com14 & ~COM14_EDGE_EN;
    value--;
    if (value > 0x0f) {
    com14 |= COM14_EEF_X2;
    value >>= 1;
    } else {
    com14 &= ~COM14_EEF_X2;
    }
    ret = ov965x_write(ov965x, REG_COM14, com14);
    if (ret < 0)
    return ret;
    edge &= ~EDGE_FACTOR_MASK;
    edge |= ((u8)value & 0x0f);
    return ov965x_write(ov965x, REG_EDGE, edge);
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_exposure(ov965x: *mut ov965x, exp: c_int) -> c_int {
    static int ov965x_set_exposure(struct ov965x *ov965x, int exp)
    {
    struct ov965x_ctrls *ctrls = &ov965x.ctrls;
    let mut auto_exposure: bool = (exp == V4L2_EXPOSURE_AUTO);
    int ret;
    u8 reg;
    if (ctrls.auto_exp.is_new) {
    ret = ov965x_read(ov965x, REG_COM8, &reg);
    if (ret < 0)
    return ret;
    if (auto_exposure)
    reg |= (COM8_AEC | COM8_AGC);
    else
    reg &= ~(COM8_AEC | COM8_AGC);
    ret = ov965x_write(ov965x, REG_COM8, reg);
    if (ret < 0)
    return ret;
    }
    if (!auto_exposure && ctrls.exposure.is_new) {
    unsigned int exposure = (ctrls.exposure.val * 100)
    / ov965x.exp_row_interval;
//
// Manual exposure value
// [b15:b0] - AECHM (b15:b10), AECH (b9:b2), COM1 (b1:b0)
//
    ret = ov965x_write(ov965x, REG_COM1, exposure & 0x3);
    if (!ret)
    ret = ov965x_write(ov965x, REG_AECH,
    (exposure >> 2) & 0xff);
    if (!ret)
    ret = ov965x_write(ov965x, REG_AECHM,
    (exposure >> 10) & 0x3f);
// Update the value to minimize rounding errors
    ctrls.exposure.val = ((exposure * ov965x.exp_row_interval)
    + 50) / 100;
    if (ret < 0)
    return ret;
    }
    v4l2_ctrl_activate(ov965x.ctrls.brightness, !exp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_flip(ov965x: *mut ov965x) -> c_int {
    static int ov965x_set_flip(struct ov965x *ov965x)
    {
    let mut mvfp: u8 = 0;
    if (ov965x.ctrls.hflip.val)
    mvfp |= MVFP_MIRROR;
    if (ov965x.ctrls.vflip.val)
    mvfp |= MVFP_FLIP;
    return ov965x_write(ov965x, REG_MVFP, mvfp);
    }
pub const NUM_SAT_LEVELS: c_int = 5;
pub const NUM_SAT_REGS: c_int = 6;
#[no_mangle]
unsafe extern "C" fn ov965x_set_saturation(ov965x: *mut ov965x, val: c_int) -> c_int {
    static int ov965x_set_saturation(struct ov965x *ov965x, int val)
    {
    static const u8 regs[NUM_SAT_LEVELS][NUM_SAT_REGS] = {
// MTX(1)...MTX(6)
    { 0x1d, 0x1f, 0x02, 0x09, 0x13, 0x1c }, /* -2 */
    { 0x2e, 0x31, 0x02, 0x0e, 0x1e, 0x2d }, /* -1 */
    { 0x3a, 0x3d, 0x03, 0x12, 0x26, 0x38 }, /*  0 */
    { 0x46, 0x49, 0x04, 0x16, 0x2e, 0x43 }, /* +1 */
    { 0x57, 0x5c, 0x05, 0x1b, 0x39, 0x54 }, /* +2 */
    };
    let mut addr: u8 = REG_MTX(1);
    int i, ret = 0;
    val += (NUM_SAT_LEVELS / 2);
    if (val >= NUM_SAT_LEVELS)
    return -EINVAL;
    for (i = 0; i < NUM_SAT_REGS && !ret; i++)
    ret = ov965x_write(ov965x, addr + i, regs[val][i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_test_pattern(ov965x: *mut ov965x, value: c_int) -> c_int {
    static int ov965x_set_test_pattern(struct ov965x *ov965x, int value)
    {
    int ret;
    u8 reg;
    ret = ov965x_read(ov965x, REG_COM23, &reg);
    if (ret < 0)
    return ret;
    reg = value ? reg | COM23_TEST_MODE : reg & ~COM23_TEST_MODE;
    return ov965x_write(ov965x, REG_COM23, reg);
    }
#[no_mangle]
unsafe extern "C" fn __g_volatile_ctrl(ov965x: *mut ov965x, ctrl: *mut v4l2_ctrl) -> c_int {
    static int __g_volatile_ctrl(struct ov965x *ov965x, struct v4l2_ctrl *ctrl)
    {
    unsigned int exposure, gain, m;
    u8 reg0, reg1, reg2;
    int ret;
    if (!ov965x.power)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_AUTOGAIN:
    if (!ctrl.val)
    return 0;
    ret = ov965x_read(ov965x, REG_GAIN, &reg0);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_VREF, &reg1);
    if (ret < 0)
    return ret;
    gain = ((reg1 >> 6) << 8) | reg0;
    m = 0x01 << fls(gain >> 4);
    ov965x.ctrls.gain.val = m * (16 + (gain & 0xf));
    break;
    case V4L2_CID_EXPOSURE_AUTO:
    if (ctrl.val == V4L2_EXPOSURE_MANUAL)
    return 0;
    ret = ov965x_read(ov965x, REG_COM1, &reg0);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_AECH, &reg1);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_AECHM, &reg2);
    if (ret < 0)
    return ret;
    exposure = ((reg2 & 0x3f) << 10) | (reg1 << 2) |
    (reg0 & 0x3);
    ov965x.ctrls.exposure.val = ((exposure *
    ov965x.exp_row_interval) + 50) / 100;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov965x_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = ctrl_to_sd(ctrl);
    struct ov965x *ov965x = to_ov965x(sd);
    int ret;
    v4l2_dbg(1, debug, sd, "g_ctrl: %s\n", ctrl.name);
    mutex_lock(&ov965x.lock);
    ret = __g_volatile_ctrl(ov965x, ctrl);
    mutex_unlock(&ov965x.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov965x_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = ctrl_to_sd(ctrl);
    struct ov965x *ov965x = to_ov965x(sd);
    let mut ret: c_int = -EINVAL;
    v4l2_dbg(1, debug, sd, "s_ctrl: %s, value: %d. power: %d\n",
    ctrl.name, ctrl.val, ov965x.power);
    mutex_lock(&ov965x.lock);
//
// If the device is not powered up now postpone applying control's
// value to the hardware, until it is ready to accept commands.
//
    if (ov965x.power == 0) {
    mutex_unlock(&ov965x.lock);
    return 0;
    }
    switch (ctrl.id) {
    case V4L2_CID_AUTO_WHITE_BALANCE:
    ret = ov965x_set_white_balance(ov965x, ctrl.val);
    break;
    case V4L2_CID_BRIGHTNESS:
    ret = ov965x_set_brightness(ov965x, ctrl.val);
    break;
    case V4L2_CID_EXPOSURE_AUTO:
    ret = ov965x_set_exposure(ov965x, ctrl.val);
    break;
    case V4L2_CID_AUTOGAIN:
    ret = ov965x_set_gain(ov965x, ctrl.val);
    break;
    case V4L2_CID_HFLIP:
    ret = ov965x_set_flip(ov965x);
    break;
    case V4L2_CID_POWER_LINE_FREQUENCY:
    ret = ov965x_set_banding_filter(ov965x, ctrl.val);
    break;
    case V4L2_CID_SATURATION:
    ret = ov965x_set_saturation(ov965x, ctrl.val);
    break;
    case V4L2_CID_SHARPNESS:
    ret = ov965x_set_sharpness(ov965x, ctrl.val);
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = ov965x_set_test_pattern(ov965x, ctrl.val);
    break;
    }
    mutex_unlock(&ov965x.lock);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov965x_ctrl_ops = {
    .g_volatile_ctrl = ov965x_g_volatile_ctrl,
    .s_ctrl	= ov965x_s_ctrl,
    };
    static const char * const test_pattern_menu[] = {
    "Disabled",
    "Color bars",
    };
#[no_mangle]
unsafe extern "C" fn ov965x_initialize_controls(ov965x: *mut ov965x) -> c_int {
    static int ov965x_initialize_controls(struct ov965x *ov965x)
    {
    const struct v4l2_ctrl_ops *ops = &ov965x_ctrl_ops;
    struct ov965x_ctrls *ctrls = &ov965x.ctrls;
    struct v4l2_ctrl_handler *hdl = &ctrls.handler;
    int ret;
    ret = v4l2_ctrl_handler_init(hdl, 16);
    if (ret < 0)
    return ret;
// Auto/manual white balance
    ctrls.auto_wb = v4l2_ctrl_new_std(hdl, ops,
    V4L2_CID_AUTO_WHITE_BALANCE,
    0, 1, 1, 1);
    ctrls.blue_balance = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_BLUE_BALANCE,
    0, 0xff, 1, 0x80);
    ctrls.red_balance = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_RED_BALANCE,
    0, 0xff, 1, 0x80);
// Auto/manual exposure
    ctrls.auto_exp =
    v4l2_ctrl_new_std_menu(hdl, ops,
    V4L2_CID_EXPOSURE_AUTO,
    V4L2_EXPOSURE_MANUAL, 0,
    V4L2_EXPOSURE_AUTO);
// Exposure time, in 100 us units. min/max is updated dynamically.
    ctrls.exposure = v4l2_ctrl_new_std(hdl, ops,
    V4L2_CID_EXPOSURE_ABSOLUTE,
    2, 1500, 1, 500);
// Auto/manual gain
    ctrls.auto_gain = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_AUTOGAIN,
    0, 1, 1, 1);
    ctrls.gain = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_GAIN,
    16, 64 * (16 + 15), 1, 64 * 16);
    ctrls.saturation = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_SATURATION,
    -2, 2, 1, 0);
    ctrls.brightness = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_BRIGHTNESS,
    -3, 3, 1, 0);
    ctrls.sharpness = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_SHARPNESS,
    0, 32, 1, 6);
    ctrls.hflip = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_HFLIP, 0, 1, 1, 0);
    ctrls.vflip = v4l2_ctrl_new_std(hdl, ops, V4L2_CID_VFLIP, 0, 1, 1, 0);
    ctrls.light_freq =
    v4l2_ctrl_new_std_menu(hdl, ops,
    V4L2_CID_POWER_LINE_FREQUENCY,
    V4L2_CID_POWER_LINE_FREQUENCY_60HZ, ~0x7,
    V4L2_CID_POWER_LINE_FREQUENCY_50HZ);
    v4l2_ctrl_new_std_menu_items(hdl, ops, V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(test_pattern_menu) - 1, 0, 0,
    test_pattern_menu);
    if (hdl.error) {
    ret = hdl.error;
    v4l2_ctrl_handler_free(hdl);
    return ret;
    }
    ctrls.gain.flags |= V4L2_CTRL_FLAG_VOLATILE;
    ctrls.exposure.flags |= V4L2_CTRL_FLAG_VOLATILE;
    v4l2_ctrl_auto_cluster(3, &ctrls.auto_wb, 0, false);
    v4l2_ctrl_auto_cluster(2, &ctrls.auto_gain, 0, true);
    v4l2_ctrl_auto_cluster(2, &ctrls.auto_exp, 1, true);
    v4l2_ctrl_cluster(2, &ctrls.hflip);
    ov965x.sd.ctrl_handler = hdl;
    return 0;
    }
//
// V4L2 subdev video and pad level operations
//
#[no_mangle]
unsafe extern "C" fn ov965x_get_default_format(mf: *mut v4l2_mbus_framefmt) {
    static void ov965x_get_default_format(struct v4l2_mbus_framefmt *mf)
    {
    mf.width = ov965x_framesizes[0].width;
    mf.height = ov965x_framesizes[0].height;
    mf.colorspace = ov965x_formats[0].colorspace;
    mf.code = ov965x_formats[0].code;
    mf.field = V4L2_FIELD_NONE;
    }
    static int ov965x_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index >= ARRAY_SIZE(ov965x_formats))
    return -EINVAL;
    code.code = ov965x_formats[code.index].code;
    return 0;
    }
    static int ov965x_enum_frame_sizes(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    let mut i: c_int = ARRAY_SIZE(ov965x_formats);
    if (fse.index >= ARRAY_SIZE(ov965x_framesizes))
    return -EINVAL;
    while (--i)
    if (fse.code == ov965x_formats[i].code)
    break;
    fse.code = ov965x_formats[i].code;
    fse.min_width  = ov965x_framesizes[fse.index].width;
    fse.max_width  = fse.min_width;
    fse.max_height = ov965x_framesizes[fse.index].height;
    fse.min_height = fse.max_height;
    return 0;
    }
    static int ov965x_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct ov965x *ov965x = to_ov965x(sd);
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fi.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    mutex_lock(&ov965x.lock);
    fi.interval = ov965x.fiv.interval;
    mutex_unlock(&ov965x.lock);
    return 0;
    }
    static int __ov965x_set_frame_interval(struct ov965x *ov965x,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct v4l2_mbus_framefmt *mbus_fmt = &ov965x.format;
    const struct ov965x_interval *fiv = &ov965x_intervals[0];
    u64 req_int, err, min_err = ~0ULL;
    unsigned int i;
    if (fi.interval.denominator == 0)
    return -EINVAL;
    req_int = (u64)fi.interval.numerator * 10000;
    do_div(req_int, fi.interval.denominator);
    for (i = 0; i < ARRAY_SIZE(ov965x_intervals); i++) {
    const struct ov965x_interval *iv = &ov965x_intervals[i];
    if (mbus_fmt.width != iv.size.width ||
    mbus_fmt.height != iv.size.height)
    continue;
    err = abs((u64)(iv.interval.numerator * 10000) /
    iv.interval.denominator - req_int);
    if (err < min_err) {
    fiv = iv;
    min_err = err;
    }
    }
    ov965x.fiv = fiv;
    v4l2_dbg(1, debug, &ov965x.sd, "Changed frame interval to %u us\n",
    fiv.interval.numerator * 1000000 / fiv.interval.denominator);
    return 0;
    }
    static int ov965x_set_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct ov965x *ov965x = to_ov965x(sd);
    int ret;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fi.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    v4l2_dbg(1, debug, sd, "Setting %d/%d frame interval\n",
    fi.interval.numerator, fi.interval.denominator);
    mutex_lock(&ov965x.lock);
    ret = __ov965x_set_frame_interval(ov965x, fi);
    ov965x.apply_frame_fmt = 1;
    mutex_unlock(&ov965x.lock);
    return ret;
    }
    static int ov965x_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    struct ov965x *ov965x = to_ov965x(sd);
    struct v4l2_mbus_framefmt *mf;
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY) {
    mf = v4l2_subdev_state_get_format(sd_state, 0);
    fmt.format = *mf;
    return 0;
    }
    mutex_lock(&ov965x.lock);
    fmt.format = ov965x.format;
    mutex_unlock(&ov965x.lock);
    return 0;
    }
    static void __ov965x_try_frame_size(struct v4l2_mbus_framefmt *mf,
    const struct ov965x_framesize **size)
    {
    const struct ov965x_framesize *fsize = &ov965x_framesizes[0],
// match = NULL;
    let mut i: c_int = ARRAY_SIZE(ov965x_framesizes);
    let mut min_err: c_uint = UINT_MAX;
    while (i--) {
    int err = abs(fsize.width - mf.width)
    + abs(fsize.height - mf.height);
    if (err < min_err) {
    min_err = err;
    match = fsize;
    }
    fsize++;
    }
    if (!match)
    match = &ov965x_framesizes[0];
    mf.width  = match.width;
    mf.height = match.height;
    if (size)
// size = match;
    }
    static int ov965x_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    let mut index: c_uint = ARRAY_SIZE(ov965x_formats);
    struct v4l2_mbus_framefmt *mf = &fmt.format;
    struct ov965x *ov965x = to_ov965x(sd);
    const struct ov965x_framesize *size = core::ptr::null_mut();
    let mut ret: c_int = 0;
    __ov965x_try_frame_size(mf, &size);
    while (--index)
    if (ov965x_formats[index].code == mf.code)
    break;
    mf.colorspace	= V4L2_COLORSPACE_JPEG;
    mf.code	= ov965x_formats[index].code;
    mf.field	= V4L2_FIELD_NONE;
    mutex_lock(&ov965x.lock);
    if (fmt.which == V4L2_SUBDEV_FORMAT_TRY) {
    if (sd_state) {
    mf = v4l2_subdev_state_get_format(sd_state, fmt.pad);
// mf = fmt->format;
    }
    } else {
    if (ov965x.streaming) {
    ret = -EBUSY;
    } else {
    ov965x.frame_size = size;
    ov965x.format = fmt.format;
    ov965x.tslb_reg = ov965x_formats[index].tslb_reg;
    ov965x.apply_frame_fmt = 1;
    }
    }
    if (!ret && fmt.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    struct v4l2_subdev_frame_interval fiv = {
    .interval = { 0, 1 }
    };
// Reset to minimum possible frame interval
    __ov965x_set_frame_interval(ov965x, &fiv);
    }
    mutex_unlock(&ov965x.lock);
    if (!ret)
    ov965x_update_exposure_ctrl(ov965x);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_set_frame_size(ov965x: *mut ov965x) -> c_int {
    static int ov965x_set_frame_size(struct ov965x *ov965x)
    {
    int i, ret = 0;
    for (i = 0; ret == 0 && i < NUM_FMT_REGS; i++)
    ret = ov965x_write(ov965x, frame_size_reg_addr[i],
    ov965x.frame_size.regs[i]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __ov965x_set_params(ov965x: *mut ov965x) -> c_int {
    static int __ov965x_set_params(struct ov965x *ov965x)
    {
    struct ov965x_ctrls *ctrls = &ov965x.ctrls;
    let mut ret: c_int = 0;
    u8 reg;
    if (ov965x.apply_frame_fmt) {
    reg = DEF_CLKRC + ov965x.fiv.clkrc_div;
    ret = ov965x_write(ov965x, REG_CLKRC, reg);
    if (ret < 0)
    return ret;
    ret = ov965x_set_frame_size(ov965x);
    if (ret < 0)
    return ret;
    ret = ov965x_read(ov965x, REG_TSLB, &reg);
    if (ret < 0)
    return ret;
    reg &= ~TSLB_YUYV_MASK;
    reg |= ov965x.tslb_reg;
    ret = ov965x_write(ov965x, REG_TSLB, reg);
    if (ret < 0)
    return ret;
    }
    ret = ov965x_set_default_gamma_curve(ov965x);
    if (ret < 0)
    return ret;
    ret = ov965x_set_color_matrix(ov965x);
    if (ret < 0)
    return ret;
//
// Select manual banding filter, the filter will
// be enabled further if required.
//
    ret = ov965x_read(ov965x, REG_COM11, &reg);
    if (!ret)
    reg |= COM11_BANDING;
    ret = ov965x_write(ov965x, REG_COM11, reg);
    if (ret < 0)
    return ret;
//
// Banding filter (REG_MBD value) needs to match selected
// resolution and frame rate, so it's always updated here.
//
    return ov965x_set_banding_filter(ov965x, ctrls.light_freq.val);
    }
#[no_mangle]
unsafe extern "C" fn ov965x_s_stream(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov965x_s_stream(struct v4l2_subdev *sd, int on)
    {
    struct ov965x *ov965x = to_ov965x(sd);
    struct ov965x_ctrls *ctrls = &ov965x.ctrls;
    let mut ret: c_int = 0;
    v4l2_dbg(1, debug, sd, "%s: on: %d\n", __func__, on);
    mutex_lock(&ov965x.lock);
    if (ov965x.streaming == !on) {
    if (on)
    ret = __ov965x_set_params(ov965x);
    if (!ret && ctrls.update) {
//
// ov965x_s_ctrl callback takes the mutex
// so it needs to be released here.
//
    mutex_unlock(&ov965x.lock);
    ret = v4l2_ctrl_handler_setup(&ctrls.handler);
    mutex_lock(&ov965x.lock);
    if (!ret)
    ctrls.update = 0;
    }
    if (!ret)
    ret = ov965x_write(ov965x, REG_COM2,
    on ? 0x01 : 0x11);
    }
    if (!ret)
    ov965x.streaming += on ? 1 : -1;
    WARN_ON(ov965x.streaming < 0);
    mutex_unlock(&ov965x.lock);
    return ret;
    }
//
// V4L2 subdev internal operations
//
#[no_mangle]
unsafe extern "C" fn ov965x_open(sd: *mut v4l2_subdev, fh: *mut v4l2_subdev_fh) -> c_int {
    static int ov965x_open(struct v4l2_subdev *sd, struct v4l2_subdev_fh *fh)
    {
    struct v4l2_mbus_framefmt *mf =
    v4l2_subdev_state_get_format(fh.state, 0);
    ov965x_get_default_format(mf);
    return 0;
    }
    static const struct v4l2_subdev_pad_ops ov965x_pad_ops = {
    .enum_mbus_code = ov965x_enum_mbus_code,
    .enum_frame_size = ov965x_enum_frame_sizes,
    .get_fmt = ov965x_get_fmt,
    .set_fmt = ov965x_set_fmt,
    .get_frame_interval = ov965x_get_frame_interval,
    .set_frame_interval = ov965x_set_frame_interval,
    };
    static const struct v4l2_subdev_video_ops ov965x_video_ops = {
    .s_stream = ov965x_s_stream,
    };
    static const struct v4l2_subdev_internal_ops ov965x_sd_internal_ops = {
    .open = ov965x_open,
    };
    static const struct v4l2_subdev_core_ops ov965x_core_ops = {
    .s_power = ov965x_s_power,
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,
    };
    static const struct v4l2_subdev_ops ov965x_subdev_ops = {
    .core = &ov965x_core_ops,
    .pad = &ov965x_pad_ops,
    .video = &ov965x_video_ops,
    };
#[no_mangle]
unsafe extern "C" fn ov965x_configure_gpios(ov965x: *mut ov965x) -> c_int {
    static int ov965x_configure_gpios(struct ov965x *ov965x)
    {
    struct device *dev = regmap_get_device(ov965x.regmap);
    ov965x.gpios[GPIO_PWDN] = devm_gpiod_get_optional(dev, "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov965x.gpios[GPIO_PWDN])) {
    dev_info(dev, "can't get %s GPIO\n", "powerdown");
    return PTR_ERR(ov965x.gpios[GPIO_PWDN]);
    }
    ov965x.gpios[GPIO_RST] = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(ov965x.gpios[GPIO_RST])) {
    dev_info(dev, "can't get %s GPIO\n", "reset");
    return PTR_ERR(ov965x.gpios[GPIO_RST]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_detect_sensor(sd: *mut v4l2_subdev) -> c_int {
    static int ov965x_detect_sensor(struct v4l2_subdev *sd)
    {
    struct ov965x *ov965x = to_ov965x(sd);
    u8 pid, ver;
    int ret;
    mutex_lock(&ov965x.lock);
    ret = __ov965x_set_power(ov965x, 1);
    if (ret)
    goto out;
    msleep(25);
// Check sensor revision
    ret = ov965x_read(ov965x, REG_PID, &pid);
    if (!ret)
    ret = ov965x_read(ov965x, REG_VER, &ver);
    __ov965x_set_power(ov965x, 0);
    if (!ret) {
    ov965x.id = OV965X_ID(pid, ver);
    if (ov965x.id == OV9650_ID || ov965x.id == OV9652_ID) {
    v4l2_info(sd, "Found OV%04X sensor\n", ov965x.id);
    } else {
    v4l2_err(sd, "Sensor detection failed (%04X)\n",
    ov965x.id);
    ret = -ENODEV;
    }
    }
    out:
    mutex_unlock(&ov965x.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_probe(client: *mut i2c_client) -> c_int {
    static int ov965x_probe(struct i2c_client *client)
    {
    struct v4l2_subdev *sd;
    struct ov965x *ov965x;
    int ret;
    static const struct regmap_config ov965x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0xab,
    };
    ov965x = devm_kzalloc(&client.dev, sizeof(*ov965x), GFP_KERNEL);
    if (!ov965x)
    return -ENOMEM;
    ov965x.regmap = devm_regmap_init_sccb(client, &ov965x_regmap_config);
    if (IS_ERR(ov965x.regmap)) {
    dev_err(&client.dev, "Failed to allocate register map\n");
    return PTR_ERR(ov965x.regmap);
    }
    if (dev_fwnode(&client.dev)) {
    ov965x.clk = devm_v4l2_sensor_clk_get(&client.dev, core::ptr::null_mut());
    if (IS_ERR(ov965x.clk))
    return dev_err_probe(&client.dev, PTR_ERR(ov965x.clk),
    "failed to get the clock\n");
    ov965x.mclk_frequency = clk_get_rate(ov965x.clk);
    ret = ov965x_configure_gpios(ov965x);
    if (ret < 0)
    return ret;
    } else {
    dev_err(&client.dev,
    "No device properties specified\n");
    return -EINVAL;
    }
    mutex_init(&ov965x.lock);
    sd = &ov965x.sd;
    v4l2_i2c_subdev_init(sd, client, &ov965x_subdev_ops);
    strscpy(sd.name, DRIVER_NAME, sizeof(sd.name));
    sd.internal_ops = &ov965x_sd_internal_ops;
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    ov965x.pad.flags = MEDIA_PAD_FL_SOURCE;
    sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&sd.entity, 1, &ov965x.pad);
    if (ret < 0)
    goto err_mutex;
    ret = ov965x_initialize_controls(ov965x);
    if (ret < 0)
    goto err_me;
    ov965x_get_default_format(&ov965x.format);
    ov965x.frame_size = &ov965x_framesizes[0];
    ov965x.fiv = &ov965x_intervals[0];
    ret = ov965x_detect_sensor(sd);
    if (ret < 0)
    goto err_ctrls;
// Update exposure time min/max to match frame format
    ov965x_update_exposure_ctrl(ov965x);
    ret = v4l2_async_register_subdev(sd);
    if (ret < 0)
    goto err_ctrls;
    return 0;
    err_ctrls:
    v4l2_ctrl_handler_free(sd.ctrl_handler);
    err_me:
    media_entity_cleanup(&sd.entity);
    err_mutex:
    mutex_destroy(&ov965x.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov965x_remove(client: *mut i2c_client) {
    static void ov965x_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct ov965x *ov965x = to_ov965x(sd);
    v4l2_async_unregister_subdev(sd);
    v4l2_ctrl_handler_free(sd.ctrl_handler);
    media_entity_cleanup(&sd.entity);
    mutex_destroy(&ov965x.lock);
    }
    static const struct i2c_device_id ov965x_id[] = {
    { .name = "OV9650" },
    { .name = "OV9652" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, ov965x_id);

    static const struct of_device_id ov965x_of_match[] = {
    { .compatible = "ovti,ov9650", },
    { .compatible = "ovti,ov9652", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ov965x_of_match);

    static struct i2c_driver ov965x_i2c_driver = {
    .driver = {
    .name	= DRIVER_NAME,
    .of_match_table = of_match_ptr(ov965x_of_match),
    },
    .probe		= ov965x_probe,
    .remove		= ov965x_remove,
    .id_table	= ov965x_id,
    };
    module_i2c_driver(ov965x_i2c_driver);
    MODULE_AUTHOR("Sylwester Nawrocki <sylvester.nawrocki@gmail.com>");
    MODULE_DESCRIPTION("OV9650/OV9652 CMOS Image Sensor driver");
    MODULE_LICENSE("GPL");
