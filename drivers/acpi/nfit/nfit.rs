//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/nfit/nfit.h
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
// NVDIMM Firmware Interface Table - NFIT
//
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

// ACPI 6.1

// https://pmem.io/documents/NVDIMM_DSM_Interface-V1.6.pdf

// https://github.com/HewlettPackard/hpe-nvm/blob/master/Documentation/

// https://msdn.microsoft.com/library/windows/hardware/mt604741

// http://www.uefi.org/RFIC_LIST (see "Virtual NVDIMM 0x1901")

pub const NVDIMM_CMD_MAX: c_int = 31;

//
// Command numbers that the kernel needs to know about to handle
// non-default DSM revision ids
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_family_cmds {
    NVDIMM_INTEL_LATCH_SHUTDOWN = 10,
    NVDIMM_INTEL_GET_MODES = 11,
    NVDIMM_INTEL_GET_FWINFO = 12,
    NVDIMM_INTEL_START_FWUPDATE = 13,
    NVDIMM_INTEL_SEND_FWUPDATE = 14,
    NVDIMM_INTEL_FINISH_FWUPDATE = 15,
    NVDIMM_INTEL_QUERY_FWUPDATE = 16,
    NVDIMM_INTEL_SET_THRESHOLD = 17,
    NVDIMM_INTEL_INJECT_ERROR = 18,
    NVDIMM_INTEL_GET_SECURITY_STATE = 19,
    NVDIMM_INTEL_SET_PASSPHRASE = 20,
    NVDIMM_INTEL_DISABLE_PASSPHRASE = 21,
    NVDIMM_INTEL_UNLOCK_UNIT = 22,
    NVDIMM_INTEL_FREEZE_LOCK = 23,
    NVDIMM_INTEL_SECURE_ERASE = 24,
    NVDIMM_INTEL_OVERWRITE = 25,
    NVDIMM_INTEL_QUERY_OVERWRITE = 26,
    NVDIMM_INTEL_SET_MASTER_PASSPHRASE = 27,
    NVDIMM_INTEL_MASTER_SECURE_ERASE = 28,
    NVDIMM_INTEL_FW_ACTIVATE_DIMMINFO = 29,
    NVDIMM_INTEL_FW_ACTIVATE_ARM = 30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_bus_family_cmds {
    NVDIMM_BUS_INTEL_FW_ACTIVATE_BUSINFO = 1,
    NVDIMM_BUS_INTEL_FW_ACTIVATE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_uuids {
// for simplicity alias the uuid index with the family id
    NFIT_DEV_DIMM = NVDIMM_FAMILY_INTEL,
    NFIT_DEV_DIMM_N_HPE1 = NVDIMM_FAMILY_HPE1,
    NFIT_DEV_DIMM_N_HPE2 = NVDIMM_FAMILY_HPE2,
    NFIT_DEV_DIMM_N_MSFT = NVDIMM_FAMILY_MSFT,
    NFIT_DEV_DIMM_N_HYPERV = NVDIMM_FAMILY_HYPERV,
//
// to_nfit_bus_uuid() expects to translate bus uuid family ids
// to a UUID index using NVDIMM_FAMILY_MAX as an offset
//
    NFIT_BUS_INTEL = NVDIMM_FAMILY_MAX + NVDIMM_BUS_FAMILY_INTEL,
    NFIT_SPA_VOLATILE,
    NFIT_SPA_PM,
    NFIT_SPA_DCR,
    NFIT_SPA_BDW,
    NFIT_SPA_VDISK,
    NFIT_SPA_VCD,
    NFIT_SPA_PDISK,
    NFIT_SPA_PCD,
    NFIT_DEV_BUS,
    NFIT_UUID_MAX,
}

//
// Region format interface codes are stored with the interface as the
// LSB and the function as the MSB.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_root_notifiers {
    NFIT_NOTIFY_UPDATE = 0x80,
    NFIT_NOTIFY_UC_MEMORY_ERROR = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_dimm_notifiers {
    NFIT_NOTIFY_DIMM_HEALTH = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_ars_state {
    ARS_REQ_SHORT,
    ARS_REQ_LONG,
    ARS_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_spa {
    pub list: list_head,
    pub nd_region: *mut nd_region,
    pub ars_state: c_ulong,
    pub clear_err_unit: u32,
    pub max_ars: u32,
    pub spa: [acpi_nfit_system_address; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_dcr {
    pub list: list_head,
    pub dcr: [acpi_nfit_control_region; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_bdw {
    pub list: list_head,
    pub bdw: [acpi_nfit_data_region; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_idt {
    pub list: list_head,
    pub idt: [acpi_nfit_interleave; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_flush {
    pub list: list_head,
    pub flush: [acpi_nfit_flush_address; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_memdev {
    pub list: list_head,
    pub memdev: [acpi_nfit_memory_map; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfit_mem_flags {
    NFIT_MEM_LSR,
    NFIT_MEM_LSW,
    NFIT_MEM_DIRTY,
    NFIT_MEM_DIRTY_COUNT,
}

pub const NFIT_DIMM_ID_LEN: c_int = 22;
// assembled tables for a given dimm/memory-device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_mem {
    pub nvdimm: *mut nvdimm,
    pub memdev_dcr: *mut acpi_nfit_memory_map,
    pub memdev_pmem: *mut acpi_nfit_memory_map,
    pub dcr: *mut acpi_nfit_control_region,
    pub spa_dcr: *mut acpi_nfit_system_address,
    pub idt_dcr: *mut acpi_nfit_interleave,
    pub flags_attr: *mut kernfs_node,
    pub nfit_flush: *mut nfit_flush,
    pub list: list_head,
    pub adev: *mut acpi_device,
    pub acpi_desc: *mut acpi_nfit_desc,
    pub fwa_state: nvdimm_fwa_state,
    pub fwa_result: nvdimm_fwa_result,
    pub fwa_count: c_int,
    pub id: [c_char; NFIT_DIMM_ID_LEN+1],
    pub flush_wpq: *mut resource,
    pub dsm_mask: c_ulong,
    pub flags: c_ulong,
    pub dirty_shutdown: u32,
    pub family: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scrub_flags {
    ARS_BUSY,
    ARS_CANCEL,
    ARS_VALID,
    ARS_POLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_desc {
    pub nd_desc: nvdimm_bus_descriptor,
    pub acpi_header: acpi_table_header,
    pub init_mutex: mutex,
    pub memdevs: list_head,
    pub flushes: list_head,
    pub dimms: list_head,
    pub spas: list_head,
    pub dcrs: list_head,
    pub bdws: list_head,
    pub idts: list_head,
    pub nvdimm_bus: *mut nvdimm_bus,
    pub dev: *mut device,
    pub ars_status: *mut nd_cmd_ars_status,
    pub scrub_spa: *mut nfit_spa,
    pub dwork: delayed_work,
    pub list: list_head,
    pub scrub_count_state: *mut kernfs_node,
    pub max_ars: c_uint,
    pub scrub_count: c_uint,
    pub scrub_mode: c_uint,
    pub scrub_flags: c_ulong,
    pub dimm_cmd_force_en: c_ulong,
    pub bus_cmd_force_en: c_ulong,
    pub bus_dsm_mask: c_ulong,
    pub 1]: unsigned long family_dsm_mask[NVDIMM_BUS_FAMILY_MAX +,
    pub platform_cap: c_uint,
    pub scrub_tmo: c_uint,
    pub fwa_state: nvdimm_fwa_state,
    pub fwa_cap: nvdimm_fwa_capability,
    pub fwa_count: c_int,
    pub fwa_noidle: bool,
    pub fwa_nosuspend: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scrub_mode {
    HW_ERROR_SCRUB_OFF,
    HW_ERROR_SCRUB_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nd_blk_mmio_selector {
    BDW,
    DCR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_blk_addr {
    pub base: *mut void __iomem,
    pub aperture: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_blk {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_blk_mmio {
    pub addr: nd_blk_addr,
    pub size: u64,
    pub base_offset: u64,
    pub line_size: u32,
    pub num_lines: u32,
    pub table_size: u32,
    pub idt: *mut acpi_nfit_interleave,
    pub spa: *mut acpi_nfit_system_address,
    pub mmio: [}; 2],
    pub nd_region: *mut nd_region,
    pub /: *mut *mut u64 bdw_offset; / post interleave offset,
    pub stat_offset: u64,
    pub cmd_offset: u64,
    pub dimm_flags: u32,
}

extern "C" {
    pub fn nfit_mce_register();
}
extern "C" {
    pub fn nfit_mce_unregister();
}

extern "C" {
    pub fn nfit_spa_type(spa: *mut acpi_nfit_system_address) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: nd_desc, acpi_nfit_desc: struct, _arg: nd_desc) -> return;
}
extern "C" {
    pub fn acpi_nfit_init(acpi_desc: *mut acpi_nfit_desc, nfit: *mut c_void, sz: acpi_size) -> c_int;
}
extern "C" {
    pub fn acpi_nfit_shutdown(data: *mut c_void);
}
extern "C" {
    pub fn __acpi_nfit_notify(dev: *mut device, handle: acpi_handle, event: u32);
}
extern "C" {
    pub fn __acpi_nvdimm_notify(dev: *mut device, event: u32);
}
extern "C" {
    pub fn acpi_nfit_desc_init(acpi_desc: *mut acpi_nfit_desc, dev: *mut device);
}
extern "C" {
    pub fn intel_fwa_supported(nvdimm_bus: *mut nvdimm_bus) -> bool;
}
extern "C" {
    pub fn nfit_intel_shutdown_status(nfit_mem: *mut nfit_mem);
}
