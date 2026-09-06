//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/adv7180.c
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
// adv7180.c Analog Devices ADV7180 video decoder driver
// Copyright (c) 2009 Intel Corporation
// Copyright (C) 2013 Cogent Embedded, Inc.
// Copyright (C) 2013 Renesas Solutions Corp.
//

pub const ADV7180_STD_AD_PAL_BG_NTSC_J_SECAM: c_uint = 0x0;
pub const ADV7180_STD_AD_PAL_BG_NTSC_J_SECAM_PED: c_uint = 0x1;
pub const ADV7180_STD_AD_PAL_N_NTSC_J_SECAM: c_uint = 0x2;
pub const ADV7180_STD_AD_PAL_N_NTSC_M_SECAM: c_uint = 0x3;
pub const ADV7180_STD_NTSC_J: c_uint = 0x4;
pub const ADV7180_STD_NTSC_M: c_uint = 0x5;
pub const ADV7180_STD_PAL60: c_uint = 0x6;
pub const ADV7180_STD_NTSC_443: c_uint = 0x7;
pub const ADV7180_STD_PAL_BG: c_uint = 0x8;
pub const ADV7180_STD_PAL_N: c_uint = 0x9;
pub const ADV7180_STD_PAL_M: c_uint = 0xa;
pub const ADV7180_STD_PAL_M_PED: c_uint = 0xb;
pub const ADV7180_STD_PAL_COMB_N: c_uint = 0xc;
pub const ADV7180_STD_PAL_COMB_N_PED: c_uint = 0xd;
pub const ADV7180_STD_PAL_SECAM: c_uint = 0xe;
pub const ADV7180_STD_PAL_SECAM_PED: c_uint = 0xf;
pub const ADV7180_REG_INPUT_CONTROL: c_uint = 0x0000;
pub const ADV7180_INPUT_CONTROL_INSEL_MASK: c_uint = 0x0f;
pub const ADV7182_REG_INPUT_VIDSEL: c_uint = 0x0002;

pub const ADV7180_REG_OUTPUT_CONTROL: c_uint = 0x0003;
pub const ADV7180_REG_EXTENDED_OUTPUT_CONTROL: c_uint = 0x0004;
pub const ADV7180_EXTENDED_OUTPUT_CONTROL_NTSCDIS: c_uint = 0xC5;
pub const ADV7180_REG_AUTODETECT_ENABLE: c_uint = 0x0007;
pub const ADV7180_AUTODETECT_DEFAULT: c_uint = 0x7f;
// Contrast
pub const ADV7180_REG_CON: c_uint = 0x0008	/*Unsigned */;
pub const ADV7180_CON_MIN: c_int = 0;
pub const ADV7180_CON_DEF: c_int = 128;
pub const ADV7180_CON_MAX: c_int = 255;
// Brightness
pub const ADV7180_REG_BRI: c_uint = 0x000a	/*Signed */;

pub const ADV7180_BRI_DEF: c_int = 0;
pub const ADV7180_BRI_MAX: c_int = 127;
// Hue
pub const ADV7180_REG_HUE: c_uint = 0x000b	/*Signed, inverted */;

pub const ADV7180_HUE_DEF: c_int = 0;
pub const ADV7180_HUE_MAX: c_int = 128;
pub const ADV7180_REG_DEF_VALUE_Y: c_uint = 0x000c;
pub const ADV7180_DEF_VAL_EN: c_uint = 0x1;
pub const ADV7180_DEF_VAL_AUTO_EN: c_uint = 0x2;
pub const ADV7180_REG_CTRL: c_uint = 0x000e;
pub const ADV7180_CTRL_IRQ_SPACE: c_uint = 0x20;
pub const ADV7180_REG_PWR_MAN: c_uint = 0x0f;
pub const ADV7180_PWR_MAN_ON: c_uint = 0x04;
pub const ADV7180_PWR_MAN_OFF: c_uint = 0x24;
pub const ADV7180_PWR_MAN_RES: c_uint = 0x80;
pub const ADV7180_REG_STATUS1: c_uint = 0x0010;
pub const ADV7180_STATUS1_IN_LOCK: c_uint = 0x01;
pub const ADV7180_STATUS1_AUTOD_MASK: c_uint = 0x70;
pub const ADV7180_STATUS1_AUTOD_NTSM_M_J: c_uint = 0x00;
pub const ADV7180_STATUS1_AUTOD_NTSC_4_43: c_uint = 0x10;
pub const ADV7180_STATUS1_AUTOD_PAL_M: c_uint = 0x20;
pub const ADV7180_STATUS1_AUTOD_PAL_60: c_uint = 0x30;
pub const ADV7180_STATUS1_AUTOD_PAL_B_G: c_uint = 0x40;
pub const ADV7180_STATUS1_AUTOD_SECAM: c_uint = 0x50;
pub const ADV7180_STATUS1_AUTOD_PAL_COMB: c_uint = 0x60;
pub const ADV7180_STATUS1_AUTOD_SECAM_525: c_uint = 0x70;
pub const ADV7180_REG_IDENT: c_uint = 0x0011;
pub const ADV7180_ID_7180: c_uint = 0x18;
pub const ADV7180_REG_STATUS3: c_uint = 0x0013;
pub const ADV7180_REG_ANALOG_CLAMP_CTL: c_uint = 0x0014;
pub const ADV7180_REG_SHAP_FILTER_CTL_1: c_uint = 0x0017;
pub const ADV7180_REG_CTRL_2: c_uint = 0x001d;
pub const ADV7180_REG_VSYNC_FIELD_CTL_1: c_uint = 0x0031;
pub const ADV7180_VSYNC_FIELD_CTL_1_NEWAV: c_uint = 0x12;
pub const ADV7180_REG_MANUAL_WIN_CTL_1: c_uint = 0x003d;
pub const ADV7180_REG_MANUAL_WIN_CTL_2: c_uint = 0x003e;
pub const ADV7180_REG_MANUAL_WIN_CTL_3: c_uint = 0x003f;
pub const ADV7180_REG_LOCK_CNT: c_uint = 0x0051;
pub const ADV7180_REG_CVBS_TRIM: c_uint = 0x0052;
pub const ADV7180_REG_CLAMP_ADJ: c_uint = 0x005a;
pub const ADV7180_REG_RES_CIR: c_uint = 0x005f;
pub const ADV7180_REG_DIFF_MODE: c_uint = 0x0060;
pub const ADV7180_REG_ICONF1: c_uint = 0x2040;
pub const ADV7180_ICONF1_ACTIVE_LOW: c_uint = 0x01;
pub const ADV7180_ICONF1_PSYNC_ONLY: c_uint = 0x10;
pub const ADV7180_ICONF1_ACTIVE_TO_CLR: c_uint = 0xC0;
// Saturation
pub const ADV7180_REG_SD_SAT_CB: c_uint = 0x00e3	/*Unsigned */;
pub const ADV7180_REG_SD_SAT_CR: c_uint = 0x00e4	/*Unsigned */;
pub const ADV7180_SAT_MIN: c_int = 0;
pub const ADV7180_SAT_DEF: c_int = 128;
pub const ADV7180_SAT_MAX: c_int = 255;
pub const ADV7180_IRQ1_LOCK: c_uint = 0x01;
pub const ADV7180_IRQ1_UNLOCK: c_uint = 0x02;
pub const ADV7180_REG_ISR1: c_uint = 0x2042;
pub const ADV7180_REG_ICR1: c_uint = 0x2043;
pub const ADV7180_REG_IMR1: c_uint = 0x2044;
pub const ADV7180_REG_IMR2: c_uint = 0x2048;
pub const ADV7180_IRQ3_AD_CHANGE: c_uint = 0x08;
pub const ADV7180_REG_ISR3: c_uint = 0x204A;
pub const ADV7180_REG_ICR3: c_uint = 0x204B;
pub const ADV7180_REG_IMR3: c_uint = 0x204C;
pub const ADV7180_REG_IMR4: c_uint = 0x2050;
pub const ADV7180_REG_NTSC_V_BIT_END: c_uint = 0x00E6;
pub const ADV7180_NTSC_V_BIT_END_MANUAL_NVEND: c_uint = 0x4F;
pub const ADV7180_REG_VPP_SLAVE_ADDR: c_uint = 0xFD;
pub const ADV7180_REG_CSI_SLAVE_ADDR: c_uint = 0xFE;
pub const ADV7180_REG_ACE_CTRL1: c_uint = 0x4080;
pub const ADV7180_REG_ACE_CTRL5: c_uint = 0x4084;
pub const ADV7180_REG_FLCONTROL: c_uint = 0x40e0;
pub const ADV7180_FLCONTROL_FL_ENABLE: c_uint = 0x1;
pub const ADV7180_REG_RST_CLAMP: c_uint = 0x809c;
pub const ADV7180_REG_AGC_ADJ1: c_uint = 0x80b6;
pub const ADV7180_REG_AGC_ADJ2: c_uint = 0x80c0;
pub const ADV7180_CSI_REG_PWRDN: c_uint = 0x00;
pub const ADV7180_CSI_PWRDN: c_uint = 0x80;
pub const ADV7180_INPUT_CVBS_AIN1: c_uint = 0x00;
pub const ADV7180_INPUT_CVBS_AIN2: c_uint = 0x01;
pub const ADV7180_INPUT_CVBS_AIN3: c_uint = 0x02;
pub const ADV7180_INPUT_CVBS_AIN4: c_uint = 0x03;
pub const ADV7180_INPUT_CVBS_AIN5: c_uint = 0x04;
pub const ADV7180_INPUT_CVBS_AIN6: c_uint = 0x05;
pub const ADV7180_INPUT_SVIDEO_AIN1_AIN2: c_uint = 0x06;
pub const ADV7180_INPUT_SVIDEO_AIN3_AIN4: c_uint = 0x07;
pub const ADV7180_INPUT_SVIDEO_AIN5_AIN6: c_uint = 0x08;
pub const ADV7180_INPUT_YPRPB_AIN1_AIN2_AIN3: c_uint = 0x09;
pub const ADV7180_INPUT_YPRPB_AIN4_AIN5_AIN6: c_uint = 0x0a;
pub const ADV7182_INPUT_CVBS_AIN1: c_uint = 0x00;
pub const ADV7182_INPUT_CVBS_AIN2: c_uint = 0x01;
pub const ADV7182_INPUT_CVBS_AIN3: c_uint = 0x02;
pub const ADV7182_INPUT_CVBS_AIN4: c_uint = 0x03;
pub const ADV7182_INPUT_CVBS_AIN5: c_uint = 0x04;
pub const ADV7182_INPUT_CVBS_AIN6: c_uint = 0x05;
pub const ADV7182_INPUT_CVBS_AIN7: c_uint = 0x06;
pub const ADV7182_INPUT_CVBS_AIN8: c_uint = 0x07;
pub const ADV7182_INPUT_SVIDEO_AIN1_AIN2: c_uint = 0x08;
pub const ADV7182_INPUT_SVIDEO_AIN3_AIN4: c_uint = 0x09;
pub const ADV7182_INPUT_SVIDEO_AIN5_AIN6: c_uint = 0x0a;
pub const ADV7182_INPUT_SVIDEO_AIN7_AIN8: c_uint = 0x0b;
pub const ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3: c_uint = 0x0c;
pub const ADV7182_INPUT_YPRPB_AIN4_AIN5_AIN6: c_uint = 0x0d;
pub const ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2: c_uint = 0x0e;
pub const ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4: c_uint = 0x0f;
pub const ADV7182_INPUT_DIFF_CVBS_AIN5_AIN6: c_uint = 0x10;
pub const ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8: c_uint = 0x11;
pub const ADV7180_DEFAULT_CSI_I2C_ADDR: c_uint = 0x44;
pub const ADV7180_DEFAULT_VPP_I2C_ADDR: c_uint = 0x42;

// Initial number of frames to skip to avoid possible garbage
pub const ADV7180_NUM_OF_SKIP_FRAMES: c_int = 2;
    struct adv7180_state;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7180_chip_info {
    pub flags: c_uint,
    pub valid_input_mask: c_uint,
    pub std): *mut *mut *mut int (set_std)(struct adv7180_state st, unsigned int,
    pub input): *mut *mut *mut int (select_input)(struct adv7180_state st, unsigned int,
    pub state): *mut *mut int (init)(struct adv7180_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv7180_state {
    pub ctrl_hdl: v4l2_ctrl_handler,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub /: *mut *mut mutex mutex; / mutual excl. when accessing chip,
    pub irq: c_int,
    pub pwdn_gpio: *mut gpio_desc,
    pub rst_gpio: *mut gpio_desc,
    pub curr_norm: v4l2_std_id,
    pub streaming: bool,
    pub input: u8,
    pub client: *mut i2c_client,
    pub register_page: c_uint,
    pub csi_client: *mut i2c_client,
    pub vpp_client: *mut i2c_client,
    pub chip_info: *const adv7180_chip_info,
    pub field: enum v4l2_field,
    pub force_bt656_4: bool,
}

    struct adv7180_state,	\
    ctrl_hdl).sd)
#[no_mangle]
unsafe extern "C" fn adv7180_select_page(state: *mut adv7180_state, page: c_uint) -> c_int {
    static int adv7180_select_page(struct adv7180_state *state, unsigned int page)
    {
    if (state.register_page != page) {
    i2c_smbus_write_byte_data(state.client, ADV7180_REG_CTRL,
    page);
    state.register_page = page;
    }
    return 0;
    }
    static int adv7180_write(struct adv7180_state *state, unsigned int reg,
    unsigned int value)
    {
    lockdep_assert_held(&state.mutex);
    adv7180_select_page(state, reg >> 8);
    return i2c_smbus_write_byte_data(state.client, reg & 0xff, value);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_read(state: *mut adv7180_state, reg: c_uint) -> c_int {
    static int adv7180_read(struct adv7180_state *state, unsigned int reg)
    {
    lockdep_assert_held(&state.mutex);
    adv7180_select_page(state, reg >> 8);
    return i2c_smbus_read_byte_data(state.client, reg & 0xff);
    }
    static int adv7180_csi_write(struct adv7180_state *state, unsigned int reg,
    unsigned int value)
    {
    return i2c_smbus_write_byte_data(state.csi_client, reg, value);
    }
    static int adv7180_set_video_standard(struct adv7180_state *state,
    unsigned int std)
    {
    return state.chip_info.set_std(state, std);
    }
    static int adv7180_vpp_write(struct adv7180_state *state, unsigned int reg,
    unsigned int value)
    {
    return i2c_smbus_write_byte_data(state.vpp_client, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_set_power(state: *mut adv7180_state, on: bool) -> c_int {
    static int adv7180_set_power(struct adv7180_state *state, bool on)
    {
    u8 val;
    int ret;
    if (on)
    val = ADV7180_PWR_MAN_ON;
    else
    val = ADV7180_PWR_MAN_OFF;
    ret = adv7180_write(state, ADV7180_REG_PWR_MAN, val);
    if (ret)
    return ret;
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    if (on) {
    adv7180_csi_write(state, 0xDE, 0x02);
    adv7180_csi_write(state, 0xD2, 0xF7);
    adv7180_csi_write(state, 0xD8, 0x65);
    adv7180_csi_write(state, 0xE0, 0x09);
    adv7180_csi_write(state, 0x2C, 0x00);
    if (state.field == V4L2_FIELD_NONE)
    adv7180_csi_write(state, 0x1D, 0x80);
    adv7180_csi_write(state, 0x00, 0x00);
    } else {
    adv7180_csi_write(state, 0x00, 0x80);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_std_to_v4l2(status1: u8) -> v4l2_std_id {
    static v4l2_std_id adv7180_std_to_v4l2(u8 status1)
    {
// in case V4L2_IN_ST_NO_SIGNAL
    if (!(status1 & ADV7180_STATUS1_IN_LOCK))
    return V4L2_STD_UNKNOWN;
    switch (status1 & ADV7180_STATUS1_AUTOD_MASK) {
    case ADV7180_STATUS1_AUTOD_NTSM_M_J:
    return V4L2_STD_NTSC;
    case ADV7180_STATUS1_AUTOD_NTSC_4_43:
    return V4L2_STD_NTSC_443;
    case ADV7180_STATUS1_AUTOD_PAL_M:
    return V4L2_STD_PAL_M;
    case ADV7180_STATUS1_AUTOD_PAL_60:
    return V4L2_STD_PAL_60;
    case ADV7180_STATUS1_AUTOD_PAL_B_G:
    return V4L2_STD_PAL;
    case ADV7180_STATUS1_AUTOD_SECAM:
    return V4L2_STD_SECAM;
    case ADV7180_STATUS1_AUTOD_PAL_COMB:
    return V4L2_STD_PAL_Nc | V4L2_STD_PAL_N;
    case ADV7180_STATUS1_AUTOD_SECAM_525:
    return V4L2_STD_SECAM;
    default:
    return V4L2_STD_UNKNOWN;
    }
    }
#[no_mangle]
unsafe extern "C" fn v4l2_std_to_adv7180(std: v4l2_std_id) -> c_int {
    static int v4l2_std_to_adv7180(v4l2_std_id std)
    {
    if (std == V4L2_STD_PAL_60)
    return ADV7180_STD_PAL60;
    if (std == V4L2_STD_NTSC_443)
    return ADV7180_STD_NTSC_443;
    if (std == V4L2_STD_PAL_N)
    return ADV7180_STD_PAL_N;
    if (std == V4L2_STD_PAL_M)
    return ADV7180_STD_PAL_M;
    if (std == V4L2_STD_PAL_Nc)
    return ADV7180_STD_PAL_COMB_N;
    if (std & V4L2_STD_PAL)
    return ADV7180_STD_PAL_BG;
    if (std & V4L2_STD_NTSC)
    return ADV7180_STD_NTSC_M;
    if (std & V4L2_STD_SECAM)
    return ADV7180_STD_PAL_SECAM;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_status_to_v4l2(status1: u8) -> u32 {
    static u32 adv7180_status_to_v4l2(u8 status1)
    {
    if (!(status1 & ADV7180_STATUS1_IN_LOCK))
    return V4L2_IN_ST_NO_SIGNAL;
    return 0;
    }
    static int __adv7180_status(struct adv7180_state *state, u32 *status,
    v4l2_std_id *std)
    {
    let mut status1: c_int = adv7180_read(state, ADV7180_REG_STATUS1);
    if (status1 < 0)
    return status1;
    if (status)
// status = adv7180_status_to_v4l2(status1);
    if (std)
// std = adv7180_std_to_v4l2(status1);
    return 0;
    }
    static inline struct adv7180_state *to_state(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct adv7180_state, sd);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_querystd(sd: *mut v4l2_subdev, std: *mut v4l2_std_id) -> c_int {
    static int adv7180_querystd(struct v4l2_subdev *sd, v4l2_std_id *std)
    {
    struct adv7180_state *state = to_state(sd);
    int ret;
    guard(mutex)(&state.mutex);
//
// We can't sample the standard if the device is streaming as that would
// interfere with the capture session as the VID_SEL reg is touched.
//
    if (state.streaming)
    return -EBUSY;
// Set the standard to autodetect PAL B/G/H/I/D, NTSC J or SECAM
    ret = adv7180_set_video_standard(state,
    ADV7180_STD_AD_PAL_BG_NTSC_J_SECAM);
    if (ret)
    return ret;
// Allow some time for the autodetection to run.
    msleep(100);
    return __adv7180_status(state, core::ptr::null_mut(), std);
    }
    static int adv7180_s_routing(struct v4l2_subdev *sd, u32 input,
    u32 output, u32 config)
    {
    struct adv7180_state *state = to_state(sd);
    let mut ret: c_int = mutex_lock_interruptible(&state.mutex);
    if (ret)
    return ret;
    if (input > 31 || !(BIT(input) & state.chip_info.valid_input_mask)) {
    ret = -EINVAL;
    goto out;
    }
    ret = state.chip_info.select_input(state, input);
    if (ret == 0)
    state.input = input;
    out:
    mutex_unlock(&state.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_g_input_status(sd: *mut v4l2_subdev, status: *mut u32) -> c_int {
    static int adv7180_g_input_status(struct v4l2_subdev *sd, u32 *status)
    {
    struct adv7180_state *state = to_state(sd);
    let mut ret: c_int = mutex_lock_interruptible(&state.mutex);
    if (ret)
    return ret;
    ret = __adv7180_status(state, status, core::ptr::null_mut());
    mutex_unlock(&state.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_program_std(state: *mut adv7180_state) -> c_int {
    static int adv7180_program_std(struct adv7180_state *state)
    {
    int ret;
    ret = v4l2_std_to_adv7180(state.curr_norm);
    if (ret < 0)
    return ret;
    ret = adv7180_set_video_standard(state, ret);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_s_std(sd: *mut v4l2_subdev, std: v4l2_std_id) -> c_int {
    static int adv7180_s_std(struct v4l2_subdev *sd, v4l2_std_id std)
    {
    struct adv7180_state *state = to_state(sd);
    int ret;
    guard(mutex)(&state.mutex);
// Make sure we can support this std
    ret = v4l2_std_to_adv7180(std);
    if (ret < 0)
    return ret;
    state.curr_norm = std;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_g_std(sd: *mut v4l2_subdev, norm: *mut v4l2_std_id) -> c_int {
    static int adv7180_g_std(struct v4l2_subdev *sd, v4l2_std_id *norm)
    {
    struct adv7180_state *state = to_state(sd);
// norm = state->curr_norm;
    return 0;
    }
    static int adv7180_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *fi)
    {
    struct adv7180_state *state = to_state(sd);
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (fi.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    if (state.curr_norm & V4L2_STD_525_60) {
    fi.interval.numerator = 1001;
    fi.interval.denominator = 30000;
    } else {
    fi.interval.numerator = 1;
    fi.interval.denominator = 25;
    }
//
// If the de-interlacer is active, the chip produces full video frames
// at the field rate.
//
    if (state.field == V4L2_FIELD_NONE)
    fi.interval.denominator *= 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_set_power_pin(state: *mut adv7180_state, on: bool) {
    static void adv7180_set_power_pin(struct adv7180_state *state, bool on)
    {
    if (!state.pwdn_gpio)
    return;
    if (on) {
    gpiod_set_value_cansleep(state.pwdn_gpio, 0);
    usleep_range(5000, 10000);
    } else {
    gpiod_set_value_cansleep(state.pwdn_gpio, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn adv7180_set_reset_pin(state: *mut adv7180_state, on: bool) {
    static void adv7180_set_reset_pin(struct adv7180_state *state, bool on)
    {
    if (!state.rst_gpio)
    return;
    if (on) {
    gpiod_set_value_cansleep(state.rst_gpio, 1);
    } else {
    gpiod_set_value_cansleep(state.rst_gpio, 0);
    usleep_range(5000, 10000);
    }
    }
    static const char * const test_pattern_menu[] = {
    "Single color",
    "Color bars",
    "Luma ramp",
    "Boundary box",
    "Disable",
    };
#[no_mangle]
unsafe extern "C" fn adv7180_test_pattern(state: *mut adv7180_state, value: c_int) -> c_int {
    static int adv7180_test_pattern(struct adv7180_state *state, int value)
    {
    let mut reg: c_uint = 0;
// Map menu value into register value
    if (value < 3)
    reg = value;
    if (value == 3)
    reg = 5;
    adv7180_write(state, ADV7180_REG_ANALOG_CLAMP_CTL, reg);
    if (value == ARRAY_SIZE(test_pattern_menu) - 1) {
    reg = adv7180_read(state, ADV7180_REG_DEF_VALUE_Y);
    reg &= ~ADV7180_DEF_VAL_EN;
    adv7180_write(state, ADV7180_REG_DEF_VALUE_Y, reg);
    return 0;
    }
    reg = adv7180_read(state, ADV7180_REG_DEF_VALUE_Y);
    reg |= ADV7180_DEF_VAL_EN | ADV7180_DEF_VAL_AUTO_EN;
    adv7180_write(state, ADV7180_REG_DEF_VALUE_Y, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int adv7180_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct v4l2_subdev *sd = to_adv7180_sd(ctrl);
    struct adv7180_state *state = to_state(sd);
    let mut ret: c_int = 0;
    int val;
    lockdep_assert_held(&state.mutex);
    val = ctrl.val;
    switch (ctrl.id) {
    case V4L2_CID_BRIGHTNESS:
    ret = adv7180_write(state, ADV7180_REG_BRI, val);
    break;
    case V4L2_CID_HUE:
// Hue is inverted according to HSL chart
    ret = adv7180_write(state, ADV7180_REG_HUE, -val);
    break;
    case V4L2_CID_CONTRAST:
    ret = adv7180_write(state, ADV7180_REG_CON, val);
    break;
    case V4L2_CID_SATURATION:
//
// This could be V4L2_CID_BLUE_BALANCE/V4L2_CID_RED_BALANCE
// Let's not confuse the user, everybody understands saturation
//
    ret = adv7180_write(state, ADV7180_REG_SD_SAT_CB, val);
    if (ret < 0)
    break;
    ret = adv7180_write(state, ADV7180_REG_SD_SAT_CR, val);
    break;
    case V4L2_CID_ADV_FAST_SWITCH:
    if (ctrl.val) {
// ADI required write
    adv7180_write(state, 0x80d9, 0x44);
    adv7180_write(state, ADV7180_REG_FLCONTROL,
    ADV7180_FLCONTROL_FL_ENABLE);
    } else {
// ADI required write
    adv7180_write(state, 0x80d9, 0xc4);
    adv7180_write(state, ADV7180_REG_FLCONTROL, 0x00);
    }
    break;
    case V4L2_CID_TEST_PATTERN:
    ret = adv7180_test_pattern(state, val);
    break;
    default:
    ret = -EINVAL;
    }
    return ret;
    }
    static const struct v4l2_ctrl_ops adv7180_ctrl_ops = {
    .s_ctrl = adv7180_s_ctrl,
    };
    static const struct v4l2_ctrl_config adv7180_ctrl_fast_switch = {
    .ops = &adv7180_ctrl_ops,
    .id = V4L2_CID_ADV_FAST_SWITCH,
    .name = "Fast Switching",
    .type = V4L2_CTRL_TYPE_BOOLEAN,
    .min = 0,
    .max = 1,
    .step = 1,
    };
#[no_mangle]
unsafe extern "C" fn adv7180_init_controls(state: *mut adv7180_state) -> c_int {
    static int adv7180_init_controls(struct adv7180_state *state)
    {
    v4l2_ctrl_handler_init(&state.ctrl_hdl, 4);
    state.ctrl_hdl.lock = &state.mutex;
    v4l2_ctrl_new_std(&state.ctrl_hdl, &adv7180_ctrl_ops,
    V4L2_CID_BRIGHTNESS, ADV7180_BRI_MIN,
    ADV7180_BRI_MAX, 1, ADV7180_BRI_DEF);
    v4l2_ctrl_new_std(&state.ctrl_hdl, &adv7180_ctrl_ops,
    V4L2_CID_CONTRAST, ADV7180_CON_MIN,
    ADV7180_CON_MAX, 1, ADV7180_CON_DEF);
    v4l2_ctrl_new_std(&state.ctrl_hdl, &adv7180_ctrl_ops,
    V4L2_CID_SATURATION, ADV7180_SAT_MIN,
    ADV7180_SAT_MAX, 1, ADV7180_SAT_DEF);
    v4l2_ctrl_new_std(&state.ctrl_hdl, &adv7180_ctrl_ops,
    V4L2_CID_HUE, ADV7180_HUE_MIN,
    ADV7180_HUE_MAX, 1, ADV7180_HUE_DEF);
    v4l2_ctrl_new_custom(&state.ctrl_hdl, &adv7180_ctrl_fast_switch, core::ptr::null_mut());
    if (state.chip_info.flags & ADV7180_FLAG_TEST_PATTERN) {
    v4l2_ctrl_new_std_menu_items(&state.ctrl_hdl,
    &adv7180_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(test_pattern_menu) - 1,
    0,
    ARRAY_SIZE(test_pattern_menu) - 1,
    test_pattern_menu);
    }
    state.sd.ctrl_handler = &state.ctrl_hdl;
    if (state.ctrl_hdl.error) {
    let mut err: c_int = state.ctrl_hdl.error;
    v4l2_ctrl_handler_free(&state.ctrl_hdl);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_exit_controls(state: *mut adv7180_state) {
    static void adv7180_exit_controls(struct adv7180_state *state)
    {
    v4l2_ctrl_handler_free(&state.ctrl_hdl);
    }
    static int adv7180_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index != 0)
    return -EINVAL;
    code.code = MEDIA_BUS_FMT_UYVY8_2X8;
    return 0;
    }
    static int adv7180_mbus_fmt(struct v4l2_subdev *sd,
    struct v4l2_mbus_framefmt *fmt)
    {
    struct adv7180_state *state = to_state(sd);
    fmt.code = MEDIA_BUS_FMT_UYVY8_2X8;
    fmt.colorspace = V4L2_COLORSPACE_SMPTE170M;
    fmt.width = 720;
    fmt.height = state.curr_norm & V4L2_STD_525_60 ? 480 : 576;
    if (state.field == V4L2_FIELD_ALTERNATE)
    fmt.height /= 2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_set_field_mode(state: *mut adv7180_state) -> c_int {
    static int adv7180_set_field_mode(struct adv7180_state *state)
    {
    if (!(state.chip_info.flags & ADV7180_FLAG_I2P))
    return 0;
    if (state.field == V4L2_FIELD_NONE) {
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    adv7180_csi_write(state, 0x01, 0x20);
    adv7180_csi_write(state, 0x02, 0x28);
    adv7180_csi_write(state, 0x03, 0x38);
    adv7180_csi_write(state, 0x04, 0x30);
    adv7180_csi_write(state, 0x05, 0x30);
    adv7180_csi_write(state, 0x06, 0x80);
    adv7180_csi_write(state, 0x07, 0x70);
    adv7180_csi_write(state, 0x08, 0x50);
    }
    adv7180_vpp_write(state, 0xa3, 0x00);
    adv7180_vpp_write(state, 0x5b, 0x00);
    adv7180_vpp_write(state, 0x55, 0x80);
    } else {
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    adv7180_csi_write(state, 0x01, 0x18);
    adv7180_csi_write(state, 0x02, 0x18);
    adv7180_csi_write(state, 0x03, 0x30);
    adv7180_csi_write(state, 0x04, 0x20);
    adv7180_csi_write(state, 0x05, 0x28);
    adv7180_csi_write(state, 0x06, 0x40);
    adv7180_csi_write(state, 0x07, 0x58);
    adv7180_csi_write(state, 0x08, 0x30);
    }
    adv7180_vpp_write(state, 0xa3, 0x70);
    adv7180_vpp_write(state, 0x5b, 0x80);
    adv7180_vpp_write(state, 0x55, 0x00);
    }
    return 0;
    }
    static int adv7180_get_pad_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct adv7180_state *state = to_state(sd);
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
    format.format = *v4l2_subdev_state_get_format(sd_state, 0);
    } else {
    adv7180_mbus_fmt(sd, &format.format);
    format.format.field = state.field;
    }
    return 0;
    }
    static int adv7180_set_pad_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct adv7180_state *state = to_state(sd);
    struct v4l2_mbus_framefmt *framefmt;
    int ret;
    switch (format.format.field) {
    case V4L2_FIELD_NONE:
    if (state.chip_info.flags & ADV7180_FLAG_I2P)
    break;
    fallthrough;
    default:
    format.format.field = V4L2_FIELD_ALTERNATE;
    break;
    }
    ret = adv7180_mbus_fmt(sd,  &format.format);
    if (format.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    state.field = format.format.field;
    } else {
    framefmt = v4l2_subdev_state_get_format(sd_state, 0);
// framefmt = format->format;
    }
    return ret;
    }
    static int adv7180_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state)
    {
    struct v4l2_subdev_format fmt = {
    .which = sd_state ? V4L2_SUBDEV_FORMAT_TRY
    : V4L2_SUBDEV_FORMAT_ACTIVE,
    };
    return adv7180_set_pad_format(sd, sd_state, &fmt);
    }
    static int adv7180_get_mbus_config(struct v4l2_subdev *sd,
    unsigned int pad,
    struct v4l2_mbus_config *cfg)
    {
    struct adv7180_state *state = to_state(sd);
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    cfg.type = V4L2_MBUS_CSI2_DPHY;
    cfg.bus.mipi_csi2.num_data_lanes = 1;
    cfg.bus.mipi_csi2.flags = 0;
    } else {
//
// The ADV7180 sensor supports BT.601/656 output modes.
// The BT.656 is default and not yet configurable by s/w.
//
    cfg.bus.parallel.flags = V4L2_MBUS_MASTER |
    V4L2_MBUS_PCLK_SAMPLE_RISING |
    V4L2_MBUS_DATA_ACTIVE_HIGH;
    cfg.type = V4L2_MBUS_BT656;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_get_skip_frames(sd: *mut v4l2_subdev, frames: *mut u32) -> c_int {
    static int adv7180_get_skip_frames(struct v4l2_subdev *sd, u32 *frames)
    {
// frames = ADV7180_NUM_OF_SKIP_FRAMES;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_g_tvnorms(sd: *mut v4l2_subdev, norm: *mut v4l2_std_id) -> c_int {
    static int adv7180_g_tvnorms(struct v4l2_subdev *sd, v4l2_std_id *norm)
    {
// norm = V4L2_STD_ALL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_device(state: *mut adv7180_state) -> c_int {
    static int init_device(struct adv7180_state *state)
    {
    int ret;
    lockdep_assert_held(&state.mutex);
    ret = adv7180_program_std(state);
    if (ret)
    return ret;
    adv7180_set_field_mode(state);
    __v4l2_ctrl_handler_setup(&state.ctrl_hdl);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_reset_device(state: *mut adv7180_state) -> c_int {
    static int adv7180_reset_device(struct adv7180_state *state)
    {
    int ret;
    lockdep_assert_held(&state.mutex);
    adv7180_set_power_pin(state, true);
    adv7180_set_reset_pin(state, false);
    adv7180_write(state, ADV7180_REG_PWR_MAN, ADV7180_PWR_MAN_RES);
    usleep_range(5000, 10000);
//
// If the devices decoder is power on after reset, power off so the
// device can be configured.
//
    if (state.chip_info.flags & ADV7180_FLAG_RESET_POWERED)
    adv7180_set_power(state, false);
    ret = state.chip_info.init(state);
    if (ret)
    return ret;
    ret = init_device(state);
    if (ret)
    return ret;
// register for interrupts
    if (state.irq > 0) {
// config the Interrupt pin to be active low
    ret = adv7180_write(state, ADV7180_REG_ICONF1,
    ADV7180_ICONF1_ACTIVE_LOW |
    ADV7180_ICONF1_PSYNC_ONLY);
    if (ret < 0)
    return ret;
    ret = adv7180_write(state, ADV7180_REG_IMR1, 0);
    if (ret < 0)
    return ret;
    ret = adv7180_write(state, ADV7180_REG_IMR2, 0);
    if (ret < 0)
    return ret;
// enable AD change interrupts
    ret = adv7180_write(state, ADV7180_REG_IMR3,
    ADV7180_IRQ3_AD_CHANGE);
    if (ret < 0)
    return ret;
    ret = adv7180_write(state, ADV7180_REG_IMR4, 0);
    if (ret < 0)
    return ret;
    }
//
// If the devices decoder is power on after reset, restore the power
// after configuration. This is to preserve the behavior of the driver,
// not doing this result in the first 35+ frames captured being garbage.
//
    if (state.chip_info.flags & ADV7180_FLAG_RESET_POWERED)
    adv7180_set_power(state, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int adv7180_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct adv7180_state *state = to_state(sd);
    int ret;
// Must wait until querystd released the lock
    guard(mutex)(&state.mutex);
//
// Always power off the decoder even if streaming is to be enabled, the
// decoder needs to be off for the device to be configured.
//
    ret = adv7180_set_power(state, false);
    if (ret)
    return ret;
    if (enable) {
    ret = init_device(state);
    if (ret)
    return ret;
    ret = adv7180_set_power(state, true);
    if (ret)
    return ret;
    }
    state.streaming = enable;
    return 0;
    }
    static int adv7180_subscribe_event(struct v4l2_subdev *sd,
    struct v4l2_fh *fh,
    struct v4l2_event_subscription *sub)
    {
    switch (sub.type) {
    case V4L2_EVENT_SOURCE_CHANGE:
    return v4l2_src_change_event_subdev_subscribe(sd, fh, sub);
    case V4L2_EVENT_CTRL:
    return v4l2_ctrl_subdev_subscribe_event(sd, fh, sub);
    default:
    return -EINVAL;
    }
    }

    static int adv7180_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct adv7180_state *state = to_state(sd);
    int ret;
    ret = adv7180_read(state, reg.reg);
    if (ret < 0)
    return ret;
    reg.val = ret;
    reg.size = 1;
    return 0;
    }
    static int adv7180_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct adv7180_state *state = to_state(sd);
    return adv7180_write(state, reg.reg, reg.val);
    }

    static const struct v4l2_subdev_video_ops adv7180_video_ops = {
    .s_std = adv7180_s_std,
    .g_std = adv7180_g_std,
    .querystd = adv7180_querystd,
    .g_input_status = adv7180_g_input_status,
    .s_routing = adv7180_s_routing,
    .g_tvnorms = adv7180_g_tvnorms,
    .s_stream = adv7180_s_stream,
    };
    static const struct v4l2_subdev_core_ops adv7180_core_ops = {
    .subscribe_event = adv7180_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,

    .g_register = adv7180_g_register,
    .s_register = adv7180_s_register,

    };
    static const struct v4l2_subdev_pad_ops adv7180_pad_ops = {
    .enum_mbus_code = adv7180_enum_mbus_code,
    .set_fmt = adv7180_set_pad_format,
    .get_fmt = adv7180_get_pad_format,
    .get_frame_interval = adv7180_get_frame_interval,
    .get_mbus_config = adv7180_get_mbus_config,
    };
    static const struct v4l2_subdev_sensor_ops adv7180_sensor_ops = {
    .g_skip_frames = adv7180_get_skip_frames,
    };
    static const struct v4l2_subdev_ops adv7180_ops = {
    .core = &adv7180_core_ops,
    .video = &adv7180_video_ops,
    .pad = &adv7180_pad_ops,
    .sensor = &adv7180_sensor_ops,
    };
    static const struct v4l2_subdev_internal_ops adv7180_internal_ops = {
    .init_state = adv7180_init_state,
    };
#[no_mangle]
unsafe extern "C" fn adv7180_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t adv7180_irq(int irq, void *devid)
    {
    struct adv7180_state *state = devid;
    u8 isr3;
    mutex_lock(&state.mutex);
    isr3 = adv7180_read(state, ADV7180_REG_ISR3);
// clear
    adv7180_write(state, ADV7180_REG_ICR3, isr3);
    if (isr3 & ADV7180_IRQ3_AD_CHANGE) {
    static const struct v4l2_event src_ch = {
    .type = V4L2_EVENT_SOURCE_CHANGE,
    .u.src_change.changes = V4L2_EVENT_SRC_CH_RESOLUTION,
    };
    v4l2_subdev_notify_event(&state.sd, &src_ch);
    }
    mutex_unlock(&state.mutex);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_init(state: *mut adv7180_state) -> c_int {
    static int adv7180_init(struct adv7180_state *state)
    {
    int ret;
// ITU-R BT.656-4 compatible
    ret = adv7180_write(state, ADV7180_REG_EXTENDED_OUTPUT_CONTROL,
    ADV7180_EXTENDED_OUTPUT_CONTROL_NTSCDIS);
    if (ret < 0)
    return ret;
// Manually set V bit end position in NTSC mode
    return adv7180_write(state, ADV7180_REG_NTSC_V_BIT_END,
    ADV7180_NTSC_V_BIT_END_MANUAL_NVEND);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_set_std(state: *mut adv7180_state, std: c_uint) -> c_int {
    static int adv7180_set_std(struct adv7180_state *state, unsigned int std)
    {
    return adv7180_write(state, ADV7180_REG_INPUT_CONTROL,
    (std << 4) | state.input);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_select_input(state: *mut adv7180_state, input: c_uint) -> c_int {
    static int adv7180_select_input(struct adv7180_state *state, unsigned int input)
    {
    int ret;
    ret = adv7180_read(state, ADV7180_REG_INPUT_CONTROL);
    if (ret < 0)
    return ret;
    ret &= ~ADV7180_INPUT_CONTROL_INSEL_MASK;
    ret |= input;
    return adv7180_write(state, ADV7180_REG_INPUT_CONTROL, ret);
    }
#[no_mangle]
unsafe extern "C" fn adv7182_init(state: *mut adv7180_state) -> c_int {
    static int adv7182_init(struct adv7180_state *state)
    {
    if (state.csi_client)
    adv7180_write(state, ADV7180_REG_CSI_SLAVE_ADDR,
    state.csi_client.addr << 1);
    if (state.vpp_client)
    adv7180_write(state, ADV7180_REG_VPP_SLAVE_ADDR,
    state.vpp_client.addr << 1);
    if (state.chip_info.flags & ADV7180_FLAG_V2) {
// ADI recommended writes for improved video quality
    adv7180_write(state, 0x0080, 0x51);
    adv7180_write(state, 0x0081, 0x51);
    adv7180_write(state, 0x0082, 0x68);
    }
// ADI required writes
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    adv7180_write(state, ADV7180_REG_OUTPUT_CONTROL, 0x4e);
    adv7180_write(state, ADV7180_REG_EXTENDED_OUTPUT_CONTROL, 0x57);
    adv7180_write(state, ADV7180_REG_CTRL_2, 0xc0);
    } else {
    if (state.chip_info.flags & ADV7180_FLAG_V2) {
    if (state.force_bt656_4) {
// ITU-R BT.656-4 compatible
    adv7180_write(state,
    ADV7180_REG_EXTENDED_OUTPUT_CONTROL,
    ADV7180_EXTENDED_OUTPUT_CONTROL_NTSCDIS);
// Manually set NEWAVMODE
    adv7180_write(state,
    ADV7180_REG_VSYNC_FIELD_CTL_1,
    ADV7180_VSYNC_FIELD_CTL_1_NEWAV);
// Manually set V bit end position in NTSC mode
    adv7180_write(state,
    ADV7180_REG_NTSC_V_BIT_END,
    ADV7180_NTSC_V_BIT_END_MANUAL_NVEND);
    } else {
    adv7180_write(state,
    ADV7180_REG_EXTENDED_OUTPUT_CONTROL,
    0x17);
    }
    } else {
    adv7180_write(state,
    ADV7180_REG_EXTENDED_OUTPUT_CONTROL,
    0x07);
    }
    adv7180_write(state, ADV7180_REG_OUTPUT_CONTROL, 0x0c);
    adv7180_write(state, ADV7180_REG_CTRL_2, 0x40);
    }
    adv7180_write(state, 0x0013, 0x00);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv7182_set_std(state: *mut adv7180_state, std: c_uint) -> c_int {
    static int adv7182_set_std(struct adv7180_state *state, unsigned int std)
    {
// Failing to set the reserved bit can result in increased video noise
    return adv7180_write(state, ADV7182_REG_INPUT_VIDSEL,
    (std << 4) | ADV7182_REG_INPUT_RESERVED);
    }
    enum adv7182_input_type {
    ADV7182_INPUT_TYPE_CVBS,
    ADV7182_INPUT_TYPE_DIFF_CVBS,
    ADV7182_INPUT_TYPE_SVIDEO,
    ADV7182_INPUT_TYPE_YPBPR,
    };
#[no_mangle]
unsafe extern "C" fn adv7182_get_input_type(input: c_uint) -> enum adv7182_input_type {
    static enum adv7182_input_type adv7182_get_input_type(unsigned int input)
    {
    switch (input) {
    case ADV7182_INPUT_CVBS_AIN1:
    case ADV7182_INPUT_CVBS_AIN2:
    case ADV7182_INPUT_CVBS_AIN3:
    case ADV7182_INPUT_CVBS_AIN4:
    case ADV7182_INPUT_CVBS_AIN5:
    case ADV7182_INPUT_CVBS_AIN6:
    case ADV7182_INPUT_CVBS_AIN7:
    case ADV7182_INPUT_CVBS_AIN8:
    return ADV7182_INPUT_TYPE_CVBS;
    case ADV7182_INPUT_SVIDEO_AIN1_AIN2:
    case ADV7182_INPUT_SVIDEO_AIN3_AIN4:
    case ADV7182_INPUT_SVIDEO_AIN5_AIN6:
    case ADV7182_INPUT_SVIDEO_AIN7_AIN8:
    return ADV7182_INPUT_TYPE_SVIDEO;
    case ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3:
    case ADV7182_INPUT_YPRPB_AIN4_AIN5_AIN6:
    return ADV7182_INPUT_TYPE_YPBPR;
    case ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2:
    case ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4:
    case ADV7182_INPUT_DIFF_CVBS_AIN5_AIN6:
    case ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8:
    return ADV7182_INPUT_TYPE_DIFF_CVBS;
    default: /* Will never happen */
    return 0;
    }
    }
// ADI recommended writes to registers 0x52, 0x53, 0x54
    static unsigned int adv7182_lbias_settings[][3] = {
    [ADV7182_INPUT_TYPE_CVBS] = { 0xCB, 0x4E, 0x80 },
    [ADV7182_INPUT_TYPE_DIFF_CVBS] = { 0xC0, 0x4E, 0x80 },
    [ADV7182_INPUT_TYPE_SVIDEO] = { 0x0B, 0xCE, 0x80 },
    [ADV7182_INPUT_TYPE_YPBPR] = { 0x0B, 0x4E, 0xC0 },
    };
    static unsigned int adv7280_lbias_settings[][3] = {
    [ADV7182_INPUT_TYPE_CVBS] = { 0xCD, 0x4E, 0x80 },
    [ADV7182_INPUT_TYPE_DIFF_CVBS] = { 0xC0, 0x4E, 0x80 },
    [ADV7182_INPUT_TYPE_SVIDEO] = { 0x0B, 0xCE, 0x80 },
    [ADV7182_INPUT_TYPE_YPBPR] = { 0x0B, 0x4E, 0xC0 },
    };
#[no_mangle]
unsafe extern "C" fn adv7182_select_input(state: *mut adv7180_state, input: c_uint) -> c_int {
    static int adv7182_select_input(struct adv7180_state *state, unsigned int input)
    {
    enum adv7182_input_type input_type;
    unsigned int *lbias;
    unsigned int i;
    int ret;
    ret = adv7180_write(state, ADV7180_REG_INPUT_CONTROL, input);
    if (ret)
    return ret;
// Reset clamp circuitry - ADI recommended writes
    adv7180_write(state, ADV7180_REG_RST_CLAMP, 0x00);
    adv7180_write(state, ADV7180_REG_RST_CLAMP, 0xff);
    input_type = adv7182_get_input_type(input);
    switch (input_type) {
    case ADV7182_INPUT_TYPE_CVBS:
    case ADV7182_INPUT_TYPE_DIFF_CVBS:
// ADI recommends to use the SH1 filter
    adv7180_write(state, ADV7180_REG_SHAP_FILTER_CTL_1, 0x41);
    break;
    default:
    adv7180_write(state, ADV7180_REG_SHAP_FILTER_CTL_1, 0x01);
    break;
    }
    if (state.chip_info.flags & ADV7180_FLAG_V2)
    lbias = adv7280_lbias_settings[input_type];
    else
    lbias = adv7182_lbias_settings[input_type];
    for (i = 0; i < ARRAY_SIZE(adv7182_lbias_settings[0]); i++)
    adv7180_write(state, ADV7180_REG_CVBS_TRIM + i, lbias[i]);
    if (input_type == ADV7182_INPUT_TYPE_DIFF_CVBS) {
// ADI required writes to make differential CVBS work
    adv7180_write(state, ADV7180_REG_RES_CIR, 0xa8);
    adv7180_write(state, ADV7180_REG_CLAMP_ADJ, 0x90);
    adv7180_write(state, ADV7180_REG_DIFF_MODE, 0xb0);
    adv7180_write(state, ADV7180_REG_AGC_ADJ1, 0x08);
    adv7180_write(state, ADV7180_REG_AGC_ADJ2, 0xa0);
    } else {
    adv7180_write(state, ADV7180_REG_RES_CIR, 0xf0);
    adv7180_write(state, ADV7180_REG_CLAMP_ADJ, 0xd0);
    adv7180_write(state, ADV7180_REG_DIFF_MODE, 0x10);
    adv7180_write(state, ADV7180_REG_AGC_ADJ1, 0x9c);
    adv7180_write(state, ADV7180_REG_AGC_ADJ2, 0x00);
    }
    return 0;
    }
    static const struct adv7180_chip_info adv7180_info = {
    .flags = ADV7180_FLAG_RESET_POWERED,
// We cannot discriminate between LQFP and 40-pin LFCSP, so accept
// all inputs and let the card driver take care of validation
//
    .valid_input_mask = BIT(ADV7180_INPUT_CVBS_AIN1) |
    BIT(ADV7180_INPUT_CVBS_AIN2) |
    BIT(ADV7180_INPUT_CVBS_AIN3) |
    BIT(ADV7180_INPUT_CVBS_AIN4) |
    BIT(ADV7180_INPUT_CVBS_AIN5) |
    BIT(ADV7180_INPUT_CVBS_AIN6) |
    BIT(ADV7180_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7180_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7180_INPUT_SVIDEO_AIN5_AIN6) |
    BIT(ADV7180_INPUT_YPRPB_AIN1_AIN2_AIN3) |
    BIT(ADV7180_INPUT_YPRPB_AIN4_AIN5_AIN6),
    .init = adv7180_init,
    .set_std = adv7180_set_std,
    .select_input = adv7180_select_input,
    };
    static const struct adv7180_chip_info adv7182_info = {
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7280_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_I2P | ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7280_m_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_MIPI_CSI2 | ADV7180_FLAG_I2P |
    ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_CVBS_AIN5) |
    BIT(ADV7182_INPUT_CVBS_AIN6) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN5_AIN6) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3) |
    BIT(ADV7182_INPUT_YPRPB_AIN4_AIN5_AIN6),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7281_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_MIPI_CSI2 |
    ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7281_m_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_MIPI_CSI2 |
    ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7281_ma_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_MIPI_CSI2 |
    ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_CVBS_AIN5) |
    BIT(ADV7182_INPUT_CVBS_AIN6) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN5_AIN6) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_YPRPB_AIN1_AIN2_AIN3) |
    BIT(ADV7182_INPUT_YPRPB_AIN4_AIN5_AIN6) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN5_AIN6) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7282_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_I2P | ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
    static const struct adv7180_chip_info adv7282_m_info = {
    .flags = ADV7180_FLAG_V2 | ADV7180_FLAG_MIPI_CSI2 | ADV7180_FLAG_I2P |
    ADV7180_FLAG_TEST_PATTERN,
    .valid_input_mask = BIT(ADV7182_INPUT_CVBS_AIN1) |
    BIT(ADV7182_INPUT_CVBS_AIN2) |
    BIT(ADV7182_INPUT_CVBS_AIN3) |
    BIT(ADV7182_INPUT_CVBS_AIN4) |
    BIT(ADV7182_INPUT_CVBS_AIN7) |
    BIT(ADV7182_INPUT_CVBS_AIN8) |
    BIT(ADV7182_INPUT_SVIDEO_AIN1_AIN2) |
    BIT(ADV7182_INPUT_SVIDEO_AIN3_AIN4) |
    BIT(ADV7182_INPUT_SVIDEO_AIN7_AIN8) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN1_AIN2) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN3_AIN4) |
    BIT(ADV7182_INPUT_DIFF_CVBS_AIN7_AIN8),
    .init = adv7182_init,
    .set_std = adv7182_set_std,
    .select_input = adv7182_select_input,
    };
#[no_mangle]
unsafe extern "C" fn adv7180_probe(client: *mut i2c_client) -> c_int {
    static int adv7180_probe(struct i2c_client *client)
    {
    struct device_node *np = client.dev.of_node;
    struct adv7180_state *state;
    struct v4l2_subdev *sd;
    int ret;
// Check if the adapter supports the needed features
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    state = devm_kzalloc(&client.dev, sizeof(*state), GFP_KERNEL);
    if (state == core::ptr::null_mut())
    return -ENOMEM;
    state.client = client;
    state.field = V4L2_FIELD_ALTERNATE;
    state.chip_info = i2c_get_match_data(client);
    state.pwdn_gpio = devm_gpiod_get_optional(&client.dev, "powerdown",
    GPIOD_OUT_HIGH);
    if (IS_ERR(state.pwdn_gpio)) {
    ret = PTR_ERR(state.pwdn_gpio);
    v4l_err(client, "request for power pin failed: %d\n", ret);
    return ret;
    }
    state.rst_gpio = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(state.rst_gpio)) {
    ret = PTR_ERR(state.rst_gpio);
    v4l_err(client, "request for reset pin failed: %d\n", ret);
    return ret;
    }
    if (of_property_read_bool(np, "adv,force-bt656-4") ||
    of_property_read_bool(np, "adi,force-bt656-4"))
    state.force_bt656_4 = true;
    if (state.chip_info.flags & ADV7180_FLAG_MIPI_CSI2) {
    state.csi_client =
    i2c_new_ancillary_device(client, "csi",
    ADV7180_DEFAULT_CSI_I2C_ADDR);
    if (IS_ERR(state.csi_client))
    return PTR_ERR(state.csi_client);
    }
    if (state.chip_info.flags & ADV7180_FLAG_I2P) {
    state.vpp_client =
    i2c_new_ancillary_device(client, "vpp",
    ADV7180_DEFAULT_VPP_I2C_ADDR);
    if (IS_ERR(state.vpp_client)) {
    ret = PTR_ERR(state.vpp_client);
    goto err_unregister_csi_client;
    }
    }
    state.irq = client.irq;
    mutex_init(&state.mutex);
    state.curr_norm = V4L2_STD_NTSC;
    state.input = 0;
    sd = &state.sd;
    v4l2_i2c_subdev_init(sd, client, &adv7180_ops);
    sd.internal_ops = &adv7180_internal_ops;
    sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE | V4L2_SUBDEV_FL_HAS_EVENTS;
    ret = adv7180_init_controls(state);
    if (ret)
    goto err_unregister_vpp_client;
    state.pad.flags = MEDIA_PAD_FL_SOURCE;
    sd.entity.function = MEDIA_ENT_F_ATV_DECODER;
    ret = media_entity_pads_init(&sd.entity, 1, &state.pad);
    if (ret)
    goto err_free_ctrl;
    mutex_lock(&state.mutex);
    ret = adv7180_reset_device(state);
    mutex_unlock(&state.mutex);
    if (ret)
    goto err_media_entity_cleanup;
    if (state.irq > 0) {
    ret = request_threaded_irq(client.irq, core::ptr::null_mut(), adv7180_irq,
    IRQF_ONESHOT | IRQF_TRIGGER_FALLING,
    KBUILD_MODNAME, state);
    if (ret)
    goto err_media_entity_cleanup;
    }
    ret = v4l2_async_register_subdev(sd);
    if (ret)
    goto err_free_irq;
    mutex_lock(&state.mutex);
    ret = adv7180_read(state, ADV7180_REG_IDENT);
    mutex_unlock(&state.mutex);
    if (ret < 0)
    goto err_v4l2_async_unregister;
    v4l_info(client, "chip id 0x%x found @ 0x%02x (%s)\n",
    ret, client.addr, client.adapter.name);
    return 0;
    err_v4l2_async_unregister:
    v4l2_async_unregister_subdev(sd);
    err_free_irq:
    if (state.irq > 0)
    free_irq(client.irq, state);
    err_media_entity_cleanup:
    media_entity_cleanup(&sd.entity);
    err_free_ctrl:
    adv7180_exit_controls(state);
    err_unregister_vpp_client:
    i2c_unregister_device(state.vpp_client);
    err_unregister_csi_client:
    i2c_unregister_device(state.csi_client);
    mutex_destroy(&state.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adv7180_remove(client: *mut i2c_client) {
    static void adv7180_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct adv7180_state *state = to_state(sd);
    v4l2_async_unregister_subdev(sd);
    if (state.irq > 0)
    free_irq(client.irq, state);
    media_entity_cleanup(&sd.entity);
    adv7180_exit_controls(state);
    i2c_unregister_device(state.vpp_client);
    i2c_unregister_device(state.csi_client);
    adv7180_set_reset_pin(state, true);
    adv7180_set_power_pin(state, false);
    mutex_destroy(&state.mutex);
    }

#[no_mangle]
unsafe extern "C" fn adv7180_suspend(dev: *mut device) -> c_int {
    static int adv7180_suspend(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct adv7180_state *state = to_state(sd);
    guard(mutex)(&state.mutex);
    return adv7180_set_power(state, false);
    }
#[no_mangle]
unsafe extern "C" fn adv7180_resume(dev: *mut device) -> c_int {
    static int adv7180_resume(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct adv7180_state *state = to_state(sd);
    int ret;
    guard(mutex)(&state.mutex);
    ret = adv7180_reset_device(state);
    if (ret < 0)
    return ret;
// If we were streaming when suspending, start decoder.
    if (state.streaming) {
    ret = adv7180_set_power(state, true);
    if (ret)
    return ret;
    }
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(adv7180_pm_ops, adv7180_suspend, adv7180_resume);

    static const struct i2c_device_id adv7180_id[] = {
    { .name = "adv7180", .driver_data = (kernel_ulong_t)&adv7180_info },
    { .name = "adv7180cp", .driver_data = (kernel_ulong_t)&adv7180_info },
    { .name = "adv7180st", .driver_data = (kernel_ulong_t)&adv7180_info },
    { .name = "adv7182", .driver_data = (kernel_ulong_t)&adv7182_info },
    { .name = "adv7280", .driver_data = (kernel_ulong_t)&adv7280_info },
    { .name = "adv7280-m", .driver_data = (kernel_ulong_t)&adv7280_m_info },
    { .name = "adv7281", .driver_data = (kernel_ulong_t)&adv7281_info },
    { .name = "adv7281-m", .driver_data = (kernel_ulong_t)&adv7281_m_info },
    { .name = "adv7281-ma", .driver_data = (kernel_ulong_t)&adv7281_ma_info },
    { .name = "adv7282", .driver_data = (kernel_ulong_t)&adv7282_info },
    { .name = "adv7282-m", .driver_data = (kernel_ulong_t)&adv7282_m_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adv7180_id);
    static const struct of_device_id adv7180_of_id[] = {
    { .compatible = "adi,adv7180", &adv7180_info },
    { .compatible = "adi,adv7180cp", &adv7180_info },
    { .compatible = "adi,adv7180st", &adv7180_info },
    { .compatible = "adi,adv7182", &adv7182_info },
    { .compatible = "adi,adv7280", &adv7280_info },
    { .compatible = "adi,adv7280-m", &adv7280_m_info },
    { .compatible = "adi,adv7281", &adv7281_info },
    { .compatible = "adi,adv7281-m", &adv7281_m_info },
    { .compatible = "adi,adv7281-ma", &adv7281_ma_info },
    { .compatible = "adi,adv7282", &adv7282_info },
    { .compatible = "adi,adv7282-m", &adv7282_m_info },
    {}
    };
    MODULE_DEVICE_TABLE(of, adv7180_of_id);
    static struct i2c_driver adv7180_driver = {
    .driver = {
    .name = KBUILD_MODNAME,
    .pm = ADV7180_PM_OPS,
    .of_match_table = adv7180_of_id,
    },
    .probe = adv7180_probe,
    .remove = adv7180_remove,
    .id_table = adv7180_id,
    };
    module_i2c_driver(adv7180_driver);
    MODULE_DESCRIPTION("Analog Devices ADV7180 video decoder driver");
    MODULE_AUTHOR("Mocean Laboratories");
    MODULE_LICENSE("GPL v2");
