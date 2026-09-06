//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_icm42607/inv_icm42607.h
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
// Copyright (C) 2026 InvenSense, Inc.
//

//
// Serial bus slew rates. Rates are expressed as range between the two
// values with the midpoint as the typical rate. For the final value of
// 2ns, 2ns is considered the max value with no expressed minimum or
// typical value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_slew_rate {
    INV_ICM42607_SLEW_RATE_20_60NS = 0,
    INV_ICM42607_SLEW_RATE_12_36NS = 1,
    INV_ICM42607_SLEW_RATE_6_19NS = 2,
    INV_ICM42607_SLEW_RATE_4_14NS = 3,
    INV_ICM42607_SLEW_RATE_2_6NS = 4,
    INV_ICM42607_SLEW_RATE_2NS = 5,
    INV_ICM42607_SLEW_RATE_NB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_sensor_mode {
    INV_ICM42607_SENSOR_MODE_OFF = 0,
    INV_ICM42607_SENSOR_MODE_STANDBY = 1,
    INV_ICM42607_SENSOR_MODE_LOW_POWER = 2,
    INV_ICM42607_SENSOR_MODE_LOW_NOISE = 3,
    INV_ICM42607_SENSOR_MODE_NB
}

// gyroscope fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_gyro_fs {
    INV_ICM42607_GYRO_FS_2000DPS = 0,
    INV_ICM42607_GYRO_FS_1000DPS = 1,
    INV_ICM42607_GYRO_FS_500DPS = 2,
    INV_ICM42607_GYRO_FS_250DPS = 3,
    INV_ICM42607_GYRO_FS_NB
}

// accelerometer fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_accel_fs {
    INV_ICM42607_ACCEL_FS_16G = 0,
    INV_ICM42607_ACCEL_FS_8G = 1,
    INV_ICM42607_ACCEL_FS_4G = 2,
    INV_ICM42607_ACCEL_FS_2G = 3,
    INV_ICM42607_ACCEL_FS_NB
}

// ODR values  - Note Gyro does not support ODR less than 12.5Hz
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_odr {
    INV_ICM42607_ODR_1600HZ = 5,
    INV_ICM42607_ODR_800HZ = 6,
    INV_ICM42607_ODR_400HZ = 7,
    INV_ICM42607_ODR_200HZ = 8,
    INV_ICM42607_ODR_100HZ = 9,
    INV_ICM42607_ODR_50HZ = 10,
    INV_ICM42607_ODR_25HZ = 11,
    INV_ICM42607_ODR_12_5HZ = 12,
    INV_ICM42607_ODR_6_25HZ_LP = 13,
    INV_ICM42607_ODR_3_125HZ_LP = 14,
    INV_ICM42607_ODR_1_5625HZ_LP = 15,
    INV_ICM42607_ODR_NB
}

// Low-Noise mode sensor data filter (bandwidth)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_filter_bw {
    INV_ICM42607_FILTER_BYPASS = 0,
    INV_ICM42607_FILTER_BW_180HZ = 1,
    INV_ICM42607_FILTER_BW_121HZ = 2,
    INV_ICM42607_FILTER_BW_73HZ = 3,
    INV_ICM42607_FILTER_BW_53HZ = 4,
    INV_ICM42607_FILTER_BW_34HZ = 5,
    INV_ICM42607_FILTER_BW_25HZ = 6,
    INV_ICM42607_FILTER_BW_16HZ = 7,
    INV_ICM42607_FILTER_BW_NB
}

// Low-Power mode sensor data filter (averaging)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_filter_avg {
    INV_ICM42607_FILTER_AVG_2X = 0,
    INV_ICM42607_FILTER_AVG_4X = 1,
    INV_ICM42607_FILTER_AVG_8X = 2,
    INV_ICM42607_FILTER_AVG_16X = 3,
    INV_ICM42607_FILTER_AVG_32X = 4,
    INV_ICM42607_FILTER_AVG_64X = 5,
// values 6 and 7 also correspond to 64x.
}

// Temperature sensor data filter (bandwidth)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42607_temp_filter_bw {
    INV_ICM42607_TEMP_FILTER_BYPASS = 0,
    INV_ICM42607_TEMP_FILTER_BW_180HZ = 1,
    INV_ICM42607_TEMP_FILTER_BW_72HZ = 2,
    INV_ICM42607_TEMP_FILTER_BW_34HZ = 3,
    INV_ICM42607_TEMP_FILTER_BW_16HZ = 4,
    INV_ICM42607_TEMP_FILTER_BW_8HZ = 5,
    INV_ICM42607_TEMP_FILTER_BW_4HZ = 6,
// value 7 also corresponds to 4Hz
}

// Signed so that negative values can signify an invalid condition.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42607_sensor_conf {
    pub mode: c_int,
    pub fs: c_int,
    pub odr: c_int,
    pub filter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42607_conf {
    pub gyro: inv_icm42607_sensor_conf,
    pub accel: inv_icm42607_sensor_conf,
    pub /: *mut *mut ktime_t gyro_stop; / earliest time to stop the gyro,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42607_hw {
    pub name: *const c_char,
    pub conf: *const inv_icm42607_conf,
    pub whoami: u8,
}

//
// struct inv_icm42607_state - driver state variables
// @hw:		Hardware specific data.
// @lock:		lock for serializing multiple registers access.
// @map:		regmap pointer.
// @indio_accel:	accelerometer IIO device.
// @indio_gyro:	gyroscope IIO device.
// @vddio_supply:	I/O voltage regulator for the chip.
// @vddio_en:		I/O voltage status for runtime PM.
// @conf:		chip sensors configurations.
// @orientation:	sensor chip orientation relative to main hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42607_state {
    pub hw: *const inv_icm42607_hw,
    pub lock: mutex,
    pub map: *mut regmap,
    pub indio_accel: *mut iio_dev,
    pub indio_gyro: *mut iio_dev,
    pub vddio_supply: *mut regulator,
    pub vddio_en: bool,
    pub conf: inv_icm42607_conf,
    pub orientation: iio_mount_matrix,
}

//
// struct inv_icm42607_sensor_state - sensor state variables
// @power_mode:		sensor requested power mode (for common frequencies)
// @filter:		sensor filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42607_sensor_state {
    pub power_mode: inv_icm42607_sensor_mode,
    pub filter: c_int,
}

// Virtual register addresses: @bank on MSB (4 upper bits), @address on LSB
// Register Map for User Bank 0
pub const INV_ICM42607_REG_MCLK_RDY: c_uint = 0x00;
pub const INV_ICM42607_REG_DEVICE_CONFIG: c_uint = 0x01;

pub const INV_ICM42607_REG_SIGNAL_PATH_RESET: c_uint = 0x02;

pub const INV_ICM42607_REG_DRIVE_CONFIG1: c_uint = 0x03;

pub const INV_ICM42607_REG_DRIVE_CONFIG2: c_uint = 0x04;

pub const INV_ICM42607_REG_DRIVE_CONFIG3: c_uint = 0x05;

pub const INV_ICM42607_REG_INT_CONFIG: c_uint = 0x06;

pub const INV_ICM42607_INT_CONFIG_INT2_ACTIVE_LOW: c_uint = 0x00;

pub const INV_ICM42607_INT_CONFIG_INT1_ACTIVE_LOW: c_uint = 0x00;
// all sensor data are 16 bits (2 registers wide) in big-endian
pub const INV_ICM42607_REG_TEMP_DATA1: c_uint = 0x09;
pub const INV_ICM42607_REG_TEMP_DATA0: c_uint = 0x0A;
pub const INV_ICM42607_REG_ACCEL_DATA_X1: c_uint = 0x0B;
pub const INV_ICM42607_REG_ACCEL_DATA_X0: c_uint = 0x0C;
pub const INV_ICM42607_REG_ACCEL_DATA_Y1: c_uint = 0x0D;
pub const INV_ICM42607_REG_ACCEL_DATA_Y0: c_uint = 0x0E;
pub const INV_ICM42607_REG_ACCEL_DATA_Z1: c_uint = 0x0F;
pub const INV_ICM42607_REG_ACCEL_DATA_Z0: c_uint = 0x10;
pub const INV_ICM42607_REG_GYRO_DATA_X1: c_uint = 0x11;
pub const INV_ICM42607_REG_GYRO_DATA_X0: c_uint = 0x12;
pub const INV_ICM42607_REG_GYRO_DATA_Y1: c_uint = 0x13;
pub const INV_ICM42607_REG_GYRO_DATA_Y0: c_uint = 0x14;
pub const INV_ICM42607_REG_GYRO_DATA_Z1: c_uint = 0x15;
pub const INV_ICM42607_REG_GYRO_DATA_Z0: c_uint = 0x16;

pub const INV_ICM42607_REG_TMST_FSYNCH: c_uint = 0x17;
pub const INV_ICM42607_REG_TMST_FSYNCL: c_uint = 0x18;
// APEX Data Registers
pub const INV_ICM42607_REG_APEX_DATA0: c_uint = 0x31;
pub const INV_ICM42607_REG_APEX_DATA1: c_uint = 0x32;
pub const INV_ICM42607_REG_APEX_DATA2: c_uint = 0x33;
pub const INV_ICM42607_REG_APEX_DATA3: c_uint = 0x34;
pub const INV_ICM42607_REG_APEX_DATA4: c_uint = 0x1D;
pub const INV_ICM42607_REG_APEX_DATA5: c_uint = 0x1E;
pub const INV_ICM42607_REG_PWR_MGMT0: c_uint = 0x1F;

pub const INV_ICM42607_REG_GYRO_CONFIG0: c_uint = 0x20;

pub const INV_ICM42607_REG_ACCEL_CONFIG0: c_uint = 0x21;

pub const INV_ICM42607_REG_TEMP_CONFIG0: c_uint = 0x22;

pub const INV_ICM42607_REG_GYRO_CONFIG1: c_uint = 0x23;

pub const INV_ICM42607_REG_ACCEL_CONFIG1: c_uint = 0x24;

pub const INV_ICM42607_REG_APEX_CONFIG0: c_uint = 0x25;

pub const INV_ICM42607_REG_APEX_CONFIG1: c_uint = 0x26;

pub const INV_ICM42607_REG_WOM_CONFIG: c_uint = 0x27;

pub const INV_ICM42607_REG_FIFO_CONFIG1: c_uint = 0x28;

pub const INV_ICM42607_REG_FIFO_CONFIG2: c_uint = 0x29;
pub const INV_ICM42607_REG_FIFO_CONFIG3: c_uint = 0x2A;

// FIFO is 2048 bytes, let 12 samples for reading latency

pub const INV_ICM42607_FIFO_1SENSOR_PACKET_SIZE: c_int = 8;
pub const INV_ICM42607_FIFO_2SENSORS_PACKET_SIZE: c_int = 16;
pub const INV_ICM42607_REG_INT_SOURCE0: c_uint = 0x2B;

pub const INV_ICM42607_REG_INT_SOURCE1: c_uint = 0x2C;

pub const INV_ICM42607_REG_INT_SOURCE3: c_uint = 0x2D;

pub const INV_ICM42607_REG_INT_SOURCE4: c_uint = 0x2E;

pub const INV_ICM42607_REG_FIFO_LOST_PKT0: c_uint = 0x2F;
pub const INV_ICM42607_REG_FIFO_LOST_PKT1: c_uint = 0x30;
pub const INV_ICM42607_REG_INTF_CONFIG0: c_uint = 0x35;

pub const INV_ICM42607_INTF_CONFIG0_UI_SIFS_CFG_SPI_DIS: c_int = 2;
pub const INV_ICM42607_INTF_CONFIG0_UI_SIFS_CFG_I2C_DIS: c_int = 3;
pub const INV_ICM42607_REG_INTF_CONFIG1: c_uint = 0x36;

pub const INV_ICM42607_INTF_CONFIG1_CLKSEL_INT: c_int = 0;
pub const INV_ICM42607_INTF_CONFIG1_CLKSEL_PLL: c_int = 1;
pub const INV_ICM42607_INTF_CONFIG1_CLKSEL_OFF: c_int = 2;
pub const INV_ICM42607_REG_INT_STATUS_DRDY: c_uint = 0x39;

pub const INV_ICM42607_REG_INT_STATUS: c_uint = 0x3A;

pub const INV_ICM42607_REG_INT_STATUS2: c_uint = 0x3B;

pub const INV_ICM42607_REG_INT_STATUS3: c_uint = 0x3C;

//
// FIFO access registers
// FIFO count is 16 bits (2 registers) big-endian
// FIFO data is a continuous read register to read FIFO content
//
pub const INV_ICM42607_REG_FIFO_COUNTH: c_uint = 0x3D;
pub const INV_ICM42607_REG_FIFO_COUNTL: c_uint = 0x3E;
pub const INV_ICM42607_REG_FIFO_DATA: c_uint = 0x3F;
pub const INV_ICM42607_REG_WHOAMI: c_uint = 0x75;
pub const INV_ICM42607P_WHOAMI: c_uint = 0x60;
pub const INV_ICM42607_WHOAMI: c_uint = 0x67;
//
// Timings as listed in section 3 of datasheet, all values listed in datasheet
// in ms except temp startup time... setting all values in us and using
// USEC_PER_MSEC to convert from values displayed in datasheet.
//

pub const INV_ICM42607_TEMP_STARTUP_TIME_US: c_int = 77;
//
// Suspend delay assumed from other icm42600 series device, not
// documented in datasheet.
//

extern "C" {
    pub fn int(: *mut *mut inv_icm42607_bus_setup)(struct inv_icm42607_state) -> typedef;
}
