//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/adc/qcom-vadc-common.h
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
// Code shared between the different Qualcomm PMIC voltage ADCs
//

pub const VADC_CONV_TIME_MIN_US: c_int = 2000;
pub const VADC_CONV_TIME_MAX_US: c_int = 2100;
// Min ADC code represents 0V
pub const VADC_MIN_ADC_CODE: c_uint = 0x6000;
// Max ADC code represents full-scale range of 1.8V
pub const VADC_MAX_ADC_CODE: c_uint = 0xa800;
pub const VADC_ABSOLUTE_RANGE_UV: c_int = 625000;
pub const VADC_RATIOMETRIC_RANGE: c_int = 1800;

pub const VADC_DECIMATION_MIN: c_int = 512;
pub const VADC_DECIMATION_MAX: c_int = 4096;

pub const ADC5_DECIMATION_SHORT: c_int = 250;
pub const ADC5_DECIMATION_MEDIUM: c_int = 420;
pub const ADC5_DECIMATION_LONG: c_int = 840;
// Default decimation - 1024 for rev2, 840 for pmic5
pub const ADC5_DECIMATION_DEFAULT: c_int = 2;
pub const ADC5_DECIMATION_SAMPLES_MAX: c_int = 3;
pub const VADC_HW_SETTLE_DELAY_MAX: c_int = 10000;
pub const VADC_HW_SETTLE_SAMPLES_MAX: c_int = 16;
pub const VADC_AVG_SAMPLES_MAX: c_int = 512;
pub const ADC5_AVG_SAMPLES_MAX: c_int = 16;
pub const PMIC5_CHG_TEMP_SCALE_FACTOR: c_int = 377500;
pub const PMIC5_SMB_TEMP_CONSTANT: c_int = 419400;
pub const PMIC5_SMB_TEMP_SCALE_FACTOR: c_int = 356;

pub const VADC5_MAX_CODE: c_uint = 0x7fff;
pub const ADC5_FULL_SCALE_CODE: c_uint = 0x70e4;
pub const ADC5_USR_DATA_CHECK: c_uint = 0x8000;
pub const R_PU_100K: c_int = 100000;

//
// VADC_CALIB_ABSOLUTE: uses the 625mV and 1.25V as reference channels.
// VADC_CALIB_RATIOMETRIC: uses the reference voltage (1.8V) and GND for
// calibration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vadc_calibration {
    VADC_CALIB_ABSOLUTE = 0,
    VADC_CALIB_RATIOMETRIC
}

//
// struct vadc_linear_graph - Represent ADC characteristics.
// @dy: numerator slope to calculate the gain.
// @dx: denominator slope to calculate the gain.
// @gnd: A/D word of the ground reference used for the channel.
//
// Each ADC device has different offset and gain parameters which are
// computed to calibrate the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vadc_linear_graph {
    pub dy: i32,
    pub dx: i32,
    pub gnd: i32,
}

//
// enum vadc_scale_fn_type - Scaling function to convert ADC code to
// physical scaled units for the channel.
// @SCALE_DEFAULT: Default scaling to convert raw adc code to voltage (uV).
// @SCALE_THERM_100K_PULLUP: Returns temperature in millidegC.
// Uses a mapping table with 100K pullup.
// @SCALE_PMIC_THERM: Returns result in milli degree's Centigrade.
// @SCALE_XOTHERM: Returns XO thermistor voltage in millidegC.
// @SCALE_PMI_CHG_TEMP: Conversion for PMI CHG temp
// @SCALE_HW_CALIB_DEFAULT: Default scaling to convert raw adc code to
// voltage (uV) with hardware applied offset/slope values to adc code.
// @SCALE_HW_CALIB_THERM_100K_PULLUP: Returns temperature in millidegC using
// lookup table. The hardware applies offset/slope to adc code.
// @SCALE_HW_CALIB_XOTHERM: Returns XO thermistor voltage in millidegC using
// 100k pullup. The hardware applies offset/slope to adc code.
// @SCALE_HW_CALIB_THERM_100K_PU_PM7: Returns temperature in millidegC using
// lookup table for PMIC7. The hardware applies offset/slope to adc code.
// @SCALE_HW_CALIB_PMIC_THERM: Returns result in milli degree's Centigrade.
// The hardware applies offset/slope to adc code.
// @SCALE_HW_CALIB_PMIC_THERM: Returns result in milli degree's Centigrade.
// The hardware applies offset/slope to adc code. This is for PMIC7.
// @SCALE_HW_CALIB_PM5_CHG_TEMP: Returns result in millidegrees for PMIC5
// charger temperature.
// @SCALE_HW_CALIB_PM5_SMB_TEMP: Returns result in millidegrees for PMIC5
// SMB1390 temperature.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vadc_scale_fn_type {
    SCALE_DEFAULT = 0,
    SCALE_THERM_100K_PULLUP,
    SCALE_PMIC_THERM,
    SCALE_XOTHERM,
    SCALE_PMI_CHG_TEMP,
    SCALE_HW_CALIB_DEFAULT,
    SCALE_HW_CALIB_THERM_100K_PULLUP,
    SCALE_HW_CALIB_XOTHERM,
    SCALE_HW_CALIB_THERM_100K_PU_PM7,
    SCALE_HW_CALIB_PMIC_THERM,
    SCALE_HW_CALIB_PMIC_THERM_PM7,
    SCALE_HW_CALIB_PM5_CHG_TEMP,
    SCALE_HW_CALIB_PM5_SMB_TEMP,
// private:
    SCALE_HW_CALIB_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc5_data {
    pub full_scale_code_volt: u32,
    pub full_scale_code_cur: u32,
    pub adc_chans: *const adc5_channels,
    pub info: *const iio_info,
    pub decimation: *mut c_uint,
    pub hw_settle_1: *mut c_uint,
    pub hw_settle_2: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_adc5_scale_type {
    pub result): *const *const adc5_data data, u16 adc_code, int,
}

extern "C" {
    pub fn qcom_adc_tm5_gen2_temp_res_scale(temp: c_int) -> u16;
}
extern "C" {
    pub fn qcom_adc5_prescaling_from_dt(num: u32, den: u32) -> c_int;
}
extern "C" {
    pub fn qcom_adc5_hw_settle_time_from_dt(value: u32, hw_settle: *const c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_adc5_avg_samples_from_dt(value: u32) -> c_int;
}
extern "C" {
    pub fn qcom_adc5_decimation_from_dt(value: u32, decimation: *const c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_vadc_decimation_from_dt(value: u32) -> c_int;
}
