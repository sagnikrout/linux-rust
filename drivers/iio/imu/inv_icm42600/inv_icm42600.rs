//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_icm42600/inv_icm42600.h
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
// Copyright (C) 2020 Invensense, Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_chip {
    INV_CHIP_INVALID,
    INV_CHIP_ICM42600,
    INV_CHIP_ICM42602,
    INV_CHIP_ICM42605,
    INV_CHIP_ICM42686,
    INV_CHIP_ICM42622,
    INV_CHIP_ICM42688,
    INV_CHIP_ICM42631,
    INV_CHIP_NB,
}

// serial bus slew rates
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_slew_rate {
    INV_ICM42600_SLEW_RATE_20_60NS,
    INV_ICM42600_SLEW_RATE_12_36NS,
    INV_ICM42600_SLEW_RATE_6_18NS,
    INV_ICM42600_SLEW_RATE_4_12NS,
    INV_ICM42600_SLEW_RATE_2_6NS,
    INV_ICM42600_SLEW_RATE_INF_2NS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_sensor_mode {
    INV_ICM42600_SENSOR_MODE_OFF,
    INV_ICM42600_SENSOR_MODE_STANDBY,
    INV_ICM42600_SENSOR_MODE_LOW_POWER,
    INV_ICM42600_SENSOR_MODE_LOW_NOISE,
    INV_ICM42600_SENSOR_MODE_NB,
}

// gyroscope fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_gyro_fs {
    INV_ICM42600_GYRO_FS_2000DPS,
    INV_ICM42600_GYRO_FS_1000DPS,
    INV_ICM42600_GYRO_FS_500DPS,
    INV_ICM42600_GYRO_FS_250DPS,
    INV_ICM42600_GYRO_FS_125DPS,
    INV_ICM42600_GYRO_FS_62_5DPS,
    INV_ICM42600_GYRO_FS_31_25DPS,
    INV_ICM42600_GYRO_FS_15_625DPS,
    INV_ICM42600_GYRO_FS_NB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42686_gyro_fs {
    INV_ICM42686_GYRO_FS_4000DPS,
    INV_ICM42686_GYRO_FS_2000DPS,
    INV_ICM42686_GYRO_FS_1000DPS,
    INV_ICM42686_GYRO_FS_500DPS,
    INV_ICM42686_GYRO_FS_250DPS,
    INV_ICM42686_GYRO_FS_125DPS,
    INV_ICM42686_GYRO_FS_62_5DPS,
    INV_ICM42686_GYRO_FS_31_25DPS,
    INV_ICM42686_GYRO_FS_NB,
}

// accelerometer fullscale values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_accel_fs {
    INV_ICM42600_ACCEL_FS_16G,
    INV_ICM42600_ACCEL_FS_8G,
    INV_ICM42600_ACCEL_FS_4G,
    INV_ICM42600_ACCEL_FS_2G,
    INV_ICM42600_ACCEL_FS_NB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42686_accel_fs {
    INV_ICM42686_ACCEL_FS_32G,
    INV_ICM42686_ACCEL_FS_16G,
    INV_ICM42686_ACCEL_FS_8G,
    INV_ICM42686_ACCEL_FS_4G,
    INV_ICM42686_ACCEL_FS_2G,
    INV_ICM42686_ACCEL_FS_NB,
}

// ODR suffixed by LN or LP are Low-Noise or Low-Power mode only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_odr {
    INV_ICM42600_ODR_8KHZ_LN = 3,
    INV_ICM42600_ODR_4KHZ_LN,
    INV_ICM42600_ODR_2KHZ_LN,
    INV_ICM42600_ODR_1KHZ_LN,
    INV_ICM42600_ODR_200HZ,
    INV_ICM42600_ODR_100HZ,
    INV_ICM42600_ODR_50HZ,
    INV_ICM42600_ODR_25HZ,
    INV_ICM42600_ODR_12_5HZ,
    INV_ICM42600_ODR_6_25HZ_LP,
    INV_ICM42600_ODR_3_125HZ_LP,
    INV_ICM42600_ODR_1_5625HZ_LP,
    INV_ICM42600_ODR_500HZ,
    INV_ICM42600_ODR_NB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_icm42600_filter {
// Low-Noise mode sensor data filter (3rd order filter by default)
    INV_ICM42600_FILTER_BW_ODR_DIV_2,

// Low-Power mode sensor data filter (averaging)
    INV_ICM42600_FILTER_AVG_1X = 1,
    INV_ICM42600_FILTER_AVG_16X = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_sensor_conf {
    pub mode: c_int,
    pub fs: c_int,
    pub odr: c_int,
    pub filter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_conf {
    pub gyro: inv_icm42600_sensor_conf,
    pub accel: inv_icm42600_sensor_conf,
    pub temp_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_suspended {
    pub gyro: inv_icm42600_sensor_mode,
    pub accel: inv_icm42600_sensor_mode,
    pub temp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_apex {
    pub on: c_uint,
    pub value: u64,
    pub enable: bool,
    pub wom: },
}

//
// struct inv_icm42600_state - driver state variables
// @lock:		lock for serializing multiple registers access.
// @chip:		chip identifier.
// @name:		chip name.
// @map:		regmap pointer.
// @vdd_supply:	VDD voltage regulator for the chip.
// @vddio_supply:	I/O voltage regulator for the chip.
// @irq:		chip irq, required to enable/disable and set wakeup
// @orientation:	sensor chip orientation relative to main hardware.
// @conf:		chip sensors configurations.
// @suspended:		suspended sensors configuration.
// @indio_gyro:	gyroscope IIO device.
// @indio_accel:	accelerometer IIO device.
// @timestamp:		interrupt timestamps.
// @apex:		APEX (Advanced Pedometer and Event detection) management
// @fifo:		FIFO management structure.
// @buffer:		data transfer buffer aligned for DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_state {
    pub lock: mutex,
    pub chip: inv_icm42600_chip,
    pub name: *const c_char,
    pub map: *mut regmap,
    pub vddio_supply: *mut regulator,
    pub irq: c_int,
    pub orientation: iio_mount_matrix,
    pub conf: inv_icm42600_conf,
    pub suspended: inv_icm42600_suspended,
    pub indio_gyro: *mut iio_dev,
    pub indio_accel: *mut iio_dev,
    pub gyro: i64,
    pub accel: i64,
    pub timestamp: },
    pub apex: inv_icm42600_apex,
    pub fifo: inv_icm42600_fifo,
    pub __aligned(IIO_DMA_MINALIGN): u8 buffer[3],
}

//
// struct inv_icm42600_sensor_state - sensor state variables
// @scales:		table of scales.
// @scales_len:		length (nb of items) of the scales table.
// @power_mode:		sensor requested power mode (for common frequencies)
// @filter:		sensor filter.
// @ts:			timestamp module states.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_icm42600_sensor_state {
    pub scales: *const c_int,
    pub scales_len: usize,
    pub power_mode: inv_icm42600_sensor_mode,
    pub filter: inv_icm42600_filter,
    pub ts: inv_sensors_timestamp,
}

// Virtual register addresses: @bank on MSB (4 upper bits), @address on LSB
// Bank selection register, available in all banks
pub const INV_ICM42600_REG_BANK_SEL: c_uint = 0x76;

// User bank 0 (MSB 0x00)
pub const INV_ICM42600_REG_DEVICE_CONFIG: c_uint = 0x0011;

pub const INV_ICM42600_REG_DRIVE_CONFIG: c_uint = 0x0013;

pub const INV_ICM42600_REG_INT_CONFIG: c_uint = 0x0014;

pub const INV_ICM42600_INT_CONFIG_INT2_ACTIVE_LOW: c_uint = 0x00;

pub const INV_ICM42600_INT_CONFIG_INT1_ACTIVE_LOW: c_uint = 0x00;
pub const INV_ICM42600_REG_FIFO_CONFIG: c_uint = 0x0016;

// all sensor data are 16 bits (2 registers wide) in big-endian
pub const INV_ICM42600_REG_TEMP_DATA: c_uint = 0x001D;
pub const INV_ICM42600_REG_ACCEL_DATA_X: c_uint = 0x001F;
pub const INV_ICM42600_REG_ACCEL_DATA_Y: c_uint = 0x0021;
pub const INV_ICM42600_REG_ACCEL_DATA_Z: c_uint = 0x0023;
pub const INV_ICM42600_REG_GYRO_DATA_X: c_uint = 0x0025;
pub const INV_ICM42600_REG_GYRO_DATA_Y: c_uint = 0x0027;
pub const INV_ICM42600_REG_GYRO_DATA_Z: c_uint = 0x0029;

pub const INV_ICM42600_REG_INT_STATUS: c_uint = 0x002D;

//
// FIFO access registers
// FIFO count is 16 bits (2 registers) big-endian
// FIFO data is a continuous read register to read FIFO content
//
pub const INV_ICM42600_REG_FIFO_COUNT: c_uint = 0x002E;
pub const INV_ICM42600_REG_FIFO_DATA: c_uint = 0x0030;
pub const INV_ICM42600_REG_INT_STATUS2: c_uint = 0x0037;

pub const INV_ICM42600_REG_INT_STATUS3: c_uint = 0x0038;

pub const INV_ICM42600_REG_SIGNAL_PATH_RESET: c_uint = 0x004B;

// default configuration: all data big-endian and fifo count in bytes
pub const INV_ICM42600_REG_INTF_CONFIG0: c_uint = 0x004C;

pub const INV_ICM42600_REG_INTF_CONFIG1: c_uint = 0x004D;

pub const INV_ICM42600_REG_PWR_MGMT0: c_uint = 0x004E;

pub const INV_ICM42600_REG_GYRO_CONFIG0: c_uint = 0x004F;

pub const INV_ICM42600_REG_ACCEL_CONFIG0: c_uint = 0x0050;

pub const INV_ICM42600_REG_GYRO_ACCEL_CONFIG0: c_uint = 0x0052;

pub const INV_ICM42600_REG_TMST_CONFIG: c_uint = 0x0054;

pub const INV_ICM42600_REG_SMD_CONFIG: c_uint = 0x0057;

pub const INV_ICM42600_SMD_CONFIG_SMD_MODE_OFF: c_uint = 0x00;
pub const INV_ICM42600_SMD_CONFIG_SMD_MODE_WOM: c_uint = 0x01;
pub const INV_ICM42600_SMD_CONFIG_SMD_MODE_SHORT: c_uint = 0x02;
pub const INV_ICM42600_SMD_CONFIG_SMD_MODE_LONG: c_uint = 0x03;
pub const INV_ICM42600_REG_FIFO_CONFIG1: c_uint = 0x005F;

// FIFO watermark is 16 bits (2 registers wide) in little-endian
pub const INV_ICM42600_REG_FIFO_WATERMARK: c_uint = 0x0060;

// FIFO is 2048 bytes, let 12 samples for reading latency

// INV_ICM42600_FIFO_WATERMARK_MAX / 8 = 232
pub const INV_ICM42600_FIFO_WATERMARK_MAX_SAMPLES: c_int = 232;
pub const INV_ICM42600_REG_INT_CONFIG1: c_uint = 0x0064;

pub const INV_ICM42600_REG_INT_SOURCE0: c_uint = 0x0065;

pub const INV_ICM42600_REG_INT_SOURCE1: c_uint = 0x0066;

pub const INV_ICM42600_REG_WHOAMI: c_uint = 0x0075;
pub const INV_ICM42600_WHOAMI_ICM42600: c_uint = 0x40;
pub const INV_ICM42600_WHOAMI_ICM42602: c_uint = 0x41;
pub const INV_ICM42600_WHOAMI_ICM42605: c_uint = 0x42;
pub const INV_ICM42600_WHOAMI_ICM42686: c_uint = 0x44;
pub const INV_ICM42600_WHOAMI_ICM42622: c_uint = 0x46;
pub const INV_ICM42600_WHOAMI_ICM42688: c_uint = 0x47;
pub const INV_ICM42600_WHOAMI_ICM42631: c_uint = 0x5C;
// User bank 1 (MSB 0x10)
pub const INV_ICM42600_REG_SENSOR_CONFIG0: c_uint = 0x1003;

// Timestamp value is 20 bits (3 registers) in little-endian
pub const INV_ICM42600_REG_TMSTVAL: c_uint = 0x1062;

pub const INV_ICM42600_REG_INTF_CONFIG4: c_uint = 0x107A;

pub const INV_ICM42600_REG_INTF_CONFIG6: c_uint = 0x107C;

// User bank 4 (MSB 0x40)
pub const INV_ICM42600_REG_ACCEL_WOM_X_THR: c_uint = 0x404A;
pub const INV_ICM42600_REG_ACCEL_WOM_Y_THR: c_uint = 0x404B;
pub const INV_ICM42600_REG_ACCEL_WOM_Z_THR: c_uint = 0x404C;
pub const INV_ICM42600_REG_INT_SOURCE8: c_uint = 0x404F;

pub const INV_ICM42600_REG_OFFSET_USER0: c_uint = 0x4077;
pub const INV_ICM42600_REG_OFFSET_USER1: c_uint = 0x4078;
pub const INV_ICM42600_REG_OFFSET_USER2: c_uint = 0x4079;
pub const INV_ICM42600_REG_OFFSET_USER3: c_uint = 0x407A;
pub const INV_ICM42600_REG_OFFSET_USER4: c_uint = 0x407B;
pub const INV_ICM42600_REG_OFFSET_USER5: c_uint = 0x407C;
pub const INV_ICM42600_REG_OFFSET_USER6: c_uint = 0x407D;
pub const INV_ICM42600_REG_OFFSET_USER7: c_uint = 0x407E;
pub const INV_ICM42600_REG_OFFSET_USER8: c_uint = 0x407F;
// Sleep times required by the driver
pub const INV_ICM42600_POWER_UP_TIME_MS: c_int = 100;
pub const INV_ICM42600_RESET_TIME_MS: c_int = 1;
pub const INV_ICM42600_ACCEL_STARTUP_TIME_MS: c_int = 20;
pub const INV_ICM42600_GYRO_STARTUP_TIME_MS: c_int = 60;
pub const INV_ICM42600_GYRO_STOP_TIME_MS: c_int = 150;
pub const INV_ICM42600_TEMP_STARTUP_TIME_MS: c_int = 14;
pub const INV_ICM42600_SUSPEND_DELAY_MS: c_int = 2000;
extern "C" {
    pub fn int(: *mut *mut inv_icm42600_bus_setup)(struct inv_icm42600_state) -> typedef;
}
extern "C" {
    pub fn inv_icm42600_odr_to_period(odr: inv_icm42600_odr) -> u32;
}
extern "C" {
    pub fn inv_icm42600_enable_wom(st: *mut inv_icm42600_state) -> c_int;
}
extern "C" {
    pub fn inv_icm42600_disable_wom(st: *mut inv_icm42600_state) -> c_int;
}
extern "C" {
    pub fn inv_icm42600_gyro_parse_fifo(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn inv_icm42600_accel_parse_fifo(indio_dev: *mut iio_dev) -> c_int;
}
