//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel_vsec.h
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
// VSEC_CAP_UNUSED is reserved. It exists to prevent zero initialized
// intel_vsec devices from being automatically set to a known
// capability with ID 0
//

pub const VSEC_FEATURE_COUNT: c_int = 7;
// Intel DVSEC offsets
pub const INTEL_DVSEC_ENTRIES: c_uint = 0xA;
pub const INTEL_DVSEC_SIZE: c_uint = 0xB;
pub const INTEL_DVSEC_TABLE: c_uint = 0xC;

pub const TABLE_OFFSET_SHIFT: c_int = 3;
pub const PMT_DISC_DWORDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_vsec_disc_source {
    INTEL_VSEC_DISC_PCI,	/* PCI, default */
    INTEL_VSEC_DISC_ACPI,	/* ACPI */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_vsec_id {
    VSEC_ID_TELEMETRY	= 2,
    VSEC_ID_WATCHER		= 3,
    VSEC_ID_CRASHLOG	= 4,
    VSEC_ID_DISCOVERY	= 12,
    VSEC_ID_SDSI		= 65,
    VSEC_ID_TPMI		= 66,
}

//
// struct intel_vsec_header - Common fields of Intel VSEC and DVSEC registers.
// @rev:         Revision ID of the VSEC/DVSEC register space
// @length:      Length of the VSEC/DVSEC register space
// @id:          ID of the feature
// @num_entries: Number of instances of the feature
// @entry_size:  Size of the discovery table for each feature
// @tbir:        BAR containing the discovery tables
// @offset:      BAR offset of start of the first discovery table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vsec_header {
    pub rev: u8,
    pub length: u16,
    pub id: u16,
    pub num_entries: u8,
    pub entry_size: u8,
    pub tbir: u8,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_vsec_quirks {
// Watcher feature not supported
    VSEC_QUIRK_NO_WATCHER	= BIT(0),

// Crashlog feature not supported
    VSEC_QUIRK_NO_CRASHLOG	= BIT(1),

// Use shift instead of mask to read discovery table offset
    VSEC_QUIRK_TABLE_SHIFT	= BIT(2),

// DVSEC not present (provided in driver data)
    VSEC_QUIRK_NO_DVSEC	= BIT(3),

// Platforms requiring quirk in the auxiliary driver
    VSEC_QUIRK_EARLY_HW     = BIT(4),
}

//
// struct pmt_callbacks - Callback infrastructure for PMT devices
// @read_telem: when specified, called by client driver to access PMT
// data (instead of direct copy).
// * dev:   device reference for the callback's use
// * guid:  ID of data to acccss
// * data:  buffer for the data to be copied
// * off:   offset into the requested buffer
// * count: size of buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_callbacks {
    pub count): *mut *mut *mut *mut int (read_telem)(struct device dev, u32 guid, u64 data, loff_t off, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsec_feature_dependency {
    pub feature: c_ulong,
    pub supplier_bitmap: c_ulong,
}

//
// struct intel_vsec_platform_info - Platform specific data
// @parent:    parent device in the auxbus chain
// @headers:   list of headers to define the PMT client devices to create
// @deps:      array of feature dependencies
// @acpi_disc: ACPI discovery tables, each entry is two QWORDs
// in little-endian format as defined by the PMT ACPI spec.
// Valid only when @provider == INTEL_VSEC_DISC_ACPI.
// @src:       source of discovery table data
// @priv_data: private data, usable by parent devices, currently a callback
// @caps:      bitmask of PMT capabilities for the given headers
// @quirks:    bitmask of VSEC device quirks
// @base_addr: allow a base address to be specified (rather than derived)
// @num_deps:  Count feature dependencies
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vsec_platform_info {
    pub parent: *mut device,
    pub headers: *mut intel_vsec_header,
    pub deps: *const vsec_feature_dependency,
    pub (*acpi_disc)[PMT_DISC_DWORDS]: *mut u32,
    pub src: intel_vsec_disc_source,
    pub priv_data: *mut c_void,
    pub caps: c_ulong,
    pub quirks: c_ulong,
    pub base_addr: u64,
    pub num_deps: c_int,
}

//
// struct intel_vsec_device - Auxbus specific device information
// @auxdev:        auxbus device struct for auxbus access
// @dev:           struct device associated with the device
// @acpi_disc:     ACPI discovery tables, each entry is two QWORDs
// in little-endian format as defined by the PMT ACPI spec.
// Valid only when @src == INTEL_VSEC_DISC_ACPI.
// @src:           source of discovery table data
// @ida:           id reference
// @num_resources: number of resources
// @id:            xarray id
// @priv_data:     any private data needed
// @priv_data_size: size of private data area
// @quirks:        specified quirks
// @base_addr:     base address of entries (if specified)
// @cap_id:        the enumerated id of the vsec feature
// @resource:      PCI discovery resources (BAR windows), one per discovery
// instance. Valid only when @src == INTEL_VSEC_DISC_PCI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vsec_device {
    pub auxdev: auxiliary_device,
    pub dev: *mut device,
    pub (*acpi_disc)[PMT_DISC_DWORDS]: *mut u32,
    pub src: intel_vsec_disc_source,
    pub ida: *mut ida,
    pub num_resources: c_int,
    pub /: *mut *mut int id; / xa,
    pub priv_data: *mut c_void,
    pub priv_data_size: usize,
    pub quirks: c_ulong,
    pub base_addr: u64,
    pub cap_id: c_ulong,
    pub __counted_by(num_resources): resource resource[],
}

//
// struct oobmsm_plat_info - Platform information for a device instance
// @cdie_mask:       Mask of all compute dies in the partition
// @package_id:      CPU Package id
// @partition:       Package partition id when multiple VSEC PCI devices per package
// @segment:         PCI segment ID
// @bus_number:      PCI bus number
// @device_number:   PCI device number
// @function_number: PCI function number
//
// Structure to store platform data for a OOBMSM device instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oobmsm_plat_info {
    pub cdie_mask: u16,
    pub package_id: u8,
    pub partition: u8,
    pub segment: u8,
    pub bus_number: u8,
    pub device_number: u8,
    pub function_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telemetry_region {
    pub plat_info: oobmsm_plat_info,
    pub addr: *mut void __iomem,
    pub size: usize,
    pub guid: u32,
    pub num_rmids: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_feature_group {
    pub id: pmt_feature_id,
    pub count: c_int,
    pub kref: kref,
    pub regions: [telemetry_region; ],
}

extern "C" {
    pub fn container_of(_arg: dev, intel_vsec_device: struct, _arg: auxdev.dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: auxdev, intel_vsec_device: struct, _arg: auxdev) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn intel_pmt_put_feature_group(feature_group: *mut pmt_feature_group);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

