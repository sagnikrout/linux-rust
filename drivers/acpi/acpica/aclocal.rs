//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/aclocal.h
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
// Name: aclocal.h - Internal data types used across the ACPI subsystem
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// acpisrc:struct_defs -- for acpisrc conversion
pub const ACPI_SERIALIZED: c_uint = 0xFF;
pub type acpi_mutex_handle = u32;

// Total number of aml opcodes defined
pub const AML_NUM_OPCODES: c_uint = 0x83;
// Forward declarations
//
// Mutex typedefs and structs
//
// Predefined handles for the mutex objects used within the subsystem
// All mutex objects are automatically created by acpi_ut_mutex_initialize.
//
// The acquire/release ordering protocol is implied via this list. Mutexes
// with a lower value must be acquired before mutexes with a higher value.
//
// NOTE: any changes here must be reflected in the acpi_gbl_mutex_names
// table below also!
//

pub const ACPI_MAX_MUTEX: c_int = 5;

// Lock structure for reader/writer interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_rw_lock {
    pub writer_mutex: acpi_mutex,
    pub reader_mutex: acpi_mutex,
    pub num_readers: u32,
}

//
// Predefined handles for spinlocks used within the subsystem.
// These spinlocks are created by acpi_ut_mutex_initialize
//
pub const ACPI_LOCK_GPES: c_int = 0;
pub const ACPI_LOCK_HARDWARE: c_int = 1;
pub const ACPI_MAX_LOCK: c_int = 1;

// This Thread ID means that the mutex is not in use (unlocked)

// This Thread ID means an invalid thread ID

// Table for the global mutexes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mutex_info {
    pub mutex: acpi_mutex,
    pub use_count: u32,
    pub thread_id: acpi_thread_id,
}

// Lock flag parameter for various interfaces
pub const ACPI_MTX_DO_NOT_LOCK: c_int = 0;
pub const ACPI_MTX_LOCK: c_int = 1;
// Field access granularities
pub const ACPI_FIELD_BYTE_GRANULARITY: c_int = 1;
pub const ACPI_FIELD_WORD_GRANULARITY: c_int = 2;
pub const ACPI_FIELD_DWORD_GRANULARITY: c_int = 4;
pub const ACPI_FIELD_QWORD_GRANULARITY: c_int = 8;

//
// Namespace typedefs and structs
//
// Operational modes of the AML interpreter/scanner
//
// The Namespace Node describes a named object that appears in the AML.
// descriptor_type is used to differentiate between internal descriptors.
//
// The node is optimized for both 32-bit and 64-bit platforms:
// 20 bytes for the 32-bit case, 32 bytes for the 64-bit case.
//
// Note: The descriptor_type and Type fields must appear in the identical
// position in both the struct acpi_namespace_node and union acpi_operand_object
// structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_namespace_node {
    pub /: *mut *mut *mut acpi_operand_object object; / Interpreter object,
    pub /: *mut *mut u8 descriptor_type; / Differentiate object descriptor types,
    pub /: *mut *mut u8 type; / ACPI Type associated with this name,
    pub /: *mut *mut u16 flags; / Miscellaneous flags,
    pub /: *mut *mut acpi_name_name; / ACPI Name, always 4 chars per ACPI spec,
    pub /: *mut *mut *mut acpi_namespace_node parent; / Parent node,
    pub /: *mut *mut *mut acpi_namespace_node child; / First child,
    pub /: *mut *mut *mut acpi_namespace_node peer; / First peer,
    pub /: *mut *mut acpi_owner_id owner_id; / Node creator,
//
// The following fields are used by the ASL compiler and disassembler only
//

    pub op: *mut acpi_parse_object,
    pub method_locals: *mut c_void,
    pub method_args: *mut c_void,
    pub value: u32,
    pub length: u32,
    pub arg_count: u8,

}

// Namespace Node flags
pub const ANOBJ_RESERVED: c_uint = 0x01	/* Available for use */;
pub const ANOBJ_TEMPORARY: c_uint = 0x02	/* Node is create by a method and is temporary */;
pub const ANOBJ_METHOD_ARG: c_uint = 0x04	/* Node is a method argument */;
pub const ANOBJ_METHOD_LOCAL: c_uint = 0x08	/* Node is a method local */;
pub const ANOBJ_SUBTREE_HAS_INI: c_uint = 0x10	/* Used to optimize device initialization */;
pub const ANOBJ_EVALUATED: c_uint = 0x20	/* Set on first evaluation of node */;
pub const ANOBJ_ALLOCATED_BUFFER: c_uint = 0x40	/* Method AML buffer is dynamic (install_method) */;
pub const ANOBJ_NODE_EARLY_INIT: c_uint = 0x80	/* acpi_exec only: Node was create via init file (-fi) */;
pub const ANOBJ_IS_EXTERNAL: c_uint = 0x08	/* iASL only: This object created via External() */;
pub const ANOBJ_METHOD_NO_RETVAL: c_uint = 0x10	/* iASL only: Method has no return value */;
pub const ANOBJ_METHOD_SOME_NO_RETVAL: c_uint = 0x20	/* iASL only: Method has at least one return value */;
pub const ANOBJ_IS_ALIAS: c_uint = 0x40	/* iASL only: Node is an alias to another node */;
pub const ANOBJ_IS_REFERENCED: c_uint = 0x80	/* iASL only: Object was referenced */;
// Internal ACPI table management - master table list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_table_list {
    pub /: *mut *mut *mut acpi_table_desc tables; / Table descriptor array,
    pub /: *mut *mut u32 current_table_count; / Tables currently in the array,
    pub /: *mut *mut u32 max_table_count; / Max tables array will hold,
    pub flags: u8,
}

// Flags for above

// List to manage incoming ACPI tables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_new_table_desc {
    pub table: *mut acpi_table_header,
    pub next: *mut acpi_new_table_desc,
}

// Predefined table indexes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_find_context {
    pub search_for: *mut c_char,
    pub list: *mut acpi_handle,
    pub count: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ns_search_data {
    pub node: *mut acpi_namespace_node,
}

// Object types used during package copies
pub const ACPI_COPY_TYPE_SIMPLE: c_int = 0;
pub const ACPI_COPY_TYPE_PACKAGE: c_int = 1;
// Info structure used to convert external<->internal namestrings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_namestring_info {
    pub external_name: *const c_char,
    pub next_external_char: *const c_char,
    pub internal_name: *mut c_char,
    pub length: u32,
    pub num_segments: u32,
    pub num_carats: u32,
    pub fully_qualified: u8,
}

// Field creation info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_create_field_info {
    pub region_node: *mut acpi_namespace_node,
    pub field_node: *mut acpi_namespace_node,
    pub register_node: *mut acpi_namespace_node,
    pub data_register_node: *mut acpi_namespace_node,
    pub connection_node: *mut acpi_namespace_node,
    pub resource_buffer: *mut u8,
    pub bank_value: u32,
    pub field_bit_position: u32,
    pub field_bit_length: u32,
    pub resource_length: u16,
    pub pin_number_index: u16,
    pub field_flags: u8,
    pub attribute: u8,
    pub field_type: u8,
    pub access_length: u8,
}

//
// Bitmapped ACPI types. Used internally only
//
pub const ACPI_BTYPE_ANY: c_uint = 0x00000000;
pub const ACPI_BTYPE_INTEGER: c_uint = 0x00000001;
pub const ACPI_BTYPE_STRING: c_uint = 0x00000002;
pub const ACPI_BTYPE_BUFFER: c_uint = 0x00000004;
pub const ACPI_BTYPE_PACKAGE: c_uint = 0x00000008;
pub const ACPI_BTYPE_FIELD_UNIT: c_uint = 0x00000010;
pub const ACPI_BTYPE_DEVICE: c_uint = 0x00000020;
pub const ACPI_BTYPE_EVENT: c_uint = 0x00000040;
pub const ACPI_BTYPE_METHOD: c_uint = 0x00000080;
pub const ACPI_BTYPE_MUTEX: c_uint = 0x00000100;
pub const ACPI_BTYPE_REGION: c_uint = 0x00000200;
pub const ACPI_BTYPE_POWER: c_uint = 0x00000400;
pub const ACPI_BTYPE_PROCESSOR: c_uint = 0x00000800;
pub const ACPI_BTYPE_THERMAL: c_uint = 0x00001000;
pub const ACPI_BTYPE_BUFFER_FIELD: c_uint = 0x00002000;
pub const ACPI_BTYPE_DDB_HANDLE: c_uint = 0x00004000;
pub const ACPI_BTYPE_DEBUG_OBJECT: c_uint = 0x00008000;
pub const ACPI_BTYPE_REFERENCE_OBJECT: c_uint = 0x00010000	/* From Index(), ref_of(), etc (type6_opcodes) */;
pub const ACPI_BTYPE_RESOURCE: c_uint = 0x00020000;
pub const ACPI_BTYPE_NAMED_REFERENCE: c_uint = 0x00040000	/* Generic unresolved Name or Namepath */;

// Used by Copy, de_ref_of, Store, Printf, Fprintf

pub const ACPI_BTYPE_OBJECTS_AND_REFS: c_uint = 0x0001FFFF	/* ARG or LOCAL */;
pub const ACPI_BTYPE_ALL_OBJECTS: c_uint = 0x0000FFFF;

//
// Information structure for ACPI predefined names.
// Each entry in the table contains the following items:
//
// name                 - The ACPI reserved name
// param_count          - Number of arguments to the method
// expected_return_btypes - Allowed type(s) for the return value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_name_info {
    pub ACPI_NONSTRING: char name[ACPI_NAMESEG_SIZE],
    pub argument_list: u16,
    pub expected_btypes: u8,
}

//
// Secondary information structures for ACPI predefined objects that return
// package objects. This structure appears as the next entry in the table
// after the NAME_INFO structure above.
//
// The reason for this is to minimize the size of the predefined name table.
//
// Used for ACPI_PTYPE1_FIXED, ACPI_PTYPE1_VAR, ACPI_PTYPE2,
// ACPI_PTYPE2_MIN, ACPI_PTYPE2_PKG_COUNT, ACPI_PTYPE2_COUNT,
// ACPI_PTYPE2_FIX_VAR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_package_info {
    pub type: u8,
    pub object_type1: u8,
    pub count1: u8,
    pub object_type2: u8,
    pub count2: u8,
    pub reserved: u16,
}

// Used for ACPI_PTYPE2_FIXED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_package_info2 {
    pub type: u8,
    pub count: u8,
    pub object_type: [u8; 4],
    pub reserved: u8,
}

// Used for ACPI_PTYPE1_OPTION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_package_info3 {
    pub type: u8,
    pub count: u8,
    pub object_type: [u8; 2],
    pub tail_object_type: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_package_info4 {
    pub type: u8,
    pub object_type1: u8,
    pub count1: u8,
    pub sub_object_types: u8,
    pub pkg_count: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_predefined_info {
    pub info: acpi_name_info,
    pub ret_info: acpi_package_info,
    pub ret_info2: acpi_package_info2,
    pub ret_info3: acpi_package_info3,
    pub ret_info4: acpi_package_info4,
}

// Reset to default packing

// Return object auto-repair info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_simple_repair_info {
    pub ACPI_NONSTRING: char name[ACPI_NAMESEG_SIZE],
    pub unexpected_btypes: u32,
    pub package_index: u32,
    pub object_converter: acpi_object_converter,
}

//
// Bitmapped return value types
// Note: the actual data types must be contiguous, a loop in nspredef.c
// depends on this.
//
pub const ACPI_RTYPE_ANY: c_uint = 0x00;
pub const ACPI_RTYPE_NONE: c_uint = 0x01;
pub const ACPI_RTYPE_INTEGER: c_uint = 0x02;
pub const ACPI_RTYPE_STRING: c_uint = 0x04;
pub const ACPI_RTYPE_BUFFER: c_uint = 0x08;
pub const ACPI_RTYPE_PACKAGE: c_uint = 0x10;
pub const ACPI_RTYPE_REFERENCE: c_uint = 0x20;
pub const ACPI_RTYPE_ALL: c_uint = 0x3F;

// Info for running the _REG methods
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_reg_walk_info {
    pub function: u32,
    pub reg_run_count: u32,
    pub space_id: acpi_adr_space_type,
}

//
// Event typedefs and structs
//
// Dispatch info for each host-installed SCI handler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sci_handler_info {
    pub next: *mut acpi_sci_handler_info,
    pub /: *mut *mut acpi_sci_handler address; / Address of handler,
    pub /: *mut *mut *mut void context; / Context to be passed to handler,
}

// Dispatch info for each GPE -- either a method or handler, cannot be both
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_handler_info {
    pub /: *mut *mut acpi_gpe_handler address; / Address of handler, if any,
    pub /: *mut *mut *mut void context; / Context to be passed to handler,
    pub /: *mut *mut *mut acpi_namespace_node method_node; / Method node for this GPE level (saved),
    pub /: *mut *mut u8 original_flags; / Original (pre-handler) GPE info,
    pub /: *mut *mut u8 originally_enabled; / True if GPE was originally enabled,
}

// Notify info for implicit notify, multiple device objects
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_notify_info {
    pub /: *mut *mut *mut acpi_namespace_node device_node; / Device to be notified,
    pub next: *mut acpi_gpe_notify_info,
}

//
// GPE dispatch info. At any time, the GPE can have at most one type
// of dispatch - Method, Handler, or Implicit Notify.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_gpe_dispatch_info {
    pub /: *mut *mut *mut acpi_namespace_node method_node; / Method node for this GPE level,
    pub /: *mut *mut *mut acpi_gpe_handler_info handler; / Installed GPE handler,
    pub /: *mut *mut *mut acpi_gpe_notify_info notify_list; / List of _PRW devices for implicit notifies,
}

//
// Information about a GPE, one per each GPE in an array.
// NOTE: Important to keep this struct as small as possible.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_event_info {
    pub /: *mut *mut acpi_gpe_dispatch_info dispatch; / Either Method, Handler, or notify_list,
    pub /: *mut *mut *mut acpi_gpe_register_info register_info; / Backpointer to register info,
    pub /: *mut *mut u8 flags; / Misc info about this GPE,
    pub /: *mut *mut u8 gpe_number; / This GPE,
    pub /: *mut *mut u8 runtime_count; / References to a run GPE,
    pub /: *mut *mut u8 disable_for_dispatch; / Masked during dispatching,
}

// GPE register address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_address {
    pub /: *mut *mut u8 space_id; / Address space where the register exists,
    pub /: *mut *mut u64 address; / 64-bit address of the register,
}

// Information about a GPE register pair, one per each status/enable pair in an array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_register_info {
    pub /: *mut *mut acpi_gpe_address status_address; / Address of status reg,
    pub /: *mut *mut acpi_gpe_address enable_address; / Address of enable reg,
    pub /: *mut *mut u16 base_gpe_number; / Base GPE number for this register,
    pub /: *mut *mut u8 enable_for_wake; / GPEs to keep enabled when sleeping,
    pub /: *mut *mut u8 enable_for_run; / GPEs to keep enabled when running,
    pub /: *mut *mut u8 mask_for_run; / GPEs to keep masked when running,
    pub /: *mut *mut u8 enable_mask; / Current mask of enabled GPEs,
}

//
// Information about a GPE register block, one per each installed block --
// GPE0, GPE1, and one per each installed GPE Block Device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_block_info {
    pub node: *mut acpi_namespace_node,
    pub previous: *mut acpi_gpe_block_info,
    pub next: *mut acpi_gpe_block_info,
    pub /: *mut *mut *mut acpi_gpe_xrupt_info xrupt_block; / Backpointer to interrupt block,
    pub /: *mut *mut *mut acpi_gpe_register_info register_info; / One per GPE register pair,
    pub /: *mut *mut *mut acpi_gpe_event_info event_info; / One for each GPE,
    pub /: *mut *mut u64 address; / Base address of the block,
    pub /: *mut *mut u32 register_count; / Number of register pairs in block,
    pub /: *mut *mut u16 gpe_count; / Number of individual GPEs in block,
    pub /: *mut *mut u16 block_base_number; / Base GPE number for this block,
    pub space_id: u8,
    pub /: *mut *mut u8 initialized; / TRUE if this block is initialized,
}

// Information about GPE interrupt handlers, one per each interrupt level used for GPEs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_xrupt_info {
    pub previous: *mut acpi_gpe_xrupt_info,
    pub next: *mut acpi_gpe_xrupt_info,
    pub /: *mut *mut *mut acpi_gpe_block_info gpe_block_list_head; / List of GPE blocks for this xrupt,
    pub /: *mut *mut u32 interrupt_number; / System interrupt number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_walk_info {
    pub gpe_device: *mut acpi_namespace_node,
    pub gpe_block: *mut acpi_gpe_block_info,
    pub count: u16,
    pub owner_id: acpi_owner_id,
    pub execute_by_owner_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpe_device_info {
    pub index: u32,
    pub next_block_base_index: u32,
    pub status: acpi_status,
    pub gpe_device: *mut acpi_namespace_node,
}

// Information about each particular fixed event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fixed_event_handler {
    pub /: *mut *mut acpi_event_handler handler; / Address of handler.,
    pub /: *mut *mut *mut void context; / Context to be passed to handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fixed_event_info {
    pub status_register_id: u8,
    pub enable_register_id: u8,
    pub status_bit_mask: u16,
    pub enable_bit_mask: u16,
}

// Information used during field processing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_field_info {
    pub skip_field: u8,
    pub field_flag: u8,
    pub pkg_length: u32,
}

// Information about the interrupt ID and _EVT of a GED device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ged_handler_info {
    pub next: *mut acpi_ged_handler_info,
    pub /: *mut *mut u32 int_id; / The interrupt ID that triggers the execution of the evt_method.,
    pub /: *mut *mut *mut acpi_namespace_node evt_method; / The _EVT method to be executed when an interrupt with ID = int_ID is received,
}

//
// Generic "state" object for stacks
//
pub const ACPI_CONTROL_NORMAL: c_uint = 0xC0;
pub const ACPI_CONTROL_CONDITIONAL_EXECUTING: c_uint = 0xC1;
pub const ACPI_CONTROL_PREDICATE_EXECUTING: c_uint = 0xC2;
pub const ACPI_CONTROL_PREDICATE_FALSE: c_uint = 0xC3;
pub const ACPI_CONTROL_PREDICATE_TRUE: c_uint = 0xC4;

// There are 2 bytes available here until the next natural alignment boundary
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_common_state {
}

//
// Update state - used to traverse complex objects such as packages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_update_state {
    pub object: *mut acpi_operand_object,
}

//
// Pkg state - used to traverse nested package structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pkg_state {
    pub index: u32,
    pub source_object: *mut acpi_operand_object,
    pub dest_object: *mut acpi_operand_object,
    pub walk_state: *mut acpi_walk_state,
    pub this_target_obj: *mut c_void,
    pub num_packages: u32,
}

//
// Control state - one per if/else and while constructs.
// Allows nesting of these constructs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_control_state {
    pub opcode: u16,
    pub predicate_op: *mut acpi_parse_object,
    pub /: *mut *mut *mut u8 aml_predicate_start; / Start of if/while predicate,
    pub /: *mut *mut *mut u8 package_end; / End of if/while block,
    pub /: *mut *mut u64 loop_timeout; / While() loop timeout,
}

//
// Scope state - current scope during namespace lookups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_scope_state {
    pub node: *mut acpi_namespace_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pscope_state {
    pub /: *mut *mut u32 arg_count; / Number of fixed arguments,
    pub /: *mut *mut *mut acpi_parse_object op; / Current op being parsed,
    pub /: *mut *mut *mut u8 arg_end; / Current argument end,
    pub /: *mut *mut *mut u8 pkg_end; / Current package end,
    pub /: *mut *mut u32 arg_list; / Next argument to parse,
}

//
// Thread state - one per thread across multiple walk states. Multiple walk
// states are created when there are nested control methods executing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_thread_state {
    pub /: *mut *mut u8 current_sync_level; / Mutex Sync (nested acquire) level,
    pub /: *mut *mut *mut acpi_walk_state walk_state_list; / Head of list of walk_states for this thread,
    pub /: *mut *mut *mut acpi_operand_object acquired_mutex_list; / List of all currently acquired mutexes,
    pub /: *mut *mut acpi_thread_id thread_id; / Running thread ID,
}

//
// Result values - used to accumulate the results of nested
// AML arguments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_result_values {
    pub obj_desc: [*mut acpi_operand_object; ACPI_RESULTS_FRAME_OBJ_NUM],
}

// Global handlers for AML Notifies
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_global_notify_handler {
    pub handler: acpi_notify_handler,
    pub context: *mut c_void,
}

//
// Notify info - used to pass info to the deferred notify
// handler/dispatcher.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_notify_info {
    pub handler_list_id: u8,
    pub node: *mut acpi_namespace_node,
    pub handler_list_head: *mut acpi_operand_object,
    pub global: *mut acpi_global_notify_handler,
}

// Generic state is union of structs above
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_generic_state {
    pub common: acpi_common_state,
    pub control: acpi_control_state,
    pub update: acpi_update_state,
    pub scope: acpi_scope_state,
    pub parse_scope: acpi_pscope_state,
    pub pkg: acpi_pkg_state,
    pub thread: acpi_thread_state,
    pub results: acpi_result_values,
    pub notify: acpi_notify_info,
}

//
// Interpreter typedefs and structs
//
// Address Range info block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_address_range {
    pub next: *mut acpi_address_range,
    pub region_node: *mut acpi_namespace_node,
    pub start_address: acpi_physical_address,
    pub end_address: acpi_physical_address,
}

//
// Parser typedefs and structs
//
// AML opcode, name, and argument layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_opcode_info {

    pub /: *mut *mut *mut char name; / Opcode name (disassembler/debug only),

    pub /: *mut *mut u32 parse_args; / Grammar/Parse time arguments,
    pub /: *mut *mut u32 runtime_args; / Interpret time arguments,
    pub /: *mut *mut u16 flags; / Misc flags,
    pub /: *mut *mut u8 object_type; / Corresponding internal object type,
    pub /: *mut *mut u8 class; / Opcode class,
    pub /: *mut *mut u8 type; / Opcode type,
}

// Value associated with the parse object
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_parse_value {
    pub /: *mut *mut u64 integer; / Integer constant (Up to 64 bits),
    pub /: *mut *mut u32 size; / bytelist or field size,
    pub /: *mut *mut *mut char string; / NULL terminated string,
    pub /: *mut *mut *mut u8 buffer; / buffer or string,
    pub /: *mut *mut *mut char name; / NULL terminated string,
    pub /: *mut *mut *mut acpi_parse_object arg; / arguments and contained ops,
}

// Macro flag: #define ACPI_DISASM_ONLY_MEMBERS(a)

// Macro flag: #define ACPI_CONVERTER_ONLY_MEMBERS(a)

// categories of comments
// Internal opcodes for disasm_opcode field above
pub const ACPI_DASM_BUFFER: c_uint = 0x00	/* Buffer is a simple data buffer */;
pub const ACPI_DASM_RESOURCE: c_uint = 0x01	/* Buffer is a Resource Descriptor */;
pub const ACPI_DASM_STRING: c_uint = 0x02	/* Buffer is a ASCII string */;
pub const ACPI_DASM_UNICODE: c_uint = 0x03	/* Buffer is a Unicode string */;
pub const ACPI_DASM_PLD_METHOD: c_uint = 0x04	/* Buffer is a _PLD method bit-packed buffer */;
pub const ACPI_DASM_UUID: c_uint = 0x05	/* Buffer is a UUID/GUID */;
pub const ACPI_DASM_EISAID: c_uint = 0x06	/* Integer is an EISAID */;
pub const ACPI_DASM_MATCHOP: c_uint = 0x07	/* Parent opcode is a Match() operator */;
pub const ACPI_DASM_LNOT_PREFIX: c_uint = 0x08	/* Start of a Lnot_equal (etc.) pair of opcodes */;
pub const ACPI_DASM_LNOT_SUFFIX: c_uint = 0x09	/* End  of a Lnot_equal (etc.) pair of opcodes */;
pub const ACPI_DASM_HID_STRING: c_uint = 0x0A	/* String is a _HID or _CID */;
pub const ACPI_DASM_IGNORE_SINGLE: c_uint = 0x0B	/* Ignore the opcode but not it's children */;
pub const ACPI_DASM_SWITCH: c_uint = 0x0C	/* While is a Switch */;
pub const ACPI_DASM_SWITCH_PREDICATE: c_uint = 0x0D	/* Object is a predicate for a Switch or Case block */;
pub const ACPI_DASM_CASE: c_uint = 0x0E	/* If/Else is a Case in a Switch/Case block */;
pub const ACPI_DASM_DEFAULT: c_uint = 0x0F	/* Else is a Default in a Switch/Case block */;
//
// List struct used in the -ca option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_comment_node {
    pub comment: *mut c_char,
    pub next: *mut acpi_comment_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_comment_addr_node {
    pub addr: *mut u8,
    pub next: *mut acpi_comment_addr_node,
}

//
// File node - used for "Include" operator file stack and
// dependency tree for the -ca option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_file_node {
    pub file: *mut c_void,
    pub filename: *mut c_char,
    pub /: *mut *mut *mut char file_start; / Points to AML and indicates when the AML for this particular file starts.,
    pub /: *mut *mut *mut char file_end; / Points to AML and indicates when the AML for this particular file ends.,
    pub next: *mut acpi_file_node,
    pub parent: *mut acpi_file_node,
    pub include_written: u8,
    pub include_comment: *mut acpi_comment_node,
}

//
// Generic operation (for example:  If, While, Store)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_parse_obj_common {
//
// Extended Op for named ops (Scope, Method, etc.), deferred ops (Methods and op_regions),
// and bytelists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_parse_obj_named {
    pub path: *mut ACPI_PARSE_COMMON char,
    pub /: *mut *mut *mut u8 data; / AML body or bytelist data,
    pub /: *mut *mut u32 length; / AML length,
    pub /: *mut *mut u32 name; / 4-byte name or zero if no name,
}

// This version is used by the iASL compiler only
pub const ACPI_MAX_PARSEOP_NAME: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_parse_obj_asl {
    pub child: *mut ACPI_PARSE_COMMON union acpi_parse_object,
    pub parent_method: *mut acpi_parse_object,
    pub filename: *mut c_char,
    pub file_changed: u8,
    pub parent_filename: *mut c_char,
    pub external_name: *mut c_char,
    pub namepath: *mut c_char,
    pub name_seg: [c_char; 4],
    pub extra_value: u32,
    pub column: u32,
    pub line_number: u32,
    pub logical_line_number: u32,
    pub logical_byte_offset: u32,
    pub end_line: u32,
    pub end_logical_line: u32,
    pub acpi_btype: u32,
    pub aml_length: u32,
    pub aml_subtree_length: u32,
    pub final_aml_length: u32,
    pub final_aml_offset: u32,
    pub compile_flags: u32,
    pub parse_opcode: u16,
    pub aml_opcode_length: u8,
    pub aml_pkg_len_bytes: u8,
    pub extra: u8,
    pub parse_op_name: [c_char; ACPI_MAX_PARSEOP_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_parse_object {
    pub common: acpi_parse_obj_common,
    pub named: acpi_parse_obj_named,
    pub asl: acpi_parse_obj_asl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asl_comment_state {
    pub comment_type: u8,
    pub spaces_before: u32,
    pub latest_parse_op: *mut acpi_parse_object,
    pub parsing_paren_brace_node: *mut acpi_parse_object,
    pub capture_comments: u8,
}

//
// Parse state - one state per parser invocation and each control
// method.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_parse_state {
    pub /: *mut *mut *mut u8 aml_start; / First AML byte,
    pub /: *mut *mut *mut u8 aml; / Next AML byte,
    pub /: *mut *mut *mut u8 aml_end; / (last + 1) AML byte,
    pub /: *mut *mut *mut u8 pkg_start; / Current package begin,
    pub /: *mut *mut *mut u8 pkg_end; / Current package end,
    pub /: *mut *mut *mut acpi_parse_object start_op; / Root of parse tree,
    pub start_node: *mut acpi_namespace_node,
    pub /: *mut *mut *mut acpi_generic_state scope; / Current scope,
    pub start_scope: *mut acpi_parse_object,
    pub aml_size: u32,
}

// Parse object flags
pub const ACPI_PARSEOP_GENERIC: c_uint = 0x01;
pub const ACPI_PARSEOP_NAMED_OBJECT: c_uint = 0x02;
pub const ACPI_PARSEOP_DEFERRED: c_uint = 0x04;
pub const ACPI_PARSEOP_BYTELIST: c_uint = 0x08;
pub const ACPI_PARSEOP_IN_STACK: c_uint = 0x10;
pub const ACPI_PARSEOP_TARGET: c_uint = 0x20;
pub const ACPI_PARSEOP_IN_CACHE: c_uint = 0x80;
// Parse object disasm_flags
pub const ACPI_PARSEOP_IGNORE: c_uint = 0x0001;
pub const ACPI_PARSEOP_PARAMETER_LIST: c_uint = 0x0002;
pub const ACPI_PARSEOP_EMPTY_TERMLIST: c_uint = 0x0004;
pub const ACPI_PARSEOP_PREDEFINED_CHECKED: c_uint = 0x0008;
pub const ACPI_PARSEOP_CLOSING_PAREN: c_uint = 0x0010;
pub const ACPI_PARSEOP_COMPOUND_ASSIGNMENT: c_uint = 0x0020;
pub const ACPI_PARSEOP_ASSIGNMENT: c_uint = 0x0040;
pub const ACPI_PARSEOP_ELSEIF: c_uint = 0x0080;
pub const ACPI_PARSEOP_LEGACY_ASL_ONLY: c_uint = 0x0100;
//
// Hardware (ACPI registers) and PNP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_bit_register_info {
    pub parent_register: u8,
    pub bit_position: u8,
    pub access_bit_mask: u16,
}

//
// Some ACPI registers have bits that must be ignored -- meaning that they
// must be preserved.
//
pub const ACPI_PM1_STATUS_PRESERVED_BITS: c_uint = 0x0800	/* Bit 11 */;
// Write-only bits must be zeroed by software
pub const ACPI_PM1_CONTROL_WRITEONLY_BITS: c_uint = 0x2004	/* Bits 13, 2 */;
// For control registers, both ignored and reserved bits must be preserved
//
// For PM1 control, the SCI enable bit (bit 0, SCI_EN) is defined by the
// ACPI specification to be a "preserved" bit - "OSPM always preserves this
// bit position", section 4.7.3.2.1. However, on some machines the OS must
// write a one to this bit after resume for the machine to work properly.
// To enable this, we no longer attempt to preserve this bit. No machines
// are known to fail if the bit is not preserved. (May 2009)
//
pub const ACPI_PM1_CONTROL_IGNORED_BITS: c_uint = 0x0200	/* Bit 9 */;
pub const ACPI_PM1_CONTROL_RESERVED_BITS: c_uint = 0xC1F8	/* Bits 14-15, 3-8 */;

pub const ACPI_PM2_CONTROL_PRESERVED_BITS: c_uint = 0xFFFFFFFE	/* All except bit 0 */;
//
// Register IDs
// These are the full ACPI registers
//
pub const ACPI_REGISTER_PM1_STATUS: c_uint = 0x01;
pub const ACPI_REGISTER_PM1_ENABLE: c_uint = 0x02;
pub const ACPI_REGISTER_PM1_CONTROL: c_uint = 0x03;
pub const ACPI_REGISTER_PM2_CONTROL: c_uint = 0x04;
pub const ACPI_REGISTER_PM_TIMER: c_uint = 0x05;
pub const ACPI_REGISTER_PROCESSOR_BLOCK: c_uint = 0x06;
pub const ACPI_REGISTER_SMI_COMMAND_BLOCK: c_uint = 0x07;
// Masks used to access the bit_registers
pub const ACPI_BITMASK_TIMER_STATUS: c_uint = 0x0001;
pub const ACPI_BITMASK_BUS_MASTER_STATUS: c_uint = 0x0010;
pub const ACPI_BITMASK_GLOBAL_LOCK_STATUS: c_uint = 0x0020;
pub const ACPI_BITMASK_POWER_BUTTON_STATUS: c_uint = 0x0100;
pub const ACPI_BITMASK_SLEEP_BUTTON_STATUS: c_uint = 0x0200;
pub const ACPI_BITMASK_RT_CLOCK_STATUS: c_uint = 0x0400;
pub const ACPI_BITMASK_PCIEXP_WAKE_STATUS: c_uint = 0x4000	/* ACPI 3.0 */;
pub const ACPI_BITMASK_WAKE_STATUS: c_uint = 0x8000;

pub const ACPI_BITMASK_TIMER_ENABLE: c_uint = 0x0001;
pub const ACPI_BITMASK_GLOBAL_LOCK_ENABLE: c_uint = 0x0020;
pub const ACPI_BITMASK_POWER_BUTTON_ENABLE: c_uint = 0x0100;
pub const ACPI_BITMASK_SLEEP_BUTTON_ENABLE: c_uint = 0x0200;
pub const ACPI_BITMASK_RT_CLOCK_ENABLE: c_uint = 0x0400;
pub const ACPI_BITMASK_PCIEXP_WAKE_DISABLE: c_uint = 0x4000	/* ACPI 3.0 */;
pub const ACPI_BITMASK_SCI_ENABLE: c_uint = 0x0001;
pub const ACPI_BITMASK_BUS_MASTER_RLD: c_uint = 0x0002;
pub const ACPI_BITMASK_GLOBAL_LOCK_RELEASE: c_uint = 0x0004;
pub const ACPI_BITMASK_SLEEP_TYPE: c_uint = 0x1C00;
pub const ACPI_BITMASK_SLEEP_ENABLE: c_uint = 0x2000;
pub const ACPI_BITMASK_ARB_DISABLE: c_uint = 0x0001;
// Raw bit position of each bit_register
pub const ACPI_BITPOSITION_TIMER_STATUS: c_uint = 0x00;
pub const ACPI_BITPOSITION_BUS_MASTER_STATUS: c_uint = 0x04;
pub const ACPI_BITPOSITION_GLOBAL_LOCK_STATUS: c_uint = 0x05;
pub const ACPI_BITPOSITION_POWER_BUTTON_STATUS: c_uint = 0x08;
pub const ACPI_BITPOSITION_SLEEP_BUTTON_STATUS: c_uint = 0x09;
pub const ACPI_BITPOSITION_RT_CLOCK_STATUS: c_uint = 0x0A;
pub const ACPI_BITPOSITION_PCIEXP_WAKE_STATUS: c_uint = 0x0E	/* ACPI 3.0 */;
pub const ACPI_BITPOSITION_WAKE_STATUS: c_uint = 0x0F;
pub const ACPI_BITPOSITION_TIMER_ENABLE: c_uint = 0x00;
pub const ACPI_BITPOSITION_GLOBAL_LOCK_ENABLE: c_uint = 0x05;
pub const ACPI_BITPOSITION_POWER_BUTTON_ENABLE: c_uint = 0x08;
pub const ACPI_BITPOSITION_SLEEP_BUTTON_ENABLE: c_uint = 0x09;
pub const ACPI_BITPOSITION_RT_CLOCK_ENABLE: c_uint = 0x0A;
pub const ACPI_BITPOSITION_PCIEXP_WAKE_DISABLE: c_uint = 0x0E	/* ACPI 3.0 */;
pub const ACPI_BITPOSITION_SCI_ENABLE: c_uint = 0x00;
pub const ACPI_BITPOSITION_BUS_MASTER_RLD: c_uint = 0x01;
pub const ACPI_BITPOSITION_GLOBAL_LOCK_RELEASE: c_uint = 0x02;
pub const ACPI_BITPOSITION_SLEEP_TYPE: c_uint = 0x0A;
pub const ACPI_BITPOSITION_SLEEP_ENABLE: c_uint = 0x0D;
pub const ACPI_BITPOSITION_ARB_DISABLE: c_uint = 0x00;
// Structs and definitions for _OSI support and I/O port validation
pub const ACPI_ALWAYS_ILLEGAL: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_interface_info {
    pub name: *mut c_char,
    pub next: *mut acpi_interface_info,
    pub flags: u8,
    pub value: u8,
}

pub const ACPI_OSI_INVALID: c_uint = 0x01;
pub const ACPI_OSI_DYNAMIC: c_uint = 0x02;
pub const ACPI_OSI_FEATURE: c_uint = 0x04;
pub const ACPI_OSI_DEFAULT_INVALID: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_port_info {
    pub name: *mut c_char,
    pub start: u16,
    pub end: u16,
    pub osi_dependency: u8,
}

//
// Resource descriptors
//
// resource_type values
pub const ACPI_ADDRESS_TYPE_MEMORY_RANGE: c_int = 0;
pub const ACPI_ADDRESS_TYPE_IO_RANGE: c_int = 1;
pub const ACPI_ADDRESS_TYPE_BUS_NUMBER_RANGE: c_int = 2;
pub const ACPI_ADDRESS_TYPE_PCC_NUMBER: c_uint = 0xA;
// Resource descriptor types and masks
pub const ACPI_RESOURCE_NAME_LARGE: c_uint = 0x80;
pub const ACPI_RESOURCE_NAME_SMALL: c_uint = 0x00;
pub const ACPI_RESOURCE_NAME_SMALL_MASK: c_uint = 0x78	/* Bits 6:3 contain the type */;
pub const ACPI_RESOURCE_NAME_SMALL_LENGTH_MASK: c_uint = 0x07	/* Bits 2:0 contain the length */;
pub const ACPI_RESOURCE_NAME_LARGE_MASK: c_uint = 0x7F	/* Bits 6:0 contain the type */;
//
// Small resource descriptor "names" as defined by the ACPI specification.
// Note: Bits 2:0 are used for the descriptor length
//
pub const ACPI_RESOURCE_NAME_IRQ: c_uint = 0x20;
pub const ACPI_RESOURCE_NAME_DMA: c_uint = 0x28;
pub const ACPI_RESOURCE_NAME_START_DEPENDENT: c_uint = 0x30;
pub const ACPI_RESOURCE_NAME_END_DEPENDENT: c_uint = 0x38;
pub const ACPI_RESOURCE_NAME_IO: c_uint = 0x40;
pub const ACPI_RESOURCE_NAME_FIXED_IO: c_uint = 0x48;
pub const ACPI_RESOURCE_NAME_FIXED_DMA: c_uint = 0x50;
pub const ACPI_RESOURCE_NAME_RESERVED_S2: c_uint = 0x58;
pub const ACPI_RESOURCE_NAME_RESERVED_S3: c_uint = 0x60;
pub const ACPI_RESOURCE_NAME_RESERVED_S4: c_uint = 0x68;
pub const ACPI_RESOURCE_NAME_VENDOR_SMALL: c_uint = 0x70;
pub const ACPI_RESOURCE_NAME_END_TAG: c_uint = 0x78;
//
// Large resource descriptor "names" as defined by the ACPI specification.
// Note: includes the Large Descriptor bit in bit[7]
//
pub const ACPI_RESOURCE_NAME_MEMORY24: c_uint = 0x81;
pub const ACPI_RESOURCE_NAME_GENERIC_REGISTER: c_uint = 0x82;
pub const ACPI_RESOURCE_NAME_RESERVED_L1: c_uint = 0x83;
pub const ACPI_RESOURCE_NAME_VENDOR_LARGE: c_uint = 0x84;
pub const ACPI_RESOURCE_NAME_MEMORY32: c_uint = 0x85;
pub const ACPI_RESOURCE_NAME_FIXED_MEMORY32: c_uint = 0x86;
pub const ACPI_RESOURCE_NAME_ADDRESS32: c_uint = 0x87;
pub const ACPI_RESOURCE_NAME_ADDRESS16: c_uint = 0x88;
pub const ACPI_RESOURCE_NAME_EXTENDED_IRQ: c_uint = 0x89;
pub const ACPI_RESOURCE_NAME_ADDRESS64: c_uint = 0x8A;
pub const ACPI_RESOURCE_NAME_EXTENDED_ADDRESS64: c_uint = 0x8B;
pub const ACPI_RESOURCE_NAME_GPIO: c_uint = 0x8C;
pub const ACPI_RESOURCE_NAME_PIN_FUNCTION: c_uint = 0x8D;
pub const ACPI_RESOURCE_NAME_SERIAL_BUS: c_uint = 0x8E;
pub const ACPI_RESOURCE_NAME_PIN_CONFIG: c_uint = 0x8F;
pub const ACPI_RESOURCE_NAME_PIN_GROUP: c_uint = 0x90;
pub const ACPI_RESOURCE_NAME_PIN_GROUP_FUNCTION: c_uint = 0x91;
pub const ACPI_RESOURCE_NAME_PIN_GROUP_CONFIG: c_uint = 0x92;
pub const ACPI_RESOURCE_NAME_CLOCK_INPUT: c_uint = 0x93;
pub const ACPI_RESOURCE_NAME_LARGE_MAX: c_uint = 0x93;
//
// Miscellaneous
//
pub const ACPI_ASCII_ZERO: c_uint = 0x30;
//
// Disassembler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_external_list {
    pub path: *mut c_char,
    pub internal_path: *mut c_char,
    pub next: *mut acpi_external_list,
    pub value: u32,
    pub length: u16,
    pub flags: u16,
    pub type: u8,
}

// Values for Flags field above
pub const ACPI_EXT_RESOLVED_REFERENCE: c_uint = 0x01	/* Object was resolved during cross ref */;
pub const ACPI_EXT_ORIGIN_FROM_FILE: c_uint = 0x02	/* External came from a file */;
pub const ACPI_EXT_INTERNAL_PATH_ALLOCATED: c_uint = 0x04	/* Deallocate internal path on completion */;
pub const ACPI_EXT_EXTERNAL_EMITTED: c_uint = 0x08	/* External() statement has been emitted */;
pub const ACPI_EXT_ORIGIN_FROM_OPCODE: c_uint = 0x10	/* External came from a External() opcode */;
pub const ACPI_EXT_CONFLICTING_DECLARATION: c_uint = 0x20	/* External has a conflicting declaration within AML */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_external_file {
    pub path: *mut c_char,
    pub next: *mut acpi_external_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_parse_object_list {
    pub op: *mut acpi_parse_object,
    pub next: *mut acpi_parse_object_list,
}

//
// Debugger
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_db_method_info {
    pub method: acpi_handle,
    pub main_thread_gate: acpi_handle,
    pub thread_complete_gate: acpi_handle,
    pub info_gate: acpi_handle,
    pub threads: *mut acpi_thread_id,
    pub num_threads: u32,
    pub num_created: u32,
    pub num_completed: u32,
    pub name: *mut c_char,
    pub flags: u32,
    pub num_loops: u32,
    pub pathname: [c_char; ACPI_DB_LINE_BUFFER_SIZE],
    pub args: *mut c_char,
    pub types: *mut acpi_object_type,
//
// Arguments to be passed to method for the commands Threads and
// Background. Note, ACPI specifies a maximum of 7 arguments (0 - 6).
//
// For the Threads command, the Number of threads, ID of current
// thread and Index of current thread inside all them created.
//
    pub init_args: c_char,
    pub arg_types: [acpi_object_type; ACPI_METHOD_NUM_ARGS],    pub arguments: [*mut c_char; ACPI_METHOD_NUM_ARGS],
    pub num_threads_str: [c_char; 11],
    pub id_of_thread_str: [c_char; 11],
    pub index_of_thread_str: [c_char; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_integrity_info {
    pub nodes: u32,
    pub objects: u32,
}

pub const ACPI_DB_DISABLE_OUTPUT: c_uint = 0x00;
pub const ACPI_DB_REDIRECTABLE_OUTPUT: c_uint = 0x01;
pub const ACPI_DB_CONSOLE_OUTPUT: c_uint = 0x02;
pub const ACPI_DB_DUPLICATE_OUTPUT: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_info {
    pub types: [u32; ACPI_TOTAL_TYPES],
}

//
// Debug
//
// Entry for a memory allocation (debug only)
pub const ACPI_MEM_MALLOC: c_int = 0;
pub const ACPI_MEM_CALLOC: c_int = 1;
pub const ACPI_MAX_MODULE_NAME: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_debug_mem_header {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_debug_mem_block {
    pub user_space: ACPI_COMMON_DEBUG_MEM_HEADER u64,
}

pub const ACPI_MEM_LIST_GLOBAL: c_int = 0;
pub const ACPI_MEM_LIST_NSNODE: c_int = 1;
pub const ACPI_MEM_LIST_MAX: c_int = 1;
pub const ACPI_NUM_MEM_LISTS: c_int = 2;
//
// Info/help support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ah_predefined_name {
    pub name: *mut c_char,
    pub description: *mut c_char,

    pub action: *mut c_char,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ah_device_id {
    pub name: *mut c_char,
    pub description: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ah_uuid {
    pub description: *mut c_char,
    pub string: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ah_table {
    pub signature: *mut c_char,
    pub description: *mut c_char,
}
