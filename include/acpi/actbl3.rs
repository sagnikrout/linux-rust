//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/actbl3.h
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
// Name: actbl3.h - ACPI Table Definitions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Additional ACPI Tables
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
// SLIC - Software Licensing Description Table
//
// Conforms to "Microsoft Software Licensing Tables (SLIC and MSDM)",
// November 29, 2011. Copyright 2011 Microsoft
//
// Basic SLIC table is only the common ACPI header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_slic {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

//
// SLIT - System Locality Distance Information Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_slit {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub locality_count: u64,
    pub /: *mut *mut u8 entry[]; / Real size = localities^2,
}

//
// SPCR - Serial Port Console Redirection table
// Version 4
//
// Conforms to "Serial Port Console Redirection Table",
// Version 1.10, Jan 5, 2023
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_spcr {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 interface_type; / 0=full 16550, 1=subset of 16550,
    pub reserved: [u8; 3],
    pub serial_port: acpi_generic_address,
    pub interrupt_type: u8,
    pub pc_interrupt: u8,
    pub interrupt: u32,
    pub baud_rate: u8,
    pub parity: u8,
    pub stop_bits: u8,
    pub flow_control: u8,
    pub terminal_type: u8,
    pub language: u8,
    pub pci_device_id: u16,
    pub pci_vendor_id: u16,
    pub pci_bus: u8,
    pub pci_device: u8,
    pub pci_function: u8,
    pub pci_flags: u32,
    pub pci_segment: u8,
    pub uart_clk_freq: u32,
    pub precise_baudrate: u32,
    pub name_space_string_length: u16,
    pub name_space_string_offset: u16,
    pub name_space_string: [c_char; ],
}

// Masks for pci_flags field above

// Values for Interface Type: See the definition of the DBG2 table
//
// SPMI - Server Platform Management Interface table
// Version 5
//
// Conforms to "Intelligent Platform Management Interface Specification
// Second Generation v2.0", Document Revision 1.0, February 12, 2004 with
// June 12, 2009 markup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_spmi {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub interface_type: u8,
    pub /: *mut *mut u8 reserved; / Must be 1,
    pub /: *mut *mut u16 spec_revision; / Version of IPMI,
    pub interrupt_type: u8,
    pub /: *mut *mut u8 gpe_number; / GPE assigned,
    pub reserved1: u8,
    pub pci_device_flag: u8,
    pub interrupt: u32,
    pub ipmi_register: acpi_generic_address,
    pub pci_segment: u8,
    pub pci_bus: u8,
    pub pci_device: u8,
    pub pci_function: u8,
    pub reserved2: u8,
}

// Values for interface_type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_spmi_interface_types {
    ACPI_SPMI_NOT_USED = 0,
    ACPI_SPMI_KEYBOARD = 1,
    ACPI_SPMI_SMI = 2,
    ACPI_SPMI_BLOCK_TRANSFER = 3,
    ACPI_SPMI_SMBUS = 4,
    ACPI_SPMI_RESERVED = 5	/* 5 and above are reserved */
}

//
// SRAT - System Resource Affinity Table
// Version 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_srat {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 table_revision; / Must be value '1',
    pub /: *mut *mut u64 reserved; / Reserved, must be zero,
}

// Values for subtable type in struct acpi_subtable_header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_srat_type {
    ACPI_SRAT_TYPE_CPU_AFFINITY = 0,
    ACPI_SRAT_TYPE_MEMORY_AFFINITY = 1,
    ACPI_SRAT_TYPE_X2APIC_CPU_AFFINITY = 2,
    ACPI_SRAT_TYPE_GICC_AFFINITY = 3,
    ACPI_SRAT_TYPE_GIC_ITS_AFFINITY = 4,	/* ACPI 6.2 */
    ACPI_SRAT_TYPE_GENERIC_AFFINITY = 5,	/* ACPI 6.3 */
    ACPI_SRAT_TYPE_GENERIC_PORT_AFFINITY = 6,	/* ACPI 6.4 */
    ACPI_SRAT_TYPE_RINTC_AFFINITY = 7,	/* ACPI 6.6 */
    ACPI_SRAT_TYPE_RESERVED = 8	/* 8 and greater are reserved */
}

//
// SRAT Subtables, correspond to Type in struct acpi_subtable_header
//
// 0: Processor Local APIC/SAPIC Affinity
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_cpu_affinity {
    pub header: acpi_subtable_header,
    pub proximity_domain_lo: u8,
    pub apic_id: u8,
    pub flags: u32,
    pub local_sapic_eid: u8,
    pub proximity_domain_hi: [u8; 3],
    pub clock_domain: u32,
}

// Flags

// 1: Memory Affinity
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_mem_affinity {
    pub header: acpi_subtable_header,
    pub proximity_domain: u32,
    pub /: *mut *mut u16 reserved; / Reserved, must be zero,
    pub base_address: u64,
    pub length: u64,
    pub reserved1: u32,
    pub flags: u32,
    pub /: *mut *mut u64 reserved2; / Reserved, must be zero,
}

// Flags

// 2: Processor Local X2_APIC Affinity (ACPI 4.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_x2apic_cpu_affinity {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u16 reserved; / Reserved, must be zero,
    pub proximity_domain: u32,
    pub apic_id: u32,
    pub flags: u32,
    pub clock_domain: u32,
    pub reserved2: u32,
}

// Flags for struct acpi_srat_cpu_affinity and struct acpi_srat_x2apic_cpu_affinity

// 3: GICC Affinity (ACPI 5.1)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_gicc_affinity {
    pub header: acpi_subtable_header,
    pub proximity_domain: u32,
    pub acpi_processor_uid: u32,
    pub flags: u32,
    pub clock_domain: u32,
}

// Flags for struct acpi_srat_gicc_affinity

// 4: GIC ITS Affinity (ACPI 6.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_gic_its_affinity {
    pub header: acpi_subtable_header,
    pub proximity_domain: u32,
    pub reserved: u16,
    pub its_id: u32,
}

//
// Common structure for SRAT subtable types:
// 5: ACPI_SRAT_TYPE_GENERIC_AFFINITY
// 6: ACPI_SRAT_TYPE_GENERIC_PORT_AFFINITY
//
pub const ACPI_SRAT_DEVICE_HANDLE_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_generic_affinity {
    pub header: acpi_subtable_header,
    pub reserved: u8,
    pub device_handle_type: u8,
    pub proximity_domain: u32,
    pub device_handle: [u8; ACPI_SRAT_DEVICE_HANDLE_SIZE],
    pub flags: u32,
    pub reserved1: u32,
}

// Flags for struct acpi_srat_generic_affinity

// 7: RINTC Affinity Structure(ACPI 6.6)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_srat_rintc_affinity {
    pub header: acpi_subtable_header,
    pub reserved: u16,
    pub proximity_domain: u32,
    pub acpi_processor_uid: u32,
    pub flags: u32,
    pub clock_domain: u32,
}

// Flags for struct acpi_srat_rintc_affinity

//
// STAO - Status Override Table (_STA override) - ACPI 6.0
// Version 1
//
// Conforms to "ACPI Specification for Status Override Table"
// 6 January 2015
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_stao {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub ignore_uart: u8,
}

//
// TCPA - Trusted Computing Platform Alliance table
// Version 2
//
// TCG Hardware Interface Table for TPM 1.2 Clients and Servers
//
// Conforms to "TCG ACPI Specification, Family 1.2 and 2.0",
// Version 1.2, Revision 8
// February 27, 2017
//
// NOTE: There are two versions of the table with the same signature --
// the client version and the server version. The common platform_class
// field is used to differentiate the two types of tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tcpa_hdr {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub platform_class: u16,
}

//
// Values for platform_class above.
// This is how the client and server subtables are differentiated
//
pub const ACPI_TCPA_CLIENT_TABLE: c_int = 0;
pub const ACPI_TCPA_SERVER_TABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tcpa_client {
    pub /: *mut *mut u32 minimum_log_length; / Minimum length for the event log area,
    pub /: *mut *mut u64 log_address; / Address of the event log area,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tcpa_server {
    pub reserved: u16,
    pub /: *mut *mut u64 minimum_log_length; / Minimum length for the event log area,
    pub /: *mut *mut u64 log_address; / Address of the event log area,
    pub spec_revision: u16,
    pub device_flags: u8,
    pub interrupt_flags: u8,
    pub gpe_number: u8,
    pub reserved2: [u8; 3],
    pub global_interrupt: u32,
    pub address: acpi_generic_address,
    pub reserved3: u32,
    pub config_address: acpi_generic_address,
    pub group: u8,
    pub /: *mut *mut u8 bus; / PCI Bus/Segment/Function numbers,
    pub device: u8,
    pub function: u8,
}

// Values for device_flags above

// Values for interrupt_flags above

//
// TPM2 - Trusted Platform Module (TPM) 2.0 Hardware Interface Table
// Version 4
//
// TCG Hardware Interface Table for TPM 2.0 Clients and Servers
//
// Conforms to "TCG ACPI Specification, Family 1.2 and 2.0",
// Version 1.2, Revision 8
// February 27, 2017
//
// Revision 3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tpm23 {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u32,
    pub control_address: u64,
    pub start_method: u32,
}

// Value for start_method above
pub const ACPI_TPM23_ACPI_START_METHOD: c_int = 2;
//
// Optional trailer for revision 3. If start method is 2, there is a 4 byte
// reserved area of all zeros.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tmp23_trailer {
    pub reserved: u32,
}

// Revision 4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_tpm2 {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub platform_class: u16,
    pub reserved: u16,
    pub control_address: u64,
    pub start_method: u32,
// Platform-specific data follows
}

// Optional trailer for revision 4 holding platform-specific data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpm2_phy {
    pub start_method_specific: [u8; 12],
    pub log_area_minimum_length: u32,
    pub log_area_start_address: u64,
}

// Values for start_method above
pub const ACPI_TPM2_NOT_ALLOWED: c_int = 0;
pub const ACPI_TPM2_RESERVED1: c_int = 1;
pub const ACPI_TPM2_START_METHOD: c_int = 2;
pub const ACPI_TPM2_RESERVED3: c_int = 3;
pub const ACPI_TPM2_RESERVED4: c_int = 4;
pub const ACPI_TPM2_RESERVED5: c_int = 5;
pub const ACPI_TPM2_MEMORY_MAPPED: c_int = 6;
pub const ACPI_TPM2_COMMAND_BUFFER: c_int = 7;
pub const ACPI_TPM2_COMMAND_BUFFER_WITH_START_METHOD: c_int = 8;
pub const ACPI_TPM2_RESERVED9: c_int = 9;
pub const ACPI_TPM2_RESERVED10: c_int = 10;

pub const ACPI_TPM2_RESERVED: c_int = 12;
pub const ACPI_TPM2_COMMAND_BUFFER_WITH_PLUTON: c_int = 13;
pub const ACPI_TPM2_CRB_WITH_ARM_FFA: c_int = 15;
// Optional trailer appears after any start_method subtables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpm2_trailer {
    pub method_parameters: [u8; 12],
    pub /: *mut *mut u32 minimum_log_length; / Minimum length for the event log area,
    pub /: *mut *mut u64 log_address; / Address of the event log area,
}

//
// Subtables (start_method-specific)
//
// 11: Start Method for ARM SMC (V1.2 Rev 8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpm2_arm_smc {
    pub global_interrupt: u32,
    pub interrupt_flags: u8,
    pub operation_flags: u8,
    pub reserved: u16,
    pub function_id: u32,
}

// Values for interrupt_flags above

// Values for operation_flags above

//
// UEFI - UEFI Boot optimization Table
// Version 1
//
// Conforms to "Unified Extensible Firmware Interface Specification",
// Version 2.3, May 8, 2009
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_uefi {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 identifier[16]; / UUID identifier,
    pub /: *mut *mut u16 data_offset; / Offset of remaining data in table,
}

//
// VIOT - Virtual I/O Translation Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_viot {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub node_count: u16,
    pub node_offset: u16,
    pub reserved: [u8; 8],
}

// VIOT subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_viot_header {
    pub type: u8,
    pub reserved: u8,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_viot_node_type {
    ACPI_VIOT_NODE_PCI_RANGE = 0x01,
    ACPI_VIOT_NODE_MMIO = 0x02,
    ACPI_VIOT_NODE_VIRTIO_IOMMU_PCI = 0x03,
    ACPI_VIOT_NODE_VIRTIO_IOMMU_MMIO = 0x04,
    ACPI_VIOT_RESERVED = 0x05
}

// VIOT subtables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_viot_pci_range {
    pub header: acpi_viot_header,
    pub endpoint_start: u32,
    pub segment_start: u16,
    pub segment_end: u16,
    pub bdf_start: u16,
    pub bdf_end: u16,
    pub output_node: u16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_viot_mmio {
    pub header: acpi_viot_header,
    pub endpoint: u32,
    pub base_address: u64,
    pub output_node: u16,
    pub reserved: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_viot_virtio_iommu_pci {
    pub header: acpi_viot_header,
    pub segment: u16,
    pub bdf: u16,
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_viot_virtio_iommu_mmio {
    pub header: acpi_viot_header,
    pub reserved: [u8; 4],
    pub base_address: u64,
}

//
// WAET - Windows ACPI Emulated devices Table
// Version 1
//
// Conforms to "Windows ACPI Emulated Devices Table", version 1.0, April 6, 2009
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_waet {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub flags: u32,
}

// Masks for Flags field above

//
// WDAT - Watchdog Action Table
// Version 1
//
// Conforms to "Hardware Watchdog Timers Design Specification",
// Copyright 2006 Microsoft Corporation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_wdat {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 header_length; / Watchdog Header Length,
    pub /: *mut *mut u16 pci_segment; / PCI Segment number,
    pub /: *mut *mut u8 pci_bus; / PCI Bus number,
    pub /: *mut *mut u8 pci_device; / PCI Device number,
    pub /: *mut *mut u8 pci_function; / PCI Function number,
    pub reserved: [u8; 3],
    pub /: *mut *mut u32 timer_period; / Period of one timer count (msec),
    pub /: *mut *mut u32 max_count; / Maximum counter value supported,
    pub /: *mut *mut u32 min_count; / Minimum counter value,
    pub flags: u8,
    pub reserved2: [u8; 3],
    pub /: *mut *mut u32 entries; / Number of watchdog entries that follow,
}

// Masks for Flags field above

pub const ACPI_WDAT_STOPPED: c_uint = 0x80;
// WDAT Instruction Entries (actions)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_wdat_entry {
    pub action: u8,
    pub instruction: u8,
    pub reserved: u16,
    pub register_region: acpi_generic_address,
    pub /: *mut *mut u32 value; / Value used with Read/Write register,
    pub /: *mut *mut u32 mask; / Bitmask required for this register instruction,
}

// Values for Action field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_wdat_actions {
    ACPI_WDAT_RESET = 1,
    ACPI_WDAT_GET_CURRENT_COUNTDOWN = 4,
    ACPI_WDAT_GET_COUNTDOWN = 5,
    ACPI_WDAT_SET_COUNTDOWN = 6,
    ACPI_WDAT_GET_RUNNING_STATE = 8,
    ACPI_WDAT_SET_RUNNING_STATE = 9,
    ACPI_WDAT_GET_STOPPED_STATE = 10,
    ACPI_WDAT_SET_STOPPED_STATE = 11,
    ACPI_WDAT_GET_REBOOT = 16,
    ACPI_WDAT_SET_REBOOT = 17,
    ACPI_WDAT_GET_SHUTDOWN = 18,
    ACPI_WDAT_SET_SHUTDOWN = 19,
    ACPI_WDAT_GET_STATUS = 32,
    ACPI_WDAT_SET_STATUS = 33,
    ACPI_WDAT_ACTION_RESERVED = 34	/* 34 and greater are reserved */
}

// Values for Instruction field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_wdat_instructions {
    ACPI_WDAT_READ_VALUE = 0,
    ACPI_WDAT_READ_COUNTDOWN = 1,
    ACPI_WDAT_WRITE_VALUE = 2,
    ACPI_WDAT_WRITE_COUNTDOWN = 3,
    ACPI_WDAT_INSTRUCTION_RESERVED = 4,	/* 4 and greater are reserved */
    ACPI_WDAT_PRESERVE_REGISTER = 0x80	/* Except for this value */
}

//
// WDDT - Watchdog Descriptor Table
// Version 1
//
// Conforms to "Using the Intel ICH Family Watchdog Timer (WDT)",
// Version 001, September 2002
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_wddt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub spec_version: u16,
    pub table_version: u16,
    pub pci_vendor_id: u16,
    pub address: acpi_generic_address,
    pub /: *mut *mut u16 max_count; / Maximum counter value supported,
    pub /: *mut *mut u16 min_count; / Minimum counter value supported,
    pub period: u16,
    pub status: u16,
    pub capability: u16,
}

// Flags for Status field above

// Flags for Capability field above

//
// WDRT - Watchdog Resource Table
// Version 1
//
// Conforms to "Watchdog Timer Hardware Requirements for Windows Server 2003",
// Version 1.01, August 28, 2006
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_wdrt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub control_register: acpi_generic_address,
    pub count_register: acpi_generic_address,
    pub pci_device_id: u16,
    pub pci_vendor_id: u16,
    pub /: *mut *mut u8 pci_bus; / PCI Bus number,
    pub /: *mut *mut u8 pci_device; / PCI Device number,
    pub /: *mut *mut u8 pci_function; / PCI Function number,
    pub /: *mut *mut u8 pci_segment; / PCI Segment number,
    pub /: *mut *mut u16 max_count; / Maximum counter value supported,
    pub units: u8,
}

//
// WPBT - Windows Platform Environment Table (ACPI 6.0)
// Version 1
//
// Conforms to "Windows Platform Binary Table (WPBT)" 29 November 2011
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_wpbt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub handoff_size: u32,
    pub handoff_address: u64,
    pub layout: u8,
    pub type: u8,
    pub arguments_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_wpbt_unicode {
    pub unicode_string: *mut u16,
}

//
// WSMT - Windows SMM Security Mitigations Table
// Version 1
//
// Conforms to "Windows SMM Security Mitigations Table",
// Version 1.0, April 18, 2016
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_wsmt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub protection_flags: u32,
}

// Flags for protection_flags field above

//
// XENV - Xen Environment Table (ACPI 6.0)
// Version 1
//
// Conforms to "ACPI Specification for Xen Environment Table" 4 January 2015
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_xenv {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub grant_table_address: u64,
    pub grant_table_size: u64,
    pub event_interrupt: u32,
    pub event_flags: u8,
}

// Reset to default packing

