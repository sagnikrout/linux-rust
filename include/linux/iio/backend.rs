//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/backend.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_data_type {
    IIO_BACKEND_TWOS_COMPLEMENT,
    IIO_BACKEND_OFFSET_BINARY,
    IIO_BACKEND_DATA_UNSIGNED,
    IIO_BACKEND_DATA_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_data_source {
    IIO_BACKEND_INTERNAL_CONTINUOUS_WAVE,
    IIO_BACKEND_EXTERNAL,
    IIO_BACKEND_INTERNAL_RAMP_16BIT,
    IIO_BACKEND_DATA_SOURCE_MAX
}

//
// IIO_BACKEND_EX_INFO - Helper for an IIO extended channel attribute
// @_name: Attribute name
// @_shared: Whether the attribute is shared between all channels
// @_what: Data private to the driver
//

//
// struct iio_backend_data_fmt - Backend data format
// @type: Data type.
// @sign_extend: Bool to tell if the data is sign extended.
// @enable: Enable/Disable the data format module. If disabled,
// not formatting will happen.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_backend_data_fmt {
    pub type: iio_backend_data_type,
    pub sign_extend: bool,
    pub enable: bool,
}

// vendor specific from 32
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_test_pattern {
    IIO_BACKEND_NO_TEST_PATTERN,
// modified prbs9
    IIO_BACKEND_ADI_PRBS_9A = 32,
// modified prbs23
    IIO_BACKEND_ADI_PRBS_23A,
    IIO_BACKEND_TEST_PATTERN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_sample_trigger {
    IIO_BACKEND_SAMPLE_TRIGGER_EDGE_FALLING,
    IIO_BACKEND_SAMPLE_TRIGGER_EDGE_RISING,
    IIO_BACKEND_SAMPLE_TRIGGER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_interface_type {
    IIO_BACKEND_INTERFACE_SERIAL_LVDS,
    IIO_BACKEND_INTERFACE_SERIAL_CMOS,
    IIO_BACKEND_INTERFACE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_filter_type {
    IIO_BACKEND_FILTER_TYPE_DISABLED,
    IIO_BACKEND_FILTER_TYPE_SINC1,
    IIO_BACKEND_FILTER_TYPE_SINC5,
    IIO_BACKEND_FILTER_TYPE_SINC5_PLUS_COMP,
    IIO_BACKEND_FILTER_TYPE_MAX
}

//
// enum iio_backend_capabilities - Backend capabilities
// Backend capabilities can be used by frontends to check if a given
// functionality is supported by the backend. This is useful for frontend
// devices which are expected to work with alternative backend
// implementations. Capabilities are loosely coupled with operations,
// meaning that a capability requires certain operations to be implemented
// by the backend. A capability might be mapped to a single operation or
// multiple operations.
//
// @IIO_BACKEND_CAP_CALIBRATION: Backend supports digital interface
// calibration. Calibration procedure is device specific.
// @IIO_BACKEND_CAP_BUFFER: Support for IIO buffer interface.
// @IIO_BACKEND_CAP_ENABLE: Backend can be explicitly enabled/disabled.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_backend_capabilities {
    IIO_BACKEND_CAP_CALIBRATION = BIT(0),
    IIO_BACKEND_CAP_BUFFER = BIT(1),
    IIO_BACKEND_CAP_ENABLE = BIT(2),
}

//
// struct iio_backend_ops - operations structure for an iio_backend
// @enable: Enable backend.
// @disable: Disable backend.
// @chan_enable: Enable one channel.
// @chan_disable: Disable one channel.
// @data_format_set: Configure the data format for a specific channel.
// @data_source_set: Configure the data source for a specific channel.
// @data_source_get: Data source getter for a specific channel.
// @set_sample_rate: Configure the sampling rate for a specific channel.
// @test_pattern_set: Configure a test pattern.
// @chan_status: Get the channel status.
// @iodelay_set: Set digital I/O delay.
// @data_sample_trigger: Control when to sample data.
// @request_buffer: Request an IIO buffer.
// @free_buffer: Free an IIO buffer.
// @extend_chan_spec: Extend an IIO channel.
// @ext_info_set: Extended info setter.
// @ext_info_get: Extended info getter.
// @interface_type_get: Interface type.
// @data_size_set: Data size.
// @oversampling_ratio_set: Set Oversampling ratio.
// @read_raw: Read a channel attribute from a backend device
// @debugfs_print_chan_status: Print channel status into a buffer.
// @debugfs_reg_access: Read or write register value of backend.
// @filter_type_set: Set filter type.
// @interface_data_align: Perform the data alignment process.
// @num_lanes_set: Set the number of lanes enabled.
// @ddr_enable: Enable interface DDR (Double Data Rate) mode.
// @ddr_disable: Disable interface DDR (Double Data Rate) mode.
// @data_stream_enable: Enable data stream.
// @data_stream_disable: Disable data stream.
// @data_transfer_addr: Set data address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_backend_ops {
    pub back): *mut *mut int (enable)(struct iio_backend,
    pub back): *mut *mut void (disable)(struct iio_backend,
    pub chan): *mut *mut *mut int (chan_enable)(struct iio_backend back, unsigned int,
    pub chan): *mut *mut *mut int (chan_disable)(struct iio_backend back, unsigned int,
    pub data): *const iio_backend_data_fmt,
    pub data): iio_backend_data_source,
    pub data): *mut iio_backend_data_source,
    pub sample_rate_hz): u64,
    pub pattern): iio_backend_test_pattern,
    pub error): *mut bool,
    pub taps): c_uint,
    pub trigger): iio_backend_sample_trigger,
    pub indio_dev): *mut iio_dev,
    pub buffer): *mut iio_buffer,
    pub chan): *mut iio_chan_spec,
    pub len): *const *const char buf, size_t,
    pub buf): *const *const iio_chan_spec chan, char,
    pub type): *mut iio_backend_interface_type,
    pub size): *mut *mut *mut int (data_size_set)(struct iio_backend back, unsigned int,
    pub ratio): unsigned int chan, unsigned int,
    pub mask): c_long,
    pub len): usize,
    pub readval): *mut unsigned int writeval, unsigned int,
    pub type): iio_backend_filter_type,
    pub timeout_us): *mut *mut *mut int (interface_data_align)(struct iio_backend back, u32,
    pub num_lanes): *mut *mut *mut int (num_lanes_set)(struct iio_backend back, unsigned int,
    pub back): *mut *mut int (ddr_enable)(struct iio_backend,
    pub back): *mut *mut int (ddr_disable)(struct iio_backend,
    pub back): *mut *mut int (data_stream_enable)(struct iio_backend,
    pub back): *mut *mut int (data_stream_disable)(struct iio_backend,
    pub address): *mut *mut *mut int (data_transfer_addr)(struct iio_backend back, u32,
}

//
// struct iio_backend_info - info structure for an iio_backend
// @name: Backend name.
// @ops: Backend operations.
// @caps: Backend capabilities. (bitmask of enum iio_backend_capabilities).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_backend_info {
    pub name: *const c_char,
    pub ops: *const iio_backend_ops,
    pub caps: u32,
}

extern "C" {
    pub fn iio_backend_chan_enable(back: *mut iio_backend, chan: c_uint) -> c_int;
}
extern "C" {
    pub fn iio_backend_chan_disable(back: *mut iio_backend, chan: c_uint) -> c_int;
}
extern "C" {
    pub fn devm_iio_backend_enable(dev: *mut device, back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_enable(back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_disable(back: *mut iio_backend);
}
extern "C" {
    pub fn iio_backend_interface_data_align(back: *mut iio_backend, timeout_us: u32) -> c_int;
}
extern "C" {
    pub fn iio_backend_num_lanes_set(back: *mut iio_backend, num_lanes: c_uint) -> c_int;
}
extern "C" {
    pub fn iio_backend_ddr_enable(back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_ddr_disable(back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_data_stream_enable(back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_data_stream_disable(back: *mut iio_backend) -> c_int;
}
extern "C" {
    pub fn iio_backend_data_transfer_addr(back: *mut iio_backend, address: u32) -> c_int;
}
extern "C" {
    pub fn iio_backend_data_size_set(back: *mut iio_backend, size: c_uint) -> c_int;
}
extern "C" {
    pub fn iio_backend_has_caps(back: *mut iio_backend, caps: u32) -> bool;
}
extern "C" {
    pub fn iio_backend_read_raw(_arg: back, _arg: chan, _arg: val, _arg: val2, _arg: IIO_CHAN_INFO_SCALE) -> return;
}
