//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/bmp280.h
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

// BMP580 specific registers
pub const BMP580_REG_CMD: c_uint = 0x7E;
pub const BMP580_REG_EFF_OSR: c_uint = 0x38;
pub const BMP580_REG_ODR_CONFIG: c_uint = 0x37;
pub const BMP580_REG_OSR_CONFIG: c_uint = 0x36;
pub const BMP580_REG_IF_CONFIG: c_uint = 0x13;
pub const BMP580_REG_REV_ID: c_uint = 0x02;
pub const BMP580_REG_CHIP_ID: c_uint = 0x01;
// OOR allows to configure a pressure alarm
pub const BMP580_REG_OOR_CONFIG: c_uint = 0x35;
pub const BMP580_REG_OOR_RANGE: c_uint = 0x34;
pub const BMP580_REG_OOR_THR_MSB: c_uint = 0x33;
pub const BMP580_REG_OOR_THR_LSB: c_uint = 0x32;
// DSP registers (IIR filters)
pub const BMP580_REG_DSP_IIR: c_uint = 0x31;
pub const BMP580_REG_DSP_CONFIG: c_uint = 0x30;
// NVM access registers
pub const BMP580_REG_NVM_DATA_MSB: c_uint = 0x2D;
pub const BMP580_REG_NVM_DATA_LSB: c_uint = 0x2C;
pub const BMP580_REG_NVM_ADDR: c_uint = 0x2B;
// Status registers
pub const BMP580_REG_STATUS: c_uint = 0x28;
pub const BMP580_REG_INT_STATUS: c_uint = 0x27;
pub const BMP580_REG_CHIP_STATUS: c_uint = 0x11;
// Data registers
pub const BMP580_REG_FIFO_DATA: c_uint = 0x29;
pub const BMP580_REG_PRESS_MSB: c_uint = 0x22;
pub const BMP580_REG_PRESS_LSB: c_uint = 0x21;
pub const BMP580_REG_PRESS_XLSB: c_uint = 0x20;
pub const BMP580_REG_TEMP_MSB: c_uint = 0x1F;
pub const BMP580_REG_TEMP_LSB: c_uint = 0x1E;
pub const BMP580_REG_TEMP_XLSB: c_uint = 0x1D;
// FIFO config registers
pub const BMP580_REG_FIFO_SEL: c_uint = 0x18;
pub const BMP580_REG_FIFO_COUNT: c_uint = 0x17;
pub const BMP580_REG_FIFO_CONFIG: c_uint = 0x16;
// Interruptions config registers
pub const BMP580_REG_INT_SOURCE: c_uint = 0x15;
pub const BMP580_REG_INT_CONFIG: c_uint = 0x14;
pub const BMP580_CMD_NOOP: c_uint = 0x00;
pub const BMP580_CMD_EXTMODE_SEQ_0: c_uint = 0x73;
pub const BMP580_CMD_EXTMODE_SEQ_1: c_uint = 0xB4;
pub const BMP580_CMD_EXTMODE_SEQ_2: c_uint = 0x69;
pub const BMP580_CMD_NVM_OP_SEQ_0: c_uint = 0x5D;
pub const BMP580_CMD_NVM_READ_SEQ_1: c_uint = 0xA5;
pub const BMP580_CMD_NVM_WRITE_SEQ_1: c_uint = 0xA0;
pub const BMP580_CMD_SOFT_RESET: c_uint = 0xB6;

pub const BMP580_MODE_SLEEP: c_int = 0;
pub const BMP580_MODE_NORMAL: c_int = 1;
pub const BMP580_MODE_FORCED: c_int = 2;
pub const BMP580_MODE_CONTINOUS: c_int = 3;

pub const BMP580_DSP_COMP_DIS: c_int = 0;
pub const BMP580_DSP_TEMP_COMP_EN: c_int = 1;
//
// In section 7.27 of datasheet, modes 2 and 3 are technically the same.
// Pressure compensation means also enabling temperature compensation
//
pub const BMP580_DSP_PRESS_COMP_EN: c_int = 2;
pub const BMP580_DSP_PRESS_TEMP_COMP_EN: c_int = 3;

pub const BMP580_FILTER_OFF: c_int = 0;
pub const BMP580_FILTER_1X: c_int = 1;
pub const BMP580_FILTER_3X: c_int = 2;
pub const BMP580_FILTER_7X: c_int = 3;
pub const BMP580_FILTER_15X: c_int = 4;
pub const BMP580_FILTER_31X: c_int = 5;
pub const BMP580_FILTER_63X: c_int = 6;
pub const BMP580_FILTER_127X: c_int = 7;

pub const BMP580_TEMP_SKIPPED: c_uint = 0x7f7f7f;
pub const BMP580_PRESS_SKIPPED: c_uint = 0x7f7f7f;
// BMP380 specific registers
pub const BMP380_REG_CMD: c_uint = 0x7E;
pub const BMP380_REG_CONFIG: c_uint = 0x1F;
pub const BMP380_REG_ODR: c_uint = 0x1D;
pub const BMP380_REG_OSR: c_uint = 0x1C;
pub const BMP380_REG_POWER_CONTROL: c_uint = 0x1B;
pub const BMP380_REG_IF_CONFIG: c_uint = 0x1A;
pub const BMP380_REG_INT_CONTROL: c_uint = 0x19;
pub const BMP380_REG_INT_STATUS: c_uint = 0x11;
pub const BMP380_REG_EVENT: c_uint = 0x10;
pub const BMP380_REG_STATUS: c_uint = 0x03;
pub const BMP380_REG_ERROR: c_uint = 0x02;
pub const BMP380_REG_ID: c_uint = 0x00;
pub const BMP380_REG_FIFO_CONFIG_1: c_uint = 0x18;
pub const BMP380_REG_FIFO_CONFIG_2: c_uint = 0x17;
pub const BMP380_REG_FIFO_WATERMARK_MSB: c_uint = 0x16;
pub const BMP380_REG_FIFO_WATERMARK_LSB: c_uint = 0x15;
pub const BMP380_REG_FIFO_DATA: c_uint = 0x14;
pub const BMP380_REG_FIFO_LENGTH_MSB: c_uint = 0x13;
pub const BMP380_REG_FIFO_LENGTH_LSB: c_uint = 0x12;
pub const BMP380_REG_SENSOR_TIME_MSB: c_uint = 0x0E;
pub const BMP380_REG_SENSOR_TIME_LSB: c_uint = 0x0D;
pub const BMP380_REG_SENSOR_TIME_XLSB: c_uint = 0x0C;
pub const BMP380_REG_TEMP_MSB: c_uint = 0x09;
pub const BMP380_REG_TEMP_LSB: c_uint = 0x08;
pub const BMP380_REG_TEMP_XLSB: c_uint = 0x07;
pub const BMP380_REG_PRESS_MSB: c_uint = 0x06;
pub const BMP380_REG_PRESS_LSB: c_uint = 0x05;
pub const BMP380_REG_PRESS_XLSB: c_uint = 0x04;
pub const BMP380_REG_CALIB_TEMP_START: c_uint = 0x31;
pub const BMP380_CALIB_REG_COUNT: c_int = 21;

pub const BMP380_FILTER_OFF: c_int = 0;
pub const BMP380_FILTER_1X: c_int = 1;
pub const BMP380_FILTER_3X: c_int = 2;
pub const BMP380_FILTER_7X: c_int = 3;
pub const BMP380_FILTER_15X: c_int = 4;
pub const BMP380_FILTER_31X: c_int = 5;
pub const BMP380_FILTER_63X: c_int = 6;
pub const BMP380_FILTER_127X: c_int = 7;

pub const BMP380_MODE_SLEEP: c_int = 0;
pub const BMP380_MODE_FORCED: c_int = 1;
pub const BMP380_MODE_NORMAL: c_int = 3;
pub const BMP380_MEAS_OFFSET: c_int = 234;
pub const BMP380_MEAS_DUR: c_int = 2020;
pub const BMP380_TEMP_MEAS_OFFSET: c_int = 163;
pub const BMP380_PRESS_MEAS_OFFSET: c_int = 392;

pub const BMP380_MAX_TEMP: c_int = 8500;
pub const BMP380_MIN_PRES: c_int = 3000000;
pub const BMP380_MAX_PRES: c_int = 12500000;
pub const BMP380_CMD_NOOP: c_uint = 0x00;
pub const BMP380_CMD_EXTMODE_EN_MID: c_uint = 0x34;
pub const BMP380_CMD_FIFO_FLUSH: c_uint = 0xB0;
pub const BMP380_CMD_SOFT_RESET: c_uint = 0xB6;

pub const BMP380_TEMP_SKIPPED: c_uint = 0x800000;
pub const BMP380_PRESS_SKIPPED: c_uint = 0x800000;
// BMP280 specific registers
pub const BMP280_REG_TEMP_XLSB: c_uint = 0xFC;
pub const BMP280_REG_TEMP_LSB: c_uint = 0xFB;
pub const BMP280_REG_TEMP_MSB: c_uint = 0xFA;
pub const BMP280_REG_PRESS_XLSB: c_uint = 0xF9;
pub const BMP280_REG_PRESS_LSB: c_uint = 0xF8;
pub const BMP280_REG_PRESS_MSB: c_uint = 0xF7;
// Helper mask to truncate excess 4 bits on pressure and temp readings

pub const BMP280_REG_CONFIG: c_uint = 0xF5;
pub const BMP280_REG_CTRL_MEAS: c_uint = 0xF4;
pub const BMP280_REG_STATUS: c_uint = 0xF3;

pub const BMP280_REG_RESET: c_uint = 0xE0;
pub const BMP280_RST_SOFT_CMD: c_uint = 0xB6;
pub const BMP280_REG_COMP_TEMP_START: c_uint = 0x88;
pub const BMP280_COMP_TEMP_REG_COUNT: c_int = 6;
pub const BMP280_REG_COMP_PRESS_START: c_uint = 0x8E;
pub const BMP280_COMP_PRESS_REG_COUNT: c_int = 18;

pub const BMP280_FILTER_OFF: c_int = 0;
pub const BMP280_FILTER_2X: c_int = 1;
pub const BMP280_FILTER_4X: c_int = 2;
pub const BMP280_FILTER_8X: c_int = 3;
pub const BMP280_FILTER_16X: c_int = 4;

pub const BMP280_OSRS_TEMP_SKIP: c_int = 0;
pub const BMP280_OSRS_TEMP_1X: c_int = 1;
pub const BMP280_OSRS_TEMP_2X: c_int = 2;
pub const BMP280_OSRS_TEMP_4X: c_int = 3;
pub const BMP280_OSRS_TEMP_8X: c_int = 4;
pub const BMP280_OSRS_TEMP_16X: c_int = 5;

pub const BMP280_OSRS_PRESS_SKIP: c_int = 0;
pub const BMP280_OSRS_PRESS_1X: c_int = 1;
pub const BMP280_OSRS_PRESS_2X: c_int = 2;
pub const BMP280_OSRS_PRESS_4X: c_int = 3;
pub const BMP280_OSRS_PRESS_8X: c_int = 4;
pub const BMP280_OSRS_PRESS_16X: c_int = 5;

pub const BMP280_MODE_SLEEP: c_int = 0;
pub const BMP280_MODE_FORCED: c_int = 1;
pub const BMP280_MODE_NORMAL: c_int = 3;
pub const BMP280_MEAS_OFFSET: c_int = 1250;
pub const BMP280_MEAS_DUR: c_int = 2300;
pub const BMP280_PRESS_HUMID_MEAS_OFFSET: c_int = 575;
// BME280 specific registers
pub const BME280_REG_HUMIDITY_LSB: c_uint = 0xFE;
pub const BME280_REG_HUMIDITY_MSB: c_uint = 0xFD;
pub const BME280_REG_CTRL_HUMIDITY: c_uint = 0xF2;
// Due to non linear mapping, and data sizes we can't do a bulk read
pub const BME280_REG_COMP_H1: c_uint = 0xA1;
pub const BME280_REG_COMP_H2: c_uint = 0xE1;
pub const BME280_REG_COMP_H3: c_uint = 0xE3;
pub const BME280_REG_COMP_H4: c_uint = 0xE4;
pub const BME280_REG_COMP_H5: c_uint = 0xE5;
pub const BME280_REG_COMP_H6: c_uint = 0xE7;

pub const BME280_CONTIGUOUS_CALIB_REGS: c_int = 7;

pub const BME280_OSRS_HUMIDITY_SKIP: c_int = 0;
pub const BME280_OSRS_HUMIDITY_1X: c_int = 1;
pub const BME280_OSRS_HUMIDITY_2X: c_int = 2;
pub const BME280_OSRS_HUMIDITY_4X: c_int = 3;
pub const BME280_OSRS_HUMIDITY_8X: c_int = 4;
pub const BME280_OSRS_HUMIDITY_16X: c_int = 5;
// BMP180 specific registers
pub const BMP180_REG_OUT_XLSB: c_uint = 0xF8;
pub const BMP180_REG_OUT_LSB: c_uint = 0xF7;
pub const BMP180_REG_OUT_MSB: c_uint = 0xF6;
pub const BMP180_REG_CALIB_START: c_uint = 0xAA;
pub const BMP180_REG_CALIB_COUNT: c_int = 22;

pub const BMP180_MEAS_TEMP: c_uint = 0x0E;
pub const BMP180_MEAS_PRESS: c_uint = 0x14;

pub const BMP180_MEAS_PRESS_1X: c_int = 0;
pub const BMP180_MEAS_PRESS_2X: c_int = 1;
pub const BMP180_MEAS_PRESS_4X: c_int = 2;
pub const BMP180_MEAS_PRESS_8X: c_int = 3;
// BMP180 and BMP280 common registers
pub const BMP280_REG_CTRL_MEAS: c_uint = 0xF4;
pub const BMP280_REG_RESET: c_uint = 0xE0;
pub const BMP280_REG_ID: c_uint = 0xD0;
pub const BMP380_CHIP_ID: c_uint = 0x50;
pub const BMP580_CHIP_ID: c_uint = 0x50;
pub const BMP580_CHIP_ID_ALT: c_uint = 0x51;
pub const BMP180_CHIP_ID: c_uint = 0x55;
pub const BMP280_CHIP_ID: c_uint = 0x58;
pub const BMP390_CHIP_ID: c_uint = 0x60;
pub const BME280_CHIP_ID: c_uint = 0x60;
pub const BMP280_SOFT_RESET_VAL: c_uint = 0xB6;
// BMP280 register skipped special values
pub const BMP280_TEMP_SKIPPED: c_uint = 0x80000;
pub const BMP280_PRESS_SKIPPED: c_uint = 0x80000;
pub const BMP280_HUMIDITY_SKIPPED: c_uint = 0x8000;
// Number of bytes for each value
pub const BMP280_NUM_PRESS_BYTES: c_int = 3;
pub const BMP280_NUM_TEMP_BYTES: c_int = 3;
pub const BME280_NUM_HUMIDITY_BYTES: c_int = 2;

// Core exported structs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp180_calib {
    pub AC1: i16,
    pub AC2: i16,
    pub AC3: i16,
    pub AC4: u16,
    pub AC5: u16,
    pub AC6: u16,
    pub B1: i16,
    pub B2: i16,
    pub MB: i16,
    pub MC: i16,
    pub MD: i16,
}

// See datasheet Section 4.2.2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp280_calib {
    pub T1: u16,
    pub T2: i16,
    pub T3: i16,
    pub P1: u16,
    pub P2: i16,
    pub P3: i16,
    pub P4: i16,
    pub P5: i16,
    pub P6: i16,
    pub P7: i16,
    pub P8: i16,
    pub P9: i16,
    pub H1: u8,
    pub H2: i16,
    pub H3: u8,
    pub H4: i16,
    pub H5: i16,
    pub H6: i8,
}

// See datasheet Section 3.11.1.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp380_calib {
    pub T1: u16,
    pub T2: u16,
    pub T3: i8,
    pub P1: i16,
    pub P2: i16,
    pub P3: i8,
    pub P4: i8,
    pub P5: u16,
    pub P6: u16,
    pub P7: i8,
    pub P8: i8,
    pub P9: i16,
    pub P10: i8,
    pub P11: i8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmp280_op_mode {
    BMP280_SLEEP,
    BMP280_FORCED,
    BMP280_NORMAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp280_data {
    pub dev: *mut device,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub done: completion,
    pub use_eoc: bool,
    pub trig_open_drain: bool,
    pub trig_active_high: bool,
    pub trig: *mut iio_trigger,
    pub chip_info: *const bmp280_chip_info,
    pub bmp180: bmp180_calib,
    pub bmp280: bmp280_calib,
    pub bmp380: bmp380_calib,
    pub calib: },
    pub supplies: [regulator_bulk_data; BMP280_NUM_SUPPLIES],
    pub start_up_time_us: c_uint,
// log of base 2 of oversampling rate
    pub oversampling_press: u8,
    pub oversampling_temp: u8,
    pub oversampling_humid: u8,
    pub iir_filter_coeff: u8,
//
// BMP380 devices introduce sampling frequency configuration. See
// datasheet sections 3.3.3. and 4.3.19 for more details.
//
// BMx280 devices allowed indirect configuration of sampling frequency
// changing the t_standby duration between measurements, as detailed on
// section 3.6.3 of the datasheet.
//
    pub sampling_freq: c_int,
// Value to hold the current operation mode of the device
    pub op_mode: bmp280_op_mode,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
//
// Sensor data buffer
    pub buf: [u8; BME280_BURST_READ_BYTES],
// Calibration data buffers
    pub sizeof(__le16)]: __le16 bmp280_cal_buf[BMP280_CONTIGUOUS_CALIB_REGS /,
    pub sizeof(__be16)]: __be16 bmp180_cal_buf[BMP180_REG_CALIB_COUNT /,
    pub bme280_humid_cal_buf: [u8; BME280_CONTIGUOUS_CALIB_REGS],
    pub bmp380_cal_buf: [u8; BMP380_CALIB_REG_COUNT],
// Miscellaneous, endianness-aware data buffers
    pub le16: __le16,
    pub be16: __be16,
    pub __aligned(IIO_DMA_MINALIGN): },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmp280_chip_info {
    pub id_reg: c_uint,
    pub chip_id: *const u8,
    pub num_chip_id: c_int,
    pub regmap_config: *const regmap_config,
    pub spi_read_extra_byte: bool,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
    pub start_up_time_us: c_uint,
    pub avail_scan_masks: *const c_ulong,
    pub oversampling_temp_avail: *const c_int,
    pub num_oversampling_temp_avail: c_int,
    pub oversampling_temp_default: c_int,
    pub oversampling_press_avail: *const c_int,
    pub num_oversampling_press_avail: c_int,
    pub oversampling_press_default: c_int,
    pub oversampling_humid_avail: *const c_int,
    pub num_oversampling_humid_avail: c_int,
    pub oversampling_humid_default: c_int,
    pub iir_filter_coeffs_avail: *const c_int,
    pub num_iir_filter_coeffs_avail: c_int,
    pub iir_filter_coeff_default: c_int,
    pub (*sampling_freq_avail)[2]: *const c_int,
    pub num_sampling_freq_avail: c_int,
    pub sampling_freq_default: c_int,
    pub temp_coeffs: *const c_int,
    pub temp_coeffs_type: c_int,
    pub press_coeffs: *const c_int,
    pub press_coeffs_type: c_int,
    pub humid_coeffs: *const c_int,
    pub humid_coeffs_type: c_int,
    pub data): *mut *mut int (chip_config)(struct bmp280_data,
    pub adc_temp): *mut *mut *mut int (read_temp)(struct bmp280_data data, s32,
    pub adc_press): *mut *mut *mut int (read_press)(struct bmp280_data data, u32,
    pub adc_humidity): *mut *mut *mut int (read_humid)(struct bmp280_data data, u32,
    pub data): *mut *mut int (read_calib)(struct bmp280_data,
    pub data): *mut *mut int (preinit)(struct bmp280_data,
    pub mode): *mut *mut *mut int (set_mode)(struct bmp280_data data, enum bmp280_op_mode,
    pub data): *mut *mut int (wait_conv)(struct bmp280_data,
    pub indio_dev): *mut *mut int (trigger_probe)(struct iio_dev,
    pub p): *mut *mut irqreturn_t (trigger_handler)(int irq, void,
}

// Chip infos for each variant
// Regmap configurations
// Probe called from different transports
// PM ops
