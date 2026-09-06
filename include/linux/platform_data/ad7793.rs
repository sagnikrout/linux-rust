//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/ad7793.h
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
// AD7792/AD7793 SPI ADC driver
//
// Copyright 2011 Analog Devices Inc.
//
// enum ad7793_clock_source - AD7793 clock source selection
// @AD7793_CLK_SRC_INT: Internal 64 kHz clock, not available at the CLK pin.
// @AD7793_CLK_SRC_INT_CO: Internal 64 kHz clock, available at the CLK pin.
// @AD7793_CLK_SRC_EXT: Use external clock.
// @AD7793_CLK_SRC_EXT_DIV2: Use external clock divided by 2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7793_clock_source {
    AD7793_CLK_SRC_INT,
    AD7793_CLK_SRC_INT_CO,
    AD7793_CLK_SRC_EXT,
    AD7793_CLK_SRC_EXT_DIV2,
}

//
// enum ad7793_bias_voltage - AD7793 bias voltage selection
// @AD7793_BIAS_VOLTAGE_DISABLED: Bias voltage generator disabled
// @AD7793_BIAS_VOLTAGE_AIN1: Bias voltage connected to AIN1(-).
// @AD7793_BIAS_VOLTAGE_AIN2: Bias voltage connected to AIN2(-).
// @AD7793_BIAS_VOLTAGE_AIN3: Bias voltage connected to AIN3(-).
// Only valid for AD7795/AD7796.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7793_bias_voltage {
    AD7793_BIAS_VOLTAGE_DISABLED,
    AD7793_BIAS_VOLTAGE_AIN1,
    AD7793_BIAS_VOLTAGE_AIN2,
    AD7793_BIAS_VOLTAGE_AIN3,
}

//
// enum ad7793_refsel - AD7793 reference voltage selection
// @AD7793_REFSEL_REFIN1: External reference applied between REFIN1(+)
// and REFIN1(-).
// @AD7793_REFSEL_REFIN2: External reference applied between REFIN2(+)
// and REFIN1(-). Only valid for AD7795/AD7796.
// @AD7793_REFSEL_INTERNAL: Internal 1.17 V reference.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7793_refsel {
    AD7793_REFSEL_REFIN1 = 0,
    AD7793_REFSEL_REFIN2 = 1,
    AD7793_REFSEL_INTERNAL = 2,
}

//
// enum ad7793_current_source_direction - AD7793 excitation current direction
// @AD7793_IEXEC1_IOUT1_IEXEC2_IOUT2: Current source IEXC1 connected to pin
// IOUT1, current source IEXC2 connected to pin IOUT2.
// @AD7793_IEXEC1_IOUT2_IEXEC2_IOUT1: Current source IEXC2 connected to pin
// IOUT1, current source IEXC1 connected to pin IOUT2.
// @AD7793_IEXEC1_IEXEC2_IOUT1: Both current sources connected to pin IOUT1.
// Only valid when the current sources are set to 10 uA or 210 uA.
// @AD7793_IEXEC1_IEXEC2_IOUT2: Both current sources connected to Pin IOUT2.
// Only valid when the current ources are set to 10 uA or 210 uA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7793_current_source_direction {
    AD7793_IEXEC1_IOUT1_IEXEC2_IOUT2 = 0,
    AD7793_IEXEC1_IOUT2_IEXEC2_IOUT1 = 1,
    AD7793_IEXEC1_IEXEC2_IOUT1 = 2,
    AD7793_IEXEC1_IEXEC2_IOUT2 = 3,
}

//
// enum ad7793_excitation_current - AD7793 excitation current selection
// @AD7793_IX_DISABLED: Excitation current Disabled.
// @AD7793_IX_10uA: Enable 10 micro-ampere excitation current.
// @AD7793_IX_210uA: Enable 210 micro-ampere excitation current.
// @AD7793_IX_1mA: Enable 1 milli-Ampere excitation current.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad7793_excitation_current {
    AD7793_IX_DISABLED = 0,
    AD7793_IX_10uA = 1,
    AD7793_IX_210uA = 2,
    AD7793_IX_1mA = 3,
}

//
// struct ad7793_platform_data - AD7793 platform data
// @clock_src: Clock source selection
// @burnout_current: If set to true the 100nA burnout current is enabled.
// @boost_enable: Enable boost for the bias voltage generator.
// @buffered: If set to true configure the device for buffered input mode.
// @unipolar: If set to true sample in unipolar mode, if set to false sample in
// bipolar mode.
// @refsel: Reference voltage selection
// @bias_voltage: Bias voltage selection
// @exitation_current: Excitation current selection
// @current_source_direction: Excitation current direction selection
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7793_platform_data {
    pub clock_src: ad7793_clock_source,
    pub burnout_current: bool,
    pub boost_enable: bool,
    pub buffered: bool,
    pub unipolar: bool,
    pub refsel: ad7793_refsel,
    pub bias_voltage: ad7793_bias_voltage,
    pub exitation_current: ad7793_excitation_current,
    pub current_source_direction: ad7793_current_source_direction,
}
