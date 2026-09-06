//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/actbl2.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: actbl2.h - ACPI Table Definitions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Additional ACPI Tables (2)
//
// These tables are not consumed directly by the ACPICA subsystem, but are
// included here to support device drivers and the AML disassembler.
//
// Values for description table header signatures for tables defined in this
// file. Useful because they make it more difficult to inadvertently type in
// the wrong signature.
//

//
// All tables must be byte-packed to match the ACPI specification, since
// the tables are provided by the system BIOS.
//

//
// Note: C bitfields are not used for this reason:
//
// "Bitfields are great and easy to read, but unfortunately the C language
// does not specify the layout of bitfields in memory, which means they are
// essentially useless for dealing with packed data in on-disk formats or
// binary wire protocols." (Or ACPI tables and buffers.) "If you ask me,
// this decision was a design error in C. Ritchie could have picked an order
// and stuck with it." Norman Ramsey.
// See http://stackoverflow.com/a/1053662/41661
//
// AEST - Arm Error Source Table
//
// Conforms to: ACPI for the Armv8 RAS Extensions 1.1(Sep 2020) and
// 2.0(May 2023) Platform Design Document.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_aest {
    pub header: acpi_table_header,
}

// Common Subtable header - one per Node Structure (Subtable)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_hdr {
    pub type: u8,
    pub length: u16,
    pub reserved: u8,
    pub node_specific_offset: u32,
    pub node_interface_offset: u32,
    pub node_interrupt_offset: u32,
    pub node_interrupt_count: u32,
    pub timestamp_rate: u64,
    pub reserved1: u64,
    pub error_injection_rate: u64,
}

// Values for Type above
pub const ACPI_AEST_PROCESSOR_ERROR_NODE: c_int = 0;
pub const ACPI_AEST_MEMORY_ERROR_NODE: c_int = 1;
pub const ACPI_AEST_SMMU_ERROR_NODE: c_int = 2;
pub const ACPI_AEST_VENDOR_ERROR_NODE: c_int = 3;
pub const ACPI_AEST_GIC_ERROR_NODE: c_int = 4;
pub const ACPI_AEST_PCIE_ERROR_NODE: c_int = 5;
pub const ACPI_AEST_PROXY_ERROR_NODE: c_int = 6;

//
// AEST subtables (Error nodes)
//
// 0: Processor Error
// Values for resource_type above, related structs below
pub const ACPI_AEST_CACHE_RESOURCE: c_int = 0;
pub const ACPI_AEST_TLB_RESOURCE: c_int = 1;
pub const ACPI_AEST_GENERIC_RESOURCE: c_int = 2;

// 0R: Processor Cache Resource Substructure
// Values for cache_type above
pub const ACPI_AEST_CACHE_DATA: c_int = 0;
pub const ACPI_AEST_CACHE_INSTRUCTION: c_int = 1;
pub const ACPI_AEST_CACHE_UNIFIED: c_int = 2;

// 1R: Processor TLB Resource Substructure
// 2R: Processor Generic Resource Substructure
// 1: Memory Error
// 2: Smmu Error
// 3: Vendor Defined
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_vendor_v2 {
    pub acpi_hid: [c_char; 8],
    pub acpi_uid: u32,
    pub vendor_specific_data: [u8; 16],
}

// 4: Gic Error
// Values for interface_type above
pub const ACPI_AEST_GIC_CPU: c_int = 0;
pub const ACPI_AEST_GIC_DISTRIBUTOR: c_int = 1;
pub const ACPI_AEST_GIC_REDISTRIBUTOR: c_int = 2;
pub const ACPI_AEST_GIC_ITS: c_int = 3;

// 5: PCIe Error
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_pcie {
    pub iort_node_reference: u32,
}

// 6: Proxy Error
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_proxy {
    pub node_address: u64,
}

// Node Interface Structure
// Node Interface Structure V2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interface_header {
    pub type: u8,
    pub group_format: u8,
    pub reserved: [u8; 2],
    pub flags: u32,
    pub address: u64,
    pub error_record_index: u32,
    pub error_record_count: u32,
}

pub const ACPI_AEST_NODE_GROUP_FORMAT_4K: c_int = 0;
pub const ACPI_AEST_NODE_GROUP_FORMAT_16K: c_int = 1;
pub const ACPI_AEST_NODE_GROUP_FORMAT_64K: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interface_common {
    pub error_node_device: u32,
    pub processor_affinity: u32,
    pub error_group_register_base: u64,
    pub fault_inject_register_base: u64,
    pub interrupt_config_register_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interface_4k {
    pub error_record_implemented: u64,
    pub error_status_reporting: u64,
    pub addressing_mode: u64,
    pub common: acpi_aest_node_interface_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interface_16k {
    pub error_record_implemented: [u64; 4],
    pub error_status_reporting: [u64; 4],
    pub addressing_mode: [u64; 4],
    pub common: acpi_aest_node_interface_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interface_64k {
    pub error_record_implemented: [u64; 14],
    pub error_status_reporting: [u64; 14],
    pub addressing_mode: [u64; 14],
    pub common: acpi_aest_node_interface_common,
}

// Values for Type field above
pub const ACPI_AEST_NODE_SYSTEM_REGISTER: c_int = 0;
pub const ACPI_AEST_NODE_MEMORY_MAPPED: c_int = 1;
pub const ACPI_AEST_NODE_SINGLE_RECORD_MEMORY_MAPPED: c_int = 2;

// Node Interrupt Structure
// Node Interrupt Structure V2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aest_node_interrupt_v2 {
    pub type: u8,
    pub reserved: [u8; 2],
    pub flags: u8,
    pub gsiv: u32,
    pub reserved1: [u8; 4],
}

// Values for Type field above
pub const ACPI_AEST_NODE_FAULT_HANDLING: c_int = 0;
pub const ACPI_AEST_NODE_ERROR_RECOVERY: c_int = 1;

//
// AGDI - Arm Generic Diagnostic Dump and Reset Device Interface
//
// Conforms to "ACPI for Arm Components 1.1, Platform Design Document"
// ARM DEN0093 v1.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_agdi {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub sdei_event: u32,
    pub gsiv: u32,
}

// Mask for Flags field above

//
// APMT - ARM Performance Monitoring Unit Table
//
// Conforms to:
// ARM Performance Monitoring Unit Architecture 1.0 Platform Design Document
// ARM DEN0117 v1.0 November 25, 2021
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_apmt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

pub const ACPI_APMT_NODE_ID_LENGTH: c_int = 4;
//
// APMT subtables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_apmt_node {
    pub length: u16,
    pub flags: u8,
    pub type: u8,
    pub id: u32,
    pub inst_primary: u64,
    pub inst_secondary: u32,
    pub base_address0: u64,
    pub base_address1: u64,
    pub ovflw_irq: u32,
    pub reserved: u32,
    pub ovflw_irq_flags: u32,
    pub proc_affinity: u32,
    pub impl_id: u32,
}

// Masks for Flags field above

// Values for Flags dual page field above

// Values for Flags processor affinity field above

// Values for Flags 64-bit atomic field above

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_apmt_node_type {
    ACPI_APMT_NODE_TYPE_MC = 0x00,
    ACPI_APMT_NODE_TYPE_SMMU = 0x01,
    ACPI_APMT_NODE_TYPE_PCIE_ROOT = 0x02,
    ACPI_APMT_NODE_TYPE_ACPI = 0x03,
    ACPI_APMT_NODE_TYPE_CACHE = 0x04,
    ACPI_APMT_NODE_TYPE_COUNT
}

// Masks for ovflw_irq_flags field above

// Values for ovflw_irq_flags mode field above

// Values for ovflw_irq_flags type field above

//
// BDAT - BIOS Data ACPI Table
//
// Conforms to "BIOS Data ACPI Table", Interface Specification v4.0 Draft 5
// Nov 2020
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_bdat {
    pub header: acpi_table_header,
    pub gas: acpi_generic_address,
}

//
// CCEL - CC-Event Log
// From: "Guest-Host-Communication Interface (GHCI) for Intel
// Trust Domain Extensions (Intel TDX)". Feb 2022
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_ccel {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub CCtype: u8,
    pub Ccsub_type: u8,
    pub reserved: u16,
    pub log_area_minimum_length: u64,
    pub log_area_start_address: u64,
}

//
// ERDT - Enhanced Resource Director Technology (ERDT) table
//
// Conforms to "Intel Resource Director Technology Architecture Specification"
// Version 1.1, January 2025
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_erdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 max_clos; / Maximum classes of service,
    pub reserved: [u8; 24],
    pub erdt_substructures: [u8; ],
}

// Values for subtable type in struct acpi_subtbl_hdr_16
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_erdt_type {
    ACPI_ERDT_TYPE_RMDD = 0,
    ACPI_ERDT_TYPE_CACD = 1,
    ACPI_ERDT_TYPE_DACD = 2,
    ACPI_ERDT_TYPE_CMRC = 3,
    ACPI_ERDT_TYPE_MMRC = 4,
    ACPI_ERDT_TYPE_MARC = 5,
    ACPI_ERDT_TYPE_CARC = 6,
    ACPI_ERDT_TYPE_CMRD = 7,
    ACPI_ERDT_TYPE_IBRD = 8,
    ACPI_ERDT_TYPE_IBAD = 9,
    ACPI_ERDT_TYPE_CARD = 10,
    ACPI_ERDT_TYPE_RESERVED = 11	/* 11 and above are reserved */
}

//
// ERDT Subtables, correspond to Type in struct acpi_subtbl_hdr_16
//
// 0: RMDD - Resource Management Domain Description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_rmdd {
    pub header: acpi_subtbl_hdr_16,
    pub flags: u16,
    pub /: *mut *mut u16 IO_l3_slices; / Number of slices in IO cache,
    pub /: *mut *mut u8 IO_l3_sets; / Number of sets in IO cache,
    pub /: *mut *mut u8 IO_l3_ways; / Number of ways in IO cache,
    pub reserved: u64,
    pub /: *mut *mut u16 domain_id; / Unique domain ID,
    pub /: *mut *mut u32 max_rmid; / Maximun RMID supported,
    pub /: *mut *mut u64 creg_base; / Control Register Base Address,
    pub /: *mut *mut u16 creg_size; / Control Register Size (4K pages),
    pub rmdd_structs: [u8; ],
}

// 1: CACD - CPU Agent Collection Description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_cacd {
    pub header: acpi_subtbl_hdr_16,
    pub reserved: u16,
    pub /: *mut *mut u16 domain_id; / Unique domain ID,
    pub X2APICIDS: [u32; ],
}

// 2: DACD - Device Agent Collection Description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_dacd {
    pub header: acpi_subtbl_hdr_16,
    pub reserved: u16,
    pub /: *mut *mut u16 domain_id; / Unique domain ID,
    pub dev_paths: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_dacd_dev_paths {
    pub header: acpi_subtable_header,
    pub segment: u16,
    pub reserved: u8,
    pub start_bus: u8,
    pub path: [u8; ],
}

// 3: CMRC - Cache Monitoring Registers for CPU Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_cmrc {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u32,
    pub flags: u32,
    pub index_fn: u8,
    pub reserved2: [u8; 11],
    pub cmt_reg_base: u64,
    pub cmt_reg_size: u32,
    pub clump_size: u16,
    pub clump_stride: u16,
    pub up_scale: u64,
}

// 4: MMRC - Memory-bandwidth Monitoring Registers for CPU Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_mmrc {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u32,
    pub flags: u32,
    pub index_fn: u8,
    pub reserved2: [u8; 11],
    pub reg_base: u64,
    pub reg_size: u32,
    pub counter_width: u8,
    pub up_scale: u64,
    pub reserved3: [u8; 7],
    pub corr_factor_list_len: u32,
    pub corr_factor_list: [u32; ],
}

// 5: MARC - Memory-bandwidth Allocation Registers for CPU Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_marc {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u16,
    pub flags: u16,
    pub index_fn: u8,
    pub reserved2: [u8; 7],
    pub reg_base_opt: u64,
    pub reg_base_min: u64,
    pub reg_base_max: u64,
    pub mba_reg_size: u32,
    pub mba_ctrl_range: u32,
}

// 6: CARC - Cache Allocation Registers for CPU Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_carc {
    pub header: acpi_subtbl_hdr_16,
}

// 7: CMRD - Cache Monitoring Registers for Device Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_cmrd {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u32,
    pub flags: u32,
    pub index_fn: u8,
    pub reserved2: [u8; 11],
    pub reg_base: u64,
    pub reg_size: u32,
    pub cmt_reg_off: u16,
    pub cmt_clump_size: u16,
    pub up_scale: u64,
}

// 8: IBRD - Cache Monitoring Registers for Device Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_ibrd {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u32,
    pub flags: u32,
    pub index_fn: u8,
    pub reserved2: [u8; 11],
    pub reg_base: u64,
    pub reg_size: u32,
    pub total_bw_offset: u16,
    pub Iomiss_bw_offset: u16,
    pub total_bw_clump: u16,
    pub Iomiss_bw_clump: u16,
    pub reserved3: [u8; 7],
    pub counter_width: u8,
    pub up_scale: u64,
    pub corr_factor_list_len: u32,
    pub corr_factor_list: [u32; ],
}

// 9: IBAD - IO bandwidth Allocation Registers for device agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_ibad {
    pub header: acpi_subtbl_hdr_16,
}

// 10: CARD - IO bandwidth Allocation Registers for Device Agents
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erdt_card {
    pub header: acpi_subtbl_hdr_16,
    pub reserved1: u32,
    pub flags: u32,
    pub contention_mask: u32,
    pub index_fn: u8,
    pub reserved2: [u8; 7],
    pub reg_base: u64,
    pub reg_size: u32,
    pub cat_reg_offset: u16,
    pub cat_reg_block_size: u16,
}

//
// IORT - IO Remapping Table
//
// Conforms to "IO Remapping Table System Software on ARM Platforms",
// Document number: ARM DEN 0049E.f, Apr 2024
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_iort {
    pub header: acpi_table_header,
    pub node_count: u32,
    pub node_offset: u32,
    pub reserved: u32,
}

//
// IORT subtables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_node {
    pub type: u8,
    pub length: u16,
    pub revision: u8,
    pub identifier: u32,
    pub mapping_count: u32,
    pub mapping_offset: u32,
    pub node_data: [c_char; ],
}

// Values for subtable Type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_iort_node_type {
    ACPI_IORT_NODE_ITS_GROUP = 0x00,
    ACPI_IORT_NODE_NAMED_COMPONENT = 0x01,
    ACPI_IORT_NODE_PCI_ROOT_COMPLEX = 0x02,
    ACPI_IORT_NODE_SMMU = 0x03,
    ACPI_IORT_NODE_SMMU_V3 = 0x04,
    ACPI_IORT_NODE_PMCG = 0x05,
    ACPI_IORT_NODE_RMR = 0x06,
    ACPI_IORT_NODE_IWB = 0x07,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_id_mapping {
    pub /: *mut *mut u32 input_base; / Lowest value in input range,
    pub /: *mut *mut u32 id_count; / Number of IDs,
    pub /: *mut *mut u32 output_base; / Lowest value in output range,
    pub /: *mut *mut u32 output_reference; / A reference to the output node,
    pub flags: u32,
}

// Masks for Flags field above for IORT subtable

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_memory_access {
    pub cache_coherency: u32,
    pub hints: u8,
    pub reserved: u16,
    pub memory_flags: u8,
}

// Values for cache_coherency field above
pub const ACPI_IORT_NODE_COHERENT: c_uint = 0x00000001	/* The device node is fully coherent */;
pub const ACPI_IORT_NODE_NOT_COHERENT: c_uint = 0x00000000	/* The device node is not coherent */;
// Masks for Hints field above

// Masks for memory_flags field above

//
// IORT node specific subtables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_its_group {
    pub its_count: u32,
    pub /: *mut *mut u32 identifiers[]; / GIC ITS identifier array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_named_component {
    pub node_flags: u32,
    pub /: *mut *mut u64 memory_properties; / Memory access properties,
    pub /: *mut *mut u8 memory_address_limit; / Memory address size limit,
    pub /: *mut *mut char device_name[]; / Path of namespace object,
}

// Masks for Flags field above

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_root_complex {
    pub /: *mut *mut u64 memory_properties; / Memory access properties,
    pub ats_attribute: u32,
    pub pci_segment_number: u32,
    pub /: *mut *mut u8 memory_address_limit; / Memory address size limit,
    pub /: *mut *mut u16 pasid_capabilities; / PASID Capabilities,
    pub /: *mut *mut u8 reserved[]; / Reserved, must be zero,
}

// Masks for ats_attribute field above

// Masks for pasid_capabilities field above

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_smmu {
    pub /: *mut *mut u64 base_address; / SMMU base address,
    pub /: *mut *mut u64 span; / Length of memory range,
    pub model: u32,
    pub flags: u32,
    pub global_interrupt_offset: u32,
    pub context_interrupt_count: u32,
    pub context_interrupt_offset: u32,
    pub pmu_interrupt_count: u32,
    pub pmu_interrupt_offset: u32,
    pub /: *mut *mut u64 interrupts[]; / Interrupt array,
}

// Values for Model field above
pub const ACPI_IORT_SMMU_V1: c_uint = 0x00000000	/* Generic SMMUv1 */;
pub const ACPI_IORT_SMMU_V2: c_uint = 0x00000001	/* Generic SMMUv2 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU400: c_uint = 0x00000002	/* ARM Corelink MMU-400 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU500: c_uint = 0x00000003	/* ARM Corelink MMU-500 */;
pub const ACPI_IORT_SMMU_CORELINK_MMU401: c_uint = 0x00000004	/* ARM Corelink MMU-401 */;
pub const ACPI_IORT_SMMU_CAVIUM_THUNDERX: c_uint = 0x00000005	/* Cavium thunder_x SMMUv2 */;
// Masks for Flags field above

// Global interrupt format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_smmu_gsi {
    pub nsg_irpt: u32,
    pub nsg_irpt_flags: u32,
    pub nsg_cfg_irpt: u32,
    pub nsg_cfg_irpt_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_smmu_v3 {
    pub /: *mut *mut u64 base_address; / SMMUv3 base address,
    pub flags: u32,
    pub reserved: u32,
    pub vatos_address: u64,
    pub model: u32,
    pub event_gsiv: u32,
    pub pri_gsiv: u32,
    pub gerr_gsiv: u32,
    pub sync_gsiv: u32,
    pub pxm: u32,
    pub id_mapping_index: u32,
}

// Values for Model field above
pub const ACPI_IORT_SMMU_V3_GENERIC: c_uint = 0x00000000	/* Generic SMMUv3 */;
pub const ACPI_IORT_SMMU_V3_HISILICON_HI161X: c_uint = 0x00000001	/* hi_silicon Hi161x SMMUv3 */;
pub const ACPI_IORT_SMMU_V3_CAVIUM_CN99XX: c_uint = 0x00000002	/* Cavium CN99xx SMMUv3 */;
// Masks for Flags field above

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_pmcg {
    pub page0_base_address: u64,
    pub overflow_gsiv: u32,
    pub node_reference: u32,
    pub page1_base_address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_rmr {
    pub flags: u32,
    pub rmr_count: u32,
    pub rmr_offset: u32,
}

// Masks for Flags field above

//
// Macro to access the Access Attributes in flags field above:
// Access Attributes is encoded in bits 9:2
//

// Values for above Access Attributes
pub const ACPI_IORT_RMR_ATTR_DEVICE_NGNRNE: c_uint = 0x00;
pub const ACPI_IORT_RMR_ATTR_DEVICE_NGNRE: c_uint = 0x01;
pub const ACPI_IORT_RMR_ATTR_DEVICE_NGRE: c_uint = 0x02;
pub const ACPI_IORT_RMR_ATTR_DEVICE_GRE: c_uint = 0x03;
pub const ACPI_IORT_RMR_ATTR_NORMAL_NC: c_uint = 0x04;
pub const ACPI_IORT_RMR_ATTR_NORMAL_IWB_OWB: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_rmr_desc {
    pub base_address: u64,
    pub length: u64,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iort_iwb {
    pub base_address: u64,
    pub /: *mut *mut u16 iwb_index; / Unique IWB identifier matching with the IWB GSI namespace.,
    pub /: *mut *mut char device_name[]; / Path of the IWB namespace object,
}

//
// IOVT - I/O Virtualization Table
//
// Conforms to "LoongArch I/O Virtualization Table",
// Version 0.1, October 2024
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_iovt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub iommu_count: u16,
    pub iommu_offset: u16,
    pub reserved: [u8; 8],
}

// IOVT subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iovt_header {
    pub type: u16,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_iovt_iommu_type {
    ACPI_IOVT_IOMMU_V1 = 0x00,
    ACPI_IOVT_IOMMU_RESERVED = 0x01	/* 1 and greater are reserved */
}

// IOVT subtables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iovt_iommu {
    pub header: acpi_iovt_header,
    pub flags: u32,
    pub segment: u16,
    pub /: *mut *mut u16 phy_width; / Physical Address Width,
    pub /: *mut *mut u16 virt_width; / Virtual Address Width,
    pub max_page_level: u16,
    pub page_size: u64,
    pub device_id: u32,
    pub base_address: u64,
    pub address_space_size: u32,
    pub interrupt_type: u8,
    pub reserved: [u8; 3],
    pub gsi_number: u32,
    pub proximity_domain: u32,
    pub max_device_num: u32,
    pub device_entry_num: u32,
    pub device_entry_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_iovt_device_entry {
    pub type: u8,
    pub length: u8,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub device_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_iovt_device_entry_type {
    ACPI_IOVT_DEVICE_ENTRY_SINGLE = 0x00,
    ACPI_IOVT_DEVICE_ENTRY_START = 0x01,
    ACPI_IOVT_DEVICE_ENTRY_END = 0x02,
    ACPI_IOVT_DEVICE_ENTRY_RESERVED = 0x03	/* 3 and greater are reserved */
}

//
// IVRS - I/O Virtualization Reporting Structure
// Version 1
//
// Conforms to "AMD I/O Virtualization Technology (IOMMU) Specification",
// Revision 1.26, February 2009.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_ivrs {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 info; / Common virtualization info,
    pub reserved: u64,
}

// Values for Info field above
pub const ACPI_IVRS_PHYSICAL_SIZE: c_uint = 0x00007F00	/* 7 bits, physical address size */;
pub const ACPI_IVRS_VIRTUAL_SIZE: c_uint = 0x003F8000	/* 7 bits, virtual address size */;
pub const ACPI_IVRS_ATS_RESERVED: c_uint = 0x00400000	/* ATS address translation range reserved */;
// IVRS subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_header {
    pub /: *mut *mut u8 type; / Subtable type,
    pub flags: u8,
    pub /: *mut *mut u16 length; / Subtable length,
    pub /: *mut *mut u16 device_id; / ID of IOMMU,
}

// Values for subtable Type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ivrs_type {
    ACPI_IVRS_TYPE_HARDWARE1 = 0x10,
    ACPI_IVRS_TYPE_HARDWARE2 = 0x11,
    ACPI_IVRS_TYPE_HARDWARE3 = 0x40,
    ACPI_IVRS_TYPE_MEMORY1 = 0x20,
    ACPI_IVRS_TYPE_MEMORY2 = 0x21,
    ACPI_IVRS_TYPE_MEMORY3 = 0x22
}

// Masks for Flags field above for IVHD subtable

// Masks for Flags field above for IVMD subtable

//
// IVRS subtables, correspond to Type in struct acpi_ivrs_header
//
// 0x10: I/O Virtualization Hardware Definition Block (IVHD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_hardware_10 {
    pub header: acpi_ivrs_header,
    pub /: *mut *mut u16 capability_offset; / Offset for IOMMU control fields,
    pub /: *mut *mut u64 base_address; / IOMMU control registers,
    pub pci_segment_group: u16,
    pub /: *mut *mut u16 info; / MSI number and unit ID,
    pub feature_reporting: u32,
}

// 0x11: I/O Virtualization Hardware Definition Block (IVHD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_hardware_11 {
    pub header: acpi_ivrs_header,
    pub /: *mut *mut u16 capability_offset; / Offset for IOMMU control fields,
    pub /: *mut *mut u64 base_address; / IOMMU control registers,
    pub pci_segment_group: u16,
    pub /: *mut *mut u16 info; / MSI number and unit ID,
    pub attributes: u32,
    pub efr_register_image: u64,
    pub reserved: u64,
}

// Masks for Info field above
pub const ACPI_IVHD_MSI_NUMBER_MASK: c_uint = 0x001F	/* 5 bits, MSI message number */;
pub const ACPI_IVHD_UNIT_ID_MASK: c_uint = 0x1F00	/* 5 bits, unit_ID */;
//
// Device Entries for IVHD subtable, appear after struct acpi_ivrs_hardware structure.
// Upper two bits of the Type field are the (encoded) length of the structure.
// Currently, only 4 and 8 byte entries are defined. 16 and 32 byte entries
// are reserved for future use but not defined.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_de_header {
    pub type: u8,
    pub id: u16,
    pub data_setting: u8,
}

// Length of device entry is in the top two bits of Type field above
pub const ACPI_IVHD_ENTRY_LENGTH: c_uint = 0xC0;
// Values for device entry Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ivrs_device_entry_type {
// 4-byte device entries, all use struct acpi_ivrs_device4

    ACPI_IVRS_TYPE_PAD4 = 0,
    ACPI_IVRS_TYPE_ALL = 1,
    ACPI_IVRS_TYPE_SELECT = 2,
    ACPI_IVRS_TYPE_START = 3,
    ACPI_IVRS_TYPE_END = 4,

// 8-byte device entries

    ACPI_IVRS_TYPE_PAD8 = 64,
    ACPI_IVRS_TYPE_NOT_USED = 65,
    ACPI_IVRS_TYPE_ALIAS_SELECT = 66,	/* Uses struct acpi_ivrs_device8a */
    ACPI_IVRS_TYPE_ALIAS_START = 67,	/* Uses struct acpi_ivrs_device8a */
    ACPI_IVRS_TYPE_EXT_SELECT = 70,	/* Uses struct acpi_ivrs_device8b */
    ACPI_IVRS_TYPE_EXT_START = 71,	/* Uses struct acpi_ivrs_device8b */
    ACPI_IVRS_TYPE_SPECIAL = 72,	/* Uses struct acpi_ivrs_device8c */

// Variable-length device entries

    ACPI_IVRS_TYPE_HID = 240	/* Uses ACPI_IVRS_DEVICE_HID */
}

// Values for Data field above

// Types 0-4: 4-byte device entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_device4 {
    pub header: acpi_ivrs_de_header,
}

// Types 66-67: 8-byte device entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_device8a {
    pub header: acpi_ivrs_de_header,
    pub reserved1: u8,
    pub used_id: u16,
    pub reserved2: u8,
}

// Types 70-71: 8-byte device entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_device8b {
    pub header: acpi_ivrs_de_header,
    pub extended_data: u32,
}

// Values for extended_data above

// Type 72: 8-byte device entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_device8c {
    pub header: acpi_ivrs_de_header,
    pub handle: u8,
    pub used_id: u16,
    pub variety: u8,
}

// Values for Variety field above
pub const ACPI_IVHD_IOAPIC: c_int = 1;
pub const ACPI_IVHD_HPET: c_int = 2;
// Type 240: variable-length device entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_device_hid {
    pub header: acpi_ivrs_de_header,
    pub acpi_hid: u64,
    pub acpi_cid: u64,
    pub uid_type: u8,
    pub uid_length: u8,
}

// Values for uid_type above
pub const ACPI_IVRS_UID_NOT_PRESENT: c_int = 0;
pub const ACPI_IVRS_UID_IS_INTEGER: c_int = 1;
pub const ACPI_IVRS_UID_IS_STRING: c_int = 2;
// 0x20, 0x21, 0x22: I/O Virtualization Memory Definition Block (IVMD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ivrs_memory {
    pub header: acpi_ivrs_header,
    pub aux_data: u16,
    pub reserved: u64,
    pub start_address: u64,
    pub memory_length: u64,
}

//
// KEYP - Key Programming Interface for Root Complex Integrity and Data
// Encryption (IDE)
// Version 1
//
// Conforms to "Key Programming Interface for Root Complex Integrity and Data
// Encryption (IDE)" document. See under ACPI-Related Documents.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_keyp {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u32,
}

// KEYP common subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_keyp_common_header {
    pub type: u8,
    pub reserved: u8,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_keyp_type {
    ACPI_KEYP_TYPE_CONFIG_UNIT = 0,
}

// Root Port Information Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_keyp_rp_info {
    pub segment: u16,
    pub bus: u8,
    pub devfn: u8,
}

// Key Configuration Unit Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_keyp_config_unit {
    pub header: acpi_keyp_common_header,
    pub protocol_type: u8,
    pub version: u8,
    pub root_port_count: u8,
    pub flags: u8,
    pub register_base_address: u64,
    pub rp_info: [acpi_keyp_rp_info; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_keyp_protocol_type {
    ACPI_KEYP_PROTO_TYPE_INVALID = 0,
    ACPI_KEYP_PROTO_TYPE_PCIE,
    ACPI_KEYP_PROTO_TYPE_CXL,
    ACPI_KEYP_PROTO_TYPE_RESERVED
}

//
// LPIT - Low Power Idle Table
//
// Conforms to "ACPI Low Power Idle Table (LPIT)" July 2014.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_lpit {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// LPIT subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_lpit_header {
    pub /: *mut *mut u32 type; / Subtable type,
    pub /: *mut *mut u32 length; / Subtable length,
    pub unique_id: u16,
    pub reserved: u16,
    pub flags: u32,
}

// Values for subtable Type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_lpit_type {
    ACPI_LPIT_TYPE_NATIVE_CSTATE = 0x00,
    ACPI_LPIT_TYPE_RESERVED = 0x01	/* 1 and above are reserved */
}

// Masks for Flags field above

//
// LPIT subtables, correspond to Type in struct acpi_lpit_header
//
// 0x00: Native C-state instruction based LPI structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_lpit_native {
    pub header: acpi_lpit_header,
    pub entry_trigger: acpi_generic_address,
    pub residency: u32,
    pub latency: u32,
    pub residency_counter: acpi_generic_address,
    pub counter_frequency: u64,
}

//
// MADT - Multiple APIC Description Table
// Version 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_madt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 address; / Physical address of local APIC,
    pub flags: u32,
}

// Masks for Flags field above

// Values for PCATCompat flag
pub const ACPI_MADT_DUAL_PIC: c_int = 1;
pub const ACPI_MADT_MULTIPLE_APIC: c_int = 0;
// Values for MADT subtable type in struct acpi_subtable_header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_type {
    ACPI_MADT_TYPE_LOCAL_APIC = 0,
    ACPI_MADT_TYPE_IO_APIC = 1,
    ACPI_MADT_TYPE_INTERRUPT_OVERRIDE = 2,
    ACPI_MADT_TYPE_NMI_SOURCE = 3,
    ACPI_MADT_TYPE_LOCAL_APIC_NMI = 4,
    ACPI_MADT_TYPE_LOCAL_APIC_OVERRIDE = 5,
    ACPI_MADT_TYPE_IO_SAPIC = 6,
    ACPI_MADT_TYPE_LOCAL_SAPIC = 7,
    ACPI_MADT_TYPE_INTERRUPT_SOURCE = 8,
    ACPI_MADT_TYPE_LOCAL_X2APIC = 9,
    ACPI_MADT_TYPE_LOCAL_X2APIC_NMI = 10,
    ACPI_MADT_TYPE_GENERIC_INTERRUPT = 11,
    ACPI_MADT_TYPE_GENERIC_DISTRIBUTOR = 12,
    ACPI_MADT_TYPE_GENERIC_MSI_FRAME = 13,
    ACPI_MADT_TYPE_GENERIC_REDISTRIBUTOR = 14,
    ACPI_MADT_TYPE_GENERIC_TRANSLATOR = 15,
    ACPI_MADT_TYPE_MULTIPROC_WAKEUP = 16,
    ACPI_MADT_TYPE_CORE_PIC = 17,
    ACPI_MADT_TYPE_LIO_PIC = 18,
    ACPI_MADT_TYPE_HT_PIC = 19,
    ACPI_MADT_TYPE_EIO_PIC = 20,
    ACPI_MADT_TYPE_MSI_PIC = 21,
    ACPI_MADT_TYPE_BIO_PIC = 22,
    ACPI_MADT_TYPE_LPC_PIC = 23,
    ACPI_MADT_TYPE_RINTC = 24,
    ACPI_MADT_TYPE_IMSIC = 25,
    ACPI_MADT_TYPE_APLIC = 26,
    ACPI_MADT_TYPE_PLIC = 27,
    ACPI_MADT_TYPE_GICV5_IRS = 28,
    ACPI_MADT_TYPE_GICV5_ITS = 29,
    ACPI_MADT_TYPE_GICV5_ITS_TRANSLATE = 30,
    ACPI_MADT_TYPE_RESERVED = 31,	/* 31 to 0x7F are reserved */
    ACPI_MADT_TYPE_OEM_RESERVED = 0x80	/* 0x80 to 0xFF are reserved for OEM use */
}

//
// MADT Subtables, correspond to Type in struct acpi_subtable_header
//
// 0: Processor Local APIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_apic {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 processor_id; / ACPI processor id,
    pub /: *mut *mut u8 id; / Processor's local APIC id,
    pub lapic_flags: u32,
}

// 1: IO APIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_io_apic {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 id; / I/O APIC ID,
    pub /: *mut *mut u8 reserved; / reserved - must be zero,
    pub /: *mut *mut u32 address; / APIC physical address,
    pub /: *mut *mut u32 global_irq_base; / Global system interrupt where INTI lines start,
}

// 2: Interrupt Override
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_interrupt_override {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 bus; / 0 - ISA,
    pub /: *mut *mut u8 source_irq; / Interrupt source (IRQ),
    pub /: *mut *mut u32 global_irq; / Global system interrupt,
    pub inti_flags: u16,
}

// 3: NMI Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_nmi_source {
    pub header: acpi_subtable_header,
    pub inti_flags: u16,
    pub /: *mut *mut u32 global_irq; / Global system interrupt,
}

// 4: Local APIC NMI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_apic_nmi {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 processor_id; / ACPI processor id,
    pub inti_flags: u16,
    pub /: *mut *mut u8 lint; / LINTn to which NMI is connected,
}

// 5: Address Override
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_apic_override {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / Reserved, must be zero,
    pub /: *mut *mut u64 address; / APIC physical address,
}

// 6: I/O Sapic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_io_sapic {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 id; / I/O SAPIC ID,
    pub /: *mut *mut u8 reserved; / Reserved, must be zero,
    pub /: *mut *mut u32 global_irq_base; / Global interrupt for SAPIC start,
    pub /: *mut *mut u64 address; / SAPIC physical address,
}

// 7: Local Sapic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_sapic {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 processor_id; / ACPI processor id,
    pub /: *mut *mut u8 id; / SAPIC ID,
    pub /: *mut *mut u8 eid; / SAPIC EID,
    pub /: *mut *mut u8 reserved[3]; / Reserved, must be zero,
    pub lapic_flags: u32,
    pub /: *mut *mut u32 uid; / Numeric UID - ACPI 3.0,
    pub /: *mut *mut char uid_string[]; / String UID - ACPI 3.0,
}

// 8: Platform Interrupt Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_interrupt_source {
    pub header: acpi_subtable_header,
    pub inti_flags: u16,
    pub /: *mut *mut u8 type; / 1=PMI, 2=INIT, 3=corrected,
    pub /: *mut *mut u8 id; / Processor ID,
    pub /: *mut *mut u8 eid; / Processor EID,
    pub /: *mut *mut u8 io_sapic_vector; / Vector value for PMI interrupts,
    pub /: *mut *mut u32 global_irq; / Global system interrupt,
    pub /: *mut *mut u32 flags; / Interrupt Source Flags,
}

// Masks for Flags field above

// 9: Processor Local X2APIC (ACPI 4.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_x2apic {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub /: *mut *mut u32 local_apic_id; / Processor x2APIC ID,
    pub lapic_flags: u32,
    pub /: *mut *mut u32 uid; / ACPI processor UID,
}

// 10: Local X2APIC NMI (ACPI 4.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_local_x2apic_nmi {
    pub header: acpi_subtable_header,
    pub inti_flags: u16,
    pub /: *mut *mut u32 uid; / ACPI processor UID,
    pub /: *mut *mut u8 lint; / LINTn to which NMI is connected,
    pub /: *mut *mut u8 reserved[3]; / reserved - must be zero,
}

// 11: Generic interrupt - GICC (ACPI 5.0 + ACPI 6.0 + ACPI 6.3 + ACPI 6.5 + ACPI 6.7 changes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_generic_interrupt {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub cpu_interface_number: u32,
    pub uid: u32,
    pub flags: u32,
    pub parking_version: u32,
    pub performance_interrupt: u32,
    pub parked_address: u64,
    pub base_address: u64,
    pub gicv_base_address: u64,
    pub gich_base_address: u64,
    pub vgic_interrupt: u32,
    pub gicr_base_address: u64,
    pub arm_mpidr: u64,
    pub efficiency_class: u8,
    pub reserved2: [u8; 1],
    pub /: *mut *mut u16 spe_interrupt; / ACPI 6.3,
    pub /: *mut *mut u16 trbe_interrupt; / ACPI 6.5,
    pub /: *mut *mut u16 iaffid; / ACPI 6.7,
    pub irs_id: u32,
}

// Masks for Flags field above
// ACPI_MADT_ENABLED                    (1)      Processor is usable if set

// 12: Generic Distributor (ACPI 5.0 + ACPI 6.0 changes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_generic_distributor {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub gic_id: u32,
    pub base_address: u64,
    pub global_irq_base: u32,
    pub version: u8,
    pub /: *mut *mut u8 reserved2[3]; / reserved - must be zero,
}

// Values for Version field above and Version field in acpi_madt_gicv5_irs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_gic_version {
    ACPI_MADT_GIC_VERSION_NONE = 0,
    ACPI_MADT_GIC_VERSION_V1 = 1,
    ACPI_MADT_GIC_VERSION_V2 = 2,
    ACPI_MADT_GIC_VERSION_V3 = 3,
    ACPI_MADT_GIC_VERSION_V4 = 4,
    ACPI_MADT_GIC_VERSION_V5 = 5,
    ACPI_MADT_GIC_VERSION_RESERVED = 6	/* 6 and greater are reserved */
}

// 13: Generic MSI Frame (ACPI 5.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_generic_msi_frame {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub msi_frame_id: u32,
    pub base_address: u64,
    pub flags: u32,
    pub spi_count: u16,
    pub spi_base: u16,
}

// Masks for Flags field above

// 14: Generic Redistributor (ACPI 5.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_generic_redistributor {
    pub header: acpi_subtable_header,
    pub flags: u8,
    pub /: *mut *mut u8 reserved; / reserved - must be zero,
    pub base_address: u64,
    pub length: u32,
}

// 15: Generic Translator (ACPI 6.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_generic_translator {
    pub header: acpi_subtable_header,
    pub flags: u8,
    pub /: *mut *mut u8 reserved; / reserved - must be zero,
    pub translation_id: u32,
    pub base_address: u64,
    pub reserved2: u32,
}

// 16: Multiprocessor wakeup (ACPI 6.6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_multiproc_wakeup {
    pub header: acpi_subtable_header,
    pub version: u16,
    pub /: *mut *mut u32 reserved; / reserved - must be zero,
    pub mailbox_address: u64,
    pub reset_vector: u64,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_multiproc_wakeup_version {
    ACPI_MADT_MP_WAKEUP_VERSION_NONE = 0,
    ACPI_MADT_MP_WAKEUP_VERSION_V1 = 1,
    ACPI_MADT_MP_WAKEUP_VERSION_RESERVED = 2, /* 2 and greater are reserved */
}

pub const ACPI_MADT_MP_WAKEUP_SIZE_V0: c_int = 16;
pub const ACPI_MADT_MP_WAKEUP_SIZE_V1: c_int = 24;
pub const ACPI_MULTIPROC_WAKEUP_MB_OS_SIZE: c_int = 2032;
pub const ACPI_MULTIPROC_WAKEUP_MB_FIRMWARE_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_multiproc_wakeup_mailbox {
    pub command: u16,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub apic_id: u32,
    pub wakeup_vector: u64,
    pub /: *mut *mut u8 reserved_os[ACPI_MULTIPROC_WAKEUP_MB_OS_SIZE]; / reserved for OS use,
    pub /: *mut *mut u8 reserved_firmware[ACPI_MULTIPROC_WAKEUP_MB_FIRMWARE_SIZE]; / reserved for firmware use,
}

pub const ACPI_MP_WAKE_COMMAND_WAKEUP: c_int = 1;
pub const ACPI_MP_WAKE_COMMAND_TEST: c_int = 2;
// 17: CPU Core Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_core_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub processor_id: u32,
    pub core_id: u32,
    pub flags: u32,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_core_pic_version {
    ACPI_MADT_CORE_PIC_VERSION_NONE = 0,
    ACPI_MADT_CORE_PIC_VERSION_V1 = 1,
    ACPI_MADT_CORE_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 18: Legacy I/O Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_lio_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub address: u64,
    pub size: u16,
    pub cascade: [u8; 2],
    pub cascade_map: [u32; 2],
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_lio_pic_version {
    ACPI_MADT_LIO_PIC_VERSION_NONE = 0,
    ACPI_MADT_LIO_PIC_VERSION_V1 = 1,
    ACPI_MADT_LIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 19: HT Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_ht_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub address: u64,
    pub size: u16,
    pub cascade: [u8; 8],
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_ht_pic_version {
    ACPI_MADT_HT_PIC_VERSION_NONE = 0,
    ACPI_MADT_HT_PIC_VERSION_V1 = 1,
    ACPI_MADT_HT_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 20: Extend I/O Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_eio_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub cascade: u8,
    pub node: u8,
    pub node_map: u64,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_eio_pic_version {
    ACPI_MADT_EIO_PIC_VERSION_NONE = 0,
    ACPI_MADT_EIO_PIC_VERSION_V1 = 1,
    ACPI_MADT_EIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 21: MSI Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_msi_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub msg_address: u64,
    pub start: u32,
    pub count: u32,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_msi_pic_version {
    ACPI_MADT_MSI_PIC_VERSION_NONE = 0,
    ACPI_MADT_MSI_PIC_VERSION_V1 = 1,
    ACPI_MADT_MSI_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 22: Bridge I/O Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_bio_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub address: u64,
    pub size: u16,
    pub id: u16,
    pub gsi_base: u16,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_bio_pic_version {
    ACPI_MADT_BIO_PIC_VERSION_NONE = 0,
    ACPI_MADT_BIO_PIC_VERSION_V1 = 1,
    ACPI_MADT_BIO_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 23: LPC Interrupt Controller (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_lpc_pic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub address: u64,
    pub size: u16,
    pub cascade: u8,
}

// Values for Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_lpc_pic_version {
    ACPI_MADT_LPC_PIC_VERSION_NONE = 0,
    ACPI_MADT_LPC_PIC_VERSION_V1 = 1,
    ACPI_MADT_LPC_PIC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 24: RISC-V INTC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_rintc {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub reserved: u8,
    pub flags: u32,
    pub hart_id: u64,
    pub /: *mut *mut u32 uid; / ACPI processor UID,
    pub /: *mut *mut u32 ext_intc_id; / External INTC Id,
    pub /: *mut *mut u64 imsic_addr; / IMSIC base address,
    pub /: *mut *mut u32 imsic_size; / IMSIC size,
}

// Values for RISC-V INTC Version field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_madt_rintc_version {
    ACPI_MADT_RINTC_VERSION_NONE = 0,
    ACPI_MADT_RINTC_VERSION_V1 = 1,
    ACPI_MADT_RINTC_VERSION_RESERVED = 2	/* 2 and greater are reserved */
}

// 25: RISC-V IMSIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_imsic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub reserved: u8,
    pub flags: u32,
    pub num_ids: u16,
    pub num_guest_ids: u16,
    pub guest_index_bits: u8,
    pub hart_index_bits: u8,
    pub group_index_bits: u8,
    pub group_index_shift: u8,
}

// 26: RISC-V APLIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_aplic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub id: u8,
    pub flags: u32,
    pub hw_id: [u8; 8],
    pub num_idcs: u16,
    pub num_sources: u16,
    pub gsi_base: u32,
    pub base_addr: u64,
    pub size: u32,
}

// 27: RISC-V PLIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_plic {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub id: u8,
    pub hw_id: [u8; 8],
    pub num_irqs: u16,
    pub max_prio: u16,
    pub flags: u32,
    pub size: u32,
    pub base_addr: u64,
    pub gsi_base: u32,
}

// 28: Arm GICv5 IRS (ACPI 6.7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_gicv5_irs {
    pub header: acpi_subtable_header,
    pub version: u8,
    pub reserved: u8,
    pub irs_id: u32,
    pub flags: u32,
    pub reserved2: u32,
    pub config_base_address: u64,
    pub setlpi_base_address: u64,
}

// 29: Arm GICv5 ITS Config Frame (ACPI 6.7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_gicv5_translator {
    pub header: acpi_subtable_header,
    pub flags: u8,
    pub /: *mut *mut u8 reserved; / reserved - must be zero,
    pub translator_id: u32,
    pub base_address: u64,
}

// 30: Arm GICv5 ITS Translate Frame (ACPI 6.7)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_gicv5_translate_frame {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / reserved - must be zero,
    pub linked_translator_id: u32,
    pub translate_frame_id: u32,
    pub reserved2: u32,
    pub base_address: u64,
}

// 80: OEM data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_madt_oem_data {
    pub oem_data): ACPI_FLEX_ARRAY(u8,,
}

//
// Common flags fields for MADT subtables
//
// MADT Local APIC flags

// MADT MPS INTI flags (inti_flags)

// Values for MPS INTI flags
pub const ACPI_MADT_POLARITY_CONFORMS: c_int = 0;
pub const ACPI_MADT_POLARITY_ACTIVE_HIGH: c_int = 1;
pub const ACPI_MADT_POLARITY_RESERVED: c_int = 2;
pub const ACPI_MADT_POLARITY_ACTIVE_LOW: c_int = 3;

//
// MCFG - PCI Memory Mapped Configuration table and subtable
// Version 1
//
// Conforms to "PCI Firmware Specification", Revision 3.0, June 20, 2005
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_mcfg {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: [u8; 8],
}

// Subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mcfg_allocation {
    pub /: *mut *mut u64 address; / Base address, processor-relative,
    pub /: *mut *mut u16 pci_segment; / PCI segment group number,
    pub /: *mut *mut u8 start_bus_number; / Starting PCI Bus number,
    pub /: *mut *mut u8 end_bus_number; / Final PCI Bus number,
    pub reserved: u32,
}

//
// MCHI - Management Controller Host Interface Table
// Version 1
//
// Conforms to "Management Component Transport Protocol (MCTP) Host
// Interface Specification", Revision 1.0.0a, October 13, 2009
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_mchi {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub interface_type: u8,
    pub protocol: u8,
    pub protocol_data: u64,
    pub interrupt_type: u8,
    pub gpe: u8,
    pub pci_device_flag: u8,
    pub global_interrupt: u32,
    pub control_register: acpi_generic_address,
    pub pci_segment: u8,
    pub pci_bus: u8,
    pub pci_device: u8,
    pub pci_function: u8,
}

//
// MPAM - Memory System Resource Partitioning and Monitoring
//
// Conforms to "ACPI for Memory System Resource Partitioning and Monitoring 2.0"
// Document number: ARM DEN 0065, December, 2022.
//
// MPAM RIS locator types. Table 11, Location types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_mpam_locator_type {
    ACPI_MPAM_LOCATION_TYPE_PROCESSOR_CACHE = 0,
    ACPI_MPAM_LOCATION_TYPE_MEMORY = 1,
    ACPI_MPAM_LOCATION_TYPE_SMMU = 2,
    ACPI_MPAM_LOCATION_TYPE_MEMORY_CACHE = 3,
    ACPI_MPAM_LOCATION_TYPE_ACPI_DEVICE = 4,
    ACPI_MPAM_LOCATION_TYPE_INTERCONNECT = 5,
    ACPI_MPAM_LOCATION_TYPE_UNKNOWN = 0xFF
}

// MPAM Functional dependency descriptor. Table 10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_func_deps {
    pub producer: u32,
    pub reserved: u32,
}

// MPAM Processor cache locator descriptor. Table 13
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_cache_locator {
    pub cache_reference: u64,
    pub reserved: u32,
}

// MPAM Memory locator descriptor. Table 14
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_memory_locator {
    pub proximity_domain: u64,
    pub reserved: u32,
}

// MPAM SMMU locator descriptor. Table 15
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_smmu_locator {
    pub smmu_interface: u64,
    pub reserved: u32,
}

// MPAM Memory-side cache locator descriptor. Table 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_memcache_locator {
    pub reserved: [u8; 7],
    pub level: u8,
    pub reference: u32,
}

// MPAM ACPI device locator descriptor. Table 17
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_acpi_locator {
    pub acpi_hw_id: u64,
    pub acpi_unique_id: u32,
}

// MPAM Interconnect locator descriptor. Table 18
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_interconnect_locator {
    pub inter_connect_desc_tbl_off: u64,
    pub reserved: u32,
}

// MPAM Locator structure. Table 12
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_generic_locator {
    pub descriptor1: u64,
    pub descriptor2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_mpam_resource_locator {
    pub cache_locator: acpi_mpam_resource_cache_locator,
    pub memory_locator: acpi_mpam_resource_memory_locator,
    pub smmu_locator: acpi_mpam_resource_smmu_locator,
    pub mem_cache_locator: acpi_mpam_resource_memcache_locator,
    pub acpi_locator: acpi_mpam_resource_acpi_locator,
    pub interconnect_ifc_locator: acpi_mpam_resource_interconnect_locator,
    pub generic_locator: acpi_mpam_resource_generic_locator,
}

// Memory System Component Resource Node Structure Table 9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_resource_node {
    pub identifier: u32,
    pub ris_index: u8,
    pub reserved1: u16,
    pub locator_type: u8,
    pub locator: acpi_mpam_resource_locator,
    pub num_functional_deps: u32,
}

// Memory System Component (MSC) Node Structure. Table 4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpam_msc_node {
    pub length: u16,
    pub interface_type: u8,
    pub reserved: u8,
    pub identifier: u32,
    pub base_address: u64,
    pub mmio_size: u32,
    pub overflow_interrupt: u32,
    pub overflow_interrupt_flags: u32,
    pub reserved1: u32,
    pub overflow_interrupt_affinity: u32,
    pub error_interrupt: u32,
    pub error_interrupt_flags: u32,
    pub reserved2: u32,
    pub error_interrupt_affinity: u32,
    pub max_nrdy_usec: u32,
    pub hardware_id_linked_device: u64,
    pub instance_id_linked_device: u32,
    pub num_resource_nodes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_mpam {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

//
// MPST - Memory Power State Table (ACPI 5.0)
// Version 1
//

// Main table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_mpst {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// Memory Platform Communication Channel Info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_channel {
}

// Memory Power Node Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_power_node {
    pub flags: u8,
    pub reserved1: u8,
    pub node_id: u16,
    pub length: u32,
    pub range_address: u64,
    pub range_length: u64,
    pub num_power_states: u32,
    pub num_physical_components: u32,
}

// Values for Flags field above
pub const ACPI_MPST_ENABLED: c_int = 1;
pub const ACPI_MPST_POWER_MANAGED: c_int = 2;
pub const ACPI_MPST_HOT_PLUG_CAPABLE: c_int = 4;
// Memory Power State Structure (follows POWER_NODE above)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_power_state {
    pub power_state: u8,
    pub info_index: u8,
}

// Physical Component ID Structure (follows POWER_STATE above)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_component {
    pub component_id: u16,
}

// Memory Power State Characteristics Structure (follows all POWER_NODEs)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_data_hdr {
    pub characteristics_count: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_power_data {
    pub structure_id: u8,
    pub flags: u8,
    pub reserved1: u16,
    pub average_power: u32,
    pub power_saving: u32,
    pub exit_latency: u64,
    pub reserved2: u64,
}

// Values for Flags field above
pub const ACPI_MPST_PRESERVE: c_int = 1;
pub const ACPI_MPST_AUTOENTRY: c_int = 2;
pub const ACPI_MPST_AUTOEXIT: c_int = 4;
// Shared Memory Region (not part of an ACPI table)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mpst_shared {
    pub signature: u32,
    pub pcc_command: u16,
    pub pcc_status: u16,
    pub command_register: u32,
    pub status_register: u32,
    pub power_state_id: u32,
    pub power_node_id: u32,
    pub energy_consumed: u64,
    pub average_power: u64,
}

//
// MSCT - Maximum System Characteristics Table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_msct {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 proximity_offset; / Location of proximity info struct(s),
    pub /: *mut *mut u32 max_proximity_domains; / Max number of proximity domains,
    pub /: *mut *mut u32 max_clock_domains; / Max number of clock domains,
    pub /: *mut *mut u64 max_address; / Max physical address in system,
}

// subtable - Maximum Proximity Domain Information. Version 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_msct_proximity {
    pub revision: u8,
    pub length: u8,
    pub /: *mut *mut u32 range_start; / Start of domain range,
    pub /: *mut *mut u32 range_end; / End of domain range,
    pub processor_capacity: u32,
    pub /: *mut *mut u64 memory_capacity; / In bytes,
}

//
// MRRM - Memory Range and Region Mapping (MRRM) table
// Conforms to "Intel Resource Director Technology Architecture Specification"
// Version 1.1, January 2025
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_mrrm {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 max_mem_region; / Max Memory Regions supported,
    pub /: *mut *mut u8 flags; / Region assignment type,
    pub reserved: [u8; 26],
    pub memory_range_entry: [u8; ],
}

// Flags

//
// Memory Range entry - Memory Range entry in MRRM table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mrrm_mem_range_entry {
    pub header: acpi_subtbl_hdr_16,
    pub /: *mut *mut u32 reserved0; / Reserved,
    pub /: *mut *mut u64 addr_base; / Base addr of the mem range,
    pub /: *mut *mut u64 addr_len; / Length of the mem range,
    pub /: *mut *mut u16 region_id_flags; / Valid local or remote Region-ID,
    pub /: *mut *mut u8 local_region_id; / Platform-assigned static local Region-ID,
    pub /: *mut *mut u8 remote_region_id; / Platform-assigned static remote Region-ID,
    pub /: *mut *mut u32 reserved1; / Reserved,
// Region-ID Programming Registers[]
}

// Values for region_id_flags above

//
// MSDM - Microsoft Data Management table
//
// Conforms to "Microsoft Software Licensing Tables (SLIC and MSDM)",
// November 29, 2011. Copyright 2011 Microsoft
//
// Basic MSDM table is only the common ACPI header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_msdm {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

//
// NFIT - NVDIMM Interface Table (ACPI 6.0+)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_nfit {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 reserved; / Reserved, must be zero,
}

// Subtable header for NFIT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_header {
    pub type: u16,
    pub length: u16,
}

// Values for subtable type in struct acpi_nfit_header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_nfit_type {
    ACPI_NFIT_TYPE_SYSTEM_ADDRESS = 0,
    ACPI_NFIT_TYPE_MEMORY_MAP = 1,
    ACPI_NFIT_TYPE_INTERLEAVE = 2,
    ACPI_NFIT_TYPE_SMBIOS = 3,
    ACPI_NFIT_TYPE_CONTROL_REGION = 4,
    ACPI_NFIT_TYPE_DATA_REGION = 5,
    ACPI_NFIT_TYPE_FLUSH_ADDRESS = 6,
    ACPI_NFIT_TYPE_CAPABILITIES = 7,
    ACPI_NFIT_TYPE_RESERVED = 8	/* 8 and greater are reserved */
}

//
// NFIT Subtables
//
// 0: System Physical Address Range Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_system_address {
    pub header: acpi_nfit_header,
    pub range_index: u16,
    pub flags: u16,
    pub /: *mut *mut u32 reserved; / Reserved, must be zero,
    pub proximity_domain: u32,
    pub range_guid: [u8; 16],
    pub address: u64,
    pub length: u64,
    pub memory_mapping: u64,
    pub /: *mut *mut u64 location_cookie; / ACPI 6.4,
}

// Flags

// Range Type GUIDs appear in the include/acuuid.h file
// 1: Memory Device to System Address Range Map Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_memory_map {
    pub header: acpi_nfit_header,
    pub device_handle: u32,
    pub physical_id: u16,
    pub region_id: u16,
    pub range_index: u16,
    pub region_index: u16,
    pub region_size: u64,
    pub region_offset: u64,
    pub address: u64,
    pub interleave_index: u16,
    pub interleave_ways: u16,
    pub flags: u16,
    pub /: *mut *mut u16 reserved; / Reserved, must be zero,
}

// Flags

// 2: Interleave Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_interleave {
    pub header: acpi_nfit_header,
    pub interleave_index: u16,
    pub /: *mut *mut u16 reserved; / Reserved, must be zero,
    pub line_count: u32,
    pub line_size: u32,
    pub /: *mut *mut u32 line_offset[]; / Variable length,
}

// 3: SMBIOS Management Information Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_smbios {
    pub header: acpi_nfit_header,
    pub /: *mut *mut u32 reserved; / Reserved, must be zero,
    pub /: *mut *mut u8 data[]; / Variable length,
}

// 4: NVDIMM Control Region Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_control_region {
    pub header: acpi_nfit_header,
    pub region_index: u16,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub subsystem_revision_id: u16,
    pub valid_fields: u8,
    pub manufacturing_location: u8,
    pub manufacturing_date: u16,
    pub /: *mut *mut u8 reserved[2]; / Reserved, must be zero,
    pub serial_number: u32,
    pub code: u16,
    pub windows: u16,
    pub window_size: u64,
    pub command_offset: u64,
    pub command_size: u64,
    pub status_offset: u64,
    pub status_size: u64,
    pub flags: u16,
    pub /: *mut *mut u8 reserved1[6]; / Reserved, must be zero,
}

// Flags

// valid_fields bits

// 5: NVDIMM Block Data Window Region Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_data_region {
    pub header: acpi_nfit_header,
    pub region_index: u16,
    pub windows: u16,
    pub offset: u64,
    pub size: u64,
    pub capacity: u64,
    pub start_address: u64,
}

// 6: Flush Hint Address Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_flush_address {
    pub header: acpi_nfit_header,
    pub device_handle: u32,
    pub hint_count: u16,
    pub /: *mut *mut u8 reserved[6]; / Reserved, must be zero,
    pub /: *mut *mut u64 hint_address[]; / Variable length,
}

// 7: Platform Capabilities Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nfit_capabilities {
    pub header: acpi_nfit_header,
    pub highest_capability: u8,
    pub /: *mut *mut u8 reserved[3]; / Reserved, must be zero,
    pub capabilities: u32,
    pub reserved2: u32,
}

// Capabilities Flags

//
// NFIT/DVDIMM device handle support - used as the _ADR for each NVDIMM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfit_device_handle {
    pub handle: u32,
}

// Device handle construction and extraction macros
pub const ACPI_NFIT_DIMM_NUMBER_MASK: c_uint = 0x0000000F;
pub const ACPI_NFIT_CHANNEL_NUMBER_MASK: c_uint = 0x000000F0;
pub const ACPI_NFIT_MEMORY_ID_MASK: c_uint = 0x00000F00;
pub const ACPI_NFIT_SOCKET_ID_MASK: c_uint = 0x0000F000;
pub const ACPI_NFIT_NODE_ID_MASK: c_uint = 0x0FFF0000;
pub const ACPI_NFIT_DIMM_NUMBER_OFFSET: c_int = 0;
pub const ACPI_NFIT_CHANNEL_NUMBER_OFFSET: c_int = 4;
pub const ACPI_NFIT_MEMORY_ID_OFFSET: c_int = 8;
pub const ACPI_NFIT_SOCKET_ID_OFFSET: c_int = 12;
pub const ACPI_NFIT_NODE_ID_OFFSET: c_int = 16;
// Macro to construct a NFIT/NVDIMM device handle

// Macros to extract individual fields from a NFIT/NVDIMM device handle

//
// NHLT - Non HDAudio Link Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_nhlt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub endpoints_count: u8,
//
// struct acpi_nhlt_endpoint endpoints[];
// struct acpi_nhlt_config oed_config;
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_endpoint {
    pub length: u32,
    pub link_type: u8,
    pub instance_id: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u16,
    pub subsystem_id: u32,
    pub device_type: u8,
    pub direction: u8,
    pub virtual_bus_id: u8,
//
// struct acpi_nhlt_config device_config;
// struct acpi_nhlt_formats_config formats_config;
// struct acpi_nhlt_devices_info devices_info;
//
}

//
// Values for link_type field above
//
// Only types PDM and SSP are used
//
pub const ACPI_NHLT_LINKTYPE_HDA: c_int = 0;
pub const ACPI_NHLT_LINKTYPE_DSP: c_int = 1;
pub const ACPI_NHLT_LINKTYPE_PDM: c_int = 2;
pub const ACPI_NHLT_LINKTYPE_SSP: c_int = 3;
pub const ACPI_NHLT_LINKTYPE_SLIMBUS: c_int = 4;
pub const ACPI_NHLT_LINKTYPE_SDW: c_int = 5;
pub const ACPI_NHLT_LINKTYPE_UAOL: c_int = 6;
// Values for device_id field above
pub const ACPI_NHLT_DEVICEID_DMIC: c_uint = 0xAE20;
pub const ACPI_NHLT_DEVICEID_BT: c_uint = 0xAE30;
pub const ACPI_NHLT_DEVICEID_I2S: c_uint = 0xAE34;
// Values for device_type field above
//
// Device types unique to endpoint of link_type=PDM
//
// Type PDM used for all SKL+ platforms
//
pub const ACPI_NHLT_DEVICETYPE_PDM: c_int = 0;
pub const ACPI_NHLT_DEVICETYPE_PDM_SKL: c_int = 1;
// Device types unique to endpoint of link_type=SSP
pub const ACPI_NHLT_DEVICETYPE_BT: c_int = 0;
pub const ACPI_NHLT_DEVICETYPE_FM: c_int = 1;
pub const ACPI_NHLT_DEVICETYPE_MODEM: c_int = 2;
pub const ACPI_NHLT_DEVICETYPE_CODEC: c_int = 4;
// Values for Direction field above
pub const ACPI_NHLT_DIR_RENDER: c_int = 0;
pub const ACPI_NHLT_DIR_CAPTURE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_config {
    pub capabilities_size: u32,
    pub capabilities: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_gendevice_config {
    pub virtual_slot: u8,
    pub config_type: u8,
}

// Values for config_type field above
pub const ACPI_NHLT_CONFIGTYPE_GENERIC: c_int = 0;
pub const ACPI_NHLT_CONFIGTYPE_MICARRAY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_micdevice_config {
    pub virtual_slot: u8,
    pub config_type: u8,
    pub array_type: u8,
}

// Values for array_type field above
pub const ACPI_NHLT_ARRAYTYPE_LINEAR2_SMALL: c_uint = 0xA;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR2_BIG: c_uint = 0xB;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO1: c_uint = 0xC;
pub const ACPI_NHLT_ARRAYTYPE_PLANAR4_LSHAPED: c_uint = 0xD;
pub const ACPI_NHLT_ARRAYTYPE_LINEAR4_GEO2: c_uint = 0xE;
pub const ACPI_NHLT_ARRAYTYPE_VENDOR: c_uint = 0xF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_vendor_mic_config {
    pub type: u8,
    pub panel: u8,
    pub /: *mut *mut u16 speaker_position_distance; / mm,
    pub /: *mut *mut u16 horizontal_offset; / mm,
    pub /: *mut *mut u16 vertical_offset; / mm,
    pub /: *mut *mut *mut u8 frequency_low_band; / 5Hz,
    pub /: *mut *mut *mut u8 frequency_high_band; / 500Hz,
    pub /: *mut *mut u16 direction_angle; / -180 - +180,
    pub /: *mut *mut u16 elevation_angle; / -180 - +180,
    pub /: *mut *mut u16 work_vertical_angle_begin; / -180 - +180 with 2 deg step,
    pub /: *mut *mut u16 work_vertical_angle_end; / -180 - +180 with 2 deg step,
    pub /: *mut *mut u16 work_horizontal_angle_begin; / -180 - +180 with 2 deg step,
    pub /: *mut *mut u16 work_horizontal_angle_end; / -180 - +180 with 2 deg step,
}

// Values for Type field above
pub const ACPI_NHLT_MICTYPE_OMNIDIRECTIONAL: c_int = 0;
pub const ACPI_NHLT_MICTYPE_SUBCARDIOID: c_int = 1;
pub const ACPI_NHLT_MICTYPE_CARDIOID: c_int = 2;
pub const ACPI_NHLT_MICTYPE_SUPERCARDIOID: c_int = 3;
pub const ACPI_NHLT_MICTYPE_HYPERCARDIOID: c_int = 4;
pub const ACPI_NHLT_MICTYPE_8SHAPED: c_int = 5;
pub const ACPI_NHLT_MICTYPE_RESERVED: c_int = 6;
pub const ACPI_NHLT_MICTYPE_VENDORDEFINED: c_int = 7;
// Values for Panel field above
pub const ACPI_NHLT_MICLOCATION_TOP: c_int = 0;
pub const ACPI_NHLT_MICLOCATION_BOTTOM: c_int = 1;
pub const ACPI_NHLT_MICLOCATION_LEFT: c_int = 2;
pub const ACPI_NHLT_MICLOCATION_RIGHT: c_int = 3;
pub const ACPI_NHLT_MICLOCATION_FRONT: c_int = 4;
pub const ACPI_NHLT_MICLOCATION_REAR: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_vendor_micdevice_config {
    pub virtual_slot: u8,
    pub config_type: u8,
    pub array_type: u8,
    pub mics_count: u8,
    pub mics: [acpi_nhlt_vendor_mic_config; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_nhlt_device_config {
    pub virtual_slot: u8,
    pub gen: acpi_nhlt_gendevice_config,
    pub mic: acpi_nhlt_micdevice_config,
    pub vendor_mic: acpi_nhlt_vendor_micdevice_config,
}

// Inherited from Microsoft's WAVEFORMATEXTENSIBLE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_wave_formatext {
    pub format_tag: u16,
    pub channel_count: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub extra_format_size: u16,
    pub valid_bits_per_sample: u16,
    pub channel_mask: u32,
    pub subformat: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_format_config {
    pub format: acpi_nhlt_wave_formatext,
    pub config: acpi_nhlt_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_formats_config {
    pub formats_count: u8,
    pub formats: [acpi_nhlt_format_config; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_device_info {
    pub id: [u8; 16],
    pub instance_id: u8,
    pub port_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_nhlt_devices_info {
    pub devices_count: u8,
    pub devices: [acpi_nhlt_device_info; ],
}

//
// PCCT - Platform Communications Channel Table (ACPI 5.0)
// Version 2 (ACPI 6.2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_pcct {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub flags: u32,
    pub reserved: u64,
}

// Values for Flags field above
pub const ACPI_PCCT_DOORBELL: c_int = 1;
// Values for subtable type in struct acpi_subtable_header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_pcct_type {
    ACPI_PCCT_TYPE_GENERIC_SUBSPACE = 0,
    ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE = 1,
    ACPI_PCCT_TYPE_HW_REDUCED_SUBSPACE_TYPE2 = 2,	/* ACPI 6.1 */
    ACPI_PCCT_TYPE_EXT_PCC_MASTER_SUBSPACE = 3,	/* ACPI 6.2 */
    ACPI_PCCT_TYPE_EXT_PCC_SLAVE_SUBSPACE = 4,	/* ACPI 6.2 */
    ACPI_PCCT_TYPE_HW_REG_COMM_SUBSPACE = 5,	/* ACPI 6.4 */
    ACPI_PCCT_TYPE_RESERVED = 6	/* 6 and greater are reserved */
}

//
// PCCT Subtables, correspond to Type in struct acpi_subtable_header
//
// 0: Generic Communications Subspace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_subspace {
    pub header: acpi_subtable_header,
    pub reserved: [u8; 6],
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: acpi_generic_address,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

// 1: HW-reduced Communications Subspace (ACPI 5.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_hw_reduced {
    pub header: acpi_subtable_header,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved: u8,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: acpi_generic_address,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
}

// 2: HW-reduced Communications Subspace Type 2 (ACPI 6.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_hw_reduced_type2 {
    pub header: acpi_subtable_header,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved: u8,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: acpi_generic_address,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u16,
    pub platform_ack_register: acpi_generic_address,
    pub ack_preserve_mask: u64,
    pub ack_write_mask: u64,
}

// 3: Extended PCC Master Subspace Type 3 (ACPI 6.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_ext_pcc_master {
    pub header: acpi_subtable_header,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved1: u8,
    pub base_address: u64,
    pub length: u32,
    pub doorbell_register: acpi_generic_address,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u32,
    pub platform_ack_register: acpi_generic_address,
    pub ack_preserve_mask: u64,
    pub ack_set_mask: u64,
    pub reserved2: u64,
    pub cmd_complete_register: acpi_generic_address,
    pub cmd_complete_mask: u64,
    pub cmd_update_register: acpi_generic_address,
    pub cmd_update_preserve_mask: u64,
    pub cmd_update_set_mask: u64,
    pub error_status_register: acpi_generic_address,
    pub error_status_mask: u64,
}

// 4: Extended PCC Slave Subspace Type 4 (ACPI 6.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_ext_pcc_slave {
    pub header: acpi_subtable_header,
    pub platform_interrupt: u32,
    pub flags: u8,
    pub reserved1: u8,
    pub base_address: u64,
    pub length: u32,
    pub doorbell_register: acpi_generic_address,
    pub preserve_mask: u64,
    pub write_mask: u64,
    pub latency: u32,
    pub max_access_rate: u32,
    pub min_turnaround_time: u32,
    pub platform_ack_register: acpi_generic_address,
    pub ack_preserve_mask: u64,
    pub ack_set_mask: u64,
    pub reserved2: u64,
    pub cmd_complete_register: acpi_generic_address,
    pub cmd_complete_mask: u64,
    pub cmd_update_register: acpi_generic_address,
    pub cmd_update_preserve_mask: u64,
    pub cmd_update_set_mask: u64,
    pub error_status_register: acpi_generic_address,
    pub error_status_mask: u64,
}

// 5: HW Registers based Communications Subspace
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_hw_reg {
    pub header: acpi_subtable_header,
    pub version: u16,
    pub base_address: u64,
    pub length: u64,
    pub doorbell_register: acpi_generic_address,
    pub doorbell_preserve: u64,
    pub doorbell_write: u64,
    pub cmd_complete_register: acpi_generic_address,
    pub cmd_complete_mask: u64,
    pub error_status_register: acpi_generic_address,
    pub error_status_mask: u64,
    pub nominal_latency: u32,
    pub min_turnaround_time: u32,
}

// Values for doorbell flags above

//
// PCC memory structures (not part of the ACPI table)
//
// Shared Memory Region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_shared_memory {
    pub signature: u32,
    pub command: u16,
    pub status: u16,
}

// Extended PCC Subspace Shared Memory Region (ACPI 6.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcct_ext_pcc_shared_memory {
    pub signature: u32,
    pub flags: u32,
    pub length: u32,
    pub command: u32,
}

//
// PDTT - Platform Debug Trigger Table (ACPI 6.2)
// Version 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_pdtt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub trigger_count: u8,
    pub reserved: [u8; 3],
    pub array_offset: u32,
}

//
// PDTT Communication Channel Identifier Structure.
// The number of these structures is defined by trigger_count above,
// starting at array_offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pdtt_channel {
    pub subchannel_id: u8,
    pub flags: u8,
}

// Flags for above

//
// PHAT - Platform Health Assessment Table (ACPI 6.4)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_phat {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// Common header for PHAT subtables that follow main table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_phat_header {
    pub type: u16,
    pub length: u16,
    pub revision: u8,
}

// Values for Type field above
pub const ACPI_PHAT_TYPE_FW_VERSION_DATA: c_int = 0;
pub const ACPI_PHAT_TYPE_FW_HEALTH_DATA: c_int = 1;

//
// PHAT subtables, correspond to Type in struct acpi_phat_header
//
// 0: Firmware Version Data Record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_phat_version_data {
    pub header: acpi_phat_header,
    pub reserved: [u8; 3],
    pub element_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_phat_version_element {
    pub guid: [u8; 16],
    pub version_value: u64,
    pub producer_id: u32,
}

// 1: Firmware Health Data Record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_phat_health_data {
    pub header: acpi_phat_header,
    pub reserved: [u8; 2],
    pub health: u8,
    pub device_guid: [u8; 16],
    pub /: *mut *mut u32 device_specific_offset; / Zero if no Device-specific data,
}

// Values for Health field above
pub const ACPI_PHAT_ERRORS_FOUND: c_int = 0;
pub const ACPI_PHAT_NO_ERRORS: c_int = 1;
pub const ACPI_PHAT_UNKNOWN_ERRORS: c_int = 2;
pub const ACPI_PHAT_ADVISORY: c_int = 3;
//
// PMTT - Platform Memory Topology Table (ACPI 5.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_pmtt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub memory_device_count: u32,
//
// Immediately followed by:
// MEMORY_DEVICE memory_device_struct[memory_device_count];
//
}

// Common header for PMTT subtables that follow main table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pmtt_header {
    pub type: u8,
    pub reserved1: u8,
    pub length: u16,
    pub flags: u16,
    pub reserved2: u16,
    pub /: *mut *mut u32 memory_device_count; / Zero means no memory device structs follow,
//
// Immediately followed by:
// u8 type_specific_data[]
// MEMORY_DEVICE memory_device_struct[memory_device_count];
//
}

// Values for Type field above
pub const ACPI_PMTT_TYPE_SOCKET: c_int = 0;
pub const ACPI_PMTT_TYPE_CONTROLLER: c_int = 1;
pub const ACPI_PMTT_TYPE_DIMM: c_int = 2;

pub const ACPI_PMTT_TYPE_VENDOR: c_uint = 0xFF;
// Values for Flags field above
pub const ACPI_PMTT_TOP_LEVEL: c_uint = 0x0001;
pub const ACPI_PMTT_PHYSICAL: c_uint = 0x0002;
pub const ACPI_PMTT_MEMORY_TYPE: c_uint = 0x000C;
//
// PMTT subtables, correspond to Type in struct acpi_pmtt_header
//
// 0: Socket Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pmtt_socket {
    pub header: acpi_pmtt_header,
    pub socket_id: u16,
    pub reserved: u16,
}

//
// Immediately followed by:
// MEMORY_DEVICE memory_device_struct[memory_device_count];
//
// 1: Memory Controller subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pmtt_controller {
    pub header: acpi_pmtt_header,
    pub controller_id: u16,
    pub reserved: u16,
}

//
// Immediately followed by:
// MEMORY_DEVICE memory_device_struct[memory_device_count];
//
// 2: Physical Component Identifier (DIMM)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pmtt_physical_component {
    pub header: acpi_pmtt_header,
    pub bios_handle: u32,
}

// 0xFF: Vendor Specific Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pmtt_vendor_specific {
    pub header: acpi_pmtt_header,
    pub type_uuid: [u8; 16],
    pub specific: [u8; ],
//
// Immediately followed by:
// u8 vendor_specific_data[];
// MEMORY_DEVICE memory_device_struct[memory_device_count];
//
}

//
// PPTT - Processor Properties Topology Table (ACPI 6.2)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_pptt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_pptt_type {
    ACPI_PPTT_TYPE_PROCESSOR = 0,
    ACPI_PPTT_TYPE_CACHE = 1,
    ACPI_PPTT_TYPE_ID = 2,
    ACPI_PPTT_TYPE_RESERVED = 3
}

// 0: Processor Hierarchy Node Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pptt_processor {
    pub header: acpi_subtable_header,
    pub reserved: u16,
    pub flags: u32,
    pub parent: u32,
    pub acpi_processor_id: u32,
    pub number_of_priv_resources: u32,
}

// Flags

// 1: Cache Type Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pptt_cache {
    pub header: acpi_subtable_header,
    pub reserved: u16,
    pub flags: u32,
    pub next_level_of_cache: u32,
    pub size: u32,
    pub number_of_sets: u32,
    pub associativity: u8,
    pub attributes: u8,
    pub line_size: u16,
}

// 1: Cache Type Structure for PPTT version 3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pptt_cache_v1 {
    pub header: acpi_subtable_header,
    pub reserved: u16,
    pub flags: u32,
    pub next_level_of_cache: u32,
    pub size: u32,
    pub number_of_sets: u32,
    pub associativity: u8,
    pub attributes: u8,
    pub line_size: u16,
    pub cache_id: u32,
}

// Flags

// Masks for Attributes

// Attributes describing cache

// 2: ID Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pptt_id {
    pub header: acpi_subtable_header,
    pub reserved: u16,
    pub vendor_id: u32,
    pub level1_id: u64,
    pub level2_id: u64,
    pub major_rev: u16,
    pub minor_rev: u16,
    pub spin_rev: u16,
}

//
// PRMT - Platform Runtime Mechanism Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_prmt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_prmt_header {
    pub platform_guid: [u8; 16],
    pub module_info_offset: u32,
    pub module_info_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_prmt_module_header {
    pub revision: u16,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_prmt_module_info {
    pub revision: u16,
    pub length: u16,
    pub module_guid: [u8; 16],
    pub major_rev: u16,
    pub minor_rev: u16,
    pub handler_info_count: u16,
    pub handler_info_offset: u32,
    pub mmio_list_pointer: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_prmt_handler_info {
    pub revision: u16,
    pub length: u16,
    pub handler_guid: [u8; 16],
    pub handler_address: u64,
    pub static_data_buffer_address: u64,
    pub acpi_param_buffer_address: u64,
}

//
// RASF - RAS Feature Table (ACPI 5.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rasf {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub channel_id: [u8; 12],
}

// RASF Platform Communication Channel Shared Memory Region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rasf_shared_memory {
    pub signature: u32,
    pub command: u16,
    pub status: u16,
    pub version: u16,
    pub capabilities: [u8; 16],
    pub set_capabilities: [u8; 16],
    pub num_parameter_blocks: u16,
    pub set_capabilities_status: u32,
}

// RASF Parameter Block Structure Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rasf_parameter_block {
    pub type: u16,
    pub version: u16,
    pub length: u16,
}

// RASF Parameter Block Structure for PATROL_SCRUB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rasf_patrol_scrub_parameter {
    pub header: acpi_rasf_parameter_block,
    pub patrol_scrub_command: u16,
    pub requested_address_range: [u64; 2],
    pub actual_address_range: [u64; 2],
    pub flags: u16,
    pub requested_speed: u8,
}

// Masks for Flags and Speed fields above
pub const ACPI_RASF_SCRUBBER_RUNNING: c_int = 1;

// Channel Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rasf_commands {
    ACPI_RASF_EXECUTE_RASF_COMMAND = 1
}

// Platform RAS Capabilities
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rasf_capabiliities {
    ACPI_HW_PATROL_SCRUB_SUPPORTED = 0,
    ACPI_SW_PATROL_SCRUB_EXPOSED = 1
}

// Patrol Scrub Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rasf_patrol_scrub_commands {
    ACPI_RASF_GET_PATROL_PARAMETERS = 1,
    ACPI_RASF_START_PATROL_SCRUBBER = 2,
    ACPI_RASF_STOP_PATROL_SCRUBBER = 3
}

// Channel Command flags

// Status values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rasf_status {
    ACPI_RASF_SUCCESS = 0,
    ACPI_RASF_NOT_VALID = 1,
    ACPI_RASF_NOT_SUPPORTED = 2,
    ACPI_RASF_BUSY = 3,
    ACPI_RASF_FAILED = 4,
    ACPI_RASF_ABORTED = 5,
    ACPI_RASF_INVALID_DATA = 6
}

// Status flags

//
// RAS2 - RAS2 Feature Table (ACPI 6.5)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_ras2 {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u16,
    pub num_pcc_descs: u16,
}

// RAS2 Platform Communication Channel Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ras2_pcc_desc {
    pub channel_id: u8,
    pub reserved: u16,
    pub feature_type: u8,
    pub instance: u32,
}

// RAS2 Platform Communication Channel Shared Memory Region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ras2_shmem {
    pub signature: u32,
    pub command: u16,
    pub status: u16,
    pub version: u16,
    pub features: [u8; 16],
    pub set_caps: [u8; 16],
    pub num_param_blks: u16,
    pub set_caps_status: u32,
}

// RAS2 Parameter Block Structure for PATROL_SCRUB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ras2_parameter_block {
    pub type: u16,
    pub version: u16,
    pub length: u16,
}

// RAS2 Parameter Block Structure for PATROL_SCRUB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ras2_patrol_scrub_param {
    pub header: acpi_ras2_parameter_block,
    pub command: u16,
    pub req_addr_range: [u64; 2],
    pub actl_addr_range: [u64; 2],
    pub flags: u32,
    pub scrub_params_out: u32,
    pub scrub_params_in: u32,
    pub ext_scrub_params: u32,
    pub scrub_rate_desc: [u8; 256],
}

// Masks for Flags field above
pub const ACPI_RAS2_SCRUBBER_RUNNING: c_int = 1;
// RAS2 Parameter Block Structure for LA2PA_TRANSLATION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ras2_la2pa_translation_parameter {
    pub header: acpi_ras2_parameter_block,
    pub addr_translation_command: u16,
    pub sub_inst_id: u64,
    pub logical_address: u64,
    pub physical_address: u64,
    pub status: u32,
}

// Channel Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_commands {
    ACPI_RAS2_EXECUTE_RAS2_COMMAND = 1
}

// Platform RAS2 Features
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_features {
    ACPI_RAS2_PATROL_SCRUB_SUPPORTED = 0,
    ACPI_RAS2_LA2PA_TRANSLATION = 1
}

// RAS2 Patrol Scrub Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_patrol_scrub_commands {
    ACPI_RAS2_GET_PATROL_PARAMETERS = 1,
    ACPI_RAS2_START_PATROL_SCRUBBER = 2,
    ACPI_RAS2_STOP_PATROL_SCRUBBER = 3
}

// RAS2 LA2PA Translation Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_la2_pa_translation_commands {
    ACPI_RAS2_GET_LA2PA_TRANSLATION = 1,
}

// RAS2 LA2PA Translation Status values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_la2_pa_translation_status {
    ACPI_RAS2_LA2PA_TRANSLATION_SUCCESS = 0,
    ACPI_RAS2_LA2PA_TRANSLATION_FAIL = 1,
}

// Channel Command flags

// Status values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ras2_status {
    ACPI_RAS2_SUCCESS = 0,
    ACPI_RAS2_NOT_VALID = 1,
    ACPI_RAS2_NOT_SUPPORTED = 2,
    ACPI_RAS2_BUSY = 3,
    ACPI_RAS2_FAILED = 4,
    ACPI_RAS2_ABORTED = 5,
    ACPI_RAS2_INVALID_DATA = 6
}

// Status flags

//
// RGRT - Regulatory Graphics Resource Table
// Version 1
//
// Conforms to "ACPI RGRT" available at:
// https://microsoft.github.io/mu/dyn/mu_plus/ms_core_pkg/acpi_RGRT/feature_acpi_rgrt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rgrt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub version: u16,
    pub image_type: u8,
    pub reserved: u8,
    pub image: [u8; ],
}

// image_type values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rgrt_image_type {
    ACPI_RGRT_TYPE_RESERVED0 = 0,
    ACPI_RGRT_IMAGE_TYPE_PNG = 1,
    ACPI_RGRT_TYPE_RESERVED = 2	/* 2 and greater are reserved */
}

//
// RHCT - RISC-V Hart Capabilities Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rhct {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 flags; / RHCT flags,
    pub time_base_freq: u64,
    pub node_count: u32,
    pub node_offset: u32,
}

// RHCT Flags

//
// RHCT subtables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rhct_node_header {
    pub type: u16,
    pub length: u16,
    pub revision: u16,
}

// Values for RHCT subtable Type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rhct_node_type {
    ACPI_RHCT_NODE_TYPE_ISA_STRING = 0x0000,
    ACPI_RHCT_NODE_TYPE_CMO = 0x0001,
    ACPI_RHCT_NODE_TYPE_MMU = 0x0002,
    ACPI_RHCT_NODE_TYPE_RESERVED = 0x0003,
    ACPI_RHCT_NODE_TYPE_HART_INFO = 0xFFFF,
}

//
// RHCT node specific subtables
//
// ISA string node structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rhct_isa_string {
    pub isa_length: u16,
    pub isa: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rhct_cmo_node {
    pub /: *mut *mut u8 reserved; / Must be zero,
    pub /: *mut *mut u8 cbom_size; / CBOM size in powerof 2,
    pub /: *mut *mut u8 cbop_size; / CBOP size in powerof 2,
    pub /: *mut *mut u8 cboz_size; / CBOZ size in powerof 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rhct_mmu_node {
    pub /: *mut *mut u8 reserved; / Must be zero,
    pub /: *mut *mut u8 mmu_type; / Virtual Address Scheme,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rhct_mmu_type {
    ACPI_RHCT_MMU_TYPE_SV39 = 0,
    ACPI_RHCT_MMU_TYPE_SV48 = 1,
    ACPI_RHCT_MMU_TYPE_SV57 = 2
}

// Hart Info node structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rhct_hart_info {
    pub num_offsets: u16,
    pub /: *mut *mut u32 uid; / ACPI processor UID,
}

//
// RIMT - RISC-V IO Remapping Table
//
// https://github.com/riscv-non-isa/riscv-acpi-rimt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rimt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 num_nodes; / Number of RIMT Nodes,
    pub /: *mut *mut u32 node_offset; / Offset to RIMT Node Array,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_node {
    pub type: u8,
    pub revision: u8,
    pub length: u16,
    pub reserved: u16,
    pub id: u16,
    pub node_data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_rimt_node_type {
    ACPI_RIMT_NODE_TYPE_IOMMU = 0x0,
    ACPI_RIMT_NODE_TYPE_PCIE_ROOT_COMPLEX = 0x1,
    ACPI_RIMT_NODE_TYPE_PLAT_DEVICE = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_iommu {
    pub /: *mut *mut u8 hardware_id[8]; / Hardware ID,
    pub /: *mut *mut u64 base_address; / Base Address,
    pub /: *mut *mut u32 flags; / Flags,
    pub /: *mut *mut u32 proximity_domain; / Proximity Domain,
    pub /: *mut *mut u16 pcie_segment_number; / PCIe Segment number,
    pub /: *mut *mut u16 pcie_bdf; / PCIe B/D/F,
    pub /: *mut *mut u16 num_interrupt_wires; / Number of interrupt wires,
    pub /: *mut *mut u16 interrupt_wire_offset; / Interrupt wire array offset,
    pub /: *mut *mut u64 interrupt_wire[]; / Interrupt wire array,
}

// IOMMU Node Flags

// Interrupt Wire Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_iommu_wire_gsi {
    pub /: *mut *mut u32 irq_num; / Interrupt Number,
    pub /: *mut *mut u32 flags; / Flags,
}

// Interrupt Wire Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_id_mapping {
    pub /: *mut *mut u32 source_id_base; / Source ID Base,
    pub /: *mut *mut u32 num_ids; / Number of IDs,
    pub /: *mut *mut u32 dest_id_base; / Destination Device ID Base,
    pub /: *mut *mut u32 dest_offset; / Destination IOMMU Offset,
    pub /: *mut *mut u32 flags; / Flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_pcie_rc {
    pub /: *mut *mut u32 flags; / Flags,
    pub /: *mut *mut u16 reserved; / Reserved,
    pub /: *mut *mut u16 pcie_segment_number; / PCIe Segment number,
    pub /: *mut *mut u16 id_mapping_offset; / ID mapping array offset,
    pub /: *mut *mut u16 num_id_mappings; / Number of ID mappings,
}

// PCIe Root Complex Node Flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rimt_platform_device {
    pub /: *mut *mut u16 id_mapping_offset; / ID Mapping array offset,
    pub /: *mut *mut u16 num_id_mappings; / Number of ID mappings,
    pub /: *mut *mut char device_name[]; / Device Object Name,
}

//
// SBST - Smart Battery Specification Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_sbst {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub warning_level: u32,
    pub low_level: u32,
    pub critical_level: u32,
}

//
// SDEI - Software Delegated Exception Interface Descriptor Table
//
// Conforms to "Software Delegated Exception Interface (SDEI)" ARM DEN0054A,
// May 8th, 2017. Copyright 2017 ARM Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_sdei {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

//
// SDEV - Secure Devices Table (ACPI 6.2)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_sdev {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_header {
    pub type: u8,
    pub flags: u8,
    pub length: u16,
}

// Values for subtable type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_sdev_type {
    ACPI_SDEV_TYPE_NAMESPACE_DEVICE = 0,
    ACPI_SDEV_TYPE_PCIE_ENDPOINT_DEVICE = 1,
    ACPI_SDEV_TYPE_RESERVED = 2	/* 2 and greater are reserved */
}

// Values for flags above

//
// SDEV subtables
//
// 0: Namespace Device Based Secure Device Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_namespace {
    pub header: acpi_sdev_header,
    pub device_id_offset: u16,
    pub device_id_length: u16,
    pub vendor_data_offset: u16,
    pub vendor_data_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_secure_component {
    pub secure_component_offset: u16,
    pub secure_component_length: u16,
}

//
// SDEV sub-subtables ("Components") for above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_component {
    pub header: acpi_sdev_header,
}

// Values for sub-subtable type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_sac_type {
    ACPI_SDEV_TYPE_ID_COMPONENT = 0,
    ACPI_SDEV_TYPE_MEM_COMPONENT = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_id_component {
    pub header: acpi_sdev_header,
    pub hardware_id_offset: u16,
    pub hardware_id_length: u16,
    pub subsystem_id_offset: u16,
    pub subsystem_id_length: u16,
    pub hardware_revision: u16,
    pub hardware_rev_present: u8,
    pub class_code_present: u8,
    pub pci_base_class: u8,
    pub pci_sub_class: u8,
    pub pci_programming_xface: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_mem_component {
    pub header: acpi_sdev_header,
    pub reserved: u32,
    pub memory_base_address: u64,
    pub memory_length: u64,
}

// 1: PCIe Endpoint Device Based Device Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_pcie {
    pub header: acpi_sdev_header,
    pub segment: u16,
    pub start_bus: u16,
    pub path_offset: u16,
    pub path_length: u16,
    pub vendor_data_offset: u16,
    pub vendor_data_length: u16,
}

// 1a: PCIe Endpoint path entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sdev_pcie_path {
    pub device: u8,
    pub function: u8,
}

//
// SVKL - Storage Volume Key Location Table (ACPI 6.4)
// From: "Guest-Host-Communication Interface (GHCI) for Intel
// Trust Domain Extensions (Intel TDX)".
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_svkl {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_svkl_key {
    pub type: u16,
    pub format: u16,
    pub size: u32,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_svkl_type {
    ACPI_SVKL_TYPE_MAIN_STORAGE = 0,
    ACPI_SVKL_TYPE_RESERVED = 1	/* 1 and greater are reserved */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_svkl_format {
    ACPI_SVKL_FORMAT_RAW_BINARY = 0,
    ACPI_SVKL_FORMAT_RESERVED = 1	/* 1 and greater are reserved */
}

//
// SWFT - SoundWire File Table
//
// Conforms to "Discovery and Configuration (DisCo) Specification for SoundWire"
// Version 2.1, 2 October 2023
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sw_file {
    pub vendor_id: u16,
    pub file_id: u32,
    pub file_version: u16,
    pub file_length: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_swft {
    pub header: acpi_table_header,
    pub files: [acpi_sw_file; ],
}

//
// TDEL - TD-Event Log
// From: "Guest-Host-Communication Interface (GHCI) for Intel
// Trust Domain Extensions (Intel TDX)".
// September 2020
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tdel {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u32,
    pub log_area_minimum_length: u64,
    pub log_area_start_address: u64,
}

// Reset to default packing

