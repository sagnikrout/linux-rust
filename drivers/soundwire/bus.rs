//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soundwire/bus.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-17 Intel Corporation.
pub const DEFAULT_BANK_SWITCH_TIMEOUT: c_int = 3000;
pub const DEFAULT_PROBE_TIMEOUT: c_int = 2000;
extern "C" {
    pub fn sdw_dmi_override_adr(bus: *mut sdw_bus, addr: u64) -> u64;
}

extern "C" {
    pub fn sdw_acpi_find_slaves(bus: *mut sdw_bus) -> c_int;
}

extern "C" {
    pub fn sdw_of_find_slaves(bus: *mut sdw_bus) -> c_int;
}
extern "C" {
    pub fn sdw_master_device_del(bus: *mut sdw_bus) -> c_int;
}

extern "C" {
    pub fn sdw_bus_debugfs_init(bus: *mut sdw_bus);
}
extern "C" {
    pub fn sdw_bus_debugfs_exit(bus: *mut sdw_bus);
}
extern "C" {
    pub fn sdw_slave_debugfs_init(slave: *mut sdw_slave);
}
extern "C" {
    pub fn sdw_slave_debugfs_exit(slave: *mut sdw_slave);
}
extern "C" {
    pub fn sdw_debugfs_init();
}
extern "C" {
    pub fn sdw_debugfs_exit();
}

//
// struct sdw_msg - Message structure
// @addr: Register address accessed in the Slave
// @len: number of messages
// @dev_num: Slave device number
// @addr_page1: SCP address page 1 Slave register
// @addr_page2: SCP address page 2 Slave register
// @flags: transfer flags, indicate if xfer is read or write
// @buf: message data buffer
// @ssp_sync: Send message at SSP (Stream Synchronization Point)
// @page: address requires paging
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_msg {
    pub addr: u16,
    pub len: u16,
    pub dev_num: u8,
    pub addr_page1: u8,
    pub addr_page2: u8,
    pub flags: u8,
    pub buf: *mut u8,
    pub ssp_sync: bool,
    pub page: bool,
}

//
// struct sdw_bpt_section - Message section structure
// @addr: Start Register address accessed in the Slave
// @len: number of bytes to transfer. More than 64Kb can be transferred
// but a practical limit of SDW_BPT_MSG_MAX_BYTES is enforced.
// @buf: section data buffer (filled by host for write, filled
// by Peripheral hardware for reads)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_bpt_section {
    pub addr: u32,
    pub len: u32,
    pub buf: *mut u8,
}

//
// struct sdw_bpt_msg - Message structure
// @sec: Pointer to array of sections
// @sections: Number of sections in the array
// @dev_num: Slave device number
// @flags: transfer flags, indicate if xfer is read or write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_bpt_msg {
    pub sec: *mut sdw_bpt_section,
    pub sections: c_int,
    pub dev_num: u8,
    pub flags: u8,
}

pub const SDW_DOUBLE_RATE_FACTOR: c_int = 2;
pub const SDW_STRM_RATE_GROUPING: c_int = 1;
extern "C" {
    pub fn sdw_find_row_index(row: c_int) -> c_int;
}
extern "C" {
    pub fn sdw_find_col_index(col: c_int) -> c_int;
}
//
// struct sdw_port_runtime - Runtime port parameters for Master or Slave
//
// @num: Port number. For audio streams, valid port number ranges from
// [1,14]
// @ch_mask: Channel mask
// @transport_params: Transport parameters
// @port_params: Port parameters
// @port_node: List node for Master or Slave port_list
// @lane: Which lane is used
//
// SoundWire spec has no mention of ports for Master interface but the
// concept is logically extended.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_runtime {
    pub num: c_int,
    pub ch_mask: c_int,
    pub transport_params: sdw_transport_params,
    pub port_params: sdw_port_params,
    pub port_node: list_head,
    pub lane: c_uint,
}

//
// struct sdw_slave_runtime - Runtime Stream parameters for Slave
//
// @slave: Slave handle
// @direction: Data direction for Slave
// @ch_count: Number of channels handled by the Slave for
// this stream
// @m_rt_node: sdw_master_runtime list node
// @port_list: List of Slave Ports configured for this stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave_runtime {
    pub slave: *mut sdw_slave,
    pub direction: sdw_data_direction,
    pub ch_count: c_uint,
    pub m_rt_node: list_head,
    pub port_list: list_head,
}

//
// struct sdw_master_runtime - Runtime stream parameters for Master
//
// @bus: Bus handle
// @stream: Stream runtime handle
// @direction: Data direction for Master
// @ch_count: Number of channels handled by the Master for
// this stream, can be zero.
// @slave_rt_list: Slave runtime list
// @port_list: List of Master Ports configured for this stream, can be zero.
// @stream_node: sdw_stream_runtime master_list node
// @bus_node: sdw_bus m_rt_list node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_master_runtime {
    pub bus: *mut sdw_bus,
    pub stream: *mut sdw_stream_runtime,
    pub direction: sdw_data_direction,
    pub ch_count: c_uint,
    pub slave_rt_list: list_head,
    pub port_list: list_head,
    pub stream_node: list_head,
    pub bus_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_transport_data {
    pub hstart: c_int,
    pub hstop: c_int,
    pub block_offset: c_int,
    pub sub_block_offset: c_int,
    pub lane: c_uint,
}

extern "C" {
    pub fn sdw_transfer(bus: *mut sdw_bus, msg: *mut sdw_msg) -> c_int;
}
extern "C" {
    pub fn sdw_transfer_defer(bus: *mut sdw_bus, msg: *mut sdw_msg) -> c_int;
}
pub const SDW_READ_INTR_CLEAR_RETRY: c_int = 10;
// Fill transport parameter data structure
// Fill port parameter data structure
// broadcast read/write for tests
extern "C" {
    pub fn sdw_bread_no_pm_unlocked(bus: *mut sdw_bus, dev_num: u16, addr: u32) -> c_int;
}
extern "C" {
    pub fn sdw_bwrite_no_pm_unlocked(bus: *mut sdw_bus, dev_num: u16, addr: u32, value: u8) -> c_int;
}
//
// At the moment we only track Master-initiated hw_reset.
// Additional fields can be added as needed
//

extern "C" {
    pub fn sdw_clear_slave_status(bus: *mut sdw_bus, request: u32);
}
extern "C" {
    pub fn sdw_slave_modalias(slave: *const sdw_slave, buf: *mut c_char, size: usize) -> c_int;
}
