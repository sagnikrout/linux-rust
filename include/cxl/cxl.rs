//! Automatically rewritten from C Header to Rust Module
//! Source: include/cxl/cxl.h
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
// Copyright(c) 2020 Intel Corporation.
// Copyright(c) 2026 Advanced Micro Devices, Inc.

//
// enum cxl_devtype - delineate type-2 from a generic type-3 device
// @CXL_DEVTYPE_DEVMEM: Vendor specific CXL Type-2 device implementing HDM-D or
// HDM-DB, no requirement that this device implements a
// mailbox, or other memory-device-standard manageability
// flows.
// @CXL_DEVTYPE_CLASSMEM: Common class definition of a CXL Type-3 device with
// HDM-H and class-mandatory memory device registers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_devtype {
    CXL_DEVTYPE_DEVMEM,
    CXL_DEVTYPE_CLASSMEM,
}

//
// Using struct_group() allows for per register-block-type helper routines,
// without requiring block-type agnostic code to include the prefix.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_regs {
//
// Common set of CXL Component register block base pointers
// @hdm_decoder: CXL 2.0 8.2.5.12 CXL HDM Decoder Capability Structure
// @ras: CXL 2.0 8.2.5.9 CXL RAS Capability Structure
//
    pub hdm_decoder: *mut void __iomem,
    pub ras: *mut void __iomem,
//
// Common set of CXL Device register block base pointers
// @status: CXL 2.0 8.2.8.3 Device Status Registers
// @mbox: CXL 2.0 8.2.8.4 Mailbox Registers
// @memdev: CXL 2.0 8.2.8.5 Memory Device Registers
//
    pub memdev: *mut *mut *mut void __iomem status, mbox,,
    pub pmu: *mut void __iomem,
//
// RCH downstream port specific RAS register
// @aer: CXL 3.0 8.2.1.1 RCH Downstream Port RCRB
//
    pub dport_aer: *mut void __iomem,
//
// RCD upstream port specific PCIe cap register
// @pcie_cap: CXL 3.0 8.2.1.2 RCD Upstream Port RCRB
//
    pub rcd_pcie_cap: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_reg_map {
    pub valid: bool,
    pub id: c_int,
    pub offset: c_ulong,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_component_reg_map {
    pub hdm_decoder: cxl_reg_map,
    pub ras: cxl_reg_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_device_reg_map {
    pub status: cxl_reg_map,
    pub mbox: cxl_reg_map,
    pub memdev: cxl_reg_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_pmu_reg_map {
    pub pmu: cxl_reg_map,
}

//
// struct cxl_register_map - DVSEC harvested register block mapping parameters
// @host: device for devm operations and logging
// @base: virtual base of the register-block-BAR + @block_offset
// @resource: physical resource base of the register block
// @max_size: maximum mapping size to perform register search
// @reg_type: see enum cxl_regloc_type
// @component_map: cxl_reg_map for component registers
// @device_map: cxl_reg_maps for device registers
// @pmu_map: cxl_reg_maps for CXL Performance Monitoring Units
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_register_map {
    pub host: *mut device,
    pub base: *mut void __iomem,
    pub resource: resource_size_t,
    pub max_size: resource_size_t,
    pub reg_type: u8,
    pub component_map: cxl_component_reg_map,
    pub device_map: cxl_device_reg_map,
    pub pmu_map: cxl_pmu_reg_map,
}

//
// struct cxl_dpa_perf - DPA performance property entry
// @dpa_range: range for DPA address
// @coord: QoS performance data (i.e. latency, bandwidth)
// @cdat_coord: raw QoS performance data from CDAT
// @qos_class: QoS Class cookies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dpa_perf {
    pub dpa_range: range,
    pub coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub cdat_coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub qos_class: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_partition_mode {
    CXL_PARTMODE_RAM,
    CXL_PARTMODE_PMEM,
}

//
// struct cxl_dpa_partition - DPA partition descriptor
// @res: shortcut to the partition in the DPA resource tree (cxlds->dpa_res)
// @perf: performance attributes of the partition from CDAT
// @mode: operation mode for the DPA capacity, e.g. ram, pmem, dynamic...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dpa_partition {
    pub res: resource,
    pub perf: cxl_dpa_perf,
    pub mode: cxl_partition_mode,
}

pub const CXL_NR_PARTITIONS_MAX: c_int = 2;
//
// struct cxl_dev_state - The driver device state
//
// cxl_dev_state represents the CXL driver/device state.  It provides an
// interface to mailbox commands as well as some cached data about the device.
// Currently only memory devices are represented.
//
// @dev: The device associated with this CXL state
// @cxlmd: The device representing the CXL.mem capabilities of @dev
// @reg_map: component and ras register mapping parameters
// @regs: Parsed register blocks
// @cxl_dvsec: Offset to the PCIe device DVSEC
// @rcd: operating in RCD mode (CXL 3.0 9.11.8 CXL Devices Attached to an RCH)
// @media_ready: Indicate whether the device media is usable
// @dpa_res: Overall DPA resource tree for the device
// @part: DPA partition array
// @nr_partitions: Number of DPA partitions
// @serial: PCIe Device Serial Number
// @type: Generic Memory Class device or Vendor Specific Memory device
// @cxl_mbox: CXL mailbox context
// @cxlfs: CXL features context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dev_state {
// public for Type2 drivers
    pub dev: *mut device,
    pub cxlmd: *mut cxl_memdev,
// private for Type2 drivers
    pub reg_map: cxl_register_map,
    pub regs: cxl_device_regs,
    pub cxl_dvsec: c_int,
    pub rcd: bool,
    pub media_ready: bool,
    pub dpa_res: resource,
    pub part: [cxl_dpa_partition; CXL_NR_PARTITIONS_MAX],
    pub nr_partitions: c_uint,
    pub serial: u64,
    pub type: cxl_devtype,
    pub cxl_mbox: cxl_mailbox,

    pub cxlfs: *mut cxl_features_state,

}

//
// cxl_dev_state_create - safely create and cast a cxl dev state embedded in a
// driver specific struct.
//
// @parent: device behind the request
// @type: CXL device type
// @serial: device identification
// @dvsec: dvsec capability offset
// @drv_struct: driver struct embedding a cxl_dev_state struct
// @member: name of the struct cxl_dev_state member in drv_struct
// @mbox: true if mailbox supported
//
// Returns a pointer to the drv_struct allocated and embedding a cxl_dev_state
// struct initialized.
//
// Introduced for Type2 driver support.
//

extern "C" {
    pub fn cxl_set_capacity(cxlds: *mut cxl_dev_state, capacity: u64) -> c_int;
}
