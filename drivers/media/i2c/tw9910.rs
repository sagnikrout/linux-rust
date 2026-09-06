//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/tw9910.c
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
// tw9910 Video Driver
//
// Copyright (C) 2017 Jacopo Mondi <jacopo+renesas@jmondi.org>
//
// Copyright (C) 2008 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on ov772x driver,
//
// Copyright (C) 2008 Kuninori Morimoto <morimoto.kuninori@renesas.com>
// Copyright 2006-7 Jonathan Corbet <corbet@lwn.net>
// Copyright (C) 2008 Magnus Damm
// Copyright (C) 2008, Guennadi Liakhovetski <kernel@pengutronix.de>
//

//
// register offset
//
pub const ID: c_uint = 0x00 /* Product ID Code Register */;
pub const STATUS1: c_uint = 0x01 /* Chip Status Register I */;
pub const INFORM: c_uint = 0x02 /* Input Format */;
pub const OPFORM: c_uint = 0x03 /* Output Format Control Register */;
pub const DLYCTR: c_uint = 0x04 /* Hysteresis and HSYNC Delay Control */;
pub const OUTCTR1: c_uint = 0x05 /* Output Control I */;
pub const ACNTL1: c_uint = 0x06 /* Analog Control Register 1 */;
pub const CROP_HI: c_uint = 0x07 /* Cropping Register, High */;
pub const VDELAY_LO: c_uint = 0x08 /* Vertical Delay Register, Low */;
pub const VACTIVE_LO: c_uint = 0x09 /* Vertical Active Register, Low */;
pub const HDELAY_LO: c_uint = 0x0A /* Horizontal Delay Register, Low */;
pub const HACTIVE_LO: c_uint = 0x0B /* Horizontal Active Register, Low */;
pub const CNTRL1: c_uint = 0x0C /* Control Register I */;
pub const VSCALE_LO: c_uint = 0x0D /* Vertical Scaling Register, Low */;
pub const SCALE_HI: c_uint = 0x0E /* Scaling Register, High */;
pub const HSCALE_LO: c_uint = 0x0F /* Horizontal Scaling Register, Low */;
pub const BRIGHT: c_uint = 0x10 /* BRIGHTNESS Control Register */;
pub const CONTRAST: c_uint = 0x11 /* CONTRAST Control Register */;
pub const SHARPNESS: c_uint = 0x12 /* SHARPNESS Control Register I */;
pub const SAT_U: c_uint = 0x13 /* Chroma (U) Gain Register */;
pub const SAT_V: c_uint = 0x14 /* Chroma (V) Gain Register */;
pub const HUE: c_uint = 0x15 /* Hue Control Register */;
pub const CORING1: c_uint = 0x17;
pub const CORING2: c_uint = 0x18 /* Coring and IF compensation */;
pub const VBICNTL: c_uint = 0x19 /* VBI Control Register */;
pub const ACNTL2: c_uint = 0x1A /* Analog Control 2 */;
pub const OUTCTR2: c_uint = 0x1B /* Output Control 2 */;
pub const SDT: c_uint = 0x1C /* Standard Selection */;
pub const SDTR: c_uint = 0x1D /* Standard Recognition */;
pub const TEST: c_uint = 0x1F /* Test Control Register */;
pub const CLMPG: c_uint = 0x20 /* Clamping Gain */;
pub const IAGC: c_uint = 0x21 /* Individual AGC Gain */;
pub const AGCGAIN: c_uint = 0x22 /* AGC Gain */;
pub const PEAKWT: c_uint = 0x23 /* White Peak Threshold */;
pub const CLMPL: c_uint = 0x24 /* Clamp level */;
pub const SYNCT: c_uint = 0x25 /* Sync Amplitude */;
pub const MISSCNT: c_uint = 0x26 /* Sync Miss Count Register */;
pub const PCLAMP: c_uint = 0x27 /* Clamp Position Register */;
pub const VCNTL1: c_uint = 0x28 /* Vertical Control I */;
pub const VCNTL2: c_uint = 0x29 /* Vertical Control II */;
pub const CKILL: c_uint = 0x2A /* Color Killer Level Control */;
pub const COMB: c_uint = 0x2B /* Comb Filter Control */;
pub const LDLY: c_uint = 0x2C /* Luma Delay and H Filter Control */;
pub const MISC1: c_uint = 0x2D /* Miscellaneous Control I */;
pub const LOOP: c_uint = 0x2E /* LOOP Control Register */;
pub const MISC2: c_uint = 0x2F /* Miscellaneous Control II */;
pub const MVSN: c_uint = 0x30 /* Macrovision Detection */;
pub const STATUS2: c_uint = 0x31 /* Chip STATUS II */;
pub const HFREF: c_uint = 0x32 /* H monitor */;
pub const CLMD: c_uint = 0x33 /* CLAMP MODE */;
pub const IDCNTL: c_uint = 0x34 /* ID Detection Control */;
pub const CLCNTL1: c_uint = 0x35 /* Clamp Control I */;
pub const ANAPLLCTL: c_uint = 0x4C;
pub const VBIMIN: c_uint = 0x4D;
pub const HSLOWCTL: c_uint = 0x4E;
pub const WSS3: c_uint = 0x4F;
pub const FILLDATA: c_uint = 0x50;
pub const SDID: c_uint = 0x51;
pub const DID: c_uint = 0x52;
pub const WSS1: c_uint = 0x53;
pub const WSS2: c_uint = 0x54;
pub const VVBI: c_uint = 0x55;
pub const LCTL6: c_uint = 0x56;
pub const LCTL7: c_uint = 0x57;
pub const LCTL8: c_uint = 0x58;
pub const LCTL9: c_uint = 0x59;
pub const LCTL10: c_uint = 0x5A;
pub const LCTL11: c_uint = 0x5B;
pub const LCTL12: c_uint = 0x5C;
pub const LCTL13: c_uint = 0x5D;
pub const LCTL14: c_uint = 0x5E;
pub const LCTL15: c_uint = 0x5F;
pub const LCTL16: c_uint = 0x60;
pub const LCTL17: c_uint = 0x61;
pub const LCTL18: c_uint = 0x62;
pub const LCTL19: c_uint = 0x63;
pub const LCTL20: c_uint = 0x64;
pub const LCTL21: c_uint = 0x65;
pub const LCTL22: c_uint = 0x66;
pub const LCTL23: c_uint = 0x67;
pub const LCTL24: c_uint = 0x68;
pub const LCTL25: c_uint = 0x69;
pub const LCTL26: c_uint = 0x6A;
pub const HSBEGIN: c_uint = 0x6B;
pub const HSEND: c_uint = 0x6C;
pub const OVSDLY: c_uint = 0x6D;
pub const OVSEND: c_uint = 0x6E;
pub const VBIDELAY: c_uint = 0x6F;
//
// register detail
//
// INFORM
pub const FC27_ON: c_uint = 0x40 /* 1 : Input crystal clock frequency is 27MHz */;
pub const FC27_FF: c_uint = 0x00 /* 0 : Square pixel mode. */;
// Must use 24.54MHz for 60Hz field rate
// source or 29.5MHz for 50Hz field rate
pub const IFSEL_S: c_uint = 0x10 /* 01 : S-video decoding */;
pub const IFSEL_C: c_uint = 0x00 /* 00 : Composite video decoding */;
// Y input video selection
pub const YSEL_M0: c_uint = 0x00 /*  00 : Mux0 selected */;
pub const YSEL_M1: c_uint = 0x04 /*  01 : Mux1 selected */;
pub const YSEL_M2: c_uint = 0x08 /*  10 : Mux2 selected */;
pub const YSEL_M3: c_uint = 0x10 /*  11 : Mux3 selected */;
// OPFORM
pub const MODE: c_uint = 0x80 /* 0 : CCIR601 compatible YCrCb 4:2:2 format */;
// 1 : ITU-R-656 compatible data sequence format
pub const LEN: c_uint = 0x40 /* 0 : 8-bit YCrCb 4:2:2 output format */;
// 1 : 16-bit YCrCb 4:2:2 output format.
pub const LLCMODE: c_uint = 0x20 /* 1 : LLC output mode. */;
// 0 : free-run output mode
pub const AINC: c_uint = 0x10 /* Serial interface auto-indexing control */;
// 0 : auto-increment
// 1 : non-auto
pub const VSCTL: c_uint = 0x08 /* 1 : Vertical out ctrl by DVALID */;
// 0 : Vertical out ctrl by HACTIVE and DVALID
pub const OEN_TRI_SEL_MASK: c_uint = 0x07;
pub const OEN_TRI_SEL_ALL_ON: c_uint = 0x00 /* Enable output for Rev0/Rev1 */;
pub const OEN_TRI_SEL_ALL_OFF_r0: c_uint = 0x06 /* All tri-stated for Rev0 */;
pub const OEN_TRI_SEL_ALL_OFF_r1: c_uint = 0x07 /* All tri-stated for Rev1 */;
// OUTCTR1
pub const VSP_LO: c_uint = 0x00 /* 0 : VS pin output polarity is active low */;
pub const VSP_HI: c_uint = 0x80 /* 1 : VS pin output polarity is active high. */;
// VS pin output control
pub const VSSL_VSYNC: c_uint = 0x00 /*   0 : VSYNC  */;
pub const VSSL_VACT: c_uint = 0x10 /*   1 : VACT   */;
pub const VSSL_FIELD: c_uint = 0x20 /*   2 : FIELD  */;
pub const VSSL_VVALID: c_uint = 0x30 /*   3 : VVALID */;
pub const VSSL_ZERO: c_uint = 0x70 /*   7 : 0      */;
pub const HSP_LOW: c_uint = 0x00 /* 0 : HS pin output polarity is active low */;
pub const HSP_HI: c_uint = 0x08 /* 1 : HS pin output polarity is active high.*/;
// HS pin output control
pub const HSSL_HACT: c_uint = 0x00 /*   0 : HACT   */;
pub const HSSL_HSYNC: c_uint = 0x01 /*   1 : HSYNC  */;
pub const HSSL_DVALID: c_uint = 0x02 /*   2 : DVALID */;
pub const HSSL_HLOCK: c_uint = 0x03 /*   3 : HLOCK  */;
pub const HSSL_ASYNCW: c_uint = 0x04 /*   4 : ASYNCW */;
pub const HSSL_ZERO: c_uint = 0x07 /*   7 : 0      */;
// ACNTL1
pub const SRESET: c_uint = 0x80 /* resets the device to its default state;
// but all register content remain unchanged.
// This bit is self-resetting.
//
pub const ACNTL1_PDN_MASK: c_uint = 0x0e;
pub const CLK_PDN: c_uint = 0x08 /* system clock power down */;
pub const Y_PDN: c_uint = 0x04 /* Luma ADC power down */;
pub const C_PDN: c_uint = 0x02 /* Chroma ADC power down */;
// ACNTL2
pub const ACNTL2_PDN_MASK: c_uint = 0x40;
pub const PLL_PDN: c_uint = 0x40 /* PLL power down */;
// VBICNTL
// RTSEL : control the real time signal output from the MPOUT pin
pub const RTSEL_MASK: c_uint = 0x07;
pub const RTSEL_VLOSS: c_uint = 0x00 /* 0000 = Video loss */;
pub const RTSEL_HLOCK: c_uint = 0x01 /* 0001 = H-lock */;
pub const RTSEL_SLOCK: c_uint = 0x02 /* 0010 = S-lock */;
pub const RTSEL_VLOCK: c_uint = 0x03 /* 0011 = V-lock */;
pub const RTSEL_MONO: c_uint = 0x04 /* 0100 = MONO */;
pub const RTSEL_DET50: c_uint = 0x05 /* 0101 = DET50 */;
pub const RTSEL_FIELD: c_uint = 0x06 /* 0110 = FIELD */;
pub const RTSEL_RTCO: c_uint = 0x07 /* 0111 = RTCO ( Real Time Control ) */;
// HSYNC start and end are constant for now
pub const HSYNC_START: c_uint = 0x0260;
pub const HSYNC_END: c_uint = 0x0300;
//
// structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw9910_scale_ctrl {
    pub name: *mut c_char,
    pub width: c_ushort,
    pub height: c_ushort,
    pub hscale: u16,
    pub vscale: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw9910_priv {
    pub subdev: v4l2_subdev,
    pub clk: *mut clk,
    pub info: *mut tw9910_video_info,
    pub pdn_gpio: *mut gpio_desc,
    pub rstb_gpio: *mut gpio_desc,
    pub scale: *const tw9910_scale_ctrl,
    pub norm: v4l2_std_id,
    pub revision: u32,
}

    static const struct tw9910_scale_ctrl tw9910_ntsc_scales[] = {
    {
    .name   = "NTSC SQ",
    .width  = 640,
    .height = 480,
    .hscale = 0x0100,
    .vscale = 0x0100,
    },
    {
    .name   = "NTSC CCIR601",
    .width  = 720,
    .height = 480,
    .hscale = 0x0100,
    .vscale = 0x0100,
    },
    {
    .name   = "NTSC SQ (CIF)",
    .width  = 320,
    .height = 240,
    .hscale = 0x0200,
    .vscale = 0x0200,
    },
    {
    .name   = "NTSC CCIR601 (CIF)",
    .width  = 360,
    .height = 240,
    .hscale = 0x0200,
    .vscale = 0x0200,
    },
    {
    .name   = "NTSC SQ (QCIF)",
    .width  = 160,
    .height = 120,
    .hscale = 0x0400,
    .vscale = 0x0400,
    },
    {
    .name   = "NTSC CCIR601 (QCIF)",
    .width  = 180,
    .height = 120,
    .hscale = 0x0400,
    .vscale = 0x0400,
    },
    };
    static const struct tw9910_scale_ctrl tw9910_pal_scales[] = {
    {
    .name   = "PAL SQ",
    .width  = 768,
    .height = 576,
    .hscale = 0x0100,
    .vscale = 0x0100,
    },
    {
    .name   = "PAL CCIR601",
    .width  = 720,
    .height = 576,
    .hscale = 0x0100,
    .vscale = 0x0100,
    },
    {
    .name   = "PAL SQ (CIF)",
    .width  = 384,
    .height = 288,
    .hscale = 0x0200,
    .vscale = 0x0200,
    },
    {
    .name   = "PAL CCIR601 (CIF)",
    .width  = 360,
    .height = 288,
    .hscale = 0x0200,
    .vscale = 0x0200,
    },
    {
    .name   = "PAL SQ (QCIF)",
    .width  = 192,
    .height = 144,
    .hscale = 0x0400,
    .vscale = 0x0400,
    },
    {
    .name   = "PAL CCIR601 (QCIF)",
    .width  = 180,
    .height = 144,
    .hscale = 0x0400,
    .vscale = 0x0400,
    },
    };
//
// general function
//
    static struct tw9910_priv *to_tw9910(const struct i2c_client *client)
    {
    return container_of(i2c_get_clientdata(client), struct tw9910_priv,
    subdev);
    }
    static int tw9910_mask_set(struct i2c_client *client, u8 command,
    u8 mask, u8 set)
    {
    let mut val: i32 = i2c_smbus_read_byte_data(client, command);
    if (val < 0)
    return val;
    val &= ~mask;
    val |= set & mask;
    return i2c_smbus_write_byte_data(client, command, val);
    }
    static int tw9910_set_scale(struct i2c_client *client,
    const struct tw9910_scale_ctrl *scale)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, SCALE_HI,
    (scale.vscale & 0x0F00) >> 4 |
    (scale.hscale & 0x0F00) >> 8);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, HSCALE_LO,
    scale.hscale & 0x00FF);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(client, VSCALE_LO,
    scale.vscale & 0x00FF);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_set_hsync(client: *mut i2c_client) -> c_int {
    static int tw9910_set_hsync(struct i2c_client *client)
    {
    struct tw9910_priv *priv = to_tw9910(client);
    int ret;
// bit 10 - 3
    ret = i2c_smbus_write_byte_data(client, HSBEGIN,
    (HSYNC_START & 0x07F8) >> 3);
    if (ret < 0)
    return ret;
// bit 10 - 3
    ret = i2c_smbus_write_byte_data(client, HSEND,
    (HSYNC_END & 0x07F8) >> 3);
    if (ret < 0)
    return ret;
// So far only revisions 0 and 1 have been seen.
// bit 2 - 0
    if (priv.revision == 1)
    ret = tw9910_mask_set(client, HSLOWCTL, 0x77,
    (HSYNC_START & 0x0007) << 4 |
    (HSYNC_END   & 0x0007));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_reset(client: *mut i2c_client) {
    static void tw9910_reset(struct i2c_client *client)
    {
    tw9910_mask_set(client, ACNTL1, SRESET, SRESET);
    usleep_range(1000, 5000);
    }
#[no_mangle]
unsafe extern "C" fn tw9910_power(client: *mut i2c_client, enable: c_int) -> c_int {
    static int tw9910_power(struct i2c_client *client, int enable)
    {
    int ret;
    u8 acntl1;
    u8 acntl2;
    if (enable) {
    acntl1 = 0;
    acntl2 = 0;
    } else {
    acntl1 = CLK_PDN | Y_PDN | C_PDN;
    acntl2 = PLL_PDN;
    }
    ret = tw9910_mask_set(client, ACNTL1, ACNTL1_PDN_MASK, acntl1);
    if (ret < 0)
    return ret;
    return tw9910_mask_set(client, ACNTL2, ACNTL2_PDN_MASK, acntl2);
    }
    static const struct tw9910_scale_ctrl *tw9910_select_norm(v4l2_std_id norm,
    u32 width, u32 height)
    {
    const struct tw9910_scale_ctrl *scale;
    const struct tw9910_scale_ctrl *ret = core::ptr::null_mut();
    let mut diff: __u32 = 0xffffffff, tmp;
    int size, i;
    if (norm & V4L2_STD_NTSC) {
    scale = tw9910_ntsc_scales;
    size = ARRAY_SIZE(tw9910_ntsc_scales);
    } else if (norm & V4L2_STD_PAL) {
    scale = tw9910_pal_scales;
    size = ARRAY_SIZE(tw9910_pal_scales);
    } else {
    return core::ptr::null_mut();
    }
    for (i = 0; i < size; i++) {
    tmp = abs(width - scale[i].width) +
    abs(height - scale[i].height);
    if (tmp < diff) {
    diff = tmp;
    ret = scale + i;
    }
    }
    return ret;
    }
//
// subdevice operations
//
#[no_mangle]
unsafe extern "C" fn tw9910_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int tw9910_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    u8 val;
    int ret;
    if (!enable) {
    switch (priv.revision) {
    case 0:
    val = OEN_TRI_SEL_ALL_OFF_r0;
    break;
    case 1:
    val = OEN_TRI_SEL_ALL_OFF_r1;
    break;
    default:
    dev_err(&client.dev, "un-supported revision\n");
    return -EINVAL;
    }
    } else {
    val = OEN_TRI_SEL_ALL_ON;
    if (!priv.scale) {
    dev_err(&client.dev, "norm select error\n");
    return -EPERM;
    }
    dev_dbg(&client.dev, "%s %dx%d\n",
    priv.scale.name,
    priv.scale.width,
    priv.scale.height);
    }
    ret = tw9910_mask_set(client, OPFORM, OEN_TRI_SEL_MASK, val);
    if (ret < 0)
    return ret;
    return tw9910_power(client, enable);
    }
#[no_mangle]
unsafe extern "C" fn tw9910_g_std(sd: *mut v4l2_subdev, norm: *mut v4l2_std_id) -> c_int {
    static int tw9910_g_std(struct v4l2_subdev *sd, v4l2_std_id *norm)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
// norm = priv->norm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_s_std(sd: *mut v4l2_subdev, norm: v4l2_std_id) -> c_int {
    static int tw9910_s_std(struct v4l2_subdev *sd, v4l2_std_id norm)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    let mut hact: c_uint = 720;
    let mut hdelay: c_uint = 15;
    unsigned int vact;
    unsigned int vdelay;
    int ret;
    if (!(norm & (V4L2_STD_NTSC | V4L2_STD_PAL)))
    return -EINVAL;
    priv.norm = norm;
    if (norm & V4L2_STD_525_60) {
    vact = 240;
    vdelay = 18;
    ret = tw9910_mask_set(client, VVBI, 0x10, 0x10);
    } else {
    vact = 288;
    vdelay = 24;
    ret = tw9910_mask_set(client, VVBI, 0x10, 0x00);
    }
    if (!ret)
    ret = i2c_smbus_write_byte_data(client, CROP_HI,
    ((vdelay >> 2) & 0xc0)	|
    ((vact >> 4) & 0x30)	|
    ((hdelay >> 6) & 0x0c)	|
    ((hact >> 8) & 0x03));
    if (!ret)
    ret = i2c_smbus_write_byte_data(client, VDELAY_LO,
    vdelay & 0xff);
    if (!ret)
    ret = i2c_smbus_write_byte_data(client, VACTIVE_LO,
    vact & 0xff);
    return ret;
    }

    static int tw9910_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    int ret;
    if (reg.reg > 0xff)
    return -EINVAL;
    reg.size = 1;
    ret = i2c_smbus_read_byte_data(client, reg.reg);
    if (ret < 0)
    return ret;
//
// ret      = int
// reg->val = __u64
//
    reg.val = (__u64)ret;
    return 0;
    }
    static int tw9910_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    if (reg.reg > 0xff ||
    reg.val > 0xff)
    return -EINVAL;
    return i2c_smbus_write_byte_data(client, reg.reg, reg.val);
    }

#[no_mangle]
unsafe extern "C" fn tw9910_set_gpio_value(desc: *mut gpio_desc, value: c_int) {
    static void tw9910_set_gpio_value(struct gpio_desc *desc, int value)
    {
    if (desc) {
    gpiod_set_value(desc, value);
    usleep_range(500, 1000);
    }
    }
#[no_mangle]
unsafe extern "C" fn tw9910_power_on(priv: *mut tw9910_priv) -> c_int {
    static int tw9910_power_on(struct tw9910_priv *priv)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&priv.subdev);
    int ret;
    if (priv.clk) {
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    }
    tw9910_set_gpio_value(priv.pdn_gpio, 0);
//
// FIXME: The reset signal is connected to a shared GPIO on some
// platforms (namely the SuperH Migo-R). Until a framework becomes
// available to handle this cleanly, request the GPIO temporarily
// to avoid conflicts.
//
    priv.rstb_gpio = gpiod_get_optional(&client.dev, "rstb",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.rstb_gpio)) {
    dev_info(&client.dev, "Unable to get GPIO \"rstb\"");
    clk_disable_unprepare(priv.clk);
    tw9910_set_gpio_value(priv.pdn_gpio, 1);
    return PTR_ERR(priv.rstb_gpio);
    }
    if (priv.rstb_gpio) {
    tw9910_set_gpio_value(priv.rstb_gpio, 1);
    tw9910_set_gpio_value(priv.rstb_gpio, 0);
    gpiod_put(priv.rstb_gpio);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_power_off(priv: *mut tw9910_priv) -> c_int {
    static int tw9910_power_off(struct tw9910_priv *priv)
    {
    clk_disable_unprepare(priv.clk);
    tw9910_set_gpio_value(priv.pdn_gpio, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int tw9910_s_power(struct v4l2_subdev *sd, int on)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    return on ? tw9910_power_on(priv) : tw9910_power_off(priv);
    }
#[no_mangle]
unsafe extern "C" fn tw9910_set_frame(sd: *mut v4l2_subdev, width: *mut u32, height: *mut u32) -> c_int {
    static int tw9910_set_frame(struct v4l2_subdev *sd, u32 *width, u32 *height)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    let mut ret: c_int = -EINVAL;
    u8 val;
// Select suitable norm.
    priv.scale = tw9910_select_norm(priv.norm, *width, *height);
    if (!priv.scale)
    goto tw9910_set_fmt_error;
// Reset hardware.
    tw9910_reset(client);
// Set bus width.
    val = 0x00;
    if (priv.info.buswidth == 16)
    val = LEN;
    ret = tw9910_mask_set(client, OPFORM, LEN, val);
    if (ret < 0)
    goto tw9910_set_fmt_error;
// Select MPOUT behavior.
    switch (priv.info.mpout) {
    case TW9910_MPO_VLOSS:
    val = RTSEL_VLOSS; break;
    case TW9910_MPO_HLOCK:
    val = RTSEL_HLOCK; break;
    case TW9910_MPO_SLOCK:
    val = RTSEL_SLOCK; break;
    case TW9910_MPO_VLOCK:
    val = RTSEL_VLOCK; break;
    case TW9910_MPO_MONO:
    val = RTSEL_MONO;  break;
    case TW9910_MPO_DET50:
    val = RTSEL_DET50; break;
    case TW9910_MPO_FIELD:
    val = RTSEL_FIELD; break;
    case TW9910_MPO_RTCO:
    val = RTSEL_RTCO;  break;
    default:
    val = 0;
    }
    ret = tw9910_mask_set(client, VBICNTL, RTSEL_MASK, val);
    if (ret < 0)
    goto tw9910_set_fmt_error;
// Set scale.
    ret = tw9910_set_scale(client, priv.scale);
    if (ret < 0)
    goto tw9910_set_fmt_error;
// Set hsync.
    ret = tw9910_set_hsync(client);
    if (ret < 0)
    goto tw9910_set_fmt_error;
// width = priv->scale->width;
// height = priv->scale->height;
    return ret;
    tw9910_set_fmt_error:
    tw9910_reset(client);
    priv.scale = core::ptr::null_mut();
    return ret;
    }
    static int tw9910_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
// Only CROP, CROP_DEFAULT and CROP_BOUNDS are supported.
    if (sel.target > V4L2_SEL_TGT_CROP_BOUNDS)
    return -EINVAL;
    sel.r.left	= 0;
    sel.r.top	= 0;
    if (priv.norm & V4L2_STD_NTSC) {
    sel.r.width	= 640;
    sel.r.height	= 480;
    } else {
    sel.r.width	= 768;
    sel.r.height	= 576;
    }
    return 0;
    }
    static int tw9910_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    if (format.pad)
    return -EINVAL;
    if (!priv.scale) {
    priv.scale = tw9910_select_norm(priv.norm, 640, 480);
    if (!priv.scale)
    return -EINVAL;
    }
    mf.width	= priv.scale.width;
    mf.height	= priv.scale.height;
    mf.code	= MEDIA_BUS_FMT_UYVY8_2X8;
    mf.colorspace	= V4L2_COLORSPACE_SMPTE170M;
    mf.field	= V4L2_FIELD_INTERLACED_BT;
    return 0;
    }
    static int tw9910_s_fmt(struct v4l2_subdev *sd,
    struct v4l2_mbus_framefmt *mf)
    {
    let mut width: u32 = mf.width, height = mf.height;
    int ret;
    WARN_ON(mf.field != V4L2_FIELD_ANY &&
    mf.field != V4L2_FIELD_INTERLACED_BT);
// Check color format.
    if (mf.code != MEDIA_BUS_FMT_UYVY8_2X8)
    return -EINVAL;
    mf.colorspace = V4L2_COLORSPACE_SMPTE170M;
    ret = tw9910_set_frame(sd, &width, &height);
    if (ret)
    return ret;
    mf.width	= width;
    mf.height	= height;
    return 0;
    }
    static int tw9910_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct tw9910_priv *priv = to_tw9910(client);
    const struct tw9910_scale_ctrl *scale;
    if (format.pad)
    return -EINVAL;
    if (mf.field == V4L2_FIELD_ANY) {
    mf.field = V4L2_FIELD_INTERLACED_BT;
    } else if (mf.field != V4L2_FIELD_INTERLACED_BT) {
    dev_err(&client.dev, "Field type %d invalid\n", mf.field);
    return -EINVAL;
    }
    mf.code = MEDIA_BUS_FMT_UYVY8_2X8;
    mf.colorspace = V4L2_COLORSPACE_SMPTE170M;
// Select suitable norm.
    scale = tw9910_select_norm(priv.norm, mf.width, mf.height);
    if (!scale)
    return -EINVAL;
    mf.width	= scale.width;
    mf.height	= scale.height;
    if (format.which == V4L2_SUBDEV_FORMAT_ACTIVE)
    return tw9910_s_fmt(sd, mf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_video_probe(client: *mut i2c_client) -> c_int {
    static int tw9910_video_probe(struct i2c_client *client)
    {
    struct tw9910_priv *priv = to_tw9910(client);
    s32 id;
    int ret;
// TW9910 only use 8 or 16 bit bus width.
    if (priv.info.buswidth != 16 && priv.info.buswidth != 8) {
    dev_err(&client.dev, "bus width error\n");
    return -ENODEV;
    }
    ret = tw9910_s_power(&priv.subdev, 1);
    if (ret < 0)
    return ret;
//
// Check and show Product ID.
// So far only revisions 0 and 1 have been seen.
//
    id = i2c_smbus_read_byte_data(client, ID);
    priv.revision = GET_REV(id);
    id = GET_ID(id);
    if (id != 0x0b || priv.revision > 0x01) {
    dev_err(&client.dev, "Product ID error %x:%x\n",
    id, priv.revision);
    ret = -ENODEV;
    goto done;
    }
    dev_info(&client.dev, "tw9910 Product ID %0x:%0x\n",
    id, priv.revision);
    priv.norm = V4L2_STD_NTSC;
    priv.scale = &tw9910_ntsc_scales[0];
    done:
    tw9910_s_power(&priv.subdev, 0);
    return ret;
    }
    static const struct v4l2_subdev_core_ops tw9910_subdev_core_ops = {

    .g_register	= tw9910_g_register,
    .s_register	= tw9910_s_register,

    .s_power	= tw9910_s_power,
    };
    static int tw9910_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_UYVY8_2X8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_g_tvnorms(sd: *mut v4l2_subdev, norm: *mut v4l2_std_id) -> c_int {
    static int tw9910_g_tvnorms(struct v4l2_subdev *sd, v4l2_std_id *norm)
    {
// norm = V4L2_STD_NTSC | V4L2_STD_PAL;
    return 0;
    }
    static const struct v4l2_subdev_video_ops tw9910_subdev_video_ops = {
    .s_std		= tw9910_s_std,
    .g_std		= tw9910_g_std,
    .s_stream	= tw9910_s_stream,
    .g_tvnorms	= tw9910_g_tvnorms,
    };
    static const struct v4l2_subdev_pad_ops tw9910_subdev_pad_ops = {
    .enum_mbus_code = tw9910_enum_mbus_code,
    .get_selection	= tw9910_get_selection,
    .get_fmt	= tw9910_get_fmt,
    .set_fmt	= tw9910_set_fmt,
    };
    static const struct v4l2_subdev_ops tw9910_subdev_ops = {
    .core	= &tw9910_subdev_core_ops,
    .video	= &tw9910_subdev_video_ops,
    .pad	= &tw9910_subdev_pad_ops,
    };
//
// i2c_driver function
//
#[no_mangle]
unsafe extern "C" fn tw9910_probe(client: *mut i2c_client) -> c_int {
    static int tw9910_probe(struct i2c_client *client)
    {
    struct tw9910_priv		*priv;
    struct tw9910_video_info	*info;
    struct i2c_adapter		*adapter = client.adapter;
    int ret;
    if (!client.dev.platform_data) {
    dev_err(&client.dev, "TW9910: missing platform data!\n");
    return -EINVAL;
    }
    info = client.dev.platform_data;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&client.dev,
    "I2C-Adapter doesn't support I2C_FUNC_SMBUS_BYTE_DATA\n");
    return -EIO;
    }
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.info = info;
    v4l2_i2c_subdev_init(&priv.subdev, client, &tw9910_subdev_ops);
    priv.clk = clk_get(&client.dev, "xti");
    if (PTR_ERR(priv.clk) == -ENOENT) {
    priv.clk = core::ptr::null_mut();
    } else if (IS_ERR(priv.clk)) {
    dev_err(&client.dev, "Unable to get xti clock\n");
    return PTR_ERR(priv.clk);
    }
    priv.pdn_gpio = gpiod_get_optional(&client.dev, "pdn",
    GPIOD_OUT_HIGH);
    if (IS_ERR(priv.pdn_gpio)) {
    dev_info(&client.dev, "Unable to get GPIO \"pdn\"");
    ret = PTR_ERR(priv.pdn_gpio);
    goto error_clk_put;
    }
    ret = tw9910_video_probe(client);
    if (ret < 0)
    goto error_gpio_put;
    ret = v4l2_async_register_subdev(&priv.subdev);
    if (ret)
    goto error_gpio_put;
    return ret;
    error_gpio_put:
    if (priv.pdn_gpio)
    gpiod_put(priv.pdn_gpio);
    error_clk_put:
    clk_put(priv.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tw9910_remove(client: *mut i2c_client) {
    static void tw9910_remove(struct i2c_client *client)
    {
    struct tw9910_priv *priv = to_tw9910(client);
    if (priv.pdn_gpio)
    gpiod_put(priv.pdn_gpio);
    clk_put(priv.clk);
    v4l2_async_unregister_subdev(&priv.subdev);
    }
    static const struct i2c_device_id tw9910_id[] = {
    { .name = "tw9910" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tw9910_id);
    static struct i2c_driver tw9910_i2c_driver = {
    .driver = {
    .name = "tw9910",
    },
    .probe    = tw9910_probe,
    .remove   = tw9910_remove,
    .id_table = tw9910_id,
    };
    module_i2c_driver(tw9910_i2c_driver);
    MODULE_DESCRIPTION("V4L2 driver for TW9910 video decoder");
    MODULE_AUTHOR("Kuninori Morimoto");
    MODULE_LICENSE("GPL v2");
