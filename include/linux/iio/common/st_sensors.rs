//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/common/st_sensors.h
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
// STMicroelectronics sensors library driver
//
// Copyright 2012-2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

//
// Buffer size max case: 2bytes per channel, 3 channels in total +
// 8bytes timestamp channel (s64)
//

pub const ST_SENSORS_ODR_LIST_MAX: c_int = 10;
pub const ST_SENSORS_FULLSCALE_AVL_MAX: c_int = 10;
pub const ST_SENSORS_NUMBER_ALL_CHANNELS: c_int = 4;
pub const ST_SENSORS_ENABLE_ALL_AXIS: c_uint = 0x07;
pub const ST_SENSORS_SCAN_X: c_int = 0;
pub const ST_SENSORS_SCAN_Y: c_int = 1;
pub const ST_SENSORS_SCAN_Z: c_int = 2;
pub const ST_SENSORS_DEFAULT_POWER_ON_VALUE: c_uint = 0x01;
pub const ST_SENSORS_DEFAULT_POWER_OFF_VALUE: c_uint = 0x00;
pub const ST_SENSORS_DEFAULT_WAI_ADDRESS: c_uint = 0x0f;
pub const ST_SENSORS_DEFAULT_AXIS_ADDR: c_uint = 0x20;
pub const ST_SENSORS_DEFAULT_AXIS_MASK: c_uint = 0x07;
pub const ST_SENSORS_DEFAULT_AXIS_N_BIT: c_int = 3;
pub const ST_SENSORS_DEFAULT_STAT_ADDR: c_uint = 0x27;
pub const ST_SENSORS_MAX_NAME: c_int = 17;
pub const ST_SENSORS_MAX_4WAI: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_odr_avl {
    pub hz: c_uint,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_odr {
    pub addr: u8,
    pub mask: u8,
    pub odr_avl: [st_sensor_odr_avl; ST_SENSORS_ODR_LIST_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_power {
    pub addr: u8,
    pub mask: u8,
    pub value_off: u8,
    pub value_on: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_axis {
    pub addr: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_fullscale_avl {
    pub num: c_uint,
    pub value: u8,
    pub gain: c_uint,
    pub gain2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_fullscale {
    pub addr: u8,
    pub mask: u8,
    pub fs_avl: [st_sensor_fullscale_avl; ST_SENSORS_FULLSCALE_AVL_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_sim {
    pub addr: u8,
    pub value: u8,
}

//
// struct st_sensor_bdu - ST sensor device block data update
// @addr: address of the register.
// @mask: mask to write the block data update flag.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_bdu {
    pub addr: u8,
    pub mask: u8,
}

//
// struct st_sensor_das - ST sensor device data alignment selection
// @addr: address of the register.
// @mask: mask to write the das flag for left alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_das {
    pub addr: u8,
    pub mask: u8,
}

//
// struct st_sensor_int_drdy - ST sensor device drdy line parameters
// @addr: address of INT drdy register.
// @mask: mask to enable drdy line.
// @addr_od: address to enable/disable Open Drain on the INT line.
// @mask_od: mask to enable/disable Open Drain on the INT line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_int_drdy {
    pub addr: u8,
    pub mask: u8,
    pub addr_od: u8,
    pub mask_od: u8,
}

//
// struct st_sensor_data_ready_irq - ST sensor device data-ready interrupt
// @int1: data-ready configuration register for INT1 pin.
// @int2: data-ready configuration register for INT2 pin.
// @addr_ihl: address to enable/disable active low on the INT lines.
// @mask_ihl: mask to enable/disable active low on the INT lines.
// @stat_drdy: status register of DRDY (data ready) interrupt.
// @ig1: represents the Interrupt Generator 1 of sensors.
// @en_addr: address of the enable ig1 register.
// @en_mask: mask to write the on/off value for enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_data_ready_irq {
    pub int1: st_sensor_int_drdy,
    pub int2: st_sensor_int_drdy,
    pub addr_ihl: u8,
    pub mask_ihl: u8,
    pub addr: u8,
    pub mask: u8,
    pub stat_drdy: },
    pub en_addr: u8,
    pub en_mask: u8,
    pub ig1: },
}

//
// struct st_sensor_settings - ST specific sensor settings
// @wai: Contents of WhoAmI register.
// @wai_addr: The address of WhoAmI register.
// @sensors_supported: List of supported sensors by struct itself.
// @ch: IIO channels for the sensor.
// @num_ch: Number of IIO channels in @ch
// @odr: Output data rate register and ODR list available.
// @pw: Power register of the sensor.
// @enable_axis: Enable one or more axis of the sensor.
// @fs: Full scale register and full scale list available.
// @bdu: Block data update register.
// @das: Data Alignment Selection register.
// @drdy_irq: Data ready register of the sensor.
// @sim: SPI serial interface mode register of the sensor.
// @multi_read_bit: Use or not particular bit for [I2C/SPI] multi-read.
// @bootime: samples to discard when sensor passing from power-down to power-up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_settings {
    pub wai: u8,
    pub wai_addr: u8,
    pub sensors_supported: [c_char; ST_SENSORS_MAX_4WAI][ST_SENSORS_MAX_NAME],
    pub ch: *mut iio_chan_spec,
    pub num_ch: c_int,
    pub odr: st_sensor_odr,
    pub pw: st_sensor_power,
    pub enable_axis: st_sensor_axis,
    pub fs: st_sensor_fullscale,
    pub bdu: st_sensor_bdu,
    pub das: st_sensor_das,
    pub drdy_irq: st_sensor_data_ready_irq,
    pub sim: st_sensor_sim,
    pub multi_read_bit: bool,
    pub bootime: c_uint,
}

//
// struct st_sensor_data - ST sensor device status
// @trig: The trigger in use by the core driver.
// @mount_matrix: The mounting matrix of the sensor.
// @sensor_settings: Pointer to the specific sensor settings in use.
// @current_fullscale: Maximum range of measure by the sensor.
// @regmap: Pointer to specific sensor regmap configuration.
// @enabled: Status of the sensor (false->off, true->on).
// @odr: Output data rate of the sensor [Hz].
// @num_data_channels: Number of data channels used in buffer.
// @drdy_int_pin: Redirect DRDY on pin 1 (1) or pin 2 (2).
// @int_pin_open_drain: Set the interrupt/DRDY to open drain.
// @irq: the IRQ number.
// @edge_irq: the IRQ triggers on edges and need special handling.
// @hw_irq_trigger: if we're using the hardware interrupt on the sensor.
// @hw_timestamp: Latest timestamp from the interrupt handler, when in use.
// @buffer_data: Data used by buffer part.
// @odr_lock: Local lock for preventing concurrent ODR accesses/changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_sensor_data {
    pub trig: *mut iio_trigger,
    pub mount_matrix: iio_mount_matrix,
    pub sensor_settings: *mut st_sensor_settings,
    pub current_fullscale: *mut st_sensor_fullscale_avl,
    pub regmap: *mut regmap,
    pub enabled: bool,
    pub odr: c_uint,
    pub num_data_channels: c_uint,
    pub drdy_int_pin: u8,
    pub int_pin_open_drain: bool,
    pub irq: c_int,
    pub edge_irq: bool,
    pub hw_irq_trigger: bool,
    pub hw_timestamp: i64,
    pub odr_lock: mutex,
    pub __aligned(IIO_DMA_MINALIGN): char buffer_data[ST_SENSORS_MAX_BUFFER_SIZE],
}

extern "C" {
    pub fn st_sensors_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn st_sensors_set_enable(indio_dev: *mut iio_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_sensors_set_axis_enable(indio_dev: *mut iio_dev, axis_enable: u8) -> c_int;
}
extern "C" {
    pub fn st_sensors_power_enable(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn st_sensors_set_odr(indio_dev: *mut iio_dev, odr: c_uint) -> c_int;
}
extern "C" {
    pub fn st_sensors_set_dataready_irq(indio_dev: *mut iio_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_sensors_set_fullscale_by_gain(indio_dev: *mut iio_dev, scale: c_int) -> c_int;
}
extern "C" {
    pub fn st_sensors_verify_id(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn st_sensors_dev_name_probe(dev: *mut device, name: *mut c_char, len: c_int);
}
// Accelerometer
extern "C" {
    pub fn st_accel_common_probe(indio_dev: *mut iio_dev) -> c_int;
}
// Gyroscope
extern "C" {
    pub fn st_gyro_common_probe(indio_dev: *mut iio_dev) -> c_int;
}
// Magnetometer
extern "C" {
    pub fn st_magn_common_probe(indio_dev: *mut iio_dev) -> c_int;
}
// Pressure
extern "C" {
    pub fn st_press_common_probe(indio_dev: *mut iio_dev) -> c_int;
}
