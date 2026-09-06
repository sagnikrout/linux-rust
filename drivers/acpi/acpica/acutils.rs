//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acutils.h
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
// Name: acutils.h -- prototypes for the common (subsystem-wide) procedures
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Strings used by the disassembler and debugger resource dump routines

//
// For the iASL compiler case, the output is redirected to stderr so that
// any of the various ACPI errors and warnings do not appear in the output
// files, for either the compiler or disassembler portions of the tool.
//

//
// non-iASL case - no redirection, nothing to do
//
// Macro flag: #define ACPI_MSG_REDIRECT_BEGIN
// Macro flag: #define ACPI_MSG_REDIRECT_END

//
// Common error message prefixes
//

//
// Common message suffix
//

// Flags to indicate implicit or explicit string-to-integer conversion

// Types for Resource descriptor entries
pub const ACPI_INVALID_RESOURCE: c_int = 0;
pub const ACPI_FIXED_LENGTH: c_int = 1;
pub const ACPI_VARIABLE_LENGTH: c_int = 2;
pub const ACPI_SMALL_VARIABLE_LENGTH: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pkg_info {
    pub free_space: *mut u8,
    pub length: acpi_size,
    pub object_space: u32,
    pub num_packages: u32,
}

// Object reference counts

// acpi_ut_dump_buffer
pub const DB_BYTE_DISPLAY: c_uint = 0x01;
pub const DB_WORD_DISPLAY: c_uint = 0x02;
pub const DB_DWORD_DISPLAY: c_uint = 0x04;
pub const DB_QWORD_DISPLAY: c_uint = 0x08;
pub const DB_DISPLAY_DATA_ONLY: c_uint = 0x10;
//
// utascii - ASCII utilities
//
extern "C" {
    pub fn acpi_ut_valid_nameseg(signature: *mut c_char) -> u8;
}
extern "C" {
    pub fn acpi_ut_valid_name_char(character: c_char, position: u32) -> u8;
}
extern "C" {
    pub fn acpi_ut_check_and_repair_ascii(name: *mut u8, repaired_name: *mut c_char, count: u32);
}
//
// utcksum - Checksum utilities
//
extern "C" {
    pub fn acpi_ut_generate_checksum(table: *mut c_void, length: u32, original_checksum: u8) -> u8;
}
extern "C" {
    pub fn acpi_ut_checksum(buffer: *mut u8, length: u32) -> u8;
}
//
// utnonansi - Non-ANSI C library functions
//
extern "C" {
    pub fn acpi_ut_strupr(src_string: *mut c_char);
}
extern "C" {
    pub fn acpi_ut_strlwr(src_string: *mut c_char);
}
extern "C" {
    pub fn acpi_ut_stricmp(string1: *mut c_char, string2: *mut c_char) -> c_int;
}
//
// utstrsuppt - string-to-integer conversion support functions
//
extern "C" {
    pub fn acpi_ut_convert_octal_string(string: *mut c_char, return_value: *mut u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_convert_decimal_string(string: *mut c_char, return_value_ptr: *mut u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_convert_hex_string(string: *mut c_char, return_value_ptr: *mut u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_remove_whitespace(string: *mut c_char) -> c_char;
}
extern "C" {
    pub fn acpi_ut_remove_leading_zeros(string: *mut c_char) -> c_char;
}
extern "C" {
    pub fn acpi_ut_detect_hex_prefix(string: *mut c_char) -> u8;
}
extern "C" {
    pub fn acpi_ut_remove_hex_prefix(string: *mut c_char);
}
extern "C" {
    pub fn acpi_ut_detect_octal_prefix(string: *mut c_char) -> u8;
}
//
// utstrtoul64 - string-to-integer conversion functions
//
extern "C" {
    pub fn acpi_ut_strtoul64(string: *mut c_char, ret_integer: *mut u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_explicit_strtoul64(string: *mut c_char) -> u64;
}
extern "C" {
    pub fn acpi_ut_implicit_strtoul64(string: *mut c_char) -> u64;
}
//
// utglobal - Global data structures and procedures
//
extern "C" {
    pub fn acpi_ut_init_globals() -> acpi_status;
}

extern "C" {
    pub fn acpi_ut_hex_to_ascii_char(integer: u64, position: u32) -> c_char;
}
extern "C" {
    pub fn acpi_ut_ascii_to_hex_byte(two_ascii_chars: *mut c_char, return_byte: *mut u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_ascii_char_to_hex(hex_char: c_int) -> u8;
}
extern "C" {
    pub fn acpi_ut_valid_object_type(type: acpi_object_type) -> u8;
}
//
// utinit - miscellaneous initialization and shutdown
//
extern "C" {
    pub fn acpi_ut_hardware_initialize() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_subsystem_shutdown();
}
//
// utcopy - Object construction and conversion interfaces
//
// utcreate - Object creation
//
// utdebug - Debug interfaces
//
extern "C" {
    pub fn acpi_ut_init_stack_ptr_trace();
}
extern "C" {
    pub fn acpi_ut_track_stack_ptr();
}
extern "C" {
    pub fn acpi_ut_dump_buffer(buffer: *mut u8, count: u32, display: u32, offset: u32);
}

extern "C" {
    pub fn acpi_ut_report_error(module_name: *mut c_char, line_number: u32);
}
extern "C" {
    pub fn acpi_ut_report_info(module_name: *mut c_char, line_number: u32);
}
extern "C" {
    pub fn acpi_ut_report_warning(module_name: *mut c_char, line_number: u32);
}
//
// utdelete - Object deletion and reference counts
//
extern "C" {
    pub fn acpi_ut_add_reference(object: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_ut_remove_reference(object: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_ut_delete_internal_package_object(object: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_ut_delete_internal_simple_object(object: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_ut_delete_internal_object_list(obj_list: *mut acpi_operand_object);
}
//
// uteval - object evaluation
//
// utids - device ID support
//
// utlock - reader/writer locks
//
extern "C" {
    pub fn acpi_ut_create_rw_lock(lock: *mut acpi_rw_lock) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_delete_rw_lock(lock: *mut acpi_rw_lock);
}
extern "C" {
    pub fn acpi_ut_acquire_read_lock(lock: *mut acpi_rw_lock) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_release_read_lock(lock: *mut acpi_rw_lock) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_acquire_write_lock(lock: *mut acpi_rw_lock) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_release_write_lock(lock: *mut acpi_rw_lock);
}
//
// utobject - internal object create/delete/cache routines
//
// module_name,

extern "C" {
    pub fn acpi_ut_delete_object_desc(object: *mut acpi_operand_object);
}
extern "C" {
    pub fn acpi_ut_valid_internal_object(object: *mut c_void) -> u8;
}
//
// utosi - Support for the _OSI predefined control method
//
extern "C" {
    pub fn acpi_ut_initialize_interfaces() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_interface_terminate() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_install_interface(interface_name: acpi_string) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_remove_interface(interface_name: acpi_string) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_update_interfaces(action: u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_osi_implementation(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// utpredef - support for predefined names
//
// this_name);
extern "C" {
    pub fn acpi_ut_get_expected_return_types(buffer: *mut c_char, expected_btypes: u32);
}

extern "C" {
    pub fn acpi_ut_get_resource_bit_width(buffer: *mut c_char, types: u16) -> u32;
}

//
// utstate - Generic state creation/cache routines
//
// list_head);
// object, u16 action);
extern "C" {
    pub fn acpi_ut_delete_generic_state(state: *mut acpi_generic_state);
}
//
// utmath
//
extern "C" {
    pub fn acpi_ut_short_shift_left(operand: u64, count: u32, out_result: *mut u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_short_shift_right(operand: u64, count: u32, out_result: *mut u64) -> acpi_status;
}
//
// utmisc
//
extern "C" {
    pub fn acpi_ut_is_pci_root_bridge(id: *mut c_char) -> u8;
}

extern "C" {
    pub fn acpi_ut_is_aml_table(table: *mut acpi_table_header) -> u8;
}

// Values for Base above (16=Hex, 10=Decimal)
pub const ACPI_ANY_BASE: c_int = 0;
extern "C" {
    pub fn acpi_ut_dword_byte_swap(value: u32) -> u32;
}
extern "C" {
    pub fn acpi_ut_set_integer_width(revision: u8);
}

//
// utownerid - Support for Table/Method Owner IDs
//
extern "C" {
    pub fn acpi_ut_allocate_owner_id(owner_id: *mut acpi_owner_id) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_release_owner_id(owner_id: *mut acpi_owner_id);
}
//
// utresrc
//
extern "C" {
    pub fn acpi_ut_get_descriptor_length(aml: *mut c_void) -> u32;
}
extern "C" {
    pub fn acpi_ut_get_resource_length(aml: *mut c_void) -> u16;
}
extern "C" {
    pub fn acpi_ut_get_resource_header_length(aml: *mut c_void) -> u8;
}
extern "C" {
    pub fn acpi_ut_get_resource_type(aml: *mut c_void) -> u8;
}
//
// utstring - String and character utilities
//
extern "C" {
    pub fn acpi_ut_print_string(string: *mut c_char, max_length: u16);
}

extern "C" {
    pub fn ut_convert_backslashes(pathname: *mut c_char);
}

extern "C" {
    pub fn acpi_ut_repair_name(name: *mut c_char);
}

extern "C" {
    pub fn acpi_ut_safe_strcpy(dest: *mut c_char, dest_size: acpi_size, source: *mut c_char) -> u8;
}
extern "C" {
    pub fn acpi_ut_safe_strcat(dest: *mut c_char, dest_size: acpi_size, source: *mut c_char) -> u8;
}

//
// utmutex - mutex support
//
extern "C" {
    pub fn acpi_ut_mutex_initialize() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_mutex_terminate();
}
extern "C" {
    pub fn acpi_ut_acquire_mutex(mutex_id: acpi_mutex_handle) -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_release_mutex(mutex_id: acpi_mutex_handle) -> acpi_status;
}
//
// utalloc - memory allocation and object caching
//
extern "C" {
    pub fn acpi_ut_create_caches() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_delete_caches() -> acpi_status;
}
extern "C" {
    pub fn acpi_ut_validate_buffer(buffer: *mut acpi_buffer) -> acpi_status;
}

extern "C" {
    pub fn acpi_ut_dump_allocation_info();
}
extern "C" {
    pub fn acpi_ut_dump_allocations(component: u32, module: *const c_char);
}

//
// utaddress - address range check
//
extern "C" {
    pub fn acpi_ut_delete_address_lists();
}
//
// utxferror - various error/warning output functions
//
// Utility functions for ACPI names and IDs
//
// utuuid -- UUID support functions
//

extern "C" {
    pub fn acpi_ut_convert_string_to_uuid(in_string: *mut c_char, uuid_buffer: *mut u8);
}
extern "C" {
    pub fn acpi_ut_convert_uuid_to_string(uuid_buffer: *mut c_char, out_string: *mut c_char) -> acpi_status;
}

