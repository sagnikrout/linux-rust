//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/m88ds3103.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Montage Technology M88DS3103/M88RS6000 demodulator driver
//
// Copyright (C) 2013 Antti Palosaari <crope@iki.fi>
//

//
// I2C address
// 0x68,
//
// enum m88ds3103_ts_mode - TS connection mode
// @M88DS3103_TS_SERIAL:	TS output pin D0, normal
// @M88DS3103_TS_SERIAL_D7:	TS output pin D7
// @M88DS3103_TS_PARALLEL:	TS Parallel mode
// @M88DS3103_TS_CI:		TS CI Mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum m88ds3103_ts_mode {
    M88DS3103_TS_SERIAL,
    M88DS3103_TS_SERIAL_D7,
    M88DS3103_TS_PARALLEL,
    M88DS3103_TS_CI
}

//
// enum m88ds3103_clock_out
// @M88DS3103_CLOCK_OUT_DISABLED:	Clock output is disabled
// @M88DS3103_CLOCK_OUT_ENABLED:	Clock output is enabled with crystal
// clock.
// @M88DS3103_CLOCK_OUT_ENABLED_DIV2:	Clock output is enabled with half
// crystal clock.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum m88ds3103_clock_out {
    M88DS3103_CLOCK_OUT_DISABLED,
    M88DS3103_CLOCK_OUT_ENABLED,
    M88DS3103_CLOCK_OUT_ENABLED_DIV2
}

//
// struct m88ds3103_platform_data - Platform data for the m88ds3103 driver
// @clk: Clock frequency.
// @i2c_wr_max: Max bytes I2C adapter can write at once.
// @ts_mode: TS mode.
// @ts_clk: TS clock (KHz).
// @ts_clk_pol: TS clk polarity. 1-active at falling edge; 0-active at rising
// edge.
// @spec_inv: Input spectrum inversion.
// @agc: AGC configuration.
// @agc_inv: AGC polarity.
// @clk_out: Clock output.
// @envelope_mode: DiSEqC envelope mode.
// @lnb_hv_pol: LNB H/V pin polarity. 0: pin high set to VOLTAGE_18, pin low to
// set VOLTAGE_13. 1: pin high set to VOLTAGE_13, pin low to set VOLTAGE_18.
// @lnb_en_pol: LNB enable pin polarity. 0: pin high to disable, pin low to
// enable. 1: pin high to enable, pin low to disable.
// @get_dvb_frontend: Get DVB frontend.
// @get_i2c_adapter: Get I2C adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m88ds3103_platform_data {
    pub clk: u32,
    pub i2c_wr_max: u16,
    pub ts_mode: m88ds3103_ts_mode,
    pub ts_clk: u32,
    pub clk_out: m88ds3103_clock_out,
    pub ts_clk_pol:1: u8,
    pub spec_inv:1: u8,
    pub agc: u8,
    pub agc_inv:1: u8,
    pub envelope_mode:1: u8,
    pub lnb_hv_pol:1: u8,
    pub lnb_en_pol:1: u8,
    pub ): *mut *mut *mut dvb_frontend (get_dvb_frontend)(i2c_client,
    pub ): *mut *mut *mut i2c_adapter (get_i2c_adapter)(i2c_client,
// private: For legacy media attach wrapper. Do not set value.
    pub attach_in_use:1: u8,
}

//
// struct m88ds3103_config - m88ds3102 configuration
//
// @i2c_addr:	I2C address. Default: none, must set. Example: 0x68, ...
// @clock:	Device's clock. Default: none, must set. Example: 27000000
// @i2c_wr_max: Max bytes I2C provider is asked to write at once.
// Default: none, must set. Example: 33, 65, ...
// @ts_mode:	TS output mode, as defined by &enum m88ds3103_ts_mode.
// Default: M88DS3103_TS_SERIAL.
// @ts_clk:	TS clk in KHz. Default: 0.
// @ts_clk_pol:	TS clk polarity.Default: 0.
// 1-active at falling edge; 0-active at rising edge.
// @spec_inv:	Spectrum inversion. Default: 0.
// @agc_inv:	AGC polarity. Default: 0.
// @clock_out:	Clock output, as defined by &enum m88ds3103_clock_out.
// Default: M88DS3103_CLOCK_OUT_DISABLED.
// @envelope_mode: DiSEqC envelope mode. Default: 0.
// @agc:	AGC configuration. Default: none, must set.
// @lnb_hv_pol:	LNB H/V pin polarity. Default: 0. Values:
// 1: pin high set to VOLTAGE_13, pin low to set VOLTAGE_18;
// 0: pin high set to VOLTAGE_18, pin low to set VOLTAGE_13.
// @lnb_en_pol:	LNB enable pin polarity. Default: 0. Values:
// 1: pin high to enable, pin low to disable;
// 0: pin high to disable, pin low to enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m88ds3103_config {
    pub i2c_addr: u8,
    pub clock: u32,
    pub i2c_wr_max: u16,
    pub ts_mode: u8,
    pub ts_clk: u32,
    pub ts_clk_pol:1: u8,
    pub spec_inv:1: u8,
    pub agc_inv:1: u8,
    pub clock_out: u8,
    pub envelope_mode:1: u8,
    pub agc: u8,
    pub lnb_hv_pol:1: u8,
    pub lnb_en_pol:1: u8,
}

//
// m88ds3103_attach - Attach a m88ds3103 demod
//
// @config: pointer to &struct m88ds3103_config with demod configuration.
// @i2c: i2c adapter to use.
// @tuner_i2c: on success, returns the I2C adapter associated with
// m88ds3103 tuner.
//
// return: FE pointer on success, NULL on failure.
// Note: Do not add new m88ds3103_attach() users! Use I2C bindings instead.
//
extern "C" {
    pub fn m88ds3103_get_agc_pwm(fe: *mut dvb_frontend, _agc_pwm: *mut u8) -> c_int;
}

