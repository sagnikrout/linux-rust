//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/gspca/etoms.c
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
// Etoms Et61x151 GPL Linux driver by Michel Xhaard (09/09/2004)
//
// V4L2 by Jean-Francois Moine <http://moinejf.free.fr>
//

    MODULE_AUTHOR("Michel Xhaard <mxhaard@users.sourceforge.net>");
    MODULE_DESCRIPTION("Etoms USB Camera Driver");
    MODULE_LICENSE("GPL");
// specific webcam descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub /: *mut *mut gspca_dev gspca_dev; / !! must be the first item,
    pub autogain: c_uchar,
    pub sensor: c_char,
pub const SENSOR_PAS106: c_int = 0;
pub const SENSOR_TAS5130CXX: c_int = 1;
    pub ag_cnt: signed char,
pub const AG_CNT_START: c_int = 13;
}

    static const struct v4l2_pix_format vga_mode[] = {
    {320, 240, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 320,
    .sizeimage = 320 * 240,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 1},
// {640, 480, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 640,
    .sizeimage = 640 * 480,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 0}, */
    };
    static const struct v4l2_pix_format sif_mode[] = {
    {176, 144, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 176,
    .sizeimage = 176 * 144,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 1},
    {352, 288, V4L2_PIX_FMT_SBGGR8, V4L2_FIELD_NONE,
    .bytesperline = 352,
    .sizeimage = 352 * 288,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 0},
    };
pub const ETOMS_ALT_SIZE_1000: c_int = 12;
pub const ET_GPIO_DIR_CTRL: c_uint = 0x04	/* Control IO bit[0..5] (0 in  1 out) */;
pub const ET_GPIO_OUT: c_uint = 0x05	/* Only IO data */;
pub const ET_GPIO_IN: c_uint = 0x06		/* Read Only IO data */;
pub const ET_RESET_ALL: c_uint = 0x03;
pub const ET_ClCK: c_uint = 0x01;
pub const ET_CTRL: c_uint = 0x02		/* enable i2c OutClck Powerdown mode */;
pub const ET_COMP: c_uint = 0x12		/* Compression register */;
pub const ET_MAXQt: c_uint = 0x13;
pub const ET_MINQt: c_uint = 0x14;
pub const ET_COMP_VAL0: c_uint = 0x02;
pub const ET_COMP_VAL1: c_uint = 0x03;
pub const ET_REG1d: c_uint = 0x1d;
pub const ET_REG1e: c_uint = 0x1e;
pub const ET_REG1f: c_uint = 0x1f;
pub const ET_REG20: c_uint = 0x20;
pub const ET_REG21: c_uint = 0x21;
pub const ET_REG22: c_uint = 0x22;
pub const ET_REG23: c_uint = 0x23;
pub const ET_REG24: c_uint = 0x24;
pub const ET_REG25: c_uint = 0x25;
// base registers for luma calculation
pub const ET_LUMA_CENTER: c_uint = 0x39;
pub const ET_G_RED: c_uint = 0x4d;
pub const ET_G_GREEN1: c_uint = 0x4e;
pub const ET_G_BLUE: c_uint = 0x4f;
pub const ET_G_GREEN2: c_uint = 0x50;
pub const ET_G_GR_H: c_uint = 0x51;
pub const ET_G_GB_H: c_uint = 0x52;
pub const ET_O_RED: c_uint = 0x34;
pub const ET_O_GREEN1: c_uint = 0x35;
pub const ET_O_BLUE: c_uint = 0x36;
pub const ET_O_GREEN2: c_uint = 0x37;
pub const ET_SYNCHRO: c_uint = 0x68;
pub const ET_STARTX: c_uint = 0x69;
pub const ET_STARTY: c_uint = 0x6a;
pub const ET_WIDTH_LOW: c_uint = 0x6b;
pub const ET_HEIGTH_LOW: c_uint = 0x6c;
pub const ET_W_H_HEIGTH: c_uint = 0x6d;
pub const ET_REG6e: c_uint = 0x6e		/* OBW */;
pub const ET_REG6f: c_uint = 0x6f		/* OBW */;
pub const ET_REG70: c_uint = 0x70		/* OBW_AWB */;
pub const ET_REG71: c_uint = 0x71		/* OBW_AWB */;
pub const ET_REG72: c_uint = 0x72		/* OBW_AWB */;
pub const ET_REG73: c_uint = 0x73		/* Clkdelay ns */;
pub const ET_REG74: c_uint = 0x74		/* test pattern */;
pub const ET_REG75: c_uint = 0x75		/* test pattern */;
pub const ET_I2C_CLK: c_uint = 0x8c;
pub const ET_PXL_CLK: c_uint = 0x60;
pub const ET_I2C_BASE: c_uint = 0x89;
pub const ET_I2C_COUNT: c_uint = 0x8a;
pub const ET_I2C_PREFETCH: c_uint = 0x8b;
pub const ET_I2C_REG: c_uint = 0x88;
pub const ET_I2C_DATA7: c_uint = 0x87;
pub const ET_I2C_DATA6: c_uint = 0x86;
pub const ET_I2C_DATA5: c_uint = 0x85;
pub const ET_I2C_DATA4: c_uint = 0x84;
pub const ET_I2C_DATA3: c_uint = 0x83;
pub const ET_I2C_DATA2: c_uint = 0x82;
pub const ET_I2C_DATA1: c_uint = 0x81;
pub const ET_I2C_DATA0: c_uint = 0x80;
pub const PAS106_REG2: c_uint = 0x02	/* pxlClk = systemClk/(reg2) */;
pub const PAS106_REG3: c_uint = 0x03	/* line/frame H [11..4] */;
pub const PAS106_REG4: c_uint = 0x04	/* line/frame L [3..0] */;
pub const PAS106_REG5: c_uint = 0x05	/* exposure time line offset(default 5) */;
pub const PAS106_REG6: c_uint = 0x06	/* exposure time pixel offset(default 6) */;
pub const PAS106_REG7: c_uint = 0x07	/* signbit Dac (default 0) */;
pub const PAS106_REG9: c_uint = 0x09;
pub const PAS106_REG0e: c_uint = 0x0e	/* global gain [4..0](default 0x0e) */;
pub const PAS106_REG13: c_uint = 0x13	/* end i2c write */;
    static const __u8 GainRGBG[] = { 0x80, 0x80, 0x80, 0x80, 0x00, 0x00 };
    static const __u8 I2c2[] = { 0x08, 0x08, 0x08, 0x08, 0x0d };
    static const __u8 I2c3[] = { 0x12, 0x05 };
    static const __u8 I2c4[] = { 0x41, 0x08 };
// read 'len' bytes to gspca_dev->usb_buf
    static void reg_r(struct gspca_dev *gspca_dev,
    __u16 index,
    __u16 len)
    {
    struct usb_device *dev = gspca_dev.dev;
    if (len > USB_BUF_SZ) {
    gspca_err(gspca_dev, "reg_r: buffer overflow\n");
    return;
    }
    usb_control_msg(dev,
    usb_rcvctrlpipe(dev, 0),
    0,
    USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_INTERFACE,
    0,
    index, gspca_dev.usb_buf, len, 500);
    gspca_dbg(gspca_dev, D_USBI, "reg read [%02x] . %02x ..\n",
    index, gspca_dev.usb_buf[0]);
    }
    static void reg_w_val(struct gspca_dev *gspca_dev,
    __u16 index,
    __u8 val)
    {
    struct usb_device *dev = gspca_dev.dev;
    gspca_dev.usb_buf[0] = val;
    usb_control_msg(dev,
    usb_sndctrlpipe(dev, 0),
    0,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_INTERFACE,
    0,
    index, gspca_dev.usb_buf, 1, 500);
    }
    static void reg_w(struct gspca_dev *gspca_dev,
    __u16 index,
    const __u8 *buffer,
    __u16 len)
    {
    struct usb_device *dev = gspca_dev.dev;
    if (len > USB_BUF_SZ) {
    pr_err("reg_w: buffer overflow\n");
    return;
    }
    gspca_dbg(gspca_dev, D_USBO, "reg write [%02x] = %02x..\n",
    index, *buffer);
    memcpy(gspca_dev.usb_buf, buffer, len);
    usb_control_msg(dev,
    usb_sndctrlpipe(dev, 0),
    0,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_INTERFACE,
    0, index, gspca_dev.usb_buf, len, 500);
    }
    static int i2c_w(struct gspca_dev *gspca_dev,
    __u8 reg,
    const __u8 *buffer,
    int len, __u8 mode)
    {
// buffer should be [D0..D7]
    __u8 ptchcount;
// set the base address
    reg_w_val(gspca_dev, ET_I2C_BASE, 0x40);
// sensor base for the pas106
// set count and prefetch
    ptchcount = ((len & 0x07) << 4) | (mode & 0x03);
    reg_w_val(gspca_dev, ET_I2C_COUNT, ptchcount);
// set the register base
    reg_w_val(gspca_dev, ET_I2C_REG, reg);
    while (--len >= 0)
    reg_w_val(gspca_dev, ET_I2C_DATA0 + len, buffer[len]);
    return 0;
    }
    static int i2c_r(struct gspca_dev *gspca_dev,
    __u8 reg)
    {
// set the base address
    reg_w_val(gspca_dev, ET_I2C_BASE, 0x40);
// sensor base for the pas106
// set count and prefetch (cnd: 4 bits - mode: 4 bits)
    reg_w_val(gspca_dev, ET_I2C_COUNT, 0x11);
    reg_w_val(gspca_dev, ET_I2C_REG, reg);	/* set the register base */
    reg_w_val(gspca_dev, ET_I2C_PREFETCH, 0x02);	/* prefetch */
    reg_w_val(gspca_dev, ET_I2C_PREFETCH, 0x00);
    reg_r(gspca_dev, ET_I2C_DATA0, 1);	/* read one byte */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn Et_WaitStatus(gspca_dev: *mut gspca_dev) -> c_int {
    static int Et_WaitStatus(struct gspca_dev *gspca_dev)
    {
    let mut retry: c_int = 10;
    while (retry--) {
    reg_r(gspca_dev, ET_ClCK, 1);
    if (gspca_dev.usb_buf[0] != 0)
    return 1;
    }
    return 0;
    }
    static int et_video(struct gspca_dev *gspca_dev,
    int on)
    {
    int ret;
    reg_w_val(gspca_dev, ET_GPIO_OUT,
    on ? 0x10		/* startvideo - set Bit5 */
    : 0);		/* stopvideo */
    ret = Et_WaitStatus(gspca_dev);
    if (ret != 0)
    gspca_err(gspca_dev, "timeout video on/off\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn Et_init2(gspca_dev: *mut gspca_dev) {
    static void Et_init2(struct gspca_dev *gspca_dev)
    {
    __u8 value;
    static const __u8 FormLine[] = { 0x84, 0x03, 0x14, 0xf4, 0x01, 0x05 };
    gspca_dbg(gspca_dev, D_STREAM, "Open Init2 ET\n");
    reg_w_val(gspca_dev, ET_GPIO_DIR_CTRL, 0x2f);
    reg_w_val(gspca_dev, ET_GPIO_OUT, 0x10);
    reg_r(gspca_dev, ET_GPIO_IN, 1);
    reg_w_val(gspca_dev, ET_ClCK, 0x14); /* 0x14 // 0x16 enabled pattern */
    reg_w_val(gspca_dev, ET_CTRL, 0x1b);
// compression et subsampling
    if (gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv)
    value = ET_COMP_VAL1;	/* 320 */
    else
    value = ET_COMP_VAL0;	/* 640 */
    reg_w_val(gspca_dev, ET_COMP, value);
    reg_w_val(gspca_dev, ET_MAXQt, 0x1f);
    reg_w_val(gspca_dev, ET_MINQt, 0x04);
// undocumented registers
    reg_w_val(gspca_dev, ET_REG1d, 0xff);
    reg_w_val(gspca_dev, ET_REG1e, 0xff);
    reg_w_val(gspca_dev, ET_REG1f, 0xff);
    reg_w_val(gspca_dev, ET_REG20, 0x35);
    reg_w_val(gspca_dev, ET_REG21, 0x01);
    reg_w_val(gspca_dev, ET_REG22, 0x00);
    reg_w_val(gspca_dev, ET_REG23, 0xff);
    reg_w_val(gspca_dev, ET_REG24, 0xff);
    reg_w_val(gspca_dev, ET_REG25, 0x0f);
// colors setting
    reg_w_val(gspca_dev, 0x30, 0x11);		/* 0x30 */
    reg_w_val(gspca_dev, 0x31, 0x40);
    reg_w_val(gspca_dev, 0x32, 0x00);
    reg_w_val(gspca_dev, ET_O_RED, 0x00);		/* 0x34 */
    reg_w_val(gspca_dev, ET_O_GREEN1, 0x00);
    reg_w_val(gspca_dev, ET_O_BLUE, 0x00);
    reg_w_val(gspca_dev, ET_O_GREEN2, 0x00);
//
    reg_w_val(gspca_dev, ET_G_RED, 0x80);		/* 0x4d */
    reg_w_val(gspca_dev, ET_G_GREEN1, 0x80);
    reg_w_val(gspca_dev, ET_G_BLUE, 0x80);
    reg_w_val(gspca_dev, ET_G_GREEN2, 0x80);
    reg_w_val(gspca_dev, ET_G_GR_H, 0x00);
    reg_w_val(gspca_dev, ET_G_GB_H, 0x00);		/* 0x52 */
// Window control registers
    reg_w_val(gspca_dev, 0x61, 0x80);		/* use cmc_out */
    reg_w_val(gspca_dev, 0x62, 0x02);
    reg_w_val(gspca_dev, 0x63, 0x03);
    reg_w_val(gspca_dev, 0x64, 0x14);
    reg_w_val(gspca_dev, 0x65, 0x0e);
    reg_w_val(gspca_dev, 0x66, 0x02);
    reg_w_val(gspca_dev, 0x67, 0x02);
//
    reg_w_val(gspca_dev, ET_SYNCHRO, 0x8f);		/* 0x68 */
    reg_w_val(gspca_dev, ET_STARTX, 0x69);		/* 0x6a //0x69 */
    reg_w_val(gspca_dev, ET_STARTY, 0x0d);		/* 0x0d //0x0c */
    reg_w_val(gspca_dev, ET_WIDTH_LOW, 0x80);
    reg_w_val(gspca_dev, ET_HEIGTH_LOW, 0xe0);
    reg_w_val(gspca_dev, ET_W_H_HEIGTH, 0x60);	/* 6d */
    reg_w_val(gspca_dev, ET_REG6e, 0x86);
    reg_w_val(gspca_dev, ET_REG6f, 0x01);
    reg_w_val(gspca_dev, ET_REG70, 0x26);
    reg_w_val(gspca_dev, ET_REG71, 0x7a);
    reg_w_val(gspca_dev, ET_REG72, 0x01);
// Clock Pattern registers *****************
    reg_w_val(gspca_dev, ET_REG73, 0x00);
    reg_w_val(gspca_dev, ET_REG74, 0x18);		/* 0x28 */
    reg_w_val(gspca_dev, ET_REG75, 0x0f);		/* 0x01 */
//
    reg_w_val(gspca_dev, 0x8a, 0x20);
    reg_w_val(gspca_dev, 0x8d, 0x0f);
    reg_w_val(gspca_dev, 0x8e, 0x08);
//
    reg_w_val(gspca_dev, 0x03, 0x08);
    reg_w_val(gspca_dev, ET_PXL_CLK, 0x03);
    reg_w_val(gspca_dev, 0x81, 0xff);
    reg_w_val(gspca_dev, 0x80, 0x00);
    reg_w_val(gspca_dev, 0x81, 0xff);
    reg_w_val(gspca_dev, 0x80, 0x20);
    reg_w_val(gspca_dev, 0x03, 0x01);
    reg_w_val(gspca_dev, 0x03, 0x00);
    reg_w_val(gspca_dev, 0x03, 0x08);
//
// reg_r(gspca_dev, ET_I2C_BASE, 1);
    always 0x40 as the pas106 ??? */
// set the sensor
    if (gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv)
    value = 0x04;		/* 320 */
    else				/* 640 */
    value = 0x1e;	/* 0x17	 * setting PixelClock
// 0x03 mean 24/(3+1) = 6 Mhz
// 0x05 -> 24/(5+1) = 4 Mhz
// 0x0b -> 24/(11+1) = 2 Mhz
// 0x17 -> 24/(23+1) = 1 Mhz
//
    reg_w_val(gspca_dev, ET_PXL_CLK, value);
// now set by fifo the FormatLine setting
    reg_w(gspca_dev, 0x62, FormLine, 6);
// set exposure times [ 0..0x78] 0->longvalue 0x78->shortvalue
    reg_w_val(gspca_dev, 0x81, 0x47);	/* 0x47; */
    reg_w_val(gspca_dev, 0x80, 0x40);	/* 0x40; */
// Pedro change
// Brightness change Brith+ decrease value
// Brigth- increase value
// original value = 0x70;
    reg_w_val(gspca_dev, 0x81, 0x30);	/* 0x20; - set brightness */
    reg_w_val(gspca_dev, 0x80, 0x20);	/* 0x20; */
    }
#[no_mangle]
unsafe extern "C" fn setbrightness(gspca_dev: *mut gspca_dev, val: i32) {
    static void setbrightness(struct gspca_dev *gspca_dev, s32 val)
    {
    int i;
    for (i = 0; i < 4; i++)
    reg_w_val(gspca_dev, ET_O_RED + i, val);
    }
#[no_mangle]
unsafe extern "C" fn setcontrast(gspca_dev: *mut gspca_dev, val: i32) {
    static void setcontrast(struct gspca_dev *gspca_dev, s32 val)
    {
    __u8 RGBG[] = { 0x80, 0x80, 0x80, 0x80, 0x00, 0x00 };
    memset(RGBG, val, sizeof(RGBG) - 2);
    reg_w(gspca_dev, ET_G_RED, RGBG, 6);
    }
#[no_mangle]
unsafe extern "C" fn setcolors(gspca_dev: *mut gspca_dev, val: i32) {
    static void setcolors(struct gspca_dev *gspca_dev, s32 val)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    __u8 I2cc[] = { 0x05, 0x02, 0x02, 0x05, 0x0d };
    let mut i2cflags: __u8 = 0x01;
// __u8 green = 0;
    I2cc[3] = val;	/* red */
    I2cc[0] = 15 - val;	/* blue */
// green = 15 - ((((7*I2cc[0]) >> 2 ) + I2cc[3]) >> 1);
// I2cc[1] = I2cc[2] = green;
    if (sd.sensor == SENSOR_PAS106) {
    i2c_w(gspca_dev, PAS106_REG13, &i2cflags, 1, 3);
    i2c_w(gspca_dev, PAS106_REG9, I2cc, sizeof I2cc, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn getcolors(gspca_dev: *mut gspca_dev) -> i32 {
    static s32 getcolors(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.sensor == SENSOR_PAS106) {
// i2c_r(gspca_dev, PAS106_REG9);		 * blue
    i2c_r(gspca_dev, PAS106_REG9 + 3);	/* red */
    return gspca_dev.usb_buf[0] & 0x0f;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setautogain(gspca_dev: *mut gspca_dev) {
    static void setautogain(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.autogain)
    sd.ag_cnt = AG_CNT_START;
    else
    sd.ag_cnt = -1;
    }
#[no_mangle]
unsafe extern "C" fn Et_init1(gspca_dev: *mut gspca_dev) {
    static void Et_init1(struct gspca_dev *gspca_dev)
    {
    __u8 value;
// __u8 I2c0 [] = {0x0a, 0x12, 0x05, 0x22, 0xac, 0x00, 0x01, 0x00};
    __u8 I2c0[] = { 0x0a, 0x12, 0x05, 0x6d, 0xcd, 0x00, 0x01, 0x00 };
// try 1/120 0x6d 0xcd 0x40
// __u8 I2c0 [] = {0x0a, 0x12, 0x05, 0xfe, 0xfe, 0xc0, 0x01, 0x00};
// 1/60000 hmm ??
    gspca_dbg(gspca_dev, D_STREAM, "Open Init1 ET\n\n");
    reg_w_val(gspca_dev, ET_GPIO_DIR_CTRL, 7);
    reg_r(gspca_dev, ET_GPIO_IN, 1);
    reg_w_val(gspca_dev, ET_RESET_ALL, 1);
    reg_w_val(gspca_dev, ET_RESET_ALL, 0);
    reg_w_val(gspca_dev, ET_ClCK, 0x10);
    reg_w_val(gspca_dev, ET_CTRL, 0x19);
// compression et subsampling
    if (gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv)
    value = ET_COMP_VAL1;
    else
    value = ET_COMP_VAL0;
    gspca_dbg(gspca_dev, D_STREAM, "Open mode %d Compression %d\n",
    gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv,
    value);
    reg_w_val(gspca_dev, ET_COMP, value);
    reg_w_val(gspca_dev, ET_MAXQt, 0x1d);
    reg_w_val(gspca_dev, ET_MINQt, 0x02);
// undocumented registers
    reg_w_val(gspca_dev, ET_REG1d, 0xff);
    reg_w_val(gspca_dev, ET_REG1e, 0xff);
    reg_w_val(gspca_dev, ET_REG1f, 0xff);
    reg_w_val(gspca_dev, ET_REG20, 0x35);
    reg_w_val(gspca_dev, ET_REG21, 0x01);
    reg_w_val(gspca_dev, ET_REG22, 0x00);
    reg_w_val(gspca_dev, ET_REG23, 0xf7);
    reg_w_val(gspca_dev, ET_REG24, 0xff);
    reg_w_val(gspca_dev, ET_REG25, 0x07);
// colors setting
    reg_w_val(gspca_dev, ET_G_RED, 0x80);
    reg_w_val(gspca_dev, ET_G_GREEN1, 0x80);
    reg_w_val(gspca_dev, ET_G_BLUE, 0x80);
    reg_w_val(gspca_dev, ET_G_GREEN2, 0x80);
    reg_w_val(gspca_dev, ET_G_GR_H, 0x00);
    reg_w_val(gspca_dev, ET_G_GB_H, 0x00);
// Window control registers
    reg_w_val(gspca_dev, ET_SYNCHRO, 0xf0);
    reg_w_val(gspca_dev, ET_STARTX, 0x56);		/* 0x56 */
    reg_w_val(gspca_dev, ET_STARTY, 0x05);		/* 0x04 */
    reg_w_val(gspca_dev, ET_WIDTH_LOW, 0x60);
    reg_w_val(gspca_dev, ET_HEIGTH_LOW, 0x20);
    reg_w_val(gspca_dev, ET_W_H_HEIGTH, 0x50);
    reg_w_val(gspca_dev, ET_REG6e, 0x86);
    reg_w_val(gspca_dev, ET_REG6f, 0x01);
    reg_w_val(gspca_dev, ET_REG70, 0x86);
    reg_w_val(gspca_dev, ET_REG71, 0x14);
    reg_w_val(gspca_dev, ET_REG72, 0x00);
// Clock Pattern registers
    reg_w_val(gspca_dev, ET_REG73, 0x00);
    reg_w_val(gspca_dev, ET_REG74, 0x00);
    reg_w_val(gspca_dev, ET_REG75, 0x0a);
    reg_w_val(gspca_dev, ET_I2C_CLK, 0x04);
    reg_w_val(gspca_dev, ET_PXL_CLK, 0x01);
// set the sensor
    if (gspca_dev.cam.cam_mode[(int) gspca_dev.curr_mode].priv) {
    I2c0[0] = 0x06;
    i2c_w(gspca_dev, PAS106_REG2, I2c0, sizeof I2c0, 1);
    i2c_w(gspca_dev, PAS106_REG9, I2c2, sizeof I2c2, 1);
    value = 0x06;
    i2c_w(gspca_dev, PAS106_REG2, &value, 1, 1);
    i2c_w(gspca_dev, PAS106_REG3, I2c3, sizeof I2c3, 1);
// value = 0x1f;
    value = 0x04;
    i2c_w(gspca_dev, PAS106_REG0e, &value, 1, 1);
    } else {
    I2c0[0] = 0x0a;
    i2c_w(gspca_dev, PAS106_REG2, I2c0, sizeof I2c0, 1);
    i2c_w(gspca_dev, PAS106_REG9, I2c2, sizeof I2c2, 1);
    value = 0x0a;
    i2c_w(gspca_dev, PAS106_REG2, &value, 1, 1);
    i2c_w(gspca_dev, PAS106_REG3, I2c3, sizeof I2c3, 1);
    value = 0x04;
// value = 0x10;
    i2c_w(gspca_dev, PAS106_REG0e, &value, 1, 1);
// bit 2 enable bit 1:2 select 0 1 2 3
    value = 0x07;                                * curve 0 *
    i2c_w(gspca_dev, PAS106_REG0f, &value, 1, 1);
//
    }
// value = 0x01;
// value = 0x22;
// i2c_w(gspca_dev, PAS106_REG5, &value, 1, 1);
// magnetude and sign bit for DAC
    i2c_w(gspca_dev, PAS106_REG7, I2c4, sizeof I2c4, 1);
// now set by fifo the whole colors setting
    reg_w(gspca_dev, ET_G_RED, GainRGBG, 6);
    setcolors(gspca_dev, getcolors(gspca_dev));
    }
// this function is called at probe time
    static int sd_config(struct gspca_dev *gspca_dev,
    const struct usb_device_id *id)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    struct cam *cam;
    cam = &gspca_dev.cam;
    sd.sensor = id.driver_info;
    if (sd.sensor == SENSOR_PAS106) {
    cam.cam_mode = sif_mode;
    cam.nmodes = ARRAY_SIZE(sif_mode);
    } else {
    cam.cam_mode = vga_mode;
    cam.nmodes = ARRAY_SIZE(vga_mode);
    }
    sd.ag_cnt = -1;
    return 0;
    }
// this function is called at probe and resume time
#[no_mangle]
unsafe extern "C" fn sd_init(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.sensor == SENSOR_PAS106)
    Et_init1(gspca_dev);
    else
    Et_init2(gspca_dev);
    reg_w_val(gspca_dev, ET_RESET_ALL, 0x08);
    et_video(gspca_dev, 0);		/* video off */
    return 0;
    }
// -- start the camera --
#[no_mangle]
unsafe extern "C" fn sd_start(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_start(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.sensor == SENSOR_PAS106)
    Et_init1(gspca_dev);
    else
    Et_init2(gspca_dev);
    setautogain(gspca_dev);
    reg_w_val(gspca_dev, ET_RESET_ALL, 0x08);
    et_video(gspca_dev, 1);		/* video on */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sd_stopN(gspca_dev: *mut gspca_dev) {
    static void sd_stopN(struct gspca_dev *gspca_dev)
    {
    et_video(gspca_dev, 0);		/* video off */
    }
#[no_mangle]
unsafe extern "C" fn Et_getgainG(gspca_dev: *mut gspca_dev) -> __u8 {
    static __u8 Et_getgainG(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.sensor == SENSOR_PAS106) {
    i2c_r(gspca_dev, PAS106_REG0e);
    gspca_dbg(gspca_dev, D_CONF, "Etoms gain G %d\n",
    gspca_dev.usb_buf[0]);
    return gspca_dev.usb_buf[0];
    }
    return 0x1f;
    }
#[no_mangle]
unsafe extern "C" fn Et_setgainG(gspca_dev: *mut gspca_dev, gain: __u8) {
    static void Et_setgainG(struct gspca_dev *gspca_dev, __u8 gain)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.sensor == SENSOR_PAS106) {
    let mut i2cflags: __u8 = 0x01;
    i2c_w(gspca_dev, PAS106_REG13, &i2cflags, 1, 3);
    i2c_w(gspca_dev, PAS106_REG0e, &gain, 1, 1);
    }
    }

    (u8)((bright > 0x1f) ? 0x1f : ((bright < 4) ? 3 : bright))

    (u8)((color > 0xff) ? 0xff : ((color < 0) ? 0 : color))
#[no_mangle]
unsafe extern "C" fn do_autogain(gspca_dev: *mut gspca_dev) {
    static void do_autogain(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    __u8 luma;
    let mut luma_mean: __u8 = 128;
    let mut luma_delta: __u8 = 20;
    let mut spring: __u8 = 4;
    int Gbright;
    __u8 r, g, b;
    if (sd.ag_cnt < 0)
    return;
    if (--sd.ag_cnt >= 0)
    return;
    sd.ag_cnt = AG_CNT_START;
    Gbright = Et_getgainG(gspca_dev);
    reg_r(gspca_dev, ET_LUMA_CENTER, 4);
    g = (gspca_dev.usb_buf[0] + gspca_dev.usb_buf[3]) >> 1;
    r = gspca_dev.usb_buf[1];
    b = gspca_dev.usb_buf[2];
    r = ((r << 8) - (r << 4) - (r << 3)) >> 10;
    b = ((b << 7) >> 10);
    g = ((g << 9) + (g << 7) + (g << 5)) >> 10;
    luma = LIMIT(r + g + b);
    gspca_dbg(gspca_dev, D_FRAM, "Etoms luma G %d\n", luma);
    if (luma < luma_mean - luma_delta || luma > luma_mean + luma_delta) {
    Gbright += (luma_mean - luma) >> spring;
    Gbright = BLIMIT(Gbright);
    gspca_dbg(gspca_dev, D_FRAM, "Etoms Gbright %d\n", Gbright);
    Et_setgainG(gspca_dev, (__u8) Gbright);
    }
    }

    static void sd_pkt_scan(struct gspca_dev *gspca_dev,
    u8 *data,			/* isoc packet */
    int len)			/* iso packet length */
    {
    int seqframe;
    seqframe = data[0] & 0x3f;
    len = (int) (((data[0] & 0xc0) << 2) | data[1]);
    if (seqframe == 0x3f) {
    gspca_dbg(gspca_dev, D_FRAM,
    "header packet found datalength %d !!\n", len);
    gspca_dbg(gspca_dev, D_FRAM, "G %d R %d G %d B %d",
    data[2], data[3], data[4], data[5]);
    data += 30;
// don't change datalength as the chips provided it
    gspca_frame_add(gspca_dev, LAST_PACKET, core::ptr::null_mut(), 0);
    gspca_frame_add(gspca_dev, FIRST_PACKET, data, len);
    return;
    }
    if (len) {
    data += 8;
    gspca_frame_add(gspca_dev, INTER_PACKET, data, len);
    } else {			/* Drop Packet */
    gspca_dev.last_packet_type = DISCARD_PACKET;
    }
    }
#[no_mangle]
unsafe extern "C" fn sd_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int sd_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct gspca_dev *gspca_dev =
    container_of(ctrl.handler, struct gspca_dev, ctrl_handler);
    struct sd *sd = (struct sd *)gspca_dev;
    gspca_dev.usb_err = 0;
    if (!gspca_dev.streaming)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    setbrightness(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_CONTRAST:
    setcontrast(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_SATURATION:
    setcolors(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_AUTOGAIN:
    sd.autogain = ctrl.val;
    setautogain(gspca_dev);
    break;
    }
    return gspca_dev.usb_err;
    }
    static const struct v4l2_ctrl_ops sd_ctrl_ops = {
    .s_ctrl = sd_s_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn sd_init_controls(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init_controls(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *)gspca_dev;
    struct v4l2_ctrl_handler *hdl = &gspca_dev.ctrl_handler;
    gspca_dev.vdev.ctrl_handler = hdl;
    v4l2_ctrl_handler_init(hdl, 4);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_BRIGHTNESS, 1, 127, 1, 63);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_CONTRAST, 0, 255, 1, 127);
    if (sd.sensor == SENSOR_PAS106)
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_SATURATION, 0, 15, 1, 7);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_AUTOGAIN, 0, 1, 1, 1);
    if (hdl.error) {
    pr_err("Could not initialize controls\n");
    return hdl.error;
    }
    return 0;
    }
// sub-driver description
    static const struct sd_desc sd_desc = {
    .name = MODULE_NAME,
    .config = sd_config,
    .init = sd_init,
    .init_controls = sd_init_controls,
    .start = sd_start,
    .stopN = sd_stopN,
    .pkt_scan = sd_pkt_scan,
    .dq_callback = do_autogain,
    };
// -- module initialisation --
    static const struct usb_device_id device_table[] = {
    {USB_DEVICE(0x102c, 0x6151), .driver_info = SENSOR_PAS106},
    {USB_DEVICE(0x102c, 0x6251), .driver_info = SENSOR_TAS5130CXX},
    {}
    };
    MODULE_DEVICE_TABLE(usb, device_table);
// -- device connect --
    static int sd_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return gspca_dev_probe(intf, id, &sd_desc, sizeof(struct sd),
    THIS_MODULE);
    }
    static struct usb_driver sd_driver = {
    .name = MODULE_NAME,
    .id_table = device_table,
    .probe = sd_probe,
    .disconnect = gspca_disconnect,

    .suspend = gspca_suspend,
    .resume = gspca_resume,
    .reset_resume = gspca_resume,

    };
    module_usb_driver(sd_driver);
