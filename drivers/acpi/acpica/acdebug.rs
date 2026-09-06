//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acdebug.h
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
// Name: acdebug.h - ACPI/AML debugger
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// The debugger is used in conjunction with the disassembler most of time

pub const ACPI_DEBUG_BUFFER_SIZE: c_uint = 0x4000	/* 16K buffer for return objects */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_db_command_info {
    pub /: *const *const *const char name; / Command Name,
    pub /: *mut *mut u8 min_args; / Minimum arguments required,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_db_command_help {
    pub /: *mut *mut u8 line_count; / Number of help lines,
    pub /: *mut *mut *mut char invocation; / Command Invocation,
    pub /: *mut *mut *mut char description; / Command Description,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_db_argument_info {
    pub /: *const *const *const char name; / Argument Name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_db_execute_walk {
    pub count: u32,
    pub max_count: u32,
    pub 1]: char name_seg[ACPI_NAMESEG_SIZE +,
}

pub const EX_NO_SINGLE_STEP: c_int = 1;
pub const EX_SINGLE_STEP: c_int = 2;
pub const EX_ALL: c_int = 4;
//
// dbxface - external debugger interfaces
//
// walk_state,
// walk_state))
//
// dbcmds - debug commands and output routines
//
extern "C" {
    pub fn acpi_db_display_table_info(table_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_display_template(buffer_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_unload_acpi_table(name: *mut c_char);
}
extern "C" {
    pub fn acpi_db_send_notify(name: *mut c_char, value: u32);
}
extern "C" {
    pub fn acpi_db_display_interfaces(action_arg: *mut c_char, interface_name_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_sleep(object_arg: *mut c_char) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_trace(enable_arg: *mut c_char, method_arg: *mut c_char, once_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_display_locks();
}
extern "C" {
    pub fn acpi_db_display_resources(object_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_display_handlers();
}
extern "C" {
    pub fn acpi_db_execute_test(type_arg: *mut c_char);
}
//
// dbconvert - miscellaneous conversion routines
//
extern "C" {
    pub fn acpi_db_hex_char_to_value(hex_char: c_int, return_value: *mut u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_convert_to_package(string: *mut c_char, object: *mut acpi_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_dump_pld_buffer(obj_desc: *mut acpi_object);
}
//
// dbmethod - control method commands
//
extern "C" {
    pub fn acpi_db_set_method_call_breakpoint(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn acpi_db_set_method_data(type_arg: *mut c_char, index_arg: *mut c_char, value_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_disassemble_method(name: *mut c_char) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_disassemble_aml(statements: *mut c_char, op: *mut acpi_parse_object);
}
extern "C" {
    pub fn acpi_db_evaluate_predefined_names();
}
extern "C" {
    pub fn acpi_db_evaluate_all(name_seg: *mut c_char);
}
//
// dbnames - namespace commands
//
extern "C" {
    pub fn acpi_db_set_scope(name: *mut c_char);
}
extern "C" {
    pub fn acpi_db_dump_namespace(start_arg: *mut c_char, depth_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_dump_namespace_paths();
}
extern "C" {
    pub fn acpi_db_dump_namespace_by_owner(owner_arg: *mut c_char, depth_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_find_name_in_namespace(name_arg: *mut c_char) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_check_predefined_names();
}
extern "C" {
    pub fn acpi_db_check_integrity();
}
extern "C" {
    pub fn acpi_db_find_references(object_arg: *mut c_char);
}
extern "C" {
    pub fn acpi_db_get_bus_info();
}
extern "C" {
    pub fn acpi_db_display_fields(address_space_id: u32) -> acpi_status;
}
//
// dbdisply - debug display commands
//
extern "C" {
    pub fn acpi_db_display_method_info(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn acpi_db_decode_and_display_object(target: *mut c_char, output_type: *mut c_char);
}
// obj_desc,
// walk_state))
extern "C" {
    pub fn acpi_db_display_all_methods(display_count_arg: *mut c_char) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_display_arguments();
}
extern "C" {
    pub fn acpi_db_display_locals();
}
extern "C" {
    pub fn acpi_db_display_results();
}
extern "C" {
    pub fn acpi_db_display_calling_tree();
}
extern "C" {
    pub fn acpi_db_display_object_type(object_arg: *mut c_char);
}
// obj_desc,
// walk_state))
//
// dbexec - debugger control method execution
//
extern "C" {
    pub fn acpi_db_delete_objects(count: u32, objects: *mut acpi_object);
}

extern "C" {
    pub fn acpi_db_get_cache_info(cache: *mut acpi_memory_list) -> u32;
}

//
// dbfileio - Debugger file I/O commands
//
extern "C" {
    pub fn acpi_db_close_debug_file();
}
extern "C" {
    pub fn acpi_db_open_debug_file(name: *mut c_char);
}
extern "C" {
    pub fn acpi_db_load_acpi_table(filename: *mut c_char) -> acpi_status;
}
extern "C" {
    pub fn acpi_db_load_tables(list_head: *mut acpi_new_table_desc) -> acpi_status;
}
//
// dbhistry - debugger HISTORY command
//
extern "C" {
    pub fn acpi_db_add_to_history(command_line: *mut c_char);
}
extern "C" {
    pub fn acpi_db_display_history();
}
//
// dbinput - user front-end to the AML debugger
//
extern "C" {
    pub fn acpi_db_execute_thread(context: *mut c_void) -> void ACPI_SYSTEM_XFACE;
}
extern "C" {
    pub fn acpi_db_user_commands() -> acpi_status;
}
//
// dbobject
//
extern "C" {
    pub fn acpi_db_decode_internal_object(obj_desc: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_db_decode_arguments(walk_state: *mut acpi_walk_state);
}
extern "C" {
    pub fn acpi_db_decode_locals(walk_state: *mut acpi_walk_state);
}
//
// dbstats - Generation and display of ACPI table statistics
//
extern "C" {
    pub fn acpi_db_generate_statistics(root: *mut acpi_parse_object, is_method: u8);
}
extern "C" {
    pub fn acpi_db_display_statistics(type_arg: *mut c_char) -> acpi_status;
}
//
// dbutils - AML debugger utilities
//
extern "C" {
    pub fn acpi_db_set_output_destination(where: u32);
}
extern "C" {
    pub fn acpi_db_dump_external_object(obj_desc: *mut acpi_object, level: u32);
}
extern "C" {
    pub fn acpi_db_prep_namestring(name: *mut c_char);
}
extern "C" {
    pub fn acpi_db_uint32_to_hex_string(value: u32, buffer: *mut c_char);
}
extern "C" {
    pub fn acpi_db_generate_interrupt(gsiv_arg: *mut c_char);
}
