//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acconvert.h
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
// Module Name: acapps - common include for ACPI applications/tools
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Definitions for comment state
pub const ASL_COMMENT_STANDARD: c_int = 1;
pub const ASLCOMMENT_INLINE: c_int = 2;
pub const ASL_COMMENT_OPEN_PAREN: c_int = 3;
pub const ASL_COMMENT_CLOSE_PAREN: c_int = 4;
pub const ASL_COMMENT_CLOSE_BRACE: c_int = 5;
// Definitions for comment print function
pub const AML_COMMENT_STANDARD: c_int = 1;
pub const AMLCOMMENT_INLINE: c_int = 2;
pub const AML_COMMENT_END_NODE: c_int = 3;
pub const AML_NAMECOMMENT: c_int = 4;
pub const AML_COMMENT_CLOSE_BRACE: c_int = 5;
pub const AML_COMMENT_ENDBLK: c_int = 6;
pub const AML_COMMENT_INCLUDE: c_int = 7;

//
// cvcompiler
//
extern "C" {
    pub fn cv_calculate_comment_lengths(op: *mut acpi_parse_object) -> u32;
}
extern "C" {
    pub fn cv_process_comment_state(input: c_char);
}
extern "C" {
    pub fn cv_add_to_comment_list(to_add: *mut c_char);
}
extern "C" {
    pub fn cv_place_comment(type: u8, comment_string: *mut c_char);
}
extern "C" {
    pub fn cv_parse_op_block_type(op: *mut acpi_parse_object) -> u32;
}
extern "C" {
    pub fn cg_write_aml_def_block_comment(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn cg_write_aml_comment(op: *mut acpi_parse_object);
}
//
// cvparser
//
extern "C" {
    pub fn cv_init_file_tree(table: *mut acpi_table_header, root_file: *mut *mut FILE);
}
extern "C" {
    pub fn cv_clear_op_comments(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn cv_label_file_node(op: *mut acpi_parse_object);
}
extern "C" {
    pub fn cv_capture_comments_only(parser_state: *mut acpi_parse_state);
}
extern "C" {
    pub fn cv_capture_comments(walk_state: *mut acpi_walk_state);
}
extern "C" {
    pub fn cv_transfer_comments(op: *mut acpi_parse_object);
}
//
// cvdisasm
//
extern "C" {
    pub fn cv_switch_files(level: u32, op: *mut acpi_parse_object);
}
extern "C" {
    pub fn cv_file_has_switched(op: *mut acpi_parse_object) -> u8;
}
extern "C" {
    pub fn cv_close_paren_write_comment(op: *mut acpi_parse_object, level: u32);
}
extern "C" {
    pub fn cv_close_brace_write_comment(op: *mut acpi_parse_object, level: u32);
}

