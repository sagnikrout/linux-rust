//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-is-errno.h
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
// Samsung Exynos4 SoC series FIMC-IS slave interface driver
//
// FIMC-IS error code definition
//
// Copyright (C) 2011 - 2013 Samsung Electronics Co., Ltd.
//
// Authors: Younghwan Joo <yhwan.joo@samsung.com>
// Sylwester Nawrocki <s.nawrocki@samsung.com>
//

// General 1 ~ 99
// Sensor 100 ~ 199
// ISP 200 ~ 299
// DRC 300 ~ 399
// SCALERC 400 ~ 499
// ODC 500 ~ 599
// DIS 600 ~ 699
// TDNR 700 ~ 799
// SCALERC 800 ~ 899
// FD 900 ~ 999
pub const IS_ERROR_TIME_OUT_FLAG: c_uint = 0x80000000;
// Set parameter error enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fimc_is_error {
// Common error (0~99)
    ERROR_COMMON_NONE		= 0,
    ERROR_COMMON_CMD		= 1,	/* Invalid command */
    ERROR_COMMON_PARAMETER		= 2,	/* Invalid parameter */
// setfile is not loaded before adjusting
    ERROR_COMMON_SETFILE_LOAD	= 3,
// setfile is not Adjusted before running.
    ERROR_COMMON_SETFILE_ADJUST	= 4,
// Index of setfile is not valid (0~MAX_SETFILE_NUM-1)
    ERROR_COMMON_SETFILE_INDEX	= 5,
// Input path can be changed in ready state(stop)
    ERROR_COMMON_INPUT_PATH		= 6,
// IP can not start if input path is not set
    ERROR_COMMON_INPUT_INIT		= 7,
// Output path can be changed in ready state (stop)
    ERROR_COMMON_OUTPUT_PATH	= 8,
// IP can not start if output path is not set
    ERROR_COMMON_OUTPUT_INIT	= 9,

    ERROR_CONTROL_NONE		= ERROR_COMMON_NONE,
    ERROR_CONTROL_BYPASS		= 11,	/* Enable or Disable */

    ERROR_OTF_INPUT_NONE		= ERROR_COMMON_NONE,
    ERROR_OTF_INPUT_CMD		= 21,
// invalid format  (DRC: YUV444, FD: YUV444, 422, 420)
    ERROR_OTF_INPUT_FORMAT		= 22,
// invalid width (DRC: 128~8192, FD: 32~8190)
    ERROR_OTF_INPUT_WIDTH		= 23,
// invalid height (DRC: 64~8192, FD: 16~8190)
    ERROR_OTF_INPUT_HEIGHT		= 24,
// invalid bit-width (DRC: 8~12bits, FD: 8bit)
    ERROR_OTF_INPUT_BIT_WIDTH	= 25,
// invalid FrameTime for ISP
    ERROR_OTF_INPUT_USER_FRAMETIIME	= 26,

    ERROR_DMA_INPUT_NONE		= ERROR_COMMON_NONE,
// invalid width (DRC: 128~8192, FD: 32~8190)
    ERROR_DMA_INPUT_WIDTH		= 31,
// invalid height (DRC: 64~8192, FD: 16~8190)
    ERROR_DMA_INPUT_HEIGHT		= 32,
// invalid format (DRC: YUV444 or YUV422, FD: YUV444, 422, 420)
    ERROR_DMA_INPUT_FORMAT		= 33,
// invalid bit-width (DRC: 8~12bit, FD: 8bit)
    ERROR_DMA_INPUT_BIT_WIDTH	= 34,
// invalid order(DRC: YYCbCrorYCbYCr, FD:NO,YYCbCr,YCbYCr,CbCr,CrCb)
    ERROR_DMA_INPUT_ORDER		= 35,
// invalid plane (DRC: 3, FD: 1, 2, 3)
    ERROR_DMA_INPUT_PLANE		= 36,

    ERROR_OTF_OUTPUT_NONE		= ERROR_COMMON_NONE,
// invalid width (DRC: 128~8192)
    ERROR_OTF_OUTPUT_WIDTH		= 41,
// invalid height (DRC: 64~8192)
    ERROR_OTF_OUTPUT_HEIGHT		= 42,
// invalid format (DRC: YUV444)
    ERROR_OTF_OUTPUT_FORMAT		= 43,
// invalid bit-width (DRC: 8~12bits)
    ERROR_OTF_OUTPUT_BIT_WIDTH	= 44,

    ERROR_DMA_OUTPUT_NONE		= ERROR_COMMON_NONE,
    ERROR_DMA_OUTPUT_WIDTH		= 51,	/* invalid width */
    ERROR_DMA_OUTPUT_HEIGHT		= 52,	/* invalid height */
    ERROR_DMA_OUTPUT_FORMAT		= 53,	/* invalid format */
    ERROR_DMA_OUTPUT_BIT_WIDTH	= 54,	/* invalid bit-width */
    ERROR_DMA_OUTPUT_PLANE		= 55,	/* invalid plane */
    ERROR_DMA_OUTPUT_ORDER		= 56,	/* invalid order */

    ERROR_GLOBAL_SHOTMODE_NONE	= ERROR_COMMON_NONE,

// SENSOR Error(100~199)
    ERROR_SENSOR_NONE		= ERROR_COMMON_NONE,
    ERROR_SENSOR_I2C_FAIL		= 101,
    ERROR_SENSOR_INVALID_FRAMERATE,
    ERROR_SENSOR_INVALID_EXPOSURETIME,
    ERROR_SENSOR_INVALID_SIZE,
    ERROR_SENSOR_INVALID_SETTING,
    ERROR_SENSOR_ACTUATOR_INIT_FAIL,
    ERROR_SENSOR_INVALID_AF_POS,
    ERROR_SENSOR_UNSUPPORT_FUNC,
    ERROR_SENSOR_UNSUPPORT_PERI,
    ERROR_SENSOR_UNSUPPORT_AF,

// ISP Error (200~299)
    ERROR_ISP_AF_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_AF_BUSY		= 201,
    ERROR_ISP_AF_INVALID_COMMAND	= 202,
    ERROR_ISP_AF_INVALID_MODE	= 203,
    ERROR_ISP_FLASH_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_AWB_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_IMAGE_EFFECT_NONE	= ERROR_COMMON_NONE,
    ERROR_ISP_ISO_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_ADJUST_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_METERING_NONE		= ERROR_COMMON_NONE,
    ERROR_ISP_AFC_NONE		= ERROR_COMMON_NONE,

// DRC Error (300~399)

// FD Error  (400~499)
    ERROR_FD_NONE					= ERROR_COMMON_NONE,
// Invalid max number (1~16)
    ERROR_FD_CONFIG_MAX_NUMBER_STATE		= 401,
    ERROR_FD_CONFIG_MAX_NUMBER_INVALID		= 402,
    ERROR_FD_CONFIG_YAW_ANGLE_STATE			= 403,
    ERROR_FD_CONFIG_YAW_ANGLE_INVALID		= 404,
    ERROR_FD_CONFIG_ROLL_ANGLE_STATE		= 405,
    ERROR_FD_CONFIG_ROLL_ANGLE_INVALID		= 406,
    ERROR_FD_CONFIG_SMILE_MODE_INVALID		= 407,
    ERROR_FD_CONFIG_BLINK_MODE_INVALID		= 408,
    ERROR_FD_CONFIG_EYES_DETECT_INVALID		= 409,
    ERROR_FD_CONFIG_MOUTH_DETECT_INVALID		= 410,
    ERROR_FD_CONFIG_ORIENTATION_STATE		= 411,
    ERROR_FD_CONFIG_ORIENTATION_INVALID		= 412,
    ERROR_FD_CONFIG_ORIENTATION_VALUE_INVALID	= 413,
// PARAM_FdResultStr can be only applied in ready-state or stream off
    ERROR_FD_RESULT					= 414,
// PARAM_FdModeStr can be only applied in ready-state or stream off
    ERROR_FD_MODE					= 415,
// Scaler Error  (500 ~ 599)
    ERROR_SCALER_NO_NONE				= ERROR_COMMON_NONE,
    ERROR_SCALER_DMA_OUTSEL				= 501,
    ERROR_SCALER_H_RATIO				= 502,
    ERROR_SCALER_V_RATIO				= 503,

    ERROR_SCALER_IMAGE_EFFECT			= 510,

    ERROR_SCALER_ROTATE				= 520,
    ERROR_SCALER_FLIP				= 521,
}
