//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid-sensor-ids.h
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
// HID Sensors Driver
// Copyright (c) 2012, Intel Corporation.
//
pub const HID_MAX_PHY_DEVICES: c_uint = 0xFF;
pub const HID_USAGE_SENSOR_COLLECTION: c_uint = 0x200001;
// Accel 3D (200073)
pub const HID_USAGE_SENSOR_ACCEL_3D: c_uint = 0x200073;
pub const HID_USAGE_SENSOR_DATA_ACCELERATION: c_uint = 0x200452;
pub const HID_USAGE_SENSOR_ACCEL_X_AXIS: c_uint = 0x200453;
pub const HID_USAGE_SENSOR_ACCEL_Y_AXIS: c_uint = 0x200454;
pub const HID_USAGE_SENSOR_ACCEL_Z_AXIS: c_uint = 0x200455;
// ALS (200041)
pub const HID_USAGE_SENSOR_ALS: c_uint = 0x200041;
pub const HID_USAGE_SENSOR_DATA_LIGHT: c_uint = 0x2004d0;
pub const HID_USAGE_SENSOR_LIGHT_ILLUM: c_uint = 0x2004d1;
pub const HID_USAGE_SENSOR_LIGHT_COLOR_TEMPERATURE: c_uint = 0x2004d2;
pub const HID_USAGE_SENSOR_LIGHT_CHROMATICITY: c_uint = 0x2004d3;
pub const HID_USAGE_SENSOR_LIGHT_CHROMATICITY_X: c_uint = 0x2004d4;
pub const HID_USAGE_SENSOR_LIGHT_CHROMATICITY_Y: c_uint = 0x2004d5;
// PROX (200011)
pub const HID_USAGE_SENSOR_PROX: c_uint = 0x200011;
pub const HID_USAGE_SENSOR_DATA_PRESENCE: c_uint = 0x2004b0;
pub const HID_USAGE_SENSOR_HUMAN_PRESENCE: c_uint = 0x2004b1;
pub const HID_USAGE_SENSOR_HUMAN_PROXIMITY: c_uint = 0x2004b2;
pub const HID_USAGE_SENSOR_HUMAN_ATTENTION: c_uint = 0x2004bd;
// Pressure (200031)
pub const HID_USAGE_SENSOR_PRESSURE: c_uint = 0x200031;
pub const HID_USAGE_SENSOR_DATA_ATMOSPHERIC_PRESSURE: c_uint = 0x200430;
pub const HID_USAGE_SENSOR_ATMOSPHERIC_PRESSURE: c_uint = 0x200431;
// Tempreture (200033)
pub const HID_USAGE_SENSOR_TEMPERATURE: c_uint = 0x200033;
pub const HID_USAGE_SENSOR_DATA_ENVIRONMENTAL_TEMPERATURE: c_uint = 0x200434;
// humidity
pub const HID_USAGE_SENSOR_HUMIDITY: c_uint = 0x200032;
pub const HID_USAGE_SENSOR_ATMOSPHERIC_HUMIDITY: c_uint = 0x200433;
// Gyro 3D: (200076)
pub const HID_USAGE_SENSOR_GYRO_3D: c_uint = 0x200076;
pub const HID_USAGE_SENSOR_DATA_ANGL_VELOCITY: c_uint = 0x200456;
pub const HID_USAGE_SENSOR_ANGL_VELOCITY_X_AXIS: c_uint = 0x200457;
pub const HID_USAGE_SENSOR_ANGL_VELOCITY_Y_AXIS: c_uint = 0x200458;
pub const HID_USAGE_SENSOR_ANGL_VELOCITY_Z_AXIS: c_uint = 0x200459;
// Gravity vector
pub const HID_USAGE_SENSOR_GRAVITY_VECTOR: c_uint = 0x20007B;
// ORIENTATION: Compass 3D: (200083)
pub const HID_USAGE_SENSOR_COMPASS_3D: c_uint = 0x200083;
pub const HID_USAGE_SENSOR_DATA_ORIENTATION: c_uint = 0x200470;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_HEADING: c_uint = 0x200471;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_HEADING_X: c_uint = 0x200472;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_HEADING_Y: c_uint = 0x200473;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_HEADING_Z: c_uint = 0x200474;
pub const HID_USAGE_SENSOR_ORIENT_COMP_MAGN_NORTH: c_uint = 0x200475;
pub const HID_USAGE_SENSOR_ORIENT_COMP_TRUE_NORTH: c_uint = 0x200476;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_NORTH: c_uint = 0x200477;
pub const HID_USAGE_SENSOR_ORIENT_TRUE_NORTH: c_uint = 0x200478;
pub const HID_USAGE_SENSOR_ORIENT_DISTANCE: c_uint = 0x200479;
pub const HID_USAGE_SENSOR_ORIENT_DISTANCE_X: c_uint = 0x20047A;
pub const HID_USAGE_SENSOR_ORIENT_DISTANCE_Y: c_uint = 0x20047B;
pub const HID_USAGE_SENSOR_ORIENT_DISTANCE_Z: c_uint = 0x20047C;
pub const HID_USAGE_SENSOR_ORIENT_DISTANCE_OUT_OF_RANGE: c_uint = 0x20047D;
// ORIENTATION: Inclinometer 3D: (200086)
pub const HID_USAGE_SENSOR_INCLINOMETER_3D: c_uint = 0x200086;
pub const HID_USAGE_SENSOR_ORIENT_TILT: c_uint = 0x20047E;
pub const HID_USAGE_SENSOR_ORIENT_TILT_X: c_uint = 0x20047F;
pub const HID_USAGE_SENSOR_ORIENT_TILT_Y: c_uint = 0x200480;
pub const HID_USAGE_SENSOR_ORIENT_TILT_Z: c_uint = 0x200481;
pub const HID_USAGE_SENSOR_DEVICE_ORIENTATION: c_uint = 0x20008A;
pub const HID_USAGE_SENSOR_RELATIVE_ORIENTATION: c_uint = 0x20008E;
pub const HID_USAGE_SENSOR_GEOMAGNETIC_ORIENTATION: c_uint = 0x2000C1;
pub const HID_USAGE_SENSOR_ORIENT_ROTATION_MATRIX: c_uint = 0x200482;
pub const HID_USAGE_SENSOR_ORIENT_QUATERNION: c_uint = 0x200483;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_FLUX: c_uint = 0x200484;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_FLUX_X_AXIS: c_uint = 0x200485;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_FLUX_Y_AXIS: c_uint = 0x200486;
pub const HID_USAGE_SENSOR_ORIENT_MAGN_FLUX_Z_AXIS: c_uint = 0x200487;
// Time (2000a0)
pub const HID_USAGE_SENSOR_TIME: c_uint = 0x2000a0;
pub const HID_USAGE_SENSOR_TIME_YEAR: c_uint = 0x200521;
pub const HID_USAGE_SENSOR_TIME_MONTH: c_uint = 0x200522;
pub const HID_USAGE_SENSOR_TIME_DAY: c_uint = 0x200523;
pub const HID_USAGE_SENSOR_TIME_HOUR: c_uint = 0x200525;
pub const HID_USAGE_SENSOR_TIME_MINUTE: c_uint = 0x200526;
pub const HID_USAGE_SENSOR_TIME_SECOND: c_uint = 0x200527;
pub const HID_USAGE_SENSOR_TIME_TIMESTAMP: c_uint = 0x200529;
// Units
pub const HID_USAGE_SENSOR_UNITS_NOT_SPECIFIED: c_uint = 0x00;
pub const HID_USAGE_SENSOR_UNITS_LUX: c_uint = 0x01;
pub const HID_USAGE_SENSOR_UNITS_KELVIN: c_uint = 0x01000100;
pub const HID_USAGE_SENSOR_UNITS_FAHRENHEIT: c_uint = 0x03000100;
pub const HID_USAGE_SENSOR_UNITS_PASCAL: c_uint = 0xF1E1;
pub const HID_USAGE_SENSOR_UNITS_NEWTON: c_uint = 0x11E1;
pub const HID_USAGE_SENSOR_UNITS_METERS_PER_SECOND: c_uint = 0x11F0;
pub const HID_USAGE_SENSOR_UNITS_METERS_PER_SEC_SQRD: c_uint = 0x11E0;
pub const HID_USAGE_SENSOR_UNITS_FARAD: c_uint = 0xE14F2000;
pub const HID_USAGE_SENSOR_UNITS_AMPERE: c_uint = 0x01001000;
pub const HID_USAGE_SENSOR_UNITS_WATT: c_uint = 0x21d1;
pub const HID_USAGE_SENSOR_UNITS_HENRY: c_uint = 0x21E1E000;
pub const HID_USAGE_SENSOR_UNITS_OHM: c_uint = 0x21D1E000;
pub const HID_USAGE_SENSOR_UNITS_VOLT: c_uint = 0x21D1F000;
pub const HID_USAGE_SENSOR_UNITS_HERTZ: c_uint = 0x01F0;
pub const HID_USAGE_SENSOR_UNITS_DEGREES_PER_SEC_SQRD: c_uint = 0x14E0;
pub const HID_USAGE_SENSOR_UNITS_RADIANS: c_uint = 0x12;
pub const HID_USAGE_SENSOR_UNITS_RADIANS_PER_SECOND: c_uint = 0x12F0;
pub const HID_USAGE_SENSOR_UNITS_RADIANS_PER_SEC_SQRD: c_uint = 0x12E0;
pub const HID_USAGE_SENSOR_UNITS_SECOND: c_uint = 0x0110;
pub const HID_USAGE_SENSOR_UNITS_GAUSS: c_uint = 0x01E1F000;
pub const HID_USAGE_SENSOR_UNITS_GRAM: c_uint = 0x0101;
pub const HID_USAGE_SENSOR_UNITS_CENTIMETER: c_uint = 0x11;
pub const HID_USAGE_SENSOR_UNITS_G: c_uint = 0x1A;
pub const HID_USAGE_SENSOR_UNITS_MILLISECOND: c_uint = 0x19;
pub const HID_USAGE_SENSOR_UNITS_PERCENT: c_uint = 0x17;
pub const HID_USAGE_SENSOR_UNITS_DEGREES: c_uint = 0x14;
pub const HID_USAGE_SENSOR_UNITS_DEGREES_PER_SECOND: c_uint = 0x15;
// Common selectors
pub const HID_USAGE_SENSOR_PROP_DESC: c_uint = 0x200300;
pub const HID_USAGE_SENSOR_PROP_FRIENDLY_NAME: c_uint = 0x200301;
pub const HID_USAGE_SENSOR_PROP_SERIAL_NUM: c_uint = 0x200307;
pub const HID_USAGE_SENSOR_PROP_MANUFACTURER: c_uint = 0x200305;
pub const HID_USAGE_SENSOR_PROP_MODEL: c_uint = 0x200306;
pub const HID_USAGE_SENSOR_PROP_REPORT_INTERVAL: c_uint = 0x20030E;
pub const HID_USAGE_SENSOR_PROP_SENSITIVITY_ABS: c_uint = 0x20030F;
pub const HID_USAGE_SENSOR_PROP_SENSITIVITY_RANGE_PCT: c_uint = 0x200310;
pub const HID_USAGE_SENSOR_PROP_SENSITIVITY_REL_PCT: c_uint = 0x200311;
pub const HID_USAGE_SENSOR_PROP_ACCURACY: c_uint = 0x200312;
pub const HID_USAGE_SENSOR_PROP_RESOLUTION: c_uint = 0x200313;
pub const HID_USAGE_SENSOR_PROP_RANGE_MAXIMUM: c_uint = 0x200314;
pub const HID_USAGE_SENSOR_PROP_RANGE_MINIMUM: c_uint = 0x200315;
pub const HID_USAGE_SENSOR_PROP_REPORT_STATE: c_uint = 0x200316;
pub const HID_USAGE_SENSOR_PROY_POWER_STATE: c_uint = 0x200319;
// Batch mode selectors
pub const HID_USAGE_SENSOR_PROP_REPORT_LATENCY: c_uint = 0x20031B;
// Per data field properties
pub const HID_USAGE_SENSOR_DATA_MOD_NONE: c_uint = 0x00;
pub const HID_USAGE_SENSOR_DATA_MOD_CHANGE_SENSITIVITY_ABS: c_uint = 0x1000;
pub const HID_USAGE_SENSOR_DATA_MOD_CHANGE_SENSITIVITY_REL_PCT: c_uint = 0xE000;
// Power state enumerations
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_UNDEFINED_ENUM: c_uint = 0x200850;
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_D0_FULL_POWER_ENUM: c_uint = 0x200851;
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_D1_LOW_POWER_ENUM: c_uint = 0x200852;
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_D2_STANDBY_WITH_WAKE_ENUM: c_uint = 0x200853;
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_D3_SLEEP_WITH_WAKE_ENUM: c_uint = 0x200854;
pub const HID_USAGE_SENSOR_PROP_POWER_STATE_D4_POWER_OFF_ENUM: c_uint = 0x200855;
// Report State enumerations
pub const HID_USAGE_SENSOR_PROP_REPORTING_STATE_NO_EVENTS_ENUM: c_uint = 0x200840;
pub const HID_USAGE_SENSOR_PROP_REPORTING_STATE_ALL_EVENTS_ENUM: c_uint = 0x200841;
// Custom Sensor (2000e1)
pub const HID_USAGE_SENSOR_HINGE: c_uint = 0x20020B;
pub const HID_USAGE_SENSOR_DATA_FIELD_LOCATION: c_uint = 0x200400;
pub const HID_USAGE_SENSOR_DATA_FIELE_TIME_SINCE_SYS_BOOT: c_uint = 0x20052B;
pub const HID_USAGE_SENSOR_DATA_FIELD_CUSTOM_USAGE: c_uint = 0x200541;
pub const HID_USAGE_SENSOR_DATA_FIELD_CUSTOM_VALUE_BASE: c_uint = 0x200543;
// Custom Sensor data 28=>x>=0

