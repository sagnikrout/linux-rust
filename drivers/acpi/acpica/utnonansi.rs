//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utnonansi.c
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
// Module Name: utnonansi - Non-ansi C library functions
//

    ACPI_MODULE_NAME("utnonansi")
//
// Non-ANSI C library functions - strlwr, strupr, stricmp, and "safe"
// string functions.
//
// FUNCTION:    acpi_ut_strlwr (strlwr)
//
// PARAMETERS:  src_string      - The source string to convert
//
// RETURN:      None
//
// DESCRIPTION: Convert a string to lowercase
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_strlwr(src_string: *mut c_char) {
    void acpi_ut_strlwr(char *src_string)
    {
    char *string;
    ACPI_FUNCTION_ENTRY();
    if (!src_string) {
    return;
    }
// Walk entire string, lowercasing the letters
    for (string = src_string; *string; string++) {
// string = (char)tolower((int)*string);
    }
    }
//
// FUNCTION:    acpi_ut_strupr (strupr)
//
// PARAMETERS:  src_string      - The source string to convert
//
// RETURN:      None
//
// DESCRIPTION: Convert a string to uppercase
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_strupr(src_string: *mut c_char) {
    void acpi_ut_strupr(char *src_string)
    {
    char *string;
    ACPI_FUNCTION_ENTRY();
    if (!src_string) {
    return;
    }
// Walk entire string, uppercasing the letters
    for (string = src_string; *string; string++) {
// string = (char)toupper((int)*string);
    }
    }
//
// FUNCTION:    acpi_ut_stricmp (stricmp)
//
// PARAMETERS:  string1             - first string to compare
// string2             - second string to compare
//
// RETURN:      int that signifies string relationship. Zero means strings
// are equal.
//
// DESCRIPTION: Case-insensitive string compare. Implementation of the
// non-ANSI stricmp function.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_stricmp(string1: *mut c_char, string2: *mut c_char) -> c_int {
    int acpi_ut_stricmp(char *string1, char *string2)
    {
    int c1;
    int c2;
    do {
    c1 = tolower((int)*string1);
    c2 = tolower((int)*string2);
    string1++;
    string2++;
    }
    while ((c1 == c2) && (c1));
    return (c1 - c2);
    }

//
// FUNCTION:    acpi_ut_safe_strcpy, acpi_ut_safe_strcat, acpi_ut_safe_strncat
//
// PARAMETERS:  Adds a "DestSize" parameter to each of the standard string
// functions. This is the size of the Destination buffer.
//
// RETURN:      TRUE if the operation would overflow the destination buffer.
//
// DESCRIPTION: Safe versions of standard Clib string functions. Ensure that
// the result of the operation will not overflow the output string
// buffer.
//
// NOTE:        These functions are typically only helpful for processing
// user input and command lines. For most ACPICA code, the
// required buffer length is precisely calculated before buffer
// allocation, so the use of these functions is unnecessary.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_safe_strcpy(dest: *mut c_char, dest_size: acpi_size, source: *mut c_char) -> u8 {
    u8 acpi_ut_safe_strcpy(char *dest, acpi_size dest_size, char *source)
    {
    if (strlen(source) >= dest_size) {
    return (TRUE);
    }
    strcpy(dest, source);
    return (FALSE);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_safe_strcat(dest: *mut c_char, dest_size: acpi_size, source: *mut c_char) -> u8 {
    u8 acpi_ut_safe_strcat(char *dest, acpi_size dest_size, char *source)
    {
    if ((strlen(dest) + strlen(source)) >= dest_size) {
    return (TRUE);
    }
    strcat(dest, source);
    return (FALSE);
    }
    u8
    acpi_ut_safe_strncat(char *dest,
    acpi_size dest_size,
    char *source, acpi_size max_transfer_length)
    {
    acpi_size actual_transfer_length;
    actual_transfer_length = ACPI_MIN(max_transfer_length, strlen(source));
    if ((strlen(dest) + actual_transfer_length) >= dest_size) {
    return (TRUE);
    }
    strncat(dest, source, max_transfer_length);
    return (FALSE);
    }
