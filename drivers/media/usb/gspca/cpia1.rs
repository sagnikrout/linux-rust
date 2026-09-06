//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/gspca/cpia1.c
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
// cpia CPiA (1) gspca driver
//
// Copyright (C) 2010-2011 Hans de Goede <hdegoede@redhat.com>
//
// This module is adapted from the in kernel v4l1 cpia driver which is :
//
// (C) Copyright 1999-2000 Peter Pregler
// (C) Copyright 1999-2000 Scott J. Bertin
// (C) Copyright 1999-2000 Johannes Erdfelt <johannes@erdfelt.com>
// (C) Copyright 2000 STMicroelectronics
//

    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("Vision CPiA");
    MODULE_LICENSE("GPL");
// constant value's
pub const MAGIC_0: c_uint = 0x19;
pub const MAGIC_1: c_uint = 0x68;
pub const DATA_IN: c_uint = 0xc0;
pub const DATA_OUT: c_uint = 0x40;

pub const SUBSAMPLE_420: c_int = 0;
pub const SUBSAMPLE_422: c_int = 1;
pub const YUVORDER_YUYV: c_int = 0;
pub const YUVORDER_UYVY: c_int = 1;
pub const NOT_COMPRESSED: c_int = 0;
pub const COMPRESSED: c_int = 1;
pub const NO_DECIMATION: c_int = 0;
pub const DECIMATION_ENAB: c_int = 1;
pub const EOI: c_uint = 0xff	/* End Of Image */;
pub const EOL: c_uint = 0xfd	/* End Of Line */;
pub const FRAME_HEADER_SIZE: c_int = 64;
// Image grab modes
pub const CPIA_GRAB_SINGLE: c_int = 0;
pub const CPIA_GRAB_CONTINEOUS: c_int = 1;
// Compression parameters
pub const CPIA_COMPRESSION_NONE: c_int = 0;
pub const CPIA_COMPRESSION_AUTO: c_int = 1;
pub const CPIA_COMPRESSION_MANUAL: c_int = 2;
pub const CPIA_COMPRESSION_TARGET_QUALITY: c_int = 0;
pub const CPIA_COMPRESSION_TARGET_FRAMERATE: c_int = 1;
// Return offsets for GetCameraState
pub const SYSTEMSTATE: c_int = 0;
pub const GRABSTATE: c_int = 1;
pub const STREAMSTATE: c_int = 2;
pub const FATALERROR: c_int = 3;
pub const CMDERROR: c_int = 4;
pub const DEBUGFLAGS: c_int = 5;
pub const VPSTATUS: c_int = 6;
pub const ERRORCODE: c_int = 7;
// SystemState
pub const UNINITIALISED_STATE: c_int = 0;
pub const PASS_THROUGH_STATE: c_int = 1;
pub const LO_POWER_STATE: c_int = 2;
pub const HI_POWER_STATE: c_int = 3;
pub const WARM_BOOT_STATE: c_int = 4;
// GrabState
pub const GRAB_IDLE: c_int = 0;
pub const GRAB_ACTIVE: c_int = 1;
pub const GRAB_DONE: c_int = 2;
// StreamState
pub const STREAM_NOT_READY: c_int = 0;
pub const STREAM_READY: c_int = 1;
pub const STREAM_OPEN: c_int = 2;
pub const STREAM_PAUSED: c_int = 3;
pub const STREAM_FINISHED: c_int = 4;
// Fatal Error, CmdError, and DebugFlags
pub const CPIA_FLAG: c_int = 1;
pub const SYSTEM_FLAG: c_int = 2;
pub const INT_CTRL_FLAG: c_int = 4;
pub const PROCESS_FLAG: c_int = 8;
pub const COM_FLAG: c_int = 16;
pub const VP_CTRL_FLAG: c_int = 32;
pub const CAPTURE_FLAG: c_int = 64;
pub const DEBUG_FLAG: c_int = 128;
// VPStatus
pub const VP_STATE_OK: c_uint = 0x00;
pub const VP_STATE_FAILED_VIDEOINIT: c_uint = 0x01;
pub const VP_STATE_FAILED_AECACBINIT: c_uint = 0x02;
pub const VP_STATE_AEC_MAX: c_uint = 0x04;
pub const VP_STATE_ACB_BMAX: c_uint = 0x08;
pub const VP_STATE_ACB_RMIN: c_uint = 0x10;
pub const VP_STATE_ACB_GMIN: c_uint = 0x20;
pub const VP_STATE_ACB_RMAX: c_uint = 0x40;
pub const VP_STATE_ACB_GMAX: c_uint = 0x80;
// default (minimum) compensation values
pub const COMP_RED: c_int = 220;
pub const COMP_GREEN1: c_int = 214;

pub const COMP_BLUE: c_int = 230;
// exposure status
pub const EXPOSURE_VERY_LIGHT: c_int = 0;
pub const EXPOSURE_LIGHT: c_int = 1;
pub const EXPOSURE_NORMAL: c_int = 2;
pub const EXPOSURE_DARK: c_int = 3;
pub const EXPOSURE_VERY_DARK: c_int = 4;

pub const ROUND_UP_EXP_FOR_FLICKER: c_int = 15;
// Constants for automatic frame rate adjustment
pub const MAX_EXP: c_int = 302;
pub const MAX_EXP_102: c_int = 255;
pub const LOW_EXP: c_int = 140;
pub const VERY_LOW_EXP: c_int = 70;
pub const TC: c_int = 94;
pub const EXP_ACC_DARK: c_int = 50;
pub const EXP_ACC_LIGHT: c_int = 90;
pub const HIGH_COMP_102: c_int = 160;
pub const MAX_COMP: c_int = 239;
pub const DARK_TIME: c_int = 3;
pub const LIGHT_TIME: c_int = 3;

    sd.params.version.firmwareRevision == (y))

pub const BRIGHTNESS_DEF: c_int = 50;
pub const CONTRAST_DEF: c_int = 48;
pub const SATURATION_DEF: c_int = 50;

pub const ILLUMINATORS_1_DEF: c_int = 0;
pub const ILLUMINATORS_2_DEF: c_int = 0;

// Developer's Guide Table 5 p 3-34
// indexed by [mains][sensorFps.baserate][sensorFps.divisor]
    static u8 flicker_jumps[2][2][4] =
    { { { 76, 38, 19, 9 }, { 92, 46, 23, 11 } },
    { { 64, 32, 16, 8 }, { 76, 38, 19, 9} }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cam_params {
    struct {
    pub firmwareVersion: u8,
    pub firmwareRevision: u8,
    pub vcVersion: u8,
    pub vcRevision: u8,
    pub version: },
    struct {
    pub vendor: u16,
    pub product: u16,
    pub deviceRevision: u16,
    pub pnpID: },
    struct {
    pub vpVersion: u8,
    pub vpRevision: u8,
    pub cameraHeadID: u16,
    pub vpVersion: },
    struct {
    pub systemState: u8,
    pub grabState: u8,
    pub streamState: u8,
    pub fatalError: u8,
    pub cmdError: u8,
    pub debugFlags: u8,
    pub vpStatus: u8,
    pub errorCode: u8,
    pub status: },
    struct {
    pub brightness: u8,
    pub contrast: u8,
    pub saturation: u8,
    pub colourParams: },
    struct {
    pub gainMode: u8,
    pub expMode: u8,
    pub compMode: u8,
    pub centreWeight: u8,
    pub gain: u8,
    pub fineExp: u8,
    pub coarseExpLo: u8,
    pub coarseExpHi: u8,
    pub redComp: u8,
    pub green1Comp: u8,
    pub green2Comp: u8,
    pub blueComp: u8,
    pub exposure: },
    struct {
    pub balanceMode: u8,
    pub redGain: u8,
    pub greenGain: u8,
    pub blueGain: u8,
    pub colourBalance: },
    struct {
    pub divisor: u8,
    pub baserate: u8,
    pub sensorFps: },
    struct {
    pub gain1: u8,
    pub gain2: u8,
    pub gain4: u8,
    pub gain8: u8,
    pub apcor: },
    struct {
    pub disabled: u8,
    pub flickerMode: u8,
    pub coarseJump: u8,
    pub allowableOverExposure: u8,
    pub flickerControl: },
    struct {
    pub gain1: u8,
    pub gain2: u8,
    pub gain4: u8,
    pub gain8: u8,
    pub vlOffset: },
    struct {
    pub mode: u8,
    pub decimation: u8,
    pub compression: },
    struct {
    pub frTargeting: u8,
    pub targetFR: u8,
    pub targetQ: u8,
    pub compressionTarget: },
    struct {
    pub yThreshold: u8,
    pub uvThreshold: u8,
    pub yuvThreshold: },
    struct {
    pub hysteresis: u8,
    pub threshMax: u8,
    pub smallStep: u8,
    pub largeStep: u8,
    pub decimationHysteresis: u8,
    pub frDiffStepThresh: u8,
    pub qDiffStepThresh: u8,
    pub decimationThreshMod: u8,
    pub compressionParams: },
    struct {
    pub /: *mut *mut u8 videoSize; / CIF/QCIF,
    pub subSample: u8,
    pub yuvOrder: u8,
    pub format: },
    struct {                        /* Intel QX3 specific data */
    pub /: *mut *mut u8 qx3_detected; / a QX3 is present,
    pub /: *mut *mut u8 toplight; / top light lit , R/W,
    pub /: *mut *mut u8 bottomlight; / bottom light lit, R/W,
    pub /: *mut *mut u8 button; / snapshot button pressed (R/O),
    pub /: *mut *mut u8 cradled; / microscope is in cradle (R/O),
    pub qx3: },
    struct {
    pub /: *mut *mut *mut u8 colStart; / skip first 8colStart pixels,
    pub /: *mut *mut *mut u8 colEnd; / finish at 8colEnd pixels,
    pub /: *mut *mut *mut u8 rowStart; / skip first 4rowStart lines,
    pub /: *mut *mut *mut u8 rowEnd; / finish at 4rowEnd lines,
    pub roi: },
    pub ecpTiming: u8,
    pub streamStartLine: u8,
}

// specific webcam descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd {
    pub /: *mut *mut gspca_dev gspca_dev; / !! must be the first item,
    pub /: *mut *mut cam_params params; / camera settings,
    pub cam_exposure: core::sync::atomic::AtomicI32,
    pub fps: core::sync::atomic::AtomicI32,
    pub exposure_count: c_int,
    pub exposure_status: u8,
    pub freq: *mut v4l2_ctrl,
    pub /: *mut *mut u8 mainsFreq; / 0 = 50hz, 1 = 60hz,
    pub first_frame: u8,
}

    static const struct v4l2_pix_format mode[] = {
    {160, 120, V4L2_PIX_FMT_CPIA1, V4L2_FIELD_NONE,
// The sizeimage is trial and error, as with low framerates
// the camera will pad out usb frames, making the image
// data larger than strictly necessary
//
    .bytesperline = 160,
    .sizeimage = 65536,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 3},
    {176, 144, V4L2_PIX_FMT_CPIA1, V4L2_FIELD_NONE,
    .bytesperline = 172,
    .sizeimage = 65536,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 2},
    {320, 240, V4L2_PIX_FMT_CPIA1, V4L2_FIELD_NONE,
    .bytesperline = 320,
    .sizeimage = 262144,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 1},
    {352, 288, V4L2_PIX_FMT_CPIA1, V4L2_FIELD_NONE,
    .bytesperline = 352,
    .sizeimage = 262144,
    .colorspace = V4L2_COLORSPACE_SRGB,
    .priv = 0},
    };
//
// General functions
//
#[no_mangle]
unsafe extern "C" fn cpia_usb_transferCmd(gspca_dev: *mut gspca_dev, command: *mut u8) -> c_int {
    static int cpia_usb_transferCmd(struct gspca_dev *gspca_dev, u8 *command)
    {
    u8 requesttype;
    unsigned int pipe;
    int ret, databytes = command[6] | (command[7] << 8);
// Sometimes we see spurious EPIPE errors
    let mut retries: c_int = 3;
    if (command[0] == DATA_IN) {
    pipe = usb_rcvctrlpipe(gspca_dev.dev, 0);
    requesttype = USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE;
    } else if (command[0] == DATA_OUT) {
    pipe = usb_sndctrlpipe(gspca_dev.dev, 0);
    requesttype = USB_TYPE_VENDOR | USB_RECIP_DEVICE;
    } else {
    gspca_err(gspca_dev, "Unexpected first byte of command: %x\n",
    command[0]);
    return -EINVAL;
    }
    retry:
    ret = usb_control_msg(gspca_dev.dev, pipe,
    command[1],
    requesttype,
    command[2] | (command[3] << 8),
    command[4] | (command[5] << 8),
    gspca_dev.usb_buf, databytes, 1000);
    if (ret < 0)
    pr_err("usb_control_msg %02x, error %d\n", command[1], ret);
    if (ret == -EPIPE && retries > 0) {
    retries--;
    goto retry;
    }
    return (ret < 0) ? ret : 0;
    }
// send an arbitrary command to the camera
    static int do_command(struct gspca_dev *gspca_dev, u16 command,
    u8 a, u8 b, u8 c, u8 d)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret, datasize;
    u8 cmd[8];
    switch (command) {
    case CPIA_COMMAND_GetCPIAVersion:
    case CPIA_COMMAND_GetPnPID:
    case CPIA_COMMAND_GetCameraStatus:
    case CPIA_COMMAND_GetVPVersion:
    case CPIA_COMMAND_GetColourParams:
    case CPIA_COMMAND_GetColourBalance:
    case CPIA_COMMAND_GetExposure:
    datasize = 8;
    break;
    case CPIA_COMMAND_ReadMCPorts:
    case CPIA_COMMAND_ReadVCRegs:
    datasize = 4;
    break;
    default:
    datasize = 0;
    break;
    }
    cmd[0] = command >> 8;
    cmd[1] = command & 0xff;
    cmd[2] = a;
    cmd[3] = b;
    cmd[4] = c;
    cmd[5] = d;
    cmd[6] = datasize;
    cmd[7] = 0;
    ret = cpia_usb_transferCmd(gspca_dev, cmd);
    if (ret)
    return ret;
    switch (command) {
    case CPIA_COMMAND_GetCPIAVersion:
    sd.params.version.firmwareVersion = gspca_dev.usb_buf[0];
    sd.params.version.firmwareRevision = gspca_dev.usb_buf[1];
    sd.params.version.vcVersion = gspca_dev.usb_buf[2];
    sd.params.version.vcRevision = gspca_dev.usb_buf[3];
    break;
    case CPIA_COMMAND_GetPnPID:
    sd.params.pnpID.vendor =
    gspca_dev.usb_buf[0] | (gspca_dev.usb_buf[1] << 8);
    sd.params.pnpID.product =
    gspca_dev.usb_buf[2] | (gspca_dev.usb_buf[3] << 8);
    sd.params.pnpID.deviceRevision =
    gspca_dev.usb_buf[4] | (gspca_dev.usb_buf[5] << 8);
    break;
    case CPIA_COMMAND_GetCameraStatus:
    sd.params.status.systemState = gspca_dev.usb_buf[0];
    sd.params.status.grabState = gspca_dev.usb_buf[1];
    sd.params.status.streamState = gspca_dev.usb_buf[2];
    sd.params.status.fatalError = gspca_dev.usb_buf[3];
    sd.params.status.cmdError = gspca_dev.usb_buf[4];
    sd.params.status.debugFlags = gspca_dev.usb_buf[5];
    sd.params.status.vpStatus = gspca_dev.usb_buf[6];
    sd.params.status.errorCode = gspca_dev.usb_buf[7];
    break;
    case CPIA_COMMAND_GetVPVersion:
    sd.params.vpVersion.vpVersion = gspca_dev.usb_buf[0];
    sd.params.vpVersion.vpRevision = gspca_dev.usb_buf[1];
    sd.params.vpVersion.cameraHeadID =
    gspca_dev.usb_buf[2] | (gspca_dev.usb_buf[3] << 8);
    break;
    case CPIA_COMMAND_GetColourParams:
    sd.params.colourParams.brightness = gspca_dev.usb_buf[0];
    sd.params.colourParams.contrast = gspca_dev.usb_buf[1];
    sd.params.colourParams.saturation = gspca_dev.usb_buf[2];
    break;
    case CPIA_COMMAND_GetColourBalance:
    sd.params.colourBalance.redGain = gspca_dev.usb_buf[0];
    sd.params.colourBalance.greenGain = gspca_dev.usb_buf[1];
    sd.params.colourBalance.blueGain = gspca_dev.usb_buf[2];
    break;
    case CPIA_COMMAND_GetExposure:
    sd.params.exposure.gain = gspca_dev.usb_buf[0];
    sd.params.exposure.fineExp = gspca_dev.usb_buf[1];
    sd.params.exposure.coarseExpLo = gspca_dev.usb_buf[2];
    sd.params.exposure.coarseExpHi = gspca_dev.usb_buf[3];
    sd.params.exposure.redComp = gspca_dev.usb_buf[4];
    sd.params.exposure.green1Comp = gspca_dev.usb_buf[5];
    sd.params.exposure.green2Comp = gspca_dev.usb_buf[6];
    sd.params.exposure.blueComp = gspca_dev.usb_buf[7];
    break;
    case CPIA_COMMAND_ReadMCPorts:
// test button press
    a = ((gspca_dev.usb_buf[1] & 0x02) == 0);
    if (a != sd.params.qx3.button) {

    input_report_key(gspca_dev.input_dev, KEY_CAMERA, a);
    input_sync(gspca_dev.input_dev);

    sd.params.qx3.button = a;
    }
    if (sd.params.qx3.button) {
// button pressed - unlock the latch
    ret = do_command(gspca_dev, CPIA_COMMAND_WriteMCPort,
    3, 0xdf, 0xdf, 0);
    if (ret)
    return ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_WriteMCPort,
    3, 0xff, 0xff, 0);
    if (ret)
    return ret;
    }
// test whether microscope is cradled
    sd.params.qx3.cradled = ((gspca_dev.usb_buf[2] & 0x40) == 0);
    break;
    }
    return 0;
    }
// send a command to the camera with an additional data transaction
    static int do_command_extended(struct gspca_dev *gspca_dev, u16 command,
    u8 a, u8 b, u8 c, u8 d,
    u8 e, u8 f, u8 g, u8 h,
    u8 i, u8 j, u8 k, u8 l)
    {
    u8 cmd[8];
    cmd[0] = command >> 8;
    cmd[1] = command & 0xff;
    cmd[2] = a;
    cmd[3] = b;
    cmd[4] = c;
    cmd[5] = d;
    cmd[6] = 8;
    cmd[7] = 0;
    gspca_dev.usb_buf[0] = e;
    gspca_dev.usb_buf[1] = f;
    gspca_dev.usb_buf[2] = g;
    gspca_dev.usb_buf[3] = h;
    gspca_dev.usb_buf[4] = i;
    gspca_dev.usb_buf[5] = j;
    gspca_dev.usb_buf[6] = k;
    gspca_dev.usb_buf[7] = l;
    return cpia_usb_transferCmd(gspca_dev, cmd);
    }
// find_over_exposure
// Finds a suitable value of OverExposure for use with SetFlickerCtrl
// Some calculation is required because this value changes with the brightness
// set with SetColourParameters
//
// Parameters: Brightness - last brightness value set with SetColourParameters
//
// Returns: OverExposure value to use with SetFlickerCtrl
//
pub const FLICKER_MAX_EXPOSURE: c_int = 250;
pub const FLICKER_ALLOWABLE_OVER_EXPOSURE: c_int = 146;
pub const FLICKER_BRIGHTNESS_CONSTANT: c_int = 59;
#[no_mangle]
unsafe extern "C" fn find_over_exposure(brightness: c_int) -> c_int {
    static int find_over_exposure(int brightness)
    {
    int MaxAllowableOverExposure, OverExposure;
    MaxAllowableOverExposure = FLICKER_MAX_EXPOSURE - brightness -
    FLICKER_BRIGHTNESS_CONSTANT;
    OverExposure = min(MaxAllowableOverExposure,
    FLICKER_ALLOWABLE_OVER_EXPOSURE);
    return OverExposure;
    }

// initialise cam_data structure
#[no_mangle]
unsafe extern "C" fn reset_camera_params(gspca_dev: *mut gspca_dev) {
    static void reset_camera_params(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    struct cam_params *params = &sd.params;
// The following parameter values are the defaults from
// "Software Developer's Guide for CPiA Cameras".  Any changes
// to the defaults are noted in comments.
    params.colourParams.brightness = BRIGHTNESS_DEF;
    params.colourParams.contrast = CONTRAST_DEF;
    params.colourParams.saturation = SATURATION_DEF;
    params.exposure.gainMode = 4;
    params.exposure.expMode = 2;		/* AEC */
    params.exposure.compMode = 1;
    params.exposure.centreWeight = 1;
    params.exposure.gain = 0;
    params.exposure.fineExp = 0;
    params.exposure.coarseExpLo = 185;
    params.exposure.coarseExpHi = 0;
    params.exposure.redComp = COMP_RED;
    params.exposure.green1Comp = COMP_GREEN1;
    params.exposure.green2Comp = COMP_GREEN2;
    params.exposure.blueComp = COMP_BLUE;
    params.colourBalance.balanceMode = 2;	/* ACB */
    params.colourBalance.redGain = 32;
    params.colourBalance.greenGain = 6;
    params.colourBalance.blueGain = 92;
    params.apcor.gain1 = 0x18;
    params.apcor.gain2 = 0x16;
    params.apcor.gain4 = 0x24;
    params.apcor.gain8 = 0x34;
    params.vlOffset.gain1 = 20;
    params.vlOffset.gain2 = 24;
    params.vlOffset.gain4 = 26;
    params.vlOffset.gain8 = 26;
    params.compressionParams.hysteresis = 3;
    params.compressionParams.threshMax = 11;
    params.compressionParams.smallStep = 1;
    params.compressionParams.largeStep = 3;
    params.compressionParams.decimationHysteresis = 2;
    params.compressionParams.frDiffStepThresh = 5;
    params.compressionParams.qDiffStepThresh = 3;
    params.compressionParams.decimationThreshMod = 2;
// End of default values from Software Developer's Guide
// Set Sensor FPS to 15fps. This seems better than 30fps
// for indoor lighting.
    params.sensorFps.divisor = 1;
    params.sensorFps.baserate = 1;
    params.flickerControl.flickerMode = 0;
    params.flickerControl.disabled = 1;
    params.flickerControl.coarseJump =
    flicker_jumps[sd.mainsFreq]
    [params.sensorFps.baserate]
    [params.sensorFps.divisor];
    params.flickerControl.allowableOverExposure =
    find_over_exposure(params.colourParams.brightness);
    params.yuvThreshold.yThreshold = 6; /* From windows driver */
    params.yuvThreshold.uvThreshold = 6; /* From windows driver */
    params.format.subSample = SUBSAMPLE_420;
    params.format.yuvOrder = YUVORDER_YUYV;
    params.compression.mode = CPIA_COMPRESSION_AUTO;
    params.compression.decimation = NO_DECIMATION;
    params.compressionTarget.frTargeting = COMP_TARGET_DEF;
    params.compressionTarget.targetFR = 15; /* From windows driver */
    params.compressionTarget.targetQ = 5; /* From windows driver */
    params.qx3.qx3_detected = 0;
    params.qx3.toplight = 0;
    params.qx3.bottomlight = 0;
    params.qx3.button = 0;
    params.qx3.cradled = 0;
    }
#[no_mangle]
unsafe extern "C" fn printstatus(gspca_dev: *mut gspca_dev, params: *mut cam_params) {
    static void printstatus(struct gspca_dev *gspca_dev, struct cam_params *params)
    {
    gspca_dbg(gspca_dev, D_PROBE, "status: %02x %02x %02x %02x %02x %02x %02x %02x\n",
    params.status.systemState, params.status.grabState,
    params.status.streamState, params.status.fatalError,
    params.status.cmdError, params.status.debugFlags,
    params.status.vpStatus, params.status.errorCode);
    }
#[no_mangle]
unsafe extern "C" fn goto_low_power(gspca_dev: *mut gspca_dev) -> c_int {
    static int goto_low_power(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_GotoLoPower, 0, 0, 0, 0);
    if (ret)
    return ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_GetCameraStatus, 0, 0, 0, 0);
    if (ret)
    return ret;
    if (sd.params.status.systemState != LO_POWER_STATE) {
    if (sd.params.status.systemState != WARM_BOOT_STATE) {
    gspca_err(gspca_dev, "unexpected state after lo power cmd: %02x\n",
    sd.params.status.systemState);
    printstatus(gspca_dev, &sd.params);
    }
    return -EIO;
    }
    gspca_dbg(gspca_dev, D_CONF, "camera now in LOW power state\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn goto_high_power(gspca_dev: *mut gspca_dev) -> c_int {
    static int goto_high_power(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_GotoHiPower, 0, 0, 0, 0);
    if (ret)
    return ret;
    msleep_interruptible(40);	/* windows driver does it too */
    if (signal_pending(current))
    return -EINTR;
    ret = do_command(gspca_dev, CPIA_COMMAND_GetCameraStatus, 0, 0, 0, 0);
    if (ret)
    return ret;
    if (sd.params.status.systemState != HI_POWER_STATE) {
    gspca_err(gspca_dev, "unexpected state after hi power cmd: %02x\n",
    sd.params.status.systemState);
    printstatus(gspca_dev, &sd.params);
    return -EIO;
    }
    gspca_dbg(gspca_dev, D_CONF, "camera now in HIGH power state\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_version_information(gspca_dev: *mut gspca_dev) -> c_int {
    static int get_version_information(struct gspca_dev *gspca_dev)
    {
    int ret;
// GetCPIAVersion
    ret = do_command(gspca_dev, CPIA_COMMAND_GetCPIAVersion, 0, 0, 0, 0);
    if (ret)
    return ret;
// GetPnPID
    return do_command(gspca_dev, CPIA_COMMAND_GetPnPID, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn save_camera_state(gspca_dev: *mut gspca_dev) -> c_int {
    static int save_camera_state(struct gspca_dev *gspca_dev)
    {
    int ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_GetColourBalance, 0, 0, 0, 0);
    if (ret)
    return ret;
    return do_command(gspca_dev, CPIA_COMMAND_GetExposure, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setformat(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setformat(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_SetFormat,
    sd.params.format.videoSize,
    sd.params.format.subSample,
    sd.params.format.yuvOrder, 0);
    if (ret)
    return ret;
    return do_command(gspca_dev, CPIA_COMMAND_SetROI,
    sd.params.roi.colStart, sd.params.roi.colEnd,
    sd.params.roi.rowStart, sd.params.roi.rowEnd);
    }
#[no_mangle]
unsafe extern "C" fn command_setcolourparams(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setcolourparams(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetColourParams,
    sd.params.colourParams.brightness,
    sd.params.colourParams.contrast,
    sd.params.colourParams.saturation, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setapcor(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setapcor(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetApcor,
    sd.params.apcor.gain1,
    sd.params.apcor.gain2,
    sd.params.apcor.gain4,
    sd.params.apcor.gain8);
    }
#[no_mangle]
unsafe extern "C" fn command_setvloffset(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setvloffset(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetVLOffset,
    sd.params.vlOffset.gain1,
    sd.params.vlOffset.gain2,
    sd.params.vlOffset.gain4,
    sd.params.vlOffset.gain8);
    }
#[no_mangle]
unsafe extern "C" fn command_setexposure(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setexposure(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret;
    ret = do_command_extended(gspca_dev, CPIA_COMMAND_SetExposure,
    sd.params.exposure.gainMode,
    1,
    sd.params.exposure.compMode,
    sd.params.exposure.centreWeight,
    sd.params.exposure.gain,
    sd.params.exposure.fineExp,
    sd.params.exposure.coarseExpLo,
    sd.params.exposure.coarseExpHi,
    sd.params.exposure.redComp,
    sd.params.exposure.green1Comp,
    sd.params.exposure.green2Comp,
    sd.params.exposure.blueComp);
    if (ret)
    return ret;
    if (sd.params.exposure.expMode != 1) {
    ret = do_command_extended(gspca_dev, CPIA_COMMAND_SetExposure,
    0,
    sd.params.exposure.expMode,
    0, 0,
    sd.params.exposure.gain,
    sd.params.exposure.fineExp,
    sd.params.exposure.coarseExpLo,
    sd.params.exposure.coarseExpHi,
    0, 0, 0, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn command_setcolourbalance(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setcolourbalance(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    if (sd.params.colourBalance.balanceMode == 1) {
    int ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_SetColourBalance,
    1,
    sd.params.colourBalance.redGain,
    sd.params.colourBalance.greenGain,
    sd.params.colourBalance.blueGain);
    if (ret)
    return ret;
    return do_command(gspca_dev, CPIA_COMMAND_SetColourBalance,
    3, 0, 0, 0);
    }
    if (sd.params.colourBalance.balanceMode == 2) {
    return do_command(gspca_dev, CPIA_COMMAND_SetColourBalance,
    2, 0, 0, 0);
    }
    if (sd.params.colourBalance.balanceMode == 3) {
    return do_command(gspca_dev, CPIA_COMMAND_SetColourBalance,
    3, 0, 0, 0);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn command_setcompressiontarget(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setcompressiontarget(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetCompressionTarget,
    sd.params.compressionTarget.frTargeting,
    sd.params.compressionTarget.targetFR,
    sd.params.compressionTarget.targetQ, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setyuvtresh(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setyuvtresh(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetYUVThresh,
    sd.params.yuvThreshold.yThreshold,
    sd.params.yuvThreshold.uvThreshold, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setcompressionparams(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setcompressionparams(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command_extended(gspca_dev,
    CPIA_COMMAND_SetCompressionParams,
    0, 0, 0, 0,
    sd.params.compressionParams.hysteresis,
    sd.params.compressionParams.threshMax,
    sd.params.compressionParams.smallStep,
    sd.params.compressionParams.largeStep,
    sd.params.compressionParams.decimationHysteresis,
    sd.params.compressionParams.frDiffStepThresh,
    sd.params.compressionParams.qDiffStepThresh,
    sd.params.compressionParams.decimationThreshMod);
    }
#[no_mangle]
unsafe extern "C" fn command_setcompression(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setcompression(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetCompression,
    sd.params.compression.mode,
    sd.params.compression.decimation, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setsensorfps(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setsensorfps(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetSensorFPS,
    sd.params.sensorFps.divisor,
    sd.params.sensorFps.baserate, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setflickerctrl(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setflickerctrl(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetFlickerCtrl,
    sd.params.flickerControl.flickerMode,
    sd.params.flickerControl.coarseJump,
    sd.params.flickerControl.allowableOverExposure,
    0);
    }
#[no_mangle]
unsafe extern "C" fn command_setecptiming(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setecptiming(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_SetECPTiming,
    sd.params.ecpTiming, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_pause(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_pause(struct gspca_dev *gspca_dev)
    {
    return do_command(gspca_dev, CPIA_COMMAND_EndStreamCap, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_resume(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_resume(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    return do_command(gspca_dev, CPIA_COMMAND_InitStreamCap,
    0, sd.params.streamStartLine, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn command_setlights(gspca_dev: *mut gspca_dev) -> c_int {
    static int command_setlights(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret, p1, p2;
    p1 = (sd.params.qx3.bottomlight == 0) << 1;
    p2 = (sd.params.qx3.toplight == 0) << 3;
    ret = do_command(gspca_dev, CPIA_COMMAND_WriteVCReg,
    0x90, 0x8f, 0x50, 0);
    if (ret)
    return ret;
    return do_command(gspca_dev, CPIA_COMMAND_WriteMCPort, 2, 0,
    p1 | p2 | 0xe0, 0);
    }
#[no_mangle]
unsafe extern "C" fn set_flicker(gspca_dev: *mut gspca_dev, on: c_int, apply: c_int) -> c_int {
    static int set_flicker(struct gspca_dev *gspca_dev, int on, int apply)
    {
// Everything in here is from the Windows driver
// define for compgain calculation

    (u8) ((((float) base - 128.0) * ((float) curexp / (float) newexp)) + 128.5)

    (u16)((float)curexp * (float)(u8)(curcomp + 128) / \
    (float)(u8)(basecomp - 128))

// equivalent functions without floating point math

    (u8)(128 + (((u32)(2*(base-128)*curexp + newexp)) / (2 * newexp)))

    (u16)(((u32)(curexp * (u8)(curcomp + 128)) / (u8)(basecomp - 128)))

    struct sd *sd = (struct sd *) gspca_dev;
    int currentexp = sd.params.exposure.coarseExpLo +
    sd.params.exposure.coarseExpHi * 256;
    int ret, startexp;
    if (on) {
    let mut cj: c_int = sd.params.flickerControl.coarseJump;
    sd.params.flickerControl.flickerMode = 1;
    sd.params.flickerControl.disabled = 0;
    if (sd.params.exposure.expMode != 2) {
    sd.params.exposure.expMode = 2;
    sd.exposure_status = EXPOSURE_NORMAL;
    }
    if (sd.params.exposure.gain >= BITS_PER_TYPE(currentexp))
    return -EINVAL;
    currentexp = currentexp << sd.params.exposure.gain;
    sd.params.exposure.gain = 0;
// round down current exposure to nearest value
    startexp = (currentexp + ROUND_UP_EXP_FOR_FLICKER) / cj;
    if (startexp < 1)
    startexp = 1;
    startexp = (startexp * cj) - 1;
    if (FIRMWARE_VERSION(1, 2))
    while (startexp > MAX_EXP_102)
    startexp -= cj;
    else
    while (startexp > MAX_EXP)
    startexp -= cj;
    sd.params.exposure.coarseExpLo = startexp & 0xff;
    sd.params.exposure.coarseExpHi = startexp >> 8;
    if (currentexp > startexp) {
    if (currentexp > (2 * startexp))
    currentexp = 2 * startexp;
    sd.params.exposure.redComp =
    COMPGAIN(COMP_RED, currentexp, startexp);
    sd.params.exposure.green1Comp =
    COMPGAIN(COMP_GREEN1, currentexp, startexp);
    sd.params.exposure.green2Comp =
    COMPGAIN(COMP_GREEN2, currentexp, startexp);
    sd.params.exposure.blueComp =
    COMPGAIN(COMP_BLUE, currentexp, startexp);
    } else {
    sd.params.exposure.redComp = COMP_RED;
    sd.params.exposure.green1Comp = COMP_GREEN1;
    sd.params.exposure.green2Comp = COMP_GREEN2;
    sd.params.exposure.blueComp = COMP_BLUE;
    }
    if (FIRMWARE_VERSION(1, 2))
    sd.params.exposure.compMode = 0;
    else
    sd.params.exposure.compMode = 1;
    sd.params.apcor.gain1 = 0x18;
    sd.params.apcor.gain2 = 0x18;
    sd.params.apcor.gain4 = 0x16;
    sd.params.apcor.gain8 = 0x14;
    } else {
    sd.params.flickerControl.flickerMode = 0;
    sd.params.flickerControl.disabled = 1;
// Average equivalent coarse for each comp channel
    startexp = EXP_FROM_COMP(COMP_RED,
    sd.params.exposure.redComp, currentexp);
    startexp += EXP_FROM_COMP(COMP_GREEN1,
    sd.params.exposure.green1Comp, currentexp);
    startexp += EXP_FROM_COMP(COMP_GREEN2,
    sd.params.exposure.green2Comp, currentexp);
    startexp += EXP_FROM_COMP(COMP_BLUE,
    sd.params.exposure.blueComp, currentexp);
    startexp = startexp >> 2;
    while (startexp > MAX_EXP && sd.params.exposure.gain <
    sd.params.exposure.gainMode - 1) {
    startexp = startexp >> 1;
    ++sd.params.exposure.gain;
    }
    if (FIRMWARE_VERSION(1, 2) && startexp > MAX_EXP_102)
    startexp = MAX_EXP_102;
    if (startexp > MAX_EXP)
    startexp = MAX_EXP;
    sd.params.exposure.coarseExpLo = startexp & 0xff;
    sd.params.exposure.coarseExpHi = startexp >> 8;
    sd.params.exposure.redComp = COMP_RED;
    sd.params.exposure.green1Comp = COMP_GREEN1;
    sd.params.exposure.green2Comp = COMP_GREEN2;
    sd.params.exposure.blueComp = COMP_BLUE;
    sd.params.exposure.compMode = 1;
    sd.params.apcor.gain1 = 0x18;
    sd.params.apcor.gain2 = 0x16;
    sd.params.apcor.gain4 = 0x24;
    sd.params.apcor.gain8 = 0x34;
    }
    sd.params.vlOffset.gain1 = 20;
    sd.params.vlOffset.gain2 = 24;
    sd.params.vlOffset.gain4 = 26;
    sd.params.vlOffset.gain8 = 26;
    if (apply) {
    ret = command_setexposure(gspca_dev);
    if (ret)
    return ret;
    ret = command_setapcor(gspca_dev);
    if (ret)
    return ret;
    ret = command_setvloffset(gspca_dev);
    if (ret)
    return ret;
    ret = command_setflickerctrl(gspca_dev);
    if (ret)
    return ret;
    }
    return 0;

    }
// monitor the exposure and adjust the sensor frame rate if needed
#[no_mangle]
unsafe extern "C" fn monitor_exposure(gspca_dev: *mut gspca_dev) {
    static void monitor_exposure(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    u8 exp_acc, bcomp, cmd[8];
    int ret, light_exp, dark_exp, very_dark_exp;
    int old_exposure, new_exposure, framerate;
    let mut setfps: c_int = 0, setexp = 0, setflicker = 0;
// get necessary stats and register settings from camera
// do_command can't handle this, so do it ourselves
    cmd[0] = CPIA_COMMAND_ReadVPRegs >> 8;
    cmd[1] = CPIA_COMMAND_ReadVPRegs & 0xff;
    cmd[2] = 30;
    cmd[3] = 4;
    cmd[4] = 9;
    cmd[5] = 8;
    cmd[6] = 8;
    cmd[7] = 0;
    ret = cpia_usb_transferCmd(gspca_dev, cmd);
    if (ret) {
    pr_err("ReadVPRegs(30,4,9,8) - failed: %d\n", ret);
    return;
    }
    exp_acc = gspca_dev.usb_buf[0];
    bcomp = gspca_dev.usb_buf[1];
    light_exp = sd.params.colourParams.brightness +
    TC - 50 + EXP_ACC_LIGHT;
    if (light_exp > 255)
    light_exp = 255;
    dark_exp = sd.params.colourParams.brightness +
    TC - 50 - EXP_ACC_DARK;
    if (dark_exp < 0)
    dark_exp = 0;
    very_dark_exp = dark_exp / 2;
    old_exposure = sd.params.exposure.coarseExpHi * 256 +
    sd.params.exposure.coarseExpLo;
    if (!sd.params.flickerControl.disabled) {
// Flicker control on
    int max_comp = FIRMWARE_VERSION(1, 2) ? MAX_COMP :
    HIGH_COMP_102;
    bcomp += 128;	/* decode */
    if (bcomp >= max_comp && exp_acc < dark_exp) {
// dark
    if (exp_acc < very_dark_exp) {
// very dark
    if (sd.exposure_status == EXPOSURE_VERY_DARK)
    ++sd.exposure_count;
    else {
    sd.exposure_status =
    EXPOSURE_VERY_DARK;
    sd.exposure_count = 1;
    }
    } else {
// just dark
    if (sd.exposure_status == EXPOSURE_DARK)
    ++sd.exposure_count;
    else {
    sd.exposure_status = EXPOSURE_DARK;
    sd.exposure_count = 1;
    }
    }
    } else if (old_exposure <= LOW_EXP || exp_acc > light_exp) {
// light
    if (old_exposure <= VERY_LOW_EXP) {
// very light
    if (sd.exposure_status == EXPOSURE_VERY_LIGHT)
    ++sd.exposure_count;
    else {
    sd.exposure_status =
    EXPOSURE_VERY_LIGHT;
    sd.exposure_count = 1;
    }
    } else {
// just light
    if (sd.exposure_status == EXPOSURE_LIGHT)
    ++sd.exposure_count;
    else {
    sd.exposure_status = EXPOSURE_LIGHT;
    sd.exposure_count = 1;
    }
    }
    } else {
// not dark or light
    sd.exposure_status = EXPOSURE_NORMAL;
    }
    } else {
// Flicker control off
    if (old_exposure >= MAX_EXP && exp_acc < dark_exp) {
// dark
    if (exp_acc < very_dark_exp) {
// very dark
    if (sd.exposure_status == EXPOSURE_VERY_DARK)
    ++sd.exposure_count;
    else {
    sd.exposure_status =
    EXPOSURE_VERY_DARK;
    sd.exposure_count = 1;
    }
    } else {
// just dark
    if (sd.exposure_status == EXPOSURE_DARK)
    ++sd.exposure_count;
    else {
    sd.exposure_status = EXPOSURE_DARK;
    sd.exposure_count = 1;
    }
    }
    } else if (old_exposure <= LOW_EXP || exp_acc > light_exp) {
// light
    if (old_exposure <= VERY_LOW_EXP) {
// very light
    if (sd.exposure_status == EXPOSURE_VERY_LIGHT)
    ++sd.exposure_count;
    else {
    sd.exposure_status =
    EXPOSURE_VERY_LIGHT;
    sd.exposure_count = 1;
    }
    } else {
// just light
    if (sd.exposure_status == EXPOSURE_LIGHT)
    ++sd.exposure_count;
    else {
    sd.exposure_status = EXPOSURE_LIGHT;
    sd.exposure_count = 1;
    }
    }
    } else {
// not dark or light
    sd.exposure_status = EXPOSURE_NORMAL;
    }
    }
    framerate = atomic_read(&sd.fps);
    if (framerate > 30 || framerate < 1)
    framerate = 1;
    if (!sd.params.flickerControl.disabled) {
// Flicker control on
    if ((sd.exposure_status == EXPOSURE_VERY_DARK ||
    sd.exposure_status == EXPOSURE_DARK) &&
    sd.exposure_count >= DARK_TIME * framerate &&
    sd.params.sensorFps.divisor < 2) {
// dark for too long
    ++sd.params.sensorFps.divisor;
    setfps = 1;
    sd.params.flickerControl.coarseJump =
    flicker_jumps[sd.mainsFreq]
    [sd.params.sensorFps.baserate]
    [sd.params.sensorFps.divisor];
    setflicker = 1;
    new_exposure = sd.params.flickerControl.coarseJump-1;
    while (new_exposure < old_exposure / 2)
    new_exposure +=
    sd.params.flickerControl.coarseJump;
    sd.params.exposure.coarseExpLo = new_exposure & 0xff;
    sd.params.exposure.coarseExpHi = new_exposure >> 8;
    setexp = 1;
    sd.exposure_status = EXPOSURE_NORMAL;
    gspca_dbg(gspca_dev, D_CONF, "Automatically decreasing sensor_fps\n");
    } else if ((sd.exposure_status == EXPOSURE_VERY_LIGHT ||
    sd.exposure_status == EXPOSURE_LIGHT) &&
    sd.exposure_count >= LIGHT_TIME * framerate &&
    sd.params.sensorFps.divisor > 0) {
// light for too long
    int max_exp = FIRMWARE_VERSION(1, 2) ? MAX_EXP_102 :
    MAX_EXP;
    --sd.params.sensorFps.divisor;
    setfps = 1;
    sd.params.flickerControl.coarseJump =
    flicker_jumps[sd.mainsFreq]
    [sd.params.sensorFps.baserate]
    [sd.params.sensorFps.divisor];
    setflicker = 1;
    new_exposure = sd.params.flickerControl.coarseJump-1;
    while (new_exposure < 2 * old_exposure &&
    new_exposure +
    sd.params.flickerControl.coarseJump < max_exp)
    new_exposure +=
    sd.params.flickerControl.coarseJump;
    sd.params.exposure.coarseExpLo = new_exposure & 0xff;
    sd.params.exposure.coarseExpHi = new_exposure >> 8;
    setexp = 1;
    sd.exposure_status = EXPOSURE_NORMAL;
    gspca_dbg(gspca_dev, D_CONF, "Automatically increasing sensor_fps\n");
    }
    } else {
// Flicker control off
    if ((sd.exposure_status == EXPOSURE_VERY_DARK ||
    sd.exposure_status == EXPOSURE_DARK) &&
    sd.exposure_count >= DARK_TIME * framerate &&
    sd.params.sensorFps.divisor < 2) {
// dark for too long
    ++sd.params.sensorFps.divisor;
    setfps = 1;
    if (sd.params.exposure.gain > 0) {
    --sd.params.exposure.gain;
    setexp = 1;
    }
    sd.exposure_status = EXPOSURE_NORMAL;
    gspca_dbg(gspca_dev, D_CONF, "Automatically decreasing sensor_fps\n");
    } else if ((sd.exposure_status == EXPOSURE_VERY_LIGHT ||
    sd.exposure_status == EXPOSURE_LIGHT) &&
    sd.exposure_count >= LIGHT_TIME * framerate &&
    sd.params.sensorFps.divisor > 0) {
// light for too long
    --sd.params.sensorFps.divisor;
    setfps = 1;
    if (sd.params.exposure.gain <
    sd.params.exposure.gainMode - 1) {
    ++sd.params.exposure.gain;
    setexp = 1;
    }
    sd.exposure_status = EXPOSURE_NORMAL;
    gspca_dbg(gspca_dev, D_CONF, "Automatically increasing sensor_fps\n");
    }
    }
    if (setexp)
    command_setexposure(gspca_dev);
    if (setfps)
    command_setsensorfps(gspca_dev);
    if (setflicker)
    command_setflickerctrl(gspca_dev);
    }
// -----------------------------------------------------------------
// if flicker is switched off, this function switches it back on.It checks,
    however, that conditions are suitable before restarting it.
    This should only be called for firmware version 1.2.
    It also adjust the colour balance when an exposure step is detected - as
    long as flicker is running
//
#[no_mangle]
unsafe extern "C" fn restart_flicker(gspca_dev: *mut gspca_dev) {
    static void restart_flicker(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int cam_exposure, old_exp;
    if (!FIRMWARE_VERSION(1, 2))
    return;
    cam_exposure = atomic_read(&sd.cam_exposure);
    if (sd.params.flickerControl.flickerMode == 0 ||
    cam_exposure == 0)
    return;
    old_exp = sd.params.exposure.coarseExpLo +
    sd.params.exposure.coarseExpHi*256;
//
    see how far away camera exposure is from a valid
    flicker exposure value
//
    cam_exposure %= sd.params.flickerControl.coarseJump;
    if (!sd.params.flickerControl.disabled &&
    cam_exposure <= sd.params.flickerControl.coarseJump - 3) {
// Flicker control auto-disabled
    sd.params.flickerControl.disabled = 1;
    }
    if (sd.params.flickerControl.disabled &&
    old_exp > sd.params.flickerControl.coarseJump +
    ROUND_UP_EXP_FOR_FLICKER) {
// exposure is now high enough to switch
    flicker control back on */
    set_flicker(gspca_dev, 1, 1);
    }
    }
// this function is called at probe time
    static int sd_config(struct gspca_dev *gspca_dev,
    const struct usb_device_id *id)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    struct cam *cam;
    sd.mainsFreq = FREQ_DEF == V4L2_CID_POWER_LINE_FREQUENCY_60HZ;
    reset_camera_params(gspca_dev);
    gspca_dbg(gspca_dev, D_PROBE, "cpia CPiA camera detected (vid/pid 0x%04X:0x%04X)\n",
    id.idVendor, id.idProduct);
    cam = &gspca_dev.cam;
    cam.cam_mode = mode;
    cam.nmodes = ARRAY_SIZE(mode);
    goto_low_power(gspca_dev);
// Check the firmware version.
    sd.params.version.firmwareVersion = 0;
    get_version_information(gspca_dev);
    if (sd.params.version.firmwareVersion != 1) {
    gspca_err(gspca_dev, "only firmware version 1 is supported (got: %d)\n",
    sd.params.version.firmwareVersion);
    return -ENODEV;
    }
// A bug in firmware 1-02 limits gainMode to 2
    if (sd.params.version.firmwareRevision <= 2 &&
    sd.params.exposure.gainMode > 2) {
    sd.params.exposure.gainMode = 2;
    }
// set QX3 detected flag
    sd.params.qx3.qx3_detected = (sd.params.pnpID.vendor == 0x0813 &&
    sd.params.pnpID.product == 0x0001);
    return 0;
    }
// -- start the camera --
#[no_mangle]
unsafe extern "C" fn sd_start(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_start(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int priv, ret;
// Start the camera in low power mode
    if (goto_low_power(gspca_dev)) {
    if (sd.params.status.systemState != WARM_BOOT_STATE) {
    gspca_err(gspca_dev, "unexpected systemstate: %02x\n",
    sd.params.status.systemState);
    printstatus(gspca_dev, &sd.params);
    return -ENODEV;
    }
// FIXME: this is just dirty trial and error
    ret = goto_high_power(gspca_dev);
    if (ret)
    return ret;
    ret = do_command(gspca_dev, CPIA_COMMAND_DiscardFrame,
    0, 0, 0, 0);
    if (ret)
    return ret;
    ret = goto_low_power(gspca_dev);
    if (ret)
    return ret;
    }
// procedure described in developer's guide p3-28
// Check the firmware version.
    sd.params.version.firmwareVersion = 0;
    get_version_information(gspca_dev);
// The fatal error checking should be done after
// the camera powers up (developer's guide p 3-38)
// Set streamState before transition to high power to avoid bug
// in firmware 1-02
    ret = do_command(gspca_dev, CPIA_COMMAND_ModifyCameraStatus,
    STREAMSTATE, 0, STREAM_NOT_READY, 0);
    if (ret)
    return ret;
// GotoHiPower
    ret = goto_high_power(gspca_dev);
    if (ret)
    return ret;
// Check the camera status
    ret = do_command(gspca_dev, CPIA_COMMAND_GetCameraStatus, 0, 0, 0, 0);
    if (ret)
    return ret;
    if (sd.params.status.fatalError) {
    gspca_err(gspca_dev, "fatal_error: %04x, vp_status: %04x\n",
    sd.params.status.fatalError,
    sd.params.status.vpStatus);
    return -EIO;
    }
// VPVersion can't be retrieved before the camera is in HiPower,
// so get it here instead of in get_version_information.
    ret = do_command(gspca_dev, CPIA_COMMAND_GetVPVersion, 0, 0, 0, 0);
    if (ret)
    return ret;
// Determine video mode settings
    sd.params.streamStartLine = 120;
    priv = gspca_dev.cam.cam_mode[gspca_dev.curr_mode].priv;
    if (priv & 0x01) { /* crop */
    sd.params.roi.colStart = 2;
    sd.params.roi.rowStart = 6;
    } else {
    sd.params.roi.colStart = 0;
    sd.params.roi.rowStart = 0;
    }
    if (priv & 0x02) { /* quarter */
    sd.params.format.videoSize = VIDEOSIZE_QCIF;
    sd.params.roi.colStart /= 2;
    sd.params.roi.rowStart /= 2;
    sd.params.streamStartLine /= 2;
    } else
    sd.params.format.videoSize = VIDEOSIZE_CIF;
    sd.params.roi.colEnd = sd.params.roi.colStart +
    (gspca_dev.pixfmt.width >> 3);
    sd.params.roi.rowEnd = sd.params.roi.rowStart +
    (gspca_dev.pixfmt.height >> 2);
// And now set the camera to a known state
    ret = do_command(gspca_dev, CPIA_COMMAND_SetGrabMode,
    CPIA_GRAB_CONTINEOUS, 0, 0, 0);
    if (ret)
    return ret;
// We start with compression disabled, as we need one uncompressed
    frame to handle later compressed frames */
    ret = do_command(gspca_dev, CPIA_COMMAND_SetCompression,
    CPIA_COMPRESSION_NONE,
    NO_DECIMATION, 0, 0);
    if (ret)
    return ret;
    ret = command_setcompressiontarget(gspca_dev);
    if (ret)
    return ret;
    ret = command_setcolourparams(gspca_dev);
    if (ret)
    return ret;
    ret = command_setformat(gspca_dev);
    if (ret)
    return ret;
    ret = command_setyuvtresh(gspca_dev);
    if (ret)
    return ret;
    ret = command_setecptiming(gspca_dev);
    if (ret)
    return ret;
    ret = command_setcompressionparams(gspca_dev);
    if (ret)
    return ret;
    ret = command_setexposure(gspca_dev);
    if (ret)
    return ret;
    ret = command_setcolourbalance(gspca_dev);
    if (ret)
    return ret;
    ret = command_setsensorfps(gspca_dev);
    if (ret)
    return ret;
    ret = command_setapcor(gspca_dev);
    if (ret)
    return ret;
    ret = command_setflickerctrl(gspca_dev);
    if (ret)
    return ret;
    ret = command_setvloffset(gspca_dev);
    if (ret)
    return ret;
// Start stream
    ret = command_resume(gspca_dev);
    if (ret)
    return ret;
// Wait 6 frames before turning compression on for the sensor to get
    all settings and AEC/ACB to settle */
    sd.first_frame = 6;
    sd.exposure_status = EXPOSURE_NORMAL;
    sd.exposure_count = 0;
    atomic_set(&sd.cam_exposure, 0);
    atomic_set(&sd.fps, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sd_stopN(gspca_dev: *mut gspca_dev) {
    static void sd_stopN(struct gspca_dev *gspca_dev)
    {
    let mut __maybe_unused: *mut sd sd = (struct sd *) gspca_dev;
    command_pause(gspca_dev);
// save camera state for later open (developers guide ch 3.5.3)
    save_camera_state(gspca_dev);
// GotoLoPower
    goto_low_power(gspca_dev);
// Update the camera status
    do_command(gspca_dev, CPIA_COMMAND_GetCameraStatus, 0, 0, 0, 0);

// If the last button state is pressed, release it now!
    if (sd.params.qx3.button) {
// The camera latch will hold the pressed state until we reset
    the latch, so we do not reset sd.params.qx3.button now, to
    avoid a false keypress being reported the next sd_start */
    input_report_key(gspca_dev.input_dev, KEY_CAMERA, 0);
    input_sync(gspca_dev.input_dev);
    }

    }
// this function is called at probe and resume time
#[no_mangle]
unsafe extern "C" fn sd_init(gspca_dev: *mut gspca_dev) -> c_int {
    static int sd_init(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
    int ret;
// Start / Stop the camera to make sure we are talking to
    a supported camera, and to get some information from it
    to print. */
    ret = sd_start(gspca_dev);
    if (ret)
    return ret;
// Ensure the QX3 illuminators' states are restored upon resume,
    or disable the illuminator controls, if this isn't a QX3 */
    if (sd.params.qx3.qx3_detected)
    command_setlights(gspca_dev);
    sd_stopN(gspca_dev);
    gspca_dbg(gspca_dev, D_PROBE, "CPIA Version:             %d.%02d (%d.%d)\n",
    sd.params.version.firmwareVersion,
    sd.params.version.firmwareRevision,
    sd.params.version.vcVersion,
    sd.params.version.vcRevision);
    gspca_dbg(gspca_dev, D_PROBE, "CPIA PnP-ID:              %04x:%04x:%04x",
    sd.params.pnpID.vendor, sd.params.pnpID.product,
    sd.params.pnpID.deviceRevision);
    gspca_dbg(gspca_dev, D_PROBE, "VP-Version:               %d.%d %04x",
    sd.params.vpVersion.vpVersion,
    sd.params.vpVersion.vpRevision,
    sd.params.vpVersion.cameraHeadID);
    return 0;
    }
    static void sd_pkt_scan(struct gspca_dev *gspca_dev,
    u8 *data,
    int len)
    {
    struct sd *sd = (struct sd *) gspca_dev;
// Check for SOF
    if (len >= 64 &&
    data[0] == MAGIC_0 && data[1] == MAGIC_1 &&
    data[16] == sd.params.format.videoSize &&
    data[17] == sd.params.format.subSample &&
    data[18] == sd.params.format.yuvOrder &&
    data[24] == sd.params.roi.colStart &&
    data[25] == sd.params.roi.colEnd &&
    data[26] == sd.params.roi.rowStart &&
    data[27] == sd.params.roi.rowEnd) {
    u8 *image;
    atomic_set(&sd.cam_exposure, data[39] * 2);
    atomic_set(&sd.fps, data[41]);
// Check for proper EOF for last frame
    image = gspca_dev.image;
    if (image != core::ptr::null_mut() &&
    gspca_dev.image_len > 4 &&
    image[gspca_dev.image_len - 4] == 0xff &&
    image[gspca_dev.image_len - 3] == 0xff &&
    image[gspca_dev.image_len - 2] == 0xff &&
    image[gspca_dev.image_len - 1] == 0xff)
    gspca_frame_add(gspca_dev, LAST_PACKET,
    core::ptr::null_mut(), 0);
    gspca_frame_add(gspca_dev, FIRST_PACKET, data, len);
    return;
    }
    gspca_frame_add(gspca_dev, INTER_PACKET, data, len);
    }
#[no_mangle]
unsafe extern "C" fn sd_dq_callback(gspca_dev: *mut gspca_dev) {
    static void sd_dq_callback(struct gspca_dev *gspca_dev)
    {
    struct sd *sd = (struct sd *) gspca_dev;
// Set the normal compression settings once we have captured a
    few uncompressed frames (and AEC has hopefully settled) */
    if (sd.first_frame) {
    sd.first_frame--;
    if (sd.first_frame == 0)
    command_setcompression(gspca_dev);
    }
// Switch flicker control back on if it got turned off
    restart_flicker(gspca_dev);
// If AEC is enabled, monitor the exposure and
    adjust the sensor frame rate if needed */
    if (sd.params.exposure.expMode == 2)
    monitor_exposure(gspca_dev);
// Update our knowledge of the camera state
    do_command(gspca_dev, CPIA_COMMAND_GetExposure, 0, 0, 0, 0);
    do_command(gspca_dev, CPIA_COMMAND_ReadMCPorts, 0, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn sd_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int sd_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct gspca_dev *gspca_dev =
    container_of(ctrl.handler, struct gspca_dev, ctrl_handler);
    struct sd *sd = (struct sd *)gspca_dev;
    gspca_dev.usb_err = 0;
    if (!gspca_dev.streaming && ctrl.id != V4L2_CID_POWER_LINE_FREQUENCY)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    sd.params.colourParams.brightness = ctrl.val;
    sd.params.flickerControl.allowableOverExposure =
    find_over_exposure(sd.params.colourParams.brightness);
    gspca_dev.usb_err = command_setcolourparams(gspca_dev);
    if (!gspca_dev.usb_err)
    gspca_dev.usb_err = command_setflickerctrl(gspca_dev);
    break;
    case V4L2_CID_CONTRAST:
    sd.params.colourParams.contrast = ctrl.val;
    gspca_dev.usb_err = command_setcolourparams(gspca_dev);
    break;
    case V4L2_CID_SATURATION:
    sd.params.colourParams.saturation = ctrl.val;
    gspca_dev.usb_err = command_setcolourparams(gspca_dev);
    break;
    case V4L2_CID_POWER_LINE_FREQUENCY:
    sd.mainsFreq = ctrl.val == V4L2_CID_POWER_LINE_FREQUENCY_60HZ;
    sd.params.flickerControl.coarseJump =
    flicker_jumps[sd.mainsFreq]
    [sd.params.sensorFps.baserate]
    [sd.params.sensorFps.divisor];
    gspca_dev.usb_err = set_flicker(gspca_dev,
    ctrl.val != V4L2_CID_POWER_LINE_FREQUENCY_DISABLED,
    gspca_dev.streaming);
    break;
    case V4L2_CID_ILLUMINATORS_1:
    sd.params.qx3.bottomlight = ctrl.val;
    gspca_dev.usb_err = command_setlights(gspca_dev);
    break;
    case V4L2_CID_ILLUMINATORS_2:
    sd.params.qx3.toplight = ctrl.val;
    gspca_dev.usb_err = command_setlights(gspca_dev);
    break;
    case CPIA1_CID_COMP_TARGET:
    sd.params.compressionTarget.frTargeting = ctrl.val;
    gspca_dev.usb_err = command_setcompressiontarget(gspca_dev);
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
    static const char * const comp_target_menu[] = {
    "Quality",
    "Framerate",
    core::ptr::null_mut()
    };
    static const struct v4l2_ctrl_config comp_target = {
    .ops = &sd_ctrl_ops,
    .id = CPIA1_CID_COMP_TARGET,
    .type = V4L2_CTRL_TYPE_MENU,
    .name = "Compression Target",
    .qmenu = comp_target_menu,
    .max = 1,
    .def = COMP_TARGET_DEF,
    };
    gspca_dev.vdev.ctrl_handler = hdl;
    v4l2_ctrl_handler_init(hdl, 7);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_BRIGHTNESS, 0, 100, 1, BRIGHTNESS_DEF);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_CONTRAST, 0, 96, 8, CONTRAST_DEF);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_SATURATION, 0, 100, 1, SATURATION_DEF);
    sd.freq = v4l2_ctrl_new_std_menu(hdl, &sd_ctrl_ops,
    V4L2_CID_POWER_LINE_FREQUENCY,
    V4L2_CID_POWER_LINE_FREQUENCY_60HZ, 0,
    FREQ_DEF);
    if (sd.params.qx3.qx3_detected) {
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_ILLUMINATORS_1, 0, 1, 1,
    ILLUMINATORS_1_DEF);
    v4l2_ctrl_new_std(hdl, &sd_ctrl_ops,
    V4L2_CID_ILLUMINATORS_2, 0, 1, 1,
    ILLUMINATORS_2_DEF);
    }
    v4l2_ctrl_new_custom(hdl, &comp_target, core::ptr::null_mut());
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
    .dq_callback = sd_dq_callback,
    .pkt_scan = sd_pkt_scan,

    .other_input = 1,

    };
// -- module initialisation --
    static const struct usb_device_id device_table[] = {
    {USB_DEVICE(0x0553, 0x0002)},
    {USB_DEVICE(0x0813, 0x0001)},
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
