//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/scpi_protocol.h
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
// SCPI Message Protocol driver header
//
// Copyright (C) 2014 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_opp {
    pub freq: u32,
    pub m_volt: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_dvfs_info {
    pub count: c_uint,
    pub /: *mut *mut unsigned int latency; / in nanoseconds,
    pub opps: *mut scpi_opp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scpi_sensor_class {
    TEMPERATURE,
    VOLTAGE,
    CURRENT,
    POWER,
    ENERGY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_sensor_info {
    pub sensor_id: u16,
    pub class: u8,
    pub trigger_type: u8,
    pub name: [c_char; 20],
    pub __packed: },
//
// struct scpi_ops - represents the various operations provided
// by SCP through SCPI message protocol
// @get_version: returns the major and minor revision on the SCPI
// message protocol
// @clk_get_range: gets clock range limit(min - max in Hz)
// @clk_get_val: gets clock value(in Hz)
// @clk_set_val: sets the clock value, setting to 0 will disable the
// clock (if supported)
// @dvfs_get_idx: gets the Operating Point of the given power domain.
// OPP is an index to the list return by @dvfs_get_info
// @dvfs_set_idx: sets the Operating Point of the given power domain.
// OPP is an index to the list return by @dvfs_get_info
// @dvfs_get_info: returns the DVFS capabilities of the given power
// domain. It includes the OPP list and the latency information
// @device_domain_id: gets the scpi domain id for a given device
// @get_transition_latency: gets the DVFS transition latency for a given device
// @add_opps_to_device: adds all the OPPs for a given device
// @sensor_get_capability: get the list of capabilities for the sensors
// @sensor_get_info: get the information of the specified sensor
// @sensor_get_value: gets the current value of the sensor
// @device_get_power_state: gets the power state of a power domain
// @device_set_power_state: sets the power state of a power domain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scpi_ops {
    pub (*get_version)(void): *mut u32,
    pub ): *mut *mut *mut int (clk_get_range)(u16, unsigned long , unsigned long,
    pub (*clk_get_val)(u16): *mut c_ulong,
    pub long): *mut *mut int (clk_set_val)(u16, unsigned,
    pub (*dvfs_get_idx)(u8): *mut c_int,
    pub u8): *mut *mut int (dvfs_set_idx)(u8,,
    pub (*dvfs_get_info)(u8): *mut scpi_dvfs_info,
    pub ): *mut *mut int (device_domain_id)(struct device,
    pub ): *mut *mut int (get_transition_latency)(struct device,
    pub ): *mut *mut int (add_opps_to_device)(struct device,
    pub sensors): *mut *mut int (sensor_get_capability)(u16,
    pub ): *mut *mut int (sensor_get_info)(u16 sensor_id, struct scpi_sensor_info,
    pub ): *mut *mut int (sensor_get_value)(u16, u64,
    pub (*device_get_power_state)(u16): *mut c_int,
    pub u8): *mut *mut int (device_set_power_state)(u16,,
}

