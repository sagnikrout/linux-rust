//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/adc/qcom-adc5-gen3-common.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Code used in the main and auxiliary Qualcomm PMIC voltage ADCs
// of type ADC5 Gen3.
//

pub const ADC5_GEN3_HS: c_uint = 0x45;

pub const ADC5_GEN3_STATUS1: c_uint = 0x46;

pub const ADC5_GEN3_TM_EN_STS: c_uint = 0x47;
pub const ADC5_GEN3_TM_HIGH_STS: c_uint = 0x48;
pub const ADC5_GEN3_TM_LOW_STS: c_uint = 0x49;
pub const ADC5_GEN3_EOC_STS: c_uint = 0x4a;

pub const ADC5_GEN3_EOC_CLR: c_uint = 0x4b;
pub const ADC5_GEN3_TM_HIGH_STS_CLR: c_uint = 0x4c;
pub const ADC5_GEN3_TM_LOW_STS_CLR: c_uint = 0x4d;
pub const ADC5_GEN3_CONV_ERR_CLR: c_uint = 0x4e;

pub const ADC5_GEN3_SID: c_uint = 0x4f;

pub const ADC5_GEN3_PERPH_CH: c_uint = 0x50;

pub const ADC5_GEN3_TIMER_SEL: c_uint = 0x51;
pub const ADC5_GEN3_TIME_IMMEDIATE: c_uint = 0x1;
pub const ADC5_GEN3_DIG_PARAM: c_uint = 0x52;

pub const ADC5_GEN3_FAST_AVG: c_uint = 0x53;

pub const ADC5_GEN3_ADC_CH_SEL_CTL: c_uint = 0x54;
pub const ADC5_GEN3_DELAY_CTL: c_uint = 0x55;

pub const ADC5_GEN3_CH_EN: c_uint = 0x56;

pub const ADC5_GEN3_LOW_THR0: c_uint = 0x57;
pub const ADC5_GEN3_LOW_THR1: c_uint = 0x58;
pub const ADC5_GEN3_HIGH_THR0: c_uint = 0x59;
pub const ADC5_GEN3_HIGH_THR1: c_uint = 0x5a;

pub const ADC5_GEN3_CONV_REQ: c_uint = 0xe5;

// ADC channels for PMIC5 Gen3
pub const ADC5_GEN3_REF_GND: c_uint = 0x00;
pub const ADC5_GEN3_1P25VREF: c_uint = 0x01;
pub const ADC5_GEN3_DIE_TEMP: c_uint = 0x03;
pub const ADC5_GEN3_USB_SNS_V_16: c_uint = 0x11;
pub const ADC5_GEN3_VIN_DIV16_MUX: c_uint = 0x12;
pub const ADC5_GEN3_VPH_PWR: c_uint = 0x8e;
pub const ADC5_GEN3_VBAT_SNS_QBG: c_uint = 0x8f;
// 100k pull-up channels
pub const ADC5_GEN3_AMUX1_THM_100K_PU: c_uint = 0x44;
pub const ADC5_GEN3_AMUX2_THM_100K_PU: c_uint = 0x45;
pub const ADC5_GEN3_AMUX3_THM_100K_PU: c_uint = 0x46;
pub const ADC5_GEN3_AMUX4_THM_100K_PU: c_uint = 0x47;
pub const ADC5_GEN3_AMUX5_THM_100K_PU: c_uint = 0x48;
pub const ADC5_GEN3_AMUX6_THM_100K_PU: c_uint = 0x49;
pub const ADC5_GEN3_AMUX1_GPIO_100K_PU: c_uint = 0x4a;
pub const ADC5_GEN3_AMUX2_GPIO_100K_PU: c_uint = 0x4b;
pub const ADC5_GEN3_AMUX3_GPIO_100K_PU: c_uint = 0x4c;
pub const ADC5_GEN3_AMUX4_GPIO_100K_PU: c_uint = 0x4d;
pub const ADC5_MAX_CHANNEL: c_uint = 0xc0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adc5_cal_method {
    ADC5_NO_CAL = 0,
    ADC5_RATIOMETRIC_CAL,
    ADC5_ABSOLUTE_CAL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adc5_time_select {
    MEAS_INT_DISABLE = 0,
    MEAS_INT_IMMEDIATE,
    MEAS_INT_50MS,
    MEAS_INT_100MS,
    MEAS_INT_1S,
    MEAS_INT_NONE,
}

//
// struct adc5_sdam_data - data per SDAM allocated for adc usage
// @base_addr: base address for the ADC SDAM peripheral.
// @irq_name: ADC IRQ name.
// @irq: ADC IRQ number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc5_sdam_data {
    pub base_addr: u16,
    pub irq_name: *const c_char,
    pub irq: c_int,
}

//
// struct adc5_device_data - Top-level ADC device data
// @regmap: ADC peripheral register map field.
// @base: array of SDAM data.
// @num_sdams: number of ADC SDAM peripherals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc5_device_data {
    pub regmap: *mut regmap,
    pub base: *mut adc5_sdam_data,
    pub num_sdams: c_int,
}

//
// struct adc5_channel_common_prop - ADC channel properties (common to ADC and TM).
// @channel: channel number, refer to the channel list.
// @cal_method: calibration method.
// @decimation: sampling rate supported for the channel.
// @sid: ID of PMIC owning the channel.
// @label: Channel name used in device tree.
// @prescale: channel scaling performed on the input signal.
// @hw_settle_time_us: the time between AMUX being configured and the
// start of conversion in uS.
// @avg_samples: ability to provide single result from the ADC
// that is an average of multiple measurements.
// @scale_fn_type: Represents the scaling function to convert voltage
// physical units desired by the client for the channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc5_channel_common_prop {
    pub channel: c_uint,
    pub cal_method: adc5_cal_method,
    pub decimation: c_uint,
    pub sid: c_uint,
    pub label: *const c_char,
    pub prescale: c_uint,
    pub hw_settle_time_us: c_uint,
    pub avg_samples: c_uint,
    pub scale_fn_type: vadc_scale_fn_type,
}

//
// struct tm5_aux_dev_wrapper - wrapper structure around TM auxiliary device
// @aux_dev: TM auxiliary device structure.
// @dev_data: Top-level ADC device data.
// @tm_props: Array of common ADC channel properties for TM channels.
// @n_tm_channels: number of TM channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm5_aux_dev_wrapper {
    pub aux_dev: auxiliary_device,
    pub dev_data: *mut adc5_device_data,
    pub tm_props: *mut adc5_channel_common_prop,
    pub n_tm_channels: c_uint,
}

extern "C" {
    pub fn adc5_gen3_mutex_lock(dev: *mut device);
}
extern "C" {
    pub fn adc5_gen3_mutex_unlock(dev: *mut device);
}
