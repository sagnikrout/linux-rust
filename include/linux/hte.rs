//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hte.h
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
// enum hte_edge - HTE line edge flags.
//
// @HTE_EDGE_NO_SETUP: No edge setup. In this case consumer will setup edges,
// for example during request irq call.
// @HTE_RISING_EDGE_TS: Rising edge.
// @HTE_FALLING_EDGE_TS: Falling edge.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hte_edge {
    HTE_EDGE_NO_SETUP = 1U << 0,
    HTE_RISING_EDGE_TS = 1U << 1,
    HTE_FALLING_EDGE_TS = 1U << 2,
}

//
// enum hte_return - HTE subsystem return values used during callback.
//
// @HTE_CB_HANDLED: The consumer handled the data.
// @HTE_RUN_SECOND_CB: The consumer needs further processing, in that case
// HTE subsystem calls secondary callback provided by the consumer where it
// is allowed to sleep.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hte_return {
    HTE_CB_HANDLED,
    HTE_RUN_SECOND_CB,
}

//
// struct hte_ts_data - HTE timestamp data.
//
// @tsc: Timestamp value.
// @seq: Sequence counter of the timestamps.
// @raw_level: Level of the line at the timestamp if provider supports it,
// -1 otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_ts_data {
    pub tsc: u64,
    pub seq: u64,
    pub raw_level: c_int,
}

//
// struct hte_clk_info - Clock source info that HTE provider uses to timestamp.
//
// @hz: Supported clock rate in HZ, for example 1KHz clock = 1000.
// @type: Supported clock type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_clk_info {
    pub hz: u64,
    pub type: clockid_t,
}

//
// typedef hte_ts_cb_t - HTE timestamp data processing primary callback.
//
// The callback is used to push timestamp data to the client and it is
// not allowed to sleep.
//
// @ts: HW timestamp data.
// @data: Client supplied data.
//
extern "C" {
    pub fn hte_return(ts: *mut *mut hte_ts_cb_t)(struct hte_ts_data, data: *mut c_void) -> typedef enum;
}
//
// typedef hte_ts_sec_cb_t - HTE timestamp data processing secondary callback.
//
// This is used when the client needs further processing where it is
// allowed to sleep.
//
// @data: Client supplied data.
//
extern "C" {
    pub fn hte_return(data: *mut *mut hte_ts_sec_cb_t)(void) -> typedef enum;
}
//
// struct hte_line_attr - Line attributes.
//
// @line_id: The logical ID understood by the consumers and providers.
// @line_data: Line data related to line_id.
// @edge_flags: Edge setup flags.
// @name: Descriptive name of the entity that is being monitored for the
// hardware timestamping. If null, HTE core will construct the name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_line_attr {
    pub line_id: u32,
    pub line_data: *mut c_void,
    pub edge_flags: c_ulong,
    pub name: *const c_char,
}

//
// struct hte_ts_desc - HTE timestamp descriptor.
//
// This structure is a communication token between consumers to subsystem
// and subsystem to providers.
//
// @attr: The line attributes.
// @hte_data: Subsystem's private data, set by HTE subsystem.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_ts_desc {
    pub attr: hte_line_attr,
    pub hte_data: *mut c_void,
}

//
// struct hte_ops - HTE operations set by providers.
//
// @request: Hook for requesting a HTE timestamp. Returns 0 on success,
// non-zero for failures.
// @release: Hook for releasing a HTE timestamp. Returns 0 on success,
// non-zero for failures.
// @enable: Hook to enable the specified timestamp. Returns 0 on success,
// non-zero for failures.
// @disable: Hook to disable specified timestamp. Returns 0 on success,
// non-zero for failures.
// @get_clk_src_info: Hook to get the clock information the provider uses
// to timestamp. Returns 0 for success and negative error code for failure. On
// success HTE subsystem fills up provided struct hte_clk_info.
//
// xlated_id parameter is used to communicate between HTE subsystem and the
// providers and is translated by the provider.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_ops {
    pub xlated_id): u32,
    pub xlated_id): u32,
    pub xlated_id): *mut *mut *mut int (enable)(struct hte_chip chip, u32,
    pub xlated_id): *mut *mut *mut int (disable)(struct hte_chip chip, u32,
    pub ci): *mut hte_clk_info,
}

//
// struct hte_chip - Abstract HTE chip.
//
// @name: functional name of the HTE IP block.
// @dev: device providing the HTE.
// @ops: callbacks for this HTE.
// @nlines: number of lines/signals supported by this chip.
// @xlate_of: Callback which translates consumer supplied logical ids to
// physical ids, return 0 for the success and negative for the failures.
// It stores (between 0 to @nlines) in xlated_id parameter for the success.
// @xlate_plat: Same as above but for the consumers with no DT node.
// @match_from_linedata: Match HTE device using the line_data.
// @of_hte_n_cells: Number of cells used to form the HTE specifier.
// @gdev: HTE subsystem abstract device, internal to the HTE subsystem.
// @data: chip specific private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hte_chip {
    pub name: *const c_char,
    pub dev: *mut device,
    pub ops: *const hte_ops,
    pub nlines: u32,
    pub xlated_id): *mut *mut hte_ts_desc desc, u32,
    pub xlated_id): *mut u32,
    pub hdesc): *const hte_ts_desc,
    pub of_hte_n_cells: u8,
    pub gdev: *mut hte_device,
    pub data: *mut c_void,
}

// HTE APIs for the providers
extern "C" {
    pub fn devm_hte_register_chip(chip: *mut hte_chip) -> c_int;
}
// HTE APIs for the consumers
extern "C" {
    pub fn hte_ts_get(dev: *mut device, desc: *mut hte_ts_desc, index: c_int) -> c_int;
}
extern "C" {
    pub fn hte_ts_put(desc: *mut hte_ts_desc) -> c_int;
}
extern "C" {
    pub fn of_hte_req_count(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn hte_enable_ts(desc: *mut hte_ts_desc) -> c_int;
}
extern "C" {
    pub fn hte_disable_ts(desc: *mut hte_ts_desc) -> c_int;
}

