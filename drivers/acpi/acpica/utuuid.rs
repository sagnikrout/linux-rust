//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utuuid.c
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
// Module Name: utuuid -- UUID support functions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("utuuid")

//
// UUID support functions.
//
// This table is used to convert an input UUID ascii string to a 16 byte
// buffer and the reverse. The table maps a UUID buffer index 0-15 to
// the index within the 36-byte UUID string where the associated 2-byte
// hex value can be found.
//
// 36-byte UUID strings are of the form:
// aabbccdd-eeff-gghh-iijj-kkllmmnnoopp
// Where aa-pp are one byte hex numbers, made up of two hex digits
//
// Note: This table is basically the inverse of the string-to-offset table
// found in the ACPI spec in the description of the to_UUID macro.
//
    const u8 acpi_gbl_map_to_uuid_offset[UUID_BUFFER_LENGTH] = {
    6, 4, 2, 0, 11, 9, 16, 14, 19, 21, 24, 26, 28, 30, 32, 34
    };
//
// FUNCTION:    acpi_ut_convert_string_to_uuid
//
// PARAMETERS:  in_string           - 36-byte formatted UUID string
// uuid_buffer         - Where the 16-byte UUID buffer is returned
//
// RETURN:      None. Output data is returned in the uuid_buffer
//
// DESCRIPTION: Convert a 36-byte formatted UUID string to 16-byte UUID buffer
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_convert_string_to_uuid(in_string: *mut c_char, uuid_buffer: *mut u8) {
    void acpi_ut_convert_string_to_uuid(char *in_string, u8 *uuid_buffer)
    {
    u32 i;
    for (i = 0; i < UUID_BUFFER_LENGTH; i++) {
    uuid_buffer[i] =
    (acpi_ut_ascii_char_to_hex
    (in_string[acpi_gbl_map_to_uuid_offset[i]]) << 4);
    uuid_buffer[i] |=
    acpi_ut_ascii_char_to_hex(in_string
    [acpi_gbl_map_to_uuid_offset[i] +
    1]);
    }
    }
//
// FUNCTION:    acpi_ut_convert_uuid_to_string
//
// PARAMETERS:  uuid_buffer         - 16-byte UUID buffer
// out_string          - 36-byte formatted UUID string
//
// RETURN:      Status
//
// DESCRIPTION: Convert 16-byte UUID buffer to 36-byte formatted UUID string
// out_string must be 37 bytes to include null terminator.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_convert_uuid_to_string(uuid_buffer: *mut c_char, out_string: *mut c_char) -> acpi_status {
    acpi_status acpi_ut_convert_uuid_to_string(char *uuid_buffer, char *out_string)
    {
    u32 i;
    if (!uuid_buffer || !out_string) {
    return (AE_BAD_PARAMETER);
    }
    for (i = 0; i < UUID_BUFFER_LENGTH; i++) {
    out_string[acpi_gbl_map_to_uuid_offset[i]] =
    acpi_ut_hex_to_ascii_char(uuid_buffer[i], 4);
    out_string[acpi_gbl_map_to_uuid_offset[i] + 1] =
    acpi_ut_hex_to_ascii_char(uuid_buffer[i], 0);
    }
// Insert required hyphens (dashes)
    out_string[UUID_HYPHEN1_OFFSET] =
    out_string[UUID_HYPHEN2_OFFSET] =
    out_string[UUID_HYPHEN3_OFFSET] =
    out_string[UUID_HYPHEN4_OFFSET] = '-';
    out_string[UUID_STRING_LENGTH] = 0;	/* Null terminate */
    return (AE_OK);
    }
