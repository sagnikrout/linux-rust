//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/dac/ad3552r.h
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
// AD3552R Digital <-> Analog converters common header
//
// Copyright 2021-2024 Analog Devices Inc.
// Author: Angelo Dureghello <adureghello@baylibre.com>
//
// Register addresses
// Primary address space
pub const AD3552R_REG_ADDR_INTERFACE_CONFIG_A: c_uint = 0x00;

pub const AD3552R_REG_ADDR_INTERFACE_CONFIG_B: c_uint = 0x01;

pub const AD3552R_REG_ADDR_DEVICE_CONFIG: c_uint = 0x02;

pub const AD3552R_REG_ADDR_CHIP_TYPE: c_uint = 0x03;

pub const AD3552R_REG_ADDR_PRODUCT_ID_L: c_uint = 0x04;
pub const AD3552R_REG_ADDR_PRODUCT_ID_H: c_uint = 0x05;
pub const AD3552R_REG_ADDR_CHIP_GRADE: c_uint = 0x06;

pub const AD3552R_REG_ADDR_SCRATCH_PAD: c_uint = 0x0A;
pub const AD3552R_REG_ADDR_SPI_REVISION: c_uint = 0x0B;
pub const AD3552R_REG_ADDR_VENDOR_L: c_uint = 0x0C;
pub const AD3552R_REG_ADDR_VENDOR_H: c_uint = 0x0D;
pub const AD3552R_REG_ADDR_STREAM_MODE: c_uint = 0x0E;

pub const AD3552R_REG_ADDR_TRANSFER_REGISTER: c_uint = 0x0F;

pub const AD3552R_REG_ADDR_INTERFACE_CONFIG_C: c_uint = 0x10;

pub const AD3552R_REG_ADDR_INTERFACE_STATUS_A: c_uint = 0x11;

pub const AD3552R_REG_ADDR_INTERFACE_CONFIG_D: c_uint = 0x14;

pub const AD3552R_REG_ADDR_SH_REFERENCE_CONFIG: c_uint = 0x15;

pub const AD3552R_REG_ADDR_ERR_ALARM_MASK: c_uint = 0x16;

pub const AD3552R_REG_ADDR_ERR_STATUS: c_uint = 0x17;

pub const AD3552R_REG_ADDR_POWERDOWN_CONFIG: c_uint = 0x18;

pub const AD3552R_REG_ADDR_CH0_CH1_OUTPUT_RANGE: c_uint = 0x19;

//
// Secondary region
// For multibyte registers specify the highest address because the access is
// done in descending order
//
pub const AD3552R_SECONDARY_REGION_START: c_uint = 0x28;
pub const AD3552R_REG_ADDR_HW_LDAC_16B: c_uint = 0x28;

pub const AD3552R_REG_ADDR_DAC_PAGE_MASK_16B: c_uint = 0x2E;
pub const AD3552R_REG_ADDR_CH_SELECT_16B: c_uint = 0x2F;
pub const AD3552R_REG_ADDR_INPUT_PAGE_MASK_16B: c_uint = 0x31;
pub const AD3552R_REG_ADDR_SW_LDAC_16B: c_uint = 0x32;

// 3 bytes registers
pub const AD3552R_REG_START_24B: c_uint = 0x37;
pub const AD3552R_REG_ADDR_HW_LDAC_24B: c_uint = 0x37;

pub const AD3552R_REG_ADDR_DAC_PAGE_MASK_24B: c_uint = 0x40;
pub const AD3552R_REG_ADDR_CH_SELECT_24B: c_uint = 0x41;
pub const AD3552R_REG_ADDR_INPUT_PAGE_MASK_24B: c_uint = 0x44;
pub const AD3552R_REG_ADDR_SW_LDAC_24B: c_uint = 0x45;

pub const AD3552R_MAX_CH: c_int = 2;

pub const AD3552R_MAX_REG_SIZE: c_int = 3;

pub const AD3552R_DEFAULT_CONFIG_B_VALUE: c_uint = 0x8;
pub const AD3552R_SCRATCH_PAD_TEST_VAL1: c_uint = 0x34;
pub const AD3552R_SCRATCH_PAD_TEST_VAL2: c_uint = 0xB2;
pub const AD3552R_GAIN_SCALE: c_int = 1000;
pub const AD3552R_LDAC_PULSE_US: c_int = 100;

pub const AD3552R_MAX_RANGES: c_int = 5;
pub const AD3542R_MAX_RANGES: c_int = 5;
pub const AD3552R_SPI: c_int = 0;
pub const AD3552R_DUAL_SPI: c_int = 1;
pub const AD3552R_QUAD_SPI: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad3552r_id {
    AD3541R_ID = 0x400b,
    AD3542R_ID = 0x4009,
    AD3551R_ID = 0x400a,
    AD3552R_ID = 0x4008,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad3552r_model_data {
    pub model_name: *const c_char,
    pub chip_id: ad3552r_id,
    pub num_hw_channels: c_uint,
    pub (*ranges_table)[2]: *const i32,
    pub num_ranges: c_int,
    pub requires_output_range: bool,
    pub num_spi_data_lanes: c_int,
    pub max_reg_addr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad3552r_ch_data {
    pub scale_int: i32,
    pub scale_dec: i32,
    pub offset_int: i32,
    pub offset_dec: i32,
    pub gain_offset: i16,
    pub rfb: u16,
    pub n: u8,
    pub p: u8,
    pub range: u8,
    pub range_override: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad3552r_ch_gain_scaling {
// Gain scaling of 1
    AD3552R_CH_GAIN_SCALING_1,
// Gain scaling of 0.5
    AD3552R_CH_GAIN_SCALING_0_5,
// Gain scaling of 0.25
    AD3552R_CH_GAIN_SCALING_0_25,
// Gain scaling of 0.125
    AD3552R_CH_GAIN_SCALING_0_125,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad3552r_ch_vref_select {
// Internal source with Vref I/O floating
    AD3552R_INTERNAL_VREF_PIN_FLOATING,
// Internal source with Vref I/O at 2.5V
    AD3552R_INTERNAL_VREF_PIN_2P5V,
// External source with Vref I/O as input
    AD3552R_EXTERNAL_VREF_PIN_INPUT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad3542r_ch_output_range {
// Range from 0 V to 2.5 V. Requires Rfb1x connection
    AD3542R_CH_OUTPUT_RANGE_0__2P5V,
// Range from 0 V to 5 V. Requires Rfb1x connection
    AD3542R_CH_OUTPUT_RANGE_0__5V,
// Range from 0 V to 10 V. Requires Rfb2x connection
    AD3542R_CH_OUTPUT_RANGE_0__10V,
// Range from -5 V to 5 V. Requires Rfb2x connection
    AD3542R_CH_OUTPUT_RANGE_NEG_5__5V,
// Range from -2.5 V to 7.5 V. Requires Rfb2x connection
    AD3542R_CH_OUTPUT_RANGE_NEG_2P5__7P5V,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad3552r_ch_output_range {
// Range from 0 V to 2.5 V. Requires Rfb1x connection
    AD3552R_CH_OUTPUT_RANGE_0__2P5V,
// Range from 0 V to 5 V. Requires Rfb1x connection
    AD3552R_CH_OUTPUT_RANGE_0__5V,
// Range from 0 V to 10 V. Requires Rfb2x connection
    AD3552R_CH_OUTPUT_RANGE_0__10V,
// Range from -5 V to 5 V. Requires Rfb2x connection
    AD3552R_CH_OUTPUT_RANGE_NEG_5__5V,
// Range from -10 V to 10 V. Requires Rfb4x connection
    AD3552R_CH_OUTPUT_RANGE_NEG_10__10V,
}

extern "C" {
    pub fn ad3552r_calc_custom_gain(p: u8, n: u8, goffs: i16) -> u16;
}
extern "C" {
    pub fn ad3552r_get_ref_voltage(dev: *mut device, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ad3552r_get_drive_strength(dev: *mut device, val: *mut u32) -> c_int;
}
