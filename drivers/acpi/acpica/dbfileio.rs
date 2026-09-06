//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/dbfileio.c
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
// Module Name: dbfileio - Debugger file I/O commands. These can't usually
// be used when running the debugger in Ring 0 (Kernel mode)
//

    ACPI_MODULE_NAME("dbfileio")

//
// FUNCTION:    acpi_db_close_debug_file
//
// PARAMETERS:  None
//
// RETURN:      None
//
// DESCRIPTION: If open, close the current debug output file
//
#[no_mangle]
pub unsafe extern "C" fn acpi_db_close_debug_file() {
    void acpi_db_close_debug_file(void)
    {
    if (acpi_gbl_debug_file) {
    fclose(acpi_gbl_debug_file);
    acpi_gbl_debug_file = core::ptr::null_mut();
    acpi_gbl_db_output_to_file = FALSE;
    acpi_os_printf("Debug output file %s closed\n",
    acpi_gbl_db_debug_filename);
    }
    }
//
// FUNCTION:    acpi_db_open_debug_file
//
// PARAMETERS:  name                - Filename to open
//
// RETURN:      None
//
// DESCRIPTION: Open a file where debug output will be directed.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_db_open_debug_file(name: *mut c_char) {
    void acpi_db_open_debug_file(char *name)
    {
    acpi_db_close_debug_file();
    acpi_gbl_debug_file = fopen(name, "w+");
    if (!acpi_gbl_debug_file) {
    acpi_os_printf("Could not open debug file %s\n", name);
    return;
    }
    acpi_os_printf("Debug output file %s opened\n", name);
    acpi_ut_safe_strncpy(acpi_gbl_db_debug_filename, name,
    sizeof(acpi_gbl_db_debug_filename));
    acpi_gbl_db_output_to_file = TRUE;
    }

//
// FUNCTION:    acpi_db_load_tables
//
// PARAMETERS:  list_head       - List of ACPI tables to load
//
// RETURN:      Status
//
// DESCRIPTION: Load ACPI tables from a previously constructed table list.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_db_load_tables(list_head: *mut acpi_new_table_desc) -> acpi_status {
    acpi_status acpi_db_load_tables(struct acpi_new_table_desc *list_head)
    {
    acpi_status status;
    struct acpi_new_table_desc *table_list_head;
    struct acpi_table_header *table;
// Load all ACPI tables in the list
    table_list_head = list_head;
    while (table_list_head) {
    table = table_list_head.table;
    status = acpi_load_table(table, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    if (status == AE_ALREADY_EXISTS) {
    acpi_os_printf
    ("Table %4.4s is already installed\n",
    table.signature);
    } else {
    acpi_os_printf("Could not install table, %s\n",
    acpi_format_exception(status));
    }
    return (status);
    }
    acpi_os_printf
    ("Acpi table [%4.4s] successfully installed and loaded\n",
    table.signature);
    table_list_head = table_list_head.next;
    }
    return (AE_OK);
    }
