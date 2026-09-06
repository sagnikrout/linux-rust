//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/pwc/pwc-ctrl.c
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
// Driver for Philips webcam
    Functions that send various control messages to the webcam, including
    video modes.
    (C) 1999-2003 Nemosoft Unv.
    (C) 2004-2006 Luc Saillard (luc@saillard.org)
    (C) 2011 Hans de Goede <hdegoede@redhat.com>
    NOTE: this version of pwc is an unofficial (modified) release of pwc & pcwx
    driver and thus may have bugs that are not present in the original version.
    Please send bug reports and support requests to <luc@saillard.org>.
    NOTE: this version of pwc is an unofficial (modified) release of pwc & pcwx
    driver and thus may have bugs that are not present in the original version.
    Please send bug reports and support requests to <luc@saillard.org>.
    The decompression routines have been implemented by reverse-engineering the
    Nemosoft binary pwcx module. Caveat emptor.
//
    Changes
    2001/08/03  Alvarado   Added methods for changing white balance and
    red/green gains
//
// Control functions for the cam; brightness, contrast, video mode, etc.

// Selectors for status controls used only in this file
pub const GET_STATUS_B00: c_uint = 0x0B00;
pub const SENSOR_TYPE_FORMATTER1: c_uint = 0x0C00;
pub const GET_STATUS_3000: c_uint = 0x3000;
pub const READ_RAW_Y_MEAN_FORMATTER: c_uint = 0x3100;
pub const SET_POWER_SAVE_MODE_FORMATTER: c_uint = 0x3200;
pub const MIRROR_IMAGE_FORMATTER: c_uint = 0x3300;
pub const LED_FORMATTER: c_uint = 0x3400;
pub const LOWLIGHT: c_uint = 0x3500;
pub const GET_STATUS_3600: c_uint = 0x3600;
pub const SENSOR_TYPE_FORMATTER2: c_uint = 0x3700;
pub const GET_STATUS_3800: c_uint = 0x3800;
pub const GET_STATUS_4000: c_uint = 0x4000;
pub const GET_STATUS_4100: c_uint = 0x4100	/* Get */;
pub const CTL_STATUS_4200: c_uint = 0x4200	/* [GS] 1 */;
// Formatters for the Video Endpoint controls [GS]ET_EP_STREAM_CTL
pub const VIDEO_OUTPUT_CONTROL_FORMATTER: c_uint = 0x0100;
    static const char *size2name[PSZ_MAX] =
    {
    "subQCIF",
    "QSIF",
    "QCIF",
    "SIF",
    "CIF",
    "VGA",
    };
//
// Entries for the Nala (645/646) camera; the Nala doesn't have compression
    preferences, so you either get compressed or non-compressed streams.
    An alternate value of 0 means this mode is not available at all.
//
pub const PWC_FPS_MAX_NALA: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Nala_table_entry {
    pub /: *mut *mut char alternate; / USB alternate setting,
    pub /: *mut *mut int compressed; / Compressed yes/no,
    pub /: *mut *mut unsigned char mode[3]; / precomputed mode table,
}

    static unsigned int Nala_fps_vector[PWC_FPS_MAX_NALA] = { 4, 5, 7, 10, 12, 15, 20, 24 };
    static struct Nala_table_entry Nala_table[PSZ_MAX][PWC_FPS_MAX_NALA] =
    {

    };
//
    static int recv_control_msg(struct pwc_device *pdev,
    u8 request, u16 value, int recv_count)
    {
    int rc;
    rc = usb_control_msg(pdev.udev, usb_rcvctrlpipe(pdev.udev, 0),
    request,
    USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, pdev.vcinterface,
    pdev.ctrl_buf, recv_count, USB_CTRL_GET_TIMEOUT);
    if (rc < 0)
    PWC_ERROR("recv_control_msg error %d req %02x val %04x\n",
    rc, request, value);
    return rc;
    }
    static inline int send_video_command(struct pwc_device *pdev,
    int index, const unsigned char *buf, int buflen)
    {
    int rc;
    memcpy(pdev.ctrl_buf, buf, buflen);
    rc = usb_control_msg(pdev.udev, usb_sndctrlpipe(pdev.udev, 0),
    SET_EP_STREAM_CTL,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    VIDEO_OUTPUT_CONTROL_FORMATTER, index,
    pdev.ctrl_buf, buflen, USB_CTRL_SET_TIMEOUT);
    if (rc >= 0)
    memcpy(pdev.cmd_buf, buf, buflen);
    else
    PWC_ERROR("send_video_command error %d\n", rc);
    return rc;
    }
    int send_control_msg(struct pwc_device *pdev,
    u8 request, u16 value, void *buf, int buflen)
    {
    return usb_control_msg(pdev.udev, usb_sndctrlpipe(pdev.udev, 0),
    request,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, pdev.vcinterface,
    buf, buflen, USB_CTRL_SET_TIMEOUT);
    }
    static int set_video_mode_Nala(struct pwc_device *pdev, int size, int pixfmt,
    int frames, int *compression, int send_to_cam)
    {
    int fps, ret = 0;
    struct Nala_table_entry *pEntry;
    int frames2frames[31] =
    { /* closest match of framerate */
    0,  0,  0,  0,  4,  /*  0-4  */
    5,  5,  7,  7, 10,  /*  5-9  */
    10, 10, 12, 12, 15,  /* 10-14 */
    15, 15, 15, 20, 20,  /* 15-19 */
    20, 20, 20, 24, 24,  /* 20-24 */
    24, 24, 24, 24, 24,  /* 25-29 */
    24                   /* 30    */
    };
    int frames2table[31] =
    { 0, 0, 0, 0, 0, /*  0-4  */
    1, 1, 1, 2, 2, /*  5-9  */
    3, 3, 4, 4, 4, /* 10-14 */
    5, 5, 5, 5, 5, /* 15-19 */
    6, 6, 6, 6, 7, /* 20-24 */
    7, 7, 7, 7, 7, /* 25-29 */
    7              /* 30    */
    };
    if (size < 0 || size > PSZ_CIF)
    return -EINVAL;
    if (frames < 4)
    frames = 4;
#[no_mangle]
pub unsafe extern "C" fn if(15: size > PSZ_QCIF && frames >) -> else {
    else if (size > PSZ_QCIF && frames > 15)
    frames = 15;
#[no_mangle]
pub unsafe extern "C" fn if(25: frames >) -> else {
    else if (frames > 25)
    frames = 25;
    frames = frames2frames[frames];
    fps = frames2table[frames];
    pEntry = &Nala_table[size][fps];
    if (pEntry.alternate == 0)
    return -EINVAL;
    if (send_to_cam)
    ret = send_video_command(pdev, pdev.vendpoint,
    pEntry.mode, 3);
    if (ret < 0)
    return ret;
    if (pEntry.compressed && pixfmt == V4L2_PIX_FMT_YUV420)
    pwc_dec1_init(pdev, pEntry.mode);
// Set various parameters
    pdev.pixfmt = pixfmt;
    pdev.vframes = frames;
    pdev.valternate = pEntry.alternate;
    pdev.width  = pwc_image_sizes[size][0];
    pdev.height = pwc_image_sizes[size][1];
    pdev.frame_size = (pdev.width * pdev.height * 3) / 2;
    if (pEntry.compressed) {
    if (pdev.release < 5) { /* 4 fold compression */
    pdev.vbandlength = 528;
    pdev.frame_size /= 4;
    }
    else {
    pdev.vbandlength = 704;
    pdev.frame_size /= 3;
    }
    }
    else
    pdev.vbandlength = 0;
// Let pwc-if.c:isoc_init know we don't support higher compression
// compression = 3;
    return 0;
    }
    static int set_video_mode_Timon(struct pwc_device *pdev, int size, int pixfmt,
    int frames, int *compression, int send_to_cam)
    {
    const struct Timon_table_entry *pChoose;
    int fps, ret = 0;
    if (size >= PSZ_MAX || *compression < 0 || *compression > 3)
    return -EINVAL;
    if (frames < 5)
    frames = 5;
#[no_mangle]
pub unsafe extern "C" fn if(15: size == PSZ_VGA && frames >) -> else {
    else if (size == PSZ_VGA && frames > 15)
    frames = 15;
#[no_mangle]
pub unsafe extern "C" fn if(30: frames >) -> else {
    else if (frames > 30)
    frames = 30;
    fps = (frames / 5) - 1;
// Find a supported framerate with progressively higher compression
    do {
    pChoose = &Timon_table[size][fps][*compression];
    if (pChoose.alternate != 0)
    break;
    (*compression)++;
    } while (*compression <= 3);
    if (pChoose.alternate == 0)
    return -ENOENT; /* Not supported. */
    if (send_to_cam)
    ret = send_video_command(pdev, pdev.vendpoint,
    pChoose.mode, 13);
    if (ret < 0)
    return ret;
    if (pChoose.bandlength > 0 && pixfmt == V4L2_PIX_FMT_YUV420)
    pwc_dec23_init(pdev, pChoose.mode);
// Set various parameters
    pdev.pixfmt = pixfmt;
    pdev.vframes = (fps + 1) * 5;
    pdev.valternate = pChoose.alternate;
    pdev.width  = pwc_image_sizes[size][0];
    pdev.height = pwc_image_sizes[size][1];
    pdev.vbandlength = pChoose.bandlength;
    if (pChoose.bandlength > 0)
    pdev.frame_size = (pChoose.bandlength * pdev.height) / 4;
    else
    pdev.frame_size = (pdev.width * pdev.height * 12) / 8;
    return 0;
    }
    static int set_video_mode_Kiara(struct pwc_device *pdev, int size, int pixfmt,
    int frames, int *compression, int send_to_cam)
    {
    const struct Kiara_table_entry *pChoose;
    int fps, ret = 0;
    if (size >= PSZ_MAX || *compression < 0 || *compression > 3)
    return -EINVAL;
    if (frames < 5)
    frames = 5;
#[no_mangle]
pub unsafe extern "C" fn if(15: size == PSZ_VGA && frames >) -> else {
    else if (size == PSZ_VGA && frames > 15)
    frames = 15;
#[no_mangle]
pub unsafe extern "C" fn if(30: frames >) -> else {
    else if (frames > 30)
    frames = 30;
    fps = (frames / 5) - 1;
// Find a supported framerate with progressively higher compression
    do {
    pChoose = &Kiara_table[size][fps][*compression];
    if (pChoose.alternate != 0)
    break;
    (*compression)++;
    } while (*compression <= 3);
    if (pChoose.alternate == 0)
    return -ENOENT; /* Not supported. */
// Firmware bug: video endpoint is 5, but commands are sent to endpoint 4
    if (send_to_cam)
    ret = send_video_command(pdev, 4, pChoose.mode, 12);
    if (ret < 0)
    return ret;
    if (pChoose.bandlength > 0 && pixfmt == V4L2_PIX_FMT_YUV420)
    pwc_dec23_init(pdev, pChoose.mode);
// All set and go
    pdev.pixfmt = pixfmt;
    pdev.vframes = (fps + 1) * 5;
    pdev.valternate = pChoose.alternate;
    pdev.width  = pwc_image_sizes[size][0];
    pdev.height = pwc_image_sizes[size][1];
    pdev.vbandlength = pChoose.bandlength;
    if (pdev.vbandlength > 0)
    pdev.frame_size = (pdev.vbandlength * pdev.height) / 4;
    else
    pdev.frame_size = (pdev.width * pdev.height * 12) / 8;
    PWC_TRACE("frame_size=%d, vframes=%d, vsize=%d, vbandlength=%d\n",
    pdev.frame_size, pdev.vframes, size, pdev.vbandlength);
    return 0;
    }
    int pwc_set_video_mode(struct pwc_device *pdev, int width, int height,
    int pixfmt, int frames, int *compression, int send_to_cam)
    {
    int ret, size;
    PWC_DEBUG_FLOW("set_video_mode(%dx%d @ %d, pixfmt %08x).\n",
    width, height, frames, pixfmt);
    size = pwc_get_size(pdev, width, height);
    PWC_TRACE("decode_size = %d.\n", size);
    if (DEVICE_USE_CODEC1(pdev.type)) {
    ret = set_video_mode_Nala(pdev, size, pixfmt, frames,
    compression, send_to_cam);
    } else if (DEVICE_USE_CODEC3(pdev.type)) {
    ret = set_video_mode_Kiara(pdev, size, pixfmt, frames,
    compression, send_to_cam);
    } else {
    ret = set_video_mode_Timon(pdev, size, pixfmt, frames,
    compression, send_to_cam);
    }
    if (ret < 0) {
    PWC_ERROR("Failed to set video mode %s@%d fps; return code = %d\n", size2name[size], frames, ret);
    return ret;
    }
    pdev.frame_total_size = pdev.frame_size + pdev.frame_header_size + pdev.frame_trailer_size;
    PWC_DEBUG_SIZE("Set resolution to %dx%d\n", pdev.width, pdev.height);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwc_get_fps_Nala(pdev: *mut pwc_device, index: c_uint, size: c_uint) -> c_uint {
    static unsigned int pwc_get_fps_Nala(struct pwc_device *pdev, unsigned int index, unsigned int size)
    {
    unsigned int i;
    for (i = 0; i < PWC_FPS_MAX_NALA; i++) {
    if (Nala_table[size][i].alternate) {
    if (index--==0) return Nala_fps_vector[i];
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwc_get_fps_Kiara(pdev: *mut pwc_device, index: c_uint, size: c_uint) -> c_uint {
    static unsigned int pwc_get_fps_Kiara(struct pwc_device *pdev, unsigned int index, unsigned int size)
    {
    unsigned int i;
    for (i = 0; i < PWC_FPS_MAX_KIARA; i++) {
    if (Kiara_table[size][i][3].alternate) {
    if (index--==0) return Kiara_fps_vector[i];
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwc_get_fps_Timon(pdev: *mut pwc_device, index: c_uint, size: c_uint) -> c_uint {
    static unsigned int pwc_get_fps_Timon(struct pwc_device *pdev, unsigned int index, unsigned int size)
    {
    unsigned int i;
    for (i=0; i < PWC_FPS_MAX_TIMON; i++) {
    if (Timon_table[size][i][3].alternate) {
    if (index--==0) return Timon_fps_vector[i];
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_get_fps(pdev: *mut pwc_device, index: c_uint, size: c_uint) -> c_uint {
    unsigned int pwc_get_fps(struct pwc_device *pdev, unsigned int index, unsigned int size)
    {
    unsigned int ret;
    if (DEVICE_USE_CODEC1(pdev.type)) {
    ret = pwc_get_fps_Nala(pdev, index, size);
    } else if (DEVICE_USE_CODEC3(pdev.type)) {
    ret = pwc_get_fps_Kiara(pdev, index, size);
    } else {
    ret = pwc_get_fps_Timon(pdev, index, size);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_get_u8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: *mut c_int) -> c_int {
    int pwc_get_u8_ctrl(struct pwc_device *pdev, u8 request, u16 value, int *data)
    {
    int ret;
    ret = recv_control_msg(pdev, request, value, 1);
    if (ret < 0)
    return ret;
// data = pdev->ctrl_buf[0];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_set_u8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: u8) -> c_int {
    int pwc_set_u8_ctrl(struct pwc_device *pdev, u8 request, u16 value, u8 data)
    {
    int ret;
    pdev.ctrl_buf[0] = data;
    ret = send_control_msg(pdev, request, value, pdev.ctrl_buf, 1);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_get_s8_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: *mut c_int) -> c_int {
    int pwc_get_s8_ctrl(struct pwc_device *pdev, u8 request, u16 value, int *data)
    {
    int ret;
    ret = recv_control_msg(pdev, request, value, 1);
    if (ret < 0)
    return ret;
// data = ((s8 *)pdev->ctrl_buf)[0];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_get_u16_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: *mut c_int) -> c_int {
    int pwc_get_u16_ctrl(struct pwc_device *pdev, u8 request, u16 value, int *data)
    {
    int ret;
    ret = recv_control_msg(pdev, request, value, 2);
    if (ret < 0)
    return ret;
// data = (pdev->ctrl_buf[1] << 8) | pdev->ctrl_buf[0];
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_set_u16_ctrl(pdev: *mut pwc_device, request: u8, value: u16, data: u16) -> c_int {
    int pwc_set_u16_ctrl(struct pwc_device *pdev, u8 request, u16 value, u16 data)
    {
    int ret;
    pdev.ctrl_buf[0] = data & 0xff;
    pdev.ctrl_buf[1] = data >> 8;
    ret = send_control_msg(pdev, request, value, pdev.ctrl_buf, 2);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_button_ctrl(pdev: *mut pwc_device, value: u16) -> c_int {
    int pwc_button_ctrl(struct pwc_device *pdev, u16 value)
    {
    int ret;
    ret = send_control_msg(pdev, SET_STATUS_CTL, value, core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
    return 0;
    }
// POWER
#[no_mangle]
pub unsafe extern "C" fn pwc_camera_power(pdev: *mut pwc_device, power: c_int) {
    void pwc_camera_power(struct pwc_device *pdev, int power)
    {
    int r;
    if (!pdev.power_save)
    return;
    if (pdev.type < 675 || (pdev.type < 730 && pdev.release < 6))
    return;	/* Not supported by Nala or Timon < release 6 */
    if (power)
    pdev.ctrl_buf[0] = 0x00; /* active */
    else
    pdev.ctrl_buf[0] = 0xFF; /* power save */
    r = send_control_msg(pdev, SET_STATUS_CTL,
    SET_POWER_SAVE_MODE_FORMATTER, pdev.ctrl_buf, 1);
    if (r < 0)
    PWC_ERROR("Failed to power %s camera (%d)\n",
    power ? "on" : "off", r);
    }
#[no_mangle]
pub unsafe extern "C" fn pwc_set_leds(pdev: *mut pwc_device, on_value: c_int, off_value: c_int) -> c_int {
    int pwc_set_leds(struct pwc_device *pdev, int on_value, int off_value)
    {
    int r;
    if (pdev.type < 730)
    return 0;
    on_value /= 100;
    off_value /= 100;
    if (on_value < 0)
    on_value = 0;
    if (on_value > 0xff)
    on_value = 0xff;
    if (off_value < 0)
    off_value = 0;
    if (off_value > 0xff)
    off_value = 0xff;
    pdev.ctrl_buf[0] = on_value;
    pdev.ctrl_buf[1] = off_value;
    r = send_control_msg(pdev,
    SET_STATUS_CTL, LED_FORMATTER, pdev.ctrl_buf, 2);
    if (r < 0)
    PWC_ERROR("Failed to set LED on/off time (%d)\n", r);
    return r;
    }

#[no_mangle]
pub unsafe extern "C" fn pwc_get_cmos_sensor(pdev: *mut pwc_device, sensor: *mut c_int) -> c_int {
    int pwc_get_cmos_sensor(struct pwc_device *pdev, int *sensor)
    {
    int ret, request;
    if (pdev.type < 675)
    request = SENSOR_TYPE_FORMATTER1;
#[no_mangle]
pub unsafe extern "C" fn if(730: pdev->type <) -> else {
    else if (pdev.type < 730)
    return -1; /* The Vesta series doesn't have this call */
    else
    request = SENSOR_TYPE_FORMATTER2;
    ret = recv_control_msg(pdev, GET_STATUS_CTL, request, 1);
    if (ret < 0)
    return ret;
    if (pdev.type < 675)
// sensor = pdev->ctrl_buf[0] | 0x100;
    else
// sensor = pdev->ctrl_buf[0];
    return 0;
    }
