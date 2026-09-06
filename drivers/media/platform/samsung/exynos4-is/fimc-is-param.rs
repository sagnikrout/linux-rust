//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is-param.h
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
// Samsung EXYNOS4x12 FIMC-IS (Imaging Subsystem) driver
//
// Copyright (C) 2011 - 2013 Samsung Electronics Co., Ltd.
//
// Authors: Younghwan Joo <yhwan.joo@samsung.com>
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//

pub const IS_DEFAULT_WIDTH: c_int = 1280;
pub const IS_DEFAULT_HEIGHT: c_int = 720;

pub const DEFAULT_PREVIEW_STILL_FRAMERATE: c_int = 30;
pub const DEFAULT_CAPTURE_STILL_FRAMERATE: c_int = 15;
pub const DEFAULT_PREVIEW_VIDEO_FRAMERATE: c_int = 30;
pub const DEFAULT_CAPTURE_VIDEO_FRAMERATE: c_int = 30;

pub const FIMC_IS_MAGIC_NUMBER: c_uint = 0x01020304;

// The parameter bitmask bit definitions.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum is_param_bit {
    PARAM_GLOBAL_SHOTMODE,
    PARAM_SENSOR_CONTROL,
    PARAM_SENSOR_OTF_OUTPUT,
    PARAM_SENSOR_FRAME_RATE,
    PARAM_BUFFER_CONTROL,
    PARAM_BUFFER_OTF_INPUT,
    PARAM_BUFFER_OTF_OUTPUT,
    PARAM_ISP_CONTROL,
    PARAM_ISP_OTF_INPUT,
    PARAM_ISP_DMA1_INPUT,
// 10
    PARAM_ISP_DMA2_INPUT,
    PARAM_ISP_AA,
    PARAM_ISP_FLASH,
    PARAM_ISP_AWB,
    PARAM_ISP_IMAGE_EFFECT,
    PARAM_ISP_ISO,
    PARAM_ISP_ADJUST,
    PARAM_ISP_METERING,
    PARAM_ISP_AFC,
    PARAM_ISP_OTF_OUTPUT,
// 20
    PARAM_ISP_DMA1_OUTPUT,
    PARAM_ISP_DMA2_OUTPUT,
    PARAM_DRC_CONTROL,
    PARAM_DRC_OTF_INPUT,
    PARAM_DRC_DMA_INPUT,
    PARAM_DRC_OTF_OUTPUT,
    PARAM_SCALERC_CONTROL,
    PARAM_SCALERC_OTF_INPUT,
    PARAM_SCALERC_IMAGE_EFFECT,
    PARAM_SCALERC_INPUT_CROP,
// 30
    PARAM_SCALERC_OUTPUT_CROP,
    PARAM_SCALERC_OTF_OUTPUT,
    PARAM_SCALERC_DMA_OUTPUT,
    PARAM_ODC_CONTROL,
    PARAM_ODC_OTF_INPUT,
    PARAM_ODC_OTF_OUTPUT,
    PARAM_DIS_CONTROL,
    PARAM_DIS_OTF_INPUT,
    PARAM_DIS_OTF_OUTPUT,
    PARAM_TDNR_CONTROL,
// 40
    PARAM_TDNR_OTF_INPUT,
    PARAM_TDNR_1ST_FRAME,
    PARAM_TDNR_OTF_OUTPUT,
    PARAM_TDNR_DMA_OUTPUT,
    PARAM_SCALERP_CONTROL,
    PARAM_SCALERP_OTF_INPUT,
    PARAM_SCALERP_IMAGE_EFFECT,
    PARAM_SCALERP_INPUT_CROP,
    PARAM_SCALERP_OUTPUT_CROP,
    PARAM_SCALERP_ROTATION,
// 50
    PARAM_SCALERP_FLIP,
    PARAM_SCALERP_OTF_OUTPUT,
    PARAM_SCALERP_DMA_OUTPUT,
    PARAM_FD_CONTROL,
    PARAM_FD_OTF_INPUT,
    PARAM_FD_DMA_INPUT,
    PARAM_FD_CONFIG,
}

// Interrupt map
pub const FIMC_IS_INT_GENERAL: c_int = 0;
pub const FIMC_IS_INT_FRAME_DONE_ISP: c_int = 1;
// Input
pub const CONTROL_COMMAND_STOP: c_int = 0;
pub const CONTROL_COMMAND_START: c_int = 1;
pub const CONTROL_BYPASS_DISABLE: c_int = 0;
pub const CONTROL_BYPASS_ENABLE: c_int = 1;
pub const CONTROL_ERROR_NONE: c_int = 0;
// OTF (On-The-Fly) input interface commands
pub const OTF_INPUT_COMMAND_DISABLE: c_int = 0;
pub const OTF_INPUT_COMMAND_ENABLE: c_int = 1;
// OTF input interface color formats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oft_input_fmt {
    OTF_INPUT_FORMAT_BAYER			= 0, /* 1 channel */
    OTF_INPUT_FORMAT_YUV444			= 1, /* 3 channels */
    OTF_INPUT_FORMAT_YUV422			= 2, /* 3 channels */
    OTF_INPUT_FORMAT_YUV420			= 3, /* 3 channels */
    OTF_INPUT_FORMAT_STRGEN_COLORBAR_BAYER	= 10,
    OTF_INPUT_FORMAT_BAYER_DMA		= 11,
}

pub const OTF_INPUT_ORDER_BAYER_GR_BG: c_int = 0;
// OTF input error codes

// DMA input commands
pub const DMA_INPUT_COMMAND_DISABLE: c_int = 0;
pub const DMA_INPUT_COMMAND_ENABLE: c_int = 1;
// DMA input color formats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_input_fmt {
    DMA_INPUT_FORMAT_BAYER			= 0,
    DMA_INPUT_FORMAT_YUV444			= 1,
    DMA_INPUT_FORMAT_YUV422			= 2,
    DMA_INPUT_FORMAT_YUV420			= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_input_order {
// (for DMA_INPUT_PLANE_3)
    DMA_INPUT_ORDER_NO	= 0,
// (only valid at DMA_INPUT_PLANE_2)
    DMA_INPUT_ORDER_CBCR	= 1,
// (only valid at DMA_INPUT_PLANE_2)
    DMA_INPUT_ORDER_CRCB	= 2,
// (only valid at DMA_INPUT_PLANE_1 & DMA_INPUT_FORMAT_YUV444)
    DMA_INPUT_ORDER_YCBCR	= 3,
// (only valid at DMA_INPUT_FORMAT_YUV422 & DMA_INPUT_PLANE_1)
    DMA_INPUT_ORDER_YYCBCR	= 4,
// (only valid at DMA_INPUT_FORMAT_YUV422 & DMA_INPUT_PLANE_1)
    DMA_INPUT_ORDER_YCBYCR	= 5,
// (only valid at DMA_INPUT_FORMAT_YUV422 & DMA_INPUT_PLANE_1)
    DMA_INPUT_ORDER_YCRYCB	= 6,
// (only valid at DMA_INPUT_FORMAT_YUV422 & DMA_INPUT_PLANE_1)
    DMA_INPUT_ORDER_CBYCRY	= 7,
// (only valid at DMA_INPUT_FORMAT_YUV422 & DMA_INPUT_PLANE_1)
    DMA_INPUT_ORDER_CRYCBY	= 8,
// (only valid at DMA_INPUT_FORMAT_BAYER)
    DMA_INPUT_ORDER_GR_BG	= 9
}

//
// Data output parameter definitions
//
pub const OTF_OUTPUT_CROP_DISABLE: c_int = 0;
pub const OTF_OUTPUT_CROP_ENABLE: c_int = 1;
pub const OTF_OUTPUT_COMMAND_DISABLE: c_int = 0;
pub const OTF_OUTPUT_COMMAND_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otf_output_fmt {
    OTF_OUTPUT_FORMAT_YUV444		= 1,
    OTF_OUTPUT_FORMAT_YUV422		= 2,
    OTF_OUTPUT_FORMAT_YUV420		= 3,
    OTF_OUTPUT_FORMAT_RGB			= 4,
}

pub const OTF_OUTPUT_ORDER_BAYER_GR_BG: c_int = 0;

pub const DMA_OUTPUT_COMMAND_DISABLE: c_int = 0;
pub const DMA_OUTPUT_COMMAND_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_output_fmt {
    DMA_OUTPUT_FORMAT_BAYER			= 0,
    DMA_OUTPUT_FORMAT_YUV444		= 1,
    DMA_OUTPUT_FORMAT_YUV422		= 2,
    DMA_OUTPUT_FORMAT_YUV420		= 3,
    DMA_OUTPUT_FORMAT_RGB			= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_output_order {
    DMA_OUTPUT_ORDER_NO		= 0,
// for DMA_OUTPUT_PLANE_3
    DMA_OUTPUT_ORDER_CBCR		= 1,
// only valid at DMA_INPUT_PLANE_2)
    DMA_OUTPUT_ORDER_CRCB		= 2,
// only valid at DMA_OUTPUT_PLANE_2)
    DMA_OUTPUT_ORDER_YYCBCR		= 3,
// only valid at DMA_OUTPUT_FORMAT_YUV422 & DMA_OUTPUT_PLANE_1
    DMA_OUTPUT_ORDER_YCBYCR		= 4,
// only valid at DMA_OUTPUT_FORMAT_YUV422 & DMA_OUTPUT_PLANE_1
    DMA_OUTPUT_ORDER_YCRYCB		= 5,
// only valid at DMA_OUTPUT_FORMAT_YUV422 & DMA_OUTPUT_PLANE_1
    DMA_OUTPUT_ORDER_CBYCRY		= 6,
// only valid at DMA_OUTPUT_FORMAT_YUV422 & DMA_OUTPUT_PLANE_1
    DMA_OUTPUT_ORDER_CRYCBY		= 7,
// only valid at DMA_OUTPUT_FORMAT_YUV422 & DMA_OUTPUT_PLANE_1
    DMA_OUTPUT_ORDER_YCBCR		= 8,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_CRYCB		= 9,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_CRCBY		= 10,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_CBYCR		= 11,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_YCRCB		= 12,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_CBCRY		= 13,
// only valid at DMA_OUTPUT_FORMAT_YUV444 & DMA_OUPUT_PLANE_1
    DMA_OUTPUT_ORDER_BGR		= 14,
// only valid at DMA_OUTPUT_FORMAT_RGB
    DMA_OUTPUT_ORDER_GB_BG		= 15
// only valid at DMA_OUTPUT_FORMAT_BAYER
}

// enum dma_output_notify_dma_done
pub const DMA_OUTPUT_NOTIFY_DMA_DONE_DISABLE: c_int = 0;
pub const DMA_OUTPUT_NOTIFY_DMA_DONE_ENABLE: c_int = 1;
// DMA output error codes

// ----------------------  Global  -----------------------------------

// 3A lock commands
pub const ISP_AA_COMMAND_START: c_int = 0;
pub const ISP_AA_COMMAND_STOP: c_int = 1;
// 3A lock target
pub const ISP_AA_TARGET_AF: c_int = 1;
pub const ISP_AA_TARGET_AE: c_int = 2;
pub const ISP_AA_TARGET_AWB: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_af_mode {
    ISP_AF_MODE_MANUAL			= 0,
    ISP_AF_MODE_SINGLE			= 1,
    ISP_AF_MODE_CONTINUOUS			= 2,
    ISP_AF_MODE_TOUCH			= 3,
    ISP_AF_MODE_SLEEP			= 4,
    ISP_AF_MODE_INIT			= 5,
    ISP_AF_MODE_SET_CENTER_WINDOW		= 6,
    ISP_AF_MODE_SET_TOUCH_WINDOW		= 7
}

// Face AF commands
pub const ISP_AF_FACE_DISABLE: c_int = 0;
pub const ISP_AF_FACE_ENABLE: c_int = 1;
// AF range
pub const ISP_AF_RANGE_NORMAL: c_int = 0;
pub const ISP_AF_RANGE_MACRO: c_int = 1;
// AF sleep
pub const ISP_AF_SLEEP_OFF: c_int = 0;
pub const ISP_AF_SLEEP_ON: c_int = 1;
// Continuous AF commands
pub const ISP_AF_CONTINUOUS_DISABLE: c_int = 0;
pub const ISP_AF_CONTINUOUS_ENABLE: c_int = 1;
// ISP AF error codes

// Flash commands
pub const ISP_FLASH_COMMAND_DISABLE: c_int = 0;

pub const ISP_FLASH_COMMAND_AUTO: c_int = 2;

// Flash red-eye commands
pub const ISP_FLASH_REDEYE_DISABLE: c_int = 0;
pub const ISP_FLASH_REDEYE_ENABLE: c_int = 1;
// Flash error codes

// --------------------------  AWB  ------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_awb_command {
    ISP_AWB_COMMAND_AUTO			= 0,
    ISP_AWB_COMMAND_ILLUMINATION		= 1,
    ISP_AWB_COMMAND_MANUAL			= 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_awb_illumination {
    ISP_AWB_ILLUMINATION_DAYLIGHT		= 0,
    ISP_AWB_ILLUMINATION_CLOUDY		= 1,
    ISP_AWB_ILLUMINATION_TUNGSTEN		= 2,
    ISP_AWB_ILLUMINATION_FLUORESCENT	= 3
}

// ISP AWN error codes

// --------------------------  Effect  -----------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_imageeffect_command {
    ISP_IMAGE_EFFECT_DISABLE		= 0,
    ISP_IMAGE_EFFECT_MONOCHROME		= 1,
    ISP_IMAGE_EFFECT_NEGATIVE_MONO		= 2,
    ISP_IMAGE_EFFECT_NEGATIVE_COLOR		= 3,
    ISP_IMAGE_EFFECT_SEPIA			= 4
}

// Image effect error codes

// ISO commands
pub const ISP_ISO_COMMAND_AUTO: c_int = 0;
pub const ISP_ISO_COMMAND_MANUAL: c_int = 1;
// ISO error codes

// ISP adjust commands

pub const ISP_ADJUST_COMMAND_MANUAL_ALL: c_uint = 0x7f;
// ISP adjustment error codes

//
// Exposure metering
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_metering_command {
    ISP_METERING_COMMAND_AVERAGE	= 0,
    ISP_METERING_COMMAND_SPOT	= 1,
    ISP_METERING_COMMAND_MATRIX	= 2,
    ISP_METERING_COMMAND_CENTER	= 3
}

// ISP metering error codes

//
// AFC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_afc_command {
    ISP_AFC_COMMAND_DISABLE		= 0,
    ISP_AFC_COMMAND_AUTO		= 1,
    ISP_AFC_COMMAND_MANUAL		= 2,
}

pub const ISP_AFC_MANUAL_50HZ: c_int = 50;
pub const ISP_AFC_MANUAL_60HZ: c_int = 60;
// ------------------------  SCENE MODE---------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_scene_mode {
    ISP_SCENE_NONE			= 0,
    ISP_SCENE_PORTRAIT		= 1,
    ISP_SCENE_LANDSCAPE		= 2,
    ISP_SCENE_SPORTS		= 3,
    ISP_SCENE_PARTYINDOOR		= 4,
    ISP_SCENE_BEACHSNOW		= 5,
    ISP_SCENE_SUNSET		= 6,
    ISP_SCENE_DAWN			= 7,
    ISP_SCENE_FALL			= 8,
    ISP_SCENE_NIGHT			= 9,
    ISP_SCENE_AGAINSTLIGHTWLIGHT	= 10,
    ISP_SCENE_AGAINSTLIGHTWOLIGHT	= 11,
    ISP_SCENE_FIRE			= 12,
    ISP_SCENE_TEXT			= 13,
    ISP_SCENE_CANDLE		= 14
}

// AFC error codes

// ----------------------------  FD  -------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fd_config_command {
    FD_CONFIG_COMMAND_MAXIMUM_NUMBER	= 0x1,
    FD_CONFIG_COMMAND_ROLL_ANGLE		= 0x2,
    FD_CONFIG_COMMAND_YAW_ANGLE		= 0x4,
    FD_CONFIG_COMMAND_SMILE_MODE		= 0x8,
    FD_CONFIG_COMMAND_BLINK_MODE		= 0x10,
    FD_CONFIG_COMMAND_EYES_DETECT		= 0x20,
    FD_CONFIG_COMMAND_MOUTH_DETECT		= 0x40,
    FD_CONFIG_COMMAND_ORIENTATION		= 0x80,
    FD_CONFIG_COMMAND_ORIENTATION_VALUE	= 0x100
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fd_config_roll_angle {
    FD_CONFIG_ROLL_ANGLE_BASIC		= 0,
    FD_CONFIG_ROLL_ANGLE_PRECISE_BASIC	= 1,
    FD_CONFIG_ROLL_ANGLE_SIDES		= 2,
    FD_CONFIG_ROLL_ANGLE_PRECISE_SIDES	= 3,
    FD_CONFIG_ROLL_ANGLE_FULL		= 4,
    FD_CONFIG_ROLL_ANGLE_PRECISE_FULL	= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fd_config_yaw_angle {
    FD_CONFIG_YAW_ANGLE_0			= 0,
    FD_CONFIG_YAW_ANGLE_45			= 1,
    FD_CONFIG_YAW_ANGLE_90			= 2,
    FD_CONFIG_YAW_ANGLE_45_90		= 3,
}

// Smile mode configuration
pub const FD_CONFIG_SMILE_MODE_DISABLE: c_int = 0;
pub const FD_CONFIG_SMILE_MODE_ENABLE: c_int = 1;
// Blink mode configuration
pub const FD_CONFIG_BLINK_MODE_DISABLE: c_int = 0;
pub const FD_CONFIG_BLINK_MODE_ENABLE: c_int = 1;
// Eyes detection configuration
pub const FD_CONFIG_EYES_DETECT_DISABLE: c_int = 0;
pub const FD_CONFIG_EYES_DETECT_ENABLE: c_int = 1;
// Mouth detection configuration
pub const FD_CONFIG_MOUTH_DETECT_DISABLE: c_int = 0;
pub const FD_CONFIG_MOUTH_DETECT_ENABLE: c_int = 1;
pub const FD_CONFIG_ORIENTATION_DISABLE: c_int = 0;
pub const FD_CONFIG_ORIENTATION_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_control {
    pub cmd: u32,
    pub bypass: u32,
    pub buffer_address: u32,
    pub buffer_size: u32,
    pub /: *mut *mut u32 skip_frames; / only valid at ISP,
    pub 6]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_otf_input {
    pub cmd: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub bitwidth: u32,
    pub order: u32,
    pub crop_offset_x: u32,
    pub crop_offset_y: u32,
    pub crop_width: u32,
    pub crop_height: u32,
    pub frametime_min: u32,
    pub frametime_max: u32,
    pub 13]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_dma_input {
    pub cmd: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub bitwidth: u32,
    pub plane: u32,
    pub order: u32,
    pub buffer_number: u32,
    pub buffer_address: u32,
    pub 10]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_otf_output {
    pub cmd: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub bitwidth: u32,
    pub order: u32,
    pub 7]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_dma_output {
    pub cmd: u32,
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub bitwidth: u32,
    pub plane: u32,
    pub order: u32,
    pub buffer_number: u32,
    pub buffer_address: u32,
    pub notify_dma_done: u32,
    pub dma_out_mask: u32,
    pub 12]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_global_shotmode {
    pub cmd: u32,
    pub skip_frames: u32,
    pub 3]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_sensor_framerate {
    pub frame_rate: u32,
    pub 2]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_aa {
    pub cmd: u32,
    pub target: u32,
    pub mode: u32,
    pub scene: u32,
    pub sleep: u32,
    pub face: u32,
    pub touch_x: u32,
    pub touch_y: u32,
    pub manual_af_setting: u32,
    pub 10]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_flash {
    pub cmd: u32,
    pub redeye: u32,
    pub 3]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_awb {
    pub cmd: u32,
    pub illumination: u32,
    pub 3]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_imageeffect {
    pub cmd: u32,
    pub 2]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_iso {
    pub cmd: u32,
    pub value: u32,
    pub 3]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_adjust {
    pub cmd: u32,
    pub contrast: i32,
    pub saturation: i32,
    pub sharpness: i32,
    pub exposure: i32,
    pub brightness: i32,
    pub hue: i32,
    pub 8]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_metering {
    pub cmd: u32,
    pub win_pos_x: u32,
    pub win_pos_y: u32,
    pub win_width: u32,
    pub win_height: u32,
    pub 6]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_isp_afc {
    pub cmd: u32,
    pub manual: u32,
    pub 3]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_scaler_imageeffect {
    pub cmd: u32,
    pub arbitrary_cb: u32,
    pub arbitrary_cr: u32,
    pub 4]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_scaler_input_crop {
    pub cmd: u32,
    pub crop_offset_x: u32,
    pub crop_offset_y: u32,
    pub crop_width: u32,
    pub crop_height: u32,
    pub in_width: u32,
    pub in_height: u32,
    pub out_width: u32,
    pub out_height: u32,
    pub 10]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_scaler_output_crop {
    pub cmd: u32,
    pub crop_offset_x: u32,
    pub crop_offset_y: u32,
    pub crop_width: u32,
    pub crop_height: u32,
    pub out_format: u32,
    pub 7]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_scaler_rotation {
    pub cmd: u32,
    pub 2]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_scaler_flip {
    pub cmd: u32,
    pub 2]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_3dnr_1stframe {
    pub cmd: u32,
    pub 2]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_fd_config {
    pub cmd: u32,
    pub max_number: u32,
    pub roll_angle: u32,
    pub yaw_angle: u32,
    pub smile_mode: u32,
    pub blink_mode: u32,
    pub eye_detect: u32,
    pub mouth_detect: u32,
    pub orientation: u32,
    pub orientation_value: u32,
    pub 11]: u32 reserved[FIMC_IS_PARAM_MAX_ENTRIES -,
    pub err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct global_param {
    pub shotmode: param_global_shotmode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_param {
    pub control: param_control,
    pub otf_output: param_otf_output,
    pub frame_rate: param_sensor_framerate,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub otf_output: param_otf_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub dma1_input: param_dma_input,
    pub dma2_input: param_dma_input,
    pub aa: param_isp_aa,
    pub flash: param_isp_flash,
    pub awb: param_isp_awb,
    pub effect: param_isp_imageeffect,
    pub iso: param_isp_iso,
    pub adjust: param_isp_adjust,
    pub metering: param_isp_metering,
    pub afc: param_isp_afc,
    pub otf_output: param_otf_output,
    pub dma1_output: param_dma_output,
    pub dma2_output: param_dma_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drc_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub dma_input: param_dma_input,
    pub otf_output: param_otf_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scalerc_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub effect: param_scaler_imageeffect,
    pub input_crop: param_scaler_input_crop,
    pub output_crop: param_scaler_output_crop,
    pub otf_output: param_otf_output,
    pub dma_output: param_dma_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct odc_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub otf_output: param_otf_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dis_param {
    pub control: param_control,
    pub otf_input: param_otf_output,
    pub otf_output: param_otf_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdnr_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub frame: param_3dnr_1stframe,
    pub otf_output: param_otf_output,
    pub dma_output: param_dma_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scalerp_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub effect: param_scaler_imageeffect,
    pub input_crop: param_scaler_input_crop,
    pub output_crop: param_scaler_output_crop,
    pub rotation: param_scaler_rotation,
    pub flip: param_scaler_flip,
    pub otf_output: param_otf_output,
    pub dma_output: param_dma_output,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_param {
    pub control: param_control,
    pub otf_input: param_otf_input,
    pub dma_input: param_dma_input,
    pub config: param_fd_config,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_param_region {
    pub global: global_param,
    pub sensor: sensor_param,
    pub buf: buffer_param,
    pub isp: isp_param,
    pub drc: drc_param,
    pub scalerc: scalerc_param,
    pub odc: odc_param,
    pub dis: dis_param,
    pub tdnr: tdnr_param,
    pub scalerp: scalerp_param,
    pub fd: fd_param,
    pub __packed: },
pub const NUMBER_OF_GAMMA_CURVE_POINTS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_tune_sensor {
    pub exposure: u32,
    pub analog_gain: u32,
    pub frame_rate: u32,
    pub actuator_position: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_tune_gammacurve {
    pub num_pts_x: [u32; NUMBER_OF_GAMMA_CURVE_POINTS],
    pub num_pts_y_r: [u32; NUMBER_OF_GAMMA_CURVE_POINTS],
    pub num_pts_y_g: [u32; NUMBER_OF_GAMMA_CURVE_POINTS],
    pub num_pts_y_b: [u32; NUMBER_OF_GAMMA_CURVE_POINTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_tune_isp {
// Brightness level: range 0...100, default 7.
    pub brightness_level: u32,
// Contrast level: range -127...127, default 0.
    pub contrast_level: i32,
// Saturation level: range -127...127, default 0.
    pub saturation_level: i32,
    pub gamma_level: i32,
    pub gamma_curve: [is_tune_gammacurve; 4],
// Hue: range -127...127, default 0.
    pub hue: i32,
// Sharpness blur: range -127...127, default 0.
    pub sharpness_blur: i32,
// Despeckle : range -127~127, default : 0
    pub despeckle: i32,
// Edge color supression: range -127...127, default 0.
    pub edge_color_supression: i32,
// Noise reduction: range -127...127, default 0.
    pub noise_reduction: i32,
// (32 * 4 + 9) * 4 = 548 bytes
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_tune_region {
    pub sensor: is_tune_sensor,
    pub isp: is_tune_isp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rational {
    pub num: u32,
    pub den: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct srational {
    pub num: i32,
    pub den: i32,
}

pub const FLASH_FIRED_SHIFT: c_int = 0;
pub const FLASH_NOT_FIRED: c_int = 0;
pub const FLASH_FIRED: c_int = 1;
pub const FLASH_STROBE_SHIFT: c_int = 1;
pub const FLASH_STROBE_NO_DETECTION: c_int = 0;
pub const FLASH_STROBE_RESERVED: c_int = 1;
pub const FLASH_STROBE_RETURN_LIGHT_NOT_DETECTED: c_int = 2;
pub const FLASH_STROBE_RETURN_LIGHT_DETECTED: c_int = 3;
pub const FLASH_MODE_SHIFT: c_int = 3;
pub const FLASH_MODE_UNKNOWN: c_int = 0;
pub const FLASH_MODE_COMPULSORY_FLASH_FIRING: c_int = 1;
pub const FLASH_MODE_COMPULSORY_FLASH_SUPPRESSION: c_int = 2;
pub const FLASH_MODE_AUTO_MODE: c_int = 3;
pub const FLASH_FUNCTION_SHIFT: c_int = 5;
pub const FLASH_FUNCTION_PRESENT: c_int = 0;
pub const FLASH_FUNCTION_NONE: c_int = 1;
pub const FLASH_RED_EYE_SHIFT: c_int = 6;
pub const FLASH_RED_EYE_DISABLED: c_int = 0;
pub const FLASH_RED_EYE_SUPPORTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apex_aperture_value {
    F1_0	= 0,
    F1_4	= 1,
    F2_0	= 2,
    F2_8	= 3,
    F4_0	= 4,
    F5_6	= 5,
    F8_9	= 6,
    F11_0	= 7,
    F16_0	= 8,
    F22_0	= 9,
    F32_0	= 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exif_attribute {
    pub exposure_time: rational,
    pub shutter_speed: srational,
    pub iso_speed_rating: u32,
    pub flash: u32,
    pub brightness: srational,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_frame_header {
    pub valid: u32,
    pub bad_mark: u32,
    pub captured: u32,
    pub frame_number: u32,
    pub exif: exif_attribute,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_fd_rect {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_face_marker {
    pub frame_number: u32,
    pub face: is_fd_rect,
    pub left_eye: is_fd_rect,
    pub right_eye: is_fd_rect,
    pub mouth: is_fd_rect,
    pub roll_angle: u32,
    pub yaw_angle: u32,
    pub confidence: u32,
    pub smile_level: i32,
    pub blink_level: i32,
    pub __packed: },
pub const MAX_FRAME_COUNT: c_int = 8;
pub const MAX_FRAME_COUNT_PREVIEW: c_int = 4;
pub const MAX_FRAME_COUNT_CAPTURE: c_int = 1;
pub const MAX_FACE_COUNT: c_int = 16;
pub const MAX_SHARED_COUNT: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_region {
    pub parameter: is_param_region,
    pub tune: is_tune_region,
    pub header: [is_frame_header; MAX_FRAME_COUNT],
    pub face: [is_face_marker; MAX_FACE_COUNT],
    pub shared: [u32; MAX_SHARED_COUNT],
    pub __packed: },
// Offset to the ISP DMA2 output buffer address array.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_debug_frame_descriptor {
    pub sensor_frame_time: u32,
    pub sensor_exposure_time: u32,
    pub sensor_analog_gain: i32,
// monitor for AA
    pub req_lei: u32,
    pub next_next_lei_exp: u32,
    pub next_next_lei_a_gain: u32,
    pub next_next_lei_d_gain: u32,
    pub next_next_lei_statlei: u32,
    pub next_next_lei_lei: u32,
    pub dummy0: u32,
}

pub const MAX_VERSION_DISPLAY_BUF: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_share_region {
    pub frame_time: u32,
    pub exposure_time: u32,
    pub analog_gain: i32,
    pub r_gain: u32,
    pub g_gain: u32,
    pub b_gain: u32,
    pub af_position: u32,
    pub af_status: u32,
// 0 : SIRC_ISP_CAMERA_AUTOFOCUSMESSAGE_NOMESSAGE
// 1 : SIRC_ISP_CAMERA_AUTOFOCUSMESSAGE_REACHED
// 2 : SIRC_ISP_CAMERA_AUTOFOCUSMESSAGE_UNABLETOREACH
// 3 : SIRC_ISP_CAMERA_AUTOFOCUSMESSAGE_LOST
// default : unknown
    pub af_scene_type: u32,
    pub frame_descp_onoff_control: u32,
    pub frame_descp_update_done: u32,
    pub frame_descp_idx: u32,
    pub frame_descp_max_idx: u32,
    pub chip_id: u32,
    pub chip_rev_no: u32,
    pub isp_fw_ver_no: [u8; MAX_VERSION_DISPLAY_BUF],
    pub isp_fw_ver_date: [u8; MAX_VERSION_DISPLAY_BUF],
    pub sirc_sdk_ver_no: [u8; MAX_VERSION_DISPLAY_BUF],
    pub sirc_sdk_rev_no: [u8; MAX_VERSION_DISPLAY_BUF],
    pub sirc_sdk_rev_date: [u8; MAX_VERSION_DISPLAY_BUF],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is_debug_control {
    pub /: *mut *mut u32 write_point; / 0~ 500KB boundary,
    pub /: *mut *mut u32 assert_flag; / 0: Not invoked, 1: Invoked,
    pub /: *mut *mut u32 pabort_flag; / 0: Not invoked, 1: Invoked,
    pub /: *mut *mut u32 dabort_flag; / 0: Not invoked, 1: Invoked,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_open_extended {
    pub actuator_type: u32,
    pub mclk: u32,
    pub mipi_lane_num: u32,
    pub mipi_speed: u32,
// Skip setfile loading when fast_open_sensor is not 0
    pub fast_open_sensor: u32,
// Activating sensor self calibration mode (6A3)
    pub self_calibration_mode: u32,
// This field is to adjust I2c clock based on ACLK200
// This value is varied in case of rev 0.2
    pub i2c_sclk: u32,
}

extern "C" {
    pub fn fimc_is_hw_get_sensor_max_framerate(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn __fimc_is_hw_update_param(is: *mut fimc_is, offset: u32) -> c_int;
}
extern "C" {
    pub fn fimc_is_set_initial_params(is: *mut fimc_is);
}
extern "C" {
    pub fn __get_pending_param_count(is: *mut fimc_is) -> c_uint;
}
extern "C" {
    pub fn __is_hw_update_params(is: *mut fimc_is) -> c_int;
}
extern "C" {
    pub fn __is_set_frame_size(is: *mut fimc_is, mf: *mut v4l2_mbus_framefmt);
}
extern "C" {
    pub fn __is_set_sensor(is: *mut fimc_is, fps: c_int);
}
extern "C" {
    pub fn __is_set_isp_aa_ae(is: *mut fimc_is);
}
extern "C" {
    pub fn __is_set_isp_flash(is: *mut fimc_is, cmd: u32, redeye: u32);
}
extern "C" {
    pub fn __is_set_isp_awb(is: *mut fimc_is, cmd: u32, val: u32);
}
extern "C" {
    pub fn __is_set_isp_effect(is: *mut fimc_is, cmd: u32);
}
extern "C" {
    pub fn __is_set_isp_iso(is: *mut fimc_is, cmd: u32, val: u32);
}
extern "C" {
    pub fn __is_set_isp_adjust(is: *mut fimc_is, cmd: u32, val: u32);
}
extern "C" {
    pub fn __is_set_isp_metering(is: *mut fimc_is, id: u32, val: u32);
}
extern "C" {
    pub fn __is_set_isp_afc(is: *mut fimc_is, cmd: u32, val: u32);
}
extern "C" {
    pub fn __is_set_drc_control(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_control(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_maxface(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_rollangle(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_yawangle(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_smilemode(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_blinkmode(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_eyedetect(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_mouthdetect(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_orientation(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_fd_config_orientation_val(is: *mut fimc_is, val: u32);
}
extern "C" {
    pub fn __is_set_isp_aa_af_mode(is: *mut fimc_is, cmd: c_int);
}
extern "C" {
    pub fn __is_set_isp_aa_af_start_stop(is: *mut fimc_is, cmd: c_int);
}
