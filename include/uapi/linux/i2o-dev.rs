//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/i2o-dev.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// I2O user space accessible structures/APIs
//
// (c) Copyright 1999, 2000 Red Hat Software
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// This header file defines the I2O APIs that are available to both
// the kernel and user level applications.  Kernel specific structures
// are defined in i2o_osm. OSMs should include _only_ i2o_osm.h which
// automatically includes this file.
//
// How many controllers are we allowing
pub const MAX_I2O_CONTROLLERS: c_int = 32;

//
// I2O Control IOCTLs and structures
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_cmd_passthru32 {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut __u32 msg; / message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_cmd_passthru {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut *mut void __user msg; / message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_cmd_hrtlct {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut *mut void __user resbuf; / Buffer for result,
    pub /: *mut *mut *mut unsigned int __user reslen; / Buffer length in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_cmd_psetget {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut unsigned int tid; / Target device TID,
    pub /: *mut *mut *mut void __user opbuf; / Operation List buffer,
    pub /: *mut *mut unsigned int oplen; / Operation List buffer length in bytes,
    pub /: *mut *mut *mut void __user resbuf; / Result List buffer,
    pub /: *mut *mut *mut unsigned int __user reslen; / Result List buffer length in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_sw_xfer {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut unsigned char flags; / Flags field,
    pub /: *mut *mut unsigned char sw_type; / Software type,
    pub /: *mut *mut unsigned int sw_id; / Software ID,
    pub /: *mut *mut *mut void __user buf; / Pointer to software buffer,
    pub /: *mut *mut *mut unsigned int __user swlen; / Length of software data,
    pub /: *mut *mut *mut unsigned int __user maxfrag; / Maximum fragment count,
    pub /: *mut *mut *mut unsigned int __user curfrag; / Current fragment count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_html {
    pub /: *mut *mut unsigned int iop; / IOP unit number,
    pub /: *mut *mut unsigned int tid; / Target device ID,
    pub /: *mut *mut unsigned int page; / HTML page,
    pub /: *mut *mut *mut void __user resbuf; / Buffer for reply HTML page,
    pub /: *mut *mut *mut unsigned int __user reslen; / Length in bytes of reply buffer,
    pub /: *mut *mut *mut void __user qbuf; / Pointer to HTTP query string,
    pub /: *mut *mut unsigned int qlen; / Length in bytes of query string buffer,
}

pub const I2O_EVT_Q_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_evt_id {
    pub iop: c_uint,
    pub tid: c_uint,
    pub evt_mask: c_uint,
}

// Event data size = frame size - message header + evt indicator
pub const I2O_EVT_DATA_SIZE: c_int = 88;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_evt_info {
    pub id: i2o_evt_id,
    pub evt_data: [c_uchar; I2O_EVT_DATA_SIZE],
    pub data_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_evt_get {
    pub info: i2o_evt_info,
    pub pending: c_int,
    pub lost: c_int,
}

//
// HRT related constants and structures
//
pub const I2O_BUS_LOCAL: c_int = 0;
pub const I2O_BUS_ISA: c_int = 1;
pub const I2O_BUS_EISA: c_int = 2;
// was  I2O_BUS_MCA	3
pub const I2O_BUS_PCI: c_int = 4;
pub const I2O_BUS_PCMCIA: c_int = 5;
pub const I2O_BUS_NUBUS: c_int = 6;
pub const I2O_BUS_CARDBUS: c_int = 7;
pub const I2O_BUS_UNKNOWN: c_uint = 0x80;
// Event indicator mask flags
pub const I2O_EVT_IND_STATE_CHANGE: c_uint = 0x80000000;
pub const I2O_EVT_IND_GENERAL_WARNING: c_uint = 0x40000000;
pub const I2O_EVT_IND_CONFIGURATION_FLAG: c_uint = 0x20000000;
pub const I2O_EVT_IND_LOCK_RELEASE: c_uint = 0x10000000;
pub const I2O_EVT_IND_CAPABILITY_CHANGE: c_uint = 0x08000000;
pub const I2O_EVT_IND_DEVICE_RESET: c_uint = 0x04000000;
pub const I2O_EVT_IND_EVT_MASK_MODIFIED: c_uint = 0x02000000;
pub const I2O_EVT_IND_FIELD_MODIFIED: c_uint = 0x01000000;
pub const I2O_EVT_IND_VENDOR_EVT: c_uint = 0x00800000;
pub const I2O_EVT_IND_DEVICE_STATE: c_uint = 0x00400000;
// Executive event indicitors
pub const I2O_EVT_IND_EXEC_RESOURCE_LIMITS: c_uint = 0x00000001;
pub const I2O_EVT_IND_EXEC_CONNECTION_FAIL: c_uint = 0x00000002;
pub const I2O_EVT_IND_EXEC_ADAPTER_FAULT: c_uint = 0x00000004;
pub const I2O_EVT_IND_EXEC_POWER_FAIL: c_uint = 0x00000008;
pub const I2O_EVT_IND_EXEC_RESET_PENDING: c_uint = 0x00000010;
pub const I2O_EVT_IND_EXEC_RESET_IMMINENT: c_uint = 0x00000020;
pub const I2O_EVT_IND_EXEC_HW_FAIL: c_uint = 0x00000040;
pub const I2O_EVT_IND_EXEC_XCT_CHANGE: c_uint = 0x00000080;
pub const I2O_EVT_IND_EXEC_NEW_LCT_ENTRY: c_uint = 0x00000100;
pub const I2O_EVT_IND_EXEC_MODIFIED_LCT: c_uint = 0x00000200;
pub const I2O_EVT_IND_EXEC_DDM_AVAILABILITY: c_uint = 0x00000400;
// Random Block Storage Event Indicators
pub const I2O_EVT_IND_BSA_VOLUME_LOAD: c_uint = 0x00000001;
pub const I2O_EVT_IND_BSA_VOLUME_UNLOAD: c_uint = 0x00000002;
pub const I2O_EVT_IND_BSA_VOLUME_UNLOAD_REQ: c_uint = 0x00000004;
pub const I2O_EVT_IND_BSA_CAPACITY_CHANGE: c_uint = 0x00000008;
pub const I2O_EVT_IND_BSA_SCSI_SMART: c_uint = 0x00000010;
// Event data for generic events
pub const I2O_EVT_STATE_CHANGE_NORMAL: c_uint = 0x00;
pub const I2O_EVT_STATE_CHANGE_SUSPENDED: c_uint = 0x01;
pub const I2O_EVT_STATE_CHANGE_RESTART: c_uint = 0x02;
pub const I2O_EVT_STATE_CHANGE_NA_RECOVER: c_uint = 0x03;
pub const I2O_EVT_STATE_CHANGE_NA_NO_RECOVER: c_uint = 0x04;
pub const I2O_EVT_STATE_CHANGE_QUIESCE_REQUEST: c_uint = 0x05;
pub const I2O_EVT_STATE_CHANGE_FAILED: c_uint = 0x10;
pub const I2O_EVT_STATE_CHANGE_FAULTED: c_uint = 0x11;
pub const I2O_EVT_GEN_WARNING_NORMAL: c_uint = 0x00;
pub const I2O_EVT_GEN_WARNING_ERROR_THRESHOLD: c_uint = 0x01;
pub const I2O_EVT_GEN_WARNING_MEDIA_FAULT: c_uint = 0x02;
pub const I2O_EVT_CAPABILITY_OTHER: c_uint = 0x01;
pub const I2O_EVT_CAPABILITY_CHANGED: c_uint = 0x02;
pub const I2O_EVT_SENSOR_STATE_CHANGED: c_uint = 0x01;
//
// I2O classes / subclasses
//
// Class ID and Code Assignments
// (LCT.ClassID.Version field)
//
pub const I2O_CLASS_VERSION_10: c_uint = 0x00;
pub const I2O_CLASS_VERSION_11: c_uint = 0x01;
// Class code names
// (from v1.5 Table 6-1 Class Code Assignments.)
//
pub const I2O_CLASS_EXECUTIVE: c_uint = 0x000;
pub const I2O_CLASS_DDM: c_uint = 0x001;
pub const I2O_CLASS_RANDOM_BLOCK_STORAGE: c_uint = 0x010;
pub const I2O_CLASS_SEQUENTIAL_STORAGE: c_uint = 0x011;
pub const I2O_CLASS_LAN: c_uint = 0x020;
pub const I2O_CLASS_WAN: c_uint = 0x030;
pub const I2O_CLASS_FIBRE_CHANNEL_PORT: c_uint = 0x040;
pub const I2O_CLASS_FIBRE_CHANNEL_PERIPHERAL: c_uint = 0x041;
pub const I2O_CLASS_SCSI_PERIPHERAL: c_uint = 0x051;
pub const I2O_CLASS_ATE_PORT: c_uint = 0x060;
pub const I2O_CLASS_ATE_PERIPHERAL: c_uint = 0x061;
pub const I2O_CLASS_FLOPPY_CONTROLLER: c_uint = 0x070;
pub const I2O_CLASS_FLOPPY_DEVICE: c_uint = 0x071;
pub const I2O_CLASS_BUS_ADAPTER: c_uint = 0x080;
pub const I2O_CLASS_PEER_TRANSPORT_AGENT: c_uint = 0x090;
pub const I2O_CLASS_PEER_TRANSPORT: c_uint = 0x091;
pub const I2O_CLASS_END: c_uint = 0xfff;
//
// Rest of 0x092 - 0x09f reserved for peer-to-peer classes
//
pub const I2O_CLASS_MATCH_ANYCLASS: c_uint = 0xffffffff;
//
// Subclasses
//
pub const I2O_SUBCLASS_i960: c_uint = 0x001;
pub const I2O_SUBCLASS_HDM: c_uint = 0x020;
pub const I2O_SUBCLASS_ISM: c_uint = 0x021;
// Operation functions
pub const I2O_PARAMS_FIELD_GET: c_uint = 0x0001;
pub const I2O_PARAMS_LIST_GET: c_uint = 0x0002;
pub const I2O_PARAMS_MORE_GET: c_uint = 0x0003;
pub const I2O_PARAMS_SIZE_GET: c_uint = 0x0004;
pub const I2O_PARAMS_TABLE_GET: c_uint = 0x0005;
pub const I2O_PARAMS_FIELD_SET: c_uint = 0x0006;
pub const I2O_PARAMS_LIST_SET: c_uint = 0x0007;
pub const I2O_PARAMS_ROW_ADD: c_uint = 0x0008;
pub const I2O_PARAMS_ROW_DELETE: c_uint = 0x0009;
pub const I2O_PARAMS_TABLE_CLEAR: c_uint = 0x000A;
//
// I2O serial number conventions / formats
// (circa v1.5)
//
pub const I2O_SNFORMAT_UNKNOWN: c_int = 0;
pub const I2O_SNFORMAT_BINARY: c_int = 1;
pub const I2O_SNFORMAT_ASCII: c_int = 2;
pub const I2O_SNFORMAT_UNICODE: c_int = 3;
pub const I2O_SNFORMAT_LAN48_MAC: c_int = 4;
pub const I2O_SNFORMAT_WAN: c_int = 5;
//
// Plus new in v2.0 (Yellowstone pdf doc)
//
pub const I2O_SNFORMAT_LAN64_MAC: c_int = 6;
pub const I2O_SNFORMAT_DDM: c_int = 7;
pub const I2O_SNFORMAT_IEEE_REG64: c_int = 8;
pub const I2O_SNFORMAT_IEEE_REG128: c_int = 9;
pub const I2O_SNFORMAT_UNKNOWN2: c_uint = 0xff;
//
// I2O Get Status State values
//
pub const ADAPTER_STATE_INITIALIZING: c_uint = 0x01;
pub const ADAPTER_STATE_RESET: c_uint = 0x02;
pub const ADAPTER_STATE_HOLD: c_uint = 0x04;
pub const ADAPTER_STATE_READY: c_uint = 0x05;
pub const ADAPTER_STATE_OPERATIONAL: c_uint = 0x08;
pub const ADAPTER_STATE_FAILED: c_uint = 0x10;
pub const ADAPTER_STATE_FAULTED: c_uint = 0x11;
//
// Software module types
//
pub const I2O_SOFTWARE_MODULE_IRTOS: c_uint = 0x11;
pub const I2O_SOFTWARE_MODULE_IOP_PRIVATE: c_uint = 0x22;
pub const I2O_SOFTWARE_MODULE_IOP_CONFIG: c_uint = 0x23;
//
// Vendors
//
pub const I2O_VENDOR_DPT: c_uint = 0x001b;
//
// DPT / Adaptec specific values for i2o_sg_io_hdr flags.
//
pub const I2O_DPT_SG_FLAG_INTERPRET: c_uint = 0x00010000;
pub const I2O_DPT_SG_FLAG_PHYSICAL: c_uint = 0x00020000;
pub const I2O_DPT_FLASH_FRAG_SIZE: c_uint = 0x10000;
pub const I2O_DPT_FLASH_READ: c_uint = 0x0101;
pub const I2O_DPT_FLASH_WRITE: c_uint = 0x0102;
