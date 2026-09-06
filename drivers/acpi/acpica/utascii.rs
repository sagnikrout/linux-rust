//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utascii.c
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
// Module Name: utascii - Utility ascii functions
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

//
// FUNCTION:    acpi_ut_valid_nameseg
//
// PARAMETERS:  name            - The name or table signature to be examined.
// Four characters, does not have to be a
// NULL terminated string.
//
// RETURN:      TRUE if signature is has 4 valid ACPI characters
//
// DESCRIPTION: Validate an ACPI table signature.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_valid_nameseg(name: *mut c_char) -> u8 {
    u8 acpi_ut_valid_nameseg(char *name)
    {
    u32 i;
// Validate each character in the signature
    for (i = 0; i < ACPI_NAMESEG_SIZE; i++) {
    if (!acpi_ut_valid_name_char(name[i], i)) {
    return (FALSE);
    }
    }
    return (TRUE);
    }
//
// FUNCTION:    acpi_ut_valid_name_char
//
// PARAMETERS:  char            - The character to be examined
// position        - Byte position (0-3)
//
// RETURN:      TRUE if the character is valid, FALSE otherwise
//
// DESCRIPTION: Check for a valid ACPI character. Must be one of:
// 1) Upper case alpha
// 2) numeric
// 3) underscore
//
// We allow a '!' as the last character because of the ASF! table
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_valid_name_char(character: c_char, position: u32) -> u8 {
    u8 acpi_ut_valid_name_char(char character, u32 position)
    {
    if (!((character >= 'A' && character <= 'Z') ||
    (character >= '0' && character <= '9') || (character == '_'))) {
// Allow a '!' in the last position
    if (character == '!' && position == 3) {
    return (TRUE);
    }
    return (FALSE);
    }
    return (TRUE);
    }
//
// FUNCTION:    acpi_ut_check_and_repair_ascii
//
// PARAMETERS:  name                - Ascii string
// count               - Number of characters to check
//
// RETURN:      None
//
// DESCRIPTION: Ensure that the requested number of characters are printable
// Ascii characters. Sets non-printable and null chars to <space>.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_check_and_repair_ascii(name: *mut u8, repaired_name: *mut c_char, count: u32) {
    void acpi_ut_check_and_repair_ascii(u8 *name, char *repaired_name, u32 count)
    {
    u32 i;
    for (i = 0; i < count; i++) {
    repaired_name[i] = (char)name[i];
    if (!name[i]) {
    return;
    }
    if (!isprint(name[i])) {
    repaired_name[i] = ' ';
    }
    }
    }
