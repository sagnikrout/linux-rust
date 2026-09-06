//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acobject.h
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
// Name: acobject.h - Definition of union acpi_operand_object  (Internal object only)
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// acpisrc:struct_defs -- for acpisrc conversion
//
// The union acpi_operand_object is used to pass AML operands from the dispatcher
// to the interpreter, and to keep track of the various handlers such as
// address space handlers and notify handlers. The object is a constant
// size in order to allow it to be cached and reused.
//
// Note: The object is optimized to be aligned and will not work if it is
// byte-packed.
//

//
// Common Descriptors
//
// Common area for all objects.
//
// descriptor_type is used to differentiate between internal descriptors, and
// must be in the same place across all descriptors
//
// Note: The descriptor_type and Type fields must appear in the identical
// position in both the struct acpi_namespace_node and union acpi_operand_object
// structures.
//

//
// Note: There are 3 bytes available here before the
// next natural alignment boundary (for both 32/64 cases)
//
// Values for Flag byte above
pub const AOPOBJ_AML_CONSTANT: c_uint = 0x01	/* Integer is an AML constant */;
pub const AOPOBJ_STATIC_POINTER: c_uint = 0x02	/* Data is part of an ACPI table, don't delete */;
pub const AOPOBJ_DATA_VALID: c_uint = 0x04	/* Object is initialized and data is valid */;
pub const AOPOBJ_OBJECT_INITIALIZED: c_uint = 0x08	/* Region is initialized */;
pub const AOPOBJ_REG_CONNECTED: c_uint = 0x10	/* _REG was run */;
pub const AOPOBJ_SETUP_COMPLETE: c_uint = 0x20	/* Region setup is complete */;
pub const AOPOBJ_INVALID: c_uint = 0x40	/* Host OS won't allow a Region address */;
//
// Basic data types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_common {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_integer {
    pub /: *mut *mut u8 fill[3]; / Prevent warning on some compilers,
    pub value: u64,
}

//
// Note: The String and Buffer object must be identical through the
// pointer and length elements. There is code that depends on this.
//
// Fields common to both Strings and Buffers
//

// Null terminated, ASCII characters only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_string {
    pub /: *mut *mut ACPI_COMMON_BUFFER_INFO(char); / String in AML stream or allocated string,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_buffer {
    pub /: *mut *mut ACPI_COMMON_BUFFER_INFO(u8); / Buffer in AML stream or allocated buffer,
    pub aml_length: u32,
    pub aml_start: *mut u8,
    pub /: *mut *mut *mut acpi_namespace_node node; / Link back to parent node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_package {
    pub /: *mut *mut *mut acpi_namespace_node node; / Link back to parent node,
    pub /: *mut *mut *mut *mut acpi_operand_object elements; / Array of pointers to acpi_objects,
    pub aml_start: *mut u8,
    pub aml_length: u32,
    pub /: *mut *mut u32 count; / # of elements in package,
}

//
// Complex data types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_event {
    pub /: *mut *mut acpi_semaphore os_semaphore; / Actual OS synchronization object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_mutex {
    pub /: *mut *mut u8 sync_level; / 0-15, specified in Mutex() call,
    pub /: *mut *mut u16 acquisition_depth; / Allow multiple Acquires, same thread,
    pub /: *mut *mut acpi_mutex os_mutex; / Actual OS synchronization object,
    pub /: *mut *mut acpi_thread_id thread_id; / Current owner of the mutex,
    pub /: *mut *mut *mut acpi_thread_state owner_thread; / Current owner of the mutex,
    pub /: *mut *mut *mut acpi_operand_object prev; / Link for list of acquired mutexes,
    pub /: *mut *mut *mut acpi_operand_object next; / Link for list of acquired mutexes,
    pub /: *mut *mut *mut acpi_namespace_node node; / Containing namespace node,
    pub /: *mut *mut u8 original_sync_level; / Owner's original sync level (0-15),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_region {
    pub space_id: u8,
    pub /: *mut *mut *mut acpi_namespace_node node; / Containing namespace node,
    pub /: *mut *mut *mut acpi_operand_object handler; / Handler for region access,
    pub next: *mut acpi_operand_object,
    pub address: acpi_physical_address,
    pub length: u32,
    pub /: *mut *mut *mut void pointer; / Only for data table regions,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_method {
    pub info_flags: u8,
    pub param_count: u8,
    pub sync_level: u8,
    pub mutex: *mut acpi_operand_object,
    pub node: *mut acpi_operand_object,
    pub aml_start: *mut u8,
    pub implementation: acpi_internal_method,
    pub handler: *mut acpi_operand_object,
    pub dispatch: },
    pub aml_length: u32,
    pub owner_id: acpi_owner_id,
    pub thread_count: u8,
}

// Flags for info_flags field above
pub const ACPI_METHOD_MODULE_LEVEL: c_uint = 0x01	/* Method is actually module-level code */;
pub const ACPI_METHOD_INTERNAL_ONLY: c_uint = 0x02	/* Method is implemented internally (_OSI) */;
pub const ACPI_METHOD_SERIALIZED: c_uint = 0x04	/* Method is serialized */;
pub const ACPI_METHOD_SERIALIZED_PENDING: c_uint = 0x08	/* Method is to be marked serialized */;
pub const ACPI_METHOD_IGNORE_SYNC_LEVEL: c_uint = 0x10	/* Method was auto-serialized at table load time */;
pub const ACPI_METHOD_MODIFIED_NAMESPACE: c_uint = 0x20	/* Method modified the namespace */;
//
// Objects that can be notified. All share a common notify_info area.
//
// Common fields for objects that support ASL notifications
//

// COMMON NOTIFY for POWER, PROCESSOR, DEVICE, and THERMAL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_notify_common {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_device {
    pub gpe_block: *mut acpi_gpe_block_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_power_resource {
    pub system_level: u32,
    pub resource_order: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_processor {
// The next two fields take advantage of the 3-byte space before NOTIFY_INFO
    pub proc_id: u8,
    pub length: u8,
    pub address: acpi_io_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_thermal_zone {
}

//
// Fields. All share a common header/info field.
//
// Common bitfield for the field objects
// "Field Datum"  -- a datum from the actual field object
// "Buffer Datum" -- a datum from a user buffer, read from or to be written to the field
//

// COMMON FIELD (for BUFFER, REGION, BANK, and INDEX fields)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_field_common {
    pub /: *mut *mut *mut acpi_operand_object region_obj; / Parent Operation Region object (REGION/BANK fields only),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_region_field {
    pub resource_length: u16,
    pub /: *mut *mut *mut acpi_operand_object region_obj; / Containing op_region object,
    pub /: *mut *mut *mut u8 resource_buffer; / resource_template for serial regions/fields,
    pub /: *mut *mut u16 pin_number_index; / Index relative to previous Connection/Template,
    pub /: *mut *mut *mut u8 internal_pcc_buffer; / Internal buffer for fields associated with PCC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_bank_field {
    pub /: *mut *mut *mut acpi_operand_object region_obj; / Containing op_region object,
    pub /: *mut *mut *mut acpi_operand_object bank_obj; / bank_select Register object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_index_field {
//
// No "RegionObj" pointer needed since the Index and Data registers
// are each field definitions unto themselves.
//
    pub /: *mut *mut *mut acpi_operand_object index_obj; / Index register,
    pub /: *mut *mut *mut acpi_operand_object data_obj; / Data register,
}

// The buffer_field is different in that it is part of a Buffer, not an op_region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_buffer_field {
    pub /: *mut *mut u8 is_create_field; / Special case for objects created by create_field(),
    pub /: *mut *mut *mut acpi_operand_object buffer_obj; / Containing Buffer object,
}

//
// Objects for handlers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_notify_handler {
    pub /: *mut *mut *mut acpi_namespace_node node; / Parent device,
    pub /: *mut *mut u32 handler_type; / Type: Device/System/Both,
    pub /: *mut *mut acpi_notify_handler handler; / Handler address,
    pub context: *mut c_void,
    pub /: *mut *mut *mut acpi_operand_object next[2]; / Device and System handler lists,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_addr_handler {
    pub space_id: u8,
    pub handler_flags: u8,
    pub handler: acpi_adr_space_handler,
    pub /: *mut *mut *mut acpi_namespace_node node; / Parent device,
    pub context: *mut c_void,
    pub context_mutex: acpi_mutex,
    pub setup: acpi_adr_space_setup,
    pub /: *mut *mut *mut acpi_operand_object region_list; / Regions using this handler,
    pub next: *mut acpi_operand_object,
}

// Flags for address handler (handler_flags)
pub const ACPI_ADDR_HANDLER_DEFAULT_INSTALLED: c_uint = 0x01;
//
// Special internal objects
//
// The Reference object is used for these opcodes:
// Arg[0-6], Local[0-7], index_op, name_op, ref_of_op, load_op, load_table_op, debug_op
// The Reference.Class differentiates these types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_reference {
    pub /: *mut *mut u8 class; / Reference Class,
    pub /: *mut *mut u8 target_type; / Used for Index Op,
    pub /: *mut *mut u8 resolved; / Reference has been resolved to a value,
    pub /: *mut *mut *mut void object; / name_op=>HANDLE to obj, index_op=>union acpi_operand_object,
    pub /: *mut *mut *mut acpi_namespace_node node; / ref_of or Namepath,
    pub /: *mut *mut *mut *mut acpi_operand_object where; / Target of Index,
    pub /: *mut *mut *mut u8 index_pointer; / Used for Buffers and Strings,
    pub /: *mut *mut *mut u8 aml; / Used for deferred resolution of the ref,
    pub /: *mut *mut u32 value; / Used for Local/Arg/Index/ddb_handle,
}

// Values for Reference.Class above
//
// Extra object is used as additional storage for types that
// have AML code in their declarations (term_args) that must be
// evaluated at run time.
//
// Currently: Region and field_unit types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_extra {
    pub /: *mut *mut *mut acpi_namespace_node method_REG; / _REG method for this region (if any),
    pub scope_node: *mut acpi_namespace_node,
    pub /: *mut *mut *mut void region_context; / Region-specific data,
    pub aml_start: *mut u8,
    pub aml_length: u32,
}

// Additional data that can be attached to namespace nodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_data {
    pub handler: acpi_object_handler,
    pub pointer: *mut c_void,
}

// Structure used when objects are cached for reuse
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_cache_list {
    pub /: *mut *mut *mut acpi_operand_object next; / Link for object cache and internal lists,
}

//
// union acpi_operand_object descriptor - a giant union of all of the above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_operand_object {
    pub common: acpi_object_common,
    pub integer: acpi_object_integer,
    pub string: acpi_object_string,
    pub buffer: acpi_object_buffer,
    pub package: acpi_object_package,
    pub event: acpi_object_event,
    pub method: acpi_object_method,
    pub mutex: acpi_object_mutex,
    pub region: acpi_object_region,
    pub common_notify: acpi_object_notify_common,
    pub device: acpi_object_device,
    pub power_resource: acpi_object_power_resource,
    pub processor: acpi_object_processor,
    pub thermal_zone: acpi_object_thermal_zone,
    pub common_field: acpi_object_field_common,
    pub field: acpi_object_region_field,
    pub buffer_field: acpi_object_buffer_field,
    pub bank_field: acpi_object_bank_field,
    pub index_field: acpi_object_index_field,
    pub notify: acpi_object_notify_handler,
    pub address_space: acpi_object_addr_handler,
    pub reference: acpi_object_reference,
    pub extra: acpi_object_extra,
    pub data: acpi_object_data,
    pub cache: acpi_object_cache_list,
//
// Add namespace node to union in order to simplify code that accepts both
// ACPI_OPERAND_OBJECTs and ACPI_NAMESPACE_NODEs. The structures share
// a common descriptor_type field in order to differentiate them.
//
    pub node: acpi_namespace_node,
}

//
// union acpi_descriptor - objects that share a common descriptor identifier
//
// Object descriptor types
pub const ACPI_DESC_TYPE_CACHED: c_uint = 0x01	/* Used only when object is cached */;
pub const ACPI_DESC_TYPE_STATE: c_uint = 0x02;
pub const ACPI_DESC_TYPE_STATE_UPDATE: c_uint = 0x03;
pub const ACPI_DESC_TYPE_STATE_PACKAGE: c_uint = 0x04;
pub const ACPI_DESC_TYPE_STATE_CONTROL: c_uint = 0x05;
pub const ACPI_DESC_TYPE_STATE_RPSCOPE: c_uint = 0x06;
pub const ACPI_DESC_TYPE_STATE_PSCOPE: c_uint = 0x07;
pub const ACPI_DESC_TYPE_STATE_WSCOPE: c_uint = 0x08;
pub const ACPI_DESC_TYPE_STATE_RESULT: c_uint = 0x09;
pub const ACPI_DESC_TYPE_STATE_NOTIFY: c_uint = 0x0A;
pub const ACPI_DESC_TYPE_STATE_THREAD: c_uint = 0x0B;
pub const ACPI_DESC_TYPE_WALK: c_uint = 0x0C;
pub const ACPI_DESC_TYPE_PARSER: c_uint = 0x0D;
pub const ACPI_DESC_TYPE_OPERAND: c_uint = 0x0E;
pub const ACPI_DESC_TYPE_NAMED: c_uint = 0x0F;
pub const ACPI_DESC_TYPE_MAX: c_uint = 0x0F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_common_descriptor {
    pub common_pointer: *mut c_void,
    pub /: *mut *mut u8 descriptor_type; / To differentiate various internal objs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_descriptor {
    pub common: acpi_common_descriptor,
    pub object: acpi_operand_object,
    pub node: acpi_namespace_node,
    pub op: acpi_parse_object,
}

