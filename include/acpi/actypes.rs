//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/actypes.h
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
// Name: actypes.h - Common data types for the entire ACPI subsystem
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// acpisrc:struct_defs -- for acpisrc conversion
//
// ACPI_MACHINE_WIDTH must be specified in an OS- or compiler-dependent
// header and must be either 32 or 64. 16-bit ACPICA is no longer
// supported, as of 12/2006.
//

//
// Data type ranges
// Note: These macros are designed to be compiler independent as well as
// working around problems that some 32-bit compilers have with 64-bit
// constants.
//

pub const ACPI_ASCII_MAX: c_uint = 0x7F;
//
// Architecture-specific ACPICA Subsystem Data Types
//
// The goal of these types is to provide source code portability across
// 16-bit, 32-bit, and 64-bit targets.
//
// 1) The following types are of fixed size for all targets (16/32/64):
//
// u8           Logical boolean
//
// u8           8-bit  (1 byte) unsigned value
// u16          16-bit (2 byte) unsigned value
// u32          32-bit (4 byte) unsigned value
// u64          64-bit (8 byte) unsigned value
//
// s16          16-bit (2 byte) signed value
// s32          32-bit (4 byte) signed value
// s64          64-bit (8 byte) signed value
//
// COMPILER_DEPENDENT_UINT64/s64 - These types are defined in the
// compiler-dependent header(s) and were introduced because there is no
// common 64-bit integer type across the various compilation models, as
// shown in the table below.
//
// Datatype  LP64 ILP64 LLP64 ILP32 LP32 16bit
// char      8    8     8     8     8    8
// short     16   16    16    16    16   16
// _int32         32
// int       32   64    32    32    16   16
// long      64   64    32    32    32   32
// long long            64    64
// pointer   64   64    64    32    32   32
//
// Note: ILP64 and LP32 are currently not supported.
//
// 2) These types represent the native word size of the target mode of the
// processor, and may be 16-bit, 32-bit, or 64-bit as required. They are
// usually used for memory allocation, efficient loop counters, and array
// indexes. The types are similar to the size_t type in the C library and
// are required because there is no C type that consistently represents the
// native data width. acpi_size is needed because there is no guarantee
// that a kernel-level C library is present.
//
// acpi_size        16/32/64-bit unsigned value
// acpi_native_int  16/32/64-bit signed value
//
// Common types for all compilers, all targets
//

pub type u8 = c_uchar;
pub type u16 = c_ushort;
pub type s16 = c_short;
pub type u64 = COMPILER_DEPENDENT_UINT64;
pub type s64 = COMPILER_DEPENDENT_INT64;

//
// Value returned by acpi_os_get_thread_id. There is no standard "thread_id"
// across operating systems or even the various UNIX systems. Since ACPICA
// only needs the thread ID as a unique thread identifier, we use a u64
// as the only common data type - it will accommodate any type of pointer or
// any type of integer. It is up to the host-dependent OSL to cast the
// native thread ID type to a u64 (in acpi_os_get_thread_id).
//

//
// Types specific to 64-bit targets
//

pub type u32 = c_uint;
pub type s32 = c_int;

pub type acpi_native_int = i64;
pub type acpi_size = u64;
pub type acpi_io_address = u64;
pub type acpi_physical_address = u64;

//
// In the case of the Itanium Processor Family (IPF), the hardware does not
// support misaligned memory transfers. Set the MISALIGNMENT_NOT_SUPPORTED
// flag to indicate that special precautions must be taken to avoid alignment
// faults. (IA64 or ia64 is currently used by existing compilers to indicate
// IPF.)
//
// Note: EM64T and other X86-64 processors support misaligned transfers,
// so there is no need to define this flag.
//

// Macro flag: #define ACPI_MISALIGNMENT_NOT_SUPPORTED

//
// Types specific to 32-bit targets
//

pub type u32 = c_uint;
pub type s32 = c_int;

pub type acpi_native_int = i32;
pub type acpi_size = u32;

//
// OSPMs can define this to shrink the size of the structures for 32-bit
// none PAE environment. ASL compiler may always define this to generate
// 32-bit OSPM compliant tables.
//
pub type acpi_io_address = u32;
pub type acpi_physical_address = u32;

//
// It is reported that, after some calculations, the physical addresses can
// wrap over the 32-bit boundary on 32-bit PAE environment.
// https://bugzilla.kernel.org/show_bug.cgi?id=87971
//
pub type acpi_io_address = u64;
pub type acpi_physical_address = u64;

// ACPI_MACHINE_WIDTH must be either 64 or 32

//
// OS-dependent types
//
// If the defaults below are not appropriate for the host system, they can
// be defined in the OS-specific header, and this will take precedence.
//
// Flags for acpi_os_acquire_lock/acpi_os_release_lock

// Object returned from acpi_os_create_cache

//
// Synchronization objects - Mutexes, Semaphores, and spin_locks
//

//
// These macros are used if the host OS does not support a mutex object.
// Map the OSL Mutex interfaces to binary semaphores.
//

// Configurable types for synchronization objects

//
// Compiler-dependent types
//
// If the defaults below are not appropriate for the host compiler, they can
// be defined in the compiler-specific header, and this will take precedence.
//
// Use C99 uintptr_t for pointer casting if available, "void *" otherwise

//
// ACPI_PRINTF_LIKE is used to tag functions as "printf-like" because
// some compilers can catch printf format string problems
//

// Macro flag: #define ACPI_PRINTF_LIKE(c)

//
// Some compilers complain about unused variables. Sometimes we don't want
// to use all the variables (for example, _acpi_module_name). This allows us
// to tell the compiler in a per-variable manner that a variable
// is unused
//

// Macro flag: #define ACPI_UNUSED_VAR

//
// All ACPICA external functions that are available to the rest of the
// kernel are tagged with these macros which can be defined as appropriate
// for the host.
//
// Notes:
// ACPI_EXPORT_SYMBOL_INIT is used for initialization and termination
// interfaces that may need special processing.
// ACPI_EXPORT_SYMBOL is used for all other public external functions.
//

// Macro flag: #define ACPI_EXPORT_SYMBOL_INIT(symbol)

// Macro flag: #define ACPI_EXPORT_SYMBOL(symbol)

//
// Compiler/Clibrary-dependent debug initialization. Used for ACPICA
// utilities only.
//

// Macro flag: #define ACPI_DEBUG_INITIALIZE()

//
// Configuration
//

// Macro flag: #define ACPI_FREE(a)
// Macro flag: #define ACPI_MEM_TRACKING(a)

//
// Memory allocation tracking (used by acpi_exec to detect memory leaks)
//

//
// Normal memory allocation directly via the OS services layer
//

// Macro flag: #define ACPI_MEM_TRACKING(a)

//
// ACPI Specification constants (Do not change unless the specification
// changes)
//
// Number of distinct FADT-based GPE register blocks (GPE0 and GPE1)
pub const ACPI_MAX_GPE_BLOCKS: c_int = 2;
// Default ACPI register widths
pub const ACPI_GPE_REGISTER_WIDTH: c_int = 8;
pub const ACPI_PM1_REGISTER_WIDTH: c_int = 16;
pub const ACPI_PM2_REGISTER_WIDTH: c_int = 8;
pub const ACPI_PM_TIMER_WIDTH: c_int = 32;
pub const ACPI_RESET_REGISTER_WIDTH: c_int = 8;
// Names within the namespace are 4 bytes long

// Sizes for ACPI table headers
pub const ACPI_OEM_ID_SIZE: c_int = 6;
pub const ACPI_OEM_TABLE_ID_SIZE: c_int = 8;
// ACPI/PNP hardware IDs

// PM Timer ticks per second (HZ)
pub const ACPI_PM_TIMER_FREQUENCY: c_int = 3579545;
//
// Independent types
//
// Logical defines and NULL

//
// Miscellaneous types
//
// Time constants for timer calculations

// Owner IDs are used to track namespace nodes for selective deletion
pub type acpi_owner_id = u16;
pub const ACPI_OWNER_ID_MAX: c_uint = 0xFFF	/* 4095 possible owner IDs */;
pub const ACPI_INTEGER_BIT_SIZE: c_int = 64;

pub const ACPI_MAX64_DECIMAL_DIGITS: c_int = 20;
pub const ACPI_MAX32_DECIMAL_DIGITS: c_int = 10;
pub const ACPI_MAX16_DECIMAL_DIGITS: c_int = 5;
pub const ACPI_MAX8_DECIMAL_DIGITS: c_int = 3;
//
// Constants with special meanings
//

pub const ACPI_WAIT_FOREVER: c_uint = 0xFFFF	/* u16, as per ACPI spec */;
pub const ACPI_DO_NOT_WAIT: c_int = 0;
//
// Obsolete: Acpi integer width. In ACPI version 1 (1996), integers are
// 32 bits. In ACPI version 2 (2000) and later, integers are max 64 bits.
// Note that this pertains to the ACPI integer type only, not to other
// integers used in the implementation of the ACPICA subsystem.
//
// 01/2010: This type is obsolete and has been removed from the entire ACPICA
// code base. It remains here for compatibility with device drivers that use
// the type. However, it will be removed in the future.
//
pub type acpi_integer = u64;

//
// Commonly used macros
//
// Data manipulation

// Size calculation

// Pointer manipulation

// Pointer/Integer type conversions

// Optimizations for 4-character (32-bit) acpi_name manipulation

// Support for the special RSDP signature (8 characters)

// Support for OEMx signature (x can be any character)

//
// Algorithm to obtain access bit or byte width.
// Can be used with access_width of struct acpi_generic_address and access_size of
// struct acpi_resource_generic_register.
//
pub const ACPI_ACCESS_BIT_SHIFT: c_int = 2;

//
// Miscellaneous constants
//
// Initialization sequence options
//
pub const ACPI_FULL_INITIALIZATION: c_uint = 0x0000;
pub const ACPI_NO_FACS_INIT: c_uint = 0x0001;
pub const ACPI_NO_ACPI_ENABLE: c_uint = 0x0002;
pub const ACPI_NO_HARDWARE_INIT: c_uint = 0x0004;
pub const ACPI_NO_EVENT_INIT: c_uint = 0x0008;
pub const ACPI_NO_HANDLER_INIT: c_uint = 0x0010;
pub const ACPI_NO_OBJECT_INIT: c_uint = 0x0020;
pub const ACPI_NO_DEVICE_INIT: c_uint = 0x0040;
pub const ACPI_NO_ADDRESS_SPACE_INIT: c_uint = 0x0080;
//
// Initialization state
//
pub const ACPI_SUBSYSTEM_INITIALIZE: c_uint = 0x01;
pub const ACPI_INITIALIZED_OK: c_uint = 0x02;
//
// Power state values
//

pub const ACPI_S_STATE_COUNT: c_int = 6;

pub const ACPI_D_STATE_COUNT: c_int = 5;

pub const ACPI_C_STATE_COUNT: c_int = 4;
//
// Sleep type invalid value
//
pub const ACPI_SLEEP_TYPE_MAX: c_uint = 0x7;
pub const ACPI_SLEEP_TYPE_INVALID: c_uint = 0xFF;
//
// Standard notify values
//

pub const ACPI_GENERIC_NOTIFY_MAX: c_uint = 0x0F;
pub const ACPI_SPECIFIC_NOTIFY_MAX: c_uint = 0x84;
//
// Types associated with ACPI names and objects. The first group of
// values (up to ACPI_TYPE_EXTERNAL_MAX) correspond to the definition
// of the ACPI object_type() operator (See the ACPI Spec). Therefore,
// only add to the first group if the spec changes.
//
// NOTE: Types must be kept in sync with the global acpi_ns_properties
// and acpi_ns_type_names arrays.
//
pub type acpi_object_type = u32;
pub const ACPI_TYPE_ANY: c_uint = 0x00;
pub const ACPI_TYPE_INTEGER: c_uint = 0x01	/* Byte/Word/Dword/Zero/One/Ones */;
pub const ACPI_TYPE_STRING: c_uint = 0x02;
pub const ACPI_TYPE_BUFFER: c_uint = 0x03;
pub const ACPI_TYPE_PACKAGE: c_uint = 0x04	/* byte_const, multiple data_term/Constant/super_name */;
pub const ACPI_TYPE_FIELD_UNIT: c_uint = 0x05;
pub const ACPI_TYPE_DEVICE: c_uint = 0x06	/* Name, multiple Node */;
pub const ACPI_TYPE_EVENT: c_uint = 0x07;
pub const ACPI_TYPE_METHOD: c_uint = 0x08	/* Name, byte_const, multiple Code */;
pub const ACPI_TYPE_MUTEX: c_uint = 0x09;
pub const ACPI_TYPE_REGION: c_uint = 0x0A;
pub const ACPI_TYPE_POWER: c_uint = 0x0B	/* Name,byte_const,word_const,multi Node */;
pub const ACPI_TYPE_PROCESSOR: c_uint = 0x0C	/* Name,byte_const,Dword_const,byte_const,multi nm_o */;
pub const ACPI_TYPE_THERMAL: c_uint = 0x0D	/* Name, multiple Node */;
pub const ACPI_TYPE_BUFFER_FIELD: c_uint = 0x0E;
pub const ACPI_TYPE_DDB_HANDLE: c_uint = 0x0F;
pub const ACPI_TYPE_DEBUG_OBJECT: c_uint = 0x10;
pub const ACPI_TYPE_EXTERNAL_MAX: c_uint = 0x10;

//
// These are object types that do not map directly to the ACPI
// object_type() operator. They are used for various internal purposes
// only. If new predefined ACPI_TYPEs are added (via the ACPI
// specification), these internal types must move upwards. (There
// is code that depends on these values being contiguous with the
// external types above.)
//
pub const ACPI_TYPE_LOCAL_REGION_FIELD: c_uint = 0x11;
pub const ACPI_TYPE_LOCAL_BANK_FIELD: c_uint = 0x12;
pub const ACPI_TYPE_LOCAL_INDEX_FIELD: c_uint = 0x13;
pub const ACPI_TYPE_LOCAL_REFERENCE: c_uint = 0x14	/* Arg#, Local#, Name, Debug, ref_of, Index */;
pub const ACPI_TYPE_LOCAL_ALIAS: c_uint = 0x15;
pub const ACPI_TYPE_LOCAL_METHOD_ALIAS: c_uint = 0x16;
pub const ACPI_TYPE_LOCAL_NOTIFY: c_uint = 0x17;
pub const ACPI_TYPE_LOCAL_ADDRESS_HANDLER: c_uint = 0x18;
pub const ACPI_TYPE_LOCAL_RESOURCE: c_uint = 0x19;
pub const ACPI_TYPE_LOCAL_RESOURCE_FIELD: c_uint = 0x1A;
pub const ACPI_TYPE_LOCAL_SCOPE: c_uint = 0x1B	/* 1 Name, multiple object_list Nodes */;
pub const ACPI_TYPE_NS_NODE_MAX: c_uint = 0x1B	/* Last typecode used within a NS Node */;

//
// These are special object types that never appear in
// a Namespace node, only in an object of union acpi_operand_object
//
pub const ACPI_TYPE_LOCAL_EXTRA: c_uint = 0x1C;
pub const ACPI_TYPE_LOCAL_DATA: c_uint = 0x1D;
pub const ACPI_TYPE_LOCAL_MAX: c_uint = 0x1D;
// All types above here are invalid
pub const ACPI_TYPE_INVALID: c_uint = 0x1E;
pub const ACPI_TYPE_NOT_FOUND: c_uint = 0xFF;

//
// All I/O
//
pub const ACPI_READ: c_int = 0;
pub const ACPI_WRITE: c_int = 1;
pub const ACPI_IO_MASK: c_int = 1;
//
// Event Types: Fixed & General Purpose
//
pub type acpi_event_type = u32;
//
// Fixed events
//
pub const ACPI_EVENT_PMTIMER: c_int = 0;
pub const ACPI_EVENT_GLOBAL: c_int = 1;
pub const ACPI_EVENT_POWER_BUTTON: c_int = 2;
pub const ACPI_EVENT_SLEEP_BUTTON: c_int = 3;
pub const ACPI_EVENT_RTC: c_int = 4;
pub const ACPI_EVENT_MAX: c_int = 4;

//
// Event status - Per event
// -------------
// The encoding of acpi_event_status is illustrated below.
// Note that a set bit (1) indicates the property is TRUE
// (e.g. if bit 0 is set then the event is enabled).
// +-------------+-+-+-+-+-+-+
// |   Bits 31:6 |5|4|3|2|1|0|
// +-------------+-+-+-+-+-+-+
// |     | | | | | |
// |     | | | | | +- Enabled?
// |     | | | | +--- Enabled for wake?
// |     | | | +----- Status bit set?
// |     | | +------- Enable bit set?
// |     | +--------- Has a handler?
// |     +----------- Masked?
// +----------------- <Reserved>
//
pub type acpi_event_status = u32;

// Actions for acpi_set_gpe, acpi_gpe_wakeup, acpi_hw_low_set_gpe
pub const ACPI_GPE_ENABLE: c_int = 0;
pub const ACPI_GPE_DISABLE: c_int = 1;
pub const ACPI_GPE_CONDITIONAL_ENABLE: c_int = 2;
//
// GPE info flags - Per GPE
// +---+-+-+-+---+
// |7:6|5|4|3|2:0|
// +---+-+-+-+---+
// |  | | |  |
// |  | | |  +-- Type of dispatch:to method, handler, notify, or none
// |  | | +----- Interrupt type: edge or level triggered
// |  | +------- Is a Wake GPE
// |  +--------- Has been enabled automatically at init time
// +------------ <Reserved>
//

//
// Flags for GPE and Lock interfaces
//
pub const ACPI_NOT_ISR: c_uint = 0x1;
pub const ACPI_ISR: c_uint = 0x0;
// Notify types
pub const ACPI_SYSTEM_NOTIFY: c_uint = 0x1;
pub const ACPI_DEVICE_NOTIFY: c_uint = 0x2;

pub const ACPI_MAX_NOTIFY_HANDLER_TYPE: c_uint = 0x3;
pub const ACPI_NUM_NOTIFY_TYPES: c_int = 2;
pub const ACPI_MAX_SYS_NOTIFY: c_uint = 0x7F;
pub const ACPI_MAX_DEVICE_SPECIFIC_NOTIFY: c_uint = 0xBF;

// Address Space (Operation Region) Types
pub type acpi_adr_space_type = u8;

pub const ACPI_NUM_PREDEFINED_REGIONS: c_int = 12;
//
// Special Address Spaces
//
// Note: A Data Table region is a special type of operation region
// that has its own AML opcode. However, internally, the AML
// interpreter simply creates an operation region with an address
// space type of ACPI_ADR_SPACE_DATA_TABLE.
//

// Values for _REG connection code
pub const ACPI_REG_DISCONNECT: c_int = 0;
pub const ACPI_REG_CONNECT: c_int = 1;
//
// bit_register IDs
//
// These values are intended to be used by the hardware interfaces
// and are mapped to individual bitfields defined within the ACPI
// registers. See the acpi_gbl_bit_register_info global table in utglobal.c
// for this mapping.
//
// PM1 Status register
pub const ACPI_BITREG_TIMER_STATUS: c_uint = 0x00;
pub const ACPI_BITREG_BUS_MASTER_STATUS: c_uint = 0x01;
pub const ACPI_BITREG_GLOBAL_LOCK_STATUS: c_uint = 0x02;
pub const ACPI_BITREG_POWER_BUTTON_STATUS: c_uint = 0x03;
pub const ACPI_BITREG_SLEEP_BUTTON_STATUS: c_uint = 0x04;
pub const ACPI_BITREG_RT_CLOCK_STATUS: c_uint = 0x05;
pub const ACPI_BITREG_WAKE_STATUS: c_uint = 0x06;
pub const ACPI_BITREG_PCIEXP_WAKE_STATUS: c_uint = 0x07;
// PM1 Enable register
pub const ACPI_BITREG_TIMER_ENABLE: c_uint = 0x08;
pub const ACPI_BITREG_GLOBAL_LOCK_ENABLE: c_uint = 0x09;
pub const ACPI_BITREG_POWER_BUTTON_ENABLE: c_uint = 0x0A;
pub const ACPI_BITREG_SLEEP_BUTTON_ENABLE: c_uint = 0x0B;
pub const ACPI_BITREG_RT_CLOCK_ENABLE: c_uint = 0x0C;
pub const ACPI_BITREG_PCIEXP_WAKE_DISABLE: c_uint = 0x0D;
// PM1 Control register
pub const ACPI_BITREG_SCI_ENABLE: c_uint = 0x0E;
pub const ACPI_BITREG_BUS_MASTER_RLD: c_uint = 0x0F;
pub const ACPI_BITREG_GLOBAL_LOCK_RELEASE: c_uint = 0x10;
pub const ACPI_BITREG_SLEEP_TYPE: c_uint = 0x11;
pub const ACPI_BITREG_SLEEP_ENABLE: c_uint = 0x12;
// PM2 Control register
pub const ACPI_BITREG_ARB_DISABLE: c_uint = 0x13;
pub const ACPI_BITREG_MAX: c_uint = 0x13;

// Status register values. A 1 clears a status bit. 0 = no effect
pub const ACPI_CLEAR_STATUS: c_int = 1;
// Enable and Control register values
pub const ACPI_ENABLE_EVENT: c_int = 1;
pub const ACPI_DISABLE_EVENT: c_int = 0;
//
// External ACPI object definition
//
// Note: Type == ACPI_TYPE_ANY (0) is used to indicate a NULL package
// element or an unresolved named reference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_object {
    pub /: *mut *mut acpi_object_type type; / See definition of acpi_ns_type for values,
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_INTEGER,
    pub /: *mut *mut u64 value; / The actual number,
    pub integer: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_STRING,
    pub /: *mut *mut u32 length; / # of bytes in string, excluding trailing null,
    pub /: *mut *mut *mut char pointer; / points to the string value,
    pub string: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_BUFFER,
    pub /: *mut *mut u32 length; / # of bytes in buffer,
    pub /: *mut *mut *mut u8 pointer; / points to the buffer,
    pub buffer: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_PACKAGE,
    pub /: *mut *mut u32 count; / # of elements in package,
    pub /: *mut *mut *mut acpi_object elements; / Pointer to an array of ACPI_OBJECTs,
    pub package: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_LOCAL_REFERENCE,
    pub /: *mut *mut acpi_object_type actual_type; / Type associated with the Handle,
    pub /: *mut *mut acpi_handle handle; / object reference,
    pub reference: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_PROCESSOR,
    pub proc_id: u32,
    pub pblk_address: acpi_io_address,
    pub pblk_length: u32,
    pub processor: },
    pub /: *mut *mut acpi_object_type type; / ACPI_TYPE_POWER,
    pub system_level: u32,
    pub resource_order: u32,
    pub power_resource: },
}

//
// List of objects, used as a parameter list for control method evaluation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_object_list {
    pub count: u32,
    pub pointer: *mut acpi_object,
}

//
// Miscellaneous common Data Structures used by the interfaces
//
pub const ACPI_NO_BUFFER: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_buffer {
    pub /: *mut *mut acpi_size length; / Length in bytes of the buffer,
    pub /: *mut *mut *mut void pointer; / pointer to buffer,
}

//
// name_type for acpi_get_name
//
pub const ACPI_FULL_PATHNAME: c_int = 0;
pub const ACPI_SINGLE_NAME: c_int = 1;
pub const ACPI_FULL_PATHNAME_NO_TRAILING: c_int = 2;
pub const ACPI_NAME_TYPE_MAX: c_int = 2;
//
// Predefined Namespace items
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_predefined_names {
    pub name: *const c_char,
    pub type: u8,
    pub val: *mut c_char,
}

//
// Structure and flags for acpi_get_system_info
//
pub const ACPI_SYS_MODE_UNKNOWN: c_uint = 0x0000;
pub const ACPI_SYS_MODE_ACPI: c_uint = 0x0001;
pub const ACPI_SYS_MODE_LEGACY: c_uint = 0x0002;
pub const ACPI_SYS_MODES_MASK: c_uint = 0x0003;
//
// System info returned by acpi_get_system_info()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_system_info {
    pub acpi_ca_version: u32,
    pub flags: u32,
    pub timer_resolution: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub debug_level: u32,
    pub debug_layer: u32,
}

//
// System statistics returned by acpi_get_statistics()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_statistics {
    pub sci_count: u32,
    pub gpe_count: u32,
    pub fixed_event_count: [u32; ACPI_NUM_FIXED_EVENTS],
    pub method_count: u32,
}

//
// Types specific to the OS service interfaces
//
// Various handlers and callback procedures
//
pub const ACPI_EVENT_TYPE_GPE: c_int = 0;
pub const ACPI_EVENT_TYPE_FIXED: c_int = 1;
pub const ACPI_INIT_DEVICE_INI: c_int = 1;
// Table Event handler (Load, load_table, etc.) and types
// Table Event Types
pub const ACPI_TABLE_EVENT_LOAD: c_uint = 0x0;
pub const ACPI_TABLE_EVENT_UNLOAD: c_uint = 0x1;
pub const ACPI_TABLE_EVENT_INSTALL: c_uint = 0x2;
pub const ACPI_TABLE_EVENT_UNINSTALL: c_uint = 0x3;
pub const ACPI_NUM_TABLE_EVENTS: c_int = 4;
// Address Spaces (For Operation Regions)

// Special Context data for generic_serial_bus/general_purpose_io (ACPI 5.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_connection_info {
    pub connection: *mut u8,
    pub length: u16,
    pub access_length: u8,
}

// Special Context data for PCC Opregion (ACPI 6.3)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pcc_info {
    pub subspace_id: u8,
    pub length: u16,
    pub internal_buffer: *mut u8,
}

// Special Context data for FFH Opregion (ACPI 6.5)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ffh_info {
    pub offset: u64,
    pub length: u64,
}

pub const ACPI_REGION_ACTIVATE: c_int = 0;
pub const ACPI_REGION_DEACTIVATE: c_int = 1;
// Interrupt handler return values
pub const ACPI_INTERRUPT_NOT_HANDLED: c_uint = 0x00;
pub const ACPI_INTERRUPT_HANDLED: c_uint = 0x01;
// GPE handler return values
pub const ACPI_REENABLE_GPE: c_uint = 0x80;
// Length of 32-bit EISAID values when converted back to a string

// Length of UUID (string) values
pub const ACPI_UUID_LENGTH: c_int = 16;
// Length of 3-byte PCI class code values when converted back to a string

// Structures used for device/processor HID, UID, CID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pnp_device_id {
    pub /: *mut *mut u32 length; / Length of string + null,
    pub string: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pnp_device_id_list {
    pub /: *mut *mut u32 count; / Number of IDs in Ids array,
    pub /: *mut *mut u32 list_size; / Size of list, including ID strings,
    pub /: *mut *mut acpi_pnp_device_id ids[]; / ID array,
}

//
// Structure returned from acpi_get_object_info.
// Optimized for both 32-bit and 64-bit builds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_info {
    pub /: *mut *mut u32 info_size; / Size of info, including ID strings,
    pub /: *mut *mut u32 name; / ACPI object Name,
    pub /: *mut *mut acpi_object_type type; / ACPI object Type,
    pub /: *mut *mut u8 param_count; / If a method, required parameter count,
    pub /: *mut *mut u16 valid; / Indicates which optional fields are valid,
    pub /: *mut *mut u8 flags; / Miscellaneous info,
    pub /: *mut *mut u8 highest_dstates[4]; / _sx_d values: 0xFF indicates not valid,
    pub /: *mut *mut u8 lowest_dstates[5]; / _sx_w values: 0xFF indicates not valid,
    pub /: *mut *mut u64 address; / _ADR value,
    pub /: *mut *mut acpi_pnp_device_id hardware_id; / _HID value,
    pub /: *mut *mut acpi_pnp_device_id unique_id; / _UID value,
    pub /: *mut *mut acpi_pnp_device_id class_code; / _CLS value,
    pub /: *mut *mut acpi_pnp_device_id_list compatible_id_list; / _CID list <must be last>,
}

// Values for Flags field above (acpi_get_object_info)
pub const ACPI_PCI_ROOT_BRIDGE: c_uint = 0x01;
// Flags for Valid field above (acpi_get_object_info)
pub const ACPI_VALID_ADR: c_uint = 0x0002;
pub const ACPI_VALID_HID: c_uint = 0x0004;
pub const ACPI_VALID_UID: c_uint = 0x0008;
pub const ACPI_VALID_CID: c_uint = 0x0020;
pub const ACPI_VALID_CLS: c_uint = 0x0040;
pub const ACPI_VALID_SXDS: c_uint = 0x0100;
pub const ACPI_VALID_SXWS: c_uint = 0x0200;
// Flags for _STA method
pub const ACPI_STA_DEVICE_PRESENT: c_uint = 0x01;
pub const ACPI_STA_DEVICE_ENABLED: c_uint = 0x02;
pub const ACPI_STA_DEVICE_UI: c_uint = 0x04;
pub const ACPI_STA_DEVICE_FUNCTIONING: c_uint = 0x08;
pub const ACPI_STA_DEVICE_OK: c_uint = 0x08	/* Synonym */;
pub const ACPI_STA_BATTERY_PRESENT: c_uint = 0x10;
// Context structs for address space handlers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_id {
    pub segment: u16,
    pub bus: u16,
    pub device: u16,
    pub function: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mem_mapping {
    pub physical_address: acpi_physical_address,
    pub logical_address: *mut u8,
    pub length: acpi_size,
    pub next_mm: *mut acpi_mem_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_mem_space_context {
    pub length: u32,
    pub address: acpi_physical_address,
    pub cur_mm: *mut acpi_mem_mapping,
    pub first_mm: *mut acpi_mem_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_data_table_mapping {
    pub pointer: *mut c_void,
}

//
// struct acpi_memory_list is used only if the ACPICA local cache is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_memory_list {
    pub list_name: *const c_char,
    pub list_head: *mut c_void,
    pub object_size: u16,
    pub max_depth: u16,
    pub current_depth: u16,

// Statistics for debug memory tracking only
    pub total_allocated: u32,
    pub total_freed: u32,
    pub max_occupied: u32,
    pub total_size: u32,
    pub current_total_size: u32,
    pub requests: u32,
    pub hits: u32,

}

// Definitions of trace event types
// Definitions of _OSI support
pub const ACPI_VENDOR_STRINGS: c_uint = 0x01;
pub const ACPI_FEATURE_STRINGS: c_uint = 0x02;
pub const ACPI_ENABLE_INTERFACES: c_uint = 0x00;
pub const ACPI_DISABLE_INTERFACES: c_uint = 0x04;

pub const ACPI_OSI_WIN_2000: c_uint = 0x01;
pub const ACPI_OSI_WIN_XP: c_uint = 0x02;
pub const ACPI_OSI_WIN_XP_SP1: c_uint = 0x03;
pub const ACPI_OSI_WINSRV_2003: c_uint = 0x04;
pub const ACPI_OSI_WIN_XP_SP2: c_uint = 0x05;
pub const ACPI_OSI_WINSRV_2003_SP1: c_uint = 0x06;
pub const ACPI_OSI_WIN_VISTA: c_uint = 0x07;
pub const ACPI_OSI_WINSRV_2008: c_uint = 0x08;
pub const ACPI_OSI_WIN_VISTA_SP1: c_uint = 0x09;
pub const ACPI_OSI_WIN_VISTA_SP2: c_uint = 0x0A;
pub const ACPI_OSI_WIN_7: c_uint = 0x0B;
pub const ACPI_OSI_WIN_8: c_uint = 0x0C;
pub const ACPI_OSI_WIN_8_1: c_uint = 0x0D;
pub const ACPI_OSI_WIN_10: c_uint = 0x0E;
pub const ACPI_OSI_WIN_10_RS1: c_uint = 0x0F;
pub const ACPI_OSI_WIN_10_RS2: c_uint = 0x10;
pub const ACPI_OSI_WIN_10_RS3: c_uint = 0x11;
pub const ACPI_OSI_WIN_10_RS4: c_uint = 0x12;
pub const ACPI_OSI_WIN_10_RS5: c_uint = 0x13;
pub const ACPI_OSI_WIN_10_19H1: c_uint = 0x14;
pub const ACPI_OSI_WIN_10_20H1: c_uint = 0x15;
pub const ACPI_OSI_WIN_11: c_uint = 0x16;
pub const ACPI_OSI_WIN_11_22H2: c_uint = 0x17;
// Definitions of getopt

// Definitions for explicit fallthrough

