//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pnp/pnpbios/pnpbios.h
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
// pnpbios.h - contains local definitions
//
// Include file for the interface to a PnP BIOS
//
// Original BIOS code (C) 1998 Christian Schmidt (chr.schmidt@tu-bs.de)
// PnP handler parts (c) 1998 Tom Lees <tom@lpsg.demon.co.uk>
// Minor reorganizations by David Hinds <dahinds@users.sourceforge.net>
//
// Return codes
//
pub const PNP_SUCCESS: c_uint = 0x00;
pub const PNP_NOT_SET_STATICALLY: c_uint = 0x7f;
pub const PNP_UNKNOWN_FUNCTION: c_uint = 0x81;
pub const PNP_FUNCTION_NOT_SUPPORTED: c_uint = 0x82;
pub const PNP_INVALID_HANDLE: c_uint = 0x83;
pub const PNP_BAD_PARAMETER: c_uint = 0x84;
pub const PNP_SET_FAILED: c_uint = 0x85;
pub const PNP_EVENTS_NOT_PENDING: c_uint = 0x86;
pub const PNP_SYSTEM_NOT_DOCKED: c_uint = 0x87;
pub const PNP_NO_ISA_PNP_CARDS: c_uint = 0x88;
pub const PNP_UNABLE_TO_DETERMINE_DOCK_CAPABILITIES: c_uint = 0x89;
pub const PNP_CONFIG_CHANGE_FAILED_NO_BATTERY: c_uint = 0x8a;
pub const PNP_CONFIG_CHANGE_FAILED_RESOURCE_CONFLICT: c_uint = 0x8b;
pub const PNP_BUFFER_TOO_SMALL: c_uint = 0x8c;
pub const PNP_USE_ESCD_SUPPORT: c_uint = 0x8d;
pub const PNP_MESSAGE_NOT_SUPPORTED: c_uint = 0x8e;
pub const PNP_HARDWARE_ERROR: c_uint = 0x8f;
pub const ESCD_SUCCESS: c_uint = 0x00;
pub const ESCD_IO_ERROR_READING: c_uint = 0x55;
pub const ESCD_INVALID: c_uint = 0x56;
pub const ESCD_BUFFER_TOO_SMALL: c_uint = 0x59;
pub const ESCD_NVRAM_TOO_SMALL: c_uint = 0x5a;
pub const ESCD_FUNCTION_NOT_SUPPORTED: c_uint = 0x81;
//
// Events that can be received by "get event"
//
pub const PNPEV_ABOUT_TO_CHANGE_CONFIG: c_uint = 0x0001;
pub const PNPEV_DOCK_CHANGED: c_uint = 0x0002;
pub const PNPEV_SYSTEM_DEVICE_CHANGED: c_uint = 0x0003;
pub const PNPEV_CONFIG_CHANGED_FAILED: c_uint = 0x0004;
pub const PNPEV_UNKNOWN_SYSTEM_EVENT: c_uint = 0xffff;
// 0x8000 through 0xfffe are OEM defined
//
// Messages that should be sent through "send message"
//
pub const PNPMSG_OK: c_uint = 0x00;
pub const PNPMSG_ABORT: c_uint = 0x01;
pub const PNPMSG_UNDOCK_DEFAULT_ACTION: c_uint = 0x40;
pub const PNPMSG_POWER_OFF: c_uint = 0x41;
pub const PNPMSG_PNP_OS_ACTIVE: c_uint = 0x42;
pub const PNPMSG_PNP_OS_INACTIVE: c_uint = 0x43;
//
// Plug and Play BIOS flags
//
pub const PNPBIOS_NO_DISABLE: c_uint = 0x0001;
pub const PNPBIOS_NO_CONFIG: c_uint = 0x0002;
pub const PNPBIOS_OUTPUT: c_uint = 0x0004;
pub const PNPBIOS_INPUT: c_uint = 0x0008;
pub const PNPBIOS_BOOTABLE: c_uint = 0x0010;
pub const PNPBIOS_DOCK: c_uint = 0x0020;
pub const PNPBIOS_REMOVABLE: c_uint = 0x0040;

//
// Function Parameters
//
pub const PNPMODE_STATIC: c_int = 1;
pub const PNPMODE_DYNAMIC: c_int = 0;
// 0x8000 through 0xffff are OEM defined

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_dev_node_info {
    pub no_nodes: __u16,
    pub max_node_size: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_docking_station_info {
    pub location_id: __u32,
    pub serial: __u32,
    pub capabilities: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_isa_config_struc {
    pub revision: __u8,
    pub no_csns: __u8,
    pub isa_rd_data_port: __u16,
    pub reserved: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct escd_info_struc {
    pub min_escd_write_size: __u16,
    pub escd_size: __u16,
    pub nv_storage_base: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_bios_node {
    pub size: __u16,
    pub handle: __u8,
    pub eisa_id: __u32,
    pub type_code: [__u8; 3],
    pub flags: __u16,
    pub data: [__u8; ],
}

// non-exported
extern "C" {
    pub fn pnp_bios_dev_node_info(data: *mut pnp_dev_node_info) -> c_int;
}
extern "C" {
    pub fn pnp_bios_get_stat_res(info: *mut c_char) -> c_int;
}
extern "C" {
    pub fn pnp_bios_isapnp_config(data: *mut pnp_isa_config_struc) -> c_int;
}
extern "C" {
    pub fn pnp_bios_escd_info(data: *mut escd_info_struc) -> c_int;
}
extern "C" {
    pub fn pnp_bios_read_escd(data: *mut c_char, nvram_base: u32) -> c_int;
}
extern "C" {
    pub fn pnp_bios_dock_station_info(data: *mut pnp_docking_station_info) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pnp_bios_install_struct {
    pub /: *mut *mut u32 signature; / "$PnP",
    pub /: *mut *mut u8 version; / in BCD,
    pub /: *mut *mut u8 length; / length in bytes, currently 21h,
    pub /: *mut *mut u16 control; / system capabilities,
    pub /: *mut *mut u8 checksum; / all bytes must add up to 0,
    pub /: *mut *mut u32 eventflag; / phys. address of the event flag,
    pub /: *mut *mut u16 rmoffset; / real mode entry point,
    pub rmcseg: u16,
    pub /: *mut *mut u16 pm16offset; / 16 bit protected mode entry,
    pub pm16cseg: u32,
    pub /: *mut *mut u32 deviceID; / EISA encoded system ID or 0,
    pub /: *mut *mut u16 rmdseg; / real mode data segment,
    pub /: *mut *mut u32 pm16dseg; / 16 bit pm data segment base,
    pub fields: },
    pub /: *mut *mut char chars[0x21]; / To calculate the checksum,
}

extern "C" {
    pub fn pnp_bios_present() -> c_int;
}
extern "C" {
    pub fn pnpbios_parse_data_stream(dev: *mut pnp_dev, node: *mut *mut pnp_bios_node) -> c_int;
}
extern "C" {
    pub fn pnpbios_read_resources_from_node(dev: *mut pnp_dev, node: *mut pnp_bios_node) -> c_int;
}
extern "C" {
    pub fn pnpbios_write_resources_to_node(dev: *mut pnp_dev, node: *mut pnp_bios_node) -> c_int;
}
extern "C" {
    pub fn pnpbios_print_status(module: *const *const c_char, status: u16);
}
extern "C" {
    pub fn pnpbios_calls_init(header: *mut *mut pnp_bios_install_struct);
}

extern "C" {
    pub fn pnpbios_interface_attach_device(node: *mut *mut pnp_bios_node) -> c_int;
}
extern "C" {
    pub fn pnpbios_proc_init() -> c_int;
}
extern "C" {
    pub fn pnpbios_proc_exit();
}

