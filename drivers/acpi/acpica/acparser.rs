//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acparser.h
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
// Module Name: acparser.h - AML Parser subcomponent prototypes and defines
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
pub const OP_HAS_RETURN_VALUE: c_int = 1;
// Variable number of arguments. This field must be 32 bits

pub const ACPI_PARSE_DELETE_TREE: c_uint = 0x0001;
pub const ACPI_PARSE_NO_TREE_DELETE: c_uint = 0x0000;
pub const ACPI_PARSE_TREE_MASK: c_uint = 0x0001;
pub const ACPI_PARSE_LOAD_PASS1: c_uint = 0x0010;
pub const ACPI_PARSE_LOAD_PASS2: c_uint = 0x0020;
pub const ACPI_PARSE_EXECUTE: c_uint = 0x0030;
pub const ACPI_PARSE_MODE_MASK: c_uint = 0x0030;
pub const ACPI_PARSE_DEFERRED_OP: c_uint = 0x0100;
pub const ACPI_PARSE_DISASSEMBLE: c_uint = 0x0200;
pub const ACPI_PARSE_MODULE_LEVEL: c_uint = 0x0400;
//
// Parser interfaces
//
// psxface - Parser external interfaces
//
extern "C" {
    pub fn acpi_ps_execute_method(info: *mut acpi_evaluate_info) -> acpi_status;
}
extern "C" {
    pub fn acpi_ps_execute_table(info: *mut acpi_evaluate_info) -> acpi_status;
}
//
// psargs - Parse AML opcode arguments
//
// Values for u8 above

//
// psfind
//
// psobject - support for parse object processing
//
// psopinfo - AML Opcode information
//
extern "C" {
    pub fn acpi_ps_get_argument_count(op_type: u32) -> u8;
}
//
// psparse - top level parsing routines
//
extern "C" {
    pub fn acpi_ps_parse_aml(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ps_get_opcode_size(opcode: u32) -> u32;
}
extern "C" {
    pub fn acpi_ps_peek_opcode(state: *mut acpi_parse_state) -> u16;
}
//
// psloop - main parse loop
//
extern "C" {
    pub fn acpi_ps_parse_loop(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// psscope - Scope stack management routines
//
// state);
extern "C" {
    pub fn acpi_ps_has_completed_scope(parser_state: *mut acpi_parse_state) -> u8;
}
extern "C" {
    pub fn acpi_ps_cleanup_scope(state: *mut acpi_parse_state);
}
//
// pstree - parse tree manipulation routines
//
// pswalk - parse tree walk routines
//
extern "C" {
    pub fn acpi_ps_delete_completed_op(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ps_delete_parse_tree(root: *mut acpi_parse_object);
}
//
// psutils - parser utilities
//
extern "C" {
    pub fn acpi_ps_init_op(op: *mut acpi_parse_object, opcode: u16);
}
extern "C" {
    pub fn acpi_ps_free_op(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn acpi_ps_is_leading_char(c: u32) -> u8;
}
extern "C" {
    pub fn acpi_ps_get_name(op: *mut acpi_parse_object) -> u32;
}
extern "C" {
    pub fn acpi_ps_set_name(op: *mut acpi_parse_object, name: u32);
}
//
// psdump - display parser tree
//
extern "C" {
    pub fn acpi_ps_show(op: *mut acpi_parse_object);
}
