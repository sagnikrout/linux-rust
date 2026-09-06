//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acstruct.h
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
// Name: acstruct.h - Internal structs
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// acpisrc:struct_defs -- for acpisrc conversion
//
// Tree walking typedefs and structs
//
// Walk state - current state of a parse tree walk. Used for both a leisurely
// stroll through the tree (for whatever reason), and for control method
// execution.
//
pub const ACPI_NEXT_OP_DOWNWARD: c_int = 1;
pub const ACPI_NEXT_OP_UPWARD: c_int = 2;
//
// Groups of definitions for walk_type used for different implementations of
// walkers (never simultaneously) - flags for interpreter:
//
pub const ACPI_WALK_NON_METHOD: c_int = 0;
pub const ACPI_WALK_METHOD: c_uint = 0x01;
pub const ACPI_WALK_METHOD_RESTART: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_walk_state {
    pub /: *mut *mut *mut acpi_walk_state next; / Next walk_state in list,
    pub /: *mut *mut u8 descriptor_type; / To differentiate various internal objs,
    pub walk_type: u8,
    pub /: *mut *mut u16 opcode; / Current AML opcode,
    pub /: *mut *mut u8 next_op_info; / Info about next_op,
    pub /: *mut *mut u8 num_operands; / Stack pointer for Operands[] array,
    pub /: *mut *mut u8 operand_index; / Index into operand stack, to be used by acpi_ds_obj_stack_push,
    pub /: *mut *mut acpi_owner_id owner_id; / Owner of objects created during the walk,
    pub /: *mut *mut u8 last_predicate; / Result of last predicate,
    pub current_result: u8,
    pub return_used: u8,
    pub scope_depth: u8,
    pub /: *mut *mut u8 pass_number; / Parse pass during table load,
    pub /: *mut *mut u8 namespace_override; / Override existing objects,
    pub /: *mut *mut u8 result_size; / Total elements for the result stack,
    pub /: *mut *mut u8 result_count; / Current number of occupied elements of result stack,
    pub aml: *mut u8,
    pub arg_types: u32,
    pub /: *mut *mut u32 method_breakpoint; / For single stepping,
    pub /: *mut *mut u32 user_breakpoint; / User AML breakpoint,
    pub parse_flags: u32,
    pub /: *mut *mut acpi_parse_state parser_state; / Current state of parser,
    pub prev_arg_types: u32,
    pub /: *mut *mut u32 arg_count; / push for fixed or var args,
    pub method_nesting_depth: u16,
    pub method_is_nested: u8,
    pub /: *mut *mut acpi_namespace_node arguments[ACPI_METHOD_NUM_ARGS]; / Control method arguments,
    pub /: *mut *mut acpi_namespace_node local_variables[ACPI_METHOD_NUM_LOCALS]; / Control method locals,
    pub /: *mut *mut *mut acpi_operand_object operands[ACPI_OBJ_NUM_OPERANDS + 1]; / Operands passed to the interpreter (+1 for NULL terminator),
    pub params: *mut acpi_operand_object,
    pub aml_last_while: *mut u8,
    pub caller_return_desc: *mut acpi_operand_object,
    pub /: *mut *mut *mut acpi_generic_state control_state; / List of control states (nested IFs),
    pub /: *mut *mut *mut acpi_namespace_node deferred_node; / Used when executing deferred opcodes,
    pub implicit_return_obj: *mut acpi_operand_object,
    pub /: *mut *mut *mut acpi_namespace_node method_call_node; / Called method Node,
    pub /: *mut *mut *mut acpi_parse_object method_call_op; / method_call Op if running a method,
    pub /: *mut *mut *mut acpi_operand_object method_desc; / Method descriptor if running a method,
    pub /: *mut *mut *mut acpi_namespace_node method_node; / Method node if running a method,
    pub /: *mut *mut *mut char method_pathname; / Full pathname of running method,
    pub /: *mut *mut *mut acpi_parse_object op; / Current parser op,
    pub /: *const *const *const acpi_opcode_info op_info; / Info on current opcode,
    pub /: *mut *mut *mut acpi_parse_object origin; / Start of walk [Obsolete],
    pub result_obj: *mut acpi_operand_object,
    pub /: *mut *mut *mut acpi_generic_state results; / Stack of accumulated results,
    pub /: *mut *mut *mut acpi_operand_object return_desc; / Return object, if any,
    pub /: *mut *mut *mut acpi_generic_state scope_info; / Stack of nested scopes,
    pub /: *mut *mut *mut acpi_parse_object prev_op; / Last op that was processed,
    pub /: *mut *mut *mut acpi_parse_object next_op; / next op to be processed,
    pub thread: *mut acpi_thread_state,
    pub descending_callback: acpi_parse_downwards,
    pub ascending_callback: acpi_parse_upwards,
}

// Info used by acpi_ns_initialize_objects and acpi_ds_initialize_objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_init_walk_info {
    pub table_index: u32,
    pub object_count: u32,
    pub method_count: u32,
    pub serial_method_count: u32,
    pub non_serial_method_count: u32,
    pub serialized_method_count: u32,
    pub device_count: u32,
    pub op_region_count: u32,
    pub field_count: u32,
    pub buffer_count: u32,
    pub package_count: u32,
    pub op_region_init: u32,
    pub field_init: u32,
    pub buffer_init: u32,
    pub package_init: u32,
    pub owner_id: acpi_owner_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_get_devices_info {
    pub user_function: acpi_walk_callback,
    pub context: *mut c_void,
    pub hid: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_aml_operands {
    pub operands: [*mut acpi_operand_object; 7],
    pub type: *mut acpi_object_integer,
    pub code: *mut acpi_object_integer,
    pub argument: *mut acpi_object_integer,
    pub fatal: },
    pub source: *mut acpi_operand_object,
    pub index: *mut acpi_object_integer,
    pub target: *mut acpi_operand_object,
    pub index: },
    pub source: *mut acpi_operand_object,
    pub index: *mut acpi_object_integer,
    pub length: *mut acpi_object_integer,
    pub target: *mut acpi_operand_object,
    pub mid: },
}

//
// Structure used to pass object evaluation information and parameters.
// Purpose is to reduce CPU stack use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_evaluate_info {
// The first 3 elements are passed by the caller to acpi_ns_evaluate
    pub /: *mut *mut *mut acpi_namespace_node prefix_node; / Input: starting node,
    pub /: *const *const *const char relative_pathname; / Input: path relative to prefix_node,
    pub /: *mut *mut *mut *mut acpi_operand_object parameters; / Input: argument list,
    pub /: *mut *mut *mut acpi_namespace_node node; / Resolved node (prefix_node:relative_pathname),
    pub /: *mut *mut *mut acpi_operand_object obj_desc; / Object attached to the resolved node,
    pub /: *mut *mut *mut char full_pathname; / Full pathname of the resolved node,
    pub /: *const *const *const acpi_predefined_info predefined; / Used if Node is a predefined name,
    pub /: *mut *mut *mut acpi_operand_object return_object; / Object returned from the evaluation,
    pub /: *mut *mut *mut acpi_operand_object parent_package; / Used if return object is a Package,
    pub /: *mut *mut u32 return_flags; / Used for return value analysis,
    pub /: *mut *mut u32 return_btype; / Bitmapped type of the returned object,
    pub /: *mut *mut u16 param_count; / Count of the input argument list,
    pub /: *mut *mut u16 node_flags; / Same as Node->Flags,
    pub /: *mut *mut u8 pass_number; / Parser pass number,
    pub /: *mut *mut u8 return_object_type; / Object type of the returned object,
    pub /: *mut *mut u8 flags; / General flags,
}

// Values for Flags above
pub const ACPI_IGNORE_RETURN_VALUE: c_int = 1;
// Defines for return_flags field above
pub const ACPI_OBJECT_REPAIRED: c_int = 1;
pub const ACPI_OBJECT_WRAPPED: c_int = 2;
// Info used by acpi_ns_initialize_devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_walk_info {
    pub table_desc: *mut acpi_table_desc,
    pub evaluate_info: *mut acpi_evaluate_info,
    pub device_count: u32,
    pub num_STA: u32,
    pub num_INI: u32,
}

// Info used by Acpi  acpi_db_display_fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_region_walk_info {
    pub debug_level: u32,
    pub count: u32,
    pub owner_id: acpi_owner_id,
    pub display_type: u8,
    pub address_space_id: u32,
}

// TBD: [Restructure] Merge with struct above
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_walk_info {
    pub debug_level: u32,
    pub count: u32,
    pub owner_id: acpi_owner_id,
    pub display_type: u8,
}

// Display Types

