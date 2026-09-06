//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acinterp.h
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
// Name: acinterp.h - Interpreter subcomponent prototypes and defines
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

// Macros for tables used for debug output

//
// If possible, pack the following structures to byte alignment, since we
// don't care about performance for debug output. Two cases where we cannot
// pack the structures:
//
// 1) Hardware does not support misaligned memory transfers
// 2) Compiler does not support pointers within packed structures
//

// Values for the Opcode field above
pub const ACPI_EXD_INIT: c_int = 0;
pub const ACPI_EXD_TYPE: c_int = 1;
pub const ACPI_EXD_UINT8: c_int = 2;
pub const ACPI_EXD_UINT16: c_int = 3;
pub const ACPI_EXD_UINT32: c_int = 4;
pub const ACPI_EXD_UINT64: c_int = 5;
pub const ACPI_EXD_LITERAL: c_int = 6;
pub const ACPI_EXD_POINTER: c_int = 7;
pub const ACPI_EXD_ADDRESS: c_int = 8;
pub const ACPI_EXD_STRING: c_int = 9;
pub const ACPI_EXD_BUFFER: c_int = 10;
pub const ACPI_EXD_PACKAGE: c_int = 11;
pub const ACPI_EXD_FIELD: c_int = 12;
pub const ACPI_EXD_REFERENCE: c_int = 13;

// restore default alignment

//
// exconvrt - object conversion
//
// Types for ->String conversion
pub const ACPI_EXPLICIT_BYTE_COPY: c_uint = 0x00000000;
pub const ACPI_EXPLICIT_CONVERT_HEX: c_uint = 0x00000001;
pub const ACPI_IMPLICIT_CONVERT_HEX: c_uint = 0x00000002;
pub const ACPI_EXPLICIT_CONVERT_DECIMAL: c_uint = 0x00000003;
//
// exdebug - AML debug object
//
// exfield - ACPI AML (p-code) execution - field manipulation
//
// exfldio - low level field I/O
//
// exmisc - misc support routines
//
extern "C" {
    pub fn acpi_ex_do_math_op(opcode: u16, operand0: u64, operand1: u64) -> u64;
}
extern "C" {
    pub fn acpi_ex_create_mutex(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_create_processor(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_create_power_resource(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_create_event(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_create_alias(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// exconfig - dynamic table load/unload
//
extern "C" {
    pub fn acpi_ex_unload_table(ddb_handle: *mut acpi_operand_object) -> acpi_status;
}
//
// exmutex - mutex support
//
extern "C" {
    pub fn acpi_ex_release_mutex_object(obj_desc: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_release_all_mutexes(thread: *mut acpi_thread_state);
}
extern "C" {
    pub fn acpi_ex_unlink_mutex(obj_desc: *mut acpi_operand_object);
}
//
// exprep - ACPI AML execution - prep utilities
//
extern "C" {
    pub fn acpi_ex_prep_field_value(info: *mut acpi_create_field_info) -> acpi_status;
}
//
// exserial - field_unit support for serial address spaces
//
// exsystem - Interface to OS services
//
extern "C" {
    pub fn acpi_ex_system_do_sleep(time: u64) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_system_do_stall(time: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_system_signal_event(obj_desc: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_system_reset_event(obj_desc: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_system_wait_mutex(mutex: acpi_mutex, timeout: u16) -> acpi_status;
}
//
// exoparg1 - ACPI AML execution, 1 operand
//
extern "C" {
    pub fn acpi_ex_opcode_0A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_1A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_1A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_1A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_1A_1T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// exoparg2 - ACPI AML execution, 2 operands
//
extern "C" {
    pub fn acpi_ex_opcode_2A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_2A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_2A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_2A_2T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// exoparg3 - ACPI AML execution, 3 operands
//
extern "C" {
    pub fn acpi_ex_opcode_3A_0T_0R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ex_opcode_3A_1T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// exoparg6 - ACPI AML execution, 6 operands
//
extern "C" {
    pub fn acpi_ex_opcode_6A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// exresolv - Object resolution and get value functions
//
// exresnte - resolve namespace node
//
// exresop - resolve operand to value
//
// exdump - Interpreter debug output routines
//
extern "C" {
    pub fn acpi_ex_dump_operand(obj_desc: *mut acpi_operand_object, depth: u32);
}
extern "C" {
    pub fn acpi_ex_dump_namespace_node(node: *mut acpi_namespace_node, flags: u32);
}
//
// exnames - AML namestring support
//
// exstore - Object store support
//
// exstoren - resolve/store object
//
// exstorob - store object - buffer/string
//
// excopy - object copy
//
// exutils - interpreter/scanner utilities
//
extern "C" {
    pub fn acpi_ex_enter_interpreter();
}
extern "C" {
    pub fn acpi_ex_exit_interpreter();
}
extern "C" {
    pub fn acpi_ex_truncate_for32bit_table(obj_desc: *mut acpi_operand_object) -> u8;
}
extern "C" {
    pub fn acpi_ex_acquire_global_lock(rule: u32);
}
extern "C" {
    pub fn acpi_ex_release_global_lock(rule: u32);
}
extern "C" {
    pub fn acpi_ex_eisa_id_to_string(dest: *mut c_char, compressed_id: u64);
}
extern "C" {
    pub fn acpi_ex_integer_to_string(dest: *mut c_char, value: u64);
}
extern "C" {
    pub fn acpi_ex_pci_cls_to_string(dest: *mut c_char, class_code[3]: u8);
}
extern "C" {
    pub fn acpi_is_valid_space_id(space_id: u8) -> u8;
}
//
// exregion - default op_region handlers
//
