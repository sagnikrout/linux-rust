//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/rtas-types.h
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

pub type rtas_arg_t = __be32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_args {
    pub token: __be32,
    pub nargs: __be32,
    pub nret: __be32,
    pub args: [rtas_arg_t; 16],
    pub /: *mut *mut *mut rtas_arg_t rets; / Pointer to return values in args[].,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_t {
    pub /: *mut *mut unsigned long entry; / physical address pointer,
    pub /: *mut *mut unsigned long base; / physical address pointer,
    pub size: c_ulong,
    pub /: *mut *mut *mut device_node dev; / virtual address pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_error_log {
// Byte 0
    pub /: *mut *mut u8 byte0; / Architectural version,
// Byte 1
    pub byte1: u8,
// XXXXXXXX
// XXX		3: Severity level of error
// XX	2: Degree of recovery
// X	1: Extended log present?
// XX	2: Reserved
//
// Byte 2
    pub byte2: u8,
// XXXXXXXX
// XXXX		4: Initiator of event
// XXXX	4: Target of failed operation
//
    pub error*/: *mut *mut u8 byte3; / General event or,
    pub /: *mut *mut __be32 extended_log_length; / length in bytes,
// Start of extended log, variable length
    pub __counted_by_be(extended_log_length): unsigned char buffer[],
}

// RTAS general extended event log, Version 6. The extended log starts
// from "buffer" field of struct rtas_error_log defined above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_ext_event_log_v6 {
// Byte 0
    pub byte0: u8,
// XXXXXXXX
// X		1: Log valid
// X		1: Unrecoverable error
// X		1: Recoverable (correctable or successfully retried)
// X		1: Bypassed unrecoverable error (degraded operation)
// X	1: Predictive error
// X	1: "New" log (always 1 for data returned from RTAS)
// X	1: Big Endian
// X	1: Reserved
//
// Byte 1
    pub /: *mut *mut u8 byte1; / reserved,
// Byte 2
    pub byte2: u8,
// XXXXXXXX
// X		1: Set to 1 (indicating log is in PowerPC format)
// XXX		3: Reserved
// XXXX	4: Log format used for bytes 12-2047
//
// Byte 3
    pub /: *mut *mut u8 byte3; / reserved,
// Byte 4-11
    pub /: *mut *mut u8 reserved[8]; / reserved,
// Byte 12-15
    pub /: *mut *mut __be32 company_id; / Company ID of the company,
// that defines the format for
// the vendor specific log type
// Byte 16-end of log
    pub /: *mut *mut u8 vendor_log[1]; / Start of vendor specific log,
// Variable length.
}

// Vendor specific Platform Event Log Format, Version 6, section header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_errorlog {
    pub /: *mut *mut __be16 id; / 0x00 2-byte ASCII section ID,
    pub /: *mut *mut __be16 length; / 0x02 Section length in bytes,
    pub /: *mut *mut u8 version; / 0x04 Section version,
    pub /: *mut *mut u8 subtype; / 0x05 Section subtype,
    pub /: *mut *mut __be16 creator_component; / 0x06 Creator component ID,
    pub /: *mut *mut u8 data[]; / 0x08 Start of section data,
}

// RTAS pseries hotplug errorlog section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_hp_errorlog {
    pub resource: u8,
    pub action: u8,
    pub id_type: u8,
    pub reserved: u8,
    pub drc_index: __be32,
    pub drc_count: __be32,
    pub ic: { __be32 count, index; },
    pub drc_name: [c_char; 1],
    pub _drc_u: },
}
