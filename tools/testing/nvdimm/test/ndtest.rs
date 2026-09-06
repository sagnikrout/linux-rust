//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/nvdimm/test/ndtest.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_priv {
    pub pdev: platform_device,
    pub dn: *mut device_node,
    pub resources: list_head,
    pub bus_desc: nvdimm_bus_descriptor,
    pub bus: *mut nvdimm_bus,
    pub config: *mut ndtest_config,
    pub dcr_dma: *mut dma_addr_t,
    pub label_dma: *mut dma_addr_t,
    pub dimm_dma: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_blk_mmio {
    pub base: *mut void __iomem,
    pub size: u64,
    pub base_offset: u64,
    pub line_size: u32,
    pub num_lines: u32,
    pub table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_dimm {
    pub dev: *mut device,
    pub nvdimm: *mut nvdimm,
    pub mmio: *mut ndtest_blk_mmio,
    pub blk_region: *mut nd_region,
    pub address: dma_addr_t,
    pub flags: c_ulonglong,
    pub config_size: c_ulong,
    pub label_area: *mut c_void,
    pub uuid_str: *mut c_char,
    pub size: c_uint,
    pub handle: c_uint,
    pub fail_cmd: c_uint,
    pub physical_id: c_uint,
    pub num_formats: c_uint,
    pub id: c_int,
    pub fail_cmd_code: c_int,
    pub no_alias: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_mapping {
    pub start: u64,
    pub size: u64,
    pub position: u8,
    pub dimm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_region {
    pub region: *mut nd_region,
    pub mapping: *mut ndtest_mapping,
    pub size: u64,
    pub type: u8,
    pub num_mappings: u8,
    pub range_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ndtest_config {
    pub dimms: *mut ndtest_dimm,
    pub regions: *mut ndtest_region,
    pub dimm_count: c_uint,
    pub dimm_start: c_uint,
    pub num_regions: u8,
}
