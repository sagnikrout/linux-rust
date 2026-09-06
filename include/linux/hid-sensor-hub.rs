//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid-sensor-hub.h
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

//
// struct hid_sensor_hub_attribute_info - Attribute info
// @usage_id:		Parent usage id of a physical device.
// @attrib_id:		Attribute id for this attribute.
// @report_id:		Report id in which this information resides.
// @index:		Field index in the report.
// @units:		Measurement unit for this attribute.
// @unit_expo:		Exponent used in the data.
// @size:		Size in bytes for data size.
// @logical_minimum:	Logical minimum value for this attribute.
// @logical_maximum:	Logical maximum value for this attribute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_sensor_hub_attribute_info {
    pub usage_id: u32,
    pub attrib_id: u32,
    pub report_id: i32,
    pub index: i32,
    pub units: i32,
    pub unit_expo: i32,
    pub size: i32,
    pub logical_minimum: i32,
    pub logical_maximum: i32,
}

//
// struct sensor_hub_pending - Synchronous read pending information
// @status:		Pending status true/false.
// @ready:		Completion synchronization data.
// @usage_id:		Usage id for physical device, e.g. gyro usage id.
// @attr_usage_id:	Usage Id of a field, e.g. X-axis for a gyro.
// @raw_size:		Response size for a read request.
// @raw_data:		Place holder for received response.
// @index:		Current write index into raw_data for multi-byte reads.
// @max_raw_size:	Total buffer size for multi-byte reads; 0 for single-value reads.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sensor_hub_pending {
    pub status: bool,
    pub ready: completion,
    pub usage_id: u32,
    pub attr_usage_id: u32,
    pub raw_size: c_int,
    pub raw_data: *mut u8,
    pub index: u32,
    pub max_raw_size: u32,
}

//
// struct hid_sensor_hub_device - Stores the hub instance data
// @hdev:		Stores the hid instance.
// @vendor_id:		Vendor id of hub device.
// @product_id:		Product id of hub device.
// @usage:		Usage id for this hub device instance.
// @start_collection_index: Starting index for a phy type collection
// @end_collection_index: Last index for a phy type collection
// @mutex_ptr:		synchronizing mutex pointer.
// @pending:		Holds information of pending sync read request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_sensor_hub_device {
    pub hdev: *mut hid_device,
    pub vendor_id: u32,
    pub product_id: u32,
    pub usage: u32,
    pub start_collection_index: c_int,
    pub end_collection_index: c_int,
    pub mutex_ptr: *mut mutex,
    pub pending: sensor_hub_pending,
}

//
// struct hid_sensor_hub_callbacks - Client callback functions
// @pdev:		Platform device instance of the client driver.
// @suspend:		Suspend callback.
// @resume:		Resume callback.
// @capture_sample:	Callback to get a sample.
// @send_event:		Send notification to indicate all samples are
// captured, process and send event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_sensor_hub_callbacks {
    pub pdev: *mut platform_device,
    pub priv): *mut *mut *mut int (suspend)(struct hid_sensor_hub_device hsdev, void,
    pub priv): *mut *mut *mut int (resume)(struct hid_sensor_hub_device hsdev, void,
    pub priv): *mut c_void,
    pub priv): *mut c_void,
}

//
// sensor_hub_device_open() - Open hub device
// @hsdev:	Hub device instance.
//
// Used to open hid device for sensor hub.
//
extern "C" {
    pub fn sensor_hub_device_open(hsdev: *mut hid_sensor_hub_device) -> c_int;
}
//
// sensor_hub_device_close() - Close hub device
// @hsdev:	Hub device instance.
//
// Used to close hid device for sensor hub.
//
extern "C" {
    pub fn sensor_hub_device_close(hsdev: *mut hid_sensor_hub_device);
}
// Registration functions
//
// sensor_hub_register_callback() - Register client callbacks
// @hsdev:	Hub device instance.
// @usage_id:	Usage id of the client (E.g. 0x200076 for Gyro).
// @usage_callback: Callback function storage
//
// Used to register callbacks by client processing drivers. Sensor
// hub core driver will call these callbacks to offload processing
// of data streams and notifications.
//
// sensor_hub_remove_callback() - Remove client callback
// @hsdev:	Hub device instance.
// @usage_id:	Usage id of the client (e.g. 0x200076 for gyro).
//
// Removes a previously registered callback for the given usage_id
// and hsdev. Once removed, the client will no longer receive data or
// event notifications.
//
// Hid sensor hub core interfaces
//
// sensor_hub_input_get_attribute_info() - Get an attribute information
// @hsdev:	Hub device instance.
// @type:	Type of this attribute, input/output/feature
// @usage_id:	Attribute usage id of parent physical device as per spec
// @attr_usage_id:	Attribute usage id as per spec
// @info:	return information about attribute after parsing report
//
// Parses report and returns the attribute information such as report id,
// field index, units and exponent etc.
//
// sensor_hub_input_attr_get_raw_value() - Synchronous read request
// @hsdev:	Hub device instance.
// @usage_id:	Attribute usage id of parent physical device as per spec
// @attr_usage_id:	Attribute usage id as per spec
// @report_id:	Report id to look for
// @flag:      Synchronous or asynchronous read
// @is_signed:   If true then fields < 32 bits will be sign-extended
//
// Issues a synchronous or asynchronous read request for an input attribute.
// Return: data up to 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sensor_hub_read_flags {
    SENSOR_HUB_SYNC,
    SENSOR_HUB_ASYNC,
}

//
// sensor_hub_input_attr_read_values() - Synchronous multi-byte read request
// @hsdev:		Hub device instance.
// @usage_id:		Attribute usage id of parent physical device as per spec
// @attr_usage_id:	Attribute usage id as per spec
// @report_id:		Report id to look for
// @flag:		Synchronous or asynchronous read
// @buffer_size:	Size of the buffer in bytes
// @buffer:		Buffer to store the read data
//
// Issues a synchronous or asynchronous read request for an input attribute,
// accumulating data into the provided buffer until it is full.
// Return: 0 on success, -ETIMEDOUT if the device did not respond, or a
// negative error code.
//
// sensor_hub_set_feature() - Feature set request
// @hsdev:	Hub device instance.
// @report_id:	Report id to look for
// @field_index:	Field index inside a report
// @buffer_size: size of the buffer
// @buffer:	buffer to use in the feature set
//
// Used to set a field in feature report. For example this can set polling
// interval, sensitivity, activate/deactivate state.
//
// sensor_hub_get_feature() - Feature get request
// @hsdev:	Hub device instance.
// @report_id:	Report id to look for
// @field_index:	Field index inside a report
// @buffer_size:	size of the buffer
// @buffer:	buffer to copy output
//
// Used to get a field in feature report. For example this can get polling
// interval, sensitivity, activate/deactivate state.
// Return: On success, it returns the number of bytes copied to buffer.
// On failure, it returns value < 0.
//
// hid-sensor-attributes
// Common hid sensor iio structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_sensor_common {
    pub hsdev: *mut hid_sensor_hub_device,
    pub pdev: *mut platform_device,
    pub usage_id: unsigned,
    pub data_ready: core::sync::atomic::AtomicI32,
    pub user_requested_state: core::sync::atomic::AtomicI32,
    pub runtime_pm_enable: core::sync::atomic::AtomicI32,
    pub poll_interval: c_int,
    pub raw_hystersis: c_int,
    pub latency_ms: c_int,
    pub trigger: *mut iio_trigger,
    pub timestamp_ns_scale: c_int,
    pub poll: hid_sensor_hub_attribute_info,
    pub report_state: hid_sensor_hub_attribute_info,
    pub power_state: hid_sensor_hub_attribute_info,
    pub sensitivity: hid_sensor_hub_attribute_info,
    pub sensitivity_rel: hid_sensor_hub_attribute_info,
    pub report_latency: hid_sensor_hub_attribute_info,
    pub work: work_struct,
}

// Convert from hid unit expo to regular exponent
extern "C" {
    pub fn hid_sensor_read_poll_value(st: *mut hid_sensor_common) -> i32;
}
extern "C" {
    pub fn hid_sensor_batch_mode_supported(st: *mut hid_sensor_common) -> bool;
}
extern "C" {
    pub fn hid_sensor_set_report_latency(st: *mut hid_sensor_common, latency: c_int) -> c_int;
}
extern "C" {
    pub fn hid_sensor_get_report_latency(st: *mut hid_sensor_common) -> c_int;
}
