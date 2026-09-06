//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/amd-sfh-hid/hid_descriptor/amd_sfh_hid_desc.h
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
// HID report descriptors, structures and routines
// Copyright 2020-2021 Advanced Micro Devices, Inc.
// Authors: Nehal Bakulchandra Shah <Nehal-bakulchandra.shah@amd.com>
// Sandeep Singh <Sandeep.singh@amd.com>
// Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum desc_type {
// Report descriptor name
    descr_size = 1,
    input_size,
    feature_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_feature_property {
// common properties
    pub report_id: u8,
    pub connection_type: u8,
    pub report_state: u8,
    pub power_state: u8,
    pub sensor_state: u8,
    pub report_interval: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_input_property {
// common properties
    pub report_id: u8,
    pub sensor_state: u8,
    pub event_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct accel3_feature_report {
    pub common_property: common_feature_property,
// properties specific to this sensor
    pub accel_change_sesnitivity: u16,
    pub accel_sensitivity_max: i16,
    pub accel_sensitivity_min: i16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct accel3_input_report {
    pub common_property: common_input_property,
// values specific to this sensor
    pub in_accel_x_value: c_int,
    pub in_accel_y_value: c_int,
    pub in_accel_z_value: c_int,
// include if required to support the "shake" event
    pub in_accel_shake_detection: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gyro_feature_report {
    pub common_property: common_feature_property,
// properties specific to this sensor
    pub gyro_change_sesnitivity: u16,
    pub gyro_sensitivity_max: i16,
    pub gyro_sensitivity_min: i16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gyro_input_report {
    pub common_property: common_input_property,
// values specific to this sensor
    pub in_angel_x_value: c_int,
    pub in_angel_y_value: c_int,
    pub in_angel_z_value: c_int,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct magno_feature_report {
    pub common_property: common_feature_property,
// properties specific to this sensor
    pub magno_headingchange_sensitivity: u16,
    pub heading_min: i16,
    pub heading_max: i16,
    pub flux_change_sensitivity: u16,
    pub flux_min: i16,
    pub flux_max: i16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct magno_input_report {
    pub common_property: common_input_property,
    pub in_magno_x: c_int,
    pub in_magno_y: c_int,
    pub in_magno_z: c_int,
    pub in_magno_accuracy: c_int,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct als_feature_report {
    pub common_property: common_feature_property,
// properties specific to this sensor
    pub als_change_sesnitivity: u16,
    pub als_sensitivity_max: i16,
    pub als_sensitivity_min: i16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct als_input_report {
    pub common_property: common_input_property,
// values specific to this sensor
    pub illuminance_value: c_int,
    pub light_color_temp: c_int,
    pub chromaticity_x_value: c_int,
    pub chromaticity_y_value: c_int,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_feature_report {
    pub common_property: common_feature_property,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_input_report {
    pub common_property: common_input_property,
// values specific to human presence sensor
    pub human_presence: u8,
    pub __packed: },
