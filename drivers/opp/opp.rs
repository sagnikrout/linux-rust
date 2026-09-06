//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/opp/opp.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Generic OPP Interface
//
// Copyright (C) 2009-2010 Texas Instruments Incorporated.
// Nishanth Menon
// Romit Dasgupta
// Kevin Hilman
//

// Lock to allow exclusive modification to the device and opp lists
// OPP Config flags

//
// struct opp_config_data - data for set config operations
// @opp_table: OPP table
// @flags: OPP config flags
// @required_dev_index: The position in the array of required_devs
//
// This structure stores the OPP config information for each OPP table
// configuration by the callers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_config_data {
    pub opp_table: *mut opp_table,
    pub flags: c_uint,
    pub required_dev_index: c_uint,
}

//
// struct dev_pm_opp_icc_bw - Interconnect bandwidth values
// @avg:	Average bandwidth corresponding to this OPP (in icc units)
// @peak:	Peak bandwidth corresponding to this OPP (in icc units)
//
// This structure stores the bandwidth values for a single interconnect path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_icc_bw {
    pub avg: u32,
    pub peak: u32,
}

//
// Internal data structure organization with the OPP layer library is as
// follows:
// opp_tables (root)
// |- device 1 (represents voltage domain 1)
// |	|- opp 1 (availability, freq, voltage)
// |	|- opp 2 ..
// ...	...
// |	`- opp n ..
// |- device 2 (represents the next voltage domain)
// ...
// `- device m (represents mth voltage domain)
// device 1, 2.. are represented by opp_table structure while each opp
// is represented by the opp structure.
//
// struct dev_pm_opp - Generic OPP description structure
// @node:	opp table node. The nodes are maintained throughout the lifetime
// of boot. It is expected only an optimal set of OPPs are
// added to the library by the SoC framework.
// IMPORTANT: the opp nodes should be maintained in increasing
// order.
// @kref:	for reference count of the OPP.
// @available:	true/false - marks if this OPP as available or not
// @dynamic:	not-created from static DT entries.
// @turbo:	true if turbo (boost) OPP
// @suspend:	true if suspend OPP
// @removed:	flag indicating that OPP's reference is dropped by OPP core.
// @rates:	Frequencies in hertz
// @level:	Performance level
// @supplies:	Power supplies voltage/current values
// @bandwidth:	Interconnect bandwidth values
// @clock_latency_ns: Latency (in nanoseconds) of switching to this OPP's
// frequency from any other OPP's frequency.
// @required_opps: List of OPPs that are required by this OPP.
// @opp_table:	points back to the opp_table struct this opp belongs to
// @np:		OPP's device node.
// @dentry:	debugfs dentry pointer (per opp)
//
// This structure stores the OPP information for a given device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp {
    pub node: list_head,
    pub kref: kref,
    pub available: bool,
    pub dynamic: bool,
    pub turbo: bool,
    pub suspend: bool,
    pub removed: bool,
    pub rates: *mut c_ulong,
    pub level: c_uint,
    pub supplies: *mut dev_pm_opp_supply,
    pub bandwidth: *mut dev_pm_opp_icc_bw,
    pub clock_latency_ns: c_ulong,
    pub required_opps: *mut dev_pm_opp,
    pub opp_table: *mut opp_table,
    pub np: *mut device_node,

    pub dentry: *mut dentry,
    pub of_name: *const c_char,

}

//
// struct opp_device - devices managed by 'struct opp_table'
// @node:	list node
// @dev:	device to which the struct object belongs
// @dentry:	debugfs dentry pointer (per device)
//
// This is an internal data structure maintaining the devices that are managed
// by 'struct opp_table'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_device {
    pub node: list_head,
    pub dev: *const device,

    pub dentry: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opp_table_access {
    OPP_TABLE_ACCESS_UNKNOWN = 0,
    OPP_TABLE_ACCESS_EXCLUSIVE = 1,
    OPP_TABLE_ACCESS_SHARED = 2,
}

//
// struct opp_table - Device opp structure
// @node:	table node - contains the devices with OPPs that
// have been registered. Nodes once added are not modified in this
// table.
// @head:	notifier head to notify the OPP availability changes.
// @dev_list:	list of devices that share these OPPs
// @opp_list:	table of opps
// @kref:	for reference count of the table.
// @lock:	mutex protecting the opp_list and dev_list.
// @np:		struct device_node pointer for opp's DT node.
// @clock_latency_ns_max: Max clock latency in nanoseconds.
// @parsed_static_opps: Count of devices for which OPPs are initialized from DT.
// @shared_opp: OPP is shared between multiple devices.
// @current_rate_single_clk: Currently configured frequency for single clk.
// @current_opp: Currently configured OPP for the table.
// @suspend_opp: Pointer to OPP to be used during device suspend.
// @required_opp_tables: List of device OPP tables that are required by OPPs in
// this table.
// @required_devs: List of devices for required OPP tables.
// @required_opp_count: Number of required devices.
// @supported_hw: Array of version number to support.
// @supported_hw_count: Number of elements in supported_hw array.
// @prop_name: A name to postfix to many DT properties, while parsing them.
// @config_clks: Platform specific config_clks() callback.
// @clks: Device's clock handles, for multiple clocks.
// @clk: Device's clock handle, for single clock.
// @clk_count: Number of clocks.
// @config_regulators: Platform specific config_regulators() callback.
// @regulators: Supply regulators
// @regulator_count: Number of power supply regulators. Its value can be -1
// (uninitialized), 0 (no opp-microvolt property) or > 0 (has opp-microvolt
// property).
// @paths: Interconnect path handles
// @path_count: Number of interconnect paths
// @enabled: Set to true if the device's resources are enabled/configured.
// @is_genpd: Marks if the OPP table belongs to a genpd.
// @dentry:	debugfs dentry pointer of the real device directory (not links).
// @dentry_name: Name of the real dentry.
//
// @voltage_tolerance_v1: In percentage, for v1 bindings only.
//
// This is an internal data structure maintaining the link to opps attached to
// a device. This structure is not meant to be shared to users as it is
// meant for book keeping and private to OPP library.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_table {
    pub lazy: list_head node,,
    pub head: blocking_notifier_head,
    pub dev_list: list_head,
    pub opp_list: list_head,
    pub kref: kref,
    pub lock: mutex,
    pub np: *mut device_node,
    pub clock_latency_ns_max: c_ulong,
// For backward compatibility with v1 bindings
    pub voltage_tolerance_v1: c_uint,
    pub parsed_static_opps: c_uint,
    pub shared_opp: opp_table_access,
    pub current_rate_single_clk: c_ulong,
    pub current_opp: *mut dev_pm_opp,
    pub suspend_opp: *mut dev_pm_opp,
    pub required_opp_tables: *mut opp_table,
    pub required_devs: *mut device,
    pub required_opp_count: c_uint,
    pub supported_hw: *mut c_uint,
    pub supported_hw_count: c_uint,
    pub prop_name: *const c_char,
    pub config_clks: config_clks_t,
    pub clks: *mut clk,
    pub clk: *mut clk,
    pub clk_count: c_int,
    pub config_regulators: config_regulators_t,
    pub regulators: *mut regulator,
    pub regulator_count: c_int,
    pub paths: *mut icc_path,
    pub path_count: c_uint,
    pub enabled: bool,
    pub is_genpd: bool,

    pub dentry: *mut dentry,
    pub dentry_name: [c_char; NAME_MAX],
}

// Routines internal to opp core
extern "C" {
    pub fn _opp_remove_all_static(opp_table: *mut opp_table) -> bool;
}
extern "C" {
    pub fn _get_opp_count(opp_table: *mut opp_table) -> c_int;
}
extern "C" {
    pub fn _opp_free(opp: *mut dev_pm_opp);
}
extern "C" {
    pub fn _opp_compare_key(opp_table: *mut opp_table, opp1: *mut dev_pm_opp, opp2: *mut dev_pm_opp) -> c_int;
}
extern "C" {
    pub fn _opp_add(dev: *mut device, new_opp: *mut dev_pm_opp, opp_table: *mut opp_table) -> c_int;
}
extern "C" {
    pub fn _opp_add_v1(opp_table: *mut opp_table, dev: *mut device, data: *mut dev_pm_opp_data, dynamic: bool) -> c_int;
}
extern "C" {
    pub fn _dev_pm_opp_cpumask_remove_table(cpumask: *const cpumask, last_cpu: c_int);
}
extern "C" {
    pub fn _required_opps_available(opp: *mut dev_pm_opp, count: c_int);
}
extern "C" {
    pub fn unlikely(_arg: !list_empty(&opp_table->lazy)) -> return;
}

extern "C" {
    pub fn _of_init_opp_table(opp_table: *mut opp_table, dev: *mut device, index: c_int);
}
extern "C" {
    pub fn _of_clear_opp_table(opp_table: *mut opp_table);
}
extern "C" {
    pub fn _of_clear_opp(opp_table: *mut opp_table, opp: *mut dev_pm_opp);
}

extern "C" {
    pub fn opp_debug_remove_one(opp: *mut dev_pm_opp);
}
extern "C" {
    pub fn opp_debug_create_one(opp: *mut dev_pm_opp, opp_table: *mut opp_table);
}
extern "C" {
    pub fn opp_debug_register(opp_dev: *mut opp_device, opp_table: *mut opp_table);
}
extern "C" {
    pub fn opp_debug_unregister(opp_dev: *mut opp_device, opp_table: *mut opp_table);
}

