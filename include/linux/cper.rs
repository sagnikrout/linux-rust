//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cper.h
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
// UEFI Common Platform Error Record
//
// Copyright (C) 2010, Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//

// CPER record signature and the size

pub const CPER_SIG_SIZE: c_int = 4;
// Used in signature_end field in struct cper_record_header
pub const CPER_SIG_END: c_uint = 0xffffffff;
//
// CPER record header revision, used in revision field in struct
// cper_record_header
//
pub const CPER_RECORD_REV: c_uint = 0x0100;
//
// CPER record length contains the CPER fields which are relevant for further
// handling of a memory error in userspace (we don't carry all the fields
// defined in the UEFI spec because some of them don't make any sense.)
// Currently, a length of 256 should be more than enough.
//
pub const CPER_REC_LEN: c_int = 256;
//
// Severity definition for error_severity in struct cper_record_header
// and section_severity in struct cper_section_descriptor
//
// Validation bits definition for validation_bits in struct
// cper_record_header. If set, corresponding fields in struct
// cper_record_header contain valid information.
//
pub const CPER_VALID_PLATFORM_ID: c_uint = 0x0001;
pub const CPER_VALID_TIMESTAMP: c_uint = 0x0002;
pub const CPER_VALID_PARTITION_ID: c_uint = 0x0004;
//
// Notification type used to generate error record, used in
// notification_type in struct cper_record_header.  These UUIDs are defined
// in the UEFI spec v2.7, sec N.2.1.
//
// Corrected Machine Check

// Corrected Platform Error

// Machine Check Exception

// PCI Express Error

// INIT Record (for IPF)

// Non-Maskable Interrupt

// BOOT Error Record

// DMA Remapping Error

// CXL Protocol Error Section

// CXL Event record UUIDs are formatted as GUIDs and reported in section type
//
// General Media Event Record
// CXL rev 3.0 Section 8.2.9.2.1.1; Table 8-43
//

//
// DRAM Event Record
// CXL rev 3.0 section 8.2.9.2.1.2; Table 8-44
//

//
// Memory Module Event Record
// CXL rev 3.0 section 8.2.9.2.1.3; Table 8-45
//

//
// Flags bits definitions for flags in struct cper_record_header
// If set, the error has been recovered
//
pub const CPER_HW_ERROR_FLAGS_RECOVERED: c_uint = 0x1;
// If set, the error is for previous boot
pub const CPER_HW_ERROR_FLAGS_PREVERR: c_uint = 0x2;
// If set, the error is injected for testing
pub const CPER_HW_ERROR_FLAGS_SIMULATED: c_uint = 0x4;
//
// CPER section header revision, used in revision field in struct
// cper_section_descriptor
//
pub const CPER_SEC_REV: c_uint = 0x0100;
//
// Validation bits definition for validation_bits in struct
// cper_section_descriptor. If set, corresponding fields in struct
// cper_section_descriptor contain valid information.
//
pub const CPER_SEC_VALID_FRU_ID: c_uint = 0x1;
pub const CPER_SEC_VALID_FRU_TEXT: c_uint = 0x2;
//
// Flags bits definitions for flags in struct cper_section_descriptor
//
// If set, the section is associated with the error condition
// directly, and should be focused on
//
pub const CPER_SEC_PRIMARY: c_uint = 0x0001;
//
// If set, the error was not contained within the processor or memory
// hierarchy and the error may have propagated to persistent storage
// or network
//
pub const CPER_SEC_CONTAINMENT_WARNING: c_uint = 0x0002;
// If set, the component must be re-initialized or re-enabled prior to use
pub const CPER_SEC_RESET: c_uint = 0x0004;
// If set, Linux may choose to discontinue use of the resource
pub const CPER_SEC_ERROR_THRESHOLD_EXCEEDED: c_uint = 0x0008;
//
// If set, resource could not be queried for error information due to
// conflicts with other system software or resources. Some fields of
// the section will be invalid
//
pub const CPER_SEC_RESOURCE_NOT_ACCESSIBLE: c_uint = 0x0010;
//
// If set, action has been taken to ensure error containment (such as
// poisoning data), but the error has not been fully corrected and the
// data has not been consumed. Linux may choose to take further
// corrective action before the data is consumed
//
pub const CPER_SEC_LATENT_ERROR: c_uint = 0x0020;
//
// Section type definitions, used in section_type field in struct
// cper_section_descriptor.  These UUIDs are defined in the UEFI spec
// v2.7, sec N.2.2.
//
// Processor Generic

// Processor Specific: X86/X86_64

// Processor Specific: IA64

// Processor Specific: ARM

// Platform Memory

// Firmware Error Record Reference

// PCI/PCI-X Bus

// PCI Component/Device

// Intel VT for Directed I/O specific DMAr

// IOMMU specific DMAr

pub const CPER_PROC_VALID_TYPE: c_uint = 0x0001;
pub const CPER_PROC_VALID_ISA: c_uint = 0x0002;
pub const CPER_PROC_VALID_ERROR_TYPE: c_uint = 0x0004;
pub const CPER_PROC_VALID_OPERATION: c_uint = 0x0008;
pub const CPER_PROC_VALID_FLAGS: c_uint = 0x0010;
pub const CPER_PROC_VALID_LEVEL: c_uint = 0x0020;
pub const CPER_PROC_VALID_VERSION: c_uint = 0x0040;
pub const CPER_PROC_VALID_BRAND_INFO: c_uint = 0x0080;
pub const CPER_PROC_VALID_ID: c_uint = 0x0100;
pub const CPER_PROC_VALID_TARGET_ADDRESS: c_uint = 0x0200;
pub const CPER_PROC_VALID_REQUESTOR_ID: c_uint = 0x0400;
pub const CPER_PROC_VALID_RESPONDER_ID: c_uint = 0x0800;
pub const CPER_PROC_VALID_IP: c_uint = 0x1000;
pub const CPER_MEM_VALID_ERROR_STATUS: c_uint = 0x0001;
pub const CPER_MEM_VALID_PA: c_uint = 0x0002;
pub const CPER_MEM_VALID_PA_MASK: c_uint = 0x0004;
pub const CPER_MEM_VALID_NODE: c_uint = 0x0008;
pub const CPER_MEM_VALID_CARD: c_uint = 0x0010;
pub const CPER_MEM_VALID_MODULE: c_uint = 0x0020;
pub const CPER_MEM_VALID_BANK: c_uint = 0x0040;
pub const CPER_MEM_VALID_DEVICE: c_uint = 0x0080;
pub const CPER_MEM_VALID_ROW: c_uint = 0x0100;
pub const CPER_MEM_VALID_COLUMN: c_uint = 0x0200;
pub const CPER_MEM_VALID_BIT_POSITION: c_uint = 0x0400;
pub const CPER_MEM_VALID_REQUESTOR_ID: c_uint = 0x0800;
pub const CPER_MEM_VALID_RESPONDER_ID: c_uint = 0x1000;
pub const CPER_MEM_VALID_TARGET_ID: c_uint = 0x2000;
pub const CPER_MEM_VALID_ERROR_TYPE: c_uint = 0x4000;
pub const CPER_MEM_VALID_RANK_NUMBER: c_uint = 0x8000;
pub const CPER_MEM_VALID_CARD_HANDLE: c_uint = 0x10000;
pub const CPER_MEM_VALID_MODULE_HANDLE: c_uint = 0x20000;
pub const CPER_MEM_VALID_ROW_EXT: c_uint = 0x40000;
pub const CPER_MEM_VALID_BANK_GROUP: c_uint = 0x80000;
pub const CPER_MEM_VALID_BANK_ADDRESS: c_uint = 0x100000;
pub const CPER_MEM_VALID_CHIP_ID: c_uint = 0x200000;
pub const CPER_MEM_EXT_ROW_MASK: c_uint = 0x3;
pub const CPER_MEM_EXT_ROW_SHIFT: c_int = 16;
pub const CPER_MEM_BANK_ADDRESS_MASK: c_uint = 0xff;
pub const CPER_MEM_BANK_GROUP_SHIFT: c_int = 8;
pub const CPER_MEM_CHIP_ID_SHIFT: c_int = 5;
pub const CPER_PCIE_VALID_PORT_TYPE: c_uint = 0x0001;
pub const CPER_PCIE_VALID_VERSION: c_uint = 0x0002;
pub const CPER_PCIE_VALID_COMMAND_STATUS: c_uint = 0x0004;
pub const CPER_PCIE_VALID_DEVICE_ID: c_uint = 0x0008;
pub const CPER_PCIE_VALID_SERIAL_NUMBER: c_uint = 0x0010;
pub const CPER_PCIE_VALID_BRIDGE_CONTROL_STATUS: c_uint = 0x0020;
pub const CPER_PCIE_VALID_CAPABILITY: c_uint = 0x0040;
pub const CPER_PCIE_VALID_AER_INFO: c_uint = 0x0080;
pub const CPER_PCIE_SLOT_SHIFT: c_int = 3;

pub const CPER_ARM_ERR_TRANSACTION_SHIFT: c_int = 16;

pub const CPER_ARM_ERR_OPERATION_SHIFT: c_int = 18;

pub const CPER_ARM_ERR_LEVEL_SHIFT: c_int = 22;

pub const CPER_ARM_ERR_PC_CORRUPT_SHIFT: c_int = 25;

pub const CPER_ARM_ERR_CORRECTED_SHIFT: c_int = 26;

pub const CPER_ARM_ERR_PRECISE_PC_SHIFT: c_int = 27;

pub const CPER_ARM_ERR_RESTARTABLE_PC_SHIFT: c_int = 28;

pub const CPER_ARM_ERR_PARTICIPATION_TYPE_SHIFT: c_int = 29;

pub const CPER_ARM_ERR_TIME_OUT_SHIFT: c_int = 31;

pub const CPER_ARM_ERR_ADDRESS_SPACE_SHIFT: c_int = 32;

pub const CPER_ARM_ERR_MEM_ATTRIBUTES_SHIFT: c_int = 34;

pub const CPER_ARM_ERR_ACCESS_MODE_SHIFT: c_int = 43;

//
// All tables and structs must be byte-packed to match CPER
// specification, since the tables are provided by the system BIOS
//

// Record Header, UEFI v2.7 sec N.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_record_header {
    pub /: *mut *mut char signature[CPER_SIG_SIZE]; / must be CPER_SIG_RECORD,
    pub /: *mut *mut u16 revision; / must be CPER_RECORD_REV,
    pub /: *mut *mut u32 signature_end; / must be CPER_SIG_END,
    pub section_count: u16,
    pub error_severity: u32,
    pub validation_bits: u32,
    pub record_length: u32,
    pub timestamp: u64,
    pub platform_id: guid_t,
    pub partition_id: guid_t,
    pub creator_id: guid_t,
    pub notification_type: guid_t,
    pub record_id: u64,
    pub flags: u32,
    pub persistence_information: u64,
    pub /: *mut *mut u8 reserved[12]; / must be zero,
}

// Section Descriptor, UEFI v2.7 sec N.2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_section_descriptor {
    pub the: *mut *mut u32 section_offset; / Offset in bytes of,
// section body from the base
// of the record header
    pub section_length: u32,
    pub /: *mut *mut u16 revision; / must be CPER_RECORD_REV,
    pub validation_bits: u8,
    pub /: *mut *mut u8 reserved; / must be zero,
    pub flags: u32,
    pub section_type: guid_t,
    pub fru_id: guid_t,
    pub section_severity: u32,
    pub fru_text: [u8; 20],
}

// Generic Processor Error Section, UEFI v2.7 sec N.2.4.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_proc_generic {
    pub validation_bits: u64,
    pub proc_type: u8,
    pub proc_isa: u8,
    pub proc_error_type: u8,
    pub operation: u8,
    pub flags: u8,
    pub level: u8,
    pub reserved: u16,
    pub cpu_version: u64,
    pub cpu_brand: [c_char; 128],
    pub proc_id: u64,
    pub target_addr: u64,
    pub requestor_id: u64,
    pub responder_id: u64,
    pub ip: u64,
}

// IA32/X64 Processor Error Section, UEFI v2.7 sec N.2.4.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_proc_ia {
    pub validation_bits: u64,
    pub lapic_id: u64,
    pub cpuid: [u8; 48],
}

// IA32/X64 Processor Error Information Structure, UEFI v2.7 sec N.2.4.2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_ia_err_info {
    pub err_type: guid_t,
    pub validation_bits: u64,
    pub check_info: u64,
    pub target_id: u64,
    pub requestor_id: u64,
    pub responder_id: u64,
    pub ip: u64,
}

// IA32/X64 Processor Context Information Structure, UEFI v2.7 sec N.2.4.2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_ia_proc_ctx {
    pub reg_ctx_type: u16,
    pub reg_arr_size: u16,
    pub msr_addr: u32,
    pub mm_reg_addr: u64,
}

// ARM Processor Error Section, UEFI v2.7 sec N.2.4.4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_proc_arm {
    pub validation_bits: u32,
    pub /: *mut *mut u16 err_info_num; / Number of Processor Error Info,
    pub Records*/: *mut *mut u16 context_info_num; / Number of Processor Context Info,
    pub section_length: u32,
    pub affinity_level: u8,
    pub /: *mut *mut u8 reserved[3]; / must be zero,
    pub mpidr: u64,
    pub midr: u64,
    pub /: *mut *mut u32 running_state; / Bit 0 set - Processor running. PSCI = 0,
    pub psci_state: u32,
}

// ARM Processor Error Information Structure, UEFI v2.7 sec N.2.4.4.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_arm_err_info {
    pub version: u8,
    pub length: u8,
    pub validation_bits: u16,
    pub type: u8,
    pub multiple_error: u16,
    pub flags: u8,
    pub error_info: u64,
    pub virt_fault_addr: u64,
    pub physical_fault_addr: u64,
}

// ARM Processor Context Information Structure, UEFI v2.7 sec N.2.4.4.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_arm_ctx_info {
    pub version: u16,
    pub type: u16,
    pub size: u32,
}

// Old Memory Error Section, UEFI v2.1, v2.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_mem_err_old {
    pub validation_bits: u64,
    pub error_status: u64,
    pub physical_addr: u64,
    pub physical_addr_mask: u64,
    pub node: u16,
    pub card: u16,
    pub module: u16,
    pub bank: u16,
    pub device: u16,
    pub row: u16,
    pub column: u16,
    pub bit_pos: u16,
    pub requestor_id: u64,
    pub responder_id: u64,
    pub target_id: u64,
    pub error_type: u8,
}

// Memory Error Section (UEFI >= v2.3), UEFI v2.8 sec N.2.5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_mem_err {
    pub validation_bits: u64,
    pub error_status: u64,
    pub physical_addr: u64,
    pub physical_addr_mask: u64,
    pub node: u16,
    pub card: u16,
    pub module: u16,
    pub bank: u16,
    pub device: u16,
    pub row: u16,
    pub column: u16,
    pub bit_pos: u16,
    pub requestor_id: u64,
    pub responder_id: u64,
    pub target_id: u64,
    pub error_type: u8,
    pub extended: u8,
    pub rank: u16,
    pub /: *mut *mut u16 mem_array_handle; / "card handle" in UEFI 2.4,
    pub /: *mut *mut u16 mem_dev_handle; / "module handle" in UEFI 2.4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_mem_err_compact {
    pub validation_bits: u64,
    pub node: u16,
    pub card: u16,
    pub module: u16,
    pub bank: u16,
    pub device: u16,
    pub row: u16,
    pub column: u16,
    pub bit_pos: u16,
    pub requestor_id: u64,
    pub responder_id: u64,
    pub target_id: u64,
    pub rank: u16,
    pub mem_array_handle: u16,
    pub mem_dev_handle: u16,
    pub extended: u8,
}

// PCI Express Error Section, UEFI v2.7 sec N.2.7
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_pcie {
    pub validation_bits: u64,
    pub port_type: u32,
    pub minor: u8,
    pub major: u8,
    pub reserved: [u8; 2],
    pub version: },
    pub command: u16,
    pub status: u16,
    pub reserved: u32,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: [u8; 3],
    pub function: u8,
    pub device: u8,
    pub segment: u16,
    pub bus: u8,
    pub secondary_bus: u8,
    pub slot: u16,
    pub reserved: u8,
    pub device_id: },
    pub lower: u32,
    pub upper: u32,
    pub serial_number: },
    pub secondary_status: u16,
    pub control: u16,
    pub bridge: },
    pub capability: [u8; 60],
    pub aer_info: [u8; 96],
}

// Firmware Error Record Reference, UEFI v2.7 sec N.2.10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cper_sec_fw_err_rec_ref {
    pub record_type: u8,
    pub revision: u8,
    pub reserved: [u8; 6],
    pub record_identifier: u64,
    pub record_identifier_guid: guid_t,
}

// Reset to default packing

extern "C" {
    pub fn cper_next_record_id() -> u64;
}
extern "C" {
    pub fn cper_mem_err_location(mem: *mut cper_mem_err_compact, msg: *mut c_char) -> c_int;
}
extern "C" {
    pub fn cper_dimm_err_location(mem: *mut cper_mem_err_compact, msg: *mut c_char) -> c_int;
}
extern "C" {
    pub fn cper_estatus_check_header(estatus: *const acpi_hest_generic_status) -> c_int;
}
extern "C" {
    pub fn cper_estatus_check(estatus: *const acpi_hest_generic_status) -> c_int;
}
