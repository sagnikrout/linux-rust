//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/s5c73m3/s5c73m3.h
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
// Samsung LSI S5C73M3 8M pixel camera driver
//
// Copyright (C) 2012, Samsung Electronics, Co., Ltd.
// Sylwester Nawrocki <s.nawrocki@samsung.com>
// Andrzej Hajda <a.hajda@samsung.com>
//

// Subdevs pad index definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5c73m3_pads {
    S5C73M3_ISP_PAD,
    S5C73M3_JPEG_PAD,
    S5C73M3_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5c73m3_oif_pads {
    OIF_ISP_PAD,
    OIF_JPEG_PAD,
    OIF_SOURCE_PAD,
    OIF_NUM_PADS
}

pub const S5C73M3_SENSOR_FW_LEN: c_int = 6;
pub const S5C73M3_SENSOR_TYPE_LEN: c_int = 12;

pub const AHB_MSB_ADDR_PTR: c_uint = 0xfcfc;
pub const REG_CMDWR_ADDRH: c_uint = 0x0050;
pub const REG_CMDWR_ADDRL: c_uint = 0x0054;
pub const REG_CMDRD_ADDRH: c_uint = 0x0058;
pub const REG_CMDRD_ADDRL: c_uint = 0x005c;
pub const REG_CMDBUF_ADDR: c_uint = 0x0f14;

pub const REG_STATUS_BOOT_SUB_MAIN_ENTER: c_uint = 0xff01;
pub const REG_STATUS_BOOT_SRAM_TIMING_OK: c_uint = 0xff02;
pub const REG_STATUS_BOOT_INTERRUPTS_EN: c_uint = 0xff03;
pub const REG_STATUS_BOOT_R_PLL_DONE: c_uint = 0xff04;
pub const REG_STATUS_BOOT_R_PLL_LOCKTIME_DONE: c_uint = 0xff05;
pub const REG_STATUS_BOOT_DELAY_COUNT_DONE: c_uint = 0xff06;
pub const REG_STATUS_BOOT_I_PLL_DONE: c_uint = 0xff07;
pub const REG_STATUS_BOOT_I_PLL_LOCKTIME_DONE: c_uint = 0xff08;
pub const REG_STATUS_BOOT_PLL_INIT_OK: c_uint = 0xff09;
pub const REG_STATUS_BOOT_SENSOR_INIT_OK: c_uint = 0xff0a;
pub const REG_STATUS_BOOT_GPIO_SETTING_OK: c_uint = 0xff0b;
pub const REG_STATUS_BOOT_READ_CAL_DATA_OK: c_uint = 0xff0c;
pub const REG_STATUS_BOOT_STABLE_AE_AWB_OK: c_uint = 0xff0d;
pub const REG_STATUS_ISP_COMMAND_COMPLETED: c_uint = 0xffff;
pub const REG_STATUS_EXCEPTION_OCCURED: c_uint = 0xdead;

pub const COMM_IMG_OUTPUT: c_uint = 0x0902;
pub const COMM_IMG_OUTPUT_HDR: c_uint = 0x0008;
pub const COMM_IMG_OUTPUT_YUV: c_uint = 0x0009;
pub const COMM_IMG_OUTPUT_INTERLEAVED: c_uint = 0x000d;
pub const COMM_STILL_PRE_FLASH: c_uint = 0x0a00;
pub const COMM_STILL_PRE_FLASH_FIRE: c_uint = 0x0000;
pub const COMM_STILL_PRE_FLASH_NON_FIRED: c_uint = 0x0000;
pub const COMM_STILL_PRE_FLASH_FIRED: c_uint = 0x0001;
pub const COMM_STILL_MAIN_FLASH: c_uint = 0x0a02;
pub const COMM_STILL_MAIN_FLASH_CANCEL: c_uint = 0x0001;
pub const COMM_STILL_MAIN_FLASH_FIRE: c_uint = 0x0002;
pub const COMM_ZOOM_STEP: c_uint = 0x0b00;
pub const COMM_IMAGE_EFFECT: c_uint = 0x0b0a;
pub const COMM_IMAGE_EFFECT_NONE: c_uint = 0x0001;
pub const COMM_IMAGE_EFFECT_NEGATIVE: c_uint = 0x0002;
pub const COMM_IMAGE_EFFECT_AQUA: c_uint = 0x0003;
pub const COMM_IMAGE_EFFECT_SEPIA: c_uint = 0x0004;
pub const COMM_IMAGE_EFFECT_MONO: c_uint = 0x0005;
pub const COMM_IMAGE_QUALITY: c_uint = 0x0b0c;
pub const COMM_IMAGE_QUALITY_SUPERFINE: c_uint = 0x0000;
pub const COMM_IMAGE_QUALITY_FINE: c_uint = 0x0001;
pub const COMM_IMAGE_QUALITY_NORMAL: c_uint = 0x0002;
pub const COMM_FLASH_MODE: c_uint = 0x0b0e;
pub const COMM_FLASH_MODE_OFF: c_uint = 0x0000;
pub const COMM_FLASH_MODE_ON: c_uint = 0x0001;
pub const COMM_FLASH_MODE_AUTO: c_uint = 0x0002;
pub const COMM_FLASH_STATUS: c_uint = 0x0b80;
pub const COMM_FLASH_STATUS_OFF: c_uint = 0x0001;
pub const COMM_FLASH_STATUS_ON: c_uint = 0x0002;
pub const COMM_FLASH_STATUS_AUTO: c_uint = 0x0003;
pub const COMM_FLASH_TORCH: c_uint = 0x0b12;
pub const COMM_FLASH_TORCH_OFF: c_uint = 0x0000;
pub const COMM_FLASH_TORCH_ON: c_uint = 0x0001;
pub const COMM_AE_NEEDS_FLASH: c_uint = 0x0cba;
pub const COMM_AE_NEEDS_FLASH_OFF: c_uint = 0x0000;
pub const COMM_AE_NEEDS_FLASH_ON: c_uint = 0x0001;
pub const COMM_CHG_MODE: c_uint = 0x0b10;
pub const COMM_CHG_MODE_NEW: c_uint = 0x8000;
pub const COMM_CHG_MODE_SUBSAMPLING_HALF: c_uint = 0x2000;
pub const COMM_CHG_MODE_SUBSAMPLING_QUARTER: c_uint = 0x4000;
pub const COMM_CHG_MODE_YUV_320_240: c_uint = 0x0001;
pub const COMM_CHG_MODE_YUV_640_480: c_uint = 0x0002;
pub const COMM_CHG_MODE_YUV_880_720: c_uint = 0x0003;
pub const COMM_CHG_MODE_YUV_960_720: c_uint = 0x0004;
pub const COMM_CHG_MODE_YUV_1184_666: c_uint = 0x0005;
pub const COMM_CHG_MODE_YUV_1280_720: c_uint = 0x0006;
pub const COMM_CHG_MODE_YUV_1536_864: c_uint = 0x0007;
pub const COMM_CHG_MODE_YUV_1600_1200: c_uint = 0x0008;
pub const COMM_CHG_MODE_YUV_1632_1224: c_uint = 0x0009;
pub const COMM_CHG_MODE_YUV_1920_1080: c_uint = 0x000a;
pub const COMM_CHG_MODE_YUV_1920_1440: c_uint = 0x000b;
pub const COMM_CHG_MODE_YUV_2304_1296: c_uint = 0x000c;
pub const COMM_CHG_MODE_YUV_3264_2448: c_uint = 0x000d;
pub const COMM_CHG_MODE_YUV_352_288: c_uint = 0x000e;
pub const COMM_CHG_MODE_YUV_1008_672: c_uint = 0x000f;
pub const COMM_CHG_MODE_JPEG_640_480: c_uint = 0x0010;
pub const COMM_CHG_MODE_JPEG_800_450: c_uint = 0x0020;
pub const COMM_CHG_MODE_JPEG_800_600: c_uint = 0x0030;
pub const COMM_CHG_MODE_JPEG_1280_720: c_uint = 0x0040;
pub const COMM_CHG_MODE_JPEG_1280_960: c_uint = 0x0050;
pub const COMM_CHG_MODE_JPEG_1600_900: c_uint = 0x0060;
pub const COMM_CHG_MODE_JPEG_1600_1200: c_uint = 0x0070;
pub const COMM_CHG_MODE_JPEG_2048_1152: c_uint = 0x0080;
pub const COMM_CHG_MODE_JPEG_2048_1536: c_uint = 0x0090;
pub const COMM_CHG_MODE_JPEG_2560_1440: c_uint = 0x00a0;
pub const COMM_CHG_MODE_JPEG_2560_1920: c_uint = 0x00b0;
pub const COMM_CHG_MODE_JPEG_3264_2176: c_uint = 0x00c0;
pub const COMM_CHG_MODE_JPEG_1024_768: c_uint = 0x00d0;
pub const COMM_CHG_MODE_JPEG_3264_1836: c_uint = 0x00e0;
pub const COMM_CHG_MODE_JPEG_3264_2448: c_uint = 0x00f0;
pub const COMM_AF_CON: c_uint = 0x0e00;
pub const COMM_AF_CON_STOP: c_uint = 0x0000;
pub const COMM_AF_CON_SCAN: c_uint = 0x0001 /* Full Search */;
pub const COMM_AF_CON_START: c_uint = 0x0002 /* Fast Search */;
pub const COMM_AF_CAL: c_uint = 0x0e06;
pub const COMM_AF_TOUCH_AF: c_uint = 0x0e0a;

pub const REG_CAF_STATUS_FIND_SEARCH_DIR: c_uint = 0x0001;
pub const REG_CAF_STATUS_FOCUSING: c_uint = 0x0002;
pub const REG_CAF_STATUS_FOCUSED: c_uint = 0x0003;
pub const REG_CAF_STATUS_UNFOCUSED: c_uint = 0x0004;
pub const REG_AF_STATUS_INVALID: c_uint = 0x0010;
pub const REG_AF_STATUS_FOCUSING: c_uint = 0x0020;
pub const REG_AF_STATUS_FOCUSED: c_uint = 0x0030;
pub const REG_AF_STATUS_UNFOCUSED: c_uint = 0x0040;

pub const COMM_AF_FACE_ZOOM: c_uint = 0x0e10;
pub const COMM_AF_MODE: c_uint = 0x0e02;
pub const COMM_AF_MODE_NORMAL: c_uint = 0x0000;
pub const COMM_AF_MODE_MACRO: c_uint = 0x0001;
pub const COMM_AF_MODE_MOVIE_CAF_START: c_uint = 0x0002;
pub const COMM_AF_MODE_MOVIE_CAF_STOP: c_uint = 0x0003;
pub const COMM_AF_MODE_PREVIEW_CAF_START: c_uint = 0x0004;
pub const COMM_AF_MODE_PREVIEW_CAF_STOP: c_uint = 0x0005;
pub const COMM_AF_SOFTLANDING: c_uint = 0x0e16;
pub const COMM_AF_SOFTLANDING_ON: c_uint = 0x0000;
pub const COMM_AF_SOFTLANDING_RES_COMPLETE: c_uint = 0x0001;
pub const COMM_FACE_DET: c_uint = 0x0e0c;
pub const COMM_FACE_DET_OFF: c_uint = 0x0000;
pub const COMM_FACE_DET_ON: c_uint = 0x0001;
pub const COMM_FACE_DET_OSD: c_uint = 0x0e0e;
pub const COMM_FACE_DET_OSD_OFF: c_uint = 0x0000;
pub const COMM_FACE_DET_OSD_ON: c_uint = 0x0001;
pub const COMM_AE_CON: c_uint = 0x0c00;
pub const COMM_AE_STOP: c_uint = 0x0000 /* lock */;
pub const COMM_AE_START: c_uint = 0x0001 /* unlock */;
pub const COMM_ISO: c_uint = 0x0c02;
pub const COMM_ISO_AUTO: c_uint = 0x0000;
pub const COMM_ISO_100: c_uint = 0x0001;
pub const COMM_ISO_200: c_uint = 0x0002;
pub const COMM_ISO_400: c_uint = 0x0003;
pub const COMM_ISO_800: c_uint = 0x0004;
pub const COMM_ISO_SPORTS: c_uint = 0x0005;
pub const COMM_ISO_NIGHT: c_uint = 0x0006;
pub const COMM_ISO_INDOOR: c_uint = 0x0007;
// 0x00000 (-2.0 EV)...0x0008 (2.0 EV), 0.5EV step
pub const COMM_EV: c_uint = 0x0c04;
pub const COMM_METERING: c_uint = 0x0c06;
pub const COMM_METERING_CENTER: c_uint = 0x0000;
pub const COMM_METERING_SPOT: c_uint = 0x0001;
pub const COMM_METERING_AVERAGE: c_uint = 0x0002;
pub const COMM_METERING_SMART: c_uint = 0x0003;
pub const COMM_WDR: c_uint = 0x0c08;
pub const COMM_WDR_OFF: c_uint = 0x0000;
pub const COMM_WDR_ON: c_uint = 0x0001;
pub const COMM_FLICKER_MODE: c_uint = 0x0c12;
pub const COMM_FLICKER_NONE: c_uint = 0x0000;
pub const COMM_FLICKER_MANUAL_50HZ: c_uint = 0x0001;
pub const COMM_FLICKER_MANUAL_60HZ: c_uint = 0x0002;
pub const COMM_FLICKER_AUTO: c_uint = 0x0003;
pub const COMM_FLICKER_AUTO_50HZ: c_uint = 0x0004;
pub const COMM_FLICKER_AUTO_60HZ: c_uint = 0x0005;
pub const COMM_FRAME_RATE: c_uint = 0x0c1e;
pub const COMM_FRAME_RATE_AUTO_SET: c_uint = 0x0000;
pub const COMM_FRAME_RATE_FIXED_30FPS: c_uint = 0x0002;
pub const COMM_FRAME_RATE_FIXED_20FPS: c_uint = 0x0003;
pub const COMM_FRAME_RATE_FIXED_15FPS: c_uint = 0x0004;
pub const COMM_FRAME_RATE_FIXED_60FPS: c_uint = 0x0007;
pub const COMM_FRAME_RATE_FIXED_120FPS: c_uint = 0x0008;
pub const COMM_FRAME_RATE_FIXED_7FPS: c_uint = 0x0009;
pub const COMM_FRAME_RATE_FIXED_10FPS: c_uint = 0x000a;
pub const COMM_FRAME_RATE_FIXED_90FPS: c_uint = 0x000b;
pub const COMM_FRAME_RATE_ANTI_SHAKE: c_uint = 0x0013;
// 0x0000...0x0004 -> sharpness: 0, 1, 2, -1, -2
pub const COMM_SHARPNESS: c_uint = 0x0c14;
// 0x0000...0x0004 -> saturation: 0, 1, 2, -1, -2
pub const COMM_SATURATION: c_uint = 0x0c16;
// 0x0000...0x0004 -> contrast: 0, 1, 2, -1, -2
pub const COMM_CONTRAST: c_uint = 0x0c18;
pub const COMM_SCENE_MODE: c_uint = 0x0c1a;
pub const COMM_SCENE_MODE_NONE: c_uint = 0x0000;
pub const COMM_SCENE_MODE_PORTRAIT: c_uint = 0x0001;
pub const COMM_SCENE_MODE_LANDSCAPE: c_uint = 0x0002;
pub const COMM_SCENE_MODE_SPORTS: c_uint = 0x0003;
pub const COMM_SCENE_MODE_INDOOR: c_uint = 0x0004;
pub const COMM_SCENE_MODE_BEACH: c_uint = 0x0005;
pub const COMM_SCENE_MODE_SUNSET: c_uint = 0x0006;
pub const COMM_SCENE_MODE_DAWN: c_uint = 0x0007;
pub const COMM_SCENE_MODE_FALL: c_uint = 0x0008;
pub const COMM_SCENE_MODE_NIGHT: c_uint = 0x0009;
pub const COMM_SCENE_MODE_AGAINST_LIGHT: c_uint = 0x000a;
pub const COMM_SCENE_MODE_FIRE: c_uint = 0x000b;
pub const COMM_SCENE_MODE_TEXT: c_uint = 0x000c;
pub const COMM_SCENE_MODE_CANDLE: c_uint = 0x000d;
pub const COMM_AE_AUTO_BRACKET: c_uint = 0x0b14;
pub const COMM_AE_AUTO_BRAKET_EV05: c_uint = 0x0080;
pub const COMM_AE_AUTO_BRAKET_EV10: c_uint = 0x0100;
pub const COMM_AE_AUTO_BRAKET_EV15: c_uint = 0x0180;
pub const COMM_AE_AUTO_BRAKET_EV20: c_uint = 0x0200;
pub const COMM_SENSOR_STREAMING: c_uint = 0x090a;
pub const COMM_SENSOR_STREAMING_OFF: c_uint = 0x0000;
pub const COMM_SENSOR_STREAMING_ON: c_uint = 0x0001;
pub const COMM_AWB_MODE: c_uint = 0x0d02;
pub const COMM_AWB_MODE_INCANDESCENT: c_uint = 0x0000;
pub const COMM_AWB_MODE_FLUORESCENT1: c_uint = 0x0001;
pub const COMM_AWB_MODE_FLUORESCENT2: c_uint = 0x0002;
pub const COMM_AWB_MODE_DAYLIGHT: c_uint = 0x0003;
pub const COMM_AWB_MODE_CLOUDY: c_uint = 0x0004;
pub const COMM_AWB_MODE_AUTO: c_uint = 0x0005;
pub const COMM_AWB_CON: c_uint = 0x0d00;
pub const COMM_AWB_STOP: c_uint = 0x0000 /* lock */;
pub const COMM_AWB_START: c_uint = 0x0001 /* unlock */;
pub const COMM_FW_UPDATE: c_uint = 0x0906;
pub const COMM_FW_UPDATE_NOT_READY: c_uint = 0x0000;
pub const COMM_FW_UPDATE_SUCCESS: c_uint = 0x0005;
pub const COMM_FW_UPDATE_FAIL: c_uint = 0x0007;
pub const COMM_FW_UPDATE_BUSY: c_uint = 0xffff;
pub const S5C73M3_MAX_SUPPLIES: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5c73m3_ctrls {
    pub handler: v4l2_ctrl_handler,
// exposure/exposure bias cluster
    pub auto_exposure: *mut v4l2_ctrl,
    pub exposure_bias: *mut v4l2_ctrl,
    pub exposure_metering: *mut v4l2_ctrl,
}

// iso/auto iso cluster
// continuous auto focus/auto focus cluster
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5c73m3_resolution_types {
    RES_ISP,
    RES_JPEG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5c73m3_interval {
    pub fps_reg: u16,
    pub interval: v4l2_fract,
// Maximum rectangle for the interval
    pub size: v4l2_frmsize_discrete,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5c73m3 {
    pub sensor_sd: v4l2_subdev,
    pub sensor_pads: [media_pad; S5C73M3_NUM_PADS],
    pub oif_sd: v4l2_subdev,
    pub oif_pads: [media_pad; OIF_NUM_PADS],
    pub spidrv: spi_driver,
    pub spi_dev: *mut spi_device,
    pub i2c_client: *mut i2c_client,
    pub i2c_write_address: u32,
    pub i2c_read_address: u32,
    pub supplies: [regulator_bulk_data; S5C73M3_MAX_SUPPLIES],
    pub stby: *mut gpio_desc,
    pub reset: *mut gpio_desc,
    pub clock: *mut clk,
// Video bus type - MIPI-CSI2/parallel
    pub bus_type: v4l2_mbus_type,
    pub sensor_pix_size: [*const s5c73m3_frame_size; 2],
    pub oif_pix_size: [*const s5c73m3_frame_size; 2],
    pub mbus_code: u32,
    pub fiv: *const s5c73m3_interval,
    pub frame_desc: v4l2_mbus_frame_desc,
// protects the struct members below
    pub lock: mutex,
    pub ctrls: s5c73m3_ctrls,
    pub streaming:1: u8,
    pub apply_fmt:1: u8,
    pub apply_fiv:1: u8,
    pub isp_ready:1: u8,
    pub power: c_short,
    pub 2]: char sensor_fw[S5C73M3_SENSOR_FW_LEN +,
    pub 2]: char sensor_type[S5C73M3_SENSOR_TYPE_LEN +,
    pub fw_file_version: [c_char; 2],
    pub fw_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s5c73m3_frame_size {
    pub width: u32,
    pub height: u32,
    pub reg_val: u8,
}

extern "C" {
    pub fn s5c73m3_register_spi_driver(state: *mut s5c73m3) -> c_int;
}
extern "C" {
    pub fn s5c73m3_unregister_spi_driver(state: *mut s5c73m3);
}
extern "C" {
    pub fn s5c73m3_read(state: *mut s5c73m3, addr: u32, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn s5c73m3_write(state: *mut s5c73m3, addr: u32, data: u16) -> c_int;
}
extern "C" {
    pub fn s5c73m3_isp_command(state: *mut s5c73m3, command: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn s5c73m3_init_controls(state: *mut s5c73m3) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: sd, s5c73m3: struct, _arg: sensor_sd) -> return;
}
extern "C" {
    pub fn container_of(_arg: sd, s5c73m3: struct, _arg: oif_sd) -> return;
}
