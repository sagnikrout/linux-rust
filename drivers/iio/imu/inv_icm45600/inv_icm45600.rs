//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_icm45600/inv_icm45600.h
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
// Copyright (C) 2025 Invensense, Inc.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45600_sensor_mode {
    INV_ICM45600_SENSOR_MODE_OFF,
    INV_ICM45600_SENSOR_MODE_STANDBY,
    INV_ICM45600_SENSOR_MODE_LOW_POWER,
    INV_ICM45600_SENSOR_MODE_LOW_NOISE,
    INV_ICM45600_SENSOR_MODE_MAX
}

// gyroscope fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45600_gyro_fs {
    INV_ICM45600_GYRO_FS_2000DPS,
    INV_ICM45600_GYRO_FS_1000DPS,
    INV_ICM45600_GYRO_FS_500DPS,
    INV_ICM45600_GYRO_FS_250DPS,
    INV_ICM45600_GYRO_FS_125DPS,
    INV_ICM45600_GYRO_FS_62_5DPS,
    INV_ICM45600_GYRO_FS_31_25DPS,
    INV_ICM45600_GYRO_FS_15_625DPS,
    INV_ICM45600_GYRO_FS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45686_gyro_fs {
    INV_ICM45686_GYRO_FS_4000DPS,
    INV_ICM45686_GYRO_FS_2000DPS,
    INV_ICM45686_GYRO_FS_1000DPS,
    INV_ICM45686_GYRO_FS_500DPS,
    INV_ICM45686_GYRO_FS_250DPS,
    INV_ICM45686_GYRO_FS_125DPS,
    INV_ICM45686_GYRO_FS_62_5DPS,
    INV_ICM45686_GYRO_FS_31_25DPS,
    INV_ICM45686_GYRO_FS_15_625DPS,
    INV_ICM45686_GYRO_FS_MAX
}

// accelerometer fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45600_accel_fs {
    INV_ICM45600_ACCEL_FS_16G,
    INV_ICM45600_ACCEL_FS_8G,
    INV_ICM45600_ACCEL_FS_4G,
    INV_ICM45600_ACCEL_FS_2G,
    INV_ICM45600_ACCEL_FS_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45686_accel_fs {
    INV_ICM45686_ACCEL_FS_32G,
    INV_ICM45686_ACCEL_FS_16G,
    INV_ICM45686_ACCEL_FS_8G,
    INV_ICM45686_ACCEL_FS_4G,
    INV_ICM45686_ACCEL_FS_2G,
    INV_ICM45686_ACCEL_FS_MAX
}

// ODR suffixed by LN or LP are Low-Noise or Low-Power mode only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm45600_odr {
    INV_ICM45600_ODR_6400HZ_LN = 0x03,
    INV_ICM45600_ODR_3200HZ_LN,
    INV_ICM45600_ODR_1600HZ_LN,
    INV_ICM45600_ODR_800HZ_LN,
    INV_ICM45600_ODR_400HZ,
    INV_ICM45600_ODR_200HZ,
    INV_ICM45600_ODR_100HZ,
    INV_ICM45600_ODR_50HZ,
    INV_ICM45600_ODR_25HZ,
    INV_ICM45600_ODR_12_5HZ,
    INV_ICM45600_ODR_6_25HZ_LP,
    INV_ICM45600_ODR_3_125HZ_LP,
    INV_ICM45600_ODR_1_5625HZ_LP,
    INV_ICM45600_ODR_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_sensor_conf {
    pub mode: u8,
    pub fs: u8,
    pub odr: u8,
    pub filter: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_conf {
    pub gyro: inv_icm45600_sensor_conf,
    pub accel: inv_icm45600_sensor_conf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_suspended {
    pub gyro: inv_icm45600_sensor_mode,
    pub accel: inv_icm45600_sensor_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_chip_info {
    pub whoami: u8,
    pub name: *const c_char,
    pub conf: *const inv_icm45600_conf,
    pub accel_scales: *const c_int,
    pub accel_scales_len: c_int,
    pub gyro_scales: *const c_int,
    pub gyro_scales_len: c_int,
}

//
// struct inv_icm45600_state - driver state variables
// @lock:		lock for serializing multiple registers access.
// @map:		regmap pointer.
// @vddio_supply:	I/O voltage regulator for the chip.
// @orientation:	sensor chip orientation relative to main hardware.
// @conf:		chip sensors configurations.
// @suspended:		suspended sensors configuration.
// @indio_gyro:	gyroscope IIO device.
// @indio_accel:	accelerometer IIO device.
// @chip_info:		chip driver data.
// @timestamp:		interrupt timestamps.
// @fifo:		FIFO management structure.
// @buffer:		data transfer buffer aligned for DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_state {
    pub lock: mutex,
    pub map: *mut regmap,
    pub vddio_supply: *mut regulator,
    pub orientation: iio_mount_matrix,
    pub conf: inv_icm45600_conf,
    pub suspended: inv_icm45600_suspended,
    pub indio_gyro: *mut iio_dev,
    pub indio_accel: *mut iio_dev,
    pub chip_info: *const inv_icm45600_chip_info,
    pub gyro: i64,
    pub accel: i64,
    pub timestamp: },
    pub fifo: inv_icm45600_fifo,
    pub buff: [u8; 2],
    pub u16: __le16,
    pub ireg: [u8; 3],
    pub __aligned(IIO_DMA_MINALIGN): } buffer,
}

//
// struct inv_icm45600_sensor_state - sensor state variables
// @scales:		table of scales.
// @scales_len:		length (nb of items) of the scales table.
// @power_mode:		sensor requested power mode (for common frequencies)
// @ts:			timestamp module states.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm45600_sensor_state {
    pub scales: *const c_int,
    pub scales_len: usize,
    pub power_mode: inv_icm45600_sensor_mode,
    pub ts: inv_sensors_timestamp,
}

// Virtual register addresses: @bank on MSB (16 bits), @address on LSB
// Indirect register access
pub const INV_ICM45600_REG_IREG_ADDR: c_uint = 0x7C;
pub const INV_ICM45600_REG_IREG_DATA: c_uint = 0x7E;
// Direct access registers
pub const INV_ICM45600_REG_MISC2: c_uint = 0x007F;

pub const INV_ICM45600_REG_DRIVE_CONFIG0: c_uint = 0x0032;

pub const INV_ICM45600_SPI_SLEW_RATE_0_5NS: c_int = 6;
pub const INV_ICM45600_SPI_SLEW_RATE_4NS: c_int = 5;
pub const INV_ICM45600_SPI_SLEW_RATE_5NS: c_int = 4;
pub const INV_ICM45600_SPI_SLEW_RATE_7NS: c_int = 3;
pub const INV_ICM45600_SPI_SLEW_RATE_10NS: c_int = 2;
pub const INV_ICM45600_SPI_SLEW_RATE_14NS: c_int = 1;
pub const INV_ICM45600_SPI_SLEW_RATE_38NS: c_int = 0;
pub const INV_ICM45600_REG_INT1_CONFIG2: c_uint = 0x0018;

pub const INV_ICM45600_INT1_CONFIG2_ACTIVE_LOW: c_uint = 0x00;
pub const INV_ICM45600_REG_FIFO_CONFIG0: c_uint = 0x001D;

pub const INV_ICM45600_FIFO_CONFIG0_MODE_BYPASS: c_int = 0;
pub const INV_ICM45600_FIFO_CONFIG0_MODE_STREAM: c_int = 1;
pub const INV_ICM45600_FIFO_CONFIG0_MODE_STOP_ON_FULL: c_int = 2;

pub const INV_ICM45600_FIFO_CONFIG0_FIFO_DEPTH_MAX: c_uint = 0x1F;
pub const INV_ICM45600_REG_FIFO_CONFIG2: c_uint = 0x0020;

pub const INV_ICM45600_REG_FIFO_CONFIG3: c_uint = 0x0021;

pub const INV_ICM45600_REG_FIFO_CONFIG4: c_uint = 0x0022;

// all sensor data are 16 bits (2 registers wide) in big-endian
pub const INV_ICM45600_REG_TEMP_DATA: c_uint = 0x000C;
pub const INV_ICM45600_REG_ACCEL_DATA_X: c_uint = 0x0000;
pub const INV_ICM45600_REG_ACCEL_DATA_Y: c_uint = 0x0002;
pub const INV_ICM45600_REG_ACCEL_DATA_Z: c_uint = 0x0004;
pub const INV_ICM45600_REG_GYRO_DATA_X: c_uint = 0x0006;
pub const INV_ICM45600_REG_GYRO_DATA_Y: c_uint = 0x0008;
pub const INV_ICM45600_REG_GYRO_DATA_Z: c_uint = 0x000A;
pub const INV_ICM45600_REG_INT_STATUS: c_uint = 0x0019;

//
// FIFO access registers
// FIFO count is 16 bits (2 registers)
// FIFO data is a continuous read register to read FIFO content
//
pub const INV_ICM45600_REG_FIFO_COUNT: c_uint = 0x0012;
pub const INV_ICM45600_REG_FIFO_DATA: c_uint = 0x0014;
pub const INV_ICM45600_REG_PWR_MGMT0: c_uint = 0x0010;

pub const INV_ICM45600_REG_ACCEL_CONFIG0: c_uint = 0x001B;

pub const INV_ICM45600_REG_GYRO_CONFIG0: c_uint = 0x001C;

pub const INV_ICM45600_REG_SMC_CONTROL_0: c_uint = 0xA258;

// FIFO watermark is 16 bits (2 registers wide) in little-endian
pub const INV_ICM45600_REG_FIFO_WATERMARK: c_uint = 0x001E;
// FIFO is configured for 8kb

pub const INV_ICM45600_REG_INT1_CONFIG0: c_uint = 0x0016;

pub const INV_ICM45600_REG_WHOAMI: c_uint = 0x0072;
pub const INV_ICM45600_WHOAMI_ICM45605: c_uint = 0xE5;
pub const INV_ICM45600_WHOAMI_ICM45686: c_uint = 0xE9;
pub const INV_ICM45600_WHOAMI_ICM45688P: c_uint = 0xE7;
pub const INV_ICM45600_WHOAMI_ICM45608: c_uint = 0x81;
pub const INV_ICM45600_WHOAMI_ICM45634: c_uint = 0x82;
pub const INV_ICM45600_WHOAMI_ICM45689: c_uint = 0x83;
pub const INV_ICM45600_WHOAMI_ICM45606: c_uint = 0x84;
pub const INV_ICM45600_WHOAMI_ICM45687: c_uint = 0x85;
// Gyro USER offset
pub const INV_ICM45600_IPREG_SYS1_REG_42: c_uint = 0xA42A;
pub const INV_ICM45600_IPREG_SYS1_REG_56: c_uint = 0xA438;
pub const INV_ICM45600_IPREG_SYS1_REG_70: c_uint = 0xA446;

// Gyro Averaging filter
pub const INV_ICM45600_IPREG_SYS1_REG_170: c_uint = 0xA4AA;

pub const INV_ICM45600_GYRO_LP_AVG_SEL_8X: c_int = 5;
pub const INV_ICM45600_GYRO_LP_AVG_SEL_2X: c_int = 1;
// Accel USER offset
pub const INV_ICM45600_IPREG_SYS2_REG_24: c_uint = 0xA518;
pub const INV_ICM45600_IPREG_SYS2_REG_32: c_uint = 0xA520;
pub const INV_ICM45600_IPREG_SYS2_REG_40: c_uint = 0xA528;

// Accel averaging filter
pub const INV_ICM45600_IPREG_SYS2_REG_129: c_uint = 0xA581;
pub const INV_ICM45600_ACCEL_LP_AVG_SEL_1X: c_uint = 0x0000;
pub const INV_ICM45600_ACCEL_LP_AVG_SEL_4X: c_uint = 0x0002;
// Sleep times required by the driver
pub const INV_ICM45600_ACCEL_STARTUP_TIME_MS: c_int = 60;
pub const INV_ICM45600_GYRO_STARTUP_TIME_MS: c_int = 60;
pub const INV_ICM45600_GYRO_STOP_TIME_MS: c_int = 150;
pub const INV_ICM45600_IREG_DELAY_US: c_int = 4;
extern "C" {
    pub fn int(: *mut *mut inv_icm45600_bus_setup)(struct inv_icm45600_state) -> typedef;
}

extern "C" {
    pub fn inv_icm45600_odr_to_period(odr: inv_icm45600_odr) -> u32;
}
extern "C" {
    pub fn inv_icm45600_gyro_parse_fifo(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn inv_icm45600_accel_parse_fifo(indio_dev: *mut iio_dev) -> c_int;
}
