//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/amlcode.h
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
// Name: amlcode.h - Definitions for AML, as included in "definition blocks"
// Declarations and definitions contained herein are derived
// directly from the ACPI specification.
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// primary opcodes

//
// Combination opcodes (actually two one-byte opcodes)
// Used by the disassembler and iASL compiler
//

// Prefixed (2-byte) opcodes (with AML_EXTENDED_PREFIX)

//
// Opcodes for "Field" operators
//

//
// Internal opcodes
// Use only "Unknown" AML opcodes, don't attempt to use
// any valid ACPI ASCII values (A-Z, 0-9, '-')
//

pub const ARG_NONE: c_uint = 0x0;
//
// Argument types for the AML Parser
// Each field in the arg_types u32 is 5 bits, allowing for a maximum of 6 arguments.
// There can be up to 31 unique argument types
// Zero is reserved as end-of-list indicator
//
pub const ARGP_BYTEDATA: c_uint = 0x01;
pub const ARGP_BYTELIST: c_uint = 0x02;
pub const ARGP_CHARLIST: c_uint = 0x03;
pub const ARGP_DATAOBJ: c_uint = 0x04;
pub const ARGP_DATAOBJLIST: c_uint = 0x05;
pub const ARGP_DWORDDATA: c_uint = 0x06;
pub const ARGP_FIELDLIST: c_uint = 0x07;
pub const ARGP_NAME: c_uint = 0x08;
pub const ARGP_NAMESTRING: c_uint = 0x09;
pub const ARGP_OBJLIST: c_uint = 0x0A;
pub const ARGP_PKGLENGTH: c_uint = 0x0B;
pub const ARGP_SUPERNAME: c_uint = 0x0C;
pub const ARGP_TARGET: c_uint = 0x0D;
pub const ARGP_TERMARG: c_uint = 0x0E;
pub const ARGP_TERMLIST: c_uint = 0x0F;
pub const ARGP_WORDDATA: c_uint = 0x10;
pub const ARGP_QWORDDATA: c_uint = 0x11;
pub const ARGP_SIMPLENAME: c_uint = 0x12	/* name_string | local_term | arg_term */;
pub const ARGP_NAME_OR_REF: c_uint = 0x13	/* For object_type only */;
pub const ARGP_MAX: c_uint = 0x13;
pub const ARGP_COMMENT: c_uint = 0x14;
//
// Resolved argument types for the AML Interpreter
// Each field in the arg_types u32 is 5 bits, allowing for a maximum of 6 arguments.
// There can be up to 31 unique argument types (0 is end-of-arg-list indicator)
//
// Note1: These values are completely independent from the ACPI_TYPEs
// i.e., ARGI_INTEGER != ACPI_TYPE_INTEGER
//
// Note2: If and when 5 bits becomes insufficient, it would probably be best
// to convert to a 6-byte array of argument types, allowing 8 bits per argument.
//
// Single, simple types
pub const ARGI_ANYTYPE: c_uint = 0x01	/* Don't care */;
pub const ARGI_PACKAGE: c_uint = 0x02;
pub const ARGI_EVENT: c_uint = 0x03;
pub const ARGI_MUTEX: c_uint = 0x04;
pub const ARGI_DDBHANDLE: c_uint = 0x05;
// Interchangeable types (via implicit conversion)
pub const ARGI_INTEGER: c_uint = 0x06;
pub const ARGI_STRING: c_uint = 0x07;
pub const ARGI_BUFFER: c_uint = 0x08;
pub const ARGI_BUFFER_OR_STRING: c_uint = 0x09	/* Used by MID op only */;
pub const ARGI_COMPUTEDATA: c_uint = 0x0A	/* Buffer, String, or Integer */;
// Reference objects
pub const ARGI_INTEGER_REF: c_uint = 0x0B;
pub const ARGI_OBJECT_REF: c_uint = 0x0C;
pub const ARGI_DEVICE_REF: c_uint = 0x0D;
pub const ARGI_REFERENCE: c_uint = 0x0E;
pub const ARGI_TARGETREF: c_uint = 0x0F	/* Target, subject to implicit conversion */;
pub const ARGI_FIXED_TARGET: c_uint = 0x10	/* Target, no implicit conversion */;
pub const ARGI_SIMPLE_TARGET: c_uint = 0x11	/* Name, Local, Arg -- no implicit conversion */;
pub const ARGI_STORE_TARGET: c_uint = 0x12	/* Target for store is TARGETREF + package objects */;
// Multiple/complex types
pub const ARGI_DATAOBJECT: c_uint = 0x13	/* Buffer, String, package or reference to a node - Used only by size_of operator */;
pub const ARGI_COMPLEXOBJ: c_uint = 0x14	/* Buffer, String, or package (Used by INDEX op only) */;
pub const ARGI_REF_OR_STRING: c_uint = 0x15	/* Reference or String (Used by DEREFOF op only) */;
pub const ARGI_REGION_OR_BUFFER: c_uint = 0x16	/* Used by LOAD op only */;
pub const ARGI_DATAREFOBJ: c_uint = 0x17;
// Note: types above can expand to 0x1F maximum
pub const ARGI_INVALID_OPCODE: c_uint = 0xFFFFFFFF;
//
// Some of the flags and types below are of the form:
//
// AML_FLAGS_EXEC_#A_#T,#R, or
// AML_TYPE_EXEC_#A_#T,#R where:
//
// #A is the number of required arguments
// #T is the number of target operands
// #R indicates whether there is a return value
//
// These types are used for the top-level dispatch of the AML
// opcode. They group similar operators that can share common
// front-end code before dispatch to the final code that implements
// the operator.
//
// Opcode information flags
//
pub const AML_LOGICAL: c_uint = 0x0001;
pub const AML_LOGICAL_NUMERIC: c_uint = 0x0002;
pub const AML_MATH: c_uint = 0x0004;
pub const AML_CREATE: c_uint = 0x0008;
pub const AML_FIELD: c_uint = 0x0010;
pub const AML_DEFER: c_uint = 0x0020;
pub const AML_NAMED: c_uint = 0x0040;
pub const AML_NSNODE: c_uint = 0x0080;
pub const AML_NSOPCODE: c_uint = 0x0100;
pub const AML_NSOBJECT: c_uint = 0x0200;
pub const AML_HAS_RETVAL: c_uint = 0x0400;
pub const AML_HAS_TARGET: c_uint = 0x0800;
pub const AML_HAS_ARGS: c_uint = 0x1000;
pub const AML_CONSTANT: c_uint = 0x2000;
pub const AML_NO_OPERAND_RESOLVE: c_uint = 0x4000;
// Convenient flag groupings of the flags above

//
// The opcode Type is used in a dispatch table, do not change
// or add anything new without updating the table.
//
pub const AML_TYPE_EXEC_0A_0T_1R: c_uint = 0x00	/* 0 Args, 0 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_1A_0T_0R: c_uint = 0x01	/* 1 Args, 0 Target, 0 ret_val */;
pub const AML_TYPE_EXEC_1A_0T_1R: c_uint = 0x02	/* 1 Args, 0 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_1A_1T_0R: c_uint = 0x03	/* 1 Args, 1 Target, 0 ret_val */;
pub const AML_TYPE_EXEC_1A_1T_1R: c_uint = 0x04	/* 1 Args, 1 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_2A_0T_0R: c_uint = 0x05	/* 2 Args, 0 Target, 0 ret_val */;
pub const AML_TYPE_EXEC_2A_0T_1R: c_uint = 0x06	/* 2 Args, 0 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_2A_1T_1R: c_uint = 0x07	/* 2 Args, 1 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_2A_2T_1R: c_uint = 0x08	/* 2 Args, 2 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_3A_0T_0R: c_uint = 0x09	/* 3 Args, 0 Target, 0 ret_val */;
pub const AML_TYPE_EXEC_3A_1T_1R: c_uint = 0x0A	/* 3 Args, 1 Target, 1 ret_val */;
pub const AML_TYPE_EXEC_6A_0T_1R: c_uint = 0x0B	/* 6 Args, 0 Target, 1 ret_val */;
// End of types used in dispatch table
pub const AML_TYPE_LITERAL: c_uint = 0x0C;
pub const AML_TYPE_CONSTANT: c_uint = 0x0D;
pub const AML_TYPE_METHOD_ARGUMENT: c_uint = 0x0E;
pub const AML_TYPE_LOCAL_VARIABLE: c_uint = 0x0F;
pub const AML_TYPE_DATA_TERM: c_uint = 0x10;
// Generic for an op that returns a value
pub const AML_TYPE_METHOD_CALL: c_uint = 0x11;
// Miscellaneous types
pub const AML_TYPE_CREATE_FIELD: c_uint = 0x12;
pub const AML_TYPE_CREATE_OBJECT: c_uint = 0x13;
pub const AML_TYPE_CONTROL: c_uint = 0x14;
pub const AML_TYPE_NAMED_NO_OBJ: c_uint = 0x15;
pub const AML_TYPE_NAMED_FIELD: c_uint = 0x16;
pub const AML_TYPE_NAMED_SIMPLE: c_uint = 0x17;
pub const AML_TYPE_NAMED_COMPLEX: c_uint = 0x18;
pub const AML_TYPE_RETURN: c_uint = 0x19;
pub const AML_TYPE_UNDEFINED: c_uint = 0x1A;
pub const AML_TYPE_BOGUS: c_uint = 0x1B;
// AML Package Length encodings
pub const ACPI_AML_PACKAGE_TYPE1: c_uint = 0x40;
pub const ACPI_AML_PACKAGE_TYPE2: c_uint = 0x4000;
pub const ACPI_AML_PACKAGE_TYPE3: c_uint = 0x400000;
pub const ACPI_AML_PACKAGE_TYPE4: c_uint = 0x40000000;
//
// Opcode classes
//
pub const AML_CLASS_EXECUTE: c_uint = 0x00;
pub const AML_CLASS_CREATE: c_uint = 0x01;
pub const AML_CLASS_ARGUMENT: c_uint = 0x02;
pub const AML_CLASS_NAMED_OBJECT: c_uint = 0x03;
pub const AML_CLASS_CONTROL: c_uint = 0x04;
pub const AML_CLASS_ASCII: c_uint = 0x05;
pub const AML_CLASS_PREFIX: c_uint = 0x06;
pub const AML_CLASS_INTERNAL: c_uint = 0x07;
pub const AML_CLASS_RETURN_VALUE: c_uint = 0x08;
pub const AML_CLASS_METHOD_CALL: c_uint = 0x09;
pub const AML_CLASS_UNKNOWN: c_uint = 0x0A;
// Comparison operation codes for match_op operator
pub const MAX_MATCH_OPERATOR: c_int = 5;
//
// field_flags
//
// This byte is extracted from the AML and includes three separate
// pieces of information about the field:
// 1) The field access type
// 2) The field update rule
// 3) The lock rule for the field
//
// Bits 00 - 03 : access_type (any_acc, byte_acc, etc.)
// 04      : lock_rule (1 == Lock)
// 05 - 06 : update_rule
//
pub const AML_FIELD_ACCESS_TYPE_MASK: c_uint = 0x0F;
pub const AML_FIELD_LOCK_RULE_MASK: c_uint = 0x10;
pub const AML_FIELD_UPDATE_RULE_MASK: c_uint = 0x60;
// 1) Field Access Types
// 2) Field Lock Rules
// 3) Field Update Rules
//
// Field Access Attributes.
// This byte is extracted from the AML via the
// access_as keyword
//
// Bit fields in the AML method_flags byte
pub const AML_METHOD_ARG_COUNT: c_uint = 0x07;
pub const AML_METHOD_SERIALIZED: c_uint = 0x08;
pub const AML_METHOD_SYNC_LEVEL: c_uint = 0xF0;
