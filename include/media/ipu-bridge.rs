//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/ipu-bridge.h
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
// Author: Dan Scally <djrscally@gmail.com>

pub const IPU_MAX_LANES: c_int = 4;
pub const IPU_MAX_PORTS: c_int = 4;
pub const MAX_NUM_LINK_FREQS: c_int = 3;
// Values are educated guesses as we don't have a spec
pub const IPU_SENSOR_ROTATION_NORMAL: c_int = 0;
pub const IPU_SENSOR_ROTATION_INVERTED: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu_sensor_swnodes {
    SWNODE_SENSOR_HID,
    SWNODE_SENSOR_PORT,
    SWNODE_SENSOR_ENDPOINT,
    SWNODE_IPU_PORT,
    SWNODE_IPU_ENDPOINT,
// below are optional / maybe empty
    SWNODE_IVSC_HID,
    SWNODE_IVSC_SENSOR_PORT,
    SWNODE_IVSC_SENSOR_ENDPOINT,
    SWNODE_IVSC_IPU_PORT,
    SWNODE_IVSC_IPU_ENDPOINT,
    SWNODE_VCM,
    SWNODE_COUNT
}

// Data representation as it is in ACPI SSDB buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_sensor_ssdb {
    pub version: u8,
    pub sku: u8,
    pub guid_csi2: [u8; 16],
    pub devfunction: u8,
    pub bus: u8,
    pub dphylinkenfuses: u32,
    pub clockdiv: u32,
    pub link: u8,
    pub lanes: u8,
    pub csiparams: [u32; 10],
    pub maxlanespeed: u32,
    pub sensorcalibfileidx: u8,
    pub sensorcalibfileidxInMBZ: [u8; 3],
    pub romtype: u8,
    pub vcmtype: u8,
    pub platforminfo: u8,
    pub platformsubinfo: u8,
    pub flash: u8,
    pub privacyled: u8,
    pub degree: u8,
    pub mipilinkdefined: u8,
    pub mclkspeed: u32,
    pub controllogicid: u8,
    pub reserved1: [u8; 3],
    pub mclkport: u8,
    pub reserved2: [u8; 13],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_property_names {
    pub clock_frequency: [c_char; 16],
    pub rotation: [c_char; 9],
    pub orientation: [c_char; 12],
    pub bus_type: [c_char; 9],
    pub data_lanes: [c_char; 11],
    pub remote_endpoint: [c_char; 16],
    pub link_frequencies: [c_char; 17],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_node_names {
    pub port: [c_char; 7],
    pub ivsc_sensor_port: [c_char; 7],
    pub ivsc_ipu_port: [c_char; 7],
    pub endpoint: [c_char; 11],
    pub remote_port: [c_char; 9],
    pub vcm: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_sensor_config {
    pub hid: *const c_char,
    pub nr_link_freqs: u8,
    pub link_freqs: [u64; MAX_NUM_LINK_FREQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_sensor {
// append ssdb.link(u8) in "-%u" format as suffix of HID
    pub 4]: char name[ACPI_ID_LEN +,
    pub adev: *mut acpi_device,
    pub csi_dev: *mut device,
    pub ivsc_adev: *mut acpi_device,
    pub 4]: char ivsc_name[ACPI_ID_LEN +,
// SWNODE_COUNT + 1 for terminating NULL
    pub 1]: *const *const software_node group[SWNODE_COUNT +,
    pub swnodes: [software_node; SWNODE_COUNT],
    pub node_names: ipu_node_names,
    pub link: u8,
    pub lanes: u8,
    pub mclkspeed: u32,
    pub rotation: u32,
    pub orientation: v4l2_fwnode_orientation,
    pub vcm_type: *const c_char,
    pub prop_names: ipu_property_names,
    pub ep_properties: [property_entry; 5],
    pub dev_properties: [property_entry; 5],
    pub ipu_properties: [property_entry; 3],
    pub ivsc_properties: [property_entry; 1],
    pub ivsc_sensor_ep_properties: [property_entry; 4],
    pub ivsc_ipu_ep_properties: [property_entry; 4],
    pub local_ref: [software_node_ref_args; 1],
    pub remote_ref: [software_node_ref_args; 1],
    pub vcm_ref: [software_node_ref_args; 1],
    pub ivsc_sensor_ref: [software_node_ref_args; 1],
    pub ivsc_ipu_ref: [software_node_ref_args; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_bridge {
    pub dev: *mut device,
    pub parse_sensor_fwnode: ipu_parse_sensor_fwnode_t,
    pub ipu_node_name: [c_char; ACPI_ID_LEN],
    pub ipu_hid_node: software_node,
    pub data_lanes: [u32; 4],
    pub n_sensors: c_uint,
    pub sensors: [ipu_sensor; IPU_MAX_PORTS],
}

extern "C" {
    pub fn ipu_bridge_parse_ssdb(adev: *mut acpi_device, sensor: *mut ipu_sensor) -> c_int;
}
extern "C" {
    pub fn ipu_bridge_instantiate_vcm(sensor: *mut device) -> c_int;
}

// Use a define to avoid the @parse_sensor_fwnode argument getting evaluated

