//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acdispat.h
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
// Name: acdispat.h - dispatcher (parser to interpreter interface)
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

//
// dsargs - execution of dynamic arguments for static objects
//
extern "C" {
    pub fn acpi_ds_get_region_arguments(rgn_desc: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ds_get_buffer_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ds_get_package_arguments(obj_desc: *mut acpi_operand_object) -> acpi_status;
}
//
// dscontrol - support for execution control opcodes
//
// dsopcode - support for late operand evaluation
//
extern "C" {
    pub fn acpi_ds_initialize_region(obj_handle: acpi_handle) -> acpi_status;
}
//
// dsexec - Parser/Interpreter interface, method execution callbacks
//
extern "C" {
    pub fn acpi_ds_exec_end_op(state: *mut acpi_walk_state) -> acpi_status;
}
//
// dsfield - Parser/Interpreter interface for AML fields
//
// dsload - Parser/Interpreter interface
//
// dsload - pass 1 namespace load callbacks
extern "C" {
    pub fn acpi_ds_load1_end_op(walk_state: *mut acpi_walk_state) -> acpi_status;
}
// dsload - pass 2 namespace load callbacks
extern "C" {
    pub fn acpi_ds_load2_end_op(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// dsmthdat - method data (locals/args)
//
extern "C" {
    pub fn acpi_ds_method_data_delete_all(walk_state: *mut acpi_walk_state);
}
extern "C" {
    pub fn acpi_ds_is_method_value(obj_desc: *mut acpi_operand_object) -> u8;
}
extern "C" {
    pub fn acpi_ds_method_data_init(walk_state: *mut acpi_walk_state);
}
//
// dsmethod - Parser/Interpreter interface - control method parsing
//
// dsinit
//
// dsobject - Parser/Interpreter interface - object initialization and conversion
//
// dspkginit - Package object initialization
//
// dsutils - Parser/Interpreter interface utility routines
//
extern "C" {
    pub fn acpi_ds_clear_implicit_return(walk_state: *mut acpi_walk_state);
}
extern "C" {
    pub fn acpi_ds_resolve_operands(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ds_clear_operands(walk_state: *mut acpi_walk_state);
}
extern "C" {
    pub fn acpi_ds_evaluate_name_path(walk_state: *mut acpi_walk_state) -> acpi_status;
}
//
// dswscope - Scope Stack manipulation
//
extern "C" {
    pub fn acpi_ds_scope_stack_pop(walk_state: *mut acpi_walk_state) -> acpi_status;
}
extern "C" {
    pub fn acpi_ds_scope_stack_clear(walk_state: *mut acpi_walk_state);
}
//
// dswstate - parser WALK_STATE management routines
//
// origin,
// mth_desc,
// thread);
extern "C" {
    pub fn acpi_ds_delete_walk_state(walk_state: *mut acpi_walk_state);
}
// thread);
extern "C" {
    pub fn acpi_ds_result_stack_clear(walk_state: *mut acpi_walk_state) -> acpi_status;
}
// thread);
//
// dsdebug - parser debugging routines
//
