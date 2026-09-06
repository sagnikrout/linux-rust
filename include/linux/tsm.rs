//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tsm.h
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

pub const TSM_REPORT_INBLOB_MAX: c_int = 64;

//
// Privilege level is a nested permission concept to allow confidential
// guests to partition address space, 4-levels are supported.
//
pub const TSM_REPORT_PRIVLEVEL_MAX: c_int = 3;
//
// struct tsm_report_desc - option descriptor for generating tsm report blobs
// @privlevel: optional privilege level to associate with @outblob
// @inblob_len: sizeof @inblob
// @inblob: arbitrary input data
// @service_provider: optional name of where to obtain the tsm report blob
// @service_guid: optional service-provider service guid to attest
// @service_manifest_version: optional service-provider service manifest version requested
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_report_desc {
    pub privlevel: c_uint,
    pub inblob_len: usize,
    pub inblob: [u8; TSM_REPORT_INBLOB_MAX],
    pub service_provider: *mut c_char,
    pub service_guid: guid_t,
    pub service_manifest_version: c_uint,
}

//
// struct tsm_report - track state of report generation relative to options
// @desc: input parameters to @report_new()
// @outblob_len: sizeof(@outblob)
// @outblob: generated evidence to provider to the attestation agent
// @auxblob_len: sizeof(@auxblob)
// @auxblob: (optional) auxiliary data to the report (e.g. certificate data)
// @manifestblob_len: sizeof(@manifestblob)
// @manifestblob: (optional) manifest data associated with the report
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_report {
    pub desc: tsm_report_desc,
    pub outblob_len: usize,
    pub outblob: *mut u8,
    pub auxblob_len: usize,
    pub auxblob: *mut u8,
    pub manifestblob_len: usize,
    pub manifestblob: *mut u8,
}

//
// enum tsm_attr_index - index used to reference report attributes
// @TSM_REPORT_GENERATION: index of the report generation number attribute
// @TSM_REPORT_PROVIDER: index of the provider name attribute
// @TSM_REPORT_PRIVLEVEL: index of the desired privilege level attribute
// @TSM_REPORT_PRIVLEVEL_FLOOR: index of the minimum allowed privileg level attribute
// @TSM_REPORT_SERVICE_PROVIDER: index of the service provider identifier attribute
// @TSM_REPORT_SERVICE_GUID: index of the service GUID attribute
// @TSM_REPORT_SERVICE_MANIFEST_VER: index of the service manifest version attribute
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsm_attr_index {
    TSM_REPORT_GENERATION,
    TSM_REPORT_PROVIDER,
    TSM_REPORT_PRIVLEVEL,
    TSM_REPORT_PRIVLEVEL_FLOOR,
    TSM_REPORT_SERVICE_PROVIDER,
    TSM_REPORT_SERVICE_GUID,
    TSM_REPORT_SERVICE_MANIFEST_VER,
}

//
// enum tsm_bin_attr_index - index used to reference binary report attributes
// @TSM_REPORT_INBLOB: index of the binary report input attribute
// @TSM_REPORT_OUTBLOB: index of the binary report output attribute
// @TSM_REPORT_AUXBLOB: index of the binary auxiliary data attribute
// @TSM_REPORT_MANIFESTBLOB: index of the binary manifest data attribute
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsm_bin_attr_index {
    TSM_REPORT_INBLOB,
    TSM_REPORT_OUTBLOB,
    TSM_REPORT_AUXBLOB,
    TSM_REPORT_MANIFESTBLOB,
}

//
// struct tsm_report_ops - attributes and operations for tsm_report instances
// @name: tsm id reflected in /sys/kernel/config/tsm/report/$report/provider
// @privlevel_floor: convey base privlevel for nested scenarios
// @report_new: Populate @report with the report blob and auxblob
// (optional), return 0 on successful population, or -errno otherwise
// @report_attr_visible: show or hide a report attribute entry
// @report_bin_attr_visible: show or hide a report binary attribute entry
//
// Implementation specific ops, only one is expected to be registered at
// a time i.e. only one of "sev-guest", "tdx-guest", etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_report_ops {
    pub name: *const c_char,
    pub privlevel_floor: c_uint,
    pub data): *mut *mut *mut int (report_new)(struct tsm_report report, void,
    pub n): *mut *mut bool (report_attr_visible)(int,
    pub n): *mut *mut bool (report_bin_attr_visible)(int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_dev {
    pub dev: device,
    pub id: c_int,
    pub pci_ops: *const pci_tsm_ops,
}

extern "C" {
    pub fn tsm_report_register(ops: *const tsm_report_ops, priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tsm_report_unregister(ops: *const tsm_report_ops) -> c_int;
}
extern "C" {
    pub fn tsm_unregister(tsm_dev: *mut tsm_dev);
}
