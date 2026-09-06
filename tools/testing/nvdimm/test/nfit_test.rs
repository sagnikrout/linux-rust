//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/nvdimm/test/nfit_test.h
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
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_test_request {
    pub list: list_head,
    pub res: resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_test_resource {
    pub requests: list_head,
    pub list: list_head,
    pub res: resource,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub req_count: c_int,
    pub buf: *mut c_void,
}

pub const ND_TRANSLATE_SPA_STATUS_INVALID_SPA: c_int = 2;
pub const NFIT_ARS_INJECT_INVALID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum err_inj_options {
    ND_ARS_ERR_INJ_OPT_NOTIFY = 0,
}

// nfit commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_cmd_num {
    NFIT_CMD_TRANSLATE_SPA = 5,
    NFIT_CMD_ARS_INJECT_SET = 7,
    NFIT_CMD_ARS_INJECT_CLEAR = 8,
    NFIT_CMD_ARS_INJECT_GET = 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_translate_spa {
    pub spa: __u64,
    pub status: __u32,
    pub flags: __u8,
    pub _reserved: [__u8; 3],
    pub translate_length: __u64,
    pub num_nvdimms: __u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_nvdimm_device {
    pub nfit_device_handle: __u32,
    pub _reserved: __u32,
    pub dpa: __u64,
    pub devices: [} __packed; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_err_inj {
    pub err_inj_spa_range_base: __u64,
    pub err_inj_spa_range_length: __u64,
    pub err_inj_options: __u8,
    pub status: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_err_inj_clr {
    pub err_inj_clr_spa_range_base: __u64,
    pub err_inj_clr_spa_range_length: __u64,
    pub status: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_ars_err_inj_stat {
    pub status: __u32,
    pub inj_err_rec_count: __u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_error_stat_query_record {
    pub err_inj_stat_spa_range_base: __u64,
    pub err_inj_stat_spa_range_length: __u64,
    pub record: [} __packed; ],
    pub __packed: },
pub const ND_INTEL_SMART: c_int = 1;
pub const ND_INTEL_SMART_THRESHOLD: c_int = 2;
pub const ND_INTEL_ENABLE_LSS_STATUS: c_int = 10;
pub const ND_INTEL_FW_GET_INFO: c_int = 12;
pub const ND_INTEL_FW_START_UPDATE: c_int = 13;
pub const ND_INTEL_FW_SEND_DATA: c_int = 14;
pub const ND_INTEL_FW_FINISH_UPDATE: c_int = 15;
pub const ND_INTEL_FW_FINISH_QUERY: c_int = 16;
pub const ND_INTEL_SMART_SET_THRESHOLD: c_int = 17;
pub const ND_INTEL_SMART_INJECT: c_int = 18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_smart_threshold {
    pub status: __u32,
    pub alarm_control: __u16,
    pub spares: __u8,
    pub media_temperature: __u16,
    pub ctrl_temperature: __u16,
    pub reserved: [__u8; 1],
    pub __packed: },
    pub data: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_smart_set_threshold {
    pub alarm_control: __u16,
    pub spares: __u8,
    pub media_temperature: __u16,
    pub ctrl_temperature: __u16,
    pub status: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_smart_inject {
    pub flags: __u64,
    pub mtemp_enable: __u8,
    pub media_temperature: __u16,
    pub spare_enable: __u8,
    pub spares: __u8,
    pub fatal_enable: __u8,
    pub unsafe_shutdown_enable: __u8,
    pub status: __u32,
    pub __packed: },
pub const INTEL_FW_STORAGE_SIZE: c_uint = 0x100000;
pub const INTEL_FW_MAX_SEND_LEN: c_uint = 0xFFEC;
pub const INTEL_FW_QUERY_INTERVAL: c_int = 250000;
pub const INTEL_FW_QUERY_MAX_TIME: c_int = 3000000;
pub const INTEL_FW_FIS_VERSION: c_uint = 0x0105;
pub const INTEL_FW_FAKE_VERSION: c_uint = 0xffffffffabcd;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_fw_update_state {
    FW_STATE_NEW = 0,
    FW_STATE_IN_PROGRESS,
    FW_STATE_VERIFY,
    FW_STATE_UPDATED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_info {
    pub status: __u32,
    pub storage_size: __u32,
    pub max_send_len: __u32,
    pub query_interval: __u32,
    pub max_query_time: __u32,
    pub update_cap: __u8,
    pub reserved: [__u8; 3],
    pub fis_version: __u32,
    pub run_version: __u64,
    pub updated_version: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_start {
    pub status: __u32,
    pub context: __u32,
    pub __packed: },
// this one has the output first because the variable input data size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_send_data {
    pub context: __u32,
    pub offset: __u32,
    pub length: __u32,
    pub data: [__u8; ],
// this field is not declared due ot variable data from input
// __u32 status;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_finish_update {
    pub ctrl_flags: __u8,
    pub reserved: [__u8; 3],
    pub context: __u32,
    pub status: __u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_fw_finish_query {
    pub context: __u32,
    pub status: __u32,
    pub updated_fw_rev: __u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_intel_lss {
    pub enable: __u8,
    pub status: __u32,
    pub __packed: },
    pub (*nfit_test_lookup_fn)(resource_size_t): *mut typedef struct nfit_test_resource,
    pub argv4): *mut acpi_object,
    pub size): resource_size_t offset, unsigned long,
    pub flags): size_t size, unsigned long,
    pub pgmap): *mut *mut *mut void __wrap_devm_memremap_pages(struct device dev, struct dev_pagemap,
    pub flags): c_ulong,
    pub addr): *mut *mut void __wrap_devm_memunmap(struct device dev, void,
    pub size): *mut *mut void __iomem __wrap_ioremap(resource_size_t offset, unsigned long,
    pub size): *mut *mut void __iomem __wrap_ioremap_wc(resource_size_t offset, unsigned long,
    pub addr): *mut void __wrap_iounmap(volatile void __iomem,
    pub addr): *mut void __wrap_memunmap(void,
    pub flags): c_int,
    pub res): *mut *mut int __wrap_insert_resource(struct resource parent, struct resource,
    pub res): *mut int __wrap_remove_resource(struct resource,
    pub name): *const resource_size_t n, char,
    pub n): resource_size_t,
    pub n): resource_size_t start, resource_size_t,
    pub buf): *mut *mut acpi_object_list p, acpi_buffer,
    pub argv4): *mut u64 rev, u64 func, union acpi_object,
    pub evaluate): nfit_test_evaluate_dsm_fn,
    pub nfit_test_teardown(void): c_void,
    pub resource): *mut *mut nfit_test_resource get_nfit_res(resource_size_t,
