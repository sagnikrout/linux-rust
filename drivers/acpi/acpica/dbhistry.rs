//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/dbhistry.c
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
// Module Name: dbhistry - debugger HISTORY command
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

    ACPI_MODULE_NAME("dbhistry")
pub const HI_NO_HISTORY: c_int = 0;
pub const HI_RECORD_HISTORY: c_int = 1;
pub const HISTORY_SIZE: c_int = 40;
    typedef struct history_info {
    char *command;
    u32 cmd_num;
    } HISTORY_INFO;
    static HISTORY_INFO acpi_gbl_history_buffer[HISTORY_SIZE];
    let mut acpi_gbl_lo_history: static u16 = 0;
    let mut acpi_gbl_num_history: static u16 = 0;
    let mut acpi_gbl_next_history_index: static u16 = 0;
//
// FUNCTION:    acpi_db_add_to_history
//
// PARAMETERS:  command_line    - Command to add
//
// RETURN:      None
//
// DESCRIPTION: Add a command line to the history buffer.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_db_add_to_history(command_line: *mut c_char) {
    void acpi_db_add_to_history(char *command_line)
    {
    u16 cmd_len;
    u16 buffer_len;
// Put command into the next available slot
    cmd_len = (u16)strlen(command_line);
    if (!cmd_len) {
    return;
    }
    if (acpi_gbl_history_buffer[acpi_gbl_next_history_index].command !=
    core::ptr::null_mut()) {
    buffer_len =
    (u16)
    strlen(acpi_gbl_history_buffer[acpi_gbl_next_history_index].
    command);
    if (cmd_len > buffer_len) {
    acpi_os_free(acpi_gbl_history_buffer
    [acpi_gbl_next_history_index].command);
    acpi_gbl_history_buffer[acpi_gbl_next_history_index].
    command = acpi_os_allocate(cmd_len + 1);
    }
    } else {
    acpi_gbl_history_buffer[acpi_gbl_next_history_index].command =
    acpi_os_allocate(cmd_len + 1);
    }
    strcpy(acpi_gbl_history_buffer[acpi_gbl_next_history_index].command,
    command_line);
    acpi_gbl_history_buffer[acpi_gbl_next_history_index].cmd_num =
    acpi_gbl_next_cmd_num;
// Adjust indexes
    if ((acpi_gbl_num_history == HISTORY_SIZE) &&
    (acpi_gbl_next_history_index == acpi_gbl_lo_history)) {
    acpi_gbl_lo_history++;
    if (acpi_gbl_lo_history >= HISTORY_SIZE) {
    acpi_gbl_lo_history = 0;
    }
    }
    acpi_gbl_next_history_index++;
    if (acpi_gbl_next_history_index >= HISTORY_SIZE) {
    acpi_gbl_next_history_index = 0;
    }
    acpi_gbl_next_cmd_num++;
    if (acpi_gbl_num_history < HISTORY_SIZE) {
    acpi_gbl_num_history++;
    }
    }
//
// FUNCTION:    acpi_db_display_history
//
// PARAMETERS:  None
//
// RETURN:      None
//
// DESCRIPTION: Display the contents of the history buffer
//
#[no_mangle]
pub unsafe extern "C" fn acpi_db_display_history() {
    void acpi_db_display_history(void)
    {
    u32 i;
    u16 history_index;
    history_index = acpi_gbl_lo_history;
// Dump entire history buffer
    for (i = 0; i < acpi_gbl_num_history; i++) {
    if (acpi_gbl_history_buffer[history_index].command) {
    acpi_os_printf("%3u %s\n",
    acpi_gbl_history_buffer[history_index].
    cmd_num,
    acpi_gbl_history_buffer[history_index].
    command);
    }
    history_index++;
    if (history_index >= HISTORY_SIZE) {
    history_index = 0;
    }
    }
    }
//
// FUNCTION:    acpi_db_get_from_history
//
// PARAMETERS:  command_num_arg         - String containing the number of the
// command to be retrieved
//
// RETURN:      Pointer to the retrieved command. Null on error.
//
// DESCRIPTION: Get a command from the history buffer
//
    char *acpi_db_get_from_history(char *command_num_arg)
    {
    u32 cmd_num;
    if (command_num_arg == core::ptr::null_mut()) {
    cmd_num = acpi_gbl_next_cmd_num - 1;
    }
    else {
    cmd_num = strtoul(command_num_arg, core::ptr::null_mut(), 0);
    }
    return (acpi_db_get_history_by_index(cmd_num));
    }
//
// FUNCTION:    acpi_db_get_history_by_index
//
// PARAMETERS:  cmd_num             - Index of the desired history entry.
// Values are 0...(acpi_gbl_next_cmd_num - 1)
//
// RETURN:      Pointer to the retrieved command. Null on error.
//
// DESCRIPTION: Get a command from the history buffer
//
    char *acpi_db_get_history_by_index(u32 cmd_num)
    {
    u32 i;
    u16 history_index;
// Search history buffer
    history_index = acpi_gbl_lo_history;
    for (i = 0; i < acpi_gbl_num_history; i++) {
    if (acpi_gbl_history_buffer[history_index].cmd_num == cmd_num) {
// Found the command, return it
    return (acpi_gbl_history_buffer[history_index].command);
    }
// History buffer is circular
    history_index++;
    if (history_index >= HISTORY_SIZE) {
    history_index = 0;
    }
    }
    acpi_os_printf("Invalid history number: %u\n", history_index);
    return (core::ptr::null_mut());
    }
