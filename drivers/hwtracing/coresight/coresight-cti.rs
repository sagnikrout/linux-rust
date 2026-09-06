//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-cti.h
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
// Copyright (c) 2018 Linaro Limited, All rights reserved.
// Author: Mike Leach <mike.leach@linaro.org>
//

//
// Device registers
// 0x000 - 0x144: CTI programming and status
// 0xEDC - 0xEF8: CTI integration test.
// 0xF00 - 0xFFC: Coresight management registers.
//
// CTI programming registers
pub const CTICONTROL: c_uint = 0x000;
pub const CTIINTACK: c_uint = 0x010;
pub const CTIAPPSET: c_uint = 0x014;
pub const CTIAPPCLEAR: c_uint = 0x018;
pub const CTIAPPPULSE: c_uint = 0x01C;

pub const CTITRIGINSTATUS: c_uint = 0x130;
pub const CTITRIGOUTSTATUS: c_uint = 0x134;
pub const CTICHINSTATUS: c_uint = 0x138;
pub const CTICHOUTSTATUS: c_uint = 0x13C;
pub const CTIGATE: c_uint = 0x140;
pub const ASICCTL: c_uint = 0x144;
// Integration test registers
pub const ITCHINACK: c_uint = 0xEDC /* WO CTI CSSoc 400 only*/;
pub const ITTRIGINACK: c_uint = 0xEE0 /* WO CTI CSSoc 400 only*/;
pub const ITCHOUT: c_uint = 0xEE4 /* WO RW-600 */;
pub const ITTRIGOUT: c_uint = 0xEE8 /* WO RW-600 */;
pub const ITCHOUTACK: c_uint = 0xEEC /* RO CTI CSSoc 400 only*/;
pub const ITTRIGOUTACK: c_uint = 0xEF0 /* RO CTI CSSoc 400 only*/;
pub const ITCHIN: c_uint = 0xEF4 /* RO */;
pub const ITTRIGIN: c_uint = 0xEF8 /* RO */;
// management registers
pub const CTIDEVAFF0: c_uint = 0xFA8;
pub const CTIDEVAFF1: c_uint = 0xFAC;
//
// CTI CSSoc 600 has a max of 32 trigger signals per direction.
// CTI CSSoc 400 has 8 IO triggers - other CTIs can be impl def.
// Max of in and out defined in the DEVID register.
// - pick up actual number used from .dts parameters if present.
//
pub const CTIINOUTEN_MAX: c_int = 32;
//
// Group of related trigger signals
//
// @nr_sigs: number of signals in the group.
// @used_mask: bitmask representing the signal indexes in the group.
// @sig_types: array of types for the signals, length nr_sigs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_trig_grp {
    pub nr_sigs: c_int,
    pub used_mask: u32,
    pub sig_types: [c_int; ],
}

//
// Trigger connection - connection between a CTI and other (coresight) device
// lists input and output trigger signals for the device
//
// @con_in: connected CTIIN signals for the device.
// @con_out: connected CTIOUT signals for the device.
// @con_dev: coresight device connected to the CTI, NULL if not CS device
// @con_dev_name: name of connected device (CS or CPU)
// @node: entry node in list of connections.
// @con_attrs: Dynamic sysfs attributes specific to this connection.
// @attr_group: Dynamic attribute group created for this connection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_trig_con {
    pub con_in: *mut cti_trig_grp,
    pub con_out: *mut cti_trig_grp,
    pub con_dev: *mut coresight_device,
    pub con_dev_name: *const c_char,
    pub node: list_head,
    pub con_attrs: *mut attribute,
    pub attr_group: *mut attribute_group,
}

//
// struct cti_device - description of CTI device properties.
//
// @nt_trig_con: Number of external devices connected to this device.
// @ctm_id: which CTM this device is connected to (by default it is
// assumed there is a single CTM per SoC, ID 0).
// @trig_cons: list of connections to this device.
// @cpu: CPU ID if associated with CPU, -1 otherwise.
// @con_groups: combined static and dynamic sysfs groups for trigger
// connections.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_device {
    pub nr_trig_con: c_int,
    pub ctm_id: u32,
    pub trig_cons: list_head,
    pub cpu: c_int,
    pub con_groups: *const attribute_group,
}

//
// struct cti_config - configuration of the CTI device hardware
//
// @nr_trig_max: Max number of trigger signals implemented on device.
// (max of trig_in or trig_out) - from ID register.
// @nr_ctm_channels: number of available CTM channels - from ID register.
// @asicctl_impl: true if asicctl is implemented.
// @enable_req_count: CTI is enabled alongside >=1 associated devices.
// @trig_in_use: bitfield of in triggers registered as in use.
// @trig_out_use: bitfield of out triggers registered as in use.
// @trig_out_filter: bitfield of out triggers that are blocked if filter
// enabled. Typically this would be dbgreq / restart on
// a core CTI.
// @trig_filter_enable: 1 if filtering enabled.
// @xtrig_rchan_sel: channel selection for xtrigger connection show.
// @ctiappset: CTI Software application channel set.
// @ctiinout_sel: register selector for INEN and OUTEN regs.
// @ctiinen: enable input trigger to a channel.
// @ctiouten: enable output trigger from a channel.
// @ctigate: gate channel output from CTI to CTM.
// @asicctl: asic control register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_config {
// hardware description
    pub nr_ctm_channels: c_int,
    pub nr_trig_max: c_int,
    pub asicctl_impl: bool,
// cti enable control
    pub enable_req_count: c_int,
// registered triggers and filtering
    pub trig_in_use: u32,
    pub trig_out_use: u32,
    pub trig_out_filter: u32,
    pub trig_filter_enable: bool,
    pub xtrig_rchan_sel: u8,
// cti cross trig programmable regs
    pub ctiappset: u32,
    pub ctiinout_sel: u8,
    pub ctiinen: [u32; CTIINOUTEN_MAX],
    pub ctiouten: [u32; CTIINOUTEN_MAX],
    pub ctigate: u32,
    pub asicctl: u32,
}

//
// struct cti_drvdata - specifics for the CTI device
// @base:	Memory mapped base address for this component..
// @csdev:	Standard CoreSight device information.
// @ctidev:	Extra information needed by the CTI/CTM framework.
// @spinlock:	Control data access to one at a time.
// @config:	Configuration data for this CTI device.
// @node:	List entry of this device in the list of CTI devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cti_drvdata {
    pub base: *mut void __iomem,
    pub csdev: *mut coresight_device,
    pub ctidev: cti_device,
    pub spinlock: raw_spinlock_t,
    pub config: cti_config,
    pub node: list_head,
}

//
// Channel operation types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cti_chan_op {
    CTI_CHAN_ATTACH,
    CTI_CHAN_DETACH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cti_trig_dir {
    CTI_TRIG_IN,
    CTI_TRIG_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cti_chan_gate_op {
    CTI_GATE_CHAN_ENABLE,
    CTI_GATE_CHAN_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cti_chan_set_op {
    CTI_CHAN_SET,
    CTI_CHAN_CLR,
    CTI_CHAN_PULSE,
}

// private cti driver fns & vars
extern "C" {
    pub fn cti_disable(csdev: *mut coresight_device, path: *mut coresight_path) -> c_int;
}
extern "C" {
    pub fn cti_write_all_hw_regs(drvdata: *mut cti_drvdata);
}
extern "C" {
    pub fn cti_write_intack(dev: *mut device, ackval: u32);
}
extern "C" {
    pub fn cti_write_single_reg(drvdata: *mut cti_drvdata, offset: c_int, value: u32);
}
extern "C" {
    pub fn cti_read_single_reg(drvdata: *mut cti_drvdata, offset: c_int) -> u32;
}
extern "C" {
    pub fn cti_create_cons_sysfs(dev: *mut device, drvdata: *mut cti_drvdata) -> c_int;
}
// Check if a cti device is enabled
