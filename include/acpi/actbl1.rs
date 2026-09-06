//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/actbl1.h
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
// Name: actbl1.h - Additional ACPI table definitions
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

// Reserved table signatures

//
// These tables have been seen in the field, but no definition has been found
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
// Common subtable headers
//
// Generic subtable header (used in MADT, SRAT, etc.)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_subtable_header {
    pub type: u8,
    pub length: u8,
}

// Subtable header for WHEA tables (EINJ, ERST, WDAT)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_whea_header {
    pub action: u8,
    pub instruction: u8,
    pub flags: u8,
    pub reserved: u8,
    pub register_region: acpi_generic_address,
    pub /: *mut *mut u64 value; / Value used with Read/Write register,
    pub /: *mut *mut u64 mask; / Bitmask required for this register instruction,
}

// https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/acpitabl/ns-acpitabl-aspt_table
pub const ASPT_REVISION_ID: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_aspt {
    pub header: acpi_table_header,
    pub num_entries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aspt_header {
    pub type: u16,
    pub length: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_aspt_type {
    ACPI_ASPT_TYPE_GLOBAL_REGS = 0,
    ACPI_ASPT_TYPE_SEV_MBOX_REGS = 1,
    ACPI_ASPT_TYPE_ACPI_MBOX_REGS = 2,
}

// 0: ASPT Global Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aspt_global_regs {
    pub header: acpi_aspt_header,
    pub reserved: u32,
    pub feature_reg_addr: u64,
    pub irq_en_reg_addr: u64,
    pub irq_st_reg_addr: u64,
}

// 1: ASPT SEV Mailbox Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aspt_sev_mbox_regs {
    pub header: acpi_aspt_header,
    pub mbox_irq_id: u8,
    pub reserved: [u8; 3],
    pub cmd_resp_reg_addr: u64,
    pub cmd_buf_lo_reg_addr: u64,
    pub cmd_buf_hi_reg_addr: u64,
}

// 2: ASPT ACPI Mailbox Registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_aspt_acpi_mbox_regs {
    pub header: acpi_aspt_header,
    pub reserved1: u32,
    pub cmd_resp_reg_addr: u64,
    pub reserved2: [u64; 2],
}

// Larger subtable header (when Length can exceed 255)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_subtbl_hdr_16 {
    pub type: u16,
    pub length: u16,
}

//
// ASF - Alert Standard Format table (Signature "ASF!")
// Revision 0x10
//
// Conforms to the Alert Standard Format Specification V2.0, 23 April 2003
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_asf {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// ASF subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_header {
    pub type: u8,
    pub reserved: u8,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_asf_type {
    ACPI_ASF_TYPE_INFO = 0,
    ACPI_ASF_TYPE_ALERT = 1,
    ACPI_ASF_TYPE_CONTROL = 2,
    ACPI_ASF_TYPE_BOOT = 3,
    ACPI_ASF_TYPE_ADDRESS = 4,
    ACPI_ASF_TYPE_RESERVED = 5
}

//
// ASF subtables
//
// 0: ASF Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_info {
    pub header: acpi_asf_header,
    pub min_reset_value: u8,
    pub min_poll_interval: u8,
    pub system_id: u16,
    pub mfg_id: u32,
    pub flags: u8,
    pub reserved2: [u8; 3],
}

// Masks for Flags field above

// 1: ASF Alerts
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_alert {
    pub header: acpi_asf_header,
    pub assert_mask: u8,
    pub deassert_mask: u8,
    pub alerts: u8,
    pub data_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_alert_data {
    pub address: u8,
    pub command: u8,
    pub mask: u8,
    pub value: u8,
    pub sensor_type: u8,
    pub type: u8,
    pub offset: u8,
    pub source_type: u8,
    pub severity: u8,
    pub sensor_number: u8,
    pub entity: u8,
    pub instance: u8,
}

// 2: ASF Remote Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_remote {
    pub header: acpi_asf_header,
    pub controls: u8,
    pub data_length: u8,
    pub reserved2: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_control_data {
    pub function: u8,
    pub address: u8,
    pub command: u8,
    pub value: u8,
}

// 3: ASF RMCP Boot Options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_rmcp {
    pub header: acpi_asf_header,
    pub capabilities: [u8; 7],
    pub completion_code: u8,
    pub enterprise_id: u32,
    pub command: u8,
    pub parameter: u16,
    pub boot_options: u16,
    pub oem_parameters: u16,
}

// 4: ASF Address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_asf_address {
    pub header: acpi_asf_header,
    pub eprom_address: u8,
    pub devices: u8,
}

//
// BERT - Boot Error Record Table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_bert {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 region_length; / Length of the boot error region,
    pub /: *mut *mut u64 address; / Physical address of the error region,
}

// Boot Error Region (not a subtable, pointed to by Address field above)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_bert_region {
    pub /: *mut *mut u32 block_status; / Type of error information,
    pub /: *mut *mut u32 raw_data_offset; / Offset to raw error data,
    pub /: *mut *mut u32 raw_data_length; / Length of raw error data,
    pub /: *mut *mut u32 data_length; / Length of generic error data,
    pub /: *mut *mut u32 error_severity; / Severity code,
}

// Values for block_status flags above

// Values for error_severity above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_bert_error_severity {
    ACPI_BERT_ERROR_CORRECTABLE = 0,
    ACPI_BERT_ERROR_FATAL = 1,
    ACPI_BERT_ERROR_CORRECTED = 2,
    ACPI_BERT_ERROR_NONE = 3,
    ACPI_BERT_ERROR_RESERVED = 4	/* 4 and greater are reserved */
}

//
// Note: The generic error data that follows the error_severity field above
// uses the struct acpi_hest_generic_data defined under the HEST table below
//
// BGRT - Boot Graphics Resource Table (ACPI 5.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_bgrt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub version: u16,
    pub status: u8,
    pub image_type: u8,
    pub image_address: u64,
    pub image_offset_x: u32,
    pub image_offset_y: u32,
}

// Flags for Status field above

//
// BOOT - Simple Boot Flag Table
// Version 1
//
// Conforms to the "Simple Boot Flag Specification", Version 2.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_boot {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 cmos_index; / Index in CMOS RAM for the boot register,
    pub reserved: [u8; 3],
}

//
// CDAT - Coherent Device Attribute Table
// Version 1
//
// Conforms to the "Coherent Device Attribute Table (CDAT) Specification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_cdat {
    pub /: *mut *mut u32 length; / Length of table in bytes, including this header,
    pub /: *mut *mut u8 revision; / ACPI Specification minor version number,
    pub /: *mut *mut u8 checksum; / To make sum of entire table == 0,
    pub reserved: [u8; 6],
    pub /: *mut *mut u32 sequence; / Used to detect runtime CDAT table changes,
}

// CDAT common subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_header {
    pub type: u8,
    pub reserved: u8,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_cdat_type {
    ACPI_CDAT_TYPE_DSMAS = 0,
    ACPI_CDAT_TYPE_DSLBIS = 1,
    ACPI_CDAT_TYPE_DSMSCIS = 2,
    ACPI_CDAT_TYPE_DSIS = 3,
    ACPI_CDAT_TYPE_DSEMTS = 4,
    ACPI_CDAT_TYPE_SSLBIS = 5,
    ACPI_CDAT_TYPE_RESERVED = 6	/* 6 through 0xFF are reserved */
}

// Subtable 0: Device Scoped Memory Affinity Structure (DSMAS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_dsmas {
    pub dsmad_handle: u8,
    pub flags: u8,
    pub reserved: u16,
    pub dpa_base_address: u64,
    pub dpa_length: u64,
}

// Flags for subtable above

// Subtable 1: Device scoped Latency and Bandwidth Information Structure (DSLBIS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_dslbis {
    pub handle: u8,
    pub matches: *mut *mut u8 flags; / If Handle matches a DSMAS handle, the definition of this field,
// Flags field in HMAT System Locality Latency
    pub data_type: u8,
    pub reserved: u8,
    pub entry_base_unit: u64,
    pub entry: [u16; 3],
    pub reserved2: u16,
}

// Subtable 2: Device Scoped Memory Side Cache Information Structure (DSMSCIS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_dsmscis {
    pub dsmas_handle: u8,
    pub reserved: [u8; 3],
    pub side_cache_size: u64,
    pub cache_attributes: u32,
}

// Subtable 3: Device Scoped Initiator Structure (DSIS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_dsis {
    pub flags: u8,
    pub handle: u8,
    pub reserved: u16,
}

// Flags for above subtable

// Subtable 4: Device Scoped EFI Memory Type Structure (DSEMTS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_dsemts {
    pub dsmas_handle: u8,
    pub memory_type: u8,
    pub reserved: u16,
    pub dpa_offset: u64,
    pub range_length: u64,
}

// Subtable 5: Switch Scoped Latency and Bandwidth Information Structure (SSLBIS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_sslbis {
    pub data_type: u8,
    pub reserved: [u8; 3],
    pub entry_base_unit: u64,
}

// Sub-subtable for above, sslbe_entries field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cdat_sslbe {
    pub portx_id: u16,
    pub porty_id: u16,
    pub latency_or_bandwidth: u16,
    pub reserved: u16,
}

pub const ACPI_CDAT_SSLBIS_US_PORT: c_uint = 0x0100;
pub const ACPI_CDAT_SSLBIS_ANY_PORT: c_uint = 0xffff;
//
// CEDT - CXL Early Discovery Table
// Version 1
//
// Conforms to the "CXL Early Discovery Table" (CXL 2.0, October 2020)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_cedt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// CEDT subtable header (Performance Record Structure)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_header {
    pub type: u8,
    pub reserved: u8,
    pub length: u16,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_cedt_type {
    ACPI_CEDT_TYPE_CHBS = 0,
    ACPI_CEDT_TYPE_CFMWS = 1,
    ACPI_CEDT_TYPE_CXIMS = 2,
    ACPI_CEDT_TYPE_RDPAS = 3,
    ACPI_CEDT_TYPE_RESERVED = 4,
}

// Values for version field above

// Values for length field above

//
// CEDT subtables
//
// 0: CXL Host Bridge Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_chbs {
    pub header: acpi_cedt_header,
    pub uid: u32,
    pub cxl_version: u32,
    pub reserved: u32,
    pub base: u64,
    pub length: u64,
}

// 1: CXL Fixed Memory Window Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_cfmws {
    pub header: acpi_cedt_header,
    pub reserved1: u32,
    pub base_hpa: u64,
    pub window_size: u64,
    pub interleave_ways: u8,
    pub interleave_arithmetic: u8,
    pub reserved2: u16,
    pub granularity: u32,
    pub restrictions: u16,
    pub qtg_id: u16,
    pub interleave_targets: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_cfmws_target_element {
    pub interleave_target: u32,
}

// Values for Interleave Arithmetic field above

// Values for Restrictions field above

// 2: CXL XOR Interleave Math Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_cxims {
    pub header: acpi_cedt_header,
    pub reserved1: u16,
    pub hbig: u8,
    pub nr_xormaps: u8,
    pub xormap_list: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_cxims_target_element {
    pub xormap: u64,
}

// 3: CXL RCEC Downstream Port Association Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cedt_rdpas {
    pub header: acpi_cedt_header,
    pub segment: u16,
    pub bdf: u16,
    pub protocol: u8,
    pub address: u64,
}

// Masks for bdf field above
pub const ACPI_CEDT_RDPAS_BUS_MASK: c_uint = 0xff00;
pub const ACPI_CEDT_RDPAS_DEVICE_MASK: c_uint = 0x00f8;
pub const ACPI_CEDT_RDPAS_FUNCTION_MASK: c_uint = 0x0007;

//
// CPEP - Corrected Platform Error Polling table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_cpep {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u64,
}

// Subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_cpep_polling {
    pub header: acpi_subtable_header,
    pub /: *mut *mut u8 id; / Processor ID,
    pub /: *mut *mut u8 eid; / Processor EID,
    pub /: *mut *mut u32 interval; / Polling interval (msec),
}

//
// CSRT - Core System Resource Table
// Version 0
//
// Conforms to the "Core System Resource Table (CSRT)", November 14, 2011
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_csrt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// Resource Group subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_csrt_group {
    pub length: u32,
    pub vendor_id: u32,
    pub subvendor_id: u32,
    pub device_id: u16,
    pub subdevice_id: u16,
    pub revision: u16,
    pub reserved: u16,
    pub shared_info_length: u32,
// Shared data immediately follows (Length = shared_info_length)
}

// Shared Info subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_csrt_shared_info {
    pub major_version: u16,
    pub minor_version: u16,
    pub mmio_base_low: u32,
    pub mmio_base_high: u32,
    pub gsi_interrupt: u32,
    pub interrupt_polarity: u8,
    pub interrupt_mode: u8,
    pub num_channels: u8,
    pub dma_address_width: u8,
    pub base_request_line: u16,
    pub num_handshake_signals: u16,
    pub max_block_size: u32,
// Resource descriptors immediately follow (Length = Group length - shared_info_length)
}

// Resource Descriptor subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_csrt_descriptor {
    pub length: u32,
    pub type: u16,
    pub subtype: u16,
    pub uid: u32,
// Resource-specific information immediately follows
}

// Resource Types
pub const ACPI_CSRT_TYPE_INTERRUPT: c_uint = 0x0001;
pub const ACPI_CSRT_TYPE_TIMER: c_uint = 0x0002;
pub const ACPI_CSRT_TYPE_DMA: c_uint = 0x0003;
// Resource Subtypes
pub const ACPI_CSRT_XRUPT_LINE: c_uint = 0x0000;
pub const ACPI_CSRT_XRUPT_CONTROLLER: c_uint = 0x0001;
pub const ACPI_CSRT_TIMER: c_uint = 0x0000;
pub const ACPI_CSRT_DMA_CHANNEL: c_uint = 0x0000;
pub const ACPI_CSRT_DMA_CONTROLLER: c_uint = 0x0001;
//
// DBG2 - Debug Port Table 2
// Version 0 (Both main table and subtables)
//
// Conforms to "Microsoft Debug Port Table 2 (DBG2)", September 21, 2020
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_dbg2 {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub info_offset: u32,
    pub info_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dbg2_header {
    pub info_offset: u32,
    pub info_count: u32,
}

// Debug Device Information Subtable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dbg2_device {
    pub revision: u8,
    pub length: u16,
    pub /: *mut *mut u8 register_count; / Number of base_address registers,
    pub namepath_length: u16,
    pub namepath_offset: u16,
    pub oem_data_length: u16,
    pub oem_data_offset: u16,
    pub port_type: u16,
    pub port_subtype: u16,
    pub reserved: u16,
    pub base_address_offset: u16,
    pub address_size_offset: u16,
//
// Data that follows:
// base_address (required) - Each in 12-byte Generic Address Structure format.
// address_size (required) - Array of u32 sizes corresponding to each base_address register.
// Namepath    (required) - Null terminated string. Single dot if not supported.
// oem_data    (optional) - Length is oem_data_length.
//
}

// Types for port_type field above
pub const ACPI_DBG2_SERIAL_PORT: c_uint = 0x8000;
pub const ACPI_DBG2_1394_PORT: c_uint = 0x8001;
pub const ACPI_DBG2_USB_PORT: c_uint = 0x8002;
pub const ACPI_DBG2_NET_PORT: c_uint = 0x8003;
// Subtypes for port_subtype field above
pub const ACPI_DBG2_16550_COMPATIBLE: c_uint = 0x0000;
pub const ACPI_DBG2_16550_SUBSET: c_uint = 0x0001;
pub const ACPI_DBG2_MAX311XE_SPI: c_uint = 0x0002;
pub const ACPI_DBG2_ARM_PL011: c_uint = 0x0003;
pub const ACPI_DBG2_MSM8X60: c_uint = 0x0004;
pub const ACPI_DBG2_16550_NVIDIA: c_uint = 0x0005;
pub const ACPI_DBG2_TI_OMAP: c_uint = 0x0006;
pub const ACPI_DBG2_APM88XXXX: c_uint = 0x0008;
pub const ACPI_DBG2_MSM8974: c_uint = 0x0009;
pub const ACPI_DBG2_SAM5250: c_uint = 0x000A;
pub const ACPI_DBG2_INTEL_USIF: c_uint = 0x000B;
pub const ACPI_DBG2_IMX6: c_uint = 0x000C;
pub const ACPI_DBG2_ARM_SBSA_32BIT: c_uint = 0x000D;
pub const ACPI_DBG2_ARM_SBSA_GENERIC: c_uint = 0x000E;
pub const ACPI_DBG2_ARM_DCC: c_uint = 0x000F;
pub const ACPI_DBG2_BCM2835: c_uint = 0x0010;
pub const ACPI_DBG2_SDM845_1_8432MHZ: c_uint = 0x0011;
pub const ACPI_DBG2_16550_WITH_GAS: c_uint = 0x0012;
pub const ACPI_DBG2_SDM845_7_372MHZ: c_uint = 0x0013;
pub const ACPI_DBG2_INTEL_LPSS: c_uint = 0x0014;
pub const ACPI_DBG2_RISCV_SBI_CON: c_uint = 0x0015;
pub const ACPI_DBG2_1394_STANDARD: c_uint = 0x0000;
pub const ACPI_DBG2_USB_XHCI: c_uint = 0x0000;
pub const ACPI_DBG2_USB_EHCI: c_uint = 0x0001;
//
// DBGP - Debug Port table
// Version 1
//
// Conforms to the "Debug Port Specification", Version 1.00, 2/9/2000
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_dbgp {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 type; / 0=full 16550, 1=subset of 16550,
    pub reserved: [u8; 3],
    pub debug_port: acpi_generic_address,
}

//
// DMAR - DMA Remapping table
// Version 1
//
// Conforms to "Intel Virtualization Technology for Directed I/O",
// Version 2.3, October 2014
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_dmar {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u8 width; / Host Address Width,
    pub flags: u8,
    pub reserved: [u8; 10],
}

// Masks for Flags field above

// DMAR subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_header {
    pub type: u16,
    pub length: u16,
}

// Values for subtable type in struct acpi_dmar_header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_dmar_type {
    ACPI_DMAR_TYPE_HARDWARE_UNIT = 0,
    ACPI_DMAR_TYPE_RESERVED_MEMORY = 1,
    ACPI_DMAR_TYPE_ROOT_ATS = 2,
    ACPI_DMAR_TYPE_HARDWARE_AFFINITY = 3,
    ACPI_DMAR_TYPE_NAMESPACE = 4,
    ACPI_DMAR_TYPE_SATC = 5,
    ACPI_DMAR_TYPE_SIDP = 6,
    ACPI_DMAR_TYPE_RESERVED = 7	/* 7 and greater are reserved */
}

// DMAR Device Scope structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_device_scope {
    pub entry_type: u8,
    pub length: u8,
    pub flags: u8,
    pub reserved: u8,
    pub enumeration_id: u8,
    pub bus: u8,
}

// Values for entry_type in struct acpi_dmar_device_scope - device types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_dmar_scope_type {
    ACPI_DMAR_SCOPE_TYPE_NOT_USED = 0,
    ACPI_DMAR_SCOPE_TYPE_ENDPOINT = 1,
    ACPI_DMAR_SCOPE_TYPE_BRIDGE = 2,
    ACPI_DMAR_SCOPE_TYPE_IOAPIC = 3,
    ACPI_DMAR_SCOPE_TYPE_HPET = 4,
    ACPI_DMAR_SCOPE_TYPE_NAMESPACE = 5,
    ACPI_DMAR_SCOPE_TYPE_RESERVED = 6	/* 6 and greater are reserved */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_pci_path {
    pub device: u8,
    pub function: u8,
}

//
// DMAR Subtables, correspond to Type in struct acpi_dmar_header
//
// 0: Hardware Unit Definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_hardware_unit {
    pub header: acpi_dmar_header,
    pub flags: u8,
    pub /: *mut *mut u8 size; / Size of the register set,
    pub segment: u16,
    pub /: *mut *mut u64 address; / Register Base Address,
}

// Masks for Flags field above

// 1: Reserved Memory Definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_reserved_memory {
    pub header: acpi_dmar_header,
    pub reserved: u16,
    pub segment: u16,
    pub /: *mut *mut u64 base_address; / 4K aligned base address,
    pub /: *mut *mut u64 end_address; / 4K aligned limit address,
}

// Masks for Flags field above

// 2: Root Port ATS Capability Reporting Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_atsr {
    pub header: acpi_dmar_header,
    pub flags: u8,
    pub reserved: u8,
    pub segment: u16,
}

// Masks for Flags field above

// 3: Remapping Hardware Static Affinity Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_rhsa {
    pub header: acpi_dmar_header,
    pub reserved: u32,
    pub base_address: u64,
    pub proximity_domain: u32,
}

// 4: ACPI Namespace Device Declaration Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_andd {
    pub header: acpi_dmar_header,
    pub reserved: [u8; 3],
    pub device_number: u8,
    pub __pad: c_char,
    pub device_name): ACPI_FLEX_ARRAY(char,,
}

// 5: SOC Integrated Address Translation Cache Reporting Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_satc {
    pub header: acpi_dmar_header,
    pub flags: u8,
    pub reserved: u8,
    pub segment: u16,
}

// 6: so_c Integrated Device Property Reporting Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dmar_sidp {
    pub header: acpi_dmar_header,
    pub reserved: u16,
    pub segment: u16,
}

//
// DRTM - Dynamic Root of Trust for Measurement table
// Conforms to "TCG D-RTM Architecture" June 17 2013, Version 1.0.0
// Table version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_drtm {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub entry_base_address: u64,
    pub entry_length: u64,
    pub entry_address32: u32,
    pub entry_address64: u64,
    pub exit_address: u64,
    pub log_area_address: u64,
    pub log_area_length: u32,
    pub arch_dependent_address: u64,
    pub flags: u32,
}

// Flag Definitions for above

// 1) Validated Tables List (64-bit addresses)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_drtm_vtable_list {
    pub validated_table_count: u32,
    pub validated_tables: [u64; ],
}

// 2) Resources List (of Resource Descriptors)
// Resource Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_drtm_resource {
    pub size: [u8; 7],
    pub type: u8,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_drtm_resource_list {
    pub resource_count: u32,
    pub resources: [acpi_drtm_resource; ],
}

// 3) Platform-specific Identifiers List
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_drtm_dps_id {
    pub dps_id_length: u32,
    pub dps_id: [u8; 16],
}

//
// DTPR - DMA TXT Protection Ranges Table
// Version 1
//
// Conforms to "Intel® Trusted Execution Technology (Intel® TXT) DMA Protection
// Ranges",
// Revision 0.73, August 2021
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_dtpr {
    pub header: acpi_table_header,
    pub /: *mut *mut u32 flags; / 36,
    pub ins_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpr_array {
    pub base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpr_instance {
    pub flags: u32,
    pub tpr_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpr_aux_sr {
    pub srl_cnt: u32,
}

//
// TPRn_BASE (ACPI_TPRN_BASE_REG)
//
// Specifies the start address of TPRn region. TPR region address and size must
// be with 1MB resolution. These bits are compared with the result of the
// TPRn_LIMIT[63:20], which is applied to the incoming address, to
// determine if an access fall within the TPRn defined region.
//
// Minimal TPRn_Base resolution is 1MB. Applied to the incoming address, to
// determine if an access fall within the TPRn defined region. Width is
// determined by a bus width which can be obtained via CPUID
// function 0x80000008.
//
pub type ACPI_TPRN_BASE_REG = u64;
// TPRn_BASE Register Bit Masks
// Bit 3 - RW: access: 1 == RO, 0 == RW register (for TPR must be RW)
pub const ACPI_TPRN_BASE_RW_SHIFT: c_int = 3;

//
// Bit 4 - Enable: 0 – TPRn address range enabled;
// 1 – TPRn address range disabled.
//
pub const ACPI_TPRN_BASE_ENABLE_SHIFT: c_int = 4;

// Bits 63:20 - tpr_base_rw
pub const ACPI_TPRN_BASE_ADDR_SHIFT: c_int = 20;

// TPRn_BASE Register Bit Handlers
//
// GET_TPRN_BASE_RW:
//
// Read RW bit from TPRn Base register - bit 3.
//
// Input:
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
//
// Output:
//
// Returns RW bit value (u64).
//

//
// GET_TPRN_BASE_ENABLE:
//
// Read Enable bit from TPRn Base register - bit 4.
//
// Input:
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
//
// Output:
//
// Returns Enable bit value (u64).
//

//
// GET_TPRN_BASE_ADDR:
//
// Read TPRn Base Register address from bits 63:20.
//
// Input:
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
//
// Output:
//
// Returns TPRn Base Register address (u64).
//

//
// SET_TPRN_BASE_RW:
//
// Set RW bit in TPRn Base register - bit 3.
//
// Input:
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
// - val (represents RW value to be set (u64))
//

//
// SET_TPRN_BASE_ENABLE:
//
// Set Enable bit in TPRn Base register - bit 4.
//
// Input:
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
// - val (represents Enable value to be set (u64))
//

//
// SET_TPRN_BASE_ADDR:
//
// Set TPRn Base Register address - bits 63:20
//
// Input
// - reg (represents TPRn Base Register (ACPI_TPRN_BASE_REG))
// - val (represents address value to be set (u64))
//

//
// TPRn_LIMIT
//
// This register defines an isolated region of memory that can be enabled
// to prohibit certain system agents from accessing memory. When an agent
// sends a request upstream, whether snooped or not, a TPR prevents that
// transaction from changing the state of memory.
//
// Minimal TPRn_Limit resolution is 1MB. Width is determined by a bus width.
//
pub type ACPI_TPRN_LIMIT_REG = u64;
// TPRn_LIMIT Register Bit Masks
// Bit 3 - RW: access: 1 == RO, 0 == RW register (for TPR must be RW)
pub const ACPI_TPRN_LIMIT_RW_SHIFT: c_int = 3;

// Bits 63:20 - tpr_limit_rw
pub const ACPI_TPRN_LIMIT_ADDR_SHIFT: c_int = 20;

// TPRn_LIMIT Register Bit Handlers
//
// GET_TPRN_LIMIT_RW:
//
// Read RW bit from TPRn Limit register - bit 3.
//
// Input:
// - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
//
// Output:
//
// Returns RW bit value (u64).
//

//
// GET_TPRN_LIMIT_ADDR:
//
// Read TPRn Limit Register address from bits 63:20.
//
// Input:
// - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
//
// Output:
//
// Returns TPRn Limit Register address (u64).
//

//
// SET_TPRN_LIMIT_RW:
//
// Set RW bit in TPRn Limit register - bit 3.
//
// Input:
// - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
// - val (represents RW value to be set (u64))
//

//
// SET_TPRN_LIMIT_ADDR:
//
// Set TPRn Limit Register address - bits 63:20.
//
// Input:
// - reg (represents TPRn Limit Register (ACPI_TPRN_LIMIT_REG))
// - val (represents address value to be set (u64))
//

//
// SERIALIZE_REQUEST
//
// This register is used to request serialization of non-coherent DMA
// transactions. OS shall  issue it before changing of TPR settings
// (base / size).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tpr_serialize_request {
    pub sr_register: u64,
//
// BIT 1 - Status of serialization request (RO)
// 0 == register idle, 1 == serialization in progress
// BIT 2 - Control field to initiate serialization (RW)
// 0 == normal, 1 == initialize serialization
// (self-clear to allow multiple serialization requests)
//
}

//
// ECDT - Embedded Controller Boot Resources Table
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_ecdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut acpi_generic_address control; / Address of EC command/status register,
    pub /: *mut *mut acpi_generic_address data; / Address of EC data register,
    pub /: *mut *mut u32 uid; / Unique ID - must be same as the EC _UID method,
    pub /: *mut *mut u8 gpe; / The GPE for the EC,
    pub /: *mut *mut u8 id[]; / Full namepath of the EC in the ACPI namespace,
}

//
// EINJ - Error Injection Table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_einj {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub header_length: u32,
    pub flags: u8,
    pub reserved: [u8; 3],
    pub entries: u32,
}

// EINJ Injection Instruction Entries (actions)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_einj_entry {
    pub /: *mut *mut acpi_whea_header whea_header; / Common header for WHEA tables,
}

// Masks for Flags field above

// Values for Action field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_einj_actions {
    ACPI_EINJ_BEGIN_OPERATION = 0x0,
    ACPI_EINJ_GET_TRIGGER_TABLE = 0x1,
    ACPI_EINJ_SET_ERROR_TYPE = 0x2,
    ACPI_EINJ_GET_ERROR_TYPE = 0x3,
    ACPI_EINJ_END_OPERATION = 0x4,
    ACPI_EINJ_EXECUTE_OPERATION = 0x5,
    ACPI_EINJ_CHECK_BUSY_STATUS = 0x6,
    ACPI_EINJ_GET_COMMAND_STATUS = 0x7,
    ACPI_EINJ_SET_ERROR_TYPE_WITH_ADDRESS = 0x8,
    ACPI_EINJ_GET_EXECUTE_TIMINGS = 0x9,
    ACPI_EINJV2_GET_ERROR_TYPE = 0x11,
    ACPI_EINJ_ACTION_RESERVED = 0x12,	/* 0x12 and greater are reserved */
    ACPI_EINJ_TRIGGER_ERROR = 0xFF	/* Except for this value */
}

// Values for Instruction field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_einj_instructions {
    ACPI_EINJ_READ_REGISTER = 0,
    ACPI_EINJ_READ_REGISTER_VALUE = 1,
    ACPI_EINJ_WRITE_REGISTER = 2,
    ACPI_EINJ_WRITE_REGISTER_VALUE = 3,
    ACPI_EINJ_NOOP = 4,
    ACPI_EINJ_FLUSH_CACHELINE = 5,
    ACPI_EINJ_INSTRUCTION_RESERVED = 6	/* 6 and greater are reserved */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_einj_error_type_with_addr {
    pub error_type: u32,
    pub vendor_struct_offset: u32,
    pub flags: u32,
    pub apic_id: u32,
    pub address: u64,
    pub range: u64,
    pub pcie_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_einj_vendor {
    pub length: u32,
    pub pcie_id: u32,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision_id: u8,
    pub reserved: [u8; 3],
}

// EINJ Trigger Error Action Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_einj_trigger {
    pub header_size: u32,
    pub revision: u32,
    pub table_size: u32,
    pub entry_count: u32,
}

// Command status return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_einj_command_status {
    ACPI_EINJ_SUCCESS = 0,
    ACPI_EINJ_FAILURE = 1,
    ACPI_EINJ_INVALID_ACCESS = 2,
    ACPI_EINJ_STATUS_RESERVED = 3	/* 3 and greater are reserved */
}

// Error types returned from ACPI_EINJ_GET_ERROR_TYPE (bitfield)

// EINJV2 error types from EINJV2_GET_ERROR_TYPE (ACPI 6.6)

//
// ERST - Error Record Serialization Table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_erst {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub header_length: u32,
    pub reserved: u32,
    pub entries: u32,
}

// ERST Serialization Entries (actions)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erst_entry {
    pub /: *mut *mut acpi_whea_header whea_header; / Common header for WHEA tables,
}

// Masks for Flags field above

// Values for Action field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_erst_actions {
    ACPI_ERST_BEGIN_WRITE = 0,
    ACPI_ERST_BEGIN_READ = 1,
    ACPI_ERST_BEGIN_CLEAR = 2,
    ACPI_ERST_END = 3,
    ACPI_ERST_SET_RECORD_OFFSET = 4,
    ACPI_ERST_EXECUTE_OPERATION = 5,
    ACPI_ERST_CHECK_BUSY_STATUS = 6,
    ACPI_ERST_GET_COMMAND_STATUS = 7,
    ACPI_ERST_GET_RECORD_ID = 8,
    ACPI_ERST_SET_RECORD_ID = 9,
    ACPI_ERST_GET_RECORD_COUNT = 10,
    ACPI_ERST_BEGIN_DUMMY_WRIITE = 11,
    ACPI_ERST_NOT_USED = 12,
    ACPI_ERST_GET_ERROR_RANGE = 13,
    ACPI_ERST_GET_ERROR_LENGTH = 14,
    ACPI_ERST_GET_ERROR_ATTRIBUTES = 15,
    ACPI_ERST_EXECUTE_TIMINGS = 16,
    ACPI_ERST_ACTION_RESERVED = 17	/* 17 and greater are reserved */
}

// Values for Instruction field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_erst_instructions {
    ACPI_ERST_READ_REGISTER = 0,
    ACPI_ERST_READ_REGISTER_VALUE = 1,
    ACPI_ERST_WRITE_REGISTER = 2,
    ACPI_ERST_WRITE_REGISTER_VALUE = 3,
    ACPI_ERST_NOOP = 4,
    ACPI_ERST_LOAD_VAR1 = 5,
    ACPI_ERST_LOAD_VAR2 = 6,
    ACPI_ERST_STORE_VAR1 = 7,
    ACPI_ERST_ADD = 8,
    ACPI_ERST_SUBTRACT = 9,
    ACPI_ERST_ADD_VALUE = 10,
    ACPI_ERST_SUBTRACT_VALUE = 11,
    ACPI_ERST_STALL = 12,
    ACPI_ERST_STALL_WHILE_TRUE = 13,
    ACPI_ERST_SKIP_NEXT_IF_TRUE = 14,
    ACPI_ERST_GOTO = 15,
    ACPI_ERST_SET_SRC_ADDRESS_BASE = 16,
    ACPI_ERST_SET_DST_ADDRESS_BASE = 17,
    ACPI_ERST_MOVE_DATA = 18,
    ACPI_ERST_INSTRUCTION_RESERVED = 19	/* 19 and greater are reserved */
}

// Command status return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_erst_command_status {
    ACPI_ERST_SUCCESS = 0,
    ACPI_ERST_NO_SPACE = 1,
    ACPI_ERST_NOT_AVAILABLE = 2,
    ACPI_ERST_FAILURE = 3,
    ACPI_ERST_RECORD_EMPTY = 4,
    ACPI_ERST_NOT_FOUND = 5,
    ACPI_ERST_STATUS_RESERVED = 6	/* 6 and greater are reserved */
}

// Error Record Serialization Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_erst_info {
    pub /: *mut *mut u16 signature; / Should be "ER",
    pub data: [u8; 48],
}

//
// FPDT - Firmware Performance Data Table (ACPI 5.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_fpdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
}

// FPDT subtable header (Performance Record Structure)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fpdt_header {
    pub type: u16,
    pub length: u8,
    pub revision: u8,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_fpdt_type {
    ACPI_FPDT_TYPE_BOOT = 0,
    ACPI_FPDT_TYPE_S3PERF = 1
}

//
// FPDT subtables
//
// 0: Firmware Basic Boot Performance Record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fpdt_boot_pointer {
    pub header: acpi_fpdt_header,
    pub reserved: [u8; 4],
    pub address: u64,
}

// 1: S3 Performance Table Pointer Record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fpdt_s3pt_pointer {
    pub header: acpi_fpdt_header,
    pub reserved: [u8; 4],
    pub address: u64,
}

//
// S3PT - S3 Performance Table. This table is pointed to by the
// S3 Pointer Record above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_s3pt {
    pub /: *mut *mut u8 signature[4]; / "S3PT",
    pub length: u32,
}

//
// S3PT Subtables (Not part of the actual FPDT)
//
// Values for Type field in S3PT header
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_s3pt_type {
    ACPI_S3PT_TYPE_RESUME = 0,
    ACPI_S3PT_TYPE_SUSPEND = 1,
    ACPI_FPDT_BOOT_PERFORMANCE = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_s3pt_resume {
    pub header: acpi_fpdt_header,
    pub resume_count: u32,
    pub full_resume: u64,
    pub average_resume: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_s3pt_suspend {
    pub header: acpi_fpdt_header,
    pub suspend_start: u64,
    pub suspend_end: u64,
}

//
// FPDT Boot Performance Record (Not part of the actual FPDT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fpdt_boot {
    pub header: acpi_fpdt_header,
    pub reserved: [u8; 4],
    pub reset_end: u64,
    pub load_start: u64,
    pub startup_start: u64,
    pub exit_services_entry: u64,
    pub exit_services_exit: u64,
}

//
// GTDT - Generic Timer Description Table (ACPI 5.1)
// Version 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_gtdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub counter_block_addresss: u64,
    pub reserved: u32,
    pub secure_el1_interrupt: u32,
    pub secure_el1_flags: u32,
    pub non_secure_el1_interrupt: u32,
    pub non_secure_el1_flags: u32,
    pub virtual_timer_interrupt: u32,
    pub virtual_timer_flags: u32,
    pub non_secure_el2_interrupt: u32,
    pub non_secure_el2_flags: u32,
    pub counter_read_block_address: u64,
    pub platform_timer_count: u32,
    pub platform_timer_offset: u32,
}

// Flag Definitions: Timer Block Physical Timers and Virtual timers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtdt_el2 {
    pub virtual_el2_timer_gsiv: u32,
    pub virtual_el2_timer_flags: u32,
}

// Common GTDT subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtdt_header {
    pub type: u8,
    pub length: u16,
}

// Values for GTDT subtable type above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_gtdt_type {
    ACPI_GTDT_TYPE_TIMER_BLOCK = 0,
    ACPI_GTDT_TYPE_WATCHDOG = 1,
    ACPI_GTDT_TYPE_RESERVED = 2	/* 2 and greater are reserved */
}

// GTDT Subtables, correspond to Type in struct acpi_gtdt_header
// 0: Generic Timer Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtdt_timer_block {
    pub header: acpi_gtdt_header,
    pub reserved: u8,
    pub block_address: u64,
    pub timer_count: u32,
    pub timer_offset: u32,
}

// Timer Sub-Structure, one per timer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtdt_timer_entry {
    pub frame_number: u8,
    pub reserved: [u8; 3],
    pub base_address: u64,
    pub el0_base_address: u64,
    pub timer_interrupt: u32,
    pub timer_flags: u32,
    pub virtual_timer_interrupt: u32,
    pub virtual_timer_flags: u32,
    pub common_flags: u32,
}

// Flag Definitions: timer_flags and virtual_timer_flags above

// Flag Definitions: common_flags above

// 1: SBSA Generic Watchdog Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtdt_watchdog {
    pub header: acpi_gtdt_header,
    pub reserved: u8,
    pub refresh_frame_address: u64,
    pub control_frame_address: u64,
    pub timer_interrupt: u32,
    pub timer_flags: u32,
}

// Flag Definitions: timer_flags above

//
// HEST - Hardware Error Source Table (ACPI 4.0)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_hest {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub error_source_count: u32,
}

// HEST subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_header {
    pub type: u16,
    pub source_id: u16,
}

// Values for Type field above for subtables
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_hest_types {
    ACPI_HEST_TYPE_IA32_CHECK = 0,
    ACPI_HEST_TYPE_IA32_CORRECTED_CHECK = 1,
    ACPI_HEST_TYPE_IA32_NMI = 2,
    ACPI_HEST_TYPE_NOT_USED3 = 3,
    ACPI_HEST_TYPE_NOT_USED4 = 4,
    ACPI_HEST_TYPE_NOT_USED5 = 5,
    ACPI_HEST_TYPE_AER_ROOT_PORT = 6,
    ACPI_HEST_TYPE_AER_ENDPOINT = 7,
    ACPI_HEST_TYPE_AER_BRIDGE = 8,
    ACPI_HEST_TYPE_GENERIC_ERROR = 9,
    ACPI_HEST_TYPE_GENERIC_ERROR_V2 = 10,
    ACPI_HEST_TYPE_IA32_DEFERRED_CHECK = 11,
    ACPI_HEST_TYPE_RESERVED = 12	/* 12 and greater are reserved */
}

//
// HEST substructures contained in subtables
//
// IA32 Error Bank(s) - Follows the struct acpi_hest_ia_machine_check and
// struct acpi_hest_ia_corrected structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_ia_error_bank {
    pub bank_number: u8,
    pub clear_status_on_init: u8,
    pub status_format: u8,
    pub reserved: u8,
    pub control_register: u32,
    pub control_data: u64,
    pub status_register: u32,
    pub address_register: u32,
    pub misc_register: u32,
}

// Common HEST sub-structure for PCI/AER structures below (6,7,8)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_aer_common {
    pub reserved1: u16,
    pub flags: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub /: *mut *mut u32 bus; / Bus and Segment numbers,
    pub device: u16,
    pub function: u16,
    pub device_control: u16,
    pub reserved2: u16,
    pub uncorrectable_mask: u32,
    pub uncorrectable_severity: u32,
    pub correctable_mask: u32,
    pub advanced_capabilities: u32,
}

// Masks for HEST Flags fields

//
// Macros to access the bus/segment numbers in Bus field above:
// Bus number is encoded in bits 7:0
// Segment number is encoded in bits 23:8
//

// Hardware Error Notification
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_notify {
    pub type: u8,
    pub length: u8,
    pub config_write_enable: u16,
    pub poll_interval: u32,
    pub vector: u32,
    pub polling_threshold_value: u32,
    pub polling_threshold_window: u32,
    pub error_threshold_value: u32,
    pub error_threshold_window: u32,
}

// Values for Notify Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_hest_notify_types {
    ACPI_HEST_NOTIFY_POLLED = 0,
    ACPI_HEST_NOTIFY_EXTERNAL = 1,
    ACPI_HEST_NOTIFY_LOCAL = 2,
    ACPI_HEST_NOTIFY_SCI = 3,
    ACPI_HEST_NOTIFY_NMI = 4,
    ACPI_HEST_NOTIFY_CMCI = 5,	/* ACPI 5.0 */
    ACPI_HEST_NOTIFY_MCE = 6,	/* ACPI 5.0 */
    ACPI_HEST_NOTIFY_GPIO = 7,	/* ACPI 6.0 */
    ACPI_HEST_NOTIFY_SEA = 8,	/* ACPI 6.1 */
    ACPI_HEST_NOTIFY_SEI = 9,	/* ACPI 6.1 */
    ACPI_HEST_NOTIFY_GSIV = 10,	/* ACPI 6.1 */
    ACPI_HEST_NOTIFY_SOFTWARE_DELEGATED = 11,	/* ACPI 6.2 */
    ACPI_HEST_NOTIFY_RESERVED = 12	/* 12 and greater are reserved */
}

// Values for config_write_enable bitfield above

//
// HEST subtables
//
// 0: IA32 Machine Check Exception
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_ia_machine_check {
    pub header: acpi_hest_header,
    pub reserved1: u16,
    pub /: *mut *mut u8 flags; / See flags ACPI_HEST_GLOBAL, etc. above,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub global_capability_data: u64,
    pub global_control_data: u64,
    pub num_hardware_banks: u8,
    pub reserved3: [u8; 7],
}

// 1: IA32 Corrected Machine Check
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_ia_corrected {
    pub header: acpi_hest_header,
    pub reserved1: u16,
    pub /: *mut *mut u8 flags; / See flags ACPI_HEST_GLOBAL, etc. above,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub notify: acpi_hest_notify,
    pub num_hardware_banks: u8,
    pub reserved2: [u8; 3],
}

// 2: IA32 Non-Maskable Interrupt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_ia_nmi {
    pub header: acpi_hest_header,
    pub reserved: u32,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
}

// 3,4,5: Not used
// 6: PCI Express Root Port AER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_aer_root {
    pub header: acpi_hest_header,
    pub aer: acpi_hest_aer_common,
    pub root_error_command: u32,
}

// 7: PCI Express AER (AER Endpoint)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_aer {
    pub header: acpi_hest_header,
    pub aer: acpi_hest_aer_common,
}

// 8: PCI Express/PCI-X Bridge AER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_aer_bridge {
    pub header: acpi_hest_header,
    pub aer: acpi_hest_aer_common,
    pub uncorrectable_mask2: u32,
    pub uncorrectable_severity2: u32,
    pub advanced_capabilities2: u32,
}

// 9: Generic Hardware Error Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_generic {
    pub header: acpi_hest_header,
    pub related_source_id: u16,
    pub reserved: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
    pub error_status_address: acpi_generic_address,
    pub notify: acpi_hest_notify,
    pub error_block_length: u32,
}

// 10: Generic Hardware Error Source, version 2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_generic_v2 {
    pub header: acpi_hest_header,
    pub related_source_id: u16,
    pub reserved: u8,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub max_raw_data_length: u32,
    pub error_status_address: acpi_generic_address,
    pub notify: acpi_hest_notify,
    pub error_block_length: u32,
    pub read_ack_register: acpi_generic_address,
    pub read_ack_preserve: u64,
    pub read_ack_write: u64,
}

// Generic Error Status block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_generic_status {
    pub block_status: u32,
    pub raw_data_offset: u32,
    pub raw_data_length: u32,
    pub data_length: u32,
    pub error_severity: u32,
}

// Values for block_status flags above

// Generic Error Data entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_generic_data {
    pub section_type: [u8; 16],
    pub error_severity: u32,
    pub revision: u16,
    pub validation_bits: u8,
    pub flags: u8,
    pub error_data_length: u32,
    pub fru_id: [u8; 16],
    pub fru_text: [u8; 20],
}

// Extension for revision 0x0300
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_generic_data_v300 {
    pub section_type: [u8; 16],
    pub error_severity: u32,
    pub revision: u16,
    pub validation_bits: u8,
    pub flags: u8,
    pub error_data_length: u32,
    pub fru_id: [u8; 16],
    pub fru_text: [u8; 20],
    pub time_stamp: u64,
}

// Values for error_severity above
pub const ACPI_HEST_GEN_ERROR_RECOVERABLE: c_int = 0;
pub const ACPI_HEST_GEN_ERROR_FATAL: c_int = 1;
pub const ACPI_HEST_GEN_ERROR_CORRECTED: c_int = 2;
pub const ACPI_HEST_GEN_ERROR_NONE: c_int = 3;
// Flags for validation_bits above

// 11: IA32 Deferred Machine Check Exception (ACPI 6.2)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hest_ia_deferred_check {
    pub header: acpi_hest_header,
    pub reserved1: u16,
    pub /: *mut *mut u8 flags; / See flags ACPI_HEST_GLOBAL, etc. above,
    pub enabled: u8,
    pub records_to_preallocate: u32,
    pub max_sections_per_record: u32,
    pub notify: acpi_hest_notify,
    pub num_hardware_banks: u8,
    pub reserved2: [u8; 3],
}

//
// HMAT - Heterogeneous Memory Attributes Table (ACPI 6.2)
// Version 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_hmat {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: u32,
}

// Values for HMAT structure types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_hmat_type {
    ACPI_HMAT_TYPE_PROXIMITY = 0,	/* Memory proximity domain attributes */
    ACPI_HMAT_TYPE_LOCALITY = 1,	/* System locality latency and bandwidth information */
    ACPI_HMAT_TYPE_CACHE = 2,	/* Memory side cache information */
    ACPI_HMAT_TYPE_RESERVED = 3	/* 3 and greater are reserved */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hmat_structure {
    pub type: u16,
    pub reserved: u16,
    pub length: u32,
}

//
// HMAT Structures, correspond to Type in struct acpi_hmat_structure
//
// 0: Memory proximity domain attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hmat_proximity_domain {
    pub header: acpi_hmat_structure,
    pub flags: u16,
    pub reserved1: u16,
    pub /: *mut *mut u32 processor_PD; / Processor proximity domain,
    pub /: *mut *mut u32 memory_PD; / Memory proximity domain,
    pub reserved2: u32,
    pub reserved3: u64,
    pub reserved4: u64,
}

// Masks for Flags field above

// 1: System locality latency and bandwidth information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hmat_locality {
    pub header: acpi_hmat_structure,
    pub flags: u8,
    pub data_type: u8,
    pub min_transfer_size: u8,
    pub reserved1: u8,
    pub number_of_initiator_Pds: u32,
    pub number_of_target_Pds: u32,
    pub reserved2: u32,
    pub entry_base_unit: u64,
}

// Masks for Flags field above

// Values for Memory Hierarchy flags
pub const ACPI_HMAT_MEMORY: c_int = 0;
pub const ACPI_HMAT_LAST_LEVEL_CACHE: c_int = 1;
pub const ACPI_HMAT_1ST_LEVEL_CACHE: c_int = 2;
pub const ACPI_HMAT_2ND_LEVEL_CACHE: c_int = 3;
pub const ACPI_HMAT_3RD_LEVEL_CACHE: c_int = 4;
pub const ACPI_HMAT_MINIMUM_XFER_SIZE: c_uint = 0x10       /* Bit 4: ACPI 6.4 */;
pub const ACPI_HMAT_NON_SEQUENTIAL_XFERS: c_uint = 0x20    /* Bit 5: ACPI 6.4 */;
// Values for data_type field above
pub const ACPI_HMAT_ACCESS_LATENCY: c_int = 0;
pub const ACPI_HMAT_READ_LATENCY: c_int = 1;
pub const ACPI_HMAT_WRITE_LATENCY: c_int = 2;
pub const ACPI_HMAT_ACCESS_BANDWIDTH: c_int = 3;
pub const ACPI_HMAT_READ_BANDWIDTH: c_int = 4;
pub const ACPI_HMAT_WRITE_BANDWIDTH: c_int = 5;
// 2: Memory side cache information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hmat_cache {
    pub header: acpi_hmat_structure,
    pub memory_PD: u32,
    pub reserved1: u32,
    pub cache_size: u64,
    pub cache_attributes: u32,
    pub address_mode: u16,
    pub number_of_SMBIOShandles: u16,
}

// Masks for cache_attributes field above

// Values for cache associativity flag

// Values for write policy flag

//
// HPET - High Precision Event Timer table
// Version 1
//
// Conforms to "IA-PC HPET (High Precision Event Timers) Specification",
// Version 1.0a, October 2004
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_hpet {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 id; / Hardware ID of event timer block,
    pub /: *mut *mut acpi_generic_address address; / Address of event timer block,
    pub /: *mut *mut u8 sequence; / HPET sequence number,
    pub /: *mut *mut u16 minimum_tick; / Main counter min tick, periodic mode,
    pub flags: u8,
}

// Masks for Flags field above

// Values for Page Protect flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_hpet_page_protect {
    ACPI_HPET_NO_PAGE_PROTECT = 0,
    ACPI_HPET_PAGE_PROTECT4 = 1,
    ACPI_HPET_PAGE_PROTECT64 = 2
}

//
// IBFT - Boot Firmware Table
// Version 1
//
// Conforms to "iSCSI Boot Firmware Table (iBFT) as Defined in ACPI 3.0b
// Specification", Version 1.01, March 1, 2007
//
// Note: It appears that this table is not intended to appear in the RSDT/XSDT.
// Therefore, it is not currently supported by the disassembler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_ibft {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub reserved: [u8; 12],
}

// IBFT common subtable header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ibft_header {
    pub type: u8,
    pub version: u8,
    pub length: u16,
    pub index: u8,
    pub flags: u8,
}

// Values for Type field above
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ibft_type {
    ACPI_IBFT_TYPE_NOT_USED = 0,
    ACPI_IBFT_TYPE_CONTROL = 1,
    ACPI_IBFT_TYPE_INITIATOR = 2,
    ACPI_IBFT_TYPE_NIC = 3,
    ACPI_IBFT_TYPE_TARGET = 4,
    ACPI_IBFT_TYPE_EXTENSIONS = 5,
    ACPI_IBFT_TYPE_RESERVED = 6	/* 6 and greater are reserved */
}

// IBFT subtables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ibft_control {
    pub header: acpi_ibft_header,
    pub extensions: u16,
    pub initiator_offset: u16,
    pub nic0_offset: u16,
    pub target0_offset: u16,
    pub nic1_offset: u16,
    pub target1_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ibft_initiator {
    pub header: acpi_ibft_header,
    pub sns_server: [u8; 16],
    pub slp_server: [u8; 16],
    pub primary_server: [u8; 16],
    pub secondary_server: [u8; 16],
    pub name_length: u16,
    pub name_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ibft_nic {
    pub header: acpi_ibft_header,
    pub ip_address: [u8; 16],
    pub subnet_mask_prefix: u8,
    pub origin: u8,
    pub gateway: [u8; 16],
    pub primary_dns: [u8; 16],
    pub secondary_dns: [u8; 16],
    pub dhcp: [u8; 16],
    pub vlan: u16,
    pub mac_address: [u8; 6],
    pub pci_address: u16,
    pub name_length: u16,
    pub name_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ibft_target {
    pub header: acpi_ibft_header,
    pub target_ip_address: [u8; 16],
    pub target_ip_socket: u16,
    pub target_boot_lun: [u8; 8],
    pub chap_type: u8,
    pub nic_association: u8,
    pub target_name_length: u16,
    pub target_name_offset: u16,
    pub chap_name_length: u16,
    pub chap_name_offset: u16,
    pub chap_secret_length: u16,
    pub chap_secret_offset: u16,
    pub reverse_chap_name_length: u16,
    pub reverse_chap_name_offset: u16,
    pub reverse_chap_secret_length: u16,
    pub reverse_chap_secret_offset: u16,
}

// Reset to default packing

