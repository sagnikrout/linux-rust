//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/vpx3220.c
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
// vpx3220a, vpx3216b & vpx3214c video decoder driver version 0.0.1
//
// Copyright (C) 2001 Laurent Pinchart <lpinchart@freegates.be>
//

    MODULE_DESCRIPTION("vpx3220a/vpx3216b/vpx3214c video decoder driver");
    MODULE_AUTHOR("Laurent Pinchart");
    MODULE_LICENSE("GPL");
    static int debug;
    module_param(debug, int, 0);
    MODULE_PARM_DESC(debug, "Debug level (0-1)");
pub const VPX_TIMEOUT_COUNT: c_int = 10;
// -----------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpx3220 {
    pub sd: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub reg: [c_uchar; 255],
    pub norm: v4l2_std_id,
    pub input: c_int,
    pub enable: c_int,
}

    static inline struct vpx3220 *to_vpx3220(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct vpx3220, sd);
    }
    static inline struct v4l2_subdev *to_sd(struct v4l2_ctrl *ctrl)
    {
    return &container_of(ctrl.handler, struct vpx3220, hdl).sd;
    }
    static char *inputs[] = { "internal", "composite", "svideo" };
// -----------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn vpx3220_write(sd: *mut v4l2_subdev, reg: u8, value: u8) -> c_int {
    static inline int vpx3220_write(struct v4l2_subdev *sd, u8 reg, u8 value)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct vpx3220 *decoder = i2c_get_clientdata(client);
    decoder.reg[reg] = value;
    return i2c_smbus_write_byte_data(client, reg, value);
    }
#[no_mangle]
pub unsafe extern "C" fn vpx3220_read(sd: *mut v4l2_subdev, reg: u8) -> c_int {
    static inline int vpx3220_read(struct v4l2_subdev *sd, u8 reg)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    return i2c_smbus_read_byte_data(client, reg);
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_fp_status(sd: *mut v4l2_subdev) -> c_int {
    static int vpx3220_fp_status(struct v4l2_subdev *sd)
    {
    unsigned char status;
    unsigned int i;
    for (i = 0; i < VPX_TIMEOUT_COUNT; i++) {
    status = vpx3220_read(sd, 0x29);
    if (!(status & 4))
    return 0;
    udelay(10);
    if (need_resched())
    cond_resched();
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_fp_write(sd: *mut v4l2_subdev, fpaddr: u8, data: u16) -> c_int {
    static int vpx3220_fp_write(struct v4l2_subdev *sd, u8 fpaddr, u16 data)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
// Write the 16-bit address to the FPWR register
    if (i2c_smbus_write_word_data(client, 0x27, swab16(fpaddr)) == -1) {
    v4l2_dbg(1, debug, sd, "%s: failed\n", __func__);
    return -1;
    }
    if (vpx3220_fp_status(sd) < 0)
    return -1;
// Write the 16-bit data to the FPDAT register
    if (i2c_smbus_write_word_data(client, 0x28, swab16(data)) == -1) {
    v4l2_dbg(1, debug, sd, "%s: failed\n", __func__);
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_fp_read(sd: *mut v4l2_subdev, fpaddr: u16) -> c_int {
    static int vpx3220_fp_read(struct v4l2_subdev *sd, u16 fpaddr)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    s16 data;
// Write the 16-bit address to the FPRD register
    if (i2c_smbus_write_word_data(client, 0x26, swab16(fpaddr)) == -1) {
    v4l2_dbg(1, debug, sd, "%s: failed\n", __func__);
    return -1;
    }
    if (vpx3220_fp_status(sd) < 0)
    return -1;
// Read the 16-bit data from the FPDAT register
    data = i2c_smbus_read_word_data(client, 0x28);
    if (data == -1) {
    v4l2_dbg(1, debug, sd, "%s: failed\n", __func__);
    return -1;
    }
    return swab16(data);
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_write_block(sd: *mut v4l2_subdev, data: *const u8, len: c_uint) -> c_int {
    static int vpx3220_write_block(struct v4l2_subdev *sd, const u8 *data, unsigned int len)
    {
    u8 reg;
    let mut ret: c_int = -1;
    while (len >= 2) {
    reg = *data++;
    ret = vpx3220_write(sd, reg, *data++);
    if (ret < 0)
    break;
    len -= 2;
    }
    return ret;
    }
    static int vpx3220_write_fp_block(struct v4l2_subdev *sd,
    const u16 *data, unsigned int len)
    {
    u8 reg;
    let mut ret: c_int = 0;
    while (len > 1) {
    reg = *data++;
    ret |= vpx3220_fp_write(sd, reg, *data++);
    len -= 2;
    }
    return ret;
    }
// ----------------------------------------------------------------------
    static const unsigned short init_ntsc[] = {
    0x1c, 0x00,		/* NTSC tint angle */
    0x88, 17,		/* Window 1 vertical */
    0x89, 240,		/* Vertical lines in */
    0x8a, 240,		/* Vertical lines out */
    0x8b, 000,		/* Horizontal begin */
    0x8c, 640,		/* Horizontal length */
    0x8d, 640,		/* Number of pixels */
    0x8f, 0xc00,		/* Disable window 2 */
    0xf0, 0x73,		/* 13.5 MHz transport, Forced
// mode, latch windows
    0xf2, 0x13,		/* NTSC M, composite input */
    0xe7, 0x1e1,		/* Enable vertical standard
// locking @ 240 lines
    };
    static const unsigned short init_pal[] = {
    0x88, 23,		/* Window 1 vertical begin */
    0x89, 288,		/* Vertical lines in (16 lines
// skipped by the VFE)
    0x8a, 288,		/* Vertical lines out (16 lines
// skipped by the VFE)
    0x8b, 16,		/* Horizontal begin */
    0x8c, 768,		/* Horizontal length */
    0x8d, 784,		/* Number of pixels
// Must be >= Horizontal begin + Horizontal length
    0x8f, 0xc00,		/* Disable window 2 */
    0xf0, 0x77,		/* 13.5 MHz transport, Forced
// mode, latch windows
    0xf2, 0x3d1,		/* PAL B,G,H,I, composite input */
    0xe7, 0x241,		/* PAL/SECAM set to 288 lines */
    };
    static const unsigned short init_secam[] = {
    0x88, 23,		/* Window 1 vertical begin */
    0x89, 288,		/* Vertical lines in (16 lines
// skipped by the VFE)
    0x8a, 288,		/* Vertical lines out (16 lines
// skipped by the VFE)
    0x8b, 16,		/* Horizontal begin */
    0x8c, 768,		/* Horizontal length */
    0x8d, 784,		/* Number of pixels
// Must be >= Horizontal begin + Horizontal length
    0x8f, 0xc00,		/* Disable window 2 */
    0xf0, 0x77,		/* 13.5 MHz transport, Forced
// mode, latch windows
    0xf2, 0x3d5,		/* SECAM, composite input */
    0xe7, 0x241,		/* PAL/SECAM set to 288 lines */
    };
    static const unsigned char init_common[] = {
    0xf2, 0x00,		/* Disable all outputs */
    0x33, 0x0d,		/* Luma : VIN2, Chroma : CIN
// (clamp off)
    0xd8, 0xa8,		/* HREF/VREF active high, VREF
// pulse = 2, Odd/Even flag
    0x20, 0x03,		/* IF compensation 0dB/oct */
    0xe0, 0xff,		/* Open up all comparators */
    0xe1, 0x00,
    0xe2, 0x7f,
    0xe3, 0x80,
    0xe4, 0x7f,
    0xe5, 0x80,
    0xe6, 0x00,		/* Brightness set to 0 */
    0xe7, 0xe0,		/* Contrast to 1.0, noise shaping
// 10 to 8 2-bit error diffusion
    0xe8, 0xf8,		/* YUV422, CbCr binary offset,
// ... (p.32)
    0xea, 0x18,		/* LLC2 connected, output FIFO
// reset with VACTintern
    0xf0, 0x8a,		/* Half full level to 10, bus
// shuffler [7:0, 23:16, 15:8]
    0xf1, 0x18,		/* Single clock, sync mode, no
// FE delay, no HLEN counter
    0xf8, 0x12,		/* Port A, PIXCLK, HF# & FE#
// strength to 2
    0xf9, 0x24,		/* Port B, HREF, VREF, PREF &
// ALPHA strength to 4
    };
    static const unsigned short init_fp[] = {
    0x59, 0,
    0xa0, 2070,		/* ACC reference */
    0xa3, 0,
    0xa4, 0,
    0xa8, 30,
    0xb2, 768,
    0xbe, 27,
    0x58, 0,
    0x26, 0,
    0x4b, 0x298,		/* PLL gain */
    };
#[no_mangle]
unsafe extern "C" fn vpx3220_init(sd: *mut v4l2_subdev, val: u32) -> c_int {
    static int vpx3220_init(struct v4l2_subdev *sd, u32 val)
    {
    struct vpx3220 *decoder = to_vpx3220(sd);
    vpx3220_write_block(sd, init_common, sizeof(init_common));
    vpx3220_write_fp_block(sd, init_fp, sizeof(init_fp) >> 1);
    if (decoder.norm & V4L2_STD_NTSC)
    vpx3220_write_fp_block(sd, init_ntsc, sizeof(init_ntsc) >> 1);
#[no_mangle]
pub unsafe extern "C" fn if(V4L2_STD_PAL: decoder->norm &) -> else {
    else if (decoder.norm & V4L2_STD_PAL)
    vpx3220_write_fp_block(sd, init_pal, sizeof(init_pal) >> 1);
#[no_mangle]
pub unsafe extern "C" fn if(V4L2_STD_SECAM: decoder->norm &) -> else {
    else if (decoder.norm & V4L2_STD_SECAM)
    vpx3220_write_fp_block(sd, init_secam, sizeof(init_secam) >> 1);
    else
    vpx3220_write_fp_block(sd, init_pal, sizeof(init_pal) >> 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_status(sd: *mut v4l2_subdev, pstatus: *mut u32, pstd: *mut v4l2_std_id) -> c_int {
    static int vpx3220_status(struct v4l2_subdev *sd, u32 *pstatus, v4l2_std_id *pstd)
    {
    let mut res: c_int = V4L2_IN_ST_NO_SIGNAL, status;
    let mut std: v4l2_std_id = pstd ? *pstd : V4L2_STD_ALL;
    status = vpx3220_fp_read(sd, 0x0f3);
    v4l2_dbg(1, debug, sd, "status: 0x%04x\n", status);
    if (status < 0)
    return status;
    if ((status & 0x20) == 0) {
    res = 0;
    switch (status & 0x18) {
    case 0x00:
    case 0x10:
    case 0x14:
    case 0x18:
    std &= V4L2_STD_PAL;
    break;
    case 0x08:
    std &= V4L2_STD_SECAM;
    break;
    case 0x04:
    case 0x0c:
    case 0x1c:
    std &= V4L2_STD_NTSC;
    break;
    }
    } else {
    std = V4L2_STD_UNKNOWN;
    }
    if (pstd)
// pstd = std;
    if (pstatus)
// pstatus = res;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_querystd(sd: *mut v4l2_subdev, std: *mut v4l2_std_id) -> c_int {
    static int vpx3220_querystd(struct v4l2_subdev *sd, v4l2_std_id *std)
    {
    v4l2_dbg(1, debug, sd, "querystd\n");
    return vpx3220_status(sd, core::ptr::null_mut(), std);
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_g_input_status(sd: *mut v4l2_subdev, status: *mut u32) -> c_int {
    static int vpx3220_g_input_status(struct v4l2_subdev *sd, u32 *status)
    {
    v4l2_dbg(1, debug, sd, "g_input_status\n");
    return vpx3220_status(sd, status, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_s_std(sd: *mut v4l2_subdev, std: v4l2_std_id) -> c_int {
    static int vpx3220_s_std(struct v4l2_subdev *sd, v4l2_std_id std)
    {
    struct vpx3220 *decoder = to_vpx3220(sd);
    int temp_input;
// Here we back up the input selection because it gets
    overwritten when we fill the registers with the
    chosen video norm */
    temp_input = vpx3220_fp_read(sd, 0xf2);
    v4l2_dbg(1, debug, sd, "s_std %llx\n", (unsigned long long)std);
    if (std & V4L2_STD_NTSC) {
    vpx3220_write_fp_block(sd, init_ntsc, sizeof(init_ntsc) >> 1);
    v4l2_dbg(1, debug, sd, "norm switched to NTSC\n");
    } else if (std & V4L2_STD_PAL) {
    vpx3220_write_fp_block(sd, init_pal, sizeof(init_pal) >> 1);
    v4l2_dbg(1, debug, sd, "norm switched to PAL\n");
    } else if (std & V4L2_STD_SECAM) {
    vpx3220_write_fp_block(sd, init_secam, sizeof(init_secam) >> 1);
    v4l2_dbg(1, debug, sd, "norm switched to SECAM\n");
    } else {
    return -EINVAL;
    }
    decoder.norm = std;
// And here we set the backed up video input again
    vpx3220_fp_write(sd, 0xf2, temp_input | 0x0010);
    udelay(10);
    return 0;
    }
    static int vpx3220_s_routing(struct v4l2_subdev *sd,
    u32 input, u32 output, u32 config)
    {
    int data;
// RJ:   input = 0: ST8 (PCTV) input
    input = 1: COMPOSITE  input
    input = 2: SVHS       input  */
    static const int input_vals[3][2] = {
    {0x0c, 0},
    {0x0d, 0},
    {0x0e, 1}
    };
    if (input > 2)
    return -EINVAL;
    v4l2_dbg(1, debug, sd, "input switched to %s\n", inputs[input]);
    vpx3220_write(sd, 0x33, input_vals[input][0]);
    data = vpx3220_fp_read(sd, 0xf2) & ~(0x0020);
    if (data < 0)
    return data;
// 0x0010 is required to latch the setting
    vpx3220_fp_write(sd, 0xf2,
    data | (input_vals[input][1] << 5) | 0x0010);
    udelay(10);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int vpx3220_s_stream(struct v4l2_subdev *sd, int enable)
    {
    v4l2_dbg(1, debug, sd, "s_stream %s\n", enable ? "on" : "off");
    vpx3220_write(sd, 0xf2, (enable ? 0x1b : 0x00));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int vpx3220_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_sd(ctrl);
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    vpx3220_write(sd, 0xe6, ctrl.val);
    return 0;
    case V4L2_CID_CONTRAST:
// Bit 7 and 8 is for noise shaping
    vpx3220_write(sd, 0xe7, ctrl.val + 192);
    return 0;
    case V4L2_CID_SATURATION:
    vpx3220_fp_write(sd, 0xa0, ctrl.val);
    return 0;
    case V4L2_CID_HUE:
    vpx3220_fp_write(sd, 0x1c, ctrl.val);
    return 0;
    }
    return -EINVAL;
    }
// -----------------------------------------------------------------------
    static const struct v4l2_ctrl_ops vpx3220_ctrl_ops = {
    .s_ctrl = vpx3220_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops vpx3220_core_ops = {
    .init = vpx3220_init,
    };
    static const struct v4l2_subdev_video_ops vpx3220_video_ops = {
    .s_std = vpx3220_s_std,
    .s_routing = vpx3220_s_routing,
    .s_stream = vpx3220_s_stream,
    .querystd = vpx3220_querystd,
    .g_input_status = vpx3220_g_input_status,
    };
    static const struct v4l2_subdev_ops vpx3220_ops = {
    .core = &vpx3220_core_ops,
    .video = &vpx3220_video_ops,
    };
// -----------------------------------------------------------------------
// Client management code
//
#[no_mangle]
unsafe extern "C" fn vpx3220_probe(client: *mut i2c_client) -> c_int {
    static int vpx3220_probe(struct i2c_client *client)
    {
    struct vpx3220 *decoder;
    struct v4l2_subdev *sd;
    const char *name = core::ptr::null_mut();
    u8 ver;
    u16 pn;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    decoder = devm_kzalloc(&client.dev, sizeof(*decoder), GFP_KERNEL);
    if (decoder == core::ptr::null_mut())
    return -ENOMEM;
    sd = &decoder.sd;
    v4l2_i2c_subdev_init(sd, client, &vpx3220_ops);
    decoder.norm = V4L2_STD_PAL;
    decoder.input = 0;
    decoder.enable = 1;
    v4l2_ctrl_handler_init(&decoder.hdl, 4);
    v4l2_ctrl_new_std(&decoder.hdl, &vpx3220_ctrl_ops,
    V4L2_CID_BRIGHTNESS, -128, 127, 1, 0);
    v4l2_ctrl_new_std(&decoder.hdl, &vpx3220_ctrl_ops,
    V4L2_CID_CONTRAST, 0, 63, 1, 32);
    v4l2_ctrl_new_std(&decoder.hdl, &vpx3220_ctrl_ops,
    V4L2_CID_SATURATION, 0, 4095, 1, 2048);
    v4l2_ctrl_new_std(&decoder.hdl, &vpx3220_ctrl_ops,
    V4L2_CID_HUE, -512, 511, 1, 0);
    sd.ctrl_handler = &decoder.hdl;
    if (decoder.hdl.error) {
    let mut err: c_int = decoder.hdl.error;
    v4l2_ctrl_handler_free(&decoder.hdl);
    return err;
    }
    v4l2_ctrl_handler_setup(&decoder.hdl);
    ver = i2c_smbus_read_byte_data(client, 0x00);
    pn = (i2c_smbus_read_byte_data(client, 0x02) << 8) +
    i2c_smbus_read_byte_data(client, 0x01);
    if (ver == 0xec) {
    switch (pn) {
    case 0x4680:
    name = "vpx3220a";
    break;
    case 0x4260:
    name = "vpx3216b";
    break;
    case 0x4280:
    name = "vpx3214c";
    break;
    }
    }
    if (name)
    v4l2_info(sd, "%s found @ 0x%x (%s)\n", name,
    client.addr << 1, client.adapter.name);
    else
    v4l2_info(sd, "chip (%02x:%04x) found @ 0x%x (%s)\n",
    ver, pn, client.addr << 1, client.adapter.name);
    vpx3220_write_block(sd, init_common, sizeof(init_common));
    vpx3220_write_fp_block(sd, init_fp, sizeof(init_fp) >> 1);
// Default to PAL
    vpx3220_write_fp_block(sd, init_pal, sizeof(init_pal) >> 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpx3220_remove(client: *mut i2c_client) {
    static void vpx3220_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct vpx3220 *decoder = to_vpx3220(sd);
    v4l2_device_unregister_subdev(sd);
    v4l2_ctrl_handler_free(&decoder.hdl);
    }
    static const struct i2c_device_id vpx3220_id[] = {
    { .name = "vpx3220a" },
    { .name = "vpx3216b" },
    { .name = "vpx3214c" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, vpx3220_id);
    static struct i2c_driver vpx3220_driver = {
    .driver = {
    .name	= "vpx3220",
    },
    .probe		= vpx3220_probe,
    .remove		= vpx3220_remove,
    .id_table	= vpx3220_id,
    };
    module_i2c_driver(vpx3220_driver);
