//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/gspca/jeilinj.c
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
// Jeilinj subdriver
//
// Supports some Jeilin dual-mode cameras which use bulk transport and
// download raw JPEG data.
//
// Copyright (C) 2009 Theodore Kilgore
//
// Sportscam DV15 support and control settings are
// Copyright (C) 2011 Patrice Chotard
//

    MODULE_AUTHOR("Theodore Kilgore <kilgota@auburn.edu>");
    MODULE_DESCRIPTION("GSPCA/JEILINJ USB Camera Driver");
    MODULE_LICENSE("GPL");
// Default timeouts, in ms
pub const JEILINJ_CMD_TIMEOUT: c_int = 500;
pub const JEILINJ_CMD_DELAY: c_int = 160;
pub const JEILINJ_DATA_TIMEOUT: c_int = 1000;
// Maximum transfer size to use.
pub const JEILINJ_MAX_TRANSFER: c_uint = 0x200;
pub const FRAME_HEADER_LEN: c_uint = 0x10;
pub const FRAME_START: c_uint = 0xFFFFFFFF;
    enum {
    SAKAR_57379,
    SPORTSCAM_DV15,
    };

// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub /: *mut *mut gspca_dev gspca_dev; / !! must be the first item,
    pub blocks_left: c_int,
    pub cap_mode: *const v4l2_pix_format,
    pub freq: *mut v4l2_ctrl,
    pub jpegqual: *mut v4l2_ctrl,
// Driver stuff
    pub type: u8,
    pub /: *mut *mut u8 quality; / image quality,
pub const QUALITY_MIN: c_int = 35;
pub const QUALITY_MAX: c_int = 85;
pub const QUALITY_DEF: c_int = 85;
    pub jpeg_hdr: [u8; JPEG_HDR_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jlj_command {
    pub instruction: [c_uchar; 2],
    pub ack_wanted: c_uchar,
    pub delay: c_uchar,
}

// AFAICT these cameras will only do 320x240.
    static struct v4l2_pix_format jlj_mode[] = {
    { 320, 240, V4L2_PIX_FMT_JPEG, V4L2_FIELD_NONE,
    .bytesperline = 320,
    .sizeimage = 320 * 240,
    .colorspace = V4L2_COLORSPACE_JPEG,
    .priv = 0},
    { 640, 480, V4L2_PIX_FMT_JPEG, V4L2_FIELD_NONE,
    .bytesperline = 640,
    .sizeimage = 640 * 480,
    .colorspace = V4L2_COLORSPACE_JPEG,
    .priv = 0}
    };
//
// cam uses endpoint 0x03 to send commands, 0x84 for read commands,
// and 0x82 for bulk transfer.
//
// All commands are two bytes only
#[no_mangle]
unsafe extern "C" fn jlj_write2(gspca_dev: *mut gspca_dev, command: *mut c_uchar) {
    static void jlj_write2(struct gspca_dev *gspca_dev, unsigned char *command)
    {
    int retval;
    if (gspca_dev.usb_err < 0)
    return;
    memcpy(gspca_dev.usb_buf, command, 2);
    retval = usb_bulk_msg(gspca_dev.dev,
    usb_sndbulkpipe(gspca_dev.dev, 3),
    gspca_dev.usb_buf, 2, core::ptr::null_mut(), 500);
    if (retval < 0) {
    pr_err("command write [%02x] error %d\n",
    gspca_dev.usb_buf[0], retval);
    gspca_dev.usb_err = retval;
    }
    }
// Responses are one byte only
#[no_mangle]
unsafe extern "C" fn jlj_read1(gspca_dev: *mut gspca_dev, response: *mut c_uchar) {
    static void jlj_read1(struct gspca_dev *gspca_dev, unsigned char *response)
    {
    int retval;
    if (gspca_dev.usb_err < 0)
    return;
    retval = usb_bulk_msg(gspca_dev.dev,
    usb_rcvbulkpipe(gspca_dev.dev, 0x84),
    gspca_dev.usb_buf, 1, core::ptr::null_mut(), 500);
// response = gspca_dev->usb_buf[0];
    if (retval < 0) {
    pr_err("read command [%02x] error %d\n",
    gspca_dev.usb_buf[0], retval);
    gspca_dev.usb_err = retval;
    }
    }
#[no_mangle]
unsafe extern "C" fn setfreq(gspca_dev: *mut gspca_dev, val: i32) {
    static void setfreq(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 freq_commands[][2] = {
    {0x71, 0x80},
    {0x70, 0x07}
    };
    freq_commands[0][1] |= val >> 1;
    jlj_write2(gspca_dev, freq_commands[0]);
    jlj_write2(gspca_dev, freq_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn setcamquality(gspca_dev: *mut gspca_dev, val: i32) {
    static void setcamquality(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 quality_commands[][2] = {
    {0x71, 0x1E},
    {0x70, 0x06}
    };
    u8 camquality;
// adapt camera quality from jpeg quality
    camquality = ((QUALITY_MAX - val) * CAMQUALITY_MAX)
    / (QUALITY_MAX - QUALITY_MIN);
    quality_commands[0][1] += camquality;
    jlj_write2(gspca_dev, quality_commands[0]);
    jlj_write2(gspca_dev, quality_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn setautogain(gspca_dev: *mut gspca_dev, val: i32) {
    static void setautogain(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 autogain_commands[][2] = {
    {0x94, 0x02},
    {0xcf, 0x00}
    };
    autogain_commands[1][1] = val << 4;
    jlj_write2(gspca_dev, autogain_commands[0]);
    jlj_write2(gspca_dev, autogain_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn setred(gspca_dev: *mut gspca_dev, val: i32) {
    static void setred(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 setred_commands[][2] = {
    {0x94, 0x02},
    {0xe6, 0x00}
    };
    setred_commands[1][1] = val;
    jlj_write2(gspca_dev, setred_commands[0]);
    jlj_write2(gspca_dev, setred_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn setgreen(gspca_dev: *mut gspca_dev, val: i32) {
    static void setgreen(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 setgreen_commands[][2] = {
    {0x94, 0x02},
    {0xe7, 0x00}
    };
    setgreen_commands[1][1] = val;
    jlj_write2(gspca_dev, setgreen_commands[0]);
    jlj_write2(gspca_dev, setgreen_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn setblue(gspca_dev: *mut gspca_dev, val: i32) {
    static void setblue(struct gspca_dev *gspca_dev, s32 val)
    {
    u8 setblue_commands[][2] = {
    {0x94, 0x02},
    {0xe9, 0x00}
    };
    setblue_commands[1][1] = val;
    jlj_write2(gspca_dev, setblue_commands[0]);
    jlj_write2(gspca_dev, setblue_commands[1]);
    }
#[no_mangle]
unsafe extern "C" fn jlj_start(gspca_dev: *mut gspca_dev) -> c_int {
    static int jlj_start(struct gspca_dev *gspca_dev)
    {
    int i;
    int start_commands_size;
    let mut response: u8 = 0xff;
    struct sd *sd = (struct sd *) gspca_dev;
    struct jlj_command start_commands[] = {
    {{0x71, 0x81}, 0, 0},
    {{0x70, 0x05}, 0, JEILINJ_CMD_DELAY},
    {{0x95, 0x70}, 1, 0},
    {{0x71, 0x81 - gspca_dev.curr_mode}, 0, 0},
    {{0x70, 0x04}, 0, JEILINJ_CMD_DELAY},
    {{0x95, 0x70}, 1, 0},
    {{0x71, 0x00}, 0, 0},   /* start streaming ??*/
    {{0x70, 0x08}, 0, JEILINJ_CMD_DELAY},
    {{0x95, 0x70}, 1, 0},
pub const SPORTSCAM_DV15_CMD_SIZE: c_int = 9;
    {{0x94, 0x02}, 0, 0},
    {{0xde, 0x24}, 0, 0},
    {{0x94, 0x02}, 0, 0},
    {{0xdd, 0xf0}, 0, 0},
    {{0x94, 0x02}, 0, 0},
    {{0xe3, 0x2c}, 0, 0},
    {{0x94, 0x02}, 0, 0},
    {{0xe4, 0x00}, 0, 0},
    {{0x94, 0x02}, 0, 0},
    {{0xe5, 0x00}, 0, 0},
    {{0x94, 0x02}, 0, 0},
    {{0xe6, 0x2c}, 0, 0},
    {{0x94, 0x03}, 0, 0},
    {{0xaa, 0x00}, 0, 0}
    };
    sd.blocks_left = 0;
// Under Windows, USB spy shows that only the 9 first start
// commands are used for SPORTSCAM_DV15 webcam
//
    if (sd.type == SPORTSCAM_DV15)
    start_commands_size = SPORTSCAM_DV15_CMD_SIZE;
    else
    start_commands_size = ARRAY_SIZE(start_commands);
    for (i = 0; i < start_commands_size; i++) {
    jlj_write2(gspca_dev, start_commands[i].instruction);
    if (start_commands[i].delay)
    msleep(start_commands[i].delay);
    if (start_commands[i].ack_wanted)
    jlj_read1(gspca_dev, &response);
    }
    setcamquality(gspca_dev, v4l2_ctrl_g_ctrl(sd.jpegqual));
    msleep(2);
    setfreq(gspca_dev, v4l2_ctrl_g_ctrl(sd.freq));
    if (gspca_dev.usb_err < 0)
    gspca_err(gspca_dev, "Start streaming command failed\n");
    return gspca_dev.usb_err;
    }
    static void sd_pkt_scan(struct gspca_dev *gspca_dev,
    u8 *data, int len)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int packet_type;
    u32 header_marker;
    gspca_dbg(gspca_dev, D_STREAM, "Got %d bytes out of %d for Block 0\n",
    len, JEILINJ_MAX_TRANSFER);
    if (len != JEILINJ_MAX_TRANSFER) {
    gspca_dbg(gspca_dev, D_PACK, "bad length\n");
    goto discard;
    }
// check if it's start of frame
    header_marker = ((u32 *)data)[0];
    if (header_marker == FRAME_START) {
    sd.blocks_left = data[0x0a] - 1;
    gspca_dbg(gspca_dev, D_STREAM, "blocks_left = 0x%x\n",
    sd.blocks_left);
// Start a new frame, and add the JPEG header, first thing
    gspca_frame_add(gspca_dev, FIRST_PACKET,
    sd.jpeg_hdr, JPEG_HDR_SZ);
// Toss line 0 of data block 0, keep the rest.
    gspca_frame_add(gspca_dev, INTER_PACKET,
    data + FRAME_HEADER_LEN,
    JEILINJ_MAX_TRANSFER - FRAME_HEADER_LEN);
    } else if (sd.blocks_left > 0) {
    gspca_dbg(gspca_dev, D_STREAM, "%d blocks remaining for frame\n",
    sd.blocks_left);
    sd.blocks_left -= 1;
    if (sd.blocks_left == 0)
    packet_type = LAST_PACKET;
    else
    packet_type = INTER_PACKET;
    gspca_frame_add(gspca_dev, packet_type,
    data, JEILINJ_MAX_TRANSFER);
    } else
    goto discard;
    return;
    discard:
// Discard data until a new frame starts.
    gspca_dev.last_packet_type = DISCARD_PACKET;
    }
// This function is called at probe time just before sd_init
    static int sd_config(struct gspca_dev *gspca_dev,
    const struct usb_device_id *id)
    {
    struct cam *cam = &gspca_dev.cam;
    struct sd *dev  = (struct sd *) gspca_dev;
    dev.type = id.driver_info;
    dev.quality = QUALITY_DEF;
    cam.cam_mode = jlj_mode;
    cam.nmodes = ARRAY_SIZE(jlj_mode);
    cam.bulk = 1;
    cam.bulk_nurbs = 1;
    cam.bulk_size = JEILINJ_MAX_TRANSFER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sd_stopN(gspca_dev: *mut gspca_dev) {
    static void sd_stopN(struct gspca_dev *gspca_dev)
    {
    int i;
    u8 *buf;
    static u8 stop_commands[][2] = {
    {0x71, 0x00},
    {0x70, 0x09},
    {0x71, 0x80},
    {0x70, 0x05}
    };
    for (;;) {
// get the image remaining blocks
    usb_bulk_msg(gspca_dev.dev,
    gspca_dev.urb[0].pipe,
    gspca_dev.urb[0].transfer_buffer,
    JEILINJ_MAX_TRANSFER, core::ptr::null_mut(),
    JEILINJ_DATA_TIMEOUT);
// search for 0xff 0xd9  (EOF for JPEG)
    i = 0;
    buf = gspca_dev.urb[0].transfer_buffer;
    while ((i < (JEILINJ_MAX_TRANSFER - 1)) &&
    ((buf[i] != 0xff) || (buf[i+1] != 0xd9)))
    i++;
    if (i != (JEILINJ_MAX_TRANSFER - 1))
// last remaining block found
    break;
    }
    for (i = 0; i < ARRAY_SIZE(stop_commands); i++)
    jlj_write2(gspca_dev, stop_commands[i]);
    }
// this function is called at probe and resume time
#[no_mangle]
unsafe extern "C" fn sd_init(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init(struct gspca_dev *gspca_dev)
    {
    return gspca_dev.usb_err;
    }
// Set up for getting frames.
#[no_mangle]
unsafe extern "C" fn sd_start(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_start(struct gspca_dev *gspca_dev)
    {
    struct sd *dev = (struct sd *) gspca_dev;
// create the JPEG header
    jpeg_define(dev.jpeg_hdr, gspca_dev.pixfmt.height,
    gspca_dev.pixfmt.width,
    0x21);          /* JPEG 422 */
    jpeg_set_qual(dev.jpeg_hdr, dev.quality);
    gspca_dbg(gspca_dev, D_STREAM, "Start streaming at %dx%d\n",
    gspca_dev.pixfmt.height, gspca_dev.pixfmt.width);
    jlj_start(gspca_dev);
    return gspca_dev.usb_err;
    }
// Table of supported USB devices
    static const struct usb_device_id device_table[] = {
    {USB_DEVICE(0x0979, 0x0280), .driver_info = SAKAR_57379},
    {USB_DEVICE(0x0979, 0x0270), .driver_info = SPORTSCAM_DV15},
    {}
    };
    MODULE_DEVICE_TABLE(usb, device_table);
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
    case V4L2_CID_POWER_LINE_FREQUENCY:
    setfreq(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_RED_BALANCE:
    setred(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_GAIN:
    setgreen(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_BLUE_BALANCE:
    setblue(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_AUTOGAIN:
    setautogain(gspca_dev, ctrl.val);
    break;
    case V4L2_CID_JPEG_COMPRESSION_QUALITY:
    jpeg_set_qual(sd.jpeg_hdr, ctrl.val);
    setcamquality(gspca_dev, ctrl.val);
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
    static const struct v4l2_ctrl_config custom_autogain = {
    .ops = &sd_ctrl_ops,
    .id = V4L2_CID_AUTOGAIN,
    .type = V4L2_CTRL_TYPE_INTEGER,
    .name = "Automatic Gain (and Exposure)",
    .max = 3,
    .step = 1,
    .def = 0,
    };
    gspca_dev.vdev.ctrl_handler = hdl;
    v4l2_ctrl_handler_init(hdl, 6);
    sd.freq = v4l2_ctrl_new_std_menu(hdl, &sd_ctrl_ops,
    V4L2_CID_POWER_LINE_FREQUENCY,
    V4L2_CID_POWER_LINE_FREQUENCY_60HZ, 1,
    V4L2_CID_POWER_LINE_FREQUENCY_60HZ);
    v4l2_ctrl_new_custom(hdl, &custom_autogain, core::ptr::null_mut());
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_RED_BALANCE, 0, 3, 1, 2);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_GAIN, 0, 3, 1, 2);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_BLUE_BALANCE, 0, 3, 1, 2);
    sd.jpegqual = v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_JPEG_COMPRESSION_QUALITY,
    QUALITY_MIN, QUALITY_MAX, 1, QUALITY_DEF);
    if (hdl.error) {
    pr_err("Could not initialize controls\n");
    return hdl.error;
    }
    return 0;
    }
    static int sd_set_jcomp(struct gspca_dev *gspca_dev,
    const struct v4l2_jpegcompression *jcomp)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    v4l2_ctrl_s_ctrl(sd.jpegqual, jcomp.quality);
    return 0;
    }
    static int sd_get_jcomp(struct gspca_dev *gspca_dev,
    struct v4l2_jpegcompression *jcomp)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    memset(jcomp, 0, sizeof *jcomp);
    jcomp.quality = v4l2_ctrl_g_ctrl(sd.jpegqual);
    jcomp.jpeg_markers = V4L2_JPEG_MARKER_DHT
    | V4L2_JPEG_MARKER_DQT;
    return 0;
    }
// sub-driver description
    static const struct sd_desc sd_desc_sakar_57379 = {
    .name   = MODULE_NAME,
    .config = sd_config,
    .init   = sd_init,
    .start  = sd_start,
    .stopN  = sd_stopN,
    .pkt_scan = sd_pkt_scan,
    };
// sub-driver description
    static const struct sd_desc sd_desc_sportscam_dv15 = {
    .name   = MODULE_NAME,
    .config = sd_config,
    .init   = sd_init,
    .init_controls = sd_init_controls,
    .start  = sd_start,
    .stopN  = sd_stopN,
    .pkt_scan = sd_pkt_scan,
    .get_jcomp = sd_get_jcomp,
    .set_jcomp = sd_set_jcomp,
    };
    static const struct sd_desc *sd_desc[2] = {
    &sd_desc_sakar_57379,
    &sd_desc_sportscam_dv15
    };
// -- device connect --
    static int sd_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    return gspca_dev_probe(intf, id,
    sd_desc[id.driver_info],
    sizeof(struct sd),
    THIS_MODULE);
    }
    static struct usb_driver sd_driver = {
    .name       = MODULE_NAME,
    .id_table   = device_table,
    .probe      = sd_probe,
    .disconnect = gspca_disconnect,

    .suspend = gspca_suspend,
    .resume  = gspca_resume,
    .reset_resume = gspca_resume,

    };
    module_usb_driver(sd_driver);
