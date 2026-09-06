//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utstring.c
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
// Module Name: utstring - Common functions for strings and characters
//

    ACPI_MODULE_NAME("utstring")
//
// FUNCTION:    acpi_ut_print_string
//
// PARAMETERS:  string          - Null terminated ASCII string
// max_length      - Maximum output length. Used to constrain the
// length of strings during debug output only.
//
// RETURN:      None
//
// DESCRIPTION: Dump an ASCII string with support for ACPI-defined escape
// sequences.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_print_string(string: *mut c_char, max_length: u16) {
    void acpi_ut_print_string(char *string, u16 max_length)
    {
    u32 i;
    if (!string) {
    acpi_os_printf("<\"core::ptr::null_mut() STRING PTR\">");
    return;
    }
    acpi_os_printf("\"");
    for (i = 0; (i < max_length) && string[i]; i++) {
// Escape sequences
    switch (string[i]) {
    case 0x07:
    acpi_os_printf("\\a");	/* BELL */
    break;
    case 0x08:
    acpi_os_printf("\\b");	/* BACKSPACE */
    break;
    case 0x0C:
    acpi_os_printf("\\f");	/* FORMFEED */
    break;
    case 0x0A:
    acpi_os_printf("\\n");	/* LINEFEED */
    break;
    case 0x0D:
    acpi_os_printf("\\r");	/* CARRIAGE RETURN */
    break;
    case 0x09:
    acpi_os_printf("\\t");	/* HORIZONTAL TAB */
    break;
    case 0x0B:
    acpi_os_printf("\\v");	/* VERTICAL TAB */
    break;
    case '\'':	/* Single Quote */
    case '\"':	/* Double Quote */
    case '\\':	/* Backslash */
    acpi_os_printf("\\%c", (int)string[i]);
    break;
    default:
// Check for printable character or hex escape
    if (isprint((int)string[i])) {
// This is a normal character
    acpi_os_printf("%c", (int)string[i]);
    } else {
// All others will be Hex escapes
    acpi_os_printf("\\x%2.2X", (s32)string[i]);
    }
    break;
    }
    }
    acpi_os_printf("\"");
    if (i == max_length && string[i]) {
    acpi_os_printf("...");
    }
    }
//
// FUNCTION:    acpi_ut_repair_name
//
// PARAMETERS:  name            - The ACPI name to be repaired
//
// RETURN:      Repaired version of the name
//
// DESCRIPTION: Repair an ACPI name: Change invalid characters to '*' and
// return the new name. NOTE: the Name parameter must reside in
// read/write memory, cannot be a const.
//
// An ACPI Name must consist of valid ACPI characters. We will repair the name
// if necessary because we don't want to abort because of this, but we want
// all namespace names to be printable. A warning message is appropriate.
//
// This issue came up because there are in fact machines that exhibit
// this problem, and we want to be able to enable ACPI support for them,
// even though there are a few bad names.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_ut_repair_name(name: *mut c_char) {
    void acpi_ut_repair_name(char *name)
    {
    u32 i;
    let mut found_bad_char: u8 = FALSE;
    u32 original_name;
    ACPI_FUNCTION_NAME(ut_repair_name);
//
// Special case for the root node. This can happen if we get an
// error during the execution of module-level code.
//
    if (ACPI_COMPARE_NAMESEG(name, ACPI_ROOT_PATHNAME)) {
    return;
    }
    ACPI_COPY_NAMESEG(&original_name, &name[0]);
// Check each character in the name
    for (i = 0; i < ACPI_NAMESEG_SIZE; i++) {
    if (acpi_ut_valid_name_char(name[i], i)) {
    continue;
    }
//
// Replace a bad character with something printable, yet technically
// "odd". This prevents any collisions with existing "good"
// names in the namespace.
//
    name[i] = '_';
    found_bad_char = TRUE;
    }
    if (found_bad_char) {
// Report warning only if in strict mode or debug mode
    if (!acpi_gbl_enable_interpreter_slack) {
    ACPI_WARNING((AE_INFO,
    "Invalid character(s) in name (0x%.8X) %p, repaired: [%4.4s]",
    original_name, name, &name[0]));
    } else {
    ACPI_DEBUG_PRINT((ACPI_DB_INFO,
    "Invalid character(s) in name (0x%.8X), repaired: [%4.4s]",
    original_name, name));
    }
    }
    }

//
// FUNCTION:    ut_convert_backslashes
//
// PARAMETERS:  pathname        - File pathname string to be converted
//
// RETURN:      Modifies the input Pathname
//
// DESCRIPTION: Convert all backslashes (0x5C) to forward slashes (0x2F) within
// the entire input file pathname string.
//
#[no_mangle]
pub unsafe extern "C" fn ut_convert_backslashes(pathname: *mut c_char) {
    void ut_convert_backslashes(char *pathname)
    {
    if (!pathname) {
    return;
    }
    while (*pathname) {
    if (*pathname == '\\') {
// pathname = '/';
    }
    pathname++;
    }
    }
