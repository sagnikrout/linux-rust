//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soundwire/cadence_master.h
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

//
// The Cadence IP supports up to 32 entries in the FIFO, though implementations
// can configure the IP to have a smaller FIFO.
//
pub const CDNS_MCP_IP_MAX_CMD_LEN: c_int = 32;
pub const SDW_CADENCE_MCP_IP_OFFSET: c_uint = 0x4000;
//
// struct sdw_cdns_pdi: PDI (Physical Data Interface) instance
//
// @num: pdi number
// @intel_alh_id: link identifier
// @l_ch_num: low channel for PDI
// @h_ch_num: high channel for PDI
// @ch_count: total channel count for PDI
// @dir: data direction
// @type: stream type, (only PCM supported)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_cdns_pdi {
    pub num: c_int,
    pub intel_alh_id: c_int,
    pub l_ch_num: c_int,
    pub h_ch_num: c_int,
    pub ch_count: c_int,
    pub dir: sdw_data_direction,
    pub type: sdw_stream_type,
}

//
// struct sdw_cdns_streams: Cadence stream data structure
//
// @num_bd: number of bidirectional streams
// @num_in: number of input streams
// @num_out: number of output streams
// @num_ch_bd: number of bidirectional stream channels
// @num_ch_bd: number of input stream channels
// @num_ch_bd: number of output stream channels
// @num_pdi: total number of PDIs
// @bd: bidirectional streams
// @in: input streams
// @out: output streams
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_cdns_streams {
    pub num_bd: c_uint,
    pub num_in: c_uint,
    pub num_out: c_uint,
    pub num_ch_bd: c_uint,
    pub num_ch_in: c_uint,
    pub num_ch_out: c_uint,
    pub num_pdi: c_uint,
    pub bd: *mut sdw_cdns_pdi,
    pub in: *mut sdw_cdns_pdi,
    pub out: *mut sdw_cdns_pdi,
}

//
// struct sdw_cdns_stream_config: stream configuration
//
// @pcm_bd: number of bidirectional PCM streams supported
// @pcm_in: number of input PCM streams supported
// @pcm_out: number of output PCM streams supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_cdns_stream_config {
    pub pcm_bd: c_uint,
    pub pcm_in: c_uint,
    pub pcm_out: c_uint,
}

//
// struct sdw_cdns_dai_runtime: Cadence DAI runtime data
//
// @name: SoundWire stream name
// @stream: stream runtime
// @pdi: PDI used for this dai
// @bus: Bus handle
// @stream_type: Stream type
// @link_id: Master link id
// @suspended: status set when suspended, to be used in .prepare
// @paused: status set in .trigger, to be used in suspend
// @direction: stream direction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_cdns_dai_runtime {
    pub name: *mut c_char,
    pub stream: *mut sdw_stream_runtime,
    pub pdi: *mut sdw_cdns_pdi,
    pub bus: *mut sdw_bus,
    pub stream_type: sdw_stream_type,
    pub link_id: c_int,
    pub suspended: bool,
    pub paused: bool,
    pub direction: c_int,
}

//
// struct sdw_cdns - Cadence driver context
// @dev: Linux device
// @bus: Bus handle
// @instance: instance number
// @ip_offset: version-dependent offset to access IP_MCP registers and fields
// @response_buf: SoundWire response buffer
// @tx_complete: Tx completion
// @ports: Data ports
// @num_ports: Total number of data ports
// @pcm: PCM streams
// @registers: Cadence registers
// @link_up: Link status
// @msg_count: Messages sent on bus
// @dai_runtime_array: runtime context for each allocated DAI.
// @status_update_lock: protect concurrency between interrupt-based and delayed work
// status update
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_cdns {
    pub dev: *mut device,
    pub bus: sdw_bus,
    pub instance: c_uint,
    pub ip_offset: u32,
//
// The datasheet says the RX FIFO AVAIL can be 2 entries more
// than the FIFO capacity, so allow for this.
//
    pub 2]: u32 response_buf[CDNS_MCP_IP_MAX_CMD_LEN +,
    pub tx_complete: completion,
    pub ports: *mut sdw_cdns_port,
    pub num_ports: c_int,
    pub pcm: sdw_cdns_streams,
    pub pdi_loopback_source: c_int,
    pub pdi_loopback_target: c_int,
    pub registers: *mut void __iomem,
    pub link_up: bool,
    pub msg_count: c_uint,
    pub interrupt_enabled: bool,
    pub work: work_struct,
    pub attach_dwork: delayed_work,
    pub list: list_head,
    pub dai_runtime_array: *mut sdw_cdns_dai_runtime,
    pub /: *mut *mut mutex status_update_lock; / add mutual exclusion to sdw_handle_slave_status(),
}

// Exported symbols
extern "C" {
    pub fn sdw_cdns_probe(cdns: *mut sdw_cdns) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sdw_cdns_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sdw_cdns_soft_reset(cdns: *mut sdw_cdns) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_init(cdns: *mut sdw_cdns) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_exit_reset(cdns: *mut sdw_cdns) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_enable_interrupt(cdns: *mut sdw_cdns, state: bool) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_is_clock_stop(cdns: *mut sdw_cdns) -> bool;
}
extern "C" {
    pub fn sdw_cdns_clock_stop(cdns: *mut sdw_cdns, block_wake: bool) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_clock_restart(cdns: *mut sdw_cdns, bus_reset: bool) -> c_int;
}

extern "C" {
    pub fn sdw_cdns_debugfs_init(cdns: *mut sdw_cdns, root: *mut dentry);
}

extern "C" {
    pub fn cdns_read_ping_status(bus: *mut sdw_bus) -> u32;
}
extern "C" {
    pub fn cdns_bus_conf(bus: *mut sdw_bus, params: *mut sdw_bus_params) -> c_int;
}
extern "C" {
    pub fn sdw_cdns_config_update(cdns: *mut sdw_cdns);
}
extern "C" {
    pub fn sdw_cdns_config_update_set_wait(cdns: *mut sdw_cdns) -> c_int;
}
// SoundWire BPT/BRA helpers to format data
