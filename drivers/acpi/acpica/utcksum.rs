//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utcksum.c
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
// Module Name: utcksum - Support generating table checksums
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// This module used for application-level code only

    ACPI_MODULE_NAME("utcksum")
//
// FUNCTION:    acpi_ut_verify_checksum
//
// PARAMETERS:  table               - ACPI table to verify
// length              - Length of entire table
//
// RETURN:      Status
//
// DESCRIPTION: Verifies that the table checksums to zero. Optionally returns
// exception on bad checksum.
// Note: We don't have to check for a CDAT here, since CDAT is
// not in the RSDT/XSDT, and the CDAT table is never installed
// via ACPICA.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_verify_checksum(table: *mut acpi_table_header, length: u32) -> acpi_status {
    acpi_status acpi_ut_verify_checksum(struct acpi_table_header *table, u32 length)
    {
    u8 checksum;
//
// FACS/S3PT:
// They are the odd tables, have no standard ACPI header and no checksum
//
    if (ACPI_COMPARE_NAMESEG(table.signature, ACPI_SIG_S3PT) ||
    ACPI_COMPARE_NAMESEG(table.signature, ACPI_SIG_FACS)) {
    return (AE_OK);
    }
// Compute the checksum on the table
    length = table.length;
    checksum =
    acpi_ut_generate_checksum(ACPI_CAST_PTR(u8, table), length,
    table.checksum);
// Computed checksum matches table?
    if (checksum != table.checksum) {
    ACPI_BIOS_WARNING((AE_INFO,
    "Incorrect checksum in table [%4.4s] - 0x%2.2X, "
    "should be 0x%2.2X",
    table.signature, table.checksum,
    table.checksum - checksum));

    return (AE_BAD_CHECKSUM);

    }
    return (AE_OK);
    }
//
// FUNCTION:    acpi_ut_verify_cdat_checksum
//
// PARAMETERS:  table               - CDAT ACPI table to verify
// length              - Length of entire table
//
// RETURN:      Status
//
// DESCRIPTION: Verifies that the CDAT table checksums to zero. Optionally
// returns an exception on bad checksum.
//
    acpi_status
    acpi_ut_verify_cdat_checksum(struct acpi_table_cdat *cdat_table, u32 length)
    {
    u8 checksum;
// Compute the checksum on the table
    checksum = acpi_ut_generate_checksum(ACPI_CAST_PTR(u8, cdat_table),
    cdat_table.length,
    cdat_table.checksum);
// Computed checksum matches table?
    if (checksum != cdat_table.checksum) {
    ACPI_BIOS_WARNING((AE_INFO,
    "Incorrect checksum in table [%4.4s] - 0x%2.2X, "
    "should be 0x%2.2X",
    acpi_gbl_CDAT, cdat_table.checksum,
    checksum));

    return (AE_BAD_CHECKSUM);

    }
    cdat_table.checksum = checksum;
    return (AE_OK);
    }
//
// FUNCTION:    acpi_ut_generate_checksum
//
// PARAMETERS:  table               - Pointer to table to be checksummed
// length              - Length of the table
// original_checksum   - Value of the checksum field
//
// RETURN:      8 bit checksum of buffer
//
// DESCRIPTION: Computes an 8 bit checksum of the table.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_generate_checksum(table: *mut c_void, length: u32, original_checksum: u8) -> u8 {
    u8 acpi_ut_generate_checksum(void *table, u32 length, u8 original_checksum)
    {
    u8 checksum;
// Sum the entire table as-is
    checksum = acpi_ut_checksum((u8 *)table, length);
// Subtract off the existing checksum value in the table
    checksum = (u8)(checksum - original_checksum);
// Compute and return the final checksum
    checksum = (u8)(0 - checksum);
    return (checksum);
    }
//
// FUNCTION:    acpi_ut_checksum
//
// PARAMETERS:  buffer          - Pointer to memory region to be checked
// length          - Length of this memory region
//
// RETURN:      Checksum (u8)
//
// DESCRIPTION: Calculates circular checksum of memory region.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_checksum(buffer: *mut u8, length: u32) -> u8 {
    u8 acpi_ut_checksum(u8 *buffer, u32 length)
    {
    let mut sum: u8 = 0;
    u8 *end = buffer + length;
    while (buffer < end) {
    sum = (u8)(sum + *(buffer++));
    }
    return (sum);
    }
