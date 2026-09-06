//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/pmt/telemetry.h
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
// Telemetry types
pub const PMT_TELEM_TELEMETRY: c_int = 0;
pub const PMT_TELEM_CRASHLOG: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_header {
    pub access_type: u8,
    pub size: u16,
    pub guid: u32,
    pub base_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_endpoint_info {
    pub dev: *mut device,
    pub header: telem_header,
}

//
// pmt_telem_get_next_endpoint() - Get next device id for a telemetry endpoint
// @start:  starting devid to look from
//
// This functions can be used in a while loop predicate to retrieve the devid
// of all available telemetry endpoints. Functions pmt_telem_get_next_endpoint()
// and pmt_telem_register_endpoint() can be used inside of the loop to examine
// endpoint info and register to receive a pointer to the endpoint. The pointer
// is then usable in the telemetry read calls to access the telemetry data.
//
// Return:
// * devid       - devid of the next present endpoint from start
// * 0           - when no more endpoints are present after start
//
extern "C" {
    pub fn pmt_telem_get_next_endpoint(start: c_ulong) -> c_ulong;
}
//
// pmt_telem_register_endpoint() - Register a telemetry endpoint
// @devid: device id/handle of the telemetry endpoint
//
// Increments the kref usage counter for the endpoint.
//
// Return:
// * endpoint    - On success returns pointer to the telemetry endpoint
// * -ENXIO      - telemetry endpoint not found
//
// pmt_telem_unregister_endpoint() - Unregister a telemetry endpoint
// @ep:   ep structure to populate.
//
// Decrements the kref usage counter for the endpoint.
//
extern "C" {
    pub fn pmt_telem_unregister_endpoint(ep: *mut telem_endpoint);
}
//
// pmt_telem_get_endpoint_info() - Get info for an endpoint from its devid
// @devid:  device id/handle of the telemetry endpoint
// @info:   Endpoint info structure to be populated
//
// Return:
// * 0           - Success
// * -ENXIO      - telemetry endpoint not found for the devid
// * -EINVAL     - @info is NULL
//
extern "C" {
    pub fn pmt_telem_get_endpoint_info(devid: c_int, info: *mut telem_endpoint_info) -> c_int;
}
//
// pmt_telem_find_and_register_endpoint() - Get a telemetry endpoint from
// device, guid and pos
// @dev:    device inside the Intel vsec
// @guid:   GUID of the telemetry space
// @pos:    Instance of the guid
//
// Return:
// * endpoint    - On success returns pointer to the telemetry endpoint
// * -ENXIO      - telemetry endpoint not found
//
// pmt_telem_read() - Read qwords from counter sram using sample id
// @ep:     Telemetry endpoint to be read
// @id:     The beginning sample id of the metric(s) to be read
// @data:   Allocated qword buffer
// @count:  Number of qwords requested
//
// Callers must ensure reads are aligned. When the call returns -ENODEV,
// the device has been removed and callers should unregister the telemetry
// endpoint.
//
// Return:
// * 0           - Success
// * -ENODEV     - The device is not present.
// * -EINVAL     - The offset is out bounds
// * -EPIPE      - The device was removed during the read. Data written
// but should be considered invalid.
//
extern "C" {
    pub fn pmt_telem_read(ep: *mut telem_endpoint, id: u32, data: *mut u64, count: u32) -> c_int;
}
//
// pmt_telem_read32() - Read qwords from counter sram using sample id
// @ep:     Telemetry endpoint to be read
// @id:     The beginning sample id of the metric(s) to be read
// @data:   Allocated dword buffer
// @count:  Number of dwords requested
//
// Callers must ensure reads are aligned. When the call returns -ENODEV,
// the device has been removed and callers should unregister the telemetry
// endpoint.
//
// Return:
// * 0           - Success
// * -ENODEV     - The device is not present.
// * -EINVAL     - The offset is out bounds
// * -EPIPE      - The device was removed during the read. Data written
// but should be considered invalid.
//
extern "C" {
    pub fn pmt_telem_read32(ep: *mut telem_endpoint, id: u32, data: *mut u32, count: u32) -> c_int;
}
