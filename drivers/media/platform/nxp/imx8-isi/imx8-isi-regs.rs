//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/imx8-isi/imx8-isi-regs.h
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
// Copyright 2019-2020 NXP
//

// ISI Registers Define
// Channel Control Register
pub const CHNL_CTRL: c_uint = 0x0000;

pub const CHNL_CTRL_CHAIN_BUF_NO_CHAIN: c_int = 0;
pub const CHNL_CTRL_CHAIN_BUF_2_CHAIN: c_int = 1;

//
// CHNL_CTRL_BLANK_PXL: i.MX8{QM,QXP} only
// CHNL_CTRL_VC_ID_1, CHNL_CTRL_VC_ID_1_MASK: i.MX95 only
//

pub const CHNL_CTRL_SRC_TYPE_DEVICE: c_int = 0;
pub const CHNL_CTRL_SRC_TYPE_MEMORY: c_int = 1;

// Channel Image Control Register
pub const CHNL_IMG_CTRL: c_uint = 0x0004;

pub const CHNL_IMG_CTRL_FORMAT_RGBA8888: c_uint = 0x00;
pub const CHNL_IMG_CTRL_FORMAT_ABGR8888: c_uint = 0x01;
pub const CHNL_IMG_CTRL_FORMAT_ARGB8888: c_uint = 0x02;
pub const CHNL_IMG_CTRL_FORMAT_RGBX888: c_uint = 0x03;
pub const CHNL_IMG_CTRL_FORMAT_XBGR888: c_uint = 0x04;
pub const CHNL_IMG_CTRL_FORMAT_XRGB888: c_uint = 0x05;
pub const CHNL_IMG_CTRL_FORMAT_RGB888P: c_uint = 0x06;
pub const CHNL_IMG_CTRL_FORMAT_BGR888P: c_uint = 0x07;
pub const CHNL_IMG_CTRL_FORMAT_A2BGR10: c_uint = 0x08;
pub const CHNL_IMG_CTRL_FORMAT_A2RGB10: c_uint = 0x09;
pub const CHNL_IMG_CTRL_FORMAT_RGB565: c_uint = 0x0a;
pub const CHNL_IMG_CTRL_FORMAT_RAW8: c_uint = 0x0b;
pub const CHNL_IMG_CTRL_FORMAT_RAW10: c_uint = 0x0c;
pub const CHNL_IMG_CTRL_FORMAT_RAW10P: c_uint = 0x0d;
pub const CHNL_IMG_CTRL_FORMAT_RAW12: c_uint = 0x0e;
pub const CHNL_IMG_CTRL_FORMAT_RAW16: c_uint = 0x0f;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_1P8P: c_uint = 0x10;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_2P8P: c_uint = 0x11;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_3P8P: c_uint = 0x12;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_1P8: c_uint = 0x13;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_1P10: c_uint = 0x14;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_2P10: c_uint = 0x15;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_3P10: c_uint = 0x16;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_1P10P: c_uint = 0x18;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_2P10P: c_uint = 0x19;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_3P10P: c_uint = 0x1a;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_1P12: c_uint = 0x1c;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_2P12: c_uint = 0x1d;
pub const CHNL_IMG_CTRL_FORMAT_YUV444_3P12: c_uint = 0x1e;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_1P8P: c_uint = 0x20;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_2P8P: c_uint = 0x21;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_3P8P: c_uint = 0x22;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_1P10: c_uint = 0x24;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_2P10: c_uint = 0x25;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_3P10: c_uint = 0x26;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_1P10P: c_uint = 0x28;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_2P10P: c_uint = 0x29;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_3P10P: c_uint = 0x2a;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_1P12: c_uint = 0x2c;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_2P12: c_uint = 0x2d;
pub const CHNL_IMG_CTRL_FORMAT_YUV422_3P12: c_uint = 0x2e;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_2P8P: c_uint = 0x31;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_3P8P: c_uint = 0x32;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_2P10: c_uint = 0x35;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_3P10: c_uint = 0x36;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_2P10P: c_uint = 0x39;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_3P10P: c_uint = 0x3a;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_2P12: c_uint = 0x3d;
pub const CHNL_IMG_CTRL_FORMAT_YUV420_3P12: c_uint = 0x3e;

pub const CHNL_IMG_CTRL_DEINT_WEAVE_ODD_EVEN: c_int = 2;
pub const CHNL_IMG_CTRL_DEINT_WEAVE_EVEN_ODD: c_int = 3;
pub const CHNL_IMG_CTRL_DEINT_BLEND_ODD_EVEN: c_int = 4;
pub const CHNL_IMG_CTRL_DEINT_BLEND_EVEN_ODD: c_int = 5;
pub const CHNL_IMG_CTRL_DEINT_LDOUBLE_ODD_EVEN: c_int = 6;
pub const CHNL_IMG_CTRL_DEINT_LDOUBLE_EVEN_ODD: c_int = 7;

pub const CHNL_IMG_CTRL_CSC_MODE_YUV2RGB: c_int = 0;
pub const CHNL_IMG_CTRL_CSC_MODE_YCBCR2RGB: c_int = 1;
pub const CHNL_IMG_CTRL_CSC_MODE_RGB2YUV: c_int = 2;
pub const CHNL_IMG_CTRL_CSC_MODE_RGB2YCBCR: c_int = 3;

// Channel Output Buffer Control Register
pub const CHNL_OUT_BUF_CTRL: c_uint = 0x0008;

pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_V_NO_PANIC: c_int = 0;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_V_PANIC_25: c_int = 1;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_V_PANIC_50: c_int = 2;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_V_PANIC_75: c_int = 3;

pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_U_NO_PANIC: c_int = 0;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_U_PANIC_25: c_int = 1;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_U_PANIC_50: c_int = 2;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_U_PANIC_75: c_int = 3;

pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_Y_NO_PANIC: c_int = 0;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_Y_PANIC_25: c_int = 1;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_Y_PANIC_50: c_int = 2;
pub const CHNL_OUT_BUF_CTRL_OFLW_PANIC_SET_THD_Y_PANIC_75: c_int = 3;
// Channel Image Configuration
pub const CHNL_IMG_CFG: c_uint = 0x000c;

// Channel Interrupt Enable Register
pub const CHNL_IER: c_uint = 0x0010;

// Channel Status Register
pub const CHNL_STS: c_uint = 0x0014;

// Channel Scale Factor Register
pub const CHNL_SCALE_FACTOR: c_uint = 0x0018;

// Channel Scale Offset Register
pub const CHNL_SCALE_OFFSET: c_uint = 0x001c;

// Channel Crop Upper Left Corner Coordinate Register
pub const CHNL_CROP_ULC: c_uint = 0x0020;

// Channel Crop Lower Right Corner Coordinate Register
pub const CHNL_CROP_LRC: c_uint = 0x0024;

// Channel Color Space Conversion Coefficient Register 0
pub const CHNL_CSC_COEFF0: c_uint = 0x0028;

// Channel Color Space Conversion Coefficient Register 1
pub const CHNL_CSC_COEFF1: c_uint = 0x002c;

// Channel Color Space Conversion Coefficient Register 2
pub const CHNL_CSC_COEFF2: c_uint = 0x0030;

// Channel Color Space Conversion Coefficient Register 3
pub const CHNL_CSC_COEFF3: c_uint = 0x0034;

// Channel Color Space Conversion Coefficient Register 4
pub const CHNL_CSC_COEFF4: c_uint = 0x0038;

// Channel Color Space Conversion Coefficient Register 5
pub const CHNL_CSC_COEFF5: c_uint = 0x003c;

// Channel Alpha Value Register for ROI 0
pub const CHNL_ROI_0_ALPHA: c_uint = 0x0040;

// Channel Upper Left Coordinate Register for ROI 0
pub const CHNL_ROI_0_ULC: c_uint = 0x0044;

// Channel Lower Right Coordinate Register for ROI 0
pub const CHNL_ROI_0_LRC: c_uint = 0x0048;

// Channel Alpha Value Register for ROI 1
pub const CHNL_ROI_1_ALPHA: c_uint = 0x004c;

// Channel Upper Left Coordinate Register for ROI 1
pub const CHNL_ROI_1_ULC: c_uint = 0x0050;

// Channel Lower Right Coordinate Register for ROI 1
pub const CHNL_ROI_1_LRC: c_uint = 0x0054;

// Channel Alpha Value Register for ROI 2
pub const CHNL_ROI_2_ALPHA: c_uint = 0x0058;

// Channel Upper Left Coordinate Register for ROI 2
pub const CHNL_ROI_2_ULC: c_uint = 0x005c;

// Channel Lower Right Coordinate Register for ROI 2
pub const CHNL_ROI_2_LRC: c_uint = 0x0060;

// Channel Alpha Value Register for ROI 3
pub const CHNL_ROI_3_ALPHA: c_uint = 0x0064;

// Channel Upper Left Coordinate Register for ROI 3
pub const CHNL_ROI_3_ULC: c_uint = 0x0068;

// Channel Lower Right Coordinate Register for ROI 3
pub const CHNL_ROI_3_LRC: c_uint = 0x006c;

// Channel RGB or Luma (Y) Output Buffer 1 Address
pub const CHNL_OUT_BUF1_ADDR_Y: c_uint = 0x0070;
// Channel Chroma (U/Cb/UV/CbCr) Output Buffer 1 Address
pub const CHNL_OUT_BUF1_ADDR_U: c_uint = 0x0074;
// Channel Chroma (V/Cr) Output Buffer 1 Address
pub const CHNL_OUT_BUF1_ADDR_V: c_uint = 0x0078;
// Channel Output Buffer Pitch
pub const CHNL_OUT_BUF_PITCH: c_uint = 0x007c;

// Channel Input Buffer Address
pub const CHNL_IN_BUF_ADDR: c_uint = 0x0080;
// Channel Input Buffer Pitch
pub const CHNL_IN_BUF_PITCH: c_uint = 0x0084;

// Channel Memory Read Control
pub const CHNL_MEM_RD_CTRL: c_uint = 0x0088;

pub const CHNL_MEM_RD_CTRL_IMG_TYPE_BGR8P: c_uint = 0x00;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_RGB8P: c_uint = 0x01;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_XRGB8: c_uint = 0x02;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_RGBX8: c_uint = 0x03;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_XBGR8: c_uint = 0x04;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_RGB565: c_uint = 0x05;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_A2BGR10: c_uint = 0x06;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_A2RGB10: c_uint = 0x07;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV444_1P8P: c_uint = 0x08;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV444_1P10: c_uint = 0x09;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV444_1P10P: c_uint = 0x0a;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV444_1P12: c_uint = 0x0b;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV444_1P8: c_uint = 0x0c;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV422_1P8P: c_uint = 0x0d;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV422_1P10: c_uint = 0x0e;
pub const CHNL_MEM_RD_CTRL_IMG_TYPE_YUV422_1P12: c_uint = 0x0f;

// Channel RGB or Luma (Y) Output Buffer 2 Address
pub const CHNL_OUT_BUF2_ADDR_Y: c_uint = 0x008c;
// Channel Chroma (U/Cb/UV/CbCr) Output Buffer 2 Address
pub const CHNL_OUT_BUF2_ADDR_U: c_uint = 0x0090;
// Channel Chroma (V/Cr) Output Buffer 2 Address
pub const CHNL_OUT_BUF2_ADDR_V: c_uint = 0x0094;
// Channel scale image config
pub const CHNL_SCL_IMG_CFG: c_uint = 0x0098;

// Channel Flow Control Register
pub const CHNL_FLOW_CTRL: c_uint = 0x009c;

// Channel Output Y-Buffer 1 Extended Address Bits
pub const CHNL_Y_BUF1_XTND_ADDR: c_uint = 0x00a0;
// Channel Output U-Buffer 1 Extended Address Bits
pub const CHNL_U_BUF1_XTND_ADDR: c_uint = 0x00a4;
// Channel Output V-Buffer 1 Extended Address Bits
pub const CHNL_V_BUF1_XTND_ADDR: c_uint = 0x00a8;
// Channel Output Y-Buffer 2 Extended Address Bits
pub const CHNL_Y_BUF2_XTND_ADDR: c_uint = 0x00ac;
// Channel Output U-Buffer 2 Extended Address Bits
pub const CHNL_U_BUF2_XTND_ADDR: c_uint = 0x00b0;
// Channel Output V-Buffer 2 Extended Address Bits
pub const CHNL_V_BUF2_XTND_ADDR: c_uint = 0x00b4;
// Channel Input Buffer Extended Address Bits
pub const CHNL_IN_BUF_XTND_ADDR: c_uint = 0x00b8;
