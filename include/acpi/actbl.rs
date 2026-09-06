//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/actbl.h
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
// Name: actbl.h - Basic ACPI Table Definitions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Fundamental ACPI tables
//
// This file contains definitions for the ACPI tables that are directly consumed
// by ACPICA. All other tables are consumed by the OS-dependent ACPI-related
// device drivers and other OS support code.
//
// The RSDP and FACS do not use the common ACPI table header. All other ACPI
// tables use the header.
//
// Values for description table header signatures for tables defined in this
// file. Useful because they make it more difficult to inadvertently type in
// the wrong signature.
//

//
// All tables and structures must be byte-packed to match the ACPI
// specification, since the tables are provided by the system BIOS
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
// Master ACPI Table Header. This common header is used by all ACPI tables
// except the RSDP and FACS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_header {
    pub /: *mut *mut char signature[ACPI_NAMESEG_SIZE] ACPI_NONSTRING; / ASCII table signature,
    pub /: *mut *mut u32 length; / Length of table in bytes, including this header,
    pub /: *mut *mut u8 revision; / ACPI Specification minor version number,
    pub /: *mut *mut u8 checksum; / To make sum of entire table == 0,
    pub /: *mut *mut char oem_id[ACPI_OEM_ID_SIZE] ACPI_NONSTRING; / ASCII OEM identification,
    pub /: *mut *mut char oem_table_id[ACPI_OEM_TABLE_ID_SIZE] ACPI_NONSTRING; / ASCII OEM table identification,
    pub /: *mut *mut u32 oem_revision; / OEM revision number,
    pub /: *mut *mut char asl_compiler_id[ACPI_NAMESEG_SIZE] ACPI_NONSTRING; / ASCII ASL compiler vendor ID,
    pub /: *mut *mut u32 asl_compiler_revision; / ASL compiler version,
}

//
// GAS - Generic Address Structure (ACPI 2.0+)
//
// Note: Since this structure is used in the ACPI tables, it is byte aligned.
// If misaligned access is not supported by the hardware, accesses to the
// 64-bit Address field must be performed with care.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_generic_address {
    pub /: *mut *mut u8 space_id; / Address space where struct or register exists,
    pub /: *mut *mut u8 bit_width; / Size in bits of given register,
    pub /: *mut *mut u8 bit_offset; / Bit offset within the register,
    pub /: *mut *mut u8 access_width; / Minimum Access size (ACPI 3.0),
    pub /: *mut *mut u64 address; / 64-bit address of struct or register,
}

//
// RSDP - Root System Description Pointer (Signature is "RSD PTR ")
// Version 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rsdp {
    pub /: *mut *mut char signature[8]; / ACPI signature, contains "RSD PTR ",
    pub /: *mut *mut u8 checksum; / ACPI 1.0 checksum,
    pub /: *mut *mut char oem_id[ACPI_OEM_ID_SIZE]; / OEM identification,
    pub /: *mut *mut u8 revision; / Must be (0) for ACPI 1.0 or (2) for ACPI 2.0+,
    pub /: *mut *mut u32 rsdt_physical_address; / 32-bit physical address of the RSDT,
    pub /: *mut *mut u32 length; / Table length in bytes, including header (ACPI 2.0+),
    pub /: *mut *mut u64 xsdt_physical_address; / 64-bit physical address of the XSDT (ACPI 2.0+),
    pub /: *mut *mut u8 extended_checksum; / Checksum of entire table (ACPI 2.0+),
    pub /: *mut *mut u8 reserved[3]; / Reserved, must be zero,
}

// Standalone struct for the ACPI 1.0 RSDP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rsdp_common {
    pub signature: [c_char; 8],
    pub checksum: u8,
    pub oem_id: [c_char; ACPI_OEM_ID_SIZE],
    pub revision: u8,
    pub rsdt_physical_address: u32,
}

// Standalone struct for the extended part of the RSDP (ACPI 2.0+)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rsdp_extension {
    pub length: u32,
    pub xsdt_physical_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

//
// RSDT/XSDT - Root System Description Tables
// Version 1 (both)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_rsdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 table_offset_entry[1]; / Array of pointers to ACPI tables,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_xsdt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u64 table_offset_entry[1]; / Array of pointers to ACPI tables,
}

//
// FACS - Firmware ACPI Control Structure (FACS)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_facs {
    pub /: *mut *mut char signature[4]; / ASCII table signature,
    pub /: *mut *mut u32 length; / Length of structure, in bytes,
    pub /: *mut *mut u32 hardware_signature; / Hardware configuration signature,
    pub /: *mut *mut u32 firmware_waking_vector; / 32-bit physical address of the Firmware Waking Vector,
    pub /: *mut *mut u32 global_lock; / Global Lock for shared hardware resources,
    pub flags: u32,
    pub /: *mut *mut u64 xfirmware_waking_vector; / 64-bit version of the Firmware Waking Vector (ACPI 2.0+),
    pub /: *mut *mut u8 version; / Version of this table (ACPI 2.0+),
    pub /: *mut *mut u8 reserved[3]; / Reserved, must be zero,
    pub /: *mut *mut u32 ospm_flags; / Flags to be set by OSPM (ACPI 4.0),
    pub /: *mut *mut u8 reserved1[24]; / Reserved, must be zero,
}

// Masks for global_lock flag field above

// Masks for Flags field above

// Masks for ospm_flags field above

//
// FADT - Fixed ACPI Description Table (Signature "FACP")
// Version 6
//
// Fields common to all versions of the FADT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_fadt {
    pub /: *mut *mut acpi_table_header header; / Common ACPI table header,
    pub /: *mut *mut u32 facs; / 32-bit physical address of FACS,
    pub /: *mut *mut u32 dsdt; / 32-bit physical address of DSDT,
    pub /: *mut *mut u8 model; / System Interrupt Model (ACPI 1.0) - not used in ACPI 2.0+,
    pub /: *mut *mut u8 preferred_profile; / Conveys preferred power management profile to OSPM.,
    pub /: *mut *mut u16 sci_interrupt; / System vector of SCI interrupt,
    pub /: *mut *mut u32 smi_command; / 32-bit Port address of SMI command port,
    pub /: *mut *mut u8 acpi_enable; / Value to write to SMI_CMD to enable ACPI,
    pub /: *mut *mut u8 acpi_disable; / Value to write to SMI_CMD to disable ACPI,
    pub /: *mut *mut u8 s4_bios_request; / Value to write to SMI_CMD to enter S4BIOS state,
    pub /: *mut *mut u8 pstate_control; / Processor performance state control,
    pub /: *mut *mut u32 pm1a_event_block; / 32-bit port address of Power Mgt 1a Event Reg Blk,
    pub /: *mut *mut u32 pm1b_event_block; / 32-bit port address of Power Mgt 1b Event Reg Blk,
    pub /: *mut *mut u32 pm1a_control_block; / 32-bit port address of Power Mgt 1a Control Reg Blk,
    pub /: *mut *mut u32 pm1b_control_block; / 32-bit port address of Power Mgt 1b Control Reg Blk,
    pub /: *mut *mut u32 pm2_control_block; / 32-bit port address of Power Mgt 2 Control Reg Blk,
    pub /: *mut *mut u32 pm_timer_block; / 32-bit port address of Power Mgt Timer Ctrl Reg Blk,
    pub /: *mut *mut u32 gpe0_block; / 32-bit port address of General Purpose Event 0 Reg Blk,
    pub /: *mut *mut u32 gpe1_block; / 32-bit port address of General Purpose Event 1 Reg Blk,
    pub /: *mut *mut u8 pm1_event_length; / Byte Length of ports at pm1x_event_block,
    pub /: *mut *mut u8 pm1_control_length; / Byte Length of ports at pm1x_control_block,
    pub /: *mut *mut u8 pm2_control_length; / Byte Length of ports at pm2_control_block,
    pub /: *mut *mut u8 pm_timer_length; / Byte Length of ports at pm_timer_block,
    pub /: *mut *mut u8 gpe0_block_length; / Byte Length of ports at gpe0_block,
    pub /: *mut *mut u8 gpe1_block_length; / Byte Length of ports at gpe1_block,
    pub /: *mut *mut u8 gpe1_base; / Offset in GPE number space where GPE1 events start,
    pub /: *mut *mut u8 cst_control; / Support for the _CST object and C-States change notification,
    pub /: *mut *mut u16 c2_latency; / Worst case HW latency to enter/exit C2 state,
    pub /: *mut *mut u16 c3_latency; / Worst case HW latency to enter/exit C3 state,
    pub /: *mut *mut u16 flush_size; / Processor memory cache line width, in bytes,
    pub /: *mut *mut u16 flush_stride; / Number of flush strides that need to be read,
    pub /: *mut *mut u8 duty_offset; / Processor duty cycle index in processor P_CNT reg,
    pub /: *mut *mut u8 duty_width; / Processor duty cycle value bit width in P_CNT register,
    pub /: *mut *mut u8 day_alarm; / Index to day-of-month alarm in RTC CMOS RAM,
    pub /: *mut *mut u8 month_alarm; / Index to month-of-year alarm in RTC CMOS RAM,
    pub /: *mut *mut u8 century; / Index to century in RTC CMOS RAM,
    pub /: *mut *mut u16 boot_flags; / IA-PC Boot Architecture Flags (see below for individual flags),
    pub /: *mut *mut u8 reserved; / Reserved, must be zero,
    pub /: *mut *mut u32 flags; / Miscellaneous flag bits (see below for individual flags),
    pub /: *mut *mut acpi_generic_address reset_register; / 64-bit address of the Reset register,
    pub /: *mut *mut u8 reset_value; / Value to write to the reset_register port to reset the system,
    pub /: *mut *mut u16 arm_boot_flags; / ARM-Specific Boot Flags (see below for individual flags) (ACPI 5.1),
    pub /: *mut *mut u8 minor_revision; / FADT Minor Revision (ACPI 5.1),
    pub /: *mut *mut u64 Xfacs; / 64-bit physical address of FACS,
    pub /: *mut *mut u64 Xdsdt; / 64-bit physical address of DSDT,
    pub /: *mut *mut acpi_generic_address xpm1a_event_block; / 64-bit Extended Power Mgt 1a Event Reg Blk address,
    pub /: *mut *mut acpi_generic_address xpm1b_event_block; / 64-bit Extended Power Mgt 1b Event Reg Blk address,
    pub /: *mut *mut acpi_generic_address xpm1a_control_block; / 64-bit Extended Power Mgt 1a Control Reg Blk address,
    pub /: *mut *mut acpi_generic_address xpm1b_control_block; / 64-bit Extended Power Mgt 1b Control Reg Blk address,
    pub /: *mut *mut acpi_generic_address xpm2_control_block; / 64-bit Extended Power Mgt 2 Control Reg Blk address,
    pub /: *mut *mut acpi_generic_address xpm_timer_block; / 64-bit Extended Power Mgt Timer Ctrl Reg Blk address,
    pub /: *mut *mut acpi_generic_address xgpe0_block; / 64-bit Extended General Purpose Event 0 Reg Blk address,
    pub /: *mut *mut acpi_generic_address xgpe1_block; / 64-bit Extended General Purpose Event 1 Reg Blk address,
    pub /: *mut *mut acpi_generic_address sleep_control; / 64-bit Sleep Control register (ACPI 5.0),
    pub /: *mut *mut acpi_generic_address sleep_status; / 64-bit Sleep Status register (ACPI 5.0),
    pub /: *mut *mut u64 hypervisor_id; / Hypervisor Vendor ID (ACPI 6.0),
}

// Masks for FADT IA-PC Boot Architecture Flags (boot_flags) [Vx]=Introduced in this FADT revision

pub const FADT2_REVISION_ID: c_int = 3;
// Masks for FADT ARM Boot Architecture Flags (arm_boot_flags) ACPI 5.1

// Masks for FADT flags

// Values for preferred_profile (Preferred Power Management Profiles)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_preferred_pm_profiles {
    PM_UNSPECIFIED = 0,
    PM_DESKTOP = 1,
    PM_MOBILE = 2,
    PM_WORKSTATION = 3,
    PM_ENTERPRISE_SERVER = 4,
    PM_SOHO_SERVER = 5,
    PM_APPLIANCE_PC = 6,
    PM_PERFORMANCE_SERVER = 7,
    PM_TABLET = 8,
    NR_PM_PROFILES = 9
}

// Values for sleep_status and sleep_control registers (V5+ FADT)
pub const ACPI_X_WAKE_STATUS: c_uint = 0x80;
pub const ACPI_X_SLEEP_TYPE_MASK: c_uint = 0x1C;
pub const ACPI_X_SLEEP_TYPE_POSITION: c_uint = 0x02;
pub const ACPI_X_SLEEP_ENABLE: c_uint = 0x20;
// Reset to default packing

//
// Internal table-related structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_name_union {
    pub integer: u32,
    pub ascii: [c_char; 4],
}

// Internal ACPI Table Descriptor. One per ACPI table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_desc {
    pub address: acpi_physical_address,
    pub pointer: *mut acpi_table_header,
    pub /: *mut *mut u32 length; / Length fixed at 32 bits (fixed in table header),
    pub signature: acpi_name_union,
    pub owner_id: acpi_owner_id,
    pub flags: u8,
    pub validation_count: u16,
}

//
// Maximum value of the validation_count field in struct acpi_table_desc.
// When reached, validation_count cannot be changed any more and the table will
// be permanently regarded as validated.
//
// This is to prevent situations in which unbalanced table get/put operations
// may cause premature table unmapping in the OS to happen.
//
// The maximum validation count can be defined to any value, but should be
// greater than the maximum number of OS early stage mapping slots to avoid
// leaking early stage table mappings to the late stage.
//

// Masks for Flags field above

//
// Get the remaining ACPI tables
//

// Macros used to generate offsets to specific table fields

//
// Sizes of the various flavors of FADT. We need to look closely
// at the FADT length because the version number essentially tells
// us nothing because of many BIOS bugs where the version does not
// match the expected length. In other words, the length of the
// FADT is the bottom line as to what the version really is.
//
// For reference, the values below are as follows:
// FADT V1 size: 0x074
// FADT V2 size: 0x084
// FADT V3 size: 0x0F4
// FADT V4 size: 0x0F4
// FADT V5 size: 0x10C
// FADT V6 size: 0x114
//

