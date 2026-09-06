//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/arm/mali-c55/mali-c55-registers.h
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
// ARM Mali-C55 ISP Driver - Register definitions
//
// Copyright (C) 2025 Ideas on Board Oy
//

// ISP Common 0x00000 - 0x000ff
pub const MALI_C55_REG_API: c_uint = 0x00000;
pub const MALI_C55_REG_PRODUCT: c_uint = 0x00004;
pub const MALI_C55_REG_VERSION: c_uint = 0x00008;
pub const MALI_C55_REG_REVISION: c_uint = 0x0000c;
pub const MALI_C55_REG_PULSE_MODE: c_uint = 0x0003c;
pub const MALI_C55_REG_INPUT_MODE_REQUEST: c_uint = 0x0009c;
pub const MALI_C55_INPUT_SAFE_STOP: c_uint = 0x00;
pub const MALI_C55_INPUT_SAFE_START: c_uint = 0x01;
pub const MALI_C55_REG_MODE_STATUS: c_uint = 0x000a0;
pub const MALI_C55_REG_INTERRUPT_MASK_VECTOR: c_uint = 0x00030;

pub const MALI_C55_REG_GLOBAL_MONITOR: c_uint = 0x00050;
pub const MALI_C55_REG_GEN_VIDEO: c_uint = 0x00080;

pub const MALI_C55_REG_MCU_CONFIG: c_uint = 0x00020;

pub const MALI_C55_REG_MCU_CONFIG_WRITE_PONG: c_uint = 0x00;

pub const MALI_C55_REG_PING_PONG_READ: c_uint = 0x00024;

pub const MALI_C55_REG_INTERRUPT_CLEAR_VECTOR: c_uint = 0x00034;
pub const MALI_C55_REG_INTERRUPT_CLEAR: c_uint = 0x00040;
pub const MALI_C55_REG_INTERRUPT_STATUS_VECTOR: c_uint = 0x00044;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_interrupts {
    MALI_C55_IRQ_ISP_START,
    MALI_C55_IRQ_ISP_DONE,
    MALI_C55_IRQ_MCM_ERROR,
    MALI_C55_IRQ_BROKEN_FRAME_ERROR,
    MALI_C55_IRQ_MET_AF_DONE,
    MALI_C55_IRQ_MET_AEXP_DONE,
    MALI_C55_IRQ_MET_AWB_DONE,
    MALI_C55_IRQ_AEXP_1024_DONE,
    MALI_C55_IRQ_IRIDIX_MET_DONE,
    MALI_C55_IRQ_LUT_INIT_DONE,
    MALI_C55_IRQ_FR_Y_DONE,
    MALI_C55_IRQ_FR_UV_DONE,
    MALI_C55_IRQ_DS_Y_DONE,
    MALI_C55_IRQ_DS_UV_DONE,
    MALI_C55_IRQ_LINEARIZATION_DONE,
    MALI_C55_IRQ_RAW_FRONTEND_DONE,
    MALI_C55_IRQ_NOISE_REDUCTION_DONE,
    MALI_C55_IRQ_IRIDIX_DONE,
    MALI_C55_IRQ_BAYER2RGB_DONE,
    MALI_C55_IRQ_WATCHDOG_TIMER,
    MALI_C55_IRQ_FRAME_COLLISION,
    MALI_C55_IRQ_UNUSED,
    MALI_C55_IRQ_DMA_ERROR,
    MALI_C55_IRQ_INPUT_STOPPED,
    MALI_C55_IRQ_MET_AWB_TARGET1_HIT,
    MALI_C55_IRQ_MET_AWB_TARGET2_HIT,
    MALI_C55_NUM_IRQ_BITS
}

pub const MALI_C55_REG_GLOBAL_PARAMETER_STATUS: c_uint = 0x00068;

pub const MALI_C55_REG_BLANKING: c_uint = 0x00084;

pub const MALI_C55_REG_HC_START: c_uint = 0x00088;

pub const MALI_C55_REG_HC_SIZE: c_uint = 0x0008c;

pub const MALI_C55_REG_VC_START_SIZE: c_uint = 0x00094;

pub const MALI_C55_REG_1024BIN_HIST: c_uint = 0x054a8;
pub const MALI_C55_1024BIN_HIST_SIZE: c_int = 4096;
// Ping/Pong Configuration Space
pub const MALI_C55_REG_BASE_ADDR: c_uint = 0x18e88;
pub const MALI_C55_REG_BYPASS_0: c_uint = 0x18eac;

pub const MALI_C55_REG_BYPASS_1: c_uint = 0x18eb0;

pub const MALI_C55_REG_BYPASS_2: c_uint = 0x18eb8;

pub const MALI_C55_REG_BYPASS_3: c_uint = 0x18ebc;

pub const MALI_C55_REG_BYPASS_4: c_uint = 0x18ec0;

pub const MALI_C55_REG_FR_BYPASS: c_uint = 0x18ec4;
pub const MALI_C55_REG_DS_BYPASS: c_uint = 0x18ec8;

pub const MALI_C55_REG_ISP_RAW_BYPASS: c_uint = 0x18ecc;

pub const MALI_C55_REG_ACTIVE_WIDTH_MASK: c_uint = 0xffff;
pub const MALI_C55_REG_ACTIVE_HEIGHT_MASK: c_uint = 0xffff0000;
pub const MALI_C55_REG_BAYER_ORDER: c_uint = 0x18e8c;

pub const MALI_C55_BAYER_ORDER_RGGB: c_int = 0;
pub const MALI_C55_BAYER_ORDER_GRBG: c_int = 1;
pub const MALI_C55_BAYER_ORDER_GBRG: c_int = 2;
pub const MALI_C55_BAYER_ORDER_BGGR: c_int = 3;
pub const MALI_C55_REG_METERING_CONFIG: c_uint = 0x18ed0;

pub const MALI_C55_REG_TPG_CH0: c_uint = 0x18ed8;

pub const MALI_C55_REG_TPG_R_BACKGROUND: c_uint = 0x18ee0;
pub const MALI_C55_REG_TPG_G_BACKGROUND: c_uint = 0x18ee4;
pub const MALI_C55_REG_TPG_B_BACKGROUND: c_uint = 0x18ee8;
pub const MALI_C55_TPG_BACKGROUND_MAX: c_uint = 0xfffff;
pub const MALI_C55_REG_INPUT_WIDTH: c_uint = 0x18f98;

pub const MALI_C55_REG_SPACE_SIZE: c_uint = 0x4000;
pub const MALI_C55_REG_CONFIG_SPACES_OFFSET: c_uint = 0x0ab6c;
pub const MALI_C55_CONFIG_SPACE_SIZE: c_uint = 0x1231c;
pub const MALI_C55_REG_DIGITAL_GAIN: c_uint = 0x1926c;

pub const MALI_C55_REG_DIGITAL_GAIN_OFFSET: c_uint = 0x19270;

pub const MALI_C55_REG_SINTER_CONFIG: c_uint = 0x19348;

// Temper DMA
pub const MALI_C55_REG_TEMPER_DMA_IO: c_uint = 0x1ab78;

// Black Level Correction Configuration
pub const MALI_C55_REG_SENSOR_OFF_PRE_SHA_00: c_uint = 0x1abcc;
pub const MALI_C55_REG_SENSOR_OFF_PRE_SHA_01: c_uint = 0x1abd0;
pub const MALI_C55_REG_SENSOR_OFF_PRE_SHA_10: c_uint = 0x1abd4;
pub const MALI_C55_REG_SENSOR_OFF_PRE_SHA_11: c_uint = 0x1abd8;
pub const MALI_C55_SENSOR_OFF_PRE_SHA_MASK: c_uint = 0xfffff;
// Lens Mesh Shading Configuration
pub const MALI_C55_REG_MESH_SHADING_TABLES: c_uint = 0x13074;
pub const MALI_C55_REG_MESH_SHADING_CONFIG: c_uint = 0x1abfc;

pub const MALI_C55_REG_MESH_SHADING_ALPHA_BANK: c_uint = 0x1ac04;

pub const MALI_C55_REG_MESH_SHADING_ALPHA: c_uint = 0x1ac08;

pub const MALI_C55_REG_MESH_SHADING_MESH_STRENGTH: c_uint = 0x1ac0c;

// AWB Gains Configuration
pub const MALI_C55_REG_AWB_GAINS1: c_uint = 0x1ac10;

pub const MALI_C55_REG_AWB_GAINS2: c_uint = 0x1ac14;

pub const MALI_C55_REG_AWB_GAINS1_AEXP: c_uint = 0x1ac18;
pub const MALI_C55_REG_AWB_GAINS2_AEXP: c_uint = 0x1ac1c;
// Colour Correction Matrix Configuration
pub const MALI_C55_REG_CCM_ENABLE: c_uint = 0x1b07c;

pub const MALI_C55_REG_CCM_COEF_R_R: c_uint = 0x1b080;
pub const MALI_C55_REG_CCM_COEF_R_G: c_uint = 0x1b084;
pub const MALI_C55_REG_CCM_COEF_R_B: c_uint = 0x1b088;
pub const MALI_C55_REG_CCM_COEF_G_R: c_uint = 0x1b090;
pub const MALI_C55_REG_CCM_COEF_G_G: c_uint = 0x1b094;
pub const MALI_C55_REG_CCM_COEF_G_B: c_uint = 0x1b098;
pub const MALI_C55_REG_CCM_COEF_B_R: c_uint = 0x1b0a0;
pub const MALI_C55_REG_CCM_COEF_B_G: c_uint = 0x1b0a4;
pub const MALI_C55_REG_CCM_COEF_B_B: c_uint = 0x1b0a8;

pub const MALI_C55_REG_CCM_ANTIFOG_GAIN_R: c_uint = 0x1b0b0;
pub const MALI_C55_REG_CCM_ANTIFOG_GAIN_G: c_uint = 0x1b0b4;
pub const MALI_C55_REG_CCM_ANTIFOG_GAIN_B: c_uint = 0x1b0b8;

pub const MALI_C55_REG_CCM_ANTIFOG_OFFSET_R: c_uint = 0x1b0c0;
pub const MALI_C55_REG_CCM_ANTIFOG_OFFSET_G: c_uint = 0x1b0c4;
pub const MALI_C55_REG_CCM_ANTIFOG_OFFSET_B: c_uint = 0x1b0c8;

// AWB Statistics Configuration
pub const MALI_C55_REG_AWB_STATS_MODE: c_uint = 0x1b29c;

pub const MALI_C55_REG_AWB_WHITE_LEVEL: c_uint = 0x1b2a0;

pub const MALI_C55_REG_AWB_BLACK_LEVEL: c_uint = 0x1b2a4;

pub const MALI_C55_REG_AWB_CR_MAX: c_uint = 0x1b2a8;

pub const MALI_C55_REG_AWB_CR_MIN: c_uint = 0x1b2ac;

pub const MALI_C55_REG_AWB_CB_MAX: c_uint = 0x1b2b0;

pub const MALI_C55_REG_AWB_CB_MIN: c_uint = 0x1b2b4;

pub const MALI_C55_REG_AWB_NODES_USED: c_uint = 0x1b2c4;

pub const MALI_C55_REG_AWB_CR_HIGH: c_uint = 0x1b2c8;

pub const MALI_C55_REG_AWB_CR_LOW: c_uint = 0x1b2cc;

pub const MALI_C55_REG_AWB_CB_HIGH: c_uint = 0x1b2d0;

pub const MALI_C55_REG_AWB_CB_LOW: c_uint = 0x1b2d4;

// AEXP Metering Histogram Configuration
pub const MALI_C55_REG_AEXP_HIST_BASE: c_uint = 0x1b730;
pub const MALI_C55_REG_AEXP_IHIST_BASE: c_uint = 0x1bbac;
pub const MALI_C55_AEXP_HIST_SKIP_OFFSET: c_int = 0;

pub const MALI_C55_AEXP_HIST_SCALE_OFFSET: c_int = 4;

pub const MALI_C55_AEXP_HIST_PLANE_MODE_OFFSET: c_int = 16;

pub const MALI_C55_AEXP_HIST_NODES_USED_OFFSET: c_int = 52;

pub const MALI_C55_AEXP_HIST_ZONE_WEIGHTS_OFFSET: c_int = 56;
pub const MALI_C55_AEXP_HIST_ZONE_WEIGHT_MASK: c_uint = 0x0f0f0f0f;
//
// The Mali-C55 ISP has up to two output pipes; known as full resolution and
// down scaled. The register space for these is laid out identically, but offset
// by 372 bytes.
//
pub const MALI_C55_CAP_DEV_FR_REG_OFFSET: c_uint = 0x0;
pub const MALI_C55_CAP_DEV_DS_REG_OFFSET: c_uint = 0x174;
pub const MALI_C55_REG_CS_CONV_CONFIG: c_uint = 0x1c098;

pub const MALI_C55_REG_Y_WRITER_MODE: c_uint = 0x1c0ec;
pub const MALI_C55_REG_UV_WRITER_MODE: c_uint = 0x1c144;

pub const MALI_C55_OUTPUT_DISABLED: c_int = 0;
pub const MALI_C55_OUTPUT_RGB32: c_int = 1;
pub const MALI_C55_OUTPUT_A2R10G10B10: c_int = 2;
pub const MALI_C55_OUTPUT_RGB565: c_int = 3;
pub const MALI_C55_OUTPUT_RGB24: c_int = 4;
pub const MALI_C55_OUTPUT_GEN32: c_int = 5;
pub const MALI_C55_OUTPUT_RAW16: c_int = 6;
pub const MALI_C55_OUTPUT_AYUV: c_int = 8;
pub const MALI_C55_OUTPUT_Y410: c_int = 9;
pub const MALI_C55_OUTPUT_YUY2: c_int = 10;
pub const MALI_C55_OUTPUT_UYVY: c_int = 11;
pub const MALI_C55_OUTPUT_Y210: c_int = 12;
pub const MALI_C55_OUTPUT_NV12_21: c_int = 13;
pub const MALI_C55_OUTPUT_YUV_420_422: c_int = 17;
pub const MALI_C55_OUTPUT_P210_P010: c_int = 19;
pub const MALI_C55_OUTPUT_YUV422: c_int = 20;

pub const MALI_C55_OUTPUT_PLANE_ALT0: c_int = 0;
pub const MALI_C55_OUTPUT_PLANE_ALT1: c_int = 1;
pub const MALI_C55_OUTPUT_PLANE_ALT2: c_int = 2;

pub const MALI_C55_REG_ACTIVE_OUT_Y_SIZE: c_uint = 0x1c0f0;
pub const MALI_C55_REG_ACTIVE_OUT_UV_SIZE: c_uint = 0x1c148;

pub const MALI_C55_REG_Y_WRITER_BANKS_BASE: c_uint = 0x1c0f4;
pub const MALI_C55_REG_Y_WRITER_BANKS_CONFIG: c_uint = 0x1c108;

pub const MALI_C55_REG_Y_WRITER_OFFSET: c_uint = 0x1c10c;
pub const MALI_C55_REG_UV_WRITER_BANKS_BASE: c_uint = 0x1c14c;
pub const MALI_C55_REG_UV_WRITER_BANKS_CONFIG: c_uint = 0x1c160;

pub const MALI_C55_REG_UV_WRITER_OFFSET: c_uint = 0x1c164;
// Macro flag: #define MALI_C55_REG_TEST_GEN_CH0_OFF_ON
pub const MALI_C55_REG_TEST_GEN_CH0_PATTERN_TYPE: c_uint = 0x18edc;
pub const MALI_C55_REG_CROP_EN: c_uint = 0x1c028;

pub const MALI_C55_REG_CROP_X_START: c_uint = 0x1c02c;
pub const MALI_C55_REG_CROP_Y_START: c_uint = 0x1c030;
pub const MALI_C55_REG_CROP_X_SIZE: c_uint = 0x1c034;
pub const MALI_C55_REG_CROP_Y_SIZE: c_uint = 0x1c038;
pub const MALI_C55_REG_SCALER_TIMEOUT_EN: c_uint = 0x1c040;

pub const MALI_C55_REG_SCALER_IN_WIDTH: c_uint = 0x1c044;
pub const MALI_C55_REG_SCALER_IN_HEIGHT: c_uint = 0x1c048;
pub const MALI_C55_REG_SCALER_OUT_WIDTH: c_uint = 0x1c04c;
pub const MALI_C55_REG_SCALER_OUT_HEIGHT: c_uint = 0x1c050;
pub const MALI_C55_REG_SCALER_HFILT_TINC: c_uint = 0x1c054;
pub const MALI_C55_REG_SCALER_HFILT_COEF: c_uint = 0x1c058;
pub const MALI_C55_REG_SCALER_VFILT_TINC: c_uint = 0x1c05c;
pub const MALI_C55_REG_SCALER_VFILT_COEF: c_uint = 0x1c060;
pub const MALI_C55_REG_GAMMA_RGB_ENABLE: c_uint = 0x1c064;

pub const MALI_C55_REG_GAMMA_GAINS_RG: c_uint = 0x1c068;

pub const MALI_C55_REG_GAMMA_GAINS_B: c_uint = 0x1c06c;

pub const MALI_C55_REG_GAMMA_OFFSETS_RG: c_uint = 0x1c070;

pub const MALI_C55_REG_GAMMA_OFFSETS_B: c_uint = 0x1c074;

//
// A re-definition of an above register. These will usually be written on a per
// capture device basis and handled with mali_c55_cap_dev_write(), but on
// startup is written by core.c
//
pub const MALI_C55_REG_FR_GAMMA_RGB_ENABLE: c_uint = 0x1c064;
pub const MALI_C55_REG_DS_GAMMA_RGB_ENABLE: c_uint = 0x1c1d8;
pub const MALI_C55_REG_FR_GAMMA_RGB_MEM: c_uint = 0x18280;
pub const MALI_C55_REG_DS_GAMMA_RGB_MEM: c_uint = 0x18484;
pub const MALI_C55_REG_FR_SCALER_HFILT: c_uint = 0x34a8;
pub const MALI_C55_REG_FR_SCALER_VFILT: c_uint = 0x44a8;
pub const MALI_C55_REG_DS_SCALER_HFILT: c_uint = 0x14a8;
pub const MALI_C55_REG_DS_SCALER_VFILT: c_uint = 0x24a8;
