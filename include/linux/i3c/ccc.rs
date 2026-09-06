//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i3c/ccc.h
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
//
// Copyright (C) 2018 Cadence Design Systems Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

// I3C CCC (Common Command Codes) related definitions
pub const I3C_CCC_RETRIES: c_int = 1;

// Commands valid in both broadcast and unicast modes

// Broadcast-only commands

// Unicast-only commands

//
// struct i3c_ccc_events - payload passed to ENEC/DISEC CCC
//
// @events: bitmask of I3C_CCC_EVENT_xxx events.
//
// Depending on the CCC command, the specific events coming from all devices
// (broadcast version) or a specific device (unicast version) will be
// enabled (ENEC) or disabled (DISEC).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_events {
    pub events: u8,
}

//
// struct i3c_ccc_mwl - payload passed to SETMWL/GETMWL CCC
//
// @len: maximum write length in bytes
//
// The maximum write length is only applicable to SDR private messages or
// extended Write CCCs (like SETXTIME).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_mwl {
    pub len: __be16,
}

//
// struct i3c_ccc_mrl - payload passed to SETMRL/GETMRL CCC
//
// @len: maximum read length in bytes
// @ibi_len: maximum IBI payload length
//
// The maximum read length is only applicable to SDR private messages or
// extended Read CCCs (like GETXTIME).
// The IBI length is only valid if the I3C slave is IBI capable
// (%I3C_BCR_IBI_REQ_CAP is set).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_mrl {
    pub read_len: __be16,
    pub ibi_len: u8,
    pub __packed: },
//
// struct i3c_ccc_dev_desc - I3C/I2C device descriptor used for DEFSLVS
//
// @dyn_addr: dynamic address assigned to the I3C slave or 0 if the entry is
// describing an I2C slave.
// @dcr: DCR value (not applicable to entries describing I2C devices)
// @lvr: LVR value (not applicable to entries describing I3C devices)
// @bcr: BCR value or 0 if this entry is describing an I2C slave
// @static_addr: static address or 0 if the device does not have a static
// address
//
// The DEFSLVS command should be passed an array of i3c_ccc_dev_desc
// descriptors (one entry per I3C/I2C dev controlled by the master).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_dev_desc {
    pub dyn_addr: u8,
    pub dcr: u8,
    pub lvr: u8,
}

//
// struct i3c_ccc_defslvs - payload passed to DEFSLVS CCC
//
// @count: number of dev descriptors
// @master: descriptor describing the current master
// @slaves: array of descriptors describing slaves controlled by the
// current master
//
// Information passed to the broadcast DEFSLVS to propagate device
// information to all masters currently acting as slaves on the bus.
// This is only meaningful if you have more than one master.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_defslvs {
    pub count: u8,
    pub master: i3c_ccc_dev_desc,
    pub slaves: [i3c_ccc_dev_desc; ],
    pub __packed: },
//
// enum i3c_ccc_test_mode - enum listing all available test modes
//
// @I3C_CCC_EXIT_TEST_MODE: exit test mode
// @I3C_CCC_VENDOR_TEST_MODE: enter vendor test mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_ccc_test_mode {
    I3C_CCC_EXIT_TEST_MODE,
    I3C_CCC_VENDOR_TEST_MODE,
}

//
// struct i3c_ccc_enttm - payload passed to ENTTM CCC
//
// @mode: one of the &enum i3c_ccc_test_mode modes
//
// Information passed to the ENTTM CCC to instruct an I3C device to enter a
// specific test mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_enttm {
    pub mode: u8,
}

//
// struct i3c_ccc_setda - payload passed to SETNEWDA and SETDASA CCCs
//
// @addr: dynamic address to assign to an I3C device
//
// Information passed to the SETNEWDA and SETDASA CCCs to assign/change the
// dynamic address of an I3C device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_setda {
    pub addr: u8,
}

//
// struct i3c_ccc_getpid - payload passed to GETPID CCC
//
// @pid: 48 bits PID in big endian
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getpid {
    pub pid: [u8; 6],
}

//
// struct i3c_ccc_getbcr - payload passed to GETBCR CCC
//
// @bcr: BCR (Bus Characteristic Register) value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getbcr {
    pub bcr: u8,
}

//
// struct i3c_ccc_getdcr - payload passed to GETDCR CCC
//
// @dcr: DCR (Device Characteristic Register) value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getdcr {
    pub dcr: u8,
}

//
// struct i3c_ccc_getstatus - payload passed to GETSTATUS CCC
//
// @status: status of the I3C slave (see I3C_CCC_STATUS_xxx macros for more
// information).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getstatus {
    pub status: __be16,
}

//
// struct i3c_ccc_getaccmst - payload passed to GETACCMST CCC
//
// @newmaster: address of the master taking bus ownership
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getaccmst {
    pub newmaster: u8,
}

//
// struct i3c_ccc_bridged_slave_desc - bridged slave descriptor
//
// @addr: dynamic address of the bridged device
// @id: ID of the slave device behind the bridge
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_bridged_slave_desc {
    pub addr: u8,
    pub id: __be16,
    pub __packed: },
//
// struct i3c_ccc_setbrgtgt - payload passed to SETBRGTGT CCC
//
// @count: number of bridged slaves
// @bslaves: bridged slave descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_setbrgtgt {
    pub count: u8,
    pub bslaves: [i3c_ccc_bridged_slave_desc; ],
    pub __packed: },
//
// enum i3c_sdr_max_data_rate - max data rate values for private SDR transfers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_sdr_max_data_rate {
    I3C_SDR0_FSCL_MAX,
    I3C_SDR1_FSCL_8MHZ,
    I3C_SDR2_FSCL_6MHZ,
    I3C_SDR3_FSCL_4MHZ,
    I3C_SDR4_FSCL_2MHZ,
}

//
// enum i3c_tsco - clock to data turn-around
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_tsco {
    I3C_TSCO_8NS,
    I3C_TSCO_9NS,
    I3C_TSCO_10NS,
    I3C_TSCO_11NS,
    I3C_TSCO_12NS,
}

//
// struct i3c_ccc_getmxds - payload passed to GETMXDS CCC
//
// @maxwr: write limitations
// @maxrd: read limitations
// @maxrdturn: maximum read turn-around expressed micro-seconds and
// little-endian formatted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getmxds {
    pub maxwr: u8,
    pub maxrd: u8,
    pub maxrdturn: [u8; 3],
    pub __packed: },

//
// struct i3c_ccc_gethdrcap - payload passed to GETHDRCAP CCC
//
// @modes: bitmap of supported HDR modes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_gethdrcap {
    pub modes: u8,
    pub __packed: },
//
// enum i3c_ccc_setxtime_subcmd - SETXTIME sub-commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_ccc_setxtime_subcmd {
    I3C_CCC_SETXTIME_ST = 0x7f,
    I3C_CCC_SETXTIME_DT = 0xbf,
    I3C_CCC_SETXTIME_ENTER_ASYNC_MODE0 = 0xdf,
    I3C_CCC_SETXTIME_ENTER_ASYNC_MODE1 = 0xef,
    I3C_CCC_SETXTIME_ENTER_ASYNC_MODE2 = 0xf7,
    I3C_CCC_SETXTIME_ENTER_ASYNC_MODE3 = 0xfb,
    I3C_CCC_SETXTIME_ASYNC_TRIGGER = 0xfd,
    I3C_CCC_SETXTIME_TPH = 0x3f,
    I3C_CCC_SETXTIME_TU = 0x9f,
    I3C_CCC_SETXTIME_ODR = 0x8f,
}

//
// struct i3c_ccc_setxtime - payload passed to SETXTIME CCC
//
// @subcmd: one of the sub-commands ddefined in &enum i3c_ccc_setxtime_subcmd
// @data: sub-command payload. Amount of data is determined by
// &i3c_ccc_setxtime->subcmd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_setxtime {
    pub subcmd: u8,
    pub data: [u8; ],
    pub __packed: },

//
// struct i3c_ccc_getxtime - payload retrieved from GETXTIME CCC
//
// @supported_modes: bitmap describing supported XTIME modes
// @state: current status (enabled mode and overflow status)
// @frequency: slave's internal oscillator frequency in 500KHz steps
// @inaccuracy: slave's internal oscillator inaccuracy in 0.1% steps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_getxtime {
    pub supported_modes: u8,
    pub state: u8,
    pub frequency: u8,
    pub inaccuracy: u8,
    pub __packed: },
//
// struct i3c_ccc_cmd_payload - CCC payload
//
// @len: requested payload length
// @actual_len: number of bytes received on a GET CCC (filled by the driver)
// @optional_bytes: GET CCCs may return up to this many fewer bytes than @len
// @data: payload data. This buffer must be DMA-able
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_cmd_payload {
    pub len: u16,
    pub actual_len: u16,
    pub optional_bytes: u16,
    pub data: *mut c_void,
}

//
// struct i3c_ccc_cmd_dest - CCC command destination
//
// @addr: can be an I3C device address or the broadcast address if this is a
// broadcast CCC
// @payload: payload to be sent to this device or broadcasted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_cmd_dest {
    pub addr: u8,
    pub payload: i3c_ccc_cmd_payload,
}

//
// struct i3c_ccc_cmd - CCC command
//
// @rnw: true if the CCC should retrieve data from the device. Only valid for
// unicast commands
// @id: CCC command id
// @ndests: number of destinations. Should always be one for broadcast commands
// @dests: array of destinations and associated payload for this CCC. Most of
// the time, only one destination is provided
// @retries: number of times to retry a failed Direct GET CCC (see
// &I3C_CCC_RETRIES)
// @err: I3C error code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ccc_cmd {
    pub rnw: u8,
    pub id: u8,
    pub ndests: c_uint,
    pub retries: c_uint,
    pub dests: *mut i3c_ccc_cmd_dest,
    pub err: i3c_error_code,
}
