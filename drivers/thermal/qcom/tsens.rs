//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/qcom/tsens.h
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//
pub const NO_PT_CALIB: c_uint = 0x0;
pub const ONE_PT_CALIB: c_uint = 0x1;
pub const ONE_PT_CALIB2: c_uint = 0x2;
pub const TWO_PT_CALIB: c_uint = 0x3;
pub const ONE_PT_CALIB2_NO_OFFSET: c_uint = 0x6;
pub const TWO_PT_CALIB_NO_OFFSET: c_uint = 0x7;
pub const CAL_DEGC_PT1: c_int = 30;
pub const CAL_DEGC_PT2: c_int = 120;
pub const SLOPE_FACTOR: c_int = 1000;
pub const SLOPE_DEFAULT: c_int = 3200;
pub const TIMEOUT_US: c_int = 100;
pub const THRESHOLD_MAX_ADC_CODE: c_uint = 0x3ff;
pub const THRESHOLD_MIN_ADC_CODE: c_uint = 0x0;
pub const MAX_SENSORS: c_int = 16;
pub const MAX_READ_RETRY: c_int = 3;

// IP version numbers in ascending order
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsens_ver {
    VER_0 = 0,
    VER_0_1,
    VER_1_X,
    VER_1_X_NO_RPM,
    VER_2_X,
    VER_2_X_NO_RPM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsens_irq_type {
    LOWER,
    UPPER,
    CRITICAL,
}

//
// struct tsens_sensor - data for each sensor connected to the tsens device
// @priv: tsens device instance that this sensor is connected to
// @tzd: pointer to the thermal zone that this sensor is in
// @offset: offset of temperature adjustment curve
// @hw_id: HW ID can be used in case of platform-specific IDs
// @slope: slope of temperature adjustment curve
// @status: 8960-specific variable to track 8960 and 8660 status register offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_sensor {
    pub priv: *mut tsens_priv,
    pub tzd: *mut thermal_zone_device,
    pub offset: c_int,
    pub hw_id: c_uint,
    pub slope: c_int,
    pub status: u32,
    pub p1_calib_offset: c_int,
    pub p2_calib_offset: c_int,
}

//
// struct tsens_ops - operations as supported by the tsens device
// @init: Function to initialize the tsens device
// @calibrate: Function to calibrate the tsens device
// @get_temp: Function which returns the temp in millidegC
// @enable: Function to enable (clocks/power) tsens device
// @disable: Function to disable the tsens device
// @suspend: Function to suspend the tsens device
// @resume: Function to resume the tsens device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_ops {
// mandatory callbacks
    pub priv): *mut *mut int (init)(struct tsens_priv,
    pub priv): *mut *mut int (calibrate)(struct tsens_priv,
    pub temp): *const *const *const int (get_temp)(struct tsens_sensor s, int,
// optional callbacks
    pub i): *mut *mut *mut int (enable)(struct tsens_priv priv, int,
    pub priv): *mut *mut void (disable)(struct tsens_priv,
    pub priv): *mut *mut int (suspend)(struct tsens_priv,
    pub priv): *mut *mut int (resume)(struct tsens_priv,
}

//
// reg_field IDs to use as an index into an array
// If you change the order of the entries, check the devm_regmap_field_alloc()
// calls in init_common()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regfield_ids {
// ----- SROT ------
// HW_VER
    VER_MAJOR,
    VER_MINOR,
    VER_STEP,
// CTRL_OFFSET
    TSENS_EN,
    TSENS_SW_RST,
    SENSOR_EN,
    CODE_OR_TEMP,
    MAIN_MEASURE_PERIOD,

// ----- TM ------
// TRDY
    TRDY,
// INTERRUPT ENABLE
    INT_EN,	/* v2+ has separate enables for crit, upper and lower irq */
// STATUS
    LAST_TEMP_0,	/* Last temperature reading */
    LAST_TEMP_1,
    LAST_TEMP_2,
    LAST_TEMP_3,
    LAST_TEMP_4,
    LAST_TEMP_5,
    LAST_TEMP_6,
    LAST_TEMP_7,
    LAST_TEMP_8,
    LAST_TEMP_9,
    LAST_TEMP_10,
    LAST_TEMP_11,
    LAST_TEMP_12,
    LAST_TEMP_13,
    LAST_TEMP_14,
    LAST_TEMP_15,
    VALID_0,		/* VALID reading or not */
    VALID_1,
    VALID_2,
    VALID_3,
    VALID_4,
    VALID_5,
    VALID_6,
    VALID_7,
    VALID_8,
    VALID_9,
    VALID_10,
    VALID_11,
    VALID_12,
    VALID_13,
    VALID_14,
    VALID_15,
    LOWER_STATUS_0,	/* LOWER threshold violated */
    LOWER_STATUS_1,
    LOWER_STATUS_2,
    LOWER_STATUS_3,
    LOWER_STATUS_4,
    LOWER_STATUS_5,
    LOWER_STATUS_6,
    LOWER_STATUS_7,
    LOWER_STATUS_8,
    LOWER_STATUS_9,
    LOWER_STATUS_10,
    LOWER_STATUS_11,
    LOWER_STATUS_12,
    LOWER_STATUS_13,
    LOWER_STATUS_14,
    LOWER_STATUS_15,
    LOW_INT_STATUS_0,	/* LOWER interrupt status */
    LOW_INT_STATUS_1,
    LOW_INT_STATUS_2,
    LOW_INT_STATUS_3,
    LOW_INT_STATUS_4,
    LOW_INT_STATUS_5,
    LOW_INT_STATUS_6,
    LOW_INT_STATUS_7,
    LOW_INT_STATUS_8,
    LOW_INT_STATUS_9,
    LOW_INT_STATUS_10,
    LOW_INT_STATUS_11,
    LOW_INT_STATUS_12,
    LOW_INT_STATUS_13,
    LOW_INT_STATUS_14,
    LOW_INT_STATUS_15,
    LOW_INT_CLEAR_0,	/* LOWER interrupt clear */
    LOW_INT_CLEAR_1,
    LOW_INT_CLEAR_2,
    LOW_INT_CLEAR_3,
    LOW_INT_CLEAR_4,
    LOW_INT_CLEAR_5,
    LOW_INT_CLEAR_6,
    LOW_INT_CLEAR_7,
    LOW_INT_CLEAR_8,
    LOW_INT_CLEAR_9,
    LOW_INT_CLEAR_10,
    LOW_INT_CLEAR_11,
    LOW_INT_CLEAR_12,
    LOW_INT_CLEAR_13,
    LOW_INT_CLEAR_14,
    LOW_INT_CLEAR_15,
    LOW_INT_MASK_0,	/* LOWER interrupt mask */
    LOW_INT_MASK_1,
    LOW_INT_MASK_2,
    LOW_INT_MASK_3,
    LOW_INT_MASK_4,
    LOW_INT_MASK_5,
    LOW_INT_MASK_6,
    LOW_INT_MASK_7,
    LOW_INT_MASK_8,
    LOW_INT_MASK_9,
    LOW_INT_MASK_10,
    LOW_INT_MASK_11,
    LOW_INT_MASK_12,
    LOW_INT_MASK_13,
    LOW_INT_MASK_14,
    LOW_INT_MASK_15,
    LOW_THRESH_0,		/* LOWER threshold values */
    LOW_THRESH_1,
    LOW_THRESH_2,
    LOW_THRESH_3,
    LOW_THRESH_4,
    LOW_THRESH_5,
    LOW_THRESH_6,
    LOW_THRESH_7,
    LOW_THRESH_8,
    LOW_THRESH_9,
    LOW_THRESH_10,
    LOW_THRESH_11,
    LOW_THRESH_12,
    LOW_THRESH_13,
    LOW_THRESH_14,
    LOW_THRESH_15,
    UPPER_STATUS_0,	/* UPPER threshold violated */
    UPPER_STATUS_1,
    UPPER_STATUS_2,
    UPPER_STATUS_3,
    UPPER_STATUS_4,
    UPPER_STATUS_5,
    UPPER_STATUS_6,
    UPPER_STATUS_7,
    UPPER_STATUS_8,
    UPPER_STATUS_9,
    UPPER_STATUS_10,
    UPPER_STATUS_11,
    UPPER_STATUS_12,
    UPPER_STATUS_13,
    UPPER_STATUS_14,
    UPPER_STATUS_15,
    UP_INT_STATUS_0,	/* UPPER interrupt status */
    UP_INT_STATUS_1,
    UP_INT_STATUS_2,
    UP_INT_STATUS_3,
    UP_INT_STATUS_4,
    UP_INT_STATUS_5,
    UP_INT_STATUS_6,
    UP_INT_STATUS_7,
    UP_INT_STATUS_8,
    UP_INT_STATUS_9,
    UP_INT_STATUS_10,
    UP_INT_STATUS_11,
    UP_INT_STATUS_12,
    UP_INT_STATUS_13,
    UP_INT_STATUS_14,
    UP_INT_STATUS_15,
    UP_INT_CLEAR_0,	/* UPPER interrupt clear */
    UP_INT_CLEAR_1,
    UP_INT_CLEAR_2,
    UP_INT_CLEAR_3,
    UP_INT_CLEAR_4,
    UP_INT_CLEAR_5,
    UP_INT_CLEAR_6,
    UP_INT_CLEAR_7,
    UP_INT_CLEAR_8,
    UP_INT_CLEAR_9,
    UP_INT_CLEAR_10,
    UP_INT_CLEAR_11,
    UP_INT_CLEAR_12,
    UP_INT_CLEAR_13,
    UP_INT_CLEAR_14,
    UP_INT_CLEAR_15,
    UP_INT_MASK_0,		/* UPPER interrupt mask */
    UP_INT_MASK_1,
    UP_INT_MASK_2,
    UP_INT_MASK_3,
    UP_INT_MASK_4,
    UP_INT_MASK_5,
    UP_INT_MASK_6,
    UP_INT_MASK_7,
    UP_INT_MASK_8,
    UP_INT_MASK_9,
    UP_INT_MASK_10,
    UP_INT_MASK_11,
    UP_INT_MASK_12,
    UP_INT_MASK_13,
    UP_INT_MASK_14,
    UP_INT_MASK_15,
    UP_THRESH_0,		/* UPPER threshold values */
    UP_THRESH_1,
    UP_THRESH_2,
    UP_THRESH_3,
    UP_THRESH_4,
    UP_THRESH_5,
    UP_THRESH_6,
    UP_THRESH_7,
    UP_THRESH_8,
    UP_THRESH_9,
    UP_THRESH_10,
    UP_THRESH_11,
    UP_THRESH_12,
    UP_THRESH_13,
    UP_THRESH_14,
    UP_THRESH_15,
    CRITICAL_STATUS_0,	/* CRITICAL threshold violated */
    CRITICAL_STATUS_1,
    CRITICAL_STATUS_2,
    CRITICAL_STATUS_3,
    CRITICAL_STATUS_4,
    CRITICAL_STATUS_5,
    CRITICAL_STATUS_6,
    CRITICAL_STATUS_7,
    CRITICAL_STATUS_8,
    CRITICAL_STATUS_9,
    CRITICAL_STATUS_10,
    CRITICAL_STATUS_11,
    CRITICAL_STATUS_12,
    CRITICAL_STATUS_13,
    CRITICAL_STATUS_14,
    CRITICAL_STATUS_15,
    CRIT_INT_STATUS_0,	/* CRITICAL interrupt status */
    CRIT_INT_STATUS_1,
    CRIT_INT_STATUS_2,
    CRIT_INT_STATUS_3,
    CRIT_INT_STATUS_4,
    CRIT_INT_STATUS_5,
    CRIT_INT_STATUS_6,
    CRIT_INT_STATUS_7,
    CRIT_INT_STATUS_8,
    CRIT_INT_STATUS_9,
    CRIT_INT_STATUS_10,
    CRIT_INT_STATUS_11,
    CRIT_INT_STATUS_12,
    CRIT_INT_STATUS_13,
    CRIT_INT_STATUS_14,
    CRIT_INT_STATUS_15,
    CRIT_INT_CLEAR_0,	/* CRITICAL interrupt clear */
    CRIT_INT_CLEAR_1,
    CRIT_INT_CLEAR_2,
    CRIT_INT_CLEAR_3,
    CRIT_INT_CLEAR_4,
    CRIT_INT_CLEAR_5,
    CRIT_INT_CLEAR_6,
    CRIT_INT_CLEAR_7,
    CRIT_INT_CLEAR_8,
    CRIT_INT_CLEAR_9,
    CRIT_INT_CLEAR_10,
    CRIT_INT_CLEAR_11,
    CRIT_INT_CLEAR_12,
    CRIT_INT_CLEAR_13,
    CRIT_INT_CLEAR_14,
    CRIT_INT_CLEAR_15,
    CRIT_INT_MASK_0,	/* CRITICAL interrupt mask */
    CRIT_INT_MASK_1,
    CRIT_INT_MASK_2,
    CRIT_INT_MASK_3,
    CRIT_INT_MASK_4,
    CRIT_INT_MASK_5,
    CRIT_INT_MASK_6,
    CRIT_INT_MASK_7,
    CRIT_INT_MASK_8,
    CRIT_INT_MASK_9,
    CRIT_INT_MASK_10,
    CRIT_INT_MASK_11,
    CRIT_INT_MASK_12,
    CRIT_INT_MASK_13,
    CRIT_INT_MASK_14,
    CRIT_INT_MASK_15,
    CRIT_THRESH_0,		/* CRITICAL threshold values */
    CRIT_THRESH_1,
    CRIT_THRESH_2,
    CRIT_THRESH_3,
    CRIT_THRESH_4,
    CRIT_THRESH_5,
    CRIT_THRESH_6,
    CRIT_THRESH_7,
    CRIT_THRESH_8,
    CRIT_THRESH_9,
    CRIT_THRESH_10,
    CRIT_THRESH_11,
    CRIT_THRESH_12,
    CRIT_THRESH_13,
    CRIT_THRESH_14,
    CRIT_THRESH_15,

// WATCHDOG
    WDOG_BARK_STATUS,
    WDOG_BARK_CLEAR,
    WDOG_BARK_MASK,
    WDOG_BARK_COUNT,

// CYCLE COMPLETION MONITOR
    CC_MON_STATUS,
    CC_MON_CLEAR,
    CC_MON_MASK,

    MIN_STATUS_0,		/* MIN threshold violated */
    MIN_STATUS_1,
    MIN_STATUS_2,
    MIN_STATUS_3,
    MIN_STATUS_4,
    MIN_STATUS_5,
    MIN_STATUS_6,
    MIN_STATUS_7,
    MIN_STATUS_8,
    MIN_STATUS_9,
    MIN_STATUS_10,
    MIN_STATUS_11,
    MIN_STATUS_12,
    MIN_STATUS_13,
    MIN_STATUS_14,
    MIN_STATUS_15,
    MAX_STATUS_0,		/* MAX threshold violated */
    MAX_STATUS_1,
    MAX_STATUS_2,
    MAX_STATUS_3,
    MAX_STATUS_4,
    MAX_STATUS_5,
    MAX_STATUS_6,
    MAX_STATUS_7,
    MAX_STATUS_8,
    MAX_STATUS_9,
    MAX_STATUS_10,
    MAX_STATUS_11,
    MAX_STATUS_12,
    MAX_STATUS_13,
    MAX_STATUS_14,
    MAX_STATUS_15,

// Keep last
    MAX_REGFIELDS
}

//
// struct tsens_features - Features supported by the IP
// @ver_major: Major number of IP version
// @crit_int: does the IP support critical interrupts?
// @combo_int: does the IP use one IRQ for up, low and critical thresholds?
// @adc:      do the sensors only output adc code (instead of temperature)?
// @srot_split: does the IP neatly splits the register space into SROT and TM,
// with SROT only being available to secure boot firmware?
// @has_watchdog: does this IP support watchdog functionality?
// @max_sensors: maximum sensors supported by this version of the IP
// @trip_min_temp: minimum trip temperature supported by this version of the IP
// @trip_max_temp: maximum trip temperature supported by this version of the IP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_features {
    pub ver_major: c_uint,
    pub crit_int:1: c_uint,
    pub combo_int:1: c_uint,
    pub adc:1: c_uint,
    pub srot_split:1: c_uint,
    pub has_watchdog:1: c_uint,
    pub max_sensors: c_uint,
    pub trip_min_temp: c_int,
    pub trip_max_temp: c_int,
}

//
// struct tsens_plat_data - tsens compile-time platform data
// @num_sensors: Number of sensors supported by platform
// @ops: operations the tsens instance supports
// @hw_ids: Subset of sensors ids supported by platform, if not the first n
// @feat: features of the IP
// @fields: bitfield locations
// @no_irq_wake: if set, TSENS interrupts will not be configured as wakeup sources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_plat_data {
    pub num_sensors: u32,
    pub ops: *const tsens_ops,
    pub hw_ids: *mut c_uint,
    pub feat: *mut tsens_features,
    pub fields: *const reg_field,
    pub no_irq_wake: bool,
}

//
// struct tsens_context - Registers to be saved/restored across a context loss
// @threshold: Threshold register value
// @control: Control register value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_context {
    pub threshold: c_int,
    pub control: c_int,
}

//
// struct tsens_priv - private data for each instance of the tsens IP
// @dev: pointer to struct device
// @num_sensors: number of sensors enabled on this device
// @tm_map: pointer to TM register address space
// @srot_map: pointer to SROT register address space
// @tm_offset: deal with old device trees that don't address TM and SROT
// address space separately
// @ul_lock: lock while processing upper/lower threshold interrupts
// @crit_lock: lock while processing critical threshold interrupts
// @rf: array of regmap_fields used to store value of the field
// @ctx: registers to be saved and restored during suspend/resume
// @feat: features of the IP
// @fields: bitfield locations
// @ops: pointer to list of callbacks supported by this device
// @debug_root: pointer to debugfs dentry for all tsens
// @debug: pointer to debugfs dentry for tsens controller
// @uplow_irq: IRQ number for uplow (upper/lower) threshold interrupts
// @crit_irq: IRQ number for critical threshold interrupts
// @combined_irq: IRQ number for combined threshold interrupts
// @sensor: list of sensors attached to this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_priv {
    pub dev: *mut device,
    pub num_sensors: u32,
    pub tm_map: *mut regmap,
    pub srot_map: *mut regmap,
    pub tm_offset: u32,
// lock for upper/lower threshold interrupts
    pub ul_lock: spinlock_t,
    pub rf: [*mut regmap_field; MAX_REGFIELDS],
    pub ctx: tsens_context,
    pub feat: *mut tsens_features,
    pub fields: *const reg_field,
    pub ops: *const tsens_ops,
    pub debug_root: *mut dentry,
    pub debug: *mut dentry,
    pub uplow_irq: c_int,
    pub crit_irq: c_int,
    pub combined_irq: c_int,
    pub __counted_by(num_sensors): tsens_sensor sensor[],
}

//
// struct tsens_single_value - internal representation of a single field inside nvmem calibration data
// @idx: index into the u32 data array
// @shift: the shift of the first bit in the value
// @blob: index of the data blob to use for this cell
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_single_value {
    pub idx: u8,
    pub shift: u8,
    pub blob: u8,
}

//
// struct tsens_legacy_calibration_format - description of calibration data used when parsing the legacy nvmem blob
// @base_len: the length of the base fields inside calibration data
// @base_shift: the shift to be applied to base data
// @sp_len: the length of the sN_pM fields inside calibration data
// @mode: descriptor of the calibration mode field
// @invalid: descriptor of the calibration mode invalid field
// @base: descriptors of the base0 and base1 fields
// @sp: descriptors of the sN_pM fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsens_legacy_calibration_format {
    pub base_len: c_uint,
    pub base_shift: c_uint,
    pub sp_len: c_uint,
// just two bits
    pub mode: tsens_single_value,
// on all platforms except 8974 invalid is the third bit of what downstream calls 'mode'
    pub invalid: tsens_single_value,
    pub base: [tsens_single_value; 2],
    pub sp: [tsens_single_value; ][2],
}

extern "C" {
    pub fn tsens_read_calibration(priv: *mut tsens_priv, shift: c_int, p1: *mut u32, p2: *mut u32, backup: bool) -> c_int;
}
extern "C" {
    pub fn tsens_calibrate_nvmem(priv: *mut tsens_priv, shift: c_int) -> c_int;
}
extern "C" {
    pub fn tsens_calibrate_common(priv: *mut tsens_priv) -> c_int;
}
extern "C" {
    pub fn compute_intercept_slope(priv: *mut tsens_priv, pt1: *mut u32, pt2: *mut u32, mode: u32);
}
extern "C" {
    pub fn init_common(priv: *mut tsens_priv) -> c_int;
}
extern "C" {
    pub fn get_temp_tsens_valid(s: *const tsens_sensor, temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn get_temp_common(s: *const tsens_sensor, temp: *mut c_int) -> c_int;
}

extern "C" {
    pub fn tsens_resume_common(priv: *mut tsens_priv) -> c_int;
}
extern "C" {
    pub fn tsens_suspend_common(priv: *mut tsens_priv) -> c_int;
}

// TSENS target
// TSENS v0.1 targets
// TSENS v1 targets
// TSENS v1 with no RPM targets
// TSENS v2 targets
// TSENS automotive targets
