//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/inv_mpu6050/inv_mpu_iio.h
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
// Copyright (C) 2012 Invensense, Inc.
//

//
// struct inv_mpu6050_reg_map - Notable registers.
// @sample_rate_div:	Divider applied to gyro output rate.
// @lpf:		Configures internal low pass filter.
// @accel_lpf:		Configures accelerometer low pass filter.
// @user_ctrl:		Enables/resets the FIFO.
// @fifo_en:		Determines which data will appear in FIFO.
// @gyro_config:	gyro config register.
// @accl_config:	accel config register
// @fifo_count_h:	Upper byte of FIFO count.
// @fifo_r_w:		FIFO register.
// @raw_gyro:		Address of first gyro register.
// @raw_accl:		Address of first accel register.
// @temperature:	temperature register
// @int_enable:	Interrupt enable register.
// @int_status:	Interrupt status register.
// @pwr_mgmt_1:	Controls chip's power state and clock source.
// @pwr_mgmt_2:	Controls power state of individual sensors.
// @int_pin_cfg;	Controls interrupt pin configuration.
// @accl_offset:	Controls the accelerometer calibration offset.
// @gyro_offset:	Controls the gyroscope calibration offset.
// @i2c_if:		Controls the i2c interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_mpu6050_reg_map {
    pub sample_rate_div: u8,
    pub lpf: u8,
    pub accel_lpf: u8,
    pub user_ctrl: u8,
    pub fifo_en: u8,
    pub gyro_config: u8,
    pub accl_config: u8,
    pub fifo_count_h: u8,
    pub fifo_r_w: u8,
    pub raw_gyro: u8,
    pub raw_accl: u8,
    pub temperature: u8,
    pub int_enable: u8,
    pub int_status: u8,
    pub pwr_mgmt_1: u8,
    pub pwr_mgmt_2: u8,
    pub int_pin_cfg: u8,
    pub accl_offset: u8,
    pub gyro_offset: u8,
    pub i2c_if: u8,
}

// device enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_devices {
    INV_MPU6050,
    INV_MPU6500,
    INV_MPU6515,
    INV_MPU6880,
    INV_MPU6000,
    INV_MPU9150,
    INV_MPU9250,
    INV_MPU9255,
    INV_ICM20608,
    INV_ICM20608D,
    INV_ICM20609,
    INV_ICM20689,
    INV_ICM20600,
    INV_ICM20602,
    INV_ICM20690,
    INV_IAM20380,
    INV_IAM20680,
    INV_IAM20680HP,
    INV_IAM20680HT,
    INV_NUM_PARTS
}

// chip sensors mask: accelerometer, gyroscope, temperature, magnetometer, WoM

//
// struct inv_mpu6050_chip_config - Cached chip configuration data.
// @clk:		selected chip clock
// @fsr:		Full scale range.
// @lpf:		Digital low pass filter frequency.
// @accl_fs:		accel full scale range.
// @accl_en:		accel engine enabled
// @gyro_en:		gyro engine enabled
// @temp_en:		temperature sensor enabled
// @magn_en:		magn engine (i2c master) enabled
// @wom_en:		Wake-on-Motion enabled
// @accl_fifo_enable:	enable accel data output
// @gyro_fifo_enable:	enable gyro data output
// @temp_fifo_enable:	enable temp data output
// @magn_fifo_enable:	enable magn data output
// @divider:		chip sample rate divider (sample rate divider - 1)
// @roc_threshold:	save ROC threshold (WoM) set value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_mpu6050_chip_config {
    pub clk:3: c_uint,
    pub fsr:2: c_uint,
    pub lpf:3: c_uint,
    pub accl_fs:2: c_uint,
    pub accl_en:1: c_uint,
    pub gyro_en:1: c_uint,
    pub temp_en:1: c_uint,
    pub magn_en:1: c_uint,
    pub wom_en:1: c_uint,
    pub accl_fifo_enable:1: c_uint,
    pub gyro_fifo_enable:1: c_uint,
    pub temp_fifo_enable:1: c_uint,
    pub magn_fifo_enable:1: c_uint,
    pub divider: u8,
    pub user_ctrl: u8,
    pub roc_threshold: u64,
}

//
// Maximum of 6 + 6 + 2 + 7 (for MPU9x50) = 21 round up to 24 and plus 8.
// May be less if fewer channels are enabled, as long as the timestamp
// remains 8 byte aligned
//
pub const INV_MPU6050_OUTPUT_DATA_SIZE: c_int = 32;
//
// struct inv_mpu6050_hw - Other important hardware information.
// @whoami:	Self identification byte from WHO_AM_I register
// @name:      name of the chip.
// @reg:   register map of the chip.
// @config:    configuration of the chip.
// @fifo_size:	size of the FIFO in bytes.
// @temp:	offset and scale to apply to raw temperature.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_mpu6050_hw {
    pub whoami: u8,
    pub name: *mut u8,
    pub reg: *const inv_mpu6050_reg_map,
    pub config: *const inv_mpu6050_chip_config,
    pub fifo_size: usize,
    pub offset: c_int,
    pub scale: c_int,
    pub temp: },
    pub accel: c_uint,
    pub gyro: c_uint,
    pub startup_time: },
}

//
// struct inv_mpu6050_state - Driver state variables.
// @lock:              Chip access lock.
// @trig:              IIO trigger.
// @chip_config:	Cached attribute information.
// @reg:		Map of important registers.
// @hw:		Other hardware-specific information.
// @chip_type:		chip type.
// @plat_data:		platform data (deprecated in favor of @orientation).
// @orientation:	sensor chip orientation relative to main hardware.
// @map		regmap pointer.
// @irq		interrupt number.
// @irq_mask		the int_pin_cfg mask to configure interrupt type.
// @timestamp:		timestamping module
// @vdd_supply:	VDD voltage regulator for the chip.
// @vddio_supply	I/O voltage regulator for the chip.
// @magn_disabled:     magnetometer disabled for backward compatibility reason.
// @magn_raw_to_gauss:	coefficient to convert mag raw value to Gauss.
// @magn_orient:       magnetometer sensor chip orientation if available.
// @suspended_sensors:	sensors mask of sensors turned off for suspend
// @data:		read buffer used for bulk reads.
// @it_timestamp:	interrupt timestamp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inv_mpu6050_state {
    pub lock: mutex,
    pub trig: *mut iio_trigger,
    pub chip_config: inv_mpu6050_chip_config,
    pub reg: *const inv_mpu6050_reg_map,
    pub hw: *const inv_mpu6050_hw,
    pub chip_type: inv_devices,
    pub muxc: *mut i2c_mux_core,
    pub mux_client: *mut i2c_client,
    pub plat_data: inv_mpu6050_platform_data,
    pub orientation: iio_mount_matrix,
    pub map: *mut regmap,
    pub irq: c_int,
    pub irq_mask: u8,
    pub skip_samples: unsigned,
    pub timestamp: inv_sensors_timestamp,
    pub vdd_supply: *mut regulator,
    pub vddio_supply: *mut regulator,
    pub magn_disabled: bool,
    pub magn_raw_to_gauss: [i32; 3],
    pub magn_orient: iio_mount_matrix,
    pub suspended_sensors: c_uint,
    pub level_shifter: bool,
    pub data: *mut u8,
    pub it_timestamp: i64,
}

// register and associated bit definition
pub const INV_MPU6050_REG_ACCEL_OFFSET: c_uint = 0x06;
pub const INV_MPU6050_REG_GYRO_OFFSET: c_uint = 0x13;
pub const INV_MPU6050_REG_SAMPLE_RATE_DIV: c_uint = 0x19;
pub const INV_MPU6050_REG_CONFIG: c_uint = 0x1A;
pub const INV_MPU6050_REG_GYRO_CONFIG: c_uint = 0x1B;
pub const INV_MPU6050_REG_ACCEL_CONFIG: c_uint = 0x1C;
pub const INV_MPU6050_REG_FIFO_EN: c_uint = 0x23;
pub const INV_MPU6050_BIT_SLAVE_0: c_uint = 0x01;
pub const INV_MPU6050_BIT_SLAVE_1: c_uint = 0x02;
pub const INV_MPU6050_BIT_SLAVE_2: c_uint = 0x04;
pub const INV_MPU6050_BIT_ACCEL_OUT: c_uint = 0x08;
pub const INV_MPU6050_BITS_GYRO_OUT: c_uint = 0x70;
pub const INV_MPU6050_BIT_TEMP_OUT: c_uint = 0x80;
pub const INV_MPU6050_REG_I2C_MST_CTRL: c_uint = 0x24;
pub const INV_MPU6050_BITS_I2C_MST_CLK_400KHZ: c_uint = 0x0D;
pub const INV_MPU6050_BIT_I2C_MST_P_NSR: c_uint = 0x10;
pub const INV_MPU6050_BIT_SLV3_FIFO_EN: c_uint = 0x20;
pub const INV_MPU6050_BIT_WAIT_FOR_ES: c_uint = 0x40;
pub const INV_MPU6050_BIT_MULT_MST_EN: c_uint = 0x80;
// control I2C slaves from 0 to 3

pub const INV_MPU6050_BIT_I2C_SLV_RNW: c_uint = 0x80;

pub const INV_MPU6050_BIT_SLV_GRP: c_uint = 0x10;
pub const INV_MPU6050_BIT_SLV_REG_DIS: c_uint = 0x20;
pub const INV_MPU6050_BIT_SLV_BYTE_SW: c_uint = 0x40;
pub const INV_MPU6050_BIT_SLV_EN: c_uint = 0x80;
// I2C master delay register
pub const INV_MPU6050_REG_I2C_SLV4_CTRL: c_uint = 0x34;

pub const INV_MPU6050_REG_I2C_MST_STATUS: c_uint = 0x36;
pub const INV_MPU6050_BIT_I2C_SLV0_NACK: c_uint = 0x01;
pub const INV_MPU6050_BIT_I2C_SLV1_NACK: c_uint = 0x02;
pub const INV_MPU6050_BIT_I2C_SLV2_NACK: c_uint = 0x04;
pub const INV_MPU6050_BIT_I2C_SLV3_NACK: c_uint = 0x08;
pub const INV_MPU6050_REG_INT_ENABLE: c_uint = 0x38;
pub const INV_MPU6050_BIT_DATA_RDY_EN: c_uint = 0x01;
pub const INV_MPU6050_BIT_DMP_INT_EN: c_uint = 0x02;

pub const INV_MPU6050_REG_RAW_ACCEL: c_uint = 0x3B;
pub const INV_MPU6050_REG_TEMPERATURE: c_uint = 0x41;
pub const INV_MPU6050_REG_RAW_GYRO: c_uint = 0x43;
pub const INV_MPU6050_REG_INT_STATUS: c_uint = 0x3A;

pub const INV_MPU6050_BIT_FIFO_OVERFLOW_INT: c_uint = 0x10;
pub const INV_MPU6050_BIT_RAW_DATA_RDY_INT: c_uint = 0x01;
pub const INV_MPU6050_REG_EXT_SENS_DATA: c_uint = 0x49;
// I2C slaves data output from 0 to 3

pub const INV_MPU6050_REG_I2C_MST_DELAY_CTRL: c_uint = 0x67;
pub const INV_MPU6050_BIT_I2C_SLV0_DLY_EN: c_uint = 0x01;
pub const INV_MPU6050_BIT_I2C_SLV1_DLY_EN: c_uint = 0x02;
pub const INV_MPU6050_BIT_I2C_SLV2_DLY_EN: c_uint = 0x04;
pub const INV_MPU6050_BIT_I2C_SLV3_DLY_EN: c_uint = 0x08;
pub const INV_MPU6050_BIT_DELAY_ES_SHADOW: c_uint = 0x80;
pub const INV_MPU6050_REG_SIGNAL_PATH_RESET: c_uint = 0x68;

pub const INV_MPU6050_REG_USER_CTRL: c_uint = 0x6A;
pub const INV_MPU6050_BIT_SIG_COND_RST: c_uint = 0x01;
pub const INV_MPU6050_BIT_FIFO_RST: c_uint = 0x04;
pub const INV_MPU6050_BIT_DMP_RST: c_uint = 0x08;
pub const INV_MPU6050_BIT_I2C_MST_EN: c_uint = 0x20;
pub const INV_MPU6050_BIT_FIFO_EN: c_uint = 0x40;
pub const INV_MPU6050_BIT_DMP_EN: c_uint = 0x80;
pub const INV_MPU6050_BIT_I2C_IF_DIS: c_uint = 0x10;
pub const INV_MPU6050_REG_PWR_MGMT_1: c_uint = 0x6B;
pub const INV_MPU6050_BIT_H_RESET: c_uint = 0x80;
pub const INV_MPU6050_BIT_SLEEP: c_uint = 0x40;
pub const INV_MPU6050_BIT_CYCLE: c_uint = 0x20;
pub const INV_MPU6050_BIT_TEMP_DIS: c_uint = 0x08;
pub const INV_MPU6050_BIT_CLK_MASK: c_uint = 0x7;
pub const INV_MPU6050_REG_PWR_MGMT_2: c_uint = 0x6C;
pub const INV_MPU6050_BIT_PWR_ACCL_STBY: c_uint = 0x38;
pub const INV_MPU6050_BIT_PWR_GYRO_STBY: c_uint = 0x07;
// ICM20609 registers
pub const INV_ICM20609_REG_ACCEL_WOM_X_THR: c_uint = 0x20;
pub const INV_ICM20609_REG_ACCEL_WOM_Y_THR: c_uint = 0x21;
pub const INV_ICM20609_REG_ACCEL_WOM_Z_THR: c_uint = 0x22;
// ICM20602 register
pub const INV_ICM20602_REG_I2C_IF: c_uint = 0x70;
pub const INV_ICM20602_BIT_I2C_IF_DIS: c_uint = 0x40;
pub const INV_MPU6050_REG_FIFO_COUNT_H: c_uint = 0x72;
pub const INV_MPU6050_REG_FIFO_R_W: c_uint = 0x74;
pub const INV_MPU6050_BYTES_PER_3AXIS_SENSOR: c_int = 6;
pub const INV_MPU6050_FIFO_COUNT_BYTE: c_int = 2;
// MPU9X50 9-axis magnetometer
pub const INV_MPU9X50_BYTES_MAGN: c_int = 7;
// FIFO temperature sample size
pub const INV_MPU6050_BYTES_PER_TEMP_SENSOR: c_int = 2;
// mpu6500 registers
pub const INV_MPU6500_REG_ACCEL_CONFIG_2: c_uint = 0x1D;
pub const INV_ICM20689_BITS_FIFO_SIZE_MAX: c_uint = 0xC0;
pub const INV_MPU6500_REG_LP_ODR: c_uint = 0x1E;
pub const INV_MPU6500_REG_WOM_THRESHOLD: c_uint = 0x1F;
pub const INV_MPU6500_REG_ACCEL_INTEL_CTRL: c_uint = 0x69;

pub const INV_MPU6500_REG_ACCEL_OFFSET: c_uint = 0x77;
// delay time in milliseconds
pub const INV_MPU6050_POWER_UP_TIME: c_int = 100;
pub const INV_MPU6050_TEMP_UP_TIME: c_int = 100;
pub const INV_MPU6050_ACCEL_STARTUP_TIME: c_int = 20;
pub const INV_MPU6050_GYRO_STARTUP_TIME: c_int = 60;
pub const INV_MPU6050_GYRO_DOWN_TIME: c_int = 150;
pub const INV_MPU6050_SUSPEND_DELAY_MS: c_int = 2000;
pub const INV_MPU6500_GYRO_STARTUP_TIME: c_int = 70;
pub const INV_MPU6500_ACCEL_STARTUP_TIME: c_int = 30;
pub const INV_ICM20602_GYRO_STARTUP_TIME: c_int = 100;
pub const INV_ICM20602_ACCEL_STARTUP_TIME: c_int = 20;
pub const INV_ICM20690_GYRO_STARTUP_TIME: c_int = 80;
pub const INV_ICM20690_ACCEL_STARTUP_TIME: c_int = 10;
// delay time in microseconds
pub const INV_MPU6050_REG_UP_TIME_MIN: c_int = 5000;
pub const INV_MPU6050_REG_UP_TIME_MAX: c_int = 10000;
pub const INV_MPU6050_TEMP_OFFSET: c_int = 12420;
pub const INV_MPU6050_TEMP_SCALE: c_int = 2941176;
pub const INV_MPU6050_MAX_GYRO_FS_PARAM: c_int = 3;
pub const INV_MPU6050_MAX_ACCL_FS_PARAM: c_int = 3;
pub const INV_MPU6050_THREE_AXIS: c_int = 3;
pub const INV_MPU6050_GYRO_CONFIG_FSR_SHIFT: c_int = 3;
pub const INV_ICM20690_GYRO_CONFIG_FSR_SHIFT: c_int = 2;
pub const INV_MPU6050_ACCL_CONFIG_FSR_SHIFT: c_int = 3;
pub const INV_MPU6500_TEMP_OFFSET: c_int = 7011;
pub const INV_MPU6500_TEMP_SCALE: c_int = 2995178;
pub const INV_ICM20608_TEMP_OFFSET: c_int = 8170;
pub const INV_ICM20608_TEMP_SCALE: c_int = 3059976;
pub const INV_MPU6050_REG_INT_PIN_CFG: c_uint = 0x37;
pub const INV_MPU6050_ACTIVE_HIGH: c_uint = 0x00;
pub const INV_MPU6050_ACTIVE_LOW: c_uint = 0x80;
// enable level triggering
pub const INV_MPU6050_LATCH_INT_EN: c_uint = 0x20;
pub const INV_MPU6050_BIT_BYPASS_EN: c_uint = 0x2;
// allow acking interrupts by any register read
pub const INV_MPU6050_INT_RD_CLEAR: c_uint = 0x10;
// Allowed timestamp period jitter in percent
pub const INV_MPU6050_TS_PERIOD_JITTER: c_int = 4;
// init parameters
pub const INV_MPU6050_MAX_FIFO_RATE: c_int = 1000;
pub const INV_MPU6050_MIN_FIFO_RATE: c_int = 4;
// chip internal frequency: 1KHz
pub const INV_MPU6050_INTERNAL_FREQ_HZ: c_int = 1000;
// return the frequency divider (chip sample rate divider + 1)

// chip sample rate divider to fifo rate

pub const INV_MPU6050_REG_WHOAMI: c_int = 117;
pub const INV_MPU6000_WHOAMI_VALUE: c_uint = 0x68;
pub const INV_MPU6050_WHOAMI_VALUE: c_uint = 0x68;
pub const INV_MPU6500_WHOAMI_VALUE: c_uint = 0x70;
pub const INV_MPU6880_WHOAMI_VALUE: c_uint = 0x78;
pub const INV_MPU9150_WHOAMI_VALUE: c_uint = 0x68;
pub const INV_MPU9250_WHOAMI_VALUE: c_uint = 0x71;
pub const INV_MPU9255_WHOAMI_VALUE: c_uint = 0x73;
pub const INV_MPU6515_WHOAMI_VALUE: c_uint = 0x74;
pub const INV_ICM20608_WHOAMI_VALUE: c_uint = 0xAF;
pub const INV_ICM20608D_WHOAMI_VALUE: c_uint = 0xAE;
pub const INV_ICM20609_WHOAMI_VALUE: c_uint = 0xA6;
pub const INV_ICM20689_WHOAMI_VALUE: c_uint = 0x98;
pub const INV_ICM20600_WHOAMI_VALUE: c_uint = 0x11;
pub const INV_ICM20602_WHOAMI_VALUE: c_uint = 0x12;
pub const INV_ICM20690_WHOAMI_VALUE: c_uint = 0x20;
pub const INV_IAM20380_WHOAMI_VALUE: c_uint = 0xB5;
pub const INV_IAM20680_WHOAMI_VALUE: c_uint = 0xA9;
pub const INV_IAM20680HP_WHOAMI_VALUE: c_uint = 0xF8;
pub const INV_IAM20680HT_WHOAMI_VALUE: c_uint = 0xFA;
// scan element definition for generic MPU6xxx devices
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_scan {
    INV_MPU6050_SCAN_ACCL_X,
    INV_MPU6050_SCAN_ACCL_Y,
    INV_MPU6050_SCAN_ACCL_Z,
    INV_MPU6050_SCAN_TEMP,
    INV_MPU6050_SCAN_GYRO_X,
    INV_MPU6050_SCAN_GYRO_Y,
    INV_MPU6050_SCAN_GYRO_Z,
    INV_MPU6050_SCAN_TIMESTAMP,

    INV_MPU9X50_SCAN_MAGN_X = INV_MPU6050_SCAN_GYRO_Z + 1,
    INV_MPU9X50_SCAN_MAGN_Y,
    INV_MPU9X50_SCAN_MAGN_Z,
    INV_MPU9X50_SCAN_TIMESTAMP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_filter_e {
    INV_MPU6050_FILTER_NOLPF2 = 0,
    INV_MPU6050_FILTER_200HZ,
    INV_MPU6050_FILTER_100HZ,
    INV_MPU6050_FILTER_45HZ,
    INV_MPU6050_FILTER_20HZ,
    INV_MPU6050_FILTER_10HZ,
    INV_MPU6050_FILTER_5HZ,
    INV_MPU6050_FILTER_NOLPF,
    NUM_MPU6050_FILTER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_lposc_e {
    INV_MPU6050_LPOSC_4HZ = 4,
    INV_MPU6050_LPOSC_8HZ,
    INV_MPU6050_LPOSC_16HZ,
    INV_MPU6050_LPOSC_31HZ,
    INV_MPU6050_LPOSC_62HZ,
    INV_MPU6050_LPOSC_125HZ,
    INV_MPU6050_LPOSC_250HZ,
    INV_MPU6050_LPOSC_500HZ,
    NUM_MPU6050_LPOSC,
}

// IIO attribute address
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum INV_MPU6050_IIO_ATTR_ADDR {
    ATTR_GYRO_MATRIX,
    ATTR_ACCL_MATRIX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_accl_fs_e {
    INV_MPU6050_FS_02G = 0,
    INV_MPU6050_FS_04G,
    INV_MPU6050_FS_08G,
    INV_MPU6050_FS_16G,
    NUM_ACCL_FSR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_fsr_e {
    INV_MPU6050_FSR_250DPS = 0,
    INV_MPU6050_FSR_500DPS,
    INV_MPU6050_FSR_1000DPS,
    INV_MPU6050_FSR_2000DPS,
    NUM_MPU6050_FSR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inv_mpu6050_clock_sel_e {
    INV_CLK_INTERNAL = 0,
    INV_CLK_PLL,
    NUM_CLK
}

extern "C" {
    pub fn inv_mpu6050_read_fifo(irq: c_int, p: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn inv_mpu6050_probe_trigger(indio_dev: *mut iio_dev, irq_type: c_int) -> c_int;
}
extern "C" {
    pub fn inv_mpu6050_prepare_fifo(st: *mut inv_mpu6050_state, enable: bool) -> c_int;
}
extern "C" {
    pub fn inv_mpu_acpi_create_mux_client(client: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn inv_mpu_acpi_delete_mux_client(client: *mut i2c_client);
}
