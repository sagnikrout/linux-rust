//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lp8788.h
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
// TI LP8788 MFD Device
//
// Copyright 2012 Texas Instruments
//
// Author: Milo(Woogyom) Kim <milo.kim@ti.com>
//

pub const LP8788_NUM_BUCKS: c_int = 4;
pub const LP8788_NUM_DLDOS: c_int = 12;
pub const LP8788_NUM_ALDOS: c_int = 10;
pub const LP8788_NUM_BUCK2_DVS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_int_id {
// interrup register 1 : Addr 00h
    LP8788_INT_TSDL,
    LP8788_INT_TSDH,
    LP8788_INT_UVLO,
    LP8788_INT_FLAGMON,
    LP8788_INT_PWRON_TIME,
    LP8788_INT_PWRON,
    LP8788_INT_COMP1,
    LP8788_INT_COMP2,

// interrupt register 2 : Addr 01h
    LP8788_INT_CHG_INPUT_STATE,
    LP8788_INT_CHG_STATE,
    LP8788_INT_EOC,
    LP8788_INT_CHG_RESTART,
    LP8788_INT_RESTART_TIMEOUT,
    LP8788_INT_FULLCHG_TIMEOUT,
    LP8788_INT_PRECHG_TIMEOUT,

// interrupt register 3 : Addr 02h
    LP8788_INT_RTC_ALARM1 = 17,
    LP8788_INT_RTC_ALARM2,
    LP8788_INT_ENTER_SYS_SUPPORT,
    LP8788_INT_EXIT_SYS_SUPPORT,
    LP8788_INT_BATT_LOW,
    LP8788_INT_NO_BATT,

    LP8788_INT_MAX = 24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_dvs_sel {
    DVS_SEL_V0,
    DVS_SEL_V1,
    DVS_SEL_V2,
    DVS_SEL_V3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_ext_ldo_en_id {
    EN_ALDO1,
    EN_ALDO234,
    EN_ALDO5,
    EN_ALDO7,
    EN_DLDO7,
    EN_DLDO911,
    EN_LDOS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_charger_event {
    NO_CHARGER,
    CHARGER_DETECTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_bl_dim_mode {
    LP8788_DIM_EXPONENTIAL,
    LP8788_DIM_LINEAR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_bl_full_scale_current {
    LP8788_FULLSCALE_5000uA,
    LP8788_FULLSCALE_8500uA,
    LP8788_FULLSCALE_1200uA,
    LP8788_FULLSCALE_1550uA,
    LP8788_FULLSCALE_1900uA,
    LP8788_FULLSCALE_2250uA,
    LP8788_FULLSCALE_2600uA,
    LP8788_FULLSCALE_2950uA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_bl_ramp_step {
    LP8788_RAMP_8us,
    LP8788_RAMP_1024us,
    LP8788_RAMP_2048us,
    LP8788_RAMP_4096us,
    LP8788_RAMP_8192us,
    LP8788_RAMP_16384us,
    LP8788_RAMP_32768us,
    LP8788_RAMP_65538us,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_isink_scale {
    LP8788_ISINK_SCALE_100mA,
    LP8788_ISINK_SCALE_120mA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_isink_number {
    LP8788_ISINK_1,
    LP8788_ISINK_2,
    LP8788_ISINK_3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_alarm_sel {
    LP8788_ALARM_1,
    LP8788_ALARM_2,
    LP8788_ALARM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp8788_adc_id {
    LPADC_VBATT_5P5,
    LPADC_VIN_CHG,
    LPADC_IBATT,
    LPADC_IC_TEMP,
    LPADC_VBATT_6P0,
    LPADC_VBATT_5P0,
    LPADC_ADC1,
    LPADC_ADC2,
    LPADC_VDD,
    LPADC_VCOIN,
    LPADC_VDD_LDO,
    LPADC_ADC3,
    LPADC_ADC4,
    LPADC_MAX,
}

//
// lp8788_buck1_dvs
// @vsel         : dvs selector for buck v1 register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_buck1_dvs {
    pub vsel: lp8788_dvs_sel,
}

//
// lp8788_buck2_dvs
// @vsel         : dvs selector for buck v2 register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_buck2_dvs {
    pub vsel: lp8788_dvs_sel,
}

//
// struct lp8788_chg_param
// @addr         : charging control register address (range : 0x11 ~ 0x1C)
// @val          : charging parameter value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_chg_param {
    pub addr: u8,
    pub val: u8,
}

//
// struct lp8788_charger_platform_data
// @adc_vbatt         : adc channel name for battery voltage
// @adc_batt_temp     : adc channel name for battery temperature
// @max_vbatt_mv      : used for calculating battery capacity
// @chg_params        : initial charging parameters
// @num_chg_params    : numbers of charging parameters
// @charger_event     : the charger event can be reported to the platform side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_charger_platform_data {
    pub adc_vbatt: *const c_char,
    pub adc_batt_temp: *const c_char,
    pub max_vbatt_mv: c_uint,
    pub chg_params: *mut lp8788_chg_param,
    pub num_chg_params: c_int,
    pub event): lp8788_charger_event,
}

//
// struct lp8788_led_platform_data
// @name         : led driver name. (default: "keyboard-backlight")
// @scale        : current scale
// @num          : current sink number
// @iout_code    : current output value (Addr 9Ah ~ 9Bh)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_led_platform_data {
    pub name: *mut c_char,
    pub scale: lp8788_isink_scale,
    pub num: lp8788_isink_number,
    pub iout_code: c_int,
}

//
// struct lp8788_vib_platform_data
// @name         : vibrator driver name
// @scale        : current scale
// @num          : current sink number
// @iout_code    : current output value (Addr 9Ah ~ 9Bh)
// @pwm_code     : PWM code value (Addr 9Ch ~ 9Eh)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_vib_platform_data {
    pub name: *mut c_char,
    pub scale: lp8788_isink_scale,
    pub num: lp8788_isink_number,
    pub iout_code: c_int,
    pub pwm_code: c_int,
}

//
// struct lp8788_platform_data
// @init_func    : used for initializing registers
// before mfd driver is registered
// @buck_data    : regulator initial data for buck
// @dldo_data    : regulator initial data for digital ldo
// @aldo_data    : regulator initial data for analog ldo
// @buck1_dvs    : configurations for buck1 dvs
// @buck2_dvs    : configurations for buck2 dvs
// @chg_pdata    : platform data for charger driver
// @alarm_sel    : rtc alarm selection (1 or 2)
// @led_pdata    : configurable data for led driver
// @vib_pdata    : configurable data for vibrator driver
// @adc_pdata    : iio map data for adc driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788_platform_data {
// general system information
    pub lp): *mut *mut int (init_func) (struct lp8788,
// regulators
    pub buck_data: [*mut regulator_init_data; LP8788_NUM_BUCKS],
    pub dldo_data: [*mut regulator_init_data; LP8788_NUM_DLDOS],
    pub aldo_data: [*mut regulator_init_data; LP8788_NUM_ALDOS],
    pub buck1_dvs: *mut lp8788_buck1_dvs,
    pub buck2_dvs: *mut lp8788_buck2_dvs,
// charger
    pub chg_pdata: *mut lp8788_charger_platform_data,
// rtc alarm
    pub alarm_sel: lp8788_alarm_sel,
// current sinks
    pub led_pdata: *mut lp8788_led_platform_data,
    pub vib_pdata: *mut lp8788_vib_platform_data,
// adc iio map data
    pub adc_pdata: *mut iio_map,
}

//
// struct lp8788
// @dev          : parent device pointer
// @regmap       : used for i2c communcation on accessing registers
// @irqdm        : interrupt domain for handling nested interrupt
// @irq          : pin number of IRQ_N
// @pdata        : lp8788 platform specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp8788 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irqdm: *mut irq_domain,
    pub irq: c_int,
    pub pdata: *mut lp8788_platform_data,
}

extern "C" {
    pub fn lp8788_irq_init(lp: *mut lp8788, chip_irq: c_int) -> c_int;
}
extern "C" {
    pub fn lp8788_irq_exit(lp: *mut lp8788);
}
extern "C" {
    pub fn lp8788_read_byte(lp: *mut lp8788, reg: u8, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn lp8788_read_multi_bytes(lp: *mut lp8788, reg: u8, data: *mut u8, count: usize) -> c_int;
}
extern "C" {
    pub fn lp8788_write_byte(lp: *mut lp8788, reg: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn lp8788_update_bits(lp: *mut lp8788, reg: u8, mask: u8, data: u8) -> c_int;
}
