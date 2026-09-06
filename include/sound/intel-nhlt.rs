//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/intel-nhlt.h
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
// intel-nhlt.h - Intel HDA Platform NHLT header
//
// Copyright (c) 2015-2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nhlt_link_type {
    NHLT_LINK_HDA = 0,
    NHLT_LINK_DSP = 1,
    NHLT_LINK_DMIC = 2,
    NHLT_LINK_SSP = 3,
    NHLT_LINK_INVALID
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nhlt_device_type {
    NHLT_DEVICE_BT = 0,
    NHLT_DEVICE_DMIC = 1,
    NHLT_DEVICE_I2S = 4,
    NHLT_DEVICE_INVALID
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wav_fmt {
    pub fmt_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wav_fmt_ext {
    pub fmt: wav_fmt,
#[repr(C)]
#[derive(Copy, Clone)]
pub union samples {
    pub valid_bits_per_sample: u16,
    pub samples_per_block: u16,
    pub reserved: u16,
    pub sample: },
    pub channel_mask: u32,
    pub sub_fmt: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_specific_cfg {
    pub size: u32,
    pub caps: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_fmt_cfg {
    pub fmt_ext: wav_fmt_ext,
    pub config: nhlt_specific_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_fmt {
    pub fmt_count: u8,
    pub fmt_config: [nhlt_fmt_cfg; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_endpoint {
    pub length: u32,
    pub linktype: u8,
    pub instance_id: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u16,
    pub subsystem_id: u32,
    pub device_type: u8,
    pub direction: u8,
    pub virtual_bus_id: u8,
    pub config: nhlt_specific_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_acpi_table {
    pub header: acpi_table_header,
    pub endpoint_count: u8,
    pub desc: [nhlt_endpoint; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_resource_desc {
    pub extra: u32,
    pub flags: u16,
    pub addr_spc_gra: u64,
    pub min_addr: u64,
    pub max_addr: u64,
    pub addr_trans_offset: u64,
    pub length: u64,
    pub __packed: },
pub const MIC_ARRAY_2CH: c_int = 2;
pub const MIC_ARRAY_4CH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_device_specific_config {
    pub virtual_slot: u8,
    pub config_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_dmic_array_config {
    pub device_config: nhlt_device_specific_config,
    pub array_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nhlt_vendor_dmic_array_config {
    pub dmic_config: nhlt_dmic_array_config,
    pub nb_mics: u8,
// TODO add vendor mic config
    pub __packed: },
}

extern "C" {
    pub fn intel_nhlt_free(addr: *mut nhlt_acpi_table);
}
extern "C" {
    pub fn intel_nhlt_get_dmic_geo(dev: *mut device, nhlt: *mut nhlt_acpi_table) -> c_int;
}
extern "C" {
    pub fn intel_nhlt_has_endpoint_type(nhlt: *mut nhlt_acpi_table, link_type: u8) -> bool;
}
extern "C" {
    pub fn intel_nhlt_ssp_endpoint_mask(nhlt: *mut nhlt_acpi_table, device_type: u8) -> c_int;
}
extern "C" {
    pub fn intel_nhlt_ssp_mclk_mask(nhlt: *mut nhlt_acpi_table, ssp_num: c_int) -> c_int;
}

