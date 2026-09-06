//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/tbprint.c
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
// Module Name: tbprint - Table output utilities
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("tbprint")
// Local prototypes
    static void acpi_tb_fix_string(char *string, acpi_size length);
    static void
    acpi_tb_cleanup_table_header(struct acpi_table_header *out_header,
    struct acpi_table_header *header);
//
// FUNCTION:    acpi_tb_fix_string
//
// PARAMETERS:  string              - String to be repaired
// length              - Maximum length
//
// RETURN:      None
//
// DESCRIPTION: Replace every non-printable or non-ascii byte in the string
// with a question mark '?'.
//
#[no_mangle]
unsafe extern "C" fn acpi_tb_fix_string(string: *mut c_char, length: acpi_size) {
    static void acpi_tb_fix_string(char *string, acpi_size length)
    {
    while (length && *string) {
    if (!isprint((int)(u8)*string)) {
// string = '?';
    }
    string++;
    length--;
    }
    }
//
// FUNCTION:    acpi_tb_cleanup_table_header
//
// PARAMETERS:  out_header          - Where the cleaned header is returned
// header              - Input ACPI table header
//
// RETURN:      Returns the cleaned header in out_header
//
// DESCRIPTION: Copy the table header and ensure that all "string" fields in
// the header consist of printable characters.
//
    static void
    acpi_tb_cleanup_table_header(struct acpi_table_header *out_header,
    struct acpi_table_header *header)
    {
    memcpy(out_header, header, sizeof(struct acpi_table_header));
    acpi_tb_fix_string(out_header.signature, ACPI_NAMESEG_SIZE);
    acpi_tb_fix_string(out_header.oem_id, ACPI_OEM_ID_SIZE);
    acpi_tb_fix_string(out_header.oem_table_id, ACPI_OEM_TABLE_ID_SIZE);
    acpi_tb_fix_string(out_header.asl_compiler_id, ACPI_NAMESEG_SIZE);
    }
//
// FUNCTION:    acpi_tb_print_table_header
//
// PARAMETERS:  address             - Table physical address
// header              - Table header
//
// RETURN:      None
//
// DESCRIPTION: Print an ACPI table header. Special cases for FACS and RSDP.
//
    void
    acpi_tb_print_table_header(acpi_physical_address address,
    struct acpi_table_header *header)
    {
    struct acpi_table_header local_header;

    if (ACPI_COMPARE_NAMESEG(header.signature, ACPI_SIG_FACS)) {
// FACS only has signature and length fields
    ACPI_INFO(("%-4.4s 0x%8.8X%8.8X %06X",
    header.signature, ACPI_FORMAT_UINT64(address),
    header.length));
    } else if (ACPI_VALIDATE_RSDP_SIG(ACPI_CAST_PTR(struct acpi_table_rsdp,
    header).signature)) {
// RSDP has no common fields
    memcpy(local_header.oem_id,
    ACPI_CAST_PTR(struct acpi_table_rsdp, header).oem_id,
    ACPI_OEM_ID_SIZE);
    acpi_tb_fix_string(local_header.oem_id, ACPI_OEM_ID_SIZE);
    ACPI_INFO(("RSDP 0x%8.8X%8.8X %06X (v%.2d %-6.6s)",
    ACPI_FORMAT_UINT64(address),
    (ACPI_CAST_PTR(struct acpi_table_rsdp, header).
    revision >
    0) ? ACPI_CAST_PTR(struct acpi_table_rsdp,
    header).length : 20,
    ACPI_CAST_PTR(struct acpi_table_rsdp,
    header).revision,
    local_header.oem_id));
    } else if (acpi_gbl_CDAT && !acpi_ut_valid_nameseg(header.signature)) {
// CDAT does not use the common ACPI table header
    ACPI_INFO(("%-4.4s 0x%8.8X%8.8X %06X",
    ACPI_SIG_CDAT, ACPI_FORMAT_UINT64(address),
    ACPI_CAST_PTR(struct acpi_table_cdat,
    header).length));
    } else {
// Standard ACPI table with full common header
    acpi_tb_cleanup_table_header(&local_header, header);
    ACPI_INFO(("%-4.4s 0x%8.8X%8.8X"
    " %06X (v%.2d %-6.6s %-8.8s %08X %-4.4s %08X)",
    local_header.signature, ACPI_FORMAT_UINT64(address),
    local_header.length, local_header.revision,
    local_header.oem_id, local_header.oem_table_id,
    local_header.oem_revision,
    local_header.asl_compiler_id,
    local_header.asl_compiler_revision));
    }

    }
