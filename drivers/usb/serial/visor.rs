//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/visor.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// USB HandSpring Visor driver
//
// Copyright (C) 1999 - 2003
// Greg Kroah-Hartman (greg@kroah.com)
//
// See Documentation/usb/usb-serial.rst for more information on using this
// driver.
//
pub const HANDSPRING_VENDOR_ID: c_uint = 0x082d;
pub const HANDSPRING_VISOR_ID: c_uint = 0x0100;
pub const HANDSPRING_TREO_ID: c_uint = 0x0200;
pub const HANDSPRING_TREO600_ID: c_uint = 0x0300;
pub const PALM_VENDOR_ID: c_uint = 0x0830;
pub const PALM_M500_ID: c_uint = 0x0001;
pub const PALM_M505_ID: c_uint = 0x0002;
pub const PALM_M515_ID: c_uint = 0x0003;
pub const PALM_I705_ID: c_uint = 0x0020;
pub const PALM_M125_ID: c_uint = 0x0040;
pub const PALM_M130_ID: c_uint = 0x0050;
pub const PALM_TUNGSTEN_T_ID: c_uint = 0x0060;
pub const PALM_TREO_650: c_uint = 0x0061;
pub const PALM_TUNGSTEN_Z_ID: c_uint = 0x0031;
pub const PALM_ZIRE_ID: c_uint = 0x0070;
pub const PALM_M100_ID: c_uint = 0x0080;
pub const GSPDA_VENDOR_ID: c_uint = 0x115e;
pub const GSPDA_XPLORE_M68_ID: c_uint = 0xf100;
pub const SONY_VENDOR_ID: c_uint = 0x054C;
pub const SONY_CLIE_3_5_ID: c_uint = 0x0038;
pub const SONY_CLIE_4_0_ID: c_uint = 0x0066;
pub const SONY_CLIE_S360_ID: c_uint = 0x0095;
pub const SONY_CLIE_4_1_ID: c_uint = 0x009A;
pub const SONY_CLIE_NX60_ID: c_uint = 0x00DA;
pub const SONY_CLIE_NZ90V_ID: c_uint = 0x00E9;
pub const SONY_CLIE_UX50_ID: c_uint = 0x0144;
pub const SONY_CLIE_TJ25_ID: c_uint = 0x0169;
pub const ACER_VENDOR_ID: c_uint = 0x0502;
pub const ACER_S10_ID: c_uint = 0x0001;
pub const SAMSUNG_VENDOR_ID: c_uint = 0x04E8;
pub const SAMSUNG_SCH_I330_ID: c_uint = 0x8001;
pub const SAMSUNG_SPH_I500_ID: c_uint = 0x6601;
pub const TAPWAVE_VENDOR_ID: c_uint = 0x12EF;
pub const TAPWAVE_ZODIAC_ID: c_uint = 0x0100;
pub const GARMIN_VENDOR_ID: c_uint = 0x091E;
pub const GARMIN_IQUE_3600_ID: c_uint = 0x0004;
pub const ACEECA_VENDOR_ID: c_uint = 0x4766;
pub const ACEECA_MEZ1000_ID: c_uint = 0x0001;
pub const KYOCERA_VENDOR_ID: c_uint = 0x0C88;
pub const KYOCERA_7135_ID: c_uint = 0x0021;
pub const FOSSIL_VENDOR_ID: c_uint = 0x0E67;
pub const FOSSIL_ABACUS_ID: c_uint = 0x0002;
//
// Handspring Visor Vendor specific request codes (bRequest values)
// A big thank you to Handspring for providing the following information.
// If anyone wants the original file where these values and structures came
// from, send email to <greg@kroah.com>.
//
// VISOR_REQUEST_BYTES_AVAILABLE asks the visor for the number of bytes that
// are available to be transferred to the host for the specified endpoint.
// Currently this is not used, and always returns 0x0001
//
pub const VISOR_REQUEST_BYTES_AVAILABLE: c_uint = 0x01;
//
// VISOR_CLOSE_NOTIFICATION is set to the device to notify it that the host
// is now closing the pipe. An empty packet is sent in response.
//
pub const VISOR_CLOSE_NOTIFICATION: c_uint = 0x02;
//
// VISOR_GET_CONNECTION_INFORMATION is sent by the host during enumeration to
// get the endpoints used by the connection.
//
pub const VISOR_GET_CONNECTION_INFORMATION: c_uint = 0x03;
//
// VISOR_GET_CONNECTION_INFORMATION returns data in the following format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct visor_connection_info {
    pub num_ports: __le16,
    pub port_function_id: __u8,
    pub port: __u8,
    pub connections: [}; 2],
}

// struct visor_connection_info.connection[x].port defines:
pub const VISOR_ENDPOINT_1: c_uint = 0x01;
pub const VISOR_ENDPOINT_2: c_uint = 0x02;
// struct visor_connection_info.connection[x].port_function_id defines:
pub const VISOR_FUNCTION_GENERIC: c_uint = 0x00;
pub const VISOR_FUNCTION_DEBUGGER: c_uint = 0x01;
pub const VISOR_FUNCTION_HOTSYNC: c_uint = 0x02;
pub const VISOR_FUNCTION_CONSOLE: c_uint = 0x03;
pub const VISOR_FUNCTION_REMOTE_FILE_SYS: c_uint = 0x04;
//
// PALM_GET_SOME_UNKNOWN_INFORMATION is sent by the host during enumeration to
// get some information from the M series devices, that is currently unknown.
//
pub const PALM_GET_EXT_CONNECTION_INFORMATION: c_uint = 0x04;
//
// struct palm_ext_connection_info - return data from a PALM_GET_EXT_CONNECTION_INFORMATION request
// @num_ports: maximum number of functions/connections in use
// @endpoint_numbers_different: will be 1 if in and out endpoints numbers are
// different, otherwise it is 0.  If value is 1, then
// connections.end_point_info is non-zero.  If value is 0, then
// connections.port contains the endpoint number, which is the same for in
// and out.
// @port_function_id: contains the creator id of the application that opened
// this connection.
// @port: contains the in/out endpoint number.  Is 0 if in and out endpoint
// numbers are different.
// @end_point_info: high nubbe is in endpoint and low nibble will indicate out
// endpoint.  Is 0 if in and out endpoints are the same.
//
// The maximum number of connections currently supported is 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct palm_ext_connection_info {
    pub num_ports: __u8,
    pub endpoint_numbers_different: __u8,
    pub reserved1: __le16,
    pub port_function_id: __u32,
    pub port: __u8,
    pub end_point_info: __u8,
    pub reserved: __le16,
    pub connections: [}; 2],
}
